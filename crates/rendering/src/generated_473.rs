//! rendering shard 473 — 120 stubs 0x86c528..0x884358 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Render filtered complete -> global gap filler distinct not yet in rendering, 57695->57815 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x86c528 — __ZN3RBX19MegaClusterInstance19autoWedgeCellScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, int, int, int)
#[doc(alias = "RBX::MegaClusterInstance::autoWedgeCellScript(int,int,int)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance19autoWedgeCellScriptEiii")]
// IDA 0x86c528: 440 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86c528() {
}

// 0x86c9b8 — __ZN3RBX19MegaClusterInstance20autoWedgeCellsScriptENS_12Region3int16E
#[doc(alias = "RBX::MegaClusterInstance::autoWedgeCellsScript(RBX::Region3int16)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance20autoWedgeCellsScriptENS_12Region3int16E")]
// IDA 0x86c9b8: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86c9b8() {
}

// 0x86ca30 — __ZN3RBX19MegaClusterInstance23cellCenterToWorldScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, int, int, int)
#[doc(alias = "RBX::MegaClusterInstance::cellCenterToWorldScript(int,int,int)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance23cellCenterToWorldScriptEiii")]
// IDA 0x86ca30: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ca30() {
}

// 0x86cbbc — __ZN3RBX19MegaClusterInstance5clearEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::clear(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance5clearEv")]
// IDA 0x86cbbc: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86cbbc() {
}

// 0x86cc5c — __ZN3RBX19MegaClusterInstance16countCellsScriptEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::countCellsScript(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance16countCellsScriptEv")]
// IDA 0x86cc5c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86cc5c() {
}

// 0x86cc64 — __ZNK3RBX19MegaClusterInstance9CellChunk12getConstDataEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance::CellChunk *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::CellChunk::getConstData(void)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance9CellChunk12getConstDataEv")]
// IDA 0x86cc64: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86cc64() {
}

// 0x86cdc4 — __ZN3RBX19MegaClusterInstance13destroyJointsEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::destroyJoints(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance13destroyJointsEv")]
// IDA 0x86cdc4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_86cdc4() {
}

// 0x86cdc8 — __ZN3RBX19MegaClusterInstance8luaCloneEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::luaClone(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance8luaCloneEv")]
// IDA 0x86cdc8: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86cdc8() {
}

// 0x86cee8 — __ZN3RBX19MegaClusterInstance7destroyEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::destroy(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance7destroyEv")]
// IDA 0x86cee8: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86cee8() {
}

// 0x86d008 — __ZN3RBX19MegaClusterInstance4joinEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::join(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance4joinEv")]
// IDA 0x86d008: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_86d008() {
}

// 0x86d00c — __ZN3RBX19MegaClusterInstance6resizeENS_8NormalIdEi
#[doc(alias = "RBX::MegaClusterInstance::resize(RBX::NormalId,int)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance6resizeENS_8NormalIdEi")]
// IDA 0x86d00c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86d00c() {
}

// 0x86d12c — __ZNK3RBX19MegaClusterInstance10encodeDataEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::encodeData(void)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance10encodeDataEv")]
// IDA 0x86d12c: 287 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86d12c() {
}

// 0x86d42c — __ZN3RBX19MegaClusterInstance10decodeDataERKSs
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, const std::string *)
#[doc(alias = "RBX::MegaClusterInstance::decodeData(std::string const&)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance10decodeDataERKSs")]
// IDA 0x86d42c: 151 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86d42c() {
}

// 0x86d5d0 — __ZN3RBX19MegaClusterInstanceC1Ev
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::MegaClusterInstance(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstanceC1Ev")]
// IDA 0x86d5d0: 418 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86d5d0() {
}

// 0x86da74 — __ZN3RBX19MegaClusterInstanceD0Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZN3RBX19MegaClusterInstanceD0Ev")]
// IDA 0x86da74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86da74() {
}

