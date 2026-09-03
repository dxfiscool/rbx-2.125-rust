// Auto-generated shard FJ — 150 RBX::Reflection stubs EA-sorted asc 0x86a0dc..0x879970 (demangled contains RBX::Reflection, 16171 total, 9748 covered -> 9898 after, 6273 remaining)
// Source: ida/export.json (85545 funcs) filtered demangled contains RBX::Reflection, EA asc not in crates/reflection/src/*.rs, next 150 after 0x869fc8 EA-sorted
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + #[doc(alias = mangled)] + pub fn stub_0x<ADDR> todo using rbx_core::SharedPtr not boost

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;


// 0x86a0dc — __ZN3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EED0Ev")]
pub fn stub_0x86a0dc() {
    // IDA 0x86a0dc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x86a108 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x86a108() -> bool {
    // IDA 0x86a108: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x86a10c — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x86a10c() -> bool {
    // IDA 0x86a10c: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x86a110 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x86a110() -> ! {
    todo!("0x86a110")
}

// 0x86a138 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::GetSetImpl<G3D::Vector2 (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(G3D::Vector2)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x86a138() -> ! {
    todo!("0x86a138")
}

// 0x86a16c — __ZN3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x86a16c() -> ! {
    todo!("0x86a16c")
}

// 0x86a280 — __ZN3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEED0Ev")]
pub fn stub_0x86a280() {
    // IDA 0x86a280: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x86a2ac — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::TextureId)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
pub fn stub_0x86a2ac() -> bool {
    // IDA 0x86a2ac: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x86a2b0 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::TextureId)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
pub fn stub_0x86a2b0() -> bool {
    // IDA 0x86a2b0: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x86a2b4 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x86a2b4() -> ! {
    todo!("0x86a2b4")
}

// 0x86a2dc — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x86a2dc() -> ! {
    todo!("0x86a2dc")
}

// 0x86a424 — __ZN3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x86a424() -> ! {
    todo!("0x86a424")
}

// 0x86a4c8 — __ZN3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEED0Ev")]
pub fn stub_0x86a4c8() {
    // IDA 0x86a4c8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x86a4f8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10isReadOnlyEv")]
pub fn stub_0x86a4f8() {
    // IDA 0x86a4f8: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x86a508 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11isWriteOnlyEv")]
pub fn stub_0x86a508() {
    // IDA 0x86a508: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x86a518 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0x86a518() -> ! {
    todo!("0x86a518")
}

// 0x86a540 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x86a540() -> ! {
    todo!("0x86a540")
}

// 0x86a658 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x86a658() -> ! {
    todo!("0x86a658")
}

// 0x86a720 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x86a720() -> ! {
    todo!("0x86a720")
}

// 0x86a744 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x86a744() -> ! {
    todo!("0x86a744")
}

// 0x86a818 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x86a818() -> ! {
    todo!("0x86a818")
}

// 0x86a83c — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x86a83c() -> ! {
    todo!("0x86a83c")
}

// 0x86a850 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_")]
pub fn stub_0x86a850() -> ! {
    todo!("0x86a850")
}

// 0x86a8cc — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_")]
pub fn stub_0x86a8cc() -> ! {
    todo!("0x86a8cc")
}

// 0x86a8ec — __ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_0x86a8ec() -> ! {
    todo!("0x86a8ec")
}

