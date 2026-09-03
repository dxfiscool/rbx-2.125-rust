//! rendering shard 470 — 120 stubs 0x724064..0x72c890 EA-sorted asc global gap filler not yet in rbx_rendering (global gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (50730->50850 distinct, fallback after 0x724064).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x724064 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev")]
// IDA 0x724064: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_724064() {
}

// 0x7240c8 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_18BallBlockConnectorEE13releaseMemoryEv")]
// IDA 0x7240c8: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7240c8() {
}

// 0x7240e4 — __ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x7240e4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7240e4() {
}

// 0x724114 — __ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev")]
// IDA 0x724114: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_724114() {
}

// 0x724178 — __ZN3RBX9AllocatorINS_17BallBallConnectorEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BallBallConnectorEE13releaseMemoryEv")]
// IDA 0x724178: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_724178() {
}

// 0x724194 — __ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x724194: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_724194() {
}

// 0x7241c4 — __ZN3RBX10IPipelinedD2Ev
// type: void __fastcall(RBX::IPipelined *__hidden this)
#[doc(alias = "RBX::IPipelined::~IPipelined()")]
#[doc(alias = "__ZN3RBX10IPipelinedD2Ev")]
// IDA 0x7241c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7241c4() {
}

// 0x724234 — __ZN3RBX4EdgeD1Ev
// type: void __fastcall(RBX::Edge *__hidden this)
#[doc(alias = "RBX::Edge::~Edge()")]
#[doc(alias = "__ZN3RBX4EdgeD1Ev")]
// IDA 0x724234: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_724234() {
}

// 0x724238 — __ZN3RBX4EdgeD0Ev
// type: void __fastcall(RBX::Edge *__hidden this)
#[doc(alias = "RBX::Edge::~Edge()")]
#[doc(alias = "__ZN3RBX4EdgeD0Ev")]
// IDA 0x724238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_724238() {
}

// 0x7242d8 — __ZN3RBX10IPipelinedD1Ev
// type: void __fastcall(RBX::IPipelined *__hidden this)
#[doc(alias = "RBX::IPipelined::~IPipelined()")]
#[doc(alias = "__ZN3RBX10IPipelinedD1Ev")]
// IDA 0x7242d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7242d8() {
}

// 0x7242dc — __ZN3RBX10IPipelinedD0Ev
// type: void __fastcall(RBX::IPipelined *__hidden this)
#[doc(alias = "RBX::IPipelined::~IPipelined()")]
#[doc(alias = "__ZN3RBX10IPipelinedD0Ev")]
// IDA 0x7242dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7242dc() {
}

// 0x72437c — __GLOBAL__I_a_310
#[doc(alias = "global constructor keyed to_a_310")]
#[doc(alias = "__GLOBAL__I_a_310")]
// IDA 0x72437c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_72437c() {
}

// 0x724650 — __ZN3RBX14ContactManagerC1EPNS_5WorldE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::World *)
#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
#[doc(alias = "__ZN3RBX14ContactManagerC1EPNS_5WorldE")]
// IDA 0x724650: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_724650() {
}

// 0x724654 — __ZN3RBX14ContactManagerC2EPNS_5WorldE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::World *)
#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
#[doc(alias = "__ZN3RBX14ContactManagerC2EPNS_5WorldE")]
// IDA 0x724654: 152 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_724654() {
}

// 0x7247ec — __ZN3RBX14ContactManagerD1Ev
// type: void __fastcall(RBX::ContactManager *__hidden this)
#[doc(alias = "RBX::ContactManager::~ContactManager()")]
#[doc(alias = "__ZN3RBX14ContactManagerD1Ev")]
// IDA 0x7247ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7247ec() {
}

// 0x7247f0 — __ZN3RBX14ContactManagerD2Ev
// type: void __fastcall(RBX::ContactManager *__hidden this)
#[doc(alias = "RBX::ContactManager::~ContactManager()")]
#[doc(alias = "__ZN3RBX14ContactManagerD2Ev")]
// IDA 0x7247f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7247f0() {
}

// 0x724920 — __ZN3RBX14ContactManager9fastClearEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
#[doc(alias = "RBX::ContactManager::fastClear(void)")]
#[doc(alias = "__ZN3RBX14ContactManager9fastClearEv")]
// IDA 0x724920: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_724920() {
}