// 0x86db20 — __ZN3RBX19MegaClusterInstanceD1Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZN3RBX19MegaClusterInstanceD1Ev")]
// IDA 0x86db20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86db20() {
}

// 0x86db30 — __ZThn32_N3RBX19MegaClusterInstanceD0Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZThn32_N3RBX19MegaClusterInstanceD0Ev")]
// IDA 0x86db30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86db30() {
}

// 0x86db38 — __ZThn36_N3RBX19MegaClusterInstanceD0Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZThn36_N3RBX19MegaClusterInstanceD0Ev")]
// IDA 0x86db38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86db38() {
}

// 0x86db40 — __ZThn132_N3RBX19MegaClusterInstanceD0Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZThn132_N3RBX19MegaClusterInstanceD0Ev")]
// IDA 0x86db40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86db40() {
}

// 0x86db48 — __ZN3RBX19MegaClusterInstanceD2Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZN3RBX19MegaClusterInstanceD2Ev")]
// IDA 0x86db48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86db48() {
}

// 0x86de38 — __ZThn32_N3RBX19MegaClusterInstanceD1Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZThn32_N3RBX19MegaClusterInstanceD1Ev")]
// IDA 0x86de38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86de38() {
}

// 0x86de48 — __ZThn36_N3RBX19MegaClusterInstanceD1Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZThn36_N3RBX19MegaClusterInstanceD1Ev")]
// IDA 0x86de48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86de48() {
}

// 0x86de58 — __ZThn132_N3RBX19MegaClusterInstanceD1Ev
// type: void __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MegaClusterInstance::~MegaClusterInstance()")]
#[doc(alias = "__ZThn132_N3RBX19MegaClusterInstanceD1Ev")]
// IDA 0x86de58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86de58() {
}

// 0x86de68 — __ZN3RBX19MegaClusterInstance11setAnchoredEb
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, bool)
#[doc(alias = "RBX::MegaClusterInstance::setAnchored(bool)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance11setAnchoredEb")]
// IDA 0x86de68: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86de68() {
}

// 0x86de98 — __ZN3RBX19MegaClusterInstance14initDimensionsEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::initDimensions(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance14initDimensionsEv")]
// IDA 0x86de98: 83 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86de98() {
}

// 0x86dfec — __ZNK3RBX19MegaClusterInstance15verifySetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::MegaClusterInstance::verifySetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance15verifySetParentEPKNS_8InstanceE")]
// IDA 0x86dfec: 81 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86dfec() {
}

// 0x86e0f4 — __ZNK3RBX19MegaClusterInstance11getPartTypeEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::getPartType(void)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance11getPartTypeEv")]
// IDA 0x86e0f4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e0f4() {
}

// 0x86e0f8 — __ZNK3RBX19MegaClusterInstance11isAllocatedEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::isAllocated(void)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance11isAllocatedEv")]
// IDA 0x86e0f8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e0f8() {
}

// 0x86e124 — __ZN3RBX19MegaClusterInstance7doAllocEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::doAlloc(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance7doAllocEv")]
// IDA 0x86e124: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e124() {
}

// 0x86e1b4 — __ZN3RBX19MegaClusterInstance17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::MegaClusterInstance::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance17onServiceProviderEPNS_15ServiceProviderES2_")]
// IDA 0x86e1b4: 45 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e1b4() {
}

// 0x86e230 — __ZN3RBX19MegaClusterInstance17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::MegaClusterInstance::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance17onAncestorChangedERKNS_15AncestorChangedE")]
// IDA 0x86e230: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e230() {
}

// 0x86e93c — __ZN3RBX19MegaClusterInstance22autoWedgeCellsInternalENS_12Region3int16E
#[doc(alias = "RBX::MegaClusterInstance::autoWedgeCellsInternal(RBX::Region3int16)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance22autoWedgeCellsInternalENS_12Region3int16E")]
// IDA 0x86e93c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e93c() {
}

// 0x86ed00 — __ZNK3RBX19MegaClusterInstance14getSizeInCellsEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::getSizeInCells(void)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance14getSizeInCellsEv")]
// IDA 0x86ed00: 5 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ed00() {
}