// 0x86a9cc — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_9FloorWireENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
pub fn stub_0x86a9cc() {
    // IDA 0x86a9cc: non-virtual thunk to `RBX::Reflection::RefPropDescriptor<RBX::FloorWire,RBX::PartInstance>::assignIDREF( int a1, int a2, int a3, int a4, ` — this/arg-adjust + tail-call (arg a1 -= 40) (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0x86a9d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
pub fn stub_0x86a9d4() -> bool {
    // IDA 0x86a9d4: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x86a9d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
pub fn stub_0x86a9d8() -> bool {
    // IDA 0x86a9d8: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x86a9dc — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x86a9dc() -> ! {
    todo!("0x86a9dc")
}

// 0x86a9fc — __ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::FloorWire::*)(void)const,void (RBX::FloorWire::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9FloorWireEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
pub fn stub_0x86a9fc() -> ! {
    todo!("0x86a9fc")
}

// 0x86af70 — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEEC1Ev")]
pub fn stub_0x86af70() -> crate::enum_desc::EnumDesc {
    // IDA 0x86af70: EnumDesc<T>::C1 -- EnumDescriptor base ctor with name "Enum", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Enum")
}

// 0x86af74 — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEEC2Ev")]
pub fn stub_0x86af74() -> crate::enum_desc::EnumDesc {
    // IDA 0x86af74: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "CellMaterial", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("CellMaterial")
}

// 0x86b2a4 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEEC1Ev")]
pub fn stub_0x86b2a4() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b2a4: EnumDesc<T>::C1 -- EnumDescriptor base ctor with name "Enum", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Enum")
}

// 0x86b2a8 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEEC2Ev")]
pub fn stub_0x86b2a8() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b2a8: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "CellBlock", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("CellBlock")
}

// 0x86b4ac — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEEC1Ev")]
pub fn stub_0x86b4ac() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b4ac: EnumDesc<T>::C1 -- EnumDescriptor base ctor with name "Enum", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Enum")
}

// 0x86b4b0 — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEEC2Ev")]
pub fn stub_0x86b4b0() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b4b0: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "CellOrientation", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("CellOrientation")
}

// 0x86b69c — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEEC1Ev")]
pub fn stub_0x86b69c() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b69c: EnumDesc<T>::C1 -- EnumDescriptor base ctor with name "Enum", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Enum")
}

// 0x86b6a0 — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEEC2Ev")]
pub fn stub_0x86b6a0() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b6a0: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "WaterForce", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("WaterForce")
}

// 0x86b8a4 — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEEC1Ev")]
pub fn stub_0x86b8a4() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b8a4: EnumDesc<T>::C1 -- EnumDescriptor base ctor with name "Enum", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("Enum")
}

// 0x86b8a8 — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEEC2Ev")]
pub fn stub_0x86b8a8() -> crate::enum_desc::EnumDesc {
    // IDA 0x86b8a8: EnumDesc<T>::C2 -- EnumDescriptor base ctor with name "WaterDirection", vtable install, empty tables (decompiled; cf. 0x37148c). Pairs are registered by the addPair stubs.
    crate::enum_desc::EnumDesc::new("WaterDirection")
}