// 0x724928 — __ZN3RBX14ContactManager7doStatsEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
#[doc(alias = "RBX::ContactManager::doStats(void)")]
#[doc(alias = "__ZN3RBX14ContactManager7doStatsEv")]
// IDA 0x724928: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_724928() {
}

// 0x72492c — __ZN3RBX14ContactManager24intersectingMySimulationEPNS_9PrimitiveENS_13SystemAddressEf
// type: int __fastcall(int, RBX::Primitive *this, int, int, float)
#[doc(alias = "RBX::ContactManager::intersectingMySimulation(RBX::Primitive *,RBX::SystemAddress,float)")]
#[doc(alias = "__ZN3RBX14ContactManager24intersectingMySimulationEPNS_9PrimitiveENS_13SystemAddressEf")]
// IDA 0x72492c: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72492c() {
}

// 0x7249bc — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveERKSt3setIS2_St4lessIS2_ESaIS2_EEf
// type: int __fastcall(int, RBX::Primitive *this, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&,float)")]
#[doc(alias = "__ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveERKSt3setIS2_St4lessIS2_ESaIS2_EEf")]
// IDA 0x7249bc: 223 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7249bc() {
}

// 0x724be4 — __ZN3RBX14ContactManager13createContactEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::createContact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager13createContactEPNS_9PrimitiveES2_")]
// IDA 0x724be4: 359 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_724be4() {
}

// 0x7250a4 — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveEf
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, float)
#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,float)")]
#[doc(alias = "__ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveEf")]
// IDA 0x7250a4: 78 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7250a4() {
}

// 0x725f1c — __ZNK3RBX14ContactManager21terrainCellsInRegion3ENS_7Region3E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::ContactManager::terrainCellsInRegion3(RBX::Region3)const")]
#[doc(alias = "__ZNK3RBX14ContactManager21terrainCellsInRegion3ENS_7Region3E")]
// IDA 0x725f1c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_725f1c() {
}

// 0x7262e0 — __ZN3RBX14ContactManager9onNewPairEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::onNewPair(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager9onNewPairEPNS_9PrimitiveES2_")]
// IDA 0x7262e0: 55 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7262e0() {
}

// 0x726370 — __ZN3RBX14ContactManager11releasePairEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::releasePair(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager11releasePairEPNS_9PrimitiveES2_")]
// IDA 0x726370: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_726370() {
}

// 0x72641c — __ZN3RBX14ContactManager16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::onPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager16onPrimitiveAddedEPNS_9PrimitiveE")]
// IDA 0x72641c: 79 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72641c() {
}

// 0x7264f8 — __ZN3RBX14ContactManager22getMegaClusterInstanceINS_5Voxel4GridEEEPT_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Voxel::Grid * RBX::ContactManager::getMegaClusterInstance<RBX::Voxel::Grid>(void)")]
#[doc(alias = "__ZN3RBX14ContactManager22getMegaClusterInstanceINS_5Voxel4GridEEEPT_v")]
// IDA 0x7264f8: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7264f8() {
}

// 0x726510 — __ZN3RBX14ContactManager22getMegaClusterInstanceINS_19MegaClusterInstanceEEEPT_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::MegaClusterInstance * RBX::ContactManager::getMegaClusterInstance<RBX::MegaClusterInstance>(void)")]
#[doc(alias = "__ZN3RBX14ContactManager22getMegaClusterInstanceINS_19MegaClusterInstanceEEEPT_v")]
// IDA 0x726510: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_726510() {
}

// 0x726524 — __ZN3RBX14ContactManager18onPrimitiveRemovedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::onPrimitiveRemoved(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager18onPrimitiveRemovedEPNS_9PrimitiveE")]
// IDA 0x726524: 80 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_726524() {
}

// 0x726604 — __ZN3RBX14ContactManager25onPrimitiveExtentsChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::onPrimitiveExtentsChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager25onPrimitiveExtentsChangedEPNS_9PrimitiveE")]
// IDA 0x726604: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_726604() {
}

// 0x72660c — __ZN3RBX14ContactManager26onPrimitiveGeometryChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::onPrimitiveGeometryChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager26onPrimitiveGeometryChangedEPNS_9PrimitiveE")]
// IDA 0x72660c: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72660c() {
}