// 0x86ed10 — __ZN3RBX19MegaClusterInstance15connectListenerEPNS_5Voxel18CellChangeListenerE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::MegaClusterInstance::connectListener(RBX::Voxel::CellChangeListener *)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance15connectListenerEPNS_5Voxel18CellChangeListenerE")]
// IDA 0x86ed10: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ed10() {
}

// 0x86ed54 — __ZN3RBX19MegaClusterInstance18disconnectListenerEPNS_5Voxel18CellChangeListenerE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::MegaClusterInstance::disconnectListener(RBX::Voxel::CellChangeListener *)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance18disconnectListenerEPNS_5Voxel18CellChangeListenerE")]
// IDA 0x86ed54: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ed54() {
}

// 0x86eda4 — __ZN3RBX19MegaClusterInstance10getSurfaceERKNS_6RbxRayERi
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, const RBX::RbxRay *, int *)
#[doc(alias = "RBX::MegaClusterInstance::getSurface(RBX::RbxRay const&,int &)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance10getSurfaceERKNS_6RbxRayERi")]
// IDA 0x86eda4: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86eda4() {
}

// 0x86ee2c — __ZNK3RBX19MegaClusterInstance15encodeChunkDataERKNS_7CellBoxE
#[doc(alias = "RBX::MegaClusterInstance::encodeChunkData(RBX::CellBox const&)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance15encodeChunkDataERKNS_7CellBoxE")]
// IDA 0x86ee2c: 101 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ee2c() {
}

// 0x86ef4c — __ZNK3RBX19MegaClusterInstance15encodeChunkDataERKNS_5Voxel6RegionINS1_4Grid5ChunkEEE
#[doc(alias = "RBX::MegaClusterInstance::encodeChunkData(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance15encodeChunkDataERKNS_5Voxel6RegionINS1_4Grid5ChunkEEE")]
// IDA 0x86ef4c: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86ef4c() {
}

// 0x86f068 — __ZN3RBX19MegaClusterInstance23decodeDataV1_DeprecatedERKSs
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, const std::string *)
#[doc(alias = "RBX::MegaClusterInstance::decodeDataV1_Deprecated(std::string const&)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance23decodeDataV1_DeprecatedERKSs")]
// IDA 0x86f068: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f068() {
}

// 0x86fdc0 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::addPair(RBX::Voxel::CellBlock,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE7addPairES3_PKc")]
// IDA 0x86fdc0: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86fdc0() {
}

// 0x870120 — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel9CellBlockEEERT_v
#[doc(alias = "RBX::Voxel::CellBlock & RBX::Reflection::Variant::genericConvert<RBX::Voxel::CellBlock>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel9CellBlockEEERT_v")]
// IDA 0x870120: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_870120() {
}

// 0x87030c — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::addPair(RBX::Voxel::CellOrientation,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE7addPairES3_PKc")]
// IDA 0x87030c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87030c() {
}

// 0x87066c — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel15CellOrientationEEERT_v
#[doc(alias = "RBX::Voxel::CellOrientation & RBX::Reflection::Variant::genericConvert<RBX::Voxel::CellOrientation>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel15CellOrientationEEERT_v")]
// IDA 0x87066c: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87066c() {
}

// 0x870858 — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::addPair(RBX::Voxel::WaterCellForce,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE7addPairES3_PKc")]
// IDA 0x870858: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_870858() {
}

// 0x870bb8 — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel14WaterCellForceEEERT_v
#[doc(alias = "RBX::Voxel::WaterCellForce & RBX::Reflection::Variant::genericConvert<RBX::Voxel::WaterCellForce>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel14WaterCellForceEEERT_v")]
// IDA 0x870bb8: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_870bb8() {
}

// 0x870da4 — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::addPair(RBX::Voxel::WaterCellDirection,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE7addPairES3_PKc")]
// IDA 0x870da4: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_870da4() {
}