// 0x86f874 — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::addPair(RBX::Voxel::CellMaterial,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE7addPairES3_PKc")]
pub fn stub_0x86f874(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x86f874: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x86fbd4 — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel12CellMaterialEEERT_v
#[doc(alias = "RBX::Voxel::CellMaterial & RBX::Reflection::Variant::genericConvert<RBX::Voxel::CellMaterial>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel12CellMaterialEEERT_v")]
pub fn stub_0x86fbd4() -> ! {
    todo!("0x86fbd4")
}

// 0x86fdc0 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::addPair(RBX::Voxel::CellBlock,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE7addPairES3_PKc")]
pub fn stub_0x86fdc0(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x86fdc0: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x870120 — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel9CellBlockEEERT_v
#[doc(alias = "RBX::Voxel::CellBlock & RBX::Reflection::Variant::genericConvert<RBX::Voxel::CellBlock>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel9CellBlockEEERT_v")]
pub fn stub_0x870120() -> ! {
    todo!("0x870120")
}

// 0x87030c — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::addPair(RBX::Voxel::CellOrientation,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE7addPairES3_PKc")]
pub fn stub_0x87030c(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x87030c: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x87066c — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel15CellOrientationEEERT_v
#[doc(alias = "RBX::Voxel::CellOrientation & RBX::Reflection::Variant::genericConvert<RBX::Voxel::CellOrientation>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel15CellOrientationEEERT_v")]
pub fn stub_0x87066c() -> ! {
    todo!("0x87066c")
}

// 0x870858 — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::addPair(RBX::Voxel::WaterCellForce,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE7addPairES3_PKc")]
pub fn stub_0x870858(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x870858: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x870bb8 — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel14WaterCellForceEEERT_v
#[doc(alias = "RBX::Voxel::WaterCellForce & RBX::Reflection::Variant::genericConvert<RBX::Voxel::WaterCellForce>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel14WaterCellForceEEERT_v")]
pub fn stub_0x870bb8() -> ! {
    todo!("0x870bb8")
}

// 0x870da4 — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::addPair(RBX::Voxel::WaterCellDirection,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE7addPairES3_PKc")]
pub fn stub_0x870da4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0x870da4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0x871104 — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel18WaterCellDirectionEEERT_v
#[doc(alias = "RBX::Voxel::WaterCellDirection & RBX::Reflection::Variant::genericConvert<RBX::Voxel::WaterCellDirection>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel18WaterCellDirectionEEERT_v")]
pub fn stub_0x871104() -> ! {
    todo!("0x871104")
}

// 0x8712f0 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsED1Ev")]
pub fn stub_0x8712f0() {
    // IDA 0x8712f0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x871314 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EED1Ev")]
pub fn stub_0x871314() {
    // IDA 0x871314: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x871338 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EED1Ev")]
pub fn stub_0x871338() {
    // IDA 0x871338: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x87138c — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),6>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EED1Ev")]
pub fn stub_0x87138c() {
    // IDA 0x87138c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x871390 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),4>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EED1Ev")]
pub fn stub_0x871390() {
    // IDA 0x871390: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8713ec — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection),5>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EED1Ev")]
pub fn stub_0x8713ec() {
    // IDA 0x8713ec: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x871454 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,bool ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EED1Ev")]
pub fn stub_0x871454() {
    // IDA 0x871454: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8714a8 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EED1Ev")]
pub fn stub_0x8714a8() {
    // IDA 0x8714a8: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x871508 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EED1Ev")]
pub fn stub_0x871508() {
    // IDA 0x871508: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x87155c — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EED1Ev")]
pub fn stub_0x87155c() {
    // IDA 0x87155c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x87159c — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EED1Ev")]
pub fn stub_0x87159c() {
    // IDA 0x87159c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8715c0 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,int ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EED1Ev")]
pub fn stub_0x8715c0() {
    // IDA 0x8715c0: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x8736a0 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,int ()(void),0>::BoundFuncDesc(int (RBX::MegaClusterInstance::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x8736a0() -> ! {
    todo!("0x8736a0")
}

// 0x8737a4 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,int ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EED0Ev")]
pub fn stub_0x8737a4() {
    // IDA 0x8737a4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x873858 — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,int ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x873858() -> ! {
    todo!("0x873858")
}

// 0x87387c — __ZN3RBX10Reflection11Call0HelperINS_19MegaClusterInstanceEMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::MegaClusterInstance,int (RBX::MegaClusterInstance::*)(void),int>::call(RBX::MegaClusterInstance*,int (RBX::MegaClusterInstance::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_19MegaClusterInstanceEMS2_FivEiE4callEPS2_S4_RNS0_7VariantE")]
pub fn stub_0x87387c() -> ! {
    todo!("0x87387c")
}

// 0x8738ac — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(void),0>::BoundFuncDesc(void (RBX::MegaClusterInstance::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x8738ac() -> ! {
    todo!("0x8738ac")
}

// 0x8739b0 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EED0Ev")]
pub fn stub_0x8739b0() {
    // IDA 0x8739b0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x873a64 — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x873a64() -> ! {
    todo!("0x873a64")
}

// 0x873a84 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EEC2EMS2_FS4_S4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::BoundFuncDesc(G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EEC2EMS2_FS4_S4_EPKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x873a84() -> ! {
    todo!("0x873a84")
}

// 0x873bfc — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x873bfc() -> ! {
    todo!("0x873bfc")
}

// 0x873c2c — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EED0Ev")]
pub fn stub_0x873c2c() {
    // IDA 0x873c2c: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x873d00 — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(G3D::Vector3),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3ES4_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x873d00() -> ! {
    todo!("0x873d00")
}

// 0x873d40 — __ZN3RBX10Reflection11Call1HelperINS_19MegaClusterInstanceEMS2_FN3G3D7Vector3ES4_ES4_S4_E4callEPS2_S6_RNS0_7VariantERKS4_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::MegaClusterInstance,G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),G3D::Vector3,G3D::Vector3>::call(RBX::MegaClusterInstance*,G3D::Vector3 (RBX::MegaClusterInstance::*)(G3D::Vector3),RBX::Reflection::Variant &,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_19MegaClusterInstanceEMS2_FN3G3D7Vector3ES4_ES4_S4_E4callEPS2_S6_RNS0_7VariantERKS4_")]
pub fn stub_0x873d40() -> ! {
    todo!("0x873d40")
}

// 0x873d7c — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector3ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector3ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x873d7c() -> ! {
    todo!("0x873d7c")
}

// 0x873f50 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EEC2EMS2_FS4_iiiEPKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::BoundFuncDesc(G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EEC2EMS2_FS4_iiiEPKcSA_SA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x873f50() -> ! {
    todo!("0x873f50")
}

// 0x874168 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EE16declareSignatureEPKcNS0_7VariantES8_S9_S8_S9_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EE16declareSignatureEPKcNS0_7VariantES8_S9_S8_S9_")]
pub fn stub_0x874168() -> ! {
    todo!("0x874168")
}

// 0x8741d0 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EED0Ev")]
pub fn stub_0x8741d0() {
    // IDA 0x8741d0: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8742bc — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,G3D::Vector3 ()(int,int,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN3G3D7Vector3EiiiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x8742bc() -> ! {
    todo!("0x8742bc")
}

// 0x87431c — __ZN3RBX10Reflection11Call3HelperINS_19MegaClusterInstanceEMS2_FN3G3D7Vector3EiiiEiiiS4_E4callEPS2_S6_RNS0_7VariantERKiSC_SC_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::MegaClusterInstance,G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),int,int,int,G3D::Vector3>::call(RBX::MegaClusterInstance*,G3D::Vector3 (RBX::MegaClusterInstance::*)(int,int,int),RBX::Reflection::Variant &,int const&,int const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_19MegaClusterInstanceEMS2_FN3G3D7Vector3EiiiEiiiS4_E4callEPS2_S6_RNS0_7VariantERKiSC_SC_")]
pub fn stub_0x87431c() -> ! {
    todo!("0x87431c")
}

// 0x874360 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16),1>::BoundFuncDesc(void (RBX::MegaClusterInstance::*)(RBX::Region3int16),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EEC2EMS2_FvS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x874360() -> ! {
    todo!("0x874360")
}

// 0x8744d8 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x8744d8() -> ! {
    todo!("0x8744d8")
}

// 0x874508 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EED0Ev")]
pub fn stub_0x874508() {
    // IDA 0x874508: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8745dc — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x8745dc() -> ! {
    todo!("0x8745dc")
}

// 0x874620 — __ZN3RBX10Reflection9ArgHelper6getArgINS_12Region3int16ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Region3int16 RBX::Reflection::ArgHelper::getArg<RBX::Region3int16,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Region3int16> const&,boost::disable_if<boost::is_same<RBX::Region3int16,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_12Region3int16ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS4_EEPNS8_10disable_ifINS8_7is_sameIS4_NS8_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x874620() -> ! {
    todo!("0x874620")
}

// 0x8747f0 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EEC2EMS2_FbiiiEPKcS8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,bool ()(int,int,int),3>::BoundFuncDesc(bool (RBX::MegaClusterInstance::*)(int,int,int),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EEC2EMS2_FbiiiEPKcS8_S8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x8747f0() -> ! {
    todo!("0x8747f0")
}

// 0x874a08 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,bool ()(int,int,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EE16declareSignatureEPKcNS0_7VariantES6_S7_S6_S7_")]
pub fn stub_0x874a08() -> ! {
    todo!("0x874a08")
}

// 0x874a70 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,bool ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EED0Ev")]
pub fn stub_0x874a70() {
    // IDA 0x874a70: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x874b5c — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,bool ()(int,int,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x874b5c() -> ! {
    todo!("0x874b5c")
}

// 0x874bbc — __ZN3RBX10Reflection11Call3HelperINS_19MegaClusterInstanceEMS2_FbiiiEiiibE4callEPS2_S4_RNS0_7VariantERKiSA_SA_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::MegaClusterInstance,bool (RBX::MegaClusterInstance::*)(int,int,int),int,int,int,bool>::call(RBX::MegaClusterInstance*,bool (RBX::MegaClusterInstance::*)(int,int,int),RBX::Reflection::Variant &,int const&,int const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_19MegaClusterInstanceEMS2_FbiiiEiiibE4callEPS2_S4_RNS0_7VariantERKiSA_SA_")]
pub fn stub_0x874bbc() -> ! {
    todo!("0x874bbc")
}

// 0x874c00 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EEC2EMS2_FviiiS4_S5_EPKcSB_SB_SB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection),5>::BoundFuncDesc(void (RBX::MegaClusterInstance::*)(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection),char const*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EEC2EMS2_FviiiS4_S5_EPKcSB_SB_SB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x874c00() -> ! {
    todo!("0x874c00")
}

// 0x874ec4 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EE16declareSignatureEPKcNS0_7VariantES9_SA_S9_SA_S9_SA_S9_SA_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EE16declareSignatureEPKcNS0_7VariantES9_SA_S9_SA_S9_SA_S9_SA_")]
pub fn stub_0x874ec4() -> ! {
    todo!("0x874ec4")
}

// 0x874f64 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection),5>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EED0Ev")]
pub fn stub_0x874f64() {
    // IDA 0x874f64: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x875068 — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection),5>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x875068() -> ! {
    todo!("0x875068")
}

// 0x8750e8 — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel14WaterCellForceELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::WaterCellForce RBX::Reflection::ArgHelper::getArg<RBX::Voxel::WaterCellForce,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::WaterCellForce> const&,boost::disable_if<boost::is_same<RBX::Voxel::WaterCellForce,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel14WaterCellForceELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x8750e8() -> ! {
    todo!("0x8750e8")
}

// 0x87527c — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel18WaterCellDirectionELi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::WaterCellDirection RBX::Reflection::ArgHelper::getArg<RBX::Voxel::WaterCellDirection,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::WaterCellDirection> const&,boost::disable_if<boost::is_same<RBX::Voxel::WaterCellDirection,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel18WaterCellDirectionELi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x87527c() -> ! {
    todo!("0x87527c")
}

// 0x875410 — __ZN3RBX10Reflection9ArgHelper8try_enumILi5ENS_5Voxel18WaterCellDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<5,RBX::Voxel::WaterCellDirection>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::WaterCellDirection &,boost::enable_if<boost::is_enum<RBX::Voxel::WaterCellDirection>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi5ENS_5Voxel18WaterCellDirectionEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x875410() -> ! {
    todo!("0x875410")
}

// 0x875464 — __ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_5Voxel14WaterCellForceEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<4,RBX::Voxel::WaterCellForce>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::WaterCellForce &,boost::enable_if<boost::is_enum<RBX::Voxel::WaterCellForce>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_5Voxel14WaterCellForceEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x875464() -> ! {
    todo!("0x875464")
}

// 0x8754b8 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EEC2EMS2_FvS3_S5_S6_S7_EPKcSD_SD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),4>::BoundFuncDesc(void (RBX::MegaClusterInstance::*)(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EEC2EMS2_FvS3_S5_S6_S7_EPKcSD_SD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x8754b8() -> ! {
    todo!("0x8754b8")
}