// 0x72676c — __ZN3RBX14ContactManager23checkMegaClusterContactEPNS_9PrimitiveEbbb
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *, bool, bool, bool)
#[doc(alias = "RBX::ContactManager::checkMegaClusterContact(RBX::Primitive *,bool,bool,bool)")]
#[doc(alias = "__ZN3RBX14ContactManager23checkMegaClusterContactEPNS_9PrimitiveEbbb")]
// IDA 0x72676c: 231 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72676c() {
}

// 0x726cf0 — __ZN3RBX14ContactManager34primitiveIsExcludedFromSpatialHashEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::primitiveIsExcludedFromSpatialHash(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager34primitiveIsExcludedFromSpatialHashEPNS_9PrimitiveE")]
// IDA 0x726cf0: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_726cf0() {
}

// 0x726d08 — __ZN3RBX14ContactManager33checkMegaClusterBigTerrainContactEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::ContactManager::checkMegaClusterBigTerrainContact(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX14ContactManager33checkMegaClusterBigTerrainContactEPNS_9PrimitiveE")]
// IDA 0x726d08: 325 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_726d08() {
}

// 0x727054 — __ZN3RBX14ContactManager18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ContactManager::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
#[doc(alias = "__ZN3RBX14ContactManager18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// IDA 0x727054: 207 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727054() {
}

// 0x727420 — __ZN3RBX14ContactManager19startLoadingTerrainEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
#[doc(alias = "RBX::ContactManager::startLoadingTerrain(void)")]
#[doc(alias = "__ZN3RBX14ContactManager19startLoadingTerrainEv")]
// IDA 0x727420: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727420() {
}

// 0x727438 — __ZN3RBX14ContactManager18doneLoadingTerrainEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
#[doc(alias = "RBX::ContactManager::doneLoadingTerrain(void)")]
#[doc(alias = "__ZN3RBX14ContactManager18doneLoadingTerrainEv")]
// IDA 0x727438: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727438() {
}

// 0x727578 — __ZN3RBX14ContactManager27applyDeferredTerrainChangesEv
// type: _DWORD __fastcall(RBX::ContactManager *__hidden this)
#[doc(alias = "RBX::ContactManager::applyDeferredTerrainChanges(void)")]
#[doc(alias = "__ZN3RBX14ContactManager27applyDeferredTerrainChangesEv")]
// IDA 0x727578: 246 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727578() {
}

// 0x727838 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::fastClear(void)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv")]
// IDA 0x727838: 218 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727838() {
}

// 0x727a94 — __ZNK3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE5beginEv
#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::begin(void)const")]
#[doc(alias = "__ZNK3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE5beginEv")]
// IDA 0x727a94: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727a94() {
}

// 0x727acc — __ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE14const_iteratorppEv
#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::const_iterator::operator++(void)")]
#[doc(alias = "__ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE14const_iteratorppEv")]
// IDA 0x727acc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727acc() {
}

// 0x727b44 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26getPrimitivesTouchingGridsERKNS_7ExtentsERKN5boost9unordered13unordered_setIPKS1_NS8_4hashISC_EESt8equal_toISC_ESaISC_EEEmRNSA_IPS1_NSD_ISL_EESF_ISL_ESaISL_EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesTouchingGrids(RBX::Extents const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,unsigned long,boost::unordered::unordered_set<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::equal_to<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26getPrimitivesTouchingGridsERKNS_7ExtentsERKN5boost9unordered13unordered_setIPKS1_NS8_4hashISC_EESt8equal_toISC_ESaISC_EEEmRNSA_IPS1_NSD_ISL_EESF_ISL_ESaISL_EEE")]
// IDA 0x727b44: 277 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727b44() {
}

// 0x727f74 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getNextGrid(RBX::Vector3int32 &,RBX::RbxRay const&,float)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf")]
// IDA 0x727f74: 259 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_727f74() {
}

// 0x728358 — __ZNK3RBX7Extents17overlapsOrTouchesERKS0_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const RBX::Extents *)
#[doc(alias = "RBX::Extents::overlapsOrTouches(RBX::Extents const&)const")]
#[doc(alias = "__ZNK3RBX7Extents17overlapsOrTouchesERKS0_")]
// IDA 0x728358: 38 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728358() {
}