// 0x871104 — __ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel18WaterCellDirectionEEERT_v
#[doc(alias = "RBX::Voxel::WaterCellDirection & RBX::Reflection::Variant::genericConvert<RBX::Voxel::WaterCellDirection>(void)")]
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5Voxel18WaterCellDirectionEEERT_v")]
// IDA 0x871104: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_871104() {
}

// 0x8712f0 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceESsED1Ev")]
// IDA 0x8712f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8712f0() {
}

// 0x871314 — __ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::MegaClusterInstance,RBX::Region3int16>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_19MegaClusterInstanceENS_12Region3int16EED1Ev")]
// IDA 0x871314: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_871314() {
}

// 0x871338 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EED1Ev
// was: __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,boost::shared_ptr<RBX::Reflection::Tuple const> ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFN5boost10shared_ptrIKNS0_5TupleEEEiiiELi3EED1Ev")]
// IDA 0x871338: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_871338() {
}

// 0x8713ec — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection),5>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFviiiNS_5Voxel14WaterCellForceENS3_18WaterCellDirectionEELi5EED1Ev")]
// IDA 0x8713ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8713ec() {
}

// 0x871454 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,bool ()(int,int,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFbiiiELi3EED1Ev")]
// IDA 0x871454: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_871454() {
}

// 0x8714a8 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(RBX::Region3int16),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvNS_12Region3int16EELi1EED1Ev")]
// IDA 0x8714a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8714a8() {
}

// 0x8714e8 — __ZN3RBX19MegaClusterInstance23cellCornerToWorldScriptEiii
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, int, int, int)
#[doc(alias = "RBX::MegaClusterInstance::cellCornerToWorldScript(int,int,int)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance23cellCornerToWorldScriptEiii")]
// IDA 0x8714e8: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8714e8() {
}

// 0x87159c — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFvvELi0EED1Ev")]
// IDA 0x87159c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_87159c() {
}

// 0x8715c0 — __ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::MegaClusterInstance,int ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_19MegaClusterInstanceEFivELi0EED1Ev")]
// IDA 0x8715c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8715c0() {
}

// 0x871618 — __ZNSt6vectorIhSaIhEE6resizeEmh
#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::resize(unsigned long,unsigned char)")]
#[doc(alias = "__ZNSt6vectorIhSaIhEE6resizeEmh")]
// IDA 0x871618: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_871618() {
}

// 0x8716d4 — __ZNK3RBX19MegaClusterInstance25encodeChunkDataIntoStreamINS_17StringWriteBufferEEEvRKNS_7CellBoxERT_
#[doc(alias = "void RBX::MegaClusterInstance::encodeChunkDataIntoStream<RBX::StringWriteBuffer>(RBX::CellBox const&,RBX::StringWriteBuffer &)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance25encodeChunkDataIntoStreamINS_17StringWriteBufferEEEvRKNS_7CellBoxERT_")]
// IDA 0x8716d4: 187 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8716d4() {
}

// 0x8718dc — __ZNK3RBX19MegaClusterInstance25encodeChunkDataIntoStreamINS_17StringWriteBufferEEEvRKNS_5Voxel6RegionINS3_4Grid5ChunkEEERT_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "void RBX::MegaClusterInstance::encodeChunkDataIntoStream<RBX::StringWriteBuffer>(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&,RBX::StringWriteBuffer &)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance25encodeChunkDataIntoStreamINS_17StringWriteBufferEEEvRKNS_5Voxel6RegionINS3_4Grid5ChunkEEERT_")]
// IDA 0x8718dc: 320 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8718dc() {
}

// 0x8722f0 — __ZN3RBX19MegaClusterInstance15setIsArchivableEb
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, bool)
#[doc(alias = "RBX::MegaClusterInstance::setIsArchivable(bool)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance15setIsArchivableEb")]
// IDA 0x8722f0: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8722f0() {
}

// 0x8722f8 — __ZN3RBX19MegaClusterInstance7setNameERKSs
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, const std::string *)
#[doc(alias = "RBX::MegaClusterInstance::setName(std::string const&)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance7setNameERKSs")]
// IDA 0x8722f8: 3 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8722f8() {
}