// 0x875728 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_")]
pub fn stub_0x875728() -> ! {
    todo!("0x875728")
}

// 0x8757a8 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),4>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EED0Ev")]
pub fn stub_0x8757a8() {
    // IDA 0x8757a8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8758a0 — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16ENS_5Voxel12CellMaterialENS4_9CellBlockENS4_15CellOrientationEELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x8758a0() -> ! {
    todo!("0x8758a0")
}

// 0x87591c — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel12CellMaterialELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::CellMaterial RBX::Reflection::ArgHelper::getArg<RBX::Voxel::CellMaterial,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::CellMaterial> const&,boost::disable_if<boost::is_same<RBX::Voxel::CellMaterial,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel12CellMaterialELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x87591c() -> ! {
    todo!("0x87591c")
}

// 0x875ab0 — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel9CellBlockELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::CellBlock RBX::Reflection::ArgHelper::getArg<RBX::Voxel::CellBlock,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::CellBlock> const&,boost::disable_if<boost::is_same<RBX::Voxel::CellBlock,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel9CellBlockELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x875ab0() -> ! {
    todo!("0x875ab0")
}

// 0x875c44 — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel15CellOrientationELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::CellOrientation RBX::Reflection::ArgHelper::getArg<RBX::Voxel::CellOrientation,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::CellOrientation> const&,boost::disable_if<boost::is_same<RBX::Voxel::CellOrientation,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel15CellOrientationELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x875c44() -> ! {
    todo!("0x875c44")
}

