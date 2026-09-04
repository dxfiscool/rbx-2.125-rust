//! core watchdog d — 150 core stubs EA-sorted, gap filler after 0x3d032c (watchdog_c max).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in core — next 150 uncovered after 0x3d032c.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
/// Batch 1: Camera descriptor vtables (IDA-grounded addresses). The
/// `RefPropDescriptor<Camera, Instance>` C2 (0x3d043c) installs `*a1 =
/// &off_1240708` (`__ZTV...RefPropDescriptorINS_6CameraENS_8InstanceEEE`
/// 0x1240700 + 8) and `a1[10] = &off_124075C` (+0x5C); its `GetSetImpl` box
/// installs `off_12407C8` (GetSetImpl vtable 0x12407c0 + 8). The
/// `PropDescriptor<Camera, float>` C2 (0x3d0a38) installs `off_1240848`
/// (0x1240840 + 8) with an `off_12408A8` GetSetImpl box; its D0 (0x3d0b4c)
/// restores `off_1270A68` (TypedPropertyDescriptor<float> 0x1270a60 + 8).
/// The `PropDescriptor<Camera, CoordinateFrame>` C2 (0x3d0bc4) installs
/// `off_12408D8` (0x12408d0 + 8) with an `off_1240948` box; its D0 (0x3d0cd8)
/// restores `off_1270DA8` (TypedPropertyDescriptor<CoordinateFrame> + 8).
pub mod camera_desc {
    /// was: `RefPropDescriptor<Camera, Instance>` vtable (IDA 0x3d043c/0x3d04e0).
    pub const CAM_REF_DESC_VTAB: &str = "off_1240708"; // IDA 0x3d043c
    /// was: `RefPropDescriptor<Camera, Instance>` sub-table (IDA 0x3d043c/0x3d04e0).
    pub const CAM_REF_SUB_VTAB: &str = "off_124075C"; // IDA 0x3d043c
    /// was: `GetSetImpl<Instance*, void(Instance*)>` vtable (IDA 0x3d043c).
    pub const CAM_INST_GETSET_VTAB: &str = "off_12407C8"; // IDA 0x3d043c
    /// was: `PropDescriptor<Camera, float>` vtable (IDA 0x3d0a38).
    pub const CAM_FLOAT_DESC_VTAB: &str = "off_1240848"; // IDA 0x3d0a38
    /// was: `GetSetImpl<float, void(float)>` vtable (IDA 0x3d0a38).
    pub const CAM_FLOAT_GETSET_VTAB: &str = "off_12408A8"; // IDA 0x3d0a38
    /// was: `TypedPropertyDescriptor<float>` vtable (IDA 0x3d0b4c).
    pub const TYPED_FLOAT_VTAB: &str = "off_1270A68"; // IDA 0x3d0b4c
    /// was: `PropDescriptor<Camera, CoordinateFrame>` vtable (IDA 0x3d0bc4).
    pub const CAM_CF_DESC_VTAB: &str = "off_12408D8"; // IDA 0x3d0bc4
    /// was: `GetSetImpl<CoordinateFrame, void(CoordinateFrame)>` vtable (IDA 0x3d0bc4).
    pub const CAM_CF_GETSET_VTAB: &str = "off_1240948"; // IDA 0x3d0bc4
    /// was: `TypedPropertyDescriptor<CoordinateFrame>` vtable (IDA 0x3d0cd8).
    pub const TYPED_CF_VTAB: &str = "off_1270DA8"; // IDA 0x3d0cd8
}