// 0x872308 — __ZNK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE12getClassNameEv")]
// IDA 0x872308: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872308() {
}

// 0x872318 — __ZNK3RBX19MegaClusterInstance19getResizeHandleMaskEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::getResizeHandleMask(void)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance19getResizeHandleMaskEv")]
// IDA 0x872318: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872318() {
}

// 0x87232c — __ZNK3RBX19MegaClusterInstance23getDragUtilitiesSupportEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::getDragUtilitiesSupport(void)const")]
#[doc(alias = "__ZNK3RBX19MegaClusterInstance23getDragUtilitiesSupportEv")]
// IDA 0x87232c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87232c() {
}

// 0x872350 — __ZN3RBX19MegaClusterInstance11setFrictionEf
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, float)
#[doc(alias = "RBX::MegaClusterInstance::setFriction(float)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance11setFrictionEf")]
// IDA 0x872350: 2 insns (MOV.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872350() {
}

// 0x872358 — __ZN3RBX19MegaClusterInstance13setElasticityEf
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, float)
#[doc(alias = "RBX::MegaClusterInstance::setElasticity(float)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance13setElasticityEf")]
// IDA 0x872358: 2 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872358() {
}

// 0x872364 — __ZN3RBX19MegaClusterInstance13setCanCollideEb
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, bool)
#[doc(alias = "RBX::MegaClusterInstance::setCanCollide(bool)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance13setCanCollideEb")]
// IDA 0x872364: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872364() {
}

// 0x872374 — __ZN3RBX19MegaClusterInstance15setTransparencyEf
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, float)
#[doc(alias = "RBX::MegaClusterInstance::setTransparency(float)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance15setTransparencyEf")]
// IDA 0x872374: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872374() {
}

// 0x87237c — __ZN3RBX19MegaClusterInstance14setReflectanceEf
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, float)
#[doc(alias = "RBX::MegaClusterInstance::setReflectance(float)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance14setReflectanceEf")]
// IDA 0x87237c: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87237c() {
}

// 0x872384 — __ZN3RBX19MegaClusterInstance8setColorENS_10BrickColorE
#[doc(alias = "RBX::MegaClusterInstance::setColor(RBX::BrickColor)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance8setColorENS_10BrickColorE")]
// IDA 0x872384: 4 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872384() {
}

// 0x872394 — __ZN3RBX19MegaClusterInstance13setPartLockedEb
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this, bool)
#[doc(alias = "RBX::MegaClusterInstance::setPartLocked(bool)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance13setPartLockedEb")]
// IDA 0x872394: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872394() {
}

// 0x87239c — __ZN3RBX19MegaClusterInstance14isSelectable3dEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "RBX::MegaClusterInstance::isSelectable3d(void)")]
#[doc(alias = "__ZN3RBX19MegaClusterInstance14isSelectable3dEv")]
// IDA 0x87239c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87239c() {
}

// 0x8723a0 — __ZThn32_NK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE12getClassNameEv")]
// IDA 0x8723a0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8723a0() {
}

// 0x8723b0 — __ZThn148_N3RBX19MegaClusterInstance14isSelectable3dEv
// type: _DWORD __fastcall(RBX::MegaClusterInstance *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::MegaClusterInstance::isSelectable3d(void)")]
#[doc(alias = "__ZThn148_N3RBX19MegaClusterInstance14isSelectable3dEv")]
// IDA 0x8723b0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8723b0() {
}

// 0x8723b4 — __ZN3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x8723b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8723b4() {
}

// 0x8723c8 — __ZN3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x8723c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8723c8() {
}

// 0x872478 — __ZThn132_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x872478: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_872478() {
}

// 0x87248c — __ZThn132_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x87248c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_87248c() {
}

// 0x872540 — __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x872540: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_872540() {
}

// 0x872554 — __ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x872554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_872554() {
}

// 0x872604 — __ZThn132_N3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x872604: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_872604() {
}