// 0x875dd8 — __ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_5Voxel15CellOrientationEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<4,RBX::Voxel::CellOrientation>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::CellOrientation &,boost::enable_if<boost::is_enum<RBX::Voxel::CellOrientation>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_5Voxel15CellOrientationEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x875dd8() -> ! {
    todo!("0x875dd8")
}

// 0x875e2c — __ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_5Voxel9CellBlockEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::Voxel::CellBlock>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::CellBlock &,boost::enable_if<boost::is_enum<RBX::Voxel::CellBlock>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_5Voxel9CellBlockEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x875e2c() -> ! {
    todo!("0x875e2c")
}

// 0x875e80 — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_5Voxel12CellMaterialEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::Voxel::CellMaterial>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::CellMaterial &,boost::enable_if<boost::is_enum<RBX::Voxel::CellMaterial>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_5Voxel12CellMaterialEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x875e80() -> ! {
    todo!("0x875e80")
}

// 0x875ed4 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EEC2EMS2_FviiiS4_S5_S6_EPKcSC_SC_SC_SC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),6>::BoundFuncDesc(void (RBX::MegaClusterInstance::*)(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),char const*,char const*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EEC2EMS2_FviiiS4_S5_S6_EPKcSC_SC_SC_SC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x875ed4() -> ! {
    todo!("0x875ed4")
}