// 0x7283d4 — __ZNK3RBX14ContactManager37anyExtentsOverlapsOrTouchesPrimitivesINS_12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS4_EESaIS4_EEEEEbRKNS_7ExtentsERKT_
#[doc(alias = "bool RBX::ContactManager::anyExtentsOverlapsOrTouchesPrimitives<RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>>(RBX::Extents const&,RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&)const")]
#[doc(alias = "__ZNK3RBX14ContactManager37anyExtentsOverlapsOrTouchesPrimitivesINS_12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS4_EESaIS4_EEEEEbRKNS_7ExtentsERKT_")]
// IDA 0x7283d4: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7283d4() {
}

// 0x728438 — __ZN3RBX9AllocatorINS_15BallBallContactEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallBallContactEEnwEm")]
// IDA 0x728438: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728438() {
}

// 0x7284a8 — __ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv")]
// IDA 0x7284a8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7284a8() {
}

// 0x7284e4 — __ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm")]
// IDA 0x7284e4: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7284e4() {
}

// 0x728554 — __ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv")]
// IDA 0x728554: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728554() {
}

// 0x728590 — __ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm")]
// IDA 0x728590: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728590() {
}

// 0x728600 — __ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm")]
// IDA 0x728600: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728600() {
}

// 0x728670 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm
#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm")]
// IDA 0x728670: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728670() {
}

// 0x7286e0 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv")]
// IDA 0x7286e0: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7286e0() {
}

// 0x72871c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b
// type: int __fastcall(int, RBX::Primitive *this)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(RBX::Primitive*,bool)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b")]
// IDA 0x72871c: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72871c() {
}

// 0x7287b0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_
// type: int __fastcall(int, RBX::Primitive *this)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_")]
// IDA 0x7287b0: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7287b0() {
}

// 0x72895c — __ZN3RBX9AllocatorINS_15BallCellContactEEnwEm
#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallCellContactEEnwEm")]
// IDA 0x72895c: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72895c() {
}

// 0x7289cc — __ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv")]
// IDA 0x7289cc: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7289cc() {
}

// 0x728a08 — __ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm
#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm")]
// IDA 0x728a08: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728a08() {
}

// 0x728a78 — __ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv")]
// IDA 0x728a78: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728a78() {
}

// 0x728ab4 — __ZN3RBX7Extents14clampToOverlapERKS0_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const RBX::Extents *)
#[doc(alias = "RBX::Extents::clampToOverlap(RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX7Extents14clampToOverlapERKS0_")]
// IDA 0x728ab4: 84 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_728ab4() {
}

// 0x729bd8 — __ZSt8for_eachIN5boost9unordered15iterator_detail10c_iteratorINS1_6detail8ptr_nodeIPN3RBX9PrimitiveEEEPKS9_EENS0_3_bi6bind_tIvNS0_4_mfi3mf4IvNS6_14ContactManagerES8_bbbEENSD_5list5INSD_5valueIPSH_EENS0_3argILi1EEENSK_IbEESP_SP_EEEEET0_T_ST_SS_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>> std::for_each<boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>)")]
#[doc(alias = "__ZSt8for_eachIN5boost9unordered15iterator_detail10c_iteratorINS1_6detail8ptr_nodeIPN3RBX9PrimitiveEEEPKS9_EENS0_3_bi6bind_tIvNS0_4_mfi3mf4IvNS6_14ContactManagerES8_bbbEENSD_5list5INSD_5valueIPSH_EENS0_3argILi1EEENSK_IbEESP_SP_EEEEET0_T_ST_SS_")]
// IDA 0x729bd8: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_729bd8() {
}

// 0x729c3c — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::clear(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE5clearEv")]
// IDA 0x729c3c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_729c3c() {
}

// 0x729c6c — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12delete_nodesEPNS1_10ptr_bucketESE_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12delete_nodesEPNS1_10ptr_bucketESE_")]
// IDA 0x729c6c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_729c6c() {
}

// 0x729d8c — __ZN5boost3_bi5list5INS0_5valueIPN3RBX14ContactManagerEEENS_3argILi1EEENS2_IbEES9_S9_EclINS_4_mfi3mf4IvS4_PNS3_9PrimitiveEbbbEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::ContactManager *>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list1<RBX::Primitive * const&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool> &,boost::_bi::list1<RBX::Primitive * const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX14ContactManagerEEENS_3argILi1EEENS2_IbEES9_S9_EclINS_4_mfi3mf4IvS4_PNS3_9PrimitiveEbbbEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i")]
// IDA 0x729d8c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_729d8c() {
}