#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Camera,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3d0400 — __ZNK3RBX10Reflection13BoundFuncDescINS_6CameraEFvfELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3d0400(
    desc: &crate::generated_core_watchdog_c::camera_enum::BoundFuncDescriptor,
    obj: *const u8,
    arg: f32,
    callee: &dyn crate::generated_core_watchdog_c::camera_boundfunc::FnCameraFloatVoid,
) {
    // IDA 0x3d0400: derived-36 + member pair + getArg<float,1> + member
    // tail-call (float forwarded in R1 — same template as 0x3cf4f4/0x3cfc6c).
    crate::generated_core_watchdog_c::camera_boundfunc::execute_float_void(desc, obj, arg, callee)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::RefPropDescriptor<RBX::Instance* (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance*)>(char const*,char const*,RBX::Instance* (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d043c — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int, int, int)
pub unsafe fn stub_3d043c(
    slot: *mut crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    name: *const std::os::raw::c_char,
    category: *const std::os::raw::c_char,
    getter_func: usize,
    getter_adj: usize,
    setter_func: usize,
    setter_adj: usize,
    attr0: u32,
    attr1: u32,
    attr2: u32,
    permissions: u32,
) -> *mut crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor {
    // IDA 0x3d043c: classDescriptor ensure + RefType<Instance*> singleton +
    // PropertyDescriptor base init + off_1240708/off_124075C + new
    // GetSetImpl(off_12407C8, {getfunc, getadj, setfunc, setadj}) into a1[11].
    // Same template as the Billboard C2 (0x3c2560) with Camera vtables.
    use crate::generated_core_watchdog_c::billboard_prop::MemberPtr;
    crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor::construct_with(
        slot,
        camera_desc::CAM_REF_DESC_VTAB,
        camera_desc::CAM_REF_SUB_VTAB,
        name,
        category,
        MemberPtr { func: getter_func, adj: getter_adj },
        MemberPtr { func: setter_func, adj: setter_adj },
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::~RefPropDescriptor()")]
// 0x3d04e0 — __ZN3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_3d04e0(
    slot: *mut crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
) {
    // IDA 0x3d04e0 D0: off_1240708 + off_124075C, delete the a1[11] box
    // (+ operator delete — slot stays caller-owned here).
    crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor::destroy_with(
        slot,
        camera_desc::CAM_REF_DESC_VTAB,
        camera_desc::CAM_REF_SUB_VTAB,
    )
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::isReadOnly(void)const")]
// 0x3d0510 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10isReadOnlyEv
// type: int __fastcall(int)
pub unsafe fn stub_3d0510(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
) -> bool {
    // IDA 0x3d0510: box +0 slot (GetSetImpl::isReadOnly) — return 0.
    crate::generated_core_watchdog_c::billboard_ref::is_read_only(desc)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::isWriteOnly(void)const")]
// 0x3d0520 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11isWriteOnlyEv
// type: int __fastcall(int)
pub unsafe fn stub_3d0520(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
) -> bool {
    // IDA 0x3d0520: box +4 slot (GetSetImpl::isWriteOnly) — return 0.
    crate::generated_core_watchdog_c::billboard_ref::is_write_only(desc)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x3d0530 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
pub unsafe fn stub_3d0530(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    a: *const u8,
    b: *const u8,
) -> bool {
    // IDA 0x3d0530: getValue(box, a2) == getValue(box, a3).
    crate::generated_core_watchdog_c::billboard_ref::equal_values(desc, a, b)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x3d0558 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub unsafe fn stub_3d0558(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *const u8,
) -> crate::generated_core_watchdog_c::billboard_ref::RefVariant {
    // IDA 0x3d0558: getValue + shared_from<Instance> + derived+36 +
    // DescribedBase-shared-ptr singleton + placement_any (refcount traffic no-op).
    crate::generated_core_watchdog_c::billboard_ref::get_variant(desc, obj)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x3d0670 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: void __fastcall(int, int, int)
pub unsafe fn stub_3d0670(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    variant: &crate::generated_core_watchdog_c::billboard_ref::RefVariant,
) {
    // IDA 0x3d0670: Variant::get<shared_ptr<DescribedBase>> + desc+0x40 setter
    // (base-36 back to derived for the box setter).
    crate::generated_core_watchdog_c::billboard_ref::set_variant(desc, obj, variant)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x3d0738 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3d0738(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    src: *const u8,
    dst: *mut u8,
) {
    // IDA 0x3d0738: v6 = getValue(box, a2); setValue(box, a3, &v6).
    crate::generated_core_watchdog_c::billboard_ref::copy_value(desc, src, dst)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x3d075c — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
pub unsafe fn stub_3d075c(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *const u8,
    xml: *mut crate::generated_core_watchdog_c::billboard_ref::XmlElement,
) {
    // IDA 0x3d075c: getValue + derived+36 + InstanceHandle +
    // XmlNameValuePair::setValue(a3 + 12).
    crate::generated_core_watchdog_c::billboard_ref::write_value(desc, obj, xml)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x3d0830 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE10readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, int, int)
pub unsafe fn stub_3d0830(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    xml: *const crate::generated_core_watchdog_c::billboard_ref::XmlElement,
    binder: &dyn crate::generated_core_watchdog_c::billboard_ref::ReferenceBinder,
) -> i32 {
    // IDA 0x3d0830: if (a3) a3 += 12; binder[4](a4, a3, a2, a1 + 40) tail-call.
    crate::generated_core_watchdog_c::billboard_ref::read_value(desc, obj, xml, binder)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d0854 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub unsafe fn stub_3d0854(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *const u8,
) -> *const u8 {
    // IDA 0x3d0854: result = getValue(box); if (result) result += 36.
    crate::generated_core_watchdog_c::billboard_ref::get_ref_value(desc, obj)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3d0868 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
pub unsafe fn stub_3d0868(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    src: *const u8,
    is_instance: fn(*const u8) -> bool,
) -> Result<(), crate::generated_core_watchdog_c::billboard_ref::BadCast> {
    // IDA 0x3d0868: null passes through; non-null __dynamic_casts
    // DescribedBase -> Instance (bad_cast throw on failure); setValue.
    crate::generated_core_watchdog_c::billboard_ref::set_ref_value(desc, obj, src, is_instance)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// 0x3d08e4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
pub unsafe fn stub_3d08e4(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    src: *const u8,
) {
    // IDA 0x3d08e4: v3 = a3 ? a3 - 36 : 0 (no cast); setValue(box, a2, &v5).
    crate::generated_core_watchdog_c::billboard_ref::set_ref_value_unsafe(desc, obj, src)
}

#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3d0904 — __ZNK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: void __fastcall(int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub unsafe fn stub_3d0904(
    desc: &crate::generated_core_watchdog_c::billboard_ref::RefPropDescriptor,
    obj: *mut u8,
    handle: *const crate::generated_core_watchdog_c::billboard_ref::InstanceHandle,
) {
    // IDA 0x3d0904: v22 = pi ? pi - 36; setValue(box, a2, &v24).
    crate::generated_core_watchdog_c::billboard_ref::assign_idref(desc, obj, handle)
}

#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::Camera,RBX::Instance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// 0x3d09e4 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_6CameraENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int)
pub unsafe fn stub_3d09e4(
    this_adj: *const u8,
    obj: *mut u8,
    handle: *const crate::generated_core_watchdog_c::billboard_ref::InstanceHandle,
) {
    // IDA 0x3d09e4 Thn40: R0 -= 40, tail-jump to assignIDREF.
    crate::generated_core_watchdog_c::billboard_ref::assign_idref_thunk(this_adj, obj, handle)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::isReadOnly(void)const")]
// 0x3d09ec — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int()
pub fn stub_3d09ec() -> bool {
    // IDA 0x3d09ec: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::isWriteOnly(void)const")]
// 0x3d09f0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int()
pub fn stub_3d09f0() -> bool {
    // IDA 0x3d09f0: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d09f4 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_3d09f4(
    imp: *const crate::generated_core_watchdog_c::billboard_prop::GetSetImpl,
    obj: *const u8,
) -> *mut u8 {
    // IDA 0x3d09f4: DescribedBase-36, (adj >> 1) adjust, virtual branch, tail-call getter.
    crate::generated_core_watchdog_c::billboard_ref::get_instance(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Instance *>::GetSetImpl<RBX::Instance * (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Instance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::Instance * const&)const")]
// 0x3d0a14 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEPNS_8InstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_3d0a14(
    imp: *const crate::generated_core_watchdog_c::billboard_prop::GetSetImpl,
    obj: *mut u8,
    value: *mut u8,
) {
    // IDA 0x3d0a14: setter(this, *a3); void result.
    crate::generated_core_watchdog_c::billboard_ref::set_instance(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::PropDescriptor<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>(char const*,char const*,float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d0a38 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_3d0a38(
    slot: *mut crate::generated_core_watchdog_c::billboard_prop::PropDescriptor,
    name: *const std::os::raw::c_char,
    category: *const std::os::raw::c_char,
    getter_func: usize,
    getter_adj: usize,
    setter_func: usize,
    setter_adj: usize,
    attr0: u32,
    attr1: u32,
    attr2: u32,
    permissions: u32,
) -> *mut crate::generated_core_watchdog_c::billboard_prop::PropDescriptor {
    // IDA 0x3d0a38: GetSetImpl vtable off_12408A8, descriptor vtable
    // off_1240848, TypedPropertyDescriptor<float> base init.
    // Same template as the Billboard UDim2 C2 (0x3c207c).
    use crate::generated_core_watchdog_c::billboard_prop::MemberPtr;
    crate::generated_core_watchdog_c::billboard_prop::PropDescriptor::construct(
        slot,
        name,
        category,
        MemberPtr { func: getter_func, adj: getter_adj },
        MemberPtr { func: setter_func, adj: setter_adj },
        camera_desc::CAM_FLOAT_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::~PropDescriptor()")]
// 0x3d0b4c — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEfED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_3d0b4c(
    slot: *mut crate::generated_core_watchdog_c::billboard_prop::PropDescriptor,
) {
    // IDA 0x3d0b4c: *a1 = &off_1270A68 (TypedPropertyDescriptor<float>);
    // delete a1[10]; delete a1 (slot stays caller-owned here).
    crate::generated_core_watchdog_c::billboard_prop::PropDescriptor::destroy(
        slot,
        camera_desc::TYPED_FLOAT_VTAB,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::isReadOnly(void)const")]
// 0x3d0b78 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
// type: int()
pub fn stub_3d0b78() -> bool {
    // IDA 0x3d0b78: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::isWriteOnly(void)const")]
// 0x3d0b7c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
// type: int()
pub fn stub_3d0b7c() -> bool {
    // IDA 0x3d0b7c: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d0b80 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub unsafe fn stub_3d0b80(
    imp: *const crate::generated_core_watchdog_c::billboard_prop::GetSetImpl,
    obj: *const u8,
) -> f32 {
    // IDA 0x3d0b80: DescribedBase-36, (adj >> 1) adjust, virtual branch, tail-call getter.
    crate::generated_core_watchdog_c::billboard_prop::get_float(&*imp, obj)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,float>::GetSetImpl<float (RBX::Camera::*)(void)const,void (RBX::Camera::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// 0x3d0ba0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
// type: int __fastcall(int, int, _DWORD *)
pub unsafe fn stub_3d0ba0(
    imp: *const crate::generated_core_watchdog_c::billboard_prop::GetSetImpl,
    obj: *mut u8,
    value: f32,
) {
    // IDA 0x3d0ba0: LDR R1, [R2] deref, then setter(this, value); void result.
    crate::generated_core_watchdog_c::billboard_prop::set_float(&*imp, obj, value)
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d0bc4 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
pub unsafe fn stub_3d0bc4(
    slot: *mut crate::generated_core_watchdog_c::billboard_prop::PropDescriptor,
    name: *const std::os::raw::c_char,
    category: *const std::os::raw::c_char,
    getter_func: usize,
    getter_adj: usize,
    setter_func: usize,
    setter_adj: usize,
    attr0: u32,
    attr1: u32,
    attr2: u32,
    permissions: u32,
) -> *mut crate::generated_core_watchdog_c::billboard_prop::PropDescriptor {
    // IDA 0x3d0bc4: GetSetImpl vtable off_1240948, descriptor vtable
    // off_12408D8, TypedPropertyDescriptor<CoordinateFrame> base init.
    // Same template as the float C2 (0x3d0a38).
    use crate::generated_core_watchdog_c::billboard_prop::MemberPtr;
    crate::generated_core_watchdog_c::billboard_prop::PropDescriptor::construct(
        slot,
        name,
        category,
        MemberPtr { func: getter_func, adj: getter_adj },
        MemberPtr { func: setter_func, adj: setter_adj },
        camera_desc::CAM_CF_DESC_VTAB,
        attr0,
        attr1,
        attr2,
        permissions,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::~PropDescriptor()")]
// 0x3d0cd8 — __ZN3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEED0Ev
// type: int __fastcall(_DWORD *)
pub unsafe fn stub_3d0cd8(
    slot: *mut crate::generated_core_watchdog_c::billboard_prop::PropDescriptor,
) {
    // IDA 0x3d0cd8: *a1 = &off_1270DA8 (TypedPropertyDescriptor<CoordinateFrame>);
    // delete a1[10]; delete a1 (slot stays caller-owned here).
    crate::generated_core_watchdog_c::billboard_prop::PropDescriptor::destroy(
        slot,
        camera_desc::TYPED_CF_VTAB,
    )
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
// 0x3d0d04 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
// type: int()
pub fn stub_3d0d04() -> bool {
    // IDA 0x3d0d04: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
// 0x3d0d08 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
// type: int()
pub fn stub_3d0d08() -> bool {
    // IDA 0x3d0d08: return 0.
    false
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d0d0c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
pub fn stub_3d0d0c() -> ! {
    todo!("0x3d0d0c __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Camera::*)(void)const,void (RBX::Camera::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
// 0x3d0d48 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
// type: int __fastcall(int, int, int)
pub fn stub_3d0d48() -> ! {
    todo!("0x3d0d48 __ZNK3RBX10Reflection14PropDescriptorINS_6CameraEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::EnumPropDescriptor<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>(char const*,char const*,RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3d0d6c — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_3d0d6c() -> ! {
    todo!("0x3d0d6c __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::~EnumPropDescriptor()")]
// 0x3d0f20 — __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_3d0f20() -> ! {
    todo!("0x3d0f20 __ZN3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEED0Ev")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::isReadOnly(void)const")]
// 0x3d0f4c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10isReadOnlyEv
// type: int __fastcall(int)
pub fn stub_3d0f4c() -> ! {
    todo!("0x3d0f4c __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10isReadOnlyEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::isWriteOnly(void)const")]
// 0x3d0f5c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11isWriteOnlyEv
// type: int __fastcall(int)
pub fn stub_3d0f5c() -> ! {
    todo!("0x3d0f5c __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11isWriteOnlyEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// 0x3d0f6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
pub fn stub_3d0f6c() -> ! {
    todo!("0x3d0f6c __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// 0x3d0f94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d0f94() -> ! {
    todo!("0x3d0f94 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// 0x3d0fb8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d0fb8() -> ! {
    todo!("0x3d0fb8 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// 0x3d1104 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
pub fn stub_3d1104() -> ! {
    todo!("0x3d1104 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::hasStringValue(void)const")]
// 0x3d1128 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14hasStringValueEv
// type: int()
pub fn stub_3d1128() -> ! {
    todo!("0x3d1128 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14hasStringValueEv")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d112c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
pub fn stub_3d112c() -> ! {
    todo!("0x3d112c __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14getStringValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// 0x3d1150 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
pub fn stub_3d1150() -> ! {
    todo!("0x3d1150 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// 0x3d1190 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d1190() -> ! {
    todo!("0x3d1190 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// 0x3d11b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
pub fn stub_3d11b0() -> ! {
    todo!("0x3d11b0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d13f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3d13f0() -> ! {
    todo!("0x3d13f0 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13getIndexValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// 0x3d140c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
pub fn stub_3d140c() -> ! {
    todo!("0x3d140c __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE13setIndexValueEPNS0_13DescribedBaseEm")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d1440 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3d1440() -> ! {
    todo!("0x3d1440 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12getEnumValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x3d1448 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
pub fn stub_3d1448() -> ! {
    todo!("0x3d1448 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE12setEnumValueEPNS0_13DescribedBaseEi")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// 0x3d1494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
pub fn stub_3d1494() -> ! {
    todo!("0x3d1494 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11getEnumItemEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// 0x3d14b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
pub fn stub_3d14b4() -> ! {
    todo!("0x3d14b4 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Camera::CameraType>::convertToIndex(RBX::Camera::CameraType)const")]
// 0x3d14e8 — __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToIndexES3_
// type: int __fastcall(int, int)
pub fn stub_3d14e8() -> ! {
    todo!("0x3d14e8 __ZNK3RBX10Reflection8EnumDescINS_6Camera10CameraTypeEE14convertToIndexES3_")
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Camera,RBX::Camera::CameraType>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// 0x3d1558 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
pub fn stub_3d1558() -> ! {
    todo!("0x3d1558 __ZNK3RBX10Reflection18EnumPropDescriptorINS_6CameraENS2_10CameraTypeEE11setIntValueEPNS0_13DescribedBaseEi")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::isReadOnly(void)const")]
// 0x3d1598 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
pub fn stub_3d1598() -> ! {
    todo!("0x3d1598 __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::isWriteOnly(void)const")]
// 0x3d159c — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
pub fn stub_3d159c() -> ! {
    todo!("0x3d159c __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3d15a0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
pub fn stub_3d15a0() -> ! {
    todo!("0x3d15a0 __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Camera,RBX::Camera::CameraType>::GetSetImpl<RBX::Camera::CameraType (RBX::Camera::*)(void)const,void (RBX::Camera::*)(RBX::Camera::CameraType)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraType const&)const")]
// 0x3d15c0 — __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3d15c0() -> ! {
    todo!("0x3d15c0 __ZNK3RBX10Reflection14PropDescriptorINS_6CameraENS2_10CameraTypeEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraType> const>::initSingleton(void)")]
// 0x3d15e4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE13initSingletonEv
pub fn stub_3d15e4() -> ! {
    todo!("0x3d15e4 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE13initSingletonEv")
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Camera::CameraType> const>::doGetSingleton(void)")]
// 0x3d15e8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE14doGetSingletonEv
// type: void *()
pub fn stub_3d15e8() -> ! {
    todo!("0x3d15e8 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Camera10CameraTypeEEEE14doGetSingletonEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::EnumDesc(void)")]
// 0x3d22c0 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC1Ev
// type: int()
pub fn stub_3d22c0() -> ! {
    todo!("0x3d22c0 __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::EnumDesc(void)")]
// 0x3d22c4 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC2Ev
// type: int __fastcall(int)
pub fn stub_3d22c4() -> ! {
    todo!("0x3d22c4 __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEEC2Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayProperty(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// 0x3d3328 — __ZN3RBX20ChangeHistoryService4Item14unplayPropertyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(RBX::Instance **, void **)
pub fn stub_3d3328() -> ! {
    todo!("0x3d3328 __ZN3RBX20ChangeHistoryService4Item14unplayPropertyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE")
}

#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint(char const*,RBX::Instance const*)")]
// 0x3d43a4 — __ZN3RBX20ChangeHistoryService15requestWaypointEPKcPKNS_8InstanceE
// type: RBX::ChangeHistoryService *__fastcall(RBX::ChangeHistoryService *this, const char *, const RBX::Instance *)
pub fn stub_3d43a4() -> ! {
    todo!("0x3d43a4 __ZN3RBX20ChangeHistoryService15requestWaypointEPKcPKNS_8InstanceE")
}

#[doc(alias = "RBX::ChangeHistoryService::isRecordable(RBX::Instance *)")]
// 0x3d576c — __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE
// type: int __fastcall(__guard *this, RBX::Instance *, int, int)
pub fn stub_3d576c() -> ! {
    todo!("0x3d576c __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::addPair(RBX::ChangeHistoryService::RuntimeUndoBehavior,char const*)")]
// 0x3d6770 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
pub fn stub_3d6770() -> ! {
    todo!("0x3d6770 __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE7addPairES3_PKc")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")]
// 0x3d6ad0 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3d6ad0() -> ! {
    todo!("0x3d6ad0 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")]
// 0x3d6b18 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3d6b18() -> ! {
    todo!("0x3d6b18 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED1Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")]
// 0x3d6b58 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3d6b58() -> ! {
    todo!("0x3d6b58 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED1Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::findItem(RBX::Instance *)")]
// 0x3d6ba0 — __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE
// type: char *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
pub fn stub_3d6ba0() -> ! {
    todo!("0x3d6ba0 __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::apply(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// 0x3d6ed8 — __ZN3RBX20ChangeHistoryService4Item5applyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, void **)
pub fn stub_3d6ed8() -> ! {
    todo!("0x3d6ed8 __ZN3RBX20ChangeHistoryService4Item5applyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE")
}

#[doc(alias = "RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(RBX::Instance const*)")]
// 0x3d6fc4 — __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub fn stub_3d6fc4() -> ! {
    todo!("0x3d6fc4 __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE")
}

#[doc(alias = "void RBX::ChangeHistoryService::Item::addClusterData<RBX::MegaClusterInstance>(RBX::MegaClusterInstance const*)")]
// 0x3d7df0 — __ZN3RBX20ChangeHistoryService4Item14addClusterDataINS_19MegaClusterInstanceEEEvPKT_
// type: RBX::MegaClusterInstance *__fastcall(int, RBX::MegaClusterInstance *this)
pub fn stub_3d7df0() -> ! {
    todo!("0x3d7df0 __ZN3RBX20ChangeHistoryService4Item14addClusterDataINS_19MegaClusterInstanceEEEvPKT_")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordProperty(RBX::Reflection::PropertyDescriptor const*)")]
// 0x3d7f90 — __ZN3RBX20ChangeHistoryService4Item14recordPropertyEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Reflection::PropertyDescriptor *)
pub fn stub_3d7f90() -> ! {
    todo!("0x3d7f90 __ZN3RBX20ChangeHistoryService4Item14recordPropertyEPKNS_10Reflection18PropertyDescriptorE")
}

#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(RBX::Instance const*)")]
// 0x3d82b0 — __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
pub fn stub_3d82b0() -> ! {
    todo!("0x3d82b0 __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// 0x3d82e8 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_3d82e8() -> ! {
    todo!("0x3d82e8 __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// 0x3d82fc — __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_3d82fc() -> ! {
    todo!("0x3d82fc __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// 0x3d8310 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED1Ev
// type: int()
pub fn stub_3d8310() -> ! {
    todo!("0x3d8310 __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED1Ev")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToString(unsigned long,std::string &)const")]
// 0x3d8318 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
pub fn stub_3d8318() -> ! {
    todo!("0x3d8318 __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE15convertToStringEmRSs")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToItem(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)const")]
// 0x3d8470 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
pub fn stub_3d8470() -> ! {
    todo!("0x3d8470 __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// 0x3d8540 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
pub fn stub_3d8540() -> ! {
    todo!("0x3d8540 __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED2Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv")]
// 0x3d8718 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv
pub fn stub_3d8718() -> ! {
    todo!("0x3d8718 __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// 0x3d9778 — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
pub fn stub_3d9778() -> ! {
    todo!("0x3d9778 __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv")]
// 0x3da568 — __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv
// type: int()
pub fn stub_3da568() -> ! {
    todo!("0x3da568 __ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3da5d0 — __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3da5d0() -> ! {
    todo!("0x3da5d0 __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3da5d4 — __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3da5d4() -> ! {
    todo!("0x3da5d4 __ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3da674 — __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3da674() -> ! {
    todo!("0x3da674 __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3da67c — __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3da67c() -> ! {
    todo!("0x3da67c __ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3da720 — __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3da720() -> ! {
    todo!("0x3da720 __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3da728 — __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3da728() -> ! {
    todo!("0x3da728 __ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::find(RBX::Reflection::PropertyDescriptor const* const&)")]
// 0x3dad08 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE4findERS6_
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_3dad08() -> ! {
    todo!("0x3dad08 __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE4findERS6_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3db268 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, unsigned int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_3db268() -> ! {
    todo!("0x3db268 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")]
// 0x3db36c — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3db36c() -> ! {
    todo!("0x3db36c __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3db420 — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int)
pub fn stub_3db420() -> ! {
    todo!("0x3db420 __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3db440 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3db440() -> ! {
    todo!("0x3db440 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3db5b8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3db5b8() -> ! {
    todo!("0x3db5b8 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")]
// 0x3db5e8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3db5e8() -> ! {
    todo!("0x3db5e8 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3db6b4 — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
pub fn stub_3db6b4() -> ! {
    todo!("0x3db6b4 __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ChangeHistoryService,void (RBX::ChangeHistoryService::*)(std::string),std::string,void>::call(RBX::ChangeHistoryService*,void (RBX::ChangeHistoryService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// 0x3db7f0 — __ZN3RBX10Reflection11Call1HelperINS_20ChangeHistoryServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
pub fn stub_3db7f0() -> ! {
    todo!("0x3db7f0 __ZN3RBX10Reflection11Call1HelperINS_20ChangeHistoryServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x3db920 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
pub fn stub_3db920() -> ! {
    todo!("0x3db920 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x3dba98 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_3dba98() -> ! {
    todo!("0x3dba98 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")]
// 0x3dbac8 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3dbac8() -> ! {
    todo!("0x3dbac8 __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x3dbb9c — __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
pub fn stub_3dbb9c() -> ! {
    todo!("0x3dbb9c __ZNK3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> *)")]
// 0x3dc8f4 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_3dc8f4() -> ! {
    todo!("0x3dc8f4 __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::addValue(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3dc928 — __ZN3RBX20ChangeHistoryService4Item8addValueERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Reflection::PropertyDescriptor *)
pub fn stub_3dc928() -> ! {
    todo!("0x3dc928 __ZN3RBX20ChangeHistoryService4Item8addValueERKNS_10Reflection18PropertyDescriptorE")
}

#[doc(alias = "std::map<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::operator[](RBX::Reflection::PropertyDescriptor const* const&)")]
// 0x3dcb74 — __ZNSt3mapIPKN3RBX10Reflection18PropertyDescriptorENS1_7VariantESt4lessIS4_ESaISt4pairIKS4_S5_EEEixERS9_
// type: int __fastcall(int, _DWORD *)
pub fn stub_3dcb74() -> ! {
    todo!("0x3dcb74 __ZNSt3mapIPKN3RBX10Reflection18PropertyDescriptorENS1_7VariantESt4lessIS4_ESaISt4pairIKS4_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dccdc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, unsigned int *)
pub fn stub_3dccdc() -> ! {
    todo!("0x3dccdc __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dcd90 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_3dcd90() -> ! {
    todo!("0x3dcd90 __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dcddc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_3dcddc() -> ! {
    todo!("0x3dcddc __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_create_node(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
// 0x3dce44 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE14_M_create_nodeERKS8_
// type: _DWORD *__fastcall(int, int *, int, int, void *, int)
pub fn stub_3dce44() -> ! {
    todo!("0x3dce44 __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE14_M_create_nodeERKS8_")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::addValueIfNotParentProperty(RBX::Reflection::Property const&)")]
// 0x3dd54c — __ZN3RBX20ChangeHistoryService4Item27addValueIfNotParentPropertyERKNS_10Reflection8PropertyE
// type: int __fastcall(int, void **)
pub fn stub_3dd54c() -> ! {
    todo!("0x3dd54c __ZN3RBX20ChangeHistoryService4Item27addValueIfNotParentPropertyERKNS_10Reflection8PropertyE")
}

#[doc(alias = "std::map<RBX::Instance *,unsigned int,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::operator[](RBX::Instance * const&)")]
// 0x3dd564 — __ZNSt3mapIPN3RBX8InstanceEjSt4lessIS2_ESaISt4pairIKS2_jEEEixERS6_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_3dd564() -> ! {
    todo!("0x3dd564 __ZNSt3mapIPN3RBX8InstanceEjSt4lessIS2_ESaISt4pairIKS2_jEEEixERS6_")
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,unsigned int>>,std::pair<RBX::Instance * const,unsigned int> const&)")]
// 0x3dd5bc — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
pub fn stub_3dd5bc() -> ! {
    todo!("0x3dd5bc __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,unsigned int> const&)")]
// 0x3dd670 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_3dd670() -> ! {
    todo!("0x3dd670 __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")]
// 0x3dd6c8 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int *)
pub fn stub_3dd6c8() -> ! {
    todo!("0x3dd6c8 __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_Rb_tree(std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>> const&)")]
// 0x3dda98 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_
pub fn stub_3dda98() -> ! {
    todo!("0x3dda98 __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>*)")]
// 0x3ddadc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
pub fn stub_3ddadc() -> ! {
    todo!("0x3ddadc __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::removeItem(RBX::Instance *)")]
// 0x3deb04 — __ZN3RBX20ChangeHistoryService8Waypoint10removeItemEPNS_8InstanceE
// type: _Rb_tree_node_base *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
pub fn stub_3deb04() -> ! {
    todo!("0x3deb04 __ZN3RBX20ChangeHistoryService8Waypoint10removeItemEPNS_8InstanceE")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::absorbProp(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// 0x3ded94 — __ZN3RBX20ChangeHistoryService4Item10absorbPropERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, int)
pub fn stub_3ded94() -> ! {
    todo!("0x3ded94 __ZN3RBX20ChangeHistoryService4Item10absorbPropERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE")
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,unsigned int>> *)")]
// 0x3df534 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_3df534() -> ! {
    todo!("0x3df534 __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::CharacterAppearance::askSetParent(RBX::Instance const*)const")]
// 0x3e113c — __ZNK3RBX19CharacterAppearance12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::CharacterAppearance *__hidden this, const RBX::Instance *)
pub fn stub_3e113c() -> ! {
    todo!("0x3e113c __ZNK3RBX19CharacterAppearance12askSetParentEPKNS_8InstanceE")
}

#[doc(alias = "RBX::ShirtGraphic::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e1178 — __ZN3RBX12ShirtGraphic11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e1178() -> ! {
    todo!("0x3e1178 __ZN3RBX12ShirtGraphic11dataChangedERKNS_10Reflection18PropertyDescriptorE")
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// 0x3e117c — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e117c() -> ! {
    todo!("0x3e117c __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev")
}

#[doc(alias = "RBX::Clothing::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e11a0 — __ZN3RBX8Clothing11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e11a0() -> ! {
    todo!("0x3e11a0 __ZN3RBX8Clothing11dataChangedERKNS_10Reflection18PropertyDescriptorE")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
// 0x3e11a4 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e11a4() -> ! {
    todo!("0x3e11a4 __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
// 0x3e11c8 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e11c8() -> ! {
    todo!("0x3e11c8 __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev")
}

#[doc(alias = "RBX::Skin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e11ec — __ZN3RBX4Skin11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e11ec() -> ! {
    todo!("0x3e11ec __ZN3RBX4Skin11dataChangedERKNS_10Reflection18PropertyDescriptorE")
}

#[doc(alias = "RBX::BodyColors::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e11f0 — __ZN3RBX10BodyColors11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e11f0() -> ! {
    todo!("0x3e11f0 __ZN3RBX10BodyColors11dataChangedERKNS_10Reflection18PropertyDescriptorE")
}

#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(RBX::Instance *)")]
// 0x3e11f4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_PNS_8InstanceE
pub fn stub_3e11f4() -> ! {
    todo!("0x3e11f4 __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_PNS_8InstanceE")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// 0x3e1344 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1344() -> ! {
    todo!("0x3e1344 __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// 0x3e1470 — __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1470() -> ! {
    todo!("0x3e1470 __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")]
// 0x3e1cc0 — __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1cc0() -> ! {
    todo!("0x3e1cc0 __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")]
// 0x3e1d7c — __ZThn32_NK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1d7c() -> ! {
    todo!("0x3e1d7c __ZThn32_NK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")]
// 0x3e1f88 — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1f88() -> ! {
    todo!("0x3e1f88 __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")]
// 0x3e2044 — __ZThn32_NK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e2044() -> ! {
    todo!("0x3e2044 __ZThn32_NK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21ac — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21ac() -> ! {
    todo!("0x3e21ac __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD1Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21b0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21b0() -> ! {
    todo!("0x3e21b0 __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD1Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21b4 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21b4() -> ! {
    todo!("0x3e21b4 __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD1Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21b8 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21b8() -> ! {
    todo!("0x3e21b8 __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD1Ev")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21bc — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21bc() -> ! {
    todo!("0x3e21bc __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD1Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")]
// 0x3e22e8 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e22e8() -> ! {
    todo!("0x3e22e8 __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")]
// 0x3e2440 — __ZThn32_NK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e2440() -> ! {
    todo!("0x3e2440 __ZThn32_NK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")]
// 0x3e27d8 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e27d8() -> ! {
    todo!("0x3e27d8 __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")]
// 0x3e2930 — __ZThn32_NK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e2930() -> ! {
    todo!("0x3e2930 __ZThn32_NK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e2ba0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e2ba0() -> ! {
    todo!("0x3e2ba0 __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE17static_getCreatorEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e2c14 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e2c14() -> ! {
    todo!("0x3e2c14 __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator12getClassNameEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e2d80 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e2d80() -> ! {
    todo!("0x3e2d80 __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE17static_getCreatorEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e2df4 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e2df4() -> ! {
    todo!("0x3e2df4 __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator12getClassNameEv")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e2f60 — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e2f60() -> ! {
    todo!("0x3e2f60 __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD2Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e2ffc — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e2ffc() -> ! {
    todo!("0x3e2ffc __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator12getClassNameEv")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator6createEv")]
// 0x3e3084 — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e3084() -> ! {
    todo!("0x3e3084 __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator6createEv")
}

#[cfg(test)]
mod batch1_tests {
    use super::camera_desc;
    use crate::generated_core_watchdog_c::billboard_prop::{
        GetSetImpl, MemberPtr, PropDescriptor,
    };
    use crate::generated_core_watchdog_c::billboard_ref::{
        InstanceHandle, RefPropDescriptor, RefVariant,
    };
    use crate::generated_core_watchdog_c::camera_boundfunc::FnCameraFloatVoid;
    extern "C" fn fake_get(this: *const u8) -> *mut u8 {
        // Slot sits at +36 (4-mod-8); the binary is ARM (unaligned-safe).
        unsafe { (this.add(36) as *const *mut u8).read_unaligned() }
    }
    extern "C" fn fake_set(this: *mut u8, v: *mut u8) {
        unsafe { (this.add(36) as *mut *mut u8).write_unaligned(v) }
    }
    extern "C" fn fake_fget(this: *const u8) -> f32 {
        unsafe { *(this.add(36) as *const f32) }
    }
    extern "C" fn fake_fset(this: *mut u8, v: f32) {
        unsafe { *(this.add(36) as *mut f32) = v }
    }

    /// Instance* slot at +36 (pointer-sized, 4-mod-8 — always touched via
    /// `read_unaligned`/`write_unaligned`, like the ARM binary).
    #[repr(C)]
    struct FakeInst {
        pad: [u8; 44],
    }
    #[repr(C)]
    struct FakeFloat {
        pad: [u8; 36],
        f: f32,
    }

    fn slot_of(fake: &FakeInst) -> *mut u8 {
        unsafe { ((std::ptr::addr_of!(*fake) as *const u8).add(36) as *const *mut u8).read_unaligned() }
    }
    fn set_slot(fake: &mut FakeInst, v: *mut u8) {
        unsafe { ((std::ptr::addr_of!(*fake) as *const u8).add(36) as *mut *mut u8).write_unaligned(v) }
    }

    fn direct(f: usize) -> MemberPtr {
        MemberPtr { func: f, adj: 0 }
    }

    fn ref_desc() -> RefPropDescriptor {
        RefPropDescriptor {
            vtable: camera_desc::CAM_REF_DESC_VTAB,
            sub_vtable: camera_desc::CAM_REF_SUB_VTAB,
            name: String::from("CameraSubject"),
            category: String::from("Camera"),
            getset: Some(Box::new(GetSetImpl {
                getter: direct(fake_get as usize),
                setter: direct(fake_set as usize),
            })),
            attributes: (0, 0, 0),
            permissions: 0,
        }
    }

    fn described_of(fake: *const u8) -> *const u8 {
        fake.wrapping_add(36)
    }

    fn imp_of(d: &RefPropDescriptor) -> *const GetSetImpl {
        d.getset.as_deref().unwrap() as *const GetSetImpl
    }

    #[test]
    fn ref_ctor_vtables_and_dtor_clears() {
        let mut slot = RefPropDescriptor::default();
        unsafe {
            super::stub_3d043c(
                &mut slot,
                std::ptr::null(),
                std::ptr::null(),
                0x40,
                0,
                0x44,
                0,
                0,
                0,
                0,
                0,
            );
        }
        assert_eq!(slot.vtable, camera_desc::CAM_REF_DESC_VTAB);
        assert_eq!(slot.sub_vtable, camera_desc::CAM_REF_SUB_VTAB);
        assert!(slot.getset.is_some());
        unsafe {
            assert!(!super::stub_3d0510(&slot));
            assert!(!super::stub_3d0520(&slot));
            super::stub_3d04e0(&mut slot);
        }
        assert_eq!(slot.vtable, camera_desc::CAM_REF_DESC_VTAB);
        assert!(slot.getset.is_none());
    }

    #[test]
    fn ref_value_paths_roundtrip() {
        let d = ref_desc();
        let imp = imp_of(&d);
        let mut fake = FakeInst { pad: [0; 44] };
        set_slot(&mut fake, 0x600 as *mut u8);
        let base = std::ptr::addr_of!(fake) as *const u8;
        let mut other = FakeInst { pad: [0; 44] };
        set_slot(&mut other, 0x601 as *mut u8);
        let other_base = std::ptr::addr_of!(other) as *const u8;
        unsafe {
            assert!(super::stub_3d0530(&d, described_of(base), described_of(base)));
            // NOTE: a null input would fault inside the member call, exactly as
            // the binary does (null derived `this` passed to the getter).
            assert!(!super::stub_3d0530(&d, described_of(base), described_of(other_base)));
            assert_eq!(super::stub_3d09f4(imp, described_of(base)), 0x600 as *mut u8);
            super::stub_3d0a14(imp, described_of(base) as *mut u8, 0x500 as *mut u8);
            assert_eq!(slot_of(&fake), 0x500 as *mut u8);
            let v = super::stub_3d0558(&d, described_of(base));
            assert_eq!(v.ptr, (0x500usize + 36) as *const u8);
            super::stub_3d0670(&d, described_of(base) as *mut u8, &v);
            assert_eq!(slot_of(&fake), 0x500 as *mut u8);
            let mut dst_fake = FakeInst { pad: [0; 44] };
            let dst_base = std::ptr::addr_of!(dst_fake) as *const u8;
            super::stub_3d0738(&d, described_of(base), described_of(dst_base) as *mut u8);
            assert_eq!(slot_of(&dst_fake), 0x500 as *mut u8);
            assert_eq!(
                super::stub_3d0854(&d, described_of(base)),
                (0x500usize + 36) as *const u8
            );
            super::stub_3d08e4(&d, described_of(base) as *mut u8, (0x700usize + 36) as *const u8);
            assert_eq!(slot_of(&fake), 0x700 as *mut u8);
            assert!(super::stub_3d0868(&d, described_of(base) as *mut u8, std::ptr::null(), |_| {
                false
            })
            .is_ok());
            assert!(super::stub_3d0868(
                &d,
                described_of(base) as *mut u8,
                0x900 as *const u8,
                |_| { false }
            )
            .is_err());
            let h = InstanceHandle { pi: (0x800usize + 36) as *const u8, _count: 1 };
            super::stub_3d0904(&d, described_of(base) as *mut u8, &h);
            assert_eq!(slot_of(&fake), 0x800 as *mut u8);
        }
    }

    #[test]
    fn float_and_cf_descriptors() {
        let mut slot = PropDescriptor::default();
        unsafe {
            super::stub_3d0a38(
                &mut slot,
                std::ptr::null(),
                std::ptr::null(),
                0x40,
                0,
                0x44,
                0,
                1,
                2,
                3,
                4,
            );
        }
        assert_eq!(slot.vtable, camera_desc::CAM_FLOAT_DESC_VTAB);
        assert_eq!(slot.attributes, (1, 2, 3));
        assert_eq!(slot.permissions, 4);
        unsafe {
            assert!(!super::stub_3d0b78());
            assert!(!super::stub_3d0b7c());
            super::stub_3d0b4c(&mut slot);
        }
        assert_eq!(slot.vtable, camera_desc::TYPED_FLOAT_VTAB);
        assert!(slot.getset.is_none());
        let mut cf = PropDescriptor::default();
        unsafe {
            super::stub_3d0bc4(
                &mut cf,
                std::ptr::null(),
                std::ptr::null(),
                0x50,
                0,
                0x54,
                0,
                0,
                0,
                0,
                0,
            );
        }
        assert_eq!(cf.vtable, camera_desc::CAM_CF_DESC_VTAB);
        unsafe {
            assert!(!super::stub_3d0d04());
            assert!(!super::stub_3d0d08());
            super::stub_3d0cd8(&mut cf);
        }
        assert_eq!(cf.vtable, camera_desc::TYPED_CF_VTAB);
    }

    #[test]
    fn float_get_set_direct() {
        let mut fake = FakeFloat { pad: [0; 36], f: 3.0 };
        let base = std::ptr::addr_of!(fake) as *const u8;
        let imp = GetSetImpl { getter: direct(fake_fget as usize), setter: direct(fake_fset as usize) };
        unsafe {
            assert_eq!(super::stub_3d0b80(&imp, described_of(base)), 3.0);
            super::stub_3d0ba0(&imp, described_of(base) as *mut u8, 4.0);
        }
        assert_eq!(fake.f, 4.0);
    }

    struct FVoid {
        seen: std::cell::Cell<f32>,
    }
    impl FnCameraFloatVoid for FVoid {
        fn call(&self, _: *const u8, arg: f32) {
            self.seen.set(arg);
        }
    }

    #[test]
    fn void_float_execute_forwards() {
        use crate::generated_core_watchdog_c::camera_enum::BoundFuncDescriptor;
        let d = BoundFuncDescriptor::default();
        let f = FVoid { seen: std::cell::Cell::new(0.0) };
        unsafe {
            super::stub_3d0400(&d, 0x100 as *const u8, 2.0, &f);
        }
        assert_eq!(f.seen.get(), 2.0);
    }

    #[test]
    fn refvariant_and_xml_shapes() {
        let v = RefVariant { type_tag: "t", ptr: 0x100 as *const u8 };
        assert_eq!(v.ptr, 0x100 as *const u8);
        let d = ref_desc();
        unsafe {
            let mut xml = crate::generated_core_watchdog_c::billboard_ref::XmlElement::default();
            let mut fake = FakeInst { pad: [0; 44] };
            set_slot(&mut fake, 0x600 as *mut u8);
            let base = std::ptr::addr_of!(fake) as *const u8;
            super::stub_3d075c(&d, described_of(base), &mut xml);
            assert_eq!(xml.value, (0x600usize + 36) as *const u8);
        }
    }

    struct RecBinder {
        seen_cookie: std::cell::Cell<usize>,
    }
    impl crate::generated_core_watchdog_c::billboard_ref::ReferenceBinder for RecBinder {
        unsafe fn bind(&self, _slot: *const u8, _obj: *mut u8, cookie: *const u8) -> i32 {
            self.seen_cookie.set(cookie as usize);
            7
        }
    }

    #[test]
    fn read_value_hops_to_binder() {
        let d = ref_desc();
        let b = RecBinder { seen_cookie: std::cell::Cell::new(0) };
        unsafe {
            let xml = crate::generated_core_watchdog_c::billboard_ref::XmlElement::default();
            let r = super::stub_3d0830(&d, 0x100 as *mut u8, &xml, &b);
            assert_eq!(r, 7);
            assert_eq!(
                b.seen_cookie.get(),
                (&d as *const RefPropDescriptor as usize) + 40
            );
        }
    }
}