// 0x8761ec — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_SA_SB_SA_SB_SA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),6>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_SA_SB_SA_SB_SA_SB_")]
pub fn stub_0x8761ec() -> ! {
    todo!("0x8761ec")
}

// 0x8762ac — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),6>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EED0Ev")]
pub fn stub_0x8762ac() {
    // IDA 0x8762ac: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x87634c — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),6>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x87634c() -> ! {
    todo!("0x87634c")
}

// 0x8763d8 — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel12CellMaterialELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::CellMaterial RBX::Reflection::ArgHelper::getArg<RBX::Voxel::CellMaterial,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::CellMaterial> const&,boost::disable_if<boost::is_same<RBX::Voxel::CellMaterial,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel12CellMaterialELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x8763d8() -> ! {
    todo!("0x8763d8")
}

// 0x87656c — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel9CellBlockELi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::CellBlock RBX::Reflection::ArgHelper::getArg<RBX::Voxel::CellBlock,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::CellBlock> const&,boost::disable_if<boost::is_same<RBX::Voxel::CellBlock,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel9CellBlockELi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x87656c() -> ! {
    todo!("0x87656c")
}

// 0x876700 — __ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel15CellOrientationELi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "RBX::Voxel::CellOrientation RBX::Reflection::ArgHelper::getArg<RBX::Voxel::CellOrientation,6>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Voxel::CellOrientation> const&,boost::disable_if<boost::is_same<RBX::Voxel::CellOrientation,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_5Voxel15CellOrientationELi6EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0x876700() -> ! {
    todo!("0x876700")
}

// 0x876894 — __ZN3RBX10Reflection9ArgHelper8try_enumILi6ENS_5Voxel15CellOrientationEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<6,RBX::Voxel::CellOrientation>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::CellOrientation &,boost::enable_if<boost::is_enum<RBX::Voxel::CellOrientation>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi6ENS_5Voxel15CellOrientationEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x876894() -> ! {
    todo!("0x876894")
}