// 0x729dd8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id>>(RBX::SpatialRegion::Id const&,boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_")]
// IDA 0x729dd8: 152 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_729dd8() {
}

// 0x729f7c — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE18reserve_for_insertEm")]
// IDA 0x729f7c: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_729f7c() {
}

// 0x729fd0 — __ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14create_bucketsEm")]
// IDA 0x729fd0: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_729fd0() {
}

// 0x72a0f8 — __ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE20min_buckets_for_sizeEm")]
// IDA 0x72a0f8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a0f8() {
}

// 0x72a188 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE11rehash_implEm")]
// IDA 0x72a188: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a188() {
}

// 0x72a1b4 — __ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISB_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISB_EEPNS1_10ptr_bucketE")]
// IDA 0x72a1b4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a1b4() {
}

// 0x72a20c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX13SpatialRegion2IdEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIN3RBX13SpatialRegion2IdEEEEE9constructEv")]
// IDA 0x72a20c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a20c() {
}

// 0x72a244 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14find_node_implIS6_SA_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::find_node_impl<RBX::SpatialRegion::Id,std::equal_to<RBX::SpatialRegion::Id>>(unsigned long,RBX::SpatialRegion::Id const&,std::equal_to<RBX::SpatialRegion::Id> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3setISaIN3RBX13SpatialRegion2IdEES6_NS6_27boost_compatible_hash_valueESt8equal_toIS6_EEEE14find_node_implIS6_SA_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")]
// IDA 0x72a244: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a244() {
}

// 0x72a2c0 — __ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0x72a2c0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a2c0() {
}

// 0x72a30c — __ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15PolyCellContactELj232ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x72a30c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a30c() {
}

// 0x72a344 — __ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0x72a344: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a344() {
}

// 0x72a390 — __ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15BallCellContactELj228ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x72a390: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a390() {
}

// 0x72a668 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::computeLevel(RBX::Primitive const*,RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE")]
// IDA 0x72a668: 59 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a668() {
}

// 0x72a728 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_")]
// IDA 0x72a728: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a728() {
}

// 0x72a844 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(RBX::Primitive*,bool)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b")]
// IDA 0x72a844: 110 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a844() {
}

// 0x72a990 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(RBX::Primitive*,RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE")]
// IDA 0x72a990: 171 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72a990() {
}

// 0x72ab58 — __ZN3RBX12ExtentsInt32C1Ev
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this)
#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
#[doc(alias = "__ZN3RBX12ExtentsInt32C1Ev")]
// IDA 0x72ab58: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72ab58() {
}

// 0x72ac08 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b
// type: int __fastcall(int, int, int, RBX::ExtentsInt32 *this, RBX::ExtentsInt32 *, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::changeMinMax(RBX::Primitive*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b")]
// IDA 0x72ac08: 123 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72ac08() {
}

// 0x72ad40 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::checkAndReleaseContacts(RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_")]
// IDA 0x72ad40: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72ad40() {
}

// 0x72adc4 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::oldExtentsOverlap(RBX::Primitive*,RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_")]
// IDA 0x72adc4: 101 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72adc4() {
}

// 0x72aef8 — __ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E
#[doc(alias = "RBX::ExtentsInt32::contains(RBX::Vector3int32 const&)const")]
#[doc(alias = "__ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E")]
// IDA 0x72aef8: 29 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72aef8() {
}

// 0x72af38 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addNode(RBX::Primitive*,RBX::Vector3int32 const&,bool)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb")]
// IDA 0x72af38: 413 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72af38() {
}

// 0x72b3c0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findNode(RBX::Primitive*,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E")]
// IDA 0x72b3c0: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b3c0() {
}

// 0x72b494 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE
// type: int __fastcall(int, RBX::NodeBase *this)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeNodeFromHash(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE")]
// IDA 0x72b494: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b494() {
}

// 0x72b4c4 — __ZN3RBX8NodeBase8getLevelEv
// type: _DWORD __fastcall(RBX::NodeBase *__hidden this)
#[doc(alias = "RBX::NodeBase::getLevel(void)")]
#[doc(alias = "__ZN3RBX8NodeBase8getLevelEv")]
// IDA 0x72b4c4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b4c4() {
}

// 0x72b528 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE
// type: int __fastcall(int, RBX::NodeBase *this)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE")]
// IDA 0x72b528: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b528() {
}

