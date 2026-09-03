//! rendering shard watchdog_rendering_w15d4 — 120 stubs EA-sorted asc Material/Texture/Shader (Material split)
//! Filter: Material|Texture|Shader (case-insensitive) demangled/mangled contains
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA dedup vs /tmp/global_eas.txt (85545 filtered) — strict 0 uncovered, fallback 120 EA-sorted asc (Material split)
//! Range: 0x97c0..0x4930cc

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x97c0 — __ZN19CRenderSettingsItem19setTextureCacheSizeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "CRenderSettingsItem::setTextureCacheSize(unsigned int)")]
#[doc(alias = "__ZN19CRenderSettingsItem19setTextureCacheSizeEj")]
// IDA 0x97c0: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x97c0() {
}

// 0xb4f4 — __ZNK3RBX15CRenderSettings19getTextureCacheSizeEv
// type: int __fastcall(RBX::CRenderSettings *this)
#[doc(alias = "RBX::CRenderSettings::getTextureCacheSize(void)const")]
#[doc(alias = "__ZNK3RBX15CRenderSettings19getTextureCacheSizeEv")]
// IDA 0xb4f4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xb4f4() {
}

// 0x38c928 — __ZN3RBX13UserInputBase11setCursorIdEPNS_5AdornERKNS_9TextureIdE
// type: int __fastcall(RBX::UserInputBase *this, RBX::Adorn *, const RBX::TextureId *)
#[doc(alias = "RBX::UserInputBase::setCursorId(RBX::Adorn *,RBX::TextureId const&)")]
#[doc(alias = "__ZN3RBX13UserInputBase11setCursorIdEPNS_5AdornERKNS_9TextureIdE")]
// IDA 0x38c928: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38c928() {
}

// 0x3d362c — __ZN3RBX20ChangeHistoryService7setCellERKN3G3D12Vector3int16ES4_NS_5Voxel4CellENS5_12CellMaterialE
// type: int __fastcall(int)
#[doc(alias = "RBX::ChangeHistoryService::setCell(G3D::Vector3int16 const&,G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)")]
#[doc(alias = "__ZN3RBX20ChangeHistoryService7setCellERKN3G3D12Vector3int16ES4_NS_5Voxel4CellENS5_12CellMaterialE")]
// IDA 0x3d362c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3d362c() {
}

// 0x3e0048 — __ZN3RBX5Shirt11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Shirt::setTemplate(RBX::TextureId)")]
#[doc(alias = "__ZN3RBX5Shirt11setTemplateENS_9TextureIdE")]
// IDA 0x3e0048: 10 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e0048() {
}

// 0x3e0068 — __ZN3RBX5Pants11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Pants::setTemplate(RBX::TextureId)")]
#[doc(alias = "__ZN3RBX5Pants11setTemplateENS_9TextureIdE")]
// IDA 0x3e0068: 10 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e0068() {
}

// 0x3e117c — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev")]
// IDA 0x3e117c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x3e117c() {
}

// 0x3e11a4 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev")]
// IDA 0x3e11a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x3e11a4() {
}

// 0x3e11c8 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev")]
// IDA 0x3e11c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x3e11c8() {
}

// 0x3e7540 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x3e7540: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7540() {
}

// 0x3e7654 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED0Ev")]
// IDA 0x3e7654: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x3e7654() {
}

// 0x3e7680 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
// IDA 0x3e7680: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7680() {
}

// 0x3e7684 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
// IDA 0x3e7684: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7684() {
}

// 0x3e7688 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x3e7688: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7688() {
}

// 0x3e76b0 — __ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
// IDA 0x3e76b0: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e76b0() {
}

// 0x3e77f8 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x3e77f8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e77f8() {
}

// 0x3e790c — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED0Ev")]
// IDA 0x3e790c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x3e790c() {
}

// 0x3e7938 — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
// IDA 0x3e7938: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7938() {
}

// 0x3e793c — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
// IDA 0x3e793c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e793c() {
}

// 0x3e7940 — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x3e7940: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7940() {
}

// 0x3e7968 — __ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
// IDA 0x3e7968: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7968() {
}