// 0x8768e8 — __ZN3RBX10Reflection9ArgHelper8try_enumILi5ENS_5Voxel9CellBlockEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<5,RBX::Voxel::CellBlock>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::CellBlock &,boost::enable_if<boost::is_enum<RBX::Voxel::CellBlock>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi5ENS_5Voxel9CellBlockEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x8768e8() -> ! {
    todo!("0x8768e8")
}

// 0x87693c — __ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_5Voxel12CellMaterialEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<4,RBX::Voxel::CellMaterial>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::Voxel::CellMaterial &,boost::enable_if<boost::is_enum<RBX::Voxel::CellMaterial>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi4ENS_5Voxel12CellMaterialEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_0x87693c() -> ! {
    todo!("0x87693c")
}

// 0x876990 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EEC2EMS2_FS7_iiiEPKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(int,int,int),3>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::MegaClusterInstance::*)(int,int,int),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EEC2EMS2_FS7_iiiEPKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x876990() -> ! {
    todo!("0x876990")
}

// 0x876ba8 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(int,int,int),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_")]
pub fn stub_0x876ba8() -> ! {
    todo!("0x876ba8")
}

// 0x876c10 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EED0Ev")]
pub fn stub_0x876c10() {
    // IDA 0x876c10: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x876cfc — __ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(int,int,int),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x876cfc() -> ! {
    todo!("0x876cfc")
}

// 0x876d5c — __ZN3RBX10Reflection11Call3HelperINS_19MegaClusterInstanceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEiiiEiiiS7_E4callEPS2_S9_RNS0_7VariantERKiSF_SF_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::MegaClusterInstance,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::MegaClusterInstance::*)(int,int,int),int,int,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::MegaClusterInstance*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::MegaClusterInstance::*)(int,int,int),RBX::Reflection::Variant &,int const&,int const&,int const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_19MegaClusterInstanceEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEiiiEiiiS7_E4callEPS2_S9_RNS0_7VariantERKiSF_SF_")]
pub fn stub_0x876d5c() -> ! {
    todo!("0x876d5c")
}

// 0x876e50 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EEC2IMS2_KFKS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::PropDescriptor<RBX::Region3int16 const (RBX::MegaClusterInstance::*)(void)const,int>(char const*,char const*,RBX::Region3int16 const (RBX::MegaClusterInstance::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EEC2IMS2_KFKS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x876e50() -> ! {
    todo!("0x876e50")
}

// 0x876f5c — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x876f5c() -> ! {
    todo!("0x876f5c")
}

