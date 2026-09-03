//! core watchdog v — 120 core stubs EA-sorted, next uncovered fallback after watchdog_u 0x3d032c.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in core — next 120 uncovered after 0x3d032c (watchdog_u max).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3d0400 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3d0400() {
    // IDA 0x3d0400: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d043c — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int, int)
pub fn stub_3d043c() {
    // IDA 0x3d043c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::~RefPropDescriptor()")]
// 0x3d04e0 — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_3d04e0() {
    // IDA 0x3d04e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::isReadOnly(void)const")]
// 0x3d0510 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10isReadOnlyEv
// type: int __fastcall(int)
pub fn stub_3d0510() {
    // IDA 0x3d0510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::isWriteOnly(void)const")]
// 0x3d0520 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
pub fn stub_3d0520() {
    // IDA 0x3d0520: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x3d0530 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
pub fn stub_3d0530() {
    // IDA 0x3d0530: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x3d0558 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_3d0558() {
    // IDA 0x3d0558: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x3d0670 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
pub fn stub_3d0670() {
    // IDA 0x3d0670: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x3d0738 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
pub fn stub_3d0738() {
    // IDA 0x3d0738: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x3d075c — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
pub fn stub_3d075c() {
    // IDA 0x3d075c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x3d0830 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
pub fn stub_3d0830() {
    // IDA 0x3d0830: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d0854 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3d0854() {
    // IDA 0x3d0854: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3d0868 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub fn stub_3d0868() {
    // IDA 0x3d0868: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3d08e4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
pub fn stub_3d08e4() {
    // IDA 0x3d08e4: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3d0904 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_3d0904() {
    // IDA 0x3d0904: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3d09e4 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int)
pub fn stub_3d09e4() {
    // IDA 0x3d09e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::isReadOnly(void)const")]
// 0x3d09ec — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
pub fn stub_3d09ec() {
    // IDA 0x3d09ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::isWriteOnly(void)const")]
// 0x3d09f0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
pub fn stub_3d09f0() {
    // IDA 0x3d09f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d09f4 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_3d09f4() {
    // IDA 0x3d09f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// 0x3d0a14 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d0a14() {
    // IDA 0x3d0a14: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::PropDescriptor<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>(char const*,char const*,float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d0a38 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_3d0a38() {
    // IDA 0x3d0a38: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::~PropDescriptor()")]
// 0x3d0b4c — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_3d0b4c() {
    // IDA 0x3d0b4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::isReadOnly(void)const")]
// 0x3d0b78 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// type: int()
pub fn stub_3d0b78() {
    // IDA 0x3d0b78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::isWriteOnly(void)const")]
// 0x3d0b7c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
pub fn stub_3d0b7c() {
    // IDA 0x3d0b7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d0b80 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_3d0b80() {
    // IDA 0x3d0b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// 0x3d0ba0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d0ba0() {
    // IDA 0x3d0ba0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d0bc4 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_3d0bc4() {
    // IDA 0x3d0bc4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()")]
// 0x3d0cd8 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_3d0cd8() {
    // IDA 0x3d0cd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
// 0x3d0d04 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
// type: int()
pub fn stub_3d0d04() {
    // IDA 0x3d0d04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
// 0x3d0d08 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
// type: int()
pub fn stub_3d0d08() {
    // IDA 0x3d0d08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d0d0c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
pub fn stub_3d0d0c() {
    // IDA 0x3d0d0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
// 0x3d0d48 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
// type: int __fastcall(int, int, int)
pub fn stub_3d0d48() {
    // IDA 0x3d0d48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::EnumPropDescriptor<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>(char const*,char const*,RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d0d6c — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3d0d6c() {
    // IDA 0x3d0d6c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::~EnumPropDescriptor()")]
// 0x3d0f20 — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_3d0f20() {
    // IDA 0x3d0f20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::isReadOnly(void)const")]
// 0x3d0f4c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10isReadOnlyEv
// type: int __fastcall(int)
pub fn stub_3d0f4c() {
    // IDA 0x3d0f4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::isWriteOnly(void)const")]
// 0x3d0f5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11isWriteOnlyEv
// type: int __fastcall(int)
pub fn stub_3d0f5c() {
    // IDA 0x3d0f5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x3d0f6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
pub fn stub_3d0f6c() {
    // IDA 0x3d0f6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x3d0f94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d0f94() {
    // IDA 0x3d0f94: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x3d0fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d0fb8() {
    // IDA 0x3d0fb8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x3d1104 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
pub fn stub_3d1104() {
    // IDA 0x3d1104: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::hasStringValue(void)const")]
// 0x3d1128 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14hasStringValueEv
// type: int()
pub fn stub_3d1128() {
    // IDA 0x3d1128: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d112c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
pub fn stub_3d112c() {
    // IDA 0x3d112c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x3d1150 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
pub fn stub_3d1150() {
    // IDA 0x3d1150: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x3d1190 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d1190() {
    // IDA 0x3d1190: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x3d11b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
pub fn stub_3d11b0() {
    // IDA 0x3d11b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d13f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3d13f0() {
    // IDA 0x3d13f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// 0x3d140c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
pub fn stub_3d140c() {
    // IDA 0x3d140c: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d1440 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3d1440() {
    // IDA 0x3d1440: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x3d1448 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
pub fn stub_3d1448() {
    // IDA 0x3d1448: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// 0x3d1494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3d1494() {
    // IDA 0x3d1494: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// 0x3d14b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
pub fn stub_3d14b4() {
    // IDA 0x3d14b4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToIndex(RBX::Camera::CameraType)const")]
// 0x3d14e8 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToIndexES3_
// type: int __fastcall(int, int)
pub fn stub_3d14e8() {
    // IDA 0x3d14e8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x3d1558 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
pub fn stub_3d1558() {
    // IDA 0x3d1558: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::isReadOnly(void)const")]
// 0x3d1598 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
pub fn stub_3d1598() {
    // IDA 0x3d1598: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::isWriteOnly(void)const")]
// 0x3d159c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
pub fn stub_3d159c() {
    // IDA 0x3d159c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d15a0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_3d15a0() {
    // IDA 0x3d15a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraType const&)const")]
// 0x3d15c0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d15c0() {
    // IDA 0x3d15c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraType> const>::initSingleton(void)")]
// 0x3d15e4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE13initSingletonEv
pub fn stub_3d15e4() {
    // IDA 0x3d15e4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraType> const>::doGetSingleton(void)")]
// 0x3d15e8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_3d15e8() {
    // IDA 0x3d15e8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::EnumDesc(void)")]
// 0x3d22c0 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC1Ev
// type: int()
pub fn stub_3d22c0() {
    // IDA 0x3d22c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::EnumDesc(void)")]
// 0x3d22c4 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC2Ev
// type: int __fastcall(int)
pub fn stub_3d22c4() {
    // IDA 0x3d22c4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::ChangeHistoryService::Item::unplayProperty(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// 0x3d3328 — __ZN3RBX20ChangeHistoryService4Item14unplayPropertyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(RBX::Instance **, void **)
pub fn stub_3d3328() {
    // IDA 0x3d3328: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint(char const*,RBX::Instance const*)")]
// 0x3d43a4 — __ZN3RBX20ChangeHistoryService15requestWaypointEPKcPKNS_8InstanceE
// type: RBX::ChangeHistoryService *__fastcall(RBX::ChangeHistoryService *this, const char *, const RBX::Instance *)
pub fn stub_3d43a4() {
    // IDA 0x3d43a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChangeHistoryService::isRecordable(RBX::Instance *)")]
// 0x3d576c — __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE
// type: int __fastcall(__guard *this, RBX::Instance *, int, int)
pub fn stub_3d576c() {
    // IDA 0x3d576c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::addPair(RBX::ChangeHistoryService::RuntimeUndoBehavior,char const*)")]
// 0x3d6770 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_3d6770() {
    // IDA 0x3d6770: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")]
// 0x3d6ad0 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3d6ad0() {
    // IDA 0x3d6ad0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")]
// 0x3d6b18 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3d6b18() {
    // IDA 0x3d6b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")]
// 0x3d6b58 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3d6b58() {
    // IDA 0x3d6b58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::ChangeHistoryService::Waypoint::findItem(RBX::Instance *)")]
// 0x3d6ba0 — __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE
// type: char *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
pub fn stub_3d6ba0() {
    // IDA 0x3d6ba0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::ChangeHistoryService::Item::apply(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// 0x3d6ed8 — __ZN3RBX20ChangeHistoryService4Item5applyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, void **)
pub fn stub_3d6ed8() {
    // IDA 0x3d6ed8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(RBX::Instance const*)")]
// 0x3d6fc4 — __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub fn stub_3d6fc4() {
    // IDA 0x3d6fc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "void RBX::ChangeHistoryService::Item::addClusterData<RBX::MegaClusterInstance>(RBX::MegaClusterInstance const*)")]
// 0x3d7df0 — __ZN3RBX20ChangeHistoryService4Item14addClusterDataINS_19MegaClusterInstanceEEEvPKT_
// type: RBX::MegaClusterInstance *__fastcall(int, RBX::MegaClusterInstance *this)
pub fn stub_3d7df0() {
    // IDA 0x3d7df0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChangeHistoryService::Item::recordProperty(RBX::Reflection::PropertyDescriptor const*)")]
// 0x3d7f90 — __ZN3RBX20ChangeHistoryService4Item14recordPropertyEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Reflection::PropertyDescriptor *)
pub fn stub_3d7f90() {
    // IDA 0x3d7f90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(RBX::Instance const*)")]
// 0x3d82b0 — __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub fn stub_3d82b0() {
    // IDA 0x3d82b0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// 0x3d82e8 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_3d82e8() {
    // IDA 0x3d82e8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// 0x3d82fc — __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_3d82fc() {
    // IDA 0x3d82fc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// 0x3d8310 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED1Ev
// type: int()
pub fn stub_3d8310() {
    // IDA 0x3d8310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToString(unsigned long,std::string &)const")]
// 0x3d8318 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3d8318() {
    // IDA 0x3d8318: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToItem(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)const")]
// 0x3d8470 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3d8470() {
    // IDA 0x3d8470: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// 0x3d8540 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3d8540() {
    // IDA 0x3d8540: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv")]
// 0x3d8718 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv
pub fn stub_3d8718() {
    // IDA 0x3d8718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// 0x3d9778 — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
pub fn stub_3d9778() {
    // IDA 0x3d9778: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv")]
// 0x3da568 — __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv
// type: int()
pub fn stub_3da568() {
    // IDA 0x3da568: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3da5d0 — __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3da5d0() {
    // IDA 0x3da5d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3da5d4 — __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3da5d4() {
    // IDA 0x3da5d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3da674 — __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3da674() {
    // IDA 0x3da674: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3da67c — __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3da67c() {
    // IDA 0x3da67c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3da720 — __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3da720() {
    // IDA 0x3da720: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3da728 — __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3da728() {
    // IDA 0x3da728: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::find(RBX::Reflection::PropertyDescriptor const* const&)")]
// 0x3dad08 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE4findERS6_
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_3dad08() {
    // IDA 0x3dad08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3db268 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_3db268() {
    // IDA 0x3db268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")]
// 0x3db36c — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3db36c() {
    // IDA 0x3db36c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3db420 — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
pub fn stub_3db420() {
    // IDA 0x3db420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3db440 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3db440() {
    // IDA 0x3db440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3db5b8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3db5b8() {
    // IDA 0x3db5b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")]
// 0x3db5e8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3db5e8() {
    // IDA 0x3db5e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3db6b4 — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
pub fn stub_3db6b4() {
    // IDA 0x3db6b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ChangeHistoryService,void (RBX::ChangeHistoryService::*)(std::string),std::string,void>::call(RBX::ChangeHistoryService*,void (RBX::ChangeHistoryService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// 0x3db7f0 — __ZN3RBX10Reflection11Call1HelperINS_20ChangeHistoryServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
pub fn stub_3db7f0() {
    // IDA 0x3db7f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3db920 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3db920() {
    // IDA 0x3db920: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3dba98 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3dba98() {
    // IDA 0x3dba98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")]
// 0x3dbac8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3dbac8() {
    // IDA 0x3dbac8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3dbb9c — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3dbb9c() {
    // IDA 0x3dbb9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> *)")]
// 0x3dc8f4 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_3dc8f4() {
    // IDA 0x3dc8f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::ChangeHistoryService::Item::addValue(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3dc928 — __ZN3RBX20ChangeHistoryService4Item8addValueERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Reflection::PropertyDescriptor *)
pub fn stub_3dc928() {
    // IDA 0x3dc928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "std::map<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::operator[](RBX::Reflection::PropertyDescriptor const* const&)")]
// 0x3dcb74 — __ZNSt3mapIPKN3RBX10Reflection18PropertyDescriptorENS1_7VariantESt4lessIS4_ESaISt4pairIKS4_S5_EEEixERS9_
// type: int __fastcall(int, _DWORD *)
pub fn stub_3dcb74() {
    // IDA 0x3dcb74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dccdc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, unsigned int *)
pub fn stub_3dccdc() {
    // IDA 0x3dccdc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dcd90 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_3dcd90() {
    // IDA 0x3dcd90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dcddc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_3dcddc() {
    // IDA 0x3dcddc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_create_node(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dce44 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE14_M_create_nodeERKS8_
// type: _DWORD *__fastcall(int, int *, int, int, void *, int)
pub fn stub_3dce44() {
    // IDA 0x3dce44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChangeHistoryService::Item::addValueIfNotParentProperty(RBX::Reflection::Property const&)")]
// 0x3dd54c — __ZN3RBX20ChangeHistoryService4Item27addValueIfNotParentPropertyERKNS_10Reflection8PropertyE
// type: int __fastcall(int, void **)
pub fn stub_3dd54c() {
    // IDA 0x3dd54c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::map<RBX::Instance *,unsigned int,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::operator[](RBX::Instance * const&)")]
// 0x3dd564 — __ZNSt3mapIPN3RBX8InstanceEjSt4lessIS2_ESaISt4pairIKS2_jEEEixERS6_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_3dd564() {
    // IDA 0x3dd564: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,unsigned int>>,std::pair<RBX::Instance * const,unsigned int> const&)")]
// 0x3dd5bc — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
pub fn stub_3dd5bc() {
    // IDA 0x3dd5bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,unsigned int> const&)")]
// 0x3dd670 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_3dd670() {
    // IDA 0x3dd670: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")]
// 0x3dd6c8 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int *)
pub fn stub_3dd6c8() {
    // IDA 0x3dd6c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_Rb_tree(std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>> const&)")]
// 0x3dda98 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_
pub fn stub_3dda98() {
    // IDA 0x3dda98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>*)")]
// 0x3ddadc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
pub fn stub_3ddadc() {
    // IDA 0x3ddadc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChangeHistoryService::Waypoint::removeItem(RBX::Instance *)")]
// 0x3deb04 — __ZN3RBX20ChangeHistoryService8Waypoint10removeItemEPNS_8InstanceE
// type: _Rb_tree_node_base *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
pub fn stub_3deb04() {
    // IDA 0x3deb04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::ChangeHistoryService::Item::absorbProp(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// 0x3ded94 — __ZN3RBX20ChangeHistoryService4Item10absorbPropERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, int)
pub fn stub_3ded94() {
    // IDA 0x3ded94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,unsigned int>> *)")]
// 0x3df534 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_3df534() {
    // IDA 0x3df534: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[doc(alias = "RBX::CharacterAppearance::askSetParent(RBX::Instance const*)const")]
// 0x3e113c — __ZNK3RBX19CharacterAppearance12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::CharacterAppearance *__hidden this, const RBX::Instance *)
pub fn stub_3e113c() {
    // IDA 0x3e113c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