// 0x872618 — __ZThn132_N3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_19MegaClusterInstanceELZNS_12sMegaClusterEENS_14FactoryProductIS2_NS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x872618: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_872618() {
}

// 0x8726cc — __ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev")]
// IDA 0x8726cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8726cc() {
}

// 0x8726e0 — __ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev")]
// IDA 0x8726e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8726e0() {
}

// 0x872790 — __ZThn132_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev")]
// IDA 0x872790: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_872790() {
}

// 0x8727a4 — __ZThn132_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev")]
// IDA 0x8727a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8727a4() {
}

// 0x8727ac — __ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0x8727ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8727ac() {
}

// 0x8727b0 — __ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0x8727b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8727b0() {
}

// 0x87284c — __ZNK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0x87284c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87284c() {
}

// 0x8728d4 — __ZNK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7Creator6createEv")]
// IDA 0x8728d4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8728d4() {
}

// 0x872a18 — __ZN3RBX4Name13callDoDeclareILZNS_12sMegaClusterEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sMegaClusterEEEEvv")]
// IDA 0x872a18: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_872a18() {
}

// 0x872a1c — __ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sMegaClusterEEEERKS0_v")]
// IDA 0x872a1c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872a1c() {
}

// 0x872afc — __ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0x872afc: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872afc() {
}

// 0x872d40 — __ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEE17static_getCreatorEv")]
// IDA 0x872d40: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_872d40() {
}

// 0x8732cc — __ZThn32_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev")]
// IDA 0x8732cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8732cc() {
}

// 0x8732e0 — __ZThn36_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED1Ev")]
// IDA 0x8732e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8732e0() {
}

// 0x8732f4 — __ZThn32_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev")]
// IDA 0x8732f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8732f4() {
}

// 0x8732fc — __ZThn36_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEENS_8InstanceEED0Ev")]
// IDA 0x8732fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8732fc() {
}

// 0x873304 — __ZThn32_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x873304: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_873304() {
}

// 0x873318 — __ZThn32_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_19MegaClusterInstanceENS_12PartInstanceELZNS_12sMegaClusterEELNS_10Reflection15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x873318: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_873318() {
}

// 0x8833dc — __ZN3RBX16CellEdgeEdgePair4testEv
// type: _DWORD __fastcall(RBX::CellEdgeEdgePair *__hidden this)
#[doc(alias = "RBX::CellEdgeEdgePair::test(void)")]
#[doc(alias = "__ZN3RBX16CellEdgeEdgePair4testEv")]
// IDA 0x8833dc: 354 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8833dc() {
}

// 0x8838cc — __ZN3RBX16CellEdgeEdgePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellEdgeEdgePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX16CellEdgeEdgePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x8838cc: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8838cc() {
}

// 0x883940 — __ZN3RBX16CellEdgeEdgePair20newEdgeEdgeConnectorEv
// type: _DWORD __fastcall(RBX::CellEdgeEdgePair *__hidden this)
#[doc(alias = "RBX::CellEdgeEdgePair::newEdgeEdgeConnector(void)")]
#[doc(alias = "__ZN3RBX16CellEdgeEdgePair20newEdgeEdgeConnectorEv")]
// IDA 0x883940: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_883940() {
}

// 0x883b7c — __ZN3RBX9AllocatorINS_15PolyCellContactEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyCellContactEEC2Ev")]
// IDA 0x883b7c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_883b7c() {
}

// 0x883be0 — __ZN3RBX16CellEdgeEdgePairD1Ev
// type: void __fastcall(RBX::CellEdgeEdgePair *__hidden this)
#[doc(alias = "RBX::CellEdgeEdgePair::~CellEdgeEdgePair()")]
#[doc(alias = "__ZN3RBX16CellEdgeEdgePairD1Ev")]
// IDA 0x883be0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_883be0() {
}

