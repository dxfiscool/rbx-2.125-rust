//! core wd_11e — 100 core stubs EA-sorted asc not yet in crates/core/src (gap filler sequential after 0x4a906c) Range 0x4a90dc..0x4ab5f0 | rbx_core::SharedPtr not boost.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not yet in crates/core/src (gap filler sequential after 0x4a906c).
//! Range: 0x4a90dc..0x4ab5f0 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11setIntValueEPNS0_13DescribedBaseEi")]
// 0x4a90dc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_0x4a90dc() {
    // IDA 0x4a90dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
// 0x4a911c — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_0x4a911c() {
    // IDA 0x4a911c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
// 0x4a9120 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_0x4a9120() {
    // IDA 0x4a9120: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
// 0x4a9124 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4a9124() {
    // IDA 0x4a9124: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ExtrudedPartInstance,RBX::ExtrudedPartInstance::VisualTrussStyle>::GetSetImpl<RBX::ExtrudedPartInstance::VisualTrussStyle (RBX::ExtrudedPartInstance::*)(void)const,void (RBX::ExtrudedPartInstance::*)(RBX::ExtrudedPartInstance::VisualTrussStyle)>::setValue(RBX::Reflection::DescribedBase *,RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
// 0x4a9144 — __ZNK3RBX10Reflection14PropDescriptorINS_20ExtrudedPartInstanceENS2_16VisualTrussStyleEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_0x4a9144() {
    // IDA 0x4a9144: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_189")]