// 0x3e7ab0 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_8ClothingEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Clothing>(char const*,char const*,RBX::TextureId RBX::Clothing::*,void (RBX::Clothing::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_8ClothingEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")]
// IDA 0x3e7ab0: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7ab0() {
}

// 0x3e7c44 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED0Ev")]
// IDA 0x3e7c44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x3e7c44() {
}

// 0x3e7c70 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE10isReadOnlyEv")]
// IDA 0x3e7c70: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7c70() {
}

// 0x3e7c74 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE11isWriteOnlyEv")]
// IDA 0x3e7c74: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7c74() {
}

// 0x3e7c78 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *this, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x3e7c78: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7c78() {
}

// 0x3e7c9c — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Clothing>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_8ClothingEE8setValueEPNS0_13DescribedBaseERKS2_")]
// IDA 0x3e7c9c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7c9c() {
}

// 0x3e7d10 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_12ShirtGraphicEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ShirtGraphic>(char const*,char const*,RBX::TextureId RBX::ShirtGraphic::*,void (RBX::ShirtGraphic::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_12ShirtGraphicEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")]
// IDA 0x3e7d10: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7d10() {
}

// 0x3e7ea4 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE10isReadOnlyEv")]
// IDA 0x3e7ea4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7ea4() {
}

// 0x3e7ea8 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE11isWriteOnlyEv")]
// IDA 0x3e7ea8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7ea8() {
}

// 0x3e7eac — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *this, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x3e7eac: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7eac() {
}

// 0x3e7ed0 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ShirtGraphic>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_12ShirtGraphicEE8setValueEPNS0_13DescribedBaseERKS2_")]
// IDA 0x3e7ed0: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e7ed0() {
}

// 0x3e8b74 — __ZNK3RBX13CharacterMesh16getBaseTextureIdEv
// type: void __fastcall(RBX::CharacterMesh *this, int)
#[doc(alias = "RBX::CharacterMesh::getBaseTextureId(void)const")]
#[doc(alias = "__ZNK3RBX13CharacterMesh16getBaseTextureIdEv")]
// IDA 0x3e8b74: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e8b74() {
}

// 0x3e8cb0 — __ZNK3RBX13CharacterMesh19getOverlayTextureIdEv
// type: void __fastcall(RBX::CharacterMesh *this, int)
#[doc(alias = "RBX::CharacterMesh::getOverlayTextureId(void)const")]
#[doc(alias = "__ZNK3RBX13CharacterMesh19getOverlayTextureIdEv")]
// IDA 0x3e8cb0: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3e8cb0() {
}

// 0x404328 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::~TToolVerb()")]
#[doc(alias = "__ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED1Ev")]
// IDA 0x404328: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x404328() {
}

// 0x407994 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
#[doc(alias = "__ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")]
// IDA 0x407994: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407994() {
}

// 0x407b18 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::~TToolVerb()")]
#[doc(alias = "__ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED0Ev")]
// IDA 0x407b18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x407b18() {
}

// 0x407bb8 — __ZNK3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::isChecked(void)const")]
#[doc(alias = "__ZNK3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE9isCheckedEv")]
// IDA 0x407bb8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407bb8() {
}

// 0x407bf0 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
#[doc(alias = "__ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")]
// IDA 0x407bf0: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407bf0() {
}

// 0x407d04 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::newMouseCommand(void)")]
#[doc(alias = "__ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE15newMouseCommandEv")]
// IDA 0x407d04: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407d04() {
}

// 0x407dd0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12MaterialToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::MaterialTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MaterialTool,RBX::Workspace *>(RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::MaterialTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MaterialTool,RBX::Workspace *>(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12MaterialToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_")]
// IDA 0x407dd0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407dd0() {
}

// 0x407ea8 — __ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv")]
// IDA 0x407ea8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x407ea8() {
}

// 0x407eac — __ZNK3RBX12MaterialTool8isStickyEv
// type: void __fastcall(RBX::MaterialTool *this, int)
#[doc(alias = "RBX::MaterialTool::isSticky(void)const")]
#[doc(alias = "__ZNK3RBX12MaterialTool8isStickyEv")]
// IDA 0x407eac: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407eac() {
}

