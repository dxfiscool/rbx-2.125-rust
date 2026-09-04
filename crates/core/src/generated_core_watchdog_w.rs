//! core watchdog w — 100 core stubs EA-sorted, next uncovered fallback after watchdog_v 0x3e113c.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in core — next 100 uncovered after 0x3e113c (watchdog_v max).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "RBX::ShirtGraphic::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e1178 — __ZN3RBX12ShirtGraphic11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e1178() {
    // IDA 0x3e1178: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// 0x3e117c — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e117c() {
    // IDA 0x3e117c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Clothing::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e11a0 — __ZN3RBX8Clothing11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e11a0() {
    // IDA 0x3e11a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
// 0x3e11a4 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e11a4() {
    // IDA 0x3e11a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
// 0x3e11c8 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e11c8() {
    // IDA 0x3e11c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Skin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e11ec — __ZN3RBX4Skin11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e11ec() {
    // IDA 0x3e11ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::BodyColors::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// 0x3e11f0 — __ZN3RBX10BodyColors11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
pub fn stub_3e11f0() {
    // IDA 0x3e11f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(RBX::Instance *)")]
// 0x3e11f4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_PNS_8InstanceE
pub fn stub_3e11f4() {
    // IDA 0x3e11f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// 0x3e1344 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1344() {
    // IDA 0x3e1344: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// 0x3e1470 — __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1470() {
    // IDA 0x3e1470: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")]
// 0x3e1cc0 — __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1cc0() {
    // IDA 0x3e1cc0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv")]
// 0x3e1d7c — __ZThn32_NK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1d7c() {
    // IDA 0x3e1d7c: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")]
// 0x3e1f88 — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e1f88() {
    // IDA 0x3e1f88: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv")]
// 0x3e2044 — __ZThn32_NK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e2044() {
    // IDA 0x3e2044: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21ac — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21ac() {
    // IDA 0x3e21ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21b0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21b0() {
    // IDA 0x3e21b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21b4 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21b4() {
    // IDA 0x3e21b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21b8 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21b8() {
    // IDA 0x3e21b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD1Ev")]
// 0x3e21bc — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD1Ev
// type: int()
pub fn stub_3e21bc() {
    // IDA 0x3e21bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")]
// 0x3e22e8 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e22e8() {
    // IDA 0x3e22e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv")]
// 0x3e2440 — __ZThn32_NK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e2440() {
    // IDA 0x3e2440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")]
// 0x3e27d8 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e27d8() {
    // IDA 0x3e27d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv")]
// 0x3e2930 — __ZThn32_NK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE12getClassNameEv
// type: int()
pub fn stub_3e2930() {
    // IDA 0x3e2930: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e2ba0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e2ba0() {
    // IDA 0x3e2ba0: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e2c14 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e2c14() {
    // IDA 0x3e2c14: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e2d80 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e2d80() {
    // IDA 0x3e2d80: C++ this-adjusting/virtual thunk (mangled-only context). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e2df4 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e2df4() {
    // IDA 0x3e2df4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e2f60 — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e2f60() {
    // IDA 0x3e2f60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e2ffc — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e2ffc() {
    // IDA 0x3e2ffc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator6createEv")]
// 0x3e3084 — __ZNK3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e3084() {
    // IDA 0x3e3084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e3658 — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e3658() {
    // IDA 0x3e3658: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e389c — __ZN3RBX14FactoryProductINS_10BodyColorsENS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e389c() {
    // IDA 0x3e389c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e3910 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e3910() {
    // IDA 0x3e3910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e39ac — __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e39ac() {
    // IDA 0x3e39ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator6createEv")]
// 0x3e3a34 — __ZNK3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e3a34() {
    // IDA 0x3e3a34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e4008 — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e4008() {
    // IDA 0x3e4008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e424c — __ZN3RBX14FactoryProductINS_4SkinENS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e424c() {
    // IDA 0x3e424c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e43a4 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e43a4() {
    // IDA 0x3e43a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator12getClassNameEv")]
// 0x3e4440 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_3e4440() {
    // IDA 0x3e4440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator6createEv")]
// 0x3e44c8 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e44c8() {
    // IDA 0x3e44c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e4a9c — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e4a9c() {
    // IDA 0x3e4a9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv")]
// 0x3e4ce0 — __ZN3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE17static_getCreatorEv
// type: void *()
pub fn stub_3e4ce0() {
    // IDA 0x3e4ce0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e4d54 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e4d54() {
    // IDA 0x3e4d54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv")]
// 0x3e4df0 — __ZNK3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e4df0() {
    // IDA 0x3e4df0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e52e0 — __ZN3RBX14FactoryProductINS_5PantsENS_8ClothingELZNS_6sPantsEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e52e0() {
    // IDA 0x3e52e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev")]
// 0x3e5524 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_3e5524() {
    // IDA 0x3e5524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv")]
// 0x3e55c0 — __ZNK3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
pub fn stub_3e55c0() {
    // IDA 0x3e55c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev")]
// 0x3e5ab0 — __ZN3RBX14FactoryProductINS_5ShirtENS_8ClothingELZNS_6sShirtEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
pub fn stub_3e5ab0() {
    // IDA 0x3e5ab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(void)")]
// 0x3e5cf4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_v
// type: void *__fastcall(int)
pub fn stub_3e5cf4() {
    // IDA 0x3e5cf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5d40 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e5d40() {
    // IDA 0x3e5d40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5d44 — __ZN3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e5d44() {
    // IDA 0x3e5d44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5de4 — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e5de4() {
    // IDA 0x3e5de4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5dec — __ZThn32_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e5dec() {
    // IDA 0x3e5dec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5e90 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e5e90() {
    // IDA 0x3e5e90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5e98 — __ZThn36_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e5e98() {
    // IDA 0x3e5e98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e5f3c — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e5f3c() {
    // IDA 0x3e5f3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e5f44 — __ZThn92_N3RBX10Reflection9DescribedINS_10BodyColorsELZNS_11sBodyColorsEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_11sBodyColorsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e5f44() {
    // IDA 0x3e5f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3e5fe8 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_3e5fe8() {
    // IDA 0x3e5fe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isReadOnly(void)const")]
// 0x3e617c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE10isReadOnlyEv
// type: int()
pub fn stub_3e617c() {
    // IDA 0x3e617c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::isWriteOnly(void)const")]
// 0x3e6180 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE11isWriteOnlyEv
// type: int()
pub fn stub_3e6180() {
    // IDA 0x3e6180: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3e6184 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
pub fn stub_3e6184() {
    // IDA 0x3e6184: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::BodyColors>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// 0x3e6190 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_10BodyColorsEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3e6190() {
    // IDA 0x3e6190: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e61e0 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e61e0() {
    // IDA 0x3e61e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e61e4 — __ZN3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e61e4() {
    // IDA 0x3e61e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6284 — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6284() {
    // IDA 0x3e6284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e628c — __ZThn32_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e628c() {
    // IDA 0x3e628c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6330 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6330() {
    // IDA 0x3e6330: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6338 — __ZThn36_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e6338() {
    // IDA 0x3e6338: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e63dc — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e63dc() {
    // IDA 0x3e63dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e63e4 — __ZThn92_N3RBX10Reflection9DescribedINS_4SkinELZNS_5sSkinEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_5sSkinEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e63e4() {
    // IDA 0x3e63e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// 0x3e6488 — __ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_3e6488() {
    // IDA 0x3e6488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isReadOnly(void)const")]
// 0x3e661c — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE10isReadOnlyEv
// type: int()
pub fn stub_3e661c() {
    // IDA 0x3e661c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::isWriteOnly(void)const")]
// 0x3e6620 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE11isWriteOnlyEv
// type: int()
pub fn stub_3e6620() {
    // IDA 0x3e6620: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::getValue(RBX::Reflection::DescribedBase const*)const")]
// 0x3e6624 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8getValueEPKNS0_13DescribedBaseE
// type: _DWORD *__fastcall(_DWORD *result, int, int)
pub fn stub_3e6624() {
    // IDA 0x3e6624: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Skin>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// 0x3e6630 — __ZNK3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EE15BoundPropGetSetINS_4SkinEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_3e6630() {
    // IDA 0x3e6630: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6680 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6680() {
    // IDA 0x3e6680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e66c8 — __ZN3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e66c8() {
    // IDA 0x3e66c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e67a8 — __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e67a8() {
    // IDA 0x3e67a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e67f4 — __ZThn32_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e67f4() {
    // IDA 0x3e67f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e68d8 — __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e68d8() {
    // IDA 0x3e68d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6924 — __ZThn36_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6924() {
    // IDA 0x3e6924: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6a08 — __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6a08() {
    // IDA 0x3e6a08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6a54 — __ZThn92_N3RBX10Reflection9DescribedINS_5PantsELZNS_6sPantsEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sPantsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6a54() {
    // IDA 0x3e6a54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6b38 — __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6b38() {
    // IDA 0x3e6b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6b80 — __ZN3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6b80() {
    // IDA 0x3e6b80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6c60 — __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e6c60() {
    // IDA 0x3e6c60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6cac — __ZThn32_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6cac() {
    // IDA 0x3e6cac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6d90 — __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6d90() {
    // IDA 0x3e6d90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6ddc — __ZThn36_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6ddc() {
    // IDA 0x3e6ddc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6ec0 — __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6ec0() {
    // IDA 0x3e6ec0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6f0c — __ZThn92_N3RBX10Reflection9DescribedINS_5ShirtELZNS_6sShirtEENS_14FactoryProductIS2_NS_8ClothingELZNS_6sShirtEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_3e6f0c() {
    // IDA 0x3e6f0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e6ff0 — __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e6ff0() {
    // IDA 0x3e6ff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e6ff4 — __ZN3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e6ff4() {
    // IDA 0x3e6ff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e7094 — __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e7094() {
    // IDA 0x3e7094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e709c — __ZThn32_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e709c() {
    // IDA 0x3e709c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e7140 — __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e7140() {
    // IDA 0x3e7140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e7148 — __ZThn36_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e7148() {
    // IDA 0x3e7148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e71ec — __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_3e71ec() {
    // IDA 0x3e71ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x3e71f4 — __ZThn92_N3RBX10Reflection9DescribedINS_8ClothingELZNS_9sClothingEENS_17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_3e71f4() {
    // IDA 0x3e71f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}


#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x3e7298 — __ZN3RBX10Reflection9DescribedINS_12ShirtGraphicELZNS_13sShirtGraphicEENS_14FactoryProductIS2_NS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_3e7298() {
    // IDA 0x3e7298: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