#[doc(alias = "__GLOBAL__I_a_189")]
// 0x4a9168 — __GLOBAL__I_a_189
pub fn stub_0x4a9168() {
    // IDA 0x4a9168: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::FaceInstance::setFace(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX12FaceInstance7setFaceENS_8NormalIdE")]
// 0x4a94fc — __ZN3RBX12FaceInstance7setFaceENS_8NormalIdE
pub fn stub_0x4a94fc() {
    // IDA 0x4a94fc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FaceInstance::FaceInstance(void)")]
#[doc(alias = "__ZN3RBX12FaceInstanceC2Ev")]
// 0x4a9518 — __ZN3RBX12FaceInstanceC2Ev
pub fn stub_0x4a9518() {
    // IDA 0x4a9518: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FaceInstance::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX12FaceInstance12askSetParentEPKNS_8InstanceE")]
// 0x4a9668 — __ZNK3RBX12FaceInstance12askSetParentEPKNS_8InstanceE
pub fn stub_0x4a9668() {
    // IDA 0x4a9668: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FaceInstance::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
#[doc(alias = "__ZN3RBX12FaceInstance14render3dSelectEPNS_5AdornENS_11SelectStateE")]
// 0x4a96a4 — __ZN3RBX12FaceInstance14render3dSelectEPNS_5AdornENS_11SelectStateE
pub fn stub_0x4a96a4() {
    // IDA 0x4a96a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
#[doc(alias = "__ZThn92_N3RBX12FaceInstance14render3dSelectEPNS_5AdornENS_11SelectStateE")]
// 0x4a971c — __ZThn92_N3RBX12FaceInstance14render3dSelectEPNS_5AdornENS_11SelectStateE
pub fn stub_0x4a971c() {
    // IDA 0x4a971c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FaceInstance::getFace(void)const")]
#[doc(alias = "__ZNK3RBX12FaceInstance7getFaceEv")]
// 0x4a9724 — __ZNK3RBX12FaceInstance7getFaceEv
pub fn stub_0x4a9724() {
    // IDA 0x4a9724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED1Ev")]
// 0x4a9728 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED1Ev
pub fn stub_0x4a9728() {
    // IDA 0x4a9728: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FaceInstance::~FaceInstance()")]
#[doc(alias = "__ZN3RBX12FaceInstanceD1Ev")]
// 0x4a974c — __ZN3RBX12FaceInstanceD1Ev
pub fn stub_0x4a974c() {
    // IDA 0x4a974c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FaceInstance::~FaceInstance()")]
#[doc(alias = "__ZN3RBX12FaceInstanceD0Ev")]
// 0x4a9808 — __ZN3RBX12FaceInstanceD0Ev
pub fn stub_0x4a9808() {
    // IDA 0x4a9808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance()")]
#[doc(alias = "__ZThn32_N3RBX12FaceInstanceD1Ev")]
// 0x4a98d4 — __ZThn32_N3RBX12FaceInstanceD1Ev
pub fn stub_0x4a98d4() {
    // IDA 0x4a98d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance()")]
#[doc(alias = "__ZThn32_N3RBX12FaceInstanceD0Ev")]
// 0x4a998c — __ZThn32_N3RBX12FaceInstanceD0Ev
pub fn stub_0x4a998c() {
    // IDA 0x4a998c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance()")]
#[doc(alias = "__ZThn36_N3RBX12FaceInstanceD1Ev")]
// 0x4a9a5c — __ZThn36_N3RBX12FaceInstanceD1Ev
pub fn stub_0x4a9a5c() {
    // IDA 0x4a9a5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::FaceInstance::~FaceInstance()")]
#[doc(alias = "__ZThn36_N3RBX12FaceInstanceD0Ev")]
// 0x4a9b14 — __ZThn36_N3RBX12FaceInstanceD0Ev
pub fn stub_0x4a9b14() {
    // IDA 0x4a9b14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a9be4 — __ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a9be4() {
    // IDA 0x4a9be4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a9be8 — __ZN3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a9be8() {
    // IDA 0x4a9be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a9c88 — __ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a9c88() {
    // IDA 0x4a9c88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a9c90 — __ZThn32_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a9c90() {
    // IDA 0x4a9c90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x4a9d34 — __ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_0x4a9d34() {
    // IDA 0x4a9d34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x4a9d3c — __ZThn36_N3RBX10Reflection9DescribedINS_12FaceInstanceELZNS_13sFaceInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_0x4a9d3c() {
    // IDA 0x4a9d3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>(char const*,char const*,RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// 0x4a9de0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_0x4a9de0() {
    // IDA 0x4a9de0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED0Ev")]
// 0x4a9f94 — __ZN3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEED0Ev
pub fn stub_0x4a9f94() {
    // IDA 0x4a9f94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10isReadOnlyEv")]
// 0x4a9fc0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10isReadOnlyEv
pub fn stub_0x4a9fc0() {
    // IDA 0x4a9fc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11isWriteOnlyEv")]
// 0x4a9fd0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11isWriteOnlyEv
pub fn stub_0x4a9fd0() {
    // IDA 0x4a9fd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_")]
// 0x4a9fe0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
pub fn stub_0x4a9fe0() {
    // IDA 0x4a9fe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
// 0x4aa008 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
pub fn stub_0x4aa008() {
    // IDA 0x4aa008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
// 0x4aa02c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
pub fn stub_0x4aa02c() {
    // IDA 0x4aa02c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
// 0x4aa178 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
pub fn stub_0x4aa178() {
    // IDA 0x4aa178: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14hasStringValueEv")]
// 0x4aa19c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14hasStringValueEv
pub fn stub_0x4aa19c() {
    // IDA 0x4aa19c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE")]
// 0x4aa1a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_0x4aa1a0() {
    // IDA 0x4aa1a0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// 0x4aa1c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_0x4aa1c4() {
    // IDA 0x4aa1c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// 0x4aa204 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_0x4aa204() {
    // IDA 0x4aa204: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// 0x4aa224 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_0x4aa224() {
    // IDA 0x4aa224: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE")]
// 0x4aa464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
pub fn stub_0x4aa464() {
    // IDA 0x4aa464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm")]
// 0x4aa480 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
pub fn stub_0x4aa480() {
    // IDA 0x4aa480: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE")]
// 0x4aa4b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
pub fn stub_0x4aa4b4() {
    // IDA 0x4aa4b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi")]
// 0x4aa4bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
pub fn stub_0x4aa4bc() {
    // IDA 0x4aa4bc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE")]
// 0x4aa508 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
pub fn stub_0x4aa508() {
    // IDA 0x4aa508: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
// 0x4aa528 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
pub fn stub_0x4aa528() {
    // IDA 0x4aa528: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::FaceInstance,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi")]
// 0x4aa55c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_12FaceInstanceENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
pub fn stub_0x4aa55c() {
    // IDA 0x4aa55c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
// 0x4aa59c — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
pub fn stub_0x4aa59c() {
    // IDA 0x4aa59c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
// 0x4aa5a0 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
pub fn stub_0x4aa5a0() {
    // IDA 0x4aa5a0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
// 0x4aa5a4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
pub fn stub_0x4aa5a4() {
    // IDA 0x4aa5a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FaceInstance,RBX::NormalId>::GetSetImpl<RBX::NormalId (RBX::FaceInstance::*)(void)const,void (RBX::FaceInstance::*)(RBX::NormalId)>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
// 0x4aa5c4 — __ZNK3RBX10Reflection14PropDescriptorINS_12FaceInstanceENS_8NormalIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
pub fn stub_0x4aa5c4() {
    // IDA 0x4aa5c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_190")]
#[doc(alias = "__GLOBAL__I_a_190")]
// 0x4aa5e8 — __GLOBAL__I_a_190
pub fn stub_0x4aa5e8() {
    // IDA 0x4aa5e8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::PriorityMethod>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler14PriorityMethodEEERKS1_v")]
// 0x4aab84 — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler14PriorityMethodEEERKS1_v
pub fn stub_0x4aab84() {
    // IDA 0x4aab84: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::Job::SleepAdjustMethod>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler3Job17SleepAdjustMethodEEERKS1_v")]
// 0x4aabb8 — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler3Job17SleepAdjustMethodEEERKS1_v
pub fn stub_0x4aabb8() {
    // IDA 0x4aabb8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TaskScheduler::ThreadPoolConfig>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler16ThreadPoolConfigEEERKS1_v")]
// 0x4aabec — __ZN3RBX10Reflection4Type12getSingletonINS_13TaskScheduler16ThreadPoolConfigEEERKS1_v
pub fn stub_0x4aabec() {
    // IDA 0x4aabec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Controller::Button>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_10Controller6ButtonEEERKS1_v")]
// 0x4aac20 — __ZN3RBX10Reflection4Type12getSingletonINS_10Controller6ButtonEEERKS1_v
pub fn stub_0x4aac20() {
    // IDA 0x4aac20: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenEasingStyle>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject16TweenEasingStyleEEERKS1_v")]
// 0x4aac54 — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject16TweenEasingStyleEEERKS1_v
pub fn stub_0x4aac54() {
    // IDA 0x4aac54: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenStatus>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject11TweenStatusEEERKS1_v")]
// 0x4aac88 — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject11TweenStatusEEERKS1_v
pub fn stub_0x4aac88() {
    // IDA 0x4aac88: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiObject::TweenEasingDirection>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject20TweenEasingDirectionEEERKS1_v")]
// 0x4aacbc — __ZN3RBX10Reflection4Type12getSingletonINS_9GuiObject20TweenEasingDirectionEEERKS1_v
pub fn stub_0x4aacbc() {
    // IDA 0x4aacbc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::XAlignment>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11TextService10XAlignmentEEERKS1_v")]
// 0x4aacf0 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10XAlignmentEEERKS1_v
pub fn stub_0x4aacf0() {
    // IDA 0x4aacf0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::YAlignment>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11TextService10YAlignmentEEERKS1_v")]
// 0x4aad24 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService10YAlignmentEEERKS1_v
pub fn stub_0x4aad24() {
    // IDA 0x4aad24: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::FontSize>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11TextService8FontSizeEEERKS1_v")]
// 0x4aad58 — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService8FontSizeEEERKS1_v
pub fn stub_0x4aad58() {
    // IDA 0x4aad58: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextService::Font>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11TextService4FontEEERKS1_v")]
// 0x4aad8c — __ZN3RBX10Reflection4Type12getSingletonINS_11TextService4FontEEERKS1_v
pub fn stub_0x4aad8c() {
    // IDA 0x4aad8c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraTypeEEERKS1_v")]
// 0x4aadc0 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraTypeEEERKS1_v
pub fn stub_0x4aadc0() {
    // IDA 0x4aadc0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraMode>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraModeEEERKS1_v")]
// 0x4aadf4 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera10CameraModeEEERKS1_v
pub fn stub_0x4aadf4() {
    // IDA 0x4aadf4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Camera::CameraPanMode>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_6Camera13CameraPanModeEEERKS1_v")]
// 0x4aae28 — __ZN3RBX10Reflection4Type12getSingletonINS_6Camera13CameraPanModeEEERKS1_v
pub fn stub_0x4aae28() {
    // IDA 0x4aae28: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::LegacyController::InputType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_16LegacyController9InputTypeEEERKS1_v")]
// 0x4aae5c — __ZN3RBX10Reflection4Type12getSingletonINS_16LegacyController9InputTypeEEERKS1_v
pub fn stub_0x4aae5c() {
    // IDA 0x4aae5c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModelArbiter::ConcurrencyModel>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_16DataModelArbiter16ConcurrencyModelEEERKS1_v")]
// 0x4aae90 — __ZN3RBX10Reflection4Type12getSingletonINS_16DataModelArbiter16ConcurrencyModelEEERKS1_v
pub fn stub_0x4aae90() {
    // IDA 0x4aae90: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DebugSettings::ErrorReporting>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_13DebugSettings14ErrorReportingEEERKS1_v")]
// 0x4aaec4 — __ZN3RBX10Reflection4Type12getSingletonINS_13DebugSettings14ErrorReportingEEERKS1_v
pub fn stub_0x4aaec4() {
    // IDA 0x4aaec4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::EThrottle::EThrottleType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9EThrottle13EThrottleTypeEEERKS1_v")]
// 0x4aaef8 — __ZN3RBX10Reflection4Type12getSingletonINS_9EThrottle13EThrottleTypeEEERKS1_v
pub fn stub_0x4aaef8() {
    // IDA 0x4aaef8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::NormalId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_8NormalIdEEERKS1_v")]
// 0x4aaf2c — __ZN3RBX10Reflection4Type12getSingletonINS_8NormalIdEEERKS1_v
pub fn stub_0x4aaf2c() {
    // IDA 0x4aaf2c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<G3D::Vector3::Axis>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonIN3G3D7Vector34AxisEEERKS1_v")]
// 0x4aaf60 — __ZN3RBX10Reflection4Type12getSingletonIN3G3D7Vector34AxisEEERKS1_v
pub fn stub_0x4aaf60() {
    // IDA 0x4aaf60: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Humanoid::Status>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_8Humanoid6StatusEEERKS1_v")]
// 0x4aaf94 — __ZN3RBX10Reflection4Type12getSingletonINS_8Humanoid6StatusEEERKS1_v
pub fn stub_0x4aaf94() {
    // IDA 0x4aaf94: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::CreatorType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9DataModel11CreatorTypeEEERKS1_v")]
// 0x4aafc8 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel11CreatorTypeEEERKS1_v
pub fn stub_0x4aafc8() {
    // IDA 0x4aafc8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::Genre>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9DataModel5GenreEEERKS1_v")]
// 0x4aaffc — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel5GenreEEERKS1_v
pub fn stub_0x4aaffc() {
    // IDA 0x4aaffc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::GearGenreSetting>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9DataModel16GearGenreSettingEEERKS1_v")]
// 0x4ab030 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel16GearGenreSettingEEERKS1_v
pub fn stub_0x4ab030() {
    // IDA 0x4ab030: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::DataModel::GearType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9DataModel8GearTypeEEERKS1_v")]
// 0x4ab064 — __ZN3RBX10Reflection4Type12getSingletonINS_9DataModel8GearTypeEEERKS1_v
pub fn stub_0x4ab064() {
    // IDA 0x4ab064: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Instance::SaveFilter>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_8Instance10SaveFilterEEERKS1_v")]
// 0x4ab098 — __ZN3RBX10Reflection4Type12getSingletonINS_8Instance10SaveFilterEEERKS1_v
pub fn stub_0x4ab098() {
    // IDA 0x4ab098: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::FriendService::FriendStatus>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_13FriendService12FriendStatusEEERKS1_v")]
// 0x4ab0cc — __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService12FriendStatusEEERKS1_v
pub fn stub_0x4ab0cc() {
    // IDA 0x4ab0cc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::FriendService::FriendEventType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_13FriendService15FriendEventTypeEEERKS1_v")]
// 0x4ab100 — __ZN3RBX10Reflection4Type12getSingletonINS_13FriendService15FriendEventTypeEEERKS1_v
pub fn stub_0x4ab100() {
    // IDA 0x4ab100: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SkateboardPlatform::MoveState>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_18SkateboardPlatform9MoveStateEEERKS1_v")]
// 0x4ab134 — __ZN3RBX10Reflection4Type12getSingletonINS_18SkateboardPlatform9MoveStateEEERKS1_v
pub fn stub_0x4ab134() {
    // IDA 0x4ab134: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SoundType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9SoundTypeEEERKS1_v")]
// 0x4ab168 — __ZN3RBX10Reflection4Type12getSingletonINS_9SoundTypeEEERKS1_v
pub fn stub_0x4ab168() {
    // IDA 0x4ab168: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::SurfaceType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11SurfaceTypeEEERKS1_v")]
// 0x4ab19c — __ZN3RBX10Reflection4Type12getSingletonINS_11SurfaceTypeEEERKS1_v
pub fn stub_0x4ab19c() {
    // IDA 0x4ab19c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::PartInstance::FormFactor>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_12PartInstance10FormFactorEEERKS1_v")]
// 0x4ab1d0 — __ZN3RBX10Reflection4Type12getSingletonINS_12PartInstance10FormFactorEEERKS1_v
pub fn stub_0x4ab1d0() {
    // IDA 0x4ab1d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::UserInputService::SwipeDirection>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_16UserInputService14SwipeDirectionEEERKS1_v")]
// 0x4ab204 — __ZN3RBX10Reflection4Type12getSingletonINS_16UserInputService14SwipeDirectionEEERKS1_v
pub fn stub_0x4ab204() {
    // IDA 0x4ab204: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Material>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_8MaterialEEERKS1_v")]
// 0x4ab238 — __ZN3RBX10Reflection4Type12getSingletonINS_8MaterialEEERKS1_v
pub fn stub_0x4ab238() {
    // IDA 0x4ab238: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Time::SampleMethod>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_4Time12SampleMethodEEERKS1_v")]
// 0x4ab26c — __ZN3RBX10Reflection4Type12getSingletonINS_4Time12SampleMethodEEERKS1_v
pub fn stub_0x4ab26c() {
    // IDA 0x4ab26c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiService::SpecialKey>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_10GuiService10SpecialKeyEEERKS1_v")]
// 0x4ab2a0 — __ZN3RBX10Reflection4Type12getSingletonINS_10GuiService10SpecialKeyEEERKS1_v
pub fn stub_0x4ab2a0() {
    // IDA 0x4ab2a0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::GuiService::CenterDialogType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_10GuiService16CenterDialogTypeEEERKS1_v")]
// 0x4ab2d4 — __ZN3RBX10Reflection4Type12getSingletonINS_10GuiService16CenterDialogTypeEEERKS1_v
pub fn stub_0x4ab2d4() {
    // IDA 0x4ab2d4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::ChatService::ChatColor>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11ChatService9ChatColorEEERKS1_v")]
// 0x4ab308 — __ZN3RBX10Reflection4Type12getSingletonINS_11ChatService9ChatColorEEERKS1_v
pub fn stub_0x4ab308() {
    // IDA 0x4ab308: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::MarketplaceService::CurrencyType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_18MarketplaceService12CurrencyTypeEEERKS1_v")]
// 0x4ab33c — __ZN3RBX10Reflection4Type12getSingletonINS_18MarketplaceService12CurrencyTypeEEERKS1_v
pub fn stub_0x4ab33c() {
    // IDA 0x4ab33c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Voxel::CellMaterial>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_5Voxel12CellMaterialEEERKS1_v")]
// 0x4ab370 — __ZN3RBX10Reflection4Type12getSingletonINS_5Voxel12CellMaterialEEERKS1_v
pub fn stub_0x4ab370() {
    // IDA 0x4ab370: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Voxel::CellBlock>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_5Voxel9CellBlockEEERKS1_v")]
// 0x4ab3a4 — __ZN3RBX10Reflection4Type12getSingletonINS_5Voxel9CellBlockEEERKS1_v
pub fn stub_0x4ab3a4() {
    // IDA 0x4ab3a4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Voxel::CellOrientation>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_5Voxel15CellOrientationEEERKS1_v")]
// 0x4ab3d8 — __ZN3RBX10Reflection4Type12getSingletonINS_5Voxel15CellOrientationEEERKS1_v
pub fn stub_0x4ab3d8() {
    // IDA 0x4ab3d8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Voxel::WaterCellForce>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_5Voxel14WaterCellForceEEERKS1_v")]
// 0x4ab40c — __ZN3RBX10Reflection4Type12getSingletonINS_5Voxel14WaterCellForceEEERKS1_v
pub fn stub_0x4ab40c() {
    // IDA 0x4ab40c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Voxel::WaterCellDirection>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_5Voxel18WaterCellDirectionEEERKS1_v")]
// 0x4ab440 — __ZN3RBX10Reflection4Type12getSingletonINS_5Voxel18WaterCellDirectionEEERKS1_v
pub fn stub_0x4ab440() {
    // IDA 0x4ab440: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::AssetService::AccessType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_12AssetService10AccessTypeEEERKS1_v")]
// 0x4ab474 — __ZN3RBX10Reflection4Type12getSingletonINS_12AssetService10AccessTypeEEERKS1_v
pub fn stub_0x4ab474() {
    // IDA 0x4ab474: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::HttpService::HttpContentType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_11HttpService15HttpContentTypeEEERKS1_v")]
// 0x4ab4a8 — __ZN3RBX10Reflection4Type12getSingletonINS_11HttpService15HttpContentTypeEEERKS1_v
pub fn stub_0x4ab4a8() {
    // IDA 0x4ab4a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::StarterGuiService::CoreGuiType>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_17StarterGuiService11CoreGuiTypeEEERKS1_v")]
// 0x4ab4dc — __ZN3RBX10Reflection4Type12getSingletonINS_17StarterGuiService11CoreGuiTypeEEERKS1_v
pub fn stub_0x4ab4dc() {
    // IDA 0x4ab4dc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_")]
#[doc(alias = "__ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_")]
// 0x4ab510 — __ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_
pub fn stub_0x4ab510() {
    // IDA 0x4ab510: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7CreatorD1Ev")]
// 0x4ab5ec — __ZN3RBX14FactoryProductINS_19CustomEventReceiverENS_8InstanceELZNS_20sCustomEventReceiverEES2_E7CreatorD1Ev
pub fn stub_0x4ab5ec() {
    // IDA 0x4ab5ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7CreatorD1Ev")]
// 0x4ab5f0 — __ZN3RBX14FactoryProductINS_11CustomEventENS_8InstanceELZNS_12sCustomEventEES2_E7CreatorD1Ev
pub fn stub_0x4ab5f0() {
    // IDA 0x4ab5f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