// 0x407f74 — __ZNK3RBX12MaterialTool13getCursorNameEv
// type: int __fastcall(RBX::MaterialTool *this)
#[doc(alias = "RBX::MaterialTool::getCursorName(void)const")]
#[doc(alias = "__ZNK3RBX12MaterialTool13getCursorNameEv")]
// IDA 0x407f74: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407f74() {
}

// 0x407f90 — __ZN5boost10shared_ptrIN3RBX12MaterialToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::MaterialTool>::shared_ptr<RBX::MaterialTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::MaterialTool>::shared_ptr<RBX::MaterialTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12MaterialToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
// IDA 0x407f90: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x407f90() {
}

// 0x408058 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12MaterialToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MaterialTool,RBX::MaterialTool>(boost::shared_ptr<RBX::MaterialTool> const*,RBX::MaterialTool *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MaterialTool,RBX::MaterialTool>(rbx_core::SharedPtr<RBX::MaterialTool> const*,RBX::MaterialTool *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12MaterialToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x408058: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x408058() {
}

// 0x40813c — __ZN5boost6detail12shared_countC2IPN3RBX12MaterialToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12MaterialToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
// IDA 0x40813c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x40813c() {
}

// 0x408234 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
// IDA 0x408234: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x408234() {
}

// 0x408238 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
// IDA 0x408238: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x408238() {
}

// 0x40823c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
// IDA 0x40823c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x40823c() {
}

// 0x40824c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x40824c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x40824c() {
}

// 0x408264 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x408264: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x408264() {
}

// 0x408268 — __ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v")]
// IDA 0x408268: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x408268() {
}

// 0x4082ac — __ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv")]
// IDA 0x4082ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x4082ac() {
}

// 0x4082b0 — __ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v")]
// IDA 0x4082b0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4082b0() {
}

// 0x47b4cc — __ZNK3RBX13DebugSettings20getVertexShaderModelEv
// type: int __fastcall(RBX::DebugSettings *this)
#[doc(alias = "RBX::DebugSettings::getVertexShaderModel(void)const")]
#[doc(alias = "__ZNK3RBX13DebugSettings20getVertexShaderModelEv")]
// IDA 0x47b4cc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x47b4cc() {
}

// 0x47b4d0 — __ZNK3RBX13DebugSettings19getPixelShaderModelEv
// type: int __fastcall(RBX::DebugSettings *this)
#[doc(alias = "RBX::DebugSettings::getPixelShaderModel(void)const")]
#[doc(alias = "__ZNK3RBX13DebugSettings19getPixelShaderModelEv")]
// IDA 0x47b4d0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x47b4d0() {
}

// 0x48f7f4 — __ZN3RBX5Decal10setTextureENS_9TextureIdE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Decal::setTexture(RBX::TextureId)")]
#[doc(alias = "__ZN3RBX5Decal10setTextureENS_9TextureIdE")]
// IDA 0x48f7f4: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x48f7f4() {
}

// 0x48fb04 — __ZN3RBX15StringConverterINS_9TextureIdEE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *)
#[doc(alias = "RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_9TextureIdEE14convertToValueERKSsRS1_")]
// IDA 0x48fb04: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x48fb04() {
}

// 0x48fc28 — __ZN3RBX10Reflection4Type12getSingletonINS_9TextureIdEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextureId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_9TextureIdEEERKS1_v")]
// IDA 0x48fc28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x48fc28() {
}

// 0x48fc2c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
// IDA 0x48fc2c: 178 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x48fc2c() {
}

// 0x48fe14 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
// IDA 0x48fe14: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x48fe14() {
}

// 0x48ffbc — __ZN3RBX10Reflection7Variant7convertINS_9TextureIdEEERT_v
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::convert<RBX::TextureId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant7convertINS_9TextureIdEEERT_v")]
// IDA 0x48ffbc: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x48ffbc() {
}

// 0x4901a8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11getDataSizeEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE11getDataSizeEPKNS0_13DescribedBaseE")]
// IDA 0x4901a8: 34 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4901a8() {
}