// 0x877080 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EED0Ev")]
pub fn stub_0x877080() {
    // IDA 0x877080: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8770ac — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE10isReadOnlyEv")]
pub fn stub_0x8770ac() {
    // IDA 0x8770ac: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x8770bc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE11isWriteOnlyEv")]
pub fn stub_0x8770bc() {
    // IDA 0x8770bc: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x8770cc — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE11equalValuesEPKNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE11equalValuesEPKNS0_13DescribedBaseES6_")]
pub fn stub_0x8770cc() -> ! {
    todo!("0x8770cc")
}

// 0x877140 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x877140() -> ! {
    todo!("0x877140")
}

// 0x87716c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x87716c() -> ! {
    todo!("0x87716c")
}

// 0x8772d8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE9copyValueEPKNS0_13DescribedBaseEPS4_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EE9copyValueEPKNS0_13DescribedBaseEPS4_")]
pub fn stub_0x8772d8() -> ! {
    todo!("0x8772d8")
}

// 0x877300 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EED1Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EED1Ev")]
pub fn stub_0x877300() {
    // IDA 0x877300: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x877324 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Region3int16>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorINS_12Region3int16EED0Ev")]
pub fn stub_0x877324() {
    // IDA 0x877324: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x877350 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::GetImpl<RBX::Region3int16 const (RBX::MegaClusterInstance::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE10isReadOnlyEv")]
pub fn stub_0x877350() {
    // IDA 0x877350: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x877354 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::GetImpl<RBX::Region3int16 const (RBX::MegaClusterInstance::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE11isWriteOnlyEv")]
pub fn stub_0x877354() {
    // IDA 0x877354: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x877358 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::GetImpl<RBX::Region3int16 const (RBX::MegaClusterInstance::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x877358() -> ! {
    todo!("0x877358")
}

// 0x877380 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE8setValueEPNS0_13DescribedBaseERS6_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::GetImpl<RBX::Region3int16 const (RBX::MegaClusterInstance::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::Region3int16 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EE7GetImplIMS2_KFKS3_vEE8setValueEPNS0_13DescribedBaseERS6_")]
pub fn stub_0x877380() -> ! {
    todo!("0x877380")
}

// 0x8774a0 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsEC2IMS2_KFSsvEMS2_FvRKSsEEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::PropDescriptor<std::string (RBX::MegaClusterInstance::*)(void)const,void (RBX::MegaClusterInstance::*)(std::string const&)>(char const*,char const*,std::string (RBX::MegaClusterInstance::*)(void)const,void (RBX::MegaClusterInstance::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsEC2IMS2_KFSsvEMS2_FvRKSsEEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x8774a0() -> ! {
    todo!("0x8774a0")
}

// 0x8775b4 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsED0Ev")]
pub fn stub_0x8775b4() {
    // IDA 0x8775b4: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x8775e0 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::GetSetImpl<std::string (RBX::MegaClusterInstance::*)(void)const,void (RBX::MegaClusterInstance::*)(std::string const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE10isReadOnlyEv")]
pub fn stub_0x8775e0() -> bool {
    // IDA 0x8775e0: GetSetImpl::isReadOnly -- hardcoded `return 0` (decompiled 0xfcb4/0x106b4/0x1084c). Get/set-bound props are never read-only.
    false
}

// 0x8775e4 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::GetSetImpl<std::string (RBX::MegaClusterInstance::*)(void)const,void (RBX::MegaClusterInstance::*)(std::string const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE11isWriteOnlyEv")]
pub fn stub_0x8775e4() -> bool {
    // IDA 0x8775e4: GetSetImpl::isWriteOnly -- hardcoded `return 0` (decompiled 0xfcb8/0x106b8/0x10850). Get/set-bound props are never write-only.
    false
}

// 0x8775e8 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::GetSetImpl<std::string (RBX::MegaClusterInstance::*)(void)const,void (RBX::MegaClusterInstance::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x8775e8() -> ! {
    todo!("0x8775e8")
}

// 0x877610 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::GetSetImpl<std::string (RBX::MegaClusterInstance::*)(void)const,void (RBX::MegaClusterInstance::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE10GetSetImplIMS2_KFSsvEMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES8_")]
pub fn stub_0x877610() -> ! {
    todo!("0x877610")
}

// 0x877634 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsEC2IiMS2_FvRKSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::PropDescriptor<int,void (RBX::MegaClusterInstance::*)(std::string const&)>(char const*,char const*,int,void (RBX::MegaClusterInstance::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsEC2IiMS2_FvRKSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x877634() -> ! {
    todo!("0x877634")
}

// 0x877740 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::SetImpl<void (RBX::MegaClusterInstance::*)(std::string const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv")]
pub fn stub_0x877740() {
    // IDA 0x877740: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x877744 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::SetImpl<void (RBX::MegaClusterInstance::*)(std::string const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv")]
pub fn stub_0x877744() {
    // IDA 0x877744: isReadOnly/isWriteOnly -- virtual forward to the bound member descriptor (cf. EnumPropDescriptor shape at decompiled 0x10064/0x10074). Member descriptors unmodeled: cutover no-op.
}

// 0x877748 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::SetImpl<void (RBX::MegaClusterInstance::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x877748() -> ! {
    todo!("0x877748")
}

// 0x877868 — __ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::SetImpl<void (RBX::MegaClusterInstance::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_")]
pub fn stub_0x877868() -> ! {
    todo!("0x877868")
}

// 0x879970 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation),6>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel12CellMaterialENS3_9CellBlockENS3_15CellOrientationEELi6EED2Ev")]
pub fn stub_0x879970() {
    // IDA 0x879970: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}