// 0x72b5b8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
// type: int __fastcall(int, RBX::SpatialHashStatic *this, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E")]
// IDA 0x72b5b8: 130 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b5b8() {
}

// 0x72b730 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv")]
// IDA 0x72b730: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b730() {
}

// 0x72b770 — __ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0x72b770: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b770() {
}

// 0x72b7c0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE")]
// IDA 0x72b7c0: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b7c0() {
}

// 0x72b8e0 — __ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0x72b8e0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72b8e0() {
}

// 0x72b92c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::~TreeNode()")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev")]
// IDA 0x72b92c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_72b92c() {
}

// 0x72ba94 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, void *, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E")]
// IDA 0x72ba94: 172 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72ba94() {
}

// 0x72bc74 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,RBX::Primitive*,RBX::Vector3int32 const&,int)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei")]
// IDA 0x72bc74: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72bc74() {
}

// 0x72bcf8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E")]
// IDA 0x72bcf8: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72bcf8() {
}

// 0x72be14 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_
// type: int __fastcall(int, int, RBX::Primitive *this)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addContactFromChildren(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *,RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_")]
// IDA 0x72be14: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72be14() {
}

// 0x72c004 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm")]
// IDA 0x72c004: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c004() {
}

// 0x72c074 — __ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE9constructEv
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::construct(void)")]
#[doc(alias = "__ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE9constructEv")]
// IDA 0x72c074: 96 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c074() {
}

// 0x72c17c — __ZN5boost4poolIN3RBX16roblox_allocatorEE26ordered_malloc_need_resizeEv
#[doc(alias = "boost::pool<RBX::roblox_allocator>::ordered_malloc_need_resize(void)")]
#[doc(alias = "__ZN5boost4poolIN3RBX16roblox_allocatorEE26ordered_malloc_need_resizeEv")]
// IDA 0x72c17c: 116 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c17c() {
}

// 0x72c2b0 — __ZN5boost25simple_segregated_storageImE17add_ordered_blockEPvmm
#[doc(alias = "boost::simple_segregated_storage<unsigned long>::add_ordered_block(void *,unsigned long,unsigned long)")]
#[doc(alias = "__ZN5boost25simple_segregated_storageImE17add_ordered_blockEPvmm")]
// IDA 0x72c2b0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c2b0() {
}

// 0x72c2f0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::TreeNode(void)")]
#[doc(alias = "__ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev")]
// IDA 0x72c2f0: 87 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c2f0() {
}

// 0x72c3e0 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev")]
// IDA 0x72c3e0: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c3e0() {
}

// 0x72c448 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEE13releaseMemoryEv")]
// IDA 0x72c448: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c448() {
}

// 0x72c468 — __ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x72c468: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c468() {
}

// 0x72c49c — __ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x72c49c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c49c() {
}

// 0x72c4d4 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm")]
// IDA 0x72c4d4: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c4d4() {
}

// 0x72c544 — __ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEE9constructIiiNS1_12Vector3int32EEEPS7_RT_RT0_RKT1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode* boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::construct<int,int,RBX::Vector3int32>(int &,int &,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN5boost11object_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEE9constructIiiNS1_12Vector3int32EEEPS7_RT_RT0_RKT1_")]
// IDA 0x72c544: 123 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c544() {
}

// 0x72c68c — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev")]
// IDA 0x72c68c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c68c() {
}

// 0x72c6f0 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEE13releaseMemoryEv")]
// IDA 0x72c6f0: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c6f0() {
}

// 0x72c70c — __ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x72c70c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c70c() {
}

// 0x72c73c — __ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX11SpatialHashINS1_9PrimitiveENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x72c73c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c73c() {
}

// 0x72c778 — __ZN3RBX12ExtentsInt325emptyEv
// type: _DWORD __fastcall(RBX::ExtentsInt32 *__hidden this)
#[doc(alias = "RBX::ExtentsInt32::empty(void)")]
#[doc(alias = "__ZN3RBX12ExtentsInt325emptyEv")]
// IDA 0x72c778: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c778() {
}

// 0x72c7c0 — __ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0x72c7c0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c7c0() {
}

// 0x72c80c — __ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x72c80c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c80c() {
}

// 0x72c844 — __ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0x72c844: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c844() {
}

// 0x72c890 — __ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x72c890: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_72c890() {
}