// 0x490204 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14hasStringValueEv")]
// IDA 0x490204: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x490204() {
}

// 0x490208 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14getStringValueEPKNS0_13DescribedBaseE")]
// IDA 0x490208: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x490208() {
}

// 0x490324 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection23TypedPropertyDescriptorINS_9TextureIdEE14setStringValueEPNS0_13DescribedBaseERKSs")]
// IDA 0x490324: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x490324() {
}

// 0x49047c — __ZN3RBX7Texture16setStudsPerTileUEf
// type: _DWORD __fastcall(RBX::Texture *__hidden this, float)
#[doc(alias = "RBX::Texture::setStudsPerTileU(float)")]
#[doc(alias = "__ZN3RBX7Texture16setStudsPerTileUEf")]
// IDA 0x49047c: 14 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x49047c() {
}

// 0x4904b0 — __ZN3RBX7Texture16setStudsPerTileVEf
// type: _DWORD __fastcall(RBX::Texture *__hidden this, float)
#[doc(alias = "RBX::Texture::setStudsPerTileV(float)")]
#[doc(alias = "__ZN3RBX7Texture16setStudsPerTileVEf")]
// IDA 0x4904b0: 14 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4904b0() {
}

// 0x4904e4 — __ZN3RBX7TextureC2Ev
// type: RBX::Decal *__fastcall(RBX::Texture *this)
#[doc(alias = "RBX::Texture::Texture(void)")]
#[doc(alias = "__ZN3RBX7TextureC2Ev")]
// IDA 0x4904e4: 220 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4904e4() {
}

// 0x49076c — __ZNK3RBX5Decal10getTextureEv
// type: _DWORD __fastcall(RBX::Decal *__hidden this)
#[doc(alias = "RBX::Decal::getTexture(void)const")]
#[doc(alias = "__ZNK3RBX5Decal10getTextureEv")]
// IDA 0x49076c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x49076c() {
}

// 0x490770 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED1Ev")]
// IDA 0x490770: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x490770() {
}

// 0x4907d0 — __ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v
// type: int(void)
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v")]
// IDA 0x4907d0: 166 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4907d0() {
}

// 0x490a7c — __ZNK3RBX7Texture16getStudsPerTileUEv
// type: _DWORD __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::getStudsPerTileU(void)const")]
#[doc(alias = "__ZNK3RBX7Texture16getStudsPerTileUEv")]
// IDA 0x490a7c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x490a7c() {
}

// 0x490a84 — __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED1Ev")]
// IDA 0x490a84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x490a84() {
}

// 0x490aa8 — __ZNK3RBX7Texture16getStudsPerTileVEv
// type: _DWORD __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::getStudsPerTileV(void)const")]
#[doc(alias = "__ZNK3RBX7Texture16getStudsPerTileVEv")]
// IDA 0x490aa8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x490aa8() {
}

// 0x490ab4 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0x490ab4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x490ab4() {
}

// 0x490e34 — __ZN3RBX7TextureD1Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::~Texture()")]
#[doc(alias = "__ZN3RBX7TextureD1Ev")]
// IDA 0x490e34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x490e34() {
}

// 0x490e74 — __ZN3RBX7TextureD0Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "RBX::Texture::~Texture()")]
#[doc(alias = "__ZN3RBX7TextureD0Ev")]
// IDA 0x490e74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x490e74() {
}

// 0x490f50 — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
// IDA 0x490f50: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x490f50() {
}

// 0x490f60 — __ZThn32_N3RBX7TextureD1Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
#[doc(alias = "__ZThn32_N3RBX7TextureD1Ev")]
// IDA 0x490f60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x490f60() {
}

// 0x490fa4 — __ZThn32_N3RBX7TextureD0Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
#[doc(alias = "__ZThn32_N3RBX7TextureD0Ev")]
// IDA 0x490fa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x490fa4() {
}

// 0x491080 — __ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE12getClassNameEv")]
// IDA 0x491080: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491080() {
}