// 0x883be4 — __ZN3RBX16CellFaceFacePairD1Ev
// type: void __fastcall(RBX::CellFaceFacePair *__hidden this)
#[doc(alias = "RBX::CellFaceFacePair::~CellFaceFacePair()")]
#[doc(alias = "__ZN3RBX16CellFaceFacePairD1Ev")]
// IDA 0x883be4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_883be4() {
}

// 0x883f70 — __ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EEixEm
#[doc(alias = "RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul>::operator[](unsigned long)")]
#[doc(alias = "__ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EEixEm")]
// IDA 0x883f70: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_883f70() {
}

// 0x883fd0 — __ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EE9push_backERKS2_
#[doc(alias = "RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul>::push_back(RBX::CellFaceFacePair::VertexStatus const&)")]
#[doc(alias = "__ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EE9push_backERKS2_")]
// IDA 0x883fd0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_883fd0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x884100 — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEnwEm
#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_19FaceVertexConnectorEEnwEm")]
// IDA 0x884100: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884100() {
}

// 0x884170 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEnwEm
#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEnwEm")]
// IDA 0x884170: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884170() {
}

// 0x8841e0 — __ZNK3RBX4POLY4Face12getSidePlaneEm
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this, unsigned int)
#[doc(alias = "RBX::POLY::Face::getSidePlane(unsigned long)const")]
#[doc(alias = "__ZNK3RBX4POLY4Face12getSidePlaneEm")]
// IDA 0x8841e0: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8841e0() {
}

// 0x884264 — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEnwEm
#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEnwEm")]
// IDA 0x884264: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884264() {
}

// 0x8842d4 — __ZNK3RBX16CellFaceFacePair10isFaceFaceEv
// type: _DWORD __fastcall(RBX::CellFaceFacePair *__hidden this)
#[doc(alias = "RBX::CellFaceFacePair::isFaceFace(void)const")]
#[doc(alias = "__ZNK3RBX16CellFaceFacePair10isFaceFaceEv")]
// IDA 0x8842d4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8842d4() {
}

// 0x8842d8 — __ZN3RBX16CellFaceFacePairD0Ev
// type: void __fastcall(RBX::CellFaceFacePair *__hidden this)
#[doc(alias = "RBX::CellFaceFacePair::~CellFaceFacePair()")]
#[doc(alias = "__ZN3RBX16CellFaceFacePairD0Ev")]
// IDA 0x8842d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8842d8() {
}

// 0x8842dc — __ZNK3RBX16CellEdgeEdgePair10isFaceFaceEv
// type: _DWORD __fastcall(RBX::CellEdgeEdgePair *__hidden this)
#[doc(alias = "RBX::CellEdgeEdgePair::isFaceFace(void)const")]
#[doc(alias = "__ZNK3RBX16CellEdgeEdgePair10isFaceFaceEv")]
// IDA 0x8842dc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8842dc() {
}

// 0x8842e0 — __ZN3RBX12PolyCellPair11pairIsValidEv
// type: _DWORD __fastcall(RBX::PolyCellPair *__hidden this)
#[doc(alias = "RBX::PolyCellPair::pairIsValid(void)")]
#[doc(alias = "__ZN3RBX12PolyCellPair11pairIsValidEv")]
// IDA 0x8842e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8842e0() {
}

// 0x8842e4 — __ZN3RBX16CellEdgeEdgePairD0Ev
// type: void __fastcall(RBX::CellEdgeEdgePair *__hidden this)
#[doc(alias = "RBX::CellEdgeEdgePair::~CellEdgeEdgePair()")]
#[doc(alias = "__ZN3RBX16CellEdgeEdgePairD0Ev")]
// IDA 0x8842e4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8842e4() {
}

// 0x8842e8 — __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// was: __ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::EdgeEdgeConnector,328u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17EdgeEdgeConnectorELj328ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x8842e8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8842e8() {
}

// 0x884320 — __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// was: __ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::FaceEdgeConnector,368u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17FaceEdgeConnectorELj368ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x884320: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884320() {
}

// 0x884358 — __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// was: __ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::FaceVertexConnector,304u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX19FaceVertexConnectorELj304ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x884358: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884358() {
}