// 0x491090 — __ZThn36_N3RBX7TextureD1Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
#[doc(alias = "__ZThn36_N3RBX7TextureD1Ev")]
// IDA 0x491090: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x491090() {
}

// 0x4910d4 — __ZThn36_N3RBX7TextureD0Ev
// type: void __fastcall(RBX::Texture *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
#[doc(alias = "__ZThn36_N3RBX7TextureD0Ev")]
// IDA 0x4910d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x4910d4() {
}

// 0x4911b0 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE17static_getCreatorEv")]
// IDA 0x4911b0: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4911b0() {
}

// 0x491224 — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0x491224: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491224() {
}

// 0x4912ac — __ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sTextureEEEEvv")]
// IDA 0x4912ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x4912ac() {
}

// 0x4912b0 — __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")]
// IDA 0x4912b0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4912b0() {
}

// 0x491570 — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0x491570: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x491570() {
}

// 0x49160c — __ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7Creator6createEv")]
// IDA 0x49160c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x49160c() {
}

// 0x491750 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextureEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Texture> RBX::Creatable<RBX::Instance>::create<RBX::Texture>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Texture> RBX::Creatable<RBX::Instance>::create<RBX::Texture>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_7TextureEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x491750: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491750() {
}

// 0x491800 — __ZN5boost10shared_ptrIN3RBX7TextureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Texture>::shared_ptr<RBX::Texture,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::Texture>::shared_ptr<RBX::Texture,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7TextureEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x491800: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491800() {
}

// 0x4918c8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(boost::shared_ptr<RBX::Texture> const*,RBX::Texture *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(rbx_core::SharedPtr<RBX::Texture> const*,RBX::Texture *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x4918c8: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4918c8() {
}

// 0x4919b0 — __ZN5boost6detail12shared_countC2IPN3RBX7TextureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX7TextureENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x4919b0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4919b0() {
}

// 0x491ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x491ab8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x491ab8() {
}

// 0x491abc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x491abc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x491abc() {
}

// 0x491ac0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x491ac0: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491ac0() {
}

// 0x491ae0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x491ae0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491ae0() {
}

// 0x491af8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Texture *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7TextureENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x491af8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491af8() {
}

// 0x491afc — __ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_7TextureENS_5DecalELZNS_8sTextureEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0x491afc: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x491afc() {
}

// 0x492510 — __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x492510: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x492510() {
}

// 0x492550 — __ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x492550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x492550() {
}

// 0x49262c — __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x49262c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x49262c() {
}

// 0x492670 — __ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x492670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x492670() {
}

// 0x49274c — __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x49274c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x49274c() {
}

// 0x492790 — __ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7TextureELZNS_8sTextureEENS_14FactoryProductIS2_NS_5DecalELZNS_8sTextureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x492790: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x492790() {
}

// 0x49286c — __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::PropDescriptor<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>(char const*,char const*,float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x49286c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x49286c() {
}

// 0x492980 — __ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7TextureEfED0Ev")]
// IDA 0x492980: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x492980() {
}

// 0x4929ac — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
// IDA 0x4929ac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4929ac() {
}

// 0x4929b0 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
// IDA 0x4929b0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4929b0() {
}

// 0x4929b4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x4929b4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4929b4() {
}

// 0x4929d4 — __ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::GetSetImpl<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7TextureEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
// IDA 0x4929d4: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4929d4() {
}

// 0x4929f8 — __ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "RBX::TextureId * rbx::any_cast<RBX::TextureId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX9TextureIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// IDA 0x4929f8: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4929f8() {
}

// 0x492a50 — __ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "RBX::TextureId & rbx::any_cast<RBX::TextureId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// IDA 0x492a50: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x492a50() {
}

// 0x492f84 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x492f84: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x492f84() {
}

// 0x493098 — __ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEED0Ev")]
// IDA 0x493098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x493098() {
}

// 0x4930c4 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE10isReadOnlyEv")]
// IDA 0x4930c4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4930c4() {
}

// 0x4930c8 — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
// IDA 0x4930c8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4930c8() {
}

// 0x4930cc — __ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::GetSetImpl<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEE10GetSetImplIMS2_KFRKS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x4930cc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x4930cc() {
}
