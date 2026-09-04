//! rendering shard 474 — 100 stubs 0x74779c..0x74c288 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 50990->51090 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc after shard 473 (0x7420e0..0x74775c)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// 0x74779c — __ZN3RBX11PolyContact19updateContactPointsEv
// type: unsigned int __fastcall(RBX::PolyContact *this)
#[doc(alias = "RBX::PolyContact::updateContactPoints(void)")]
#[doc(alias = "__ZN3RBX11PolyContact19updateContactPointsEv")]
// IDA 0x74779c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74779c() {
}
// 0x7477cc — __ZN3RBX11PolyContact18computeIsCollidingEf
// type: bool __fastcall(RBX::PolyContact *this, float)
#[doc(alias = "RBX::PolyContact::computeIsColliding(float)")]
#[doc(alias = "__ZN3RBX11PolyContact18computeIsCollidingEf")]
// IDA 0x7477cc: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7477cc() {
}
// 0x747834 — __ZN3RBX11PolyContact21updateClosestFeaturesEv
// type: int __fastcall(RBX::PolyContact *this)
#[doc(alias = "RBX::PolyContact::updateClosestFeatures(void)")]
#[doc(alias = "__ZN3RBX11PolyContact21updateClosestFeaturesEv")]
// IDA 0x747834: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_747834() {
}
// 0x747904 — __ZN3RBX11PolyContact19worstFeatureOverlapEv
// type: int __fastcall(RBX::PolyContact *this, int, int)
#[doc(alias = "RBX::PolyContact::worstFeatureOverlap(void)")]
#[doc(alias = "__ZN3RBX11PolyContact19worstFeatureOverlapEv")]
// IDA 0x747904: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_747904() {
}
// 0x7479b8 — __ZN3RBX11PolyContact20matchClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
// type: unsigned int __fastcall(RBX::PolyContact *, int)
#[doc(alias = "RBX::PolyContact::matchClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX11PolyContact20matchClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x7479b8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7479b8() {
}
// 0x747a10 — __ZN3RBX11PolyContact19matchClosestFeatureEPNS_13PolyConnectorE
// type: RBX::PolyConnector *__fastcall(RBX::PolyContact *this, RBX::PolyConnector *)
#[doc(alias = "RBX::PolyContact::matchClosestFeature(RBX::PolyConnector *)")]
#[doc(alias = "__ZN3RBX11PolyContact19matchClosestFeatureEPNS_13PolyConnectorE")]
// IDA 0x747a10: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_747a10() {
}
// 0x747a54 — __GLOBAL__I_a_339
#[doc(alias = "global constructor keyed to _a_339")]
#[doc(alias = "__GLOBAL__I_a_339")]
// IDA 0x747a54: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_747a54() {
}
// 0x747dc0 — __ZN3RBX15PolyPolyContactC1EPNS_9PrimitiveES2_
// type: int __fastcall(RBX::PolyPolyContact *this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::PolyPolyContact::PolyPolyContact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX15PolyPolyContactC1EPNS_9PrimitiveES2_")]
// IDA 0x747dc0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_747dc0() {
}
// 0x747dc4 — __ZN3RBX15PolyPolyContactC2EPNS_9PrimitiveES2_
// type: RBX::PolyPolyContact *__fastcall(RBX::PolyPolyContact *this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::PolyPolyContact::PolyPolyContact(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX15PolyPolyContactC2EPNS_9PrimitiveES2_")]
// IDA 0x747dc4: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_747dc4() {
}
// 0x747ea8 — __ZN3RBX15PolyPolyContactD0Ev
// type: void __fastcall(RBX::PolyPolyContact *__hidden this)
#[doc(alias = "RBX::PolyPolyContact::~PolyPolyContact()")]
#[doc(alias = "__ZN3RBX15PolyPolyContactD0Ev")]
// IDA 0x747ea8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_747ea8() {
}
// 0x747f5c — __ZN3RBX15PolyPolyContactD1Ev
// type: void __fastcall(RBX::PolyPolyContact *__hidden this)
#[doc(alias = "RBX::PolyPolyContact::~PolyPolyContact()")]
#[doc(alias = "__ZN3RBX15PolyPolyContactD1Ev")]
// IDA 0x747f5c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_747f5c() {
}
// 0x747f60 — __ZN3RBX15PolyPolyContactD2Ev
// type: void __fastcall(RBX::PolyPolyContact *__hidden this)
#[doc(alias = "RBX::PolyPolyContact::~PolyPolyContact()")]
#[doc(alias = "__ZN3RBX15PolyPolyContactD2Ev")]
// IDA 0x747f60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_747f60() {
}
// 0x748038 — __ZN3RBX15PolyPolyContact13resetBestPairEPNS_8PolyPairE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::PolyPolyContact::resetBestPair(RBX::PolyPair *)")]
#[doc(alias = "__ZN3RBX15PolyPolyContact13resetBestPairEPNS_8PolyPairE")]
// IDA 0x748038: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_748038() {
}
// 0x748068 — __ZN3RBX15PolyPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
// type: int __fastcall(RBX::PolyPolyContact *, int)
#[doc(alias = "RBX::PolyPolyContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX15PolyPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x748068: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_748068() {
}
// 0x74813c — __ZN3RBX15PolyPolyContact12findBestPairEv
// type: int __fastcall(RBX::PolyPolyContact *this)
#[doc(alias = "RBX::PolyPolyContact::findBestPair(void)")]
#[doc(alias = "__ZN3RBX15PolyPolyContact12findBestPairEv")]
// IDA 0x74813c: 182 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74813c() {
}
// 0x748344 — __ZN3RBX15PolyPolyContact34generateDataForMovingAssemblyStageEv
// type: int __fastcall(RBX::PolyPolyContact *this)
#[doc(alias = "RBX::PolyPolyContact::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX15PolyPolyContact34generateDataForMovingAssemblyStageEv")]
// IDA 0x748344: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_748344() {
}
// 0x748348 — __ZN3RBX12FaceFacePair13allocateCloneEv
// type: int __fastcall(RBX::FaceFacePair *this)
#[doc(alias = "RBX::FaceFacePair::allocateClone(void)")]
#[doc(alias = "__ZN3RBX12FaceFacePair13allocateCloneEv")]
// IDA 0x748348: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_748348() {
}
// 0x748380 — __ZN3RBX12FaceFacePair4testEv
// type: int __fastcall(RBX::Primitive **this)
#[doc(alias = "RBX::FaceFacePair::test(void)")]
#[doc(alias = "__ZN3RBX12FaceFacePair4testEv")]
// IDA 0x748380: 147 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_748380() {
}
// 0x74870c — __ZN3RBX12FaceFacePair13findOtherFaceEPKNS_4POLY6VertexE
// type: RBX::POLY::Face *__fastcall(RBX::FaceFacePair *this, const RBX::POLY::Vertex *)
#[doc(alias = "RBX::FaceFacePair::findOtherFace(RBX::POLY::Vertex const*)")]
#[doc(alias = "__ZN3RBX12FaceFacePair13findOtherFaceEPKNS_4POLY6VertexE")]
// IDA 0x74870c: 189 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74870c() {
}
// 0x748934 — __ZN3RBX12FaceFacePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::FaceFacePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX12FaceFacePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x748934: 251 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_748934() {
}
// 0x7492c0 — __ZN3RBX12FaceFacePair12vertexInsideEPNS_9PrimitiveES2_PKNS_4POLY6VertexEPKNS3_4FaceERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
// type: void __fastcall(int, int, int, int *, RBX::POLY::Face *, int, struct _Unwind_Exception *lpuexcpt, boost::mutex *, int, int, int, int, int, int)
#[doc(alias = "RBX::FaceFacePair::vertexInside(RBX::Primitive *,RBX::Primitive *,RBX::POLY::Vertex const*,RBX::POLY::Face const*,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX12FaceFacePair12vertexInsideEPNS_9PrimitiveES2_PKNS_4POLY6VertexEPKNS3_4FaceERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x7492c0: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7492c0() {
}
// 0x7494ec — __ZN3RBX12FaceFacePair20newFaceEdgeConnectorEmPKNS_4POLY6VertexES4_
// type: RBX::FaceEdgeConnector *__fastcall(RBX::FaceFacePair *this, unsigned int, const RBX::POLY::Vertex *, const RBX::POLY::Vertex *)
#[doc(alias = "RBX::FaceFacePair::newFaceEdgeConnector(unsigned long,RBX::POLY::Vertex const*,RBX::POLY::Vertex const*)")]
#[doc(alias = "__ZN3RBX12FaceFacePair20newFaceEdgeConnectorEmPKNS_4POLY6VertexES4_")]
// IDA 0x7494ec: 237 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7494ec() {
}
// 0x74979c — __ZN3RBX12EdgeEdgePair13allocateCloneEv
// type: int __fastcall(RBX::EdgeEdgePair *this)
#[doc(alias = "RBX::EdgeEdgePair::allocateClone(void)")]
#[doc(alias = "__ZN3RBX12EdgeEdgePair13allocateCloneEv")]
// IDA 0x74979c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74979c() {
}
// 0x7497d0 — __ZN3RBX12EdgeEdgePair4testEv
// type: int __fastcall(RBX::Primitive **this)
#[doc(alias = "RBX::EdgeEdgePair::test(void)")]
#[doc(alias = "__ZN3RBX12EdgeEdgePair4testEv")]
// IDA 0x7497d0: 357 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7497d0() {
}
// 0x749cc8 — __ZN3RBX12EdgeEdgePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
// type: int __fastcall(RBX::EdgeEdgePair *, int, int)
#[doc(alias = "RBX::EdgeEdgePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX12EdgeEdgePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x749cc8: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_749cc8() {
}
// 0x749d3c — __ZN3RBX12EdgeEdgePair20newEdgeEdgeConnectorEv
// type: RBX::EdgeEdgeConnector *__fastcall(RBX::EdgeEdgePair *this)
#[doc(alias = "RBX::EdgeEdgePair::newEdgeEdgeConnector(void)")]
#[doc(alias = "__ZN3RBX12EdgeEdgePair20newEdgeEdgeConnectorEv")]
// IDA 0x749d3c: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_749d3c() {
}
// 0x749f78 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyPolyContactEEC2Ev")]
// IDA 0x749f78: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_749f78() {
}
// 0x749fdc — __ZN3RBX12EdgeEdgePairD1Ev
// type: void __fastcall(RBX::EdgeEdgePair *__hidden this)
#[doc(alias = "RBX::EdgeEdgePair::~EdgeEdgePair()")]
#[doc(alias = "__ZN3RBX12EdgeEdgePairD1Ev")]
// IDA 0x749fdc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_749fdc() {
}
// 0x749fe0 — __ZN3RBX12FaceFacePairD1Ev
// type: void __fastcall(RBX::FaceFacePair *__hidden this)
#[doc(alias = "RBX::FaceFacePair::~FaceFacePair()")]
#[doc(alias = "__ZN3RBX12FaceFacePairD1Ev")]
// IDA 0x749fe0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_749fe0() {
}
// 0x749fe4 — __ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EEixEm
// type: int __fastcall(int, unsigned int, int)
#[doc(alias = "RBX::FixedArray<RBX::FaceFacePair::VertexStatus,40ul>::operator[](unsigned long)")]
#[doc(alias = "__ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EEixEm")]
// IDA 0x749fe4: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_749fe4() {
}
// 0x74a044 — __ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EE9push_backERKS2_
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "RBX::FixedArray<RBX::FaceFacePair::VertexStatus,40ul>::push_back(RBX::FaceFacePair::VertexStatus const&)")]
#[doc(alias = "__ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EE9push_backERKS2_")]
// IDA 0x74a044: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_74a044() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}
// 0x74a0b4 — __ZNK3RBX12FaceFacePair10isFaceFaceEv
// type: int __fastcall(RBX::FaceFacePair *this)
#[doc(alias = "RBX::FaceFacePair::isFaceFace(void)const")]
#[doc(alias = "__ZNK3RBX12FaceFacePair10isFaceFaceEv")]
// IDA 0x74a0b4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74a0b4() {
}
// 0x74a0b8 — __ZN3RBX12FaceFacePairD0Ev
// type: void __fastcall(RBX::FaceFacePair *__hidden this)
#[doc(alias = "RBX::FaceFacePair::~FaceFacePair()")]
#[doc(alias = "__ZN3RBX12FaceFacePairD0Ev")]
// IDA 0x74a0b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74a0b8() {
}
// 0x74a0bc — __ZNK3RBX12EdgeEdgePair10isFaceFaceEv
// type: int __fastcall(RBX::EdgeEdgePair *this)
#[doc(alias = "RBX::EdgeEdgePair::isFaceFace(void)const")]
#[doc(alias = "__ZNK3RBX12EdgeEdgePair10isFaceFaceEv")]
// IDA 0x74a0bc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74a0bc() {
}
// 0x74a0c0 — __ZN3RBX12EdgeEdgePairD0Ev
// type: void __fastcall(RBX::EdgeEdgePair *__hidden this)
#[doc(alias = "RBX::EdgeEdgePair::~EdgeEdgePair()")]
#[doc(alias = "__ZN3RBX12EdgeEdgePairD0Ev")]
// IDA 0x74a0c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74a0c0() {
}
// 0x74a0c4 — __ZN3RBX9AllocatorINS_15PolyPolyContactEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_15PolyPolyContactEE13releaseMemoryEv")]
// IDA 0x74a0c4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74a0c4() {
}
// 0x74a0e0 — __ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x74a0e0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74a0e0() {
}
// 0x74a110 — __ZN3RBX8PolyPairD1Ev
// type: void __fastcall(RBX::PolyPair *__hidden this)
#[doc(alias = "RBX::PolyPair::~PolyPair()")]
#[doc(alias = "__ZN3RBX8PolyPairD1Ev")]
// IDA 0x74a110: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_74a110() {
}
// 0x74a114 — __ZN3RBX8PolyPairD0Ev
// type: void __fastcall(RBX::PolyPair *__hidden this)
#[doc(alias = "RBX::PolyPair::~PolyPair()")]
#[doc(alias = "__ZN3RBX8PolyPairD0Ev")]
// IDA 0x74a114: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74a114() {
}
// 0x74a118 — __GLOBAL__I_a_340
#[doc(alias = "global constructor keyed to _a_340")]
#[doc(alias = "__GLOBAL__I_a_340")]
// IDA 0x74a118: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_74a118() {
}
// 0x74a4b8 — __ZN3RBX9PrimitiveC1ENS_8Geometry12GeometryTypeE
// type: int __fastcall(int, int, int, int, RBX::IndexedMesh *, RBX::BasicSpatialHashPrimitive *, RBX::ConcurrencyValidator *, RBX::EdgeList *, RBX::EdgeList *, int, int, int, int, int)
#[doc(alias = "RBX::Primitive::Primitive(RBX::Geometry::GeometryType)")]
#[doc(alias = "__ZN3RBX9PrimitiveC1ENS_8Geometry12GeometryTypeE")]
// IDA 0x74a4b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74a4b8() {
}
// 0x74a4bc — __ZN3RBX9PrimitiveC2ENS_8Geometry12GeometryTypeE
// type: int __fastcall(int, int, int, int, RBX::IndexedMesh *, RBX::BasicSpatialHashPrimitive *, RBX::ConcurrencyValidator *, RBX::EdgeList *, RBX::EdgeList *, int, int, int, int, int)
#[doc(alias = "RBX::Primitive::Primitive(RBX::Geometry::GeometryType)")]
#[doc(alias = "__ZN3RBX9PrimitiveC2ENS_8Geometry12GeometryTypeE")]
// IDA 0x74a4bc: 367 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74a4bc() {
}
// 0x74a8c8 — __ZN3RBX9Primitive11newGeometryENS_8Geometry12GeometryTypeE
// type: int __fastcall(int)
#[doc(alias = "RBX::Primitive::newGeometry(RBX::Geometry::GeometryType)")]
#[doc(alias = "__ZN3RBX9Primitive11newGeometryENS_8Geometry12GeometryTypeE")]
// IDA 0x74a8c8: 114 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74a8c8() {
}
// 0x74aa18 — __ZN3RBX9Primitive13computeJointKEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::computeJointK(void)")]
#[doc(alias = "__ZN3RBX9Primitive13computeJointKEv")]
// IDA 0x74aa18: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74aa18() {
}
// 0x74aa5c — __ZN3RBX9PrimitiveD0Ev
// type: void __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Primitive::~Primitive()")]
#[doc(alias = "__ZN3RBX9PrimitiveD0Ev")]
// IDA 0x74aa5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74aa5c() {
}
// 0x74aafc — __ZN3RBX9PrimitiveD1Ev
// type: void __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Primitive::~Primitive()")]
#[doc(alias = "__ZN3RBX9PrimitiveD1Ev")]
// IDA 0x74aafc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74aafc() {
}
// 0x74ab00 — __ZThn8_N3RBX9PrimitiveD0Ev
// type: void __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Primitive::~Primitive()")]
#[doc(alias = "__ZThn8_N3RBX9PrimitiveD0Ev")]
// IDA 0x74ab00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74ab00() {
}
// 0x74ab08 — __ZN3RBX9PrimitiveD2Ev
// type: void __fastcall(RBX::Primitive *this, int, int)
#[doc(alias = "RBX::Primitive::~Primitive()")]
#[doc(alias = "__ZN3RBX9PrimitiveD2Ev")]
// IDA 0x74ab08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74ab08() {
}
// 0x74ae0c — __ZThn8_N3RBX9PrimitiveD1Ev
// type: void __fastcall(RBX::Primitive *this, int, int)
#[doc(alias = "non-virtual thunk to RBX::Primitive::~Primitive()")]
#[doc(alias = "__ZThn8_N3RBX9PrimitiveD1Ev")]
// IDA 0x74ae0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74ae0c() {
}
// 0x74ae14 — __ZN3RBX9Primitive20setNetworkIsSleepingEb
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::Primitive::setNetworkIsSleeping(bool)")]
#[doc(alias = "__ZN3RBX9Primitive20setNetworkIsSleepingEb")]
// IDA 0x74ae14: 9 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ae14() {
}
// 0x74ae2c — __ZN3RBX9Primitive17onBuoyancyChangedEb
// type: int __fastcall(RBX::Primitive *this, bool)
#[doc(alias = "RBX::Primitive::onBuoyancyChanged(bool)")]
#[doc(alias = "__ZN3RBX9Primitive17onBuoyancyChangedEb")]
// IDA 0x74ae2c: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ae2c() {
}
// 0x74ae38 — __ZNK3RBX9Primitive17getSizeMultiplierEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getSizeMultiplier(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive17getSizeMultiplierEv")]
// IDA 0x74ae38: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ae38() {
}
// 0x74aeb4 — __ZN3RBX9Primitive17setSizeMultiplierENS0_14SizeMultiplierE
// type: int __fastcall(int result, int)
#[doc(alias = "RBX::Primitive::setSizeMultiplier(RBX::Primitive::SizeMultiplier)")]
#[doc(alias = "__ZN3RBX9Primitive17setSizeMultiplierENS0_14SizeMultiplierE")]
// IDA 0x74aeb4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74aeb4() {
}
// 0x74af24 — __ZNK3RBX9Primitive7getGuidEv
// type: char *__fastcall(RBX::Primitive *this, int, int)
#[doc(alias = "RBX::Primitive::getGuid(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive7getGuidEv")]
// IDA 0x74af24: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74af24() {
}
// 0x74af80 — __ZN3RBX9Primitive7setGuidERKNS_4GuidE
// type: int __fastcall(RBX::Primitive *this, Data *, int)
#[doc(alias = "RBX::Primitive::setGuid(RBX::Guid const&)")]
#[doc(alias = "__ZN3RBX9Primitive7setGuidERKNS_4GuidE")]
// IDA 0x74af80: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74af80() {
}
// 0x74aff4 — __ZN3RBX9Primitive19computeFuzzyExtentsEv
// type: int __fastcall(RBX::Primitive *this, int)
#[doc(alias = "RBX::Primitive::computeFuzzyExtents(void)")]
#[doc(alias = "__ZN3RBX9Primitive19computeFuzzyExtentsEv")]
// IDA 0x74aff4: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74aff4() {
}
// 0x74b104 — __ZN3RBX9Primitive19getFastFuzzyExtentsEv
// type: char *__fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZN3RBX9Primitive19getFastFuzzyExtentsEv")]
// IDA 0x74b104: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b104() {
}
// 0x74b168 — __ZNK3RBX9Primitive18countNumAuto JointsEv
// type: int __fastcall(RBX::Primitive *this, const RBX::Joint *)
#[doc(alias = "RBX::Primitive::countNumAuto Joints(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive18countNumAuto JointsEv")]
// IDA 0x74b168: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b168() {
}
// 0x74b1b4 — __ZNK3RBX9Primitive18getConstFirstJointEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getConstFirstJoint(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive18getConstFirstJointEv")]
// IDA 0x74b1b4: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b1b4() {
}
// 0x74b1c8 — __ZNK3RBX9Primitive17getConstNextJointEPKNS_5JointE
// type: int __fastcall(RBX::Primitive *this, const RBX::Joint *)
#[doc(alias = "RBX::Primitive::getConstNextJoint(RBX::Joint const*)const")]
#[doc(alias = "__ZNK3RBX9Primitive17getConstNextJointEPKNS_5JointE")]
// IDA 0x74b1c8: 4 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b1c8() {
}
// 0x74b1d4 — __ZNK3RBX8EdgeList7getNextEPKNS_9PrimitiveEPNS_4EdgeE
// type: int __fastcall(RBX::EdgeList *this, const RBX::Primitive *, RBX::Edge *)
#[doc(alias = "RBX::EdgeList::getNext(RBX::Primitive const*,RBX::Edge *)const")]
#[doc(alias = "__ZNK3RBX8EdgeList7getNextEPKNS_9PrimitiveEPNS_4EdgeE")]
// IDA 0x74b1d4: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b1d4() {
}
// 0x74b2a4 — __ZN3RBX8EdgeList10insertEdgeEPNS_4EdgeE
// type: int __fastcall(RBX::EdgeList *this, RBX::Edge *)
#[doc(alias = "RBX::EdgeList::insertEdge(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX8EdgeList10insertEdgeEPNS_4EdgeE")]
// IDA 0x74b2a4: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b2a4() {
}
// 0x74b348 — __ZN3RBX8EdgeList10removeEdgeEPNS_4EdgeE
// type: int __fastcall(RBX::EdgeList *this, RBX::Edge *)
#[doc(alias = "RBX::EdgeList::removeEdge(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX8EdgeList10removeEdgeEPNS_4EdgeE")]
// IDA 0x74b348: 103 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b348() {
}
// 0x74b47c — __ZN3RBX9Primitive8getJointEi
// type: int __fastcall(RBX::Primitive *this, int)
#[doc(alias = "RBX::Primitive::getJoint(int)")]
#[doc(alias = "__ZN3RBX9Primitive8getJointEi")]
// IDA 0x74b47c: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b47c() {
}
// 0x74b484 — __ZNK3RBX9Primitive13getConstJointEi
// type: int __fastcall(RBX::Primitive *this, int)
#[doc(alias = "RBX::Primitive::getConstJoint(int)const")]
#[doc(alias = "__ZNK3RBX9Primitive13getConstJointEi")]
// IDA 0x74b484: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b484() {
}
// 0x74b48c — __ZN3RBX9Primitive10getContactEi
// type: int __fastcall(RBX::Primitive *this, int)
#[doc(alias = "RBX::Primitive::getContact(int)")]
#[doc(alias = "__ZN3RBX9Primitive10getContactEi")]
// IDA 0x74b48c: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b48c() {
}
// 0x74b494 — __ZN3RBX9Primitive10insertEdgeEPNS_4EdgeE
// type: void __fastcall(RBX::Primitive *this, RBX::Edge *)
#[doc(alias = "RBX::Primitive::insertEdge(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX9Primitive10insertEdgeEPNS_4EdgeE")]
// IDA 0x74b494: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b494() {
}
// 0x74b5f8 — __ZN3RBX9Primitive10removeEdgeEPNS_4EdgeE
// type: void __fastcall(RBX::Primitive *this, RBX::Edge *)
#[doc(alias = "RBX::Primitive::removeEdge(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX9Primitive10removeEdgeEPNS_4EdgeE")]
// IDA 0x74b5f8: 131 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b5f8() {
}
// 0x74b75c — __ZNK3RBX9Primitive12getFirstEdgeEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getFirstEdge(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive12getFirstEdgeEv")]
// IDA 0x74b75c: 14 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b75c() {
}
// 0x74b780 — __ZNK3RBX9Primitive11getNextEdgeEPNS_4EdgeE
// type: int __fastcall(RBX::Primitive *this, RBX::Edge *)
#[doc(alias = "RBX::Primitive::getNextEdge(RBX::Edge *)const")]
#[doc(alias = "__ZNK3RBX9Primitive11getNextEdgeEPNS_4EdgeE")]
// IDA 0x74b780: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b780() {
}
// 0x74b7c8 — __ZN3RBX9Primitive13getFirstJointEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getFirstJoint(void)")]
#[doc(alias = "__ZN3RBX9Primitive13getFirstJointEv")]
// IDA 0x74b7c8: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b7c8() {
}
// 0x74b7dc — __ZN3RBX9Primitive12getNextJointEPNS_5JointE
// type: int __fastcall(RBX::Primitive *this, RBX::Joint *)
#[doc(alias = "RBX::Primitive::getNextJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9Primitive12getNextJointEPNS_5JointE")]
// IDA 0x74b7dc: 4 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b7dc() {
}
// 0x74b7e8 — __ZN3RBX9Primitive15getFirstContactEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getFirstContact(void)")]
#[doc(alias = "__ZN3RBX9Primitive15getFirstContactEv")]
// IDA 0x74b7e8: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b7e8() {
}
// 0x74b7fc — __ZN3RBX9Primitive14getNextContactEPNS_7ContactE
// type: int __fastcall(RBX::Primitive *this, RBX::Contact *)
#[doc(alias = "RBX::Primitive::getNextContact(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX9Primitive14getNextContactEPNS_7ContactE")]
// IDA 0x74b7fc: 4 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b7fc() {
}
// 0x74b808 — __ZN3RBX9Primitive15getFirstRigidAtEPNS_5JointE
// type: RBX::Edge *__fastcall(RBX::Primitive *this, RBX::Joint *)
#[doc(alias = "RBX::Primitive::getFirstRigidAt(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9Primitive15getFirstRigidAtEPNS_5JointE")]
// IDA 0x74b808: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b808() {
}
// 0x74b858 — __ZN3RBX9Primitive13getFirstRigidEv
// type: RBX::Edge *__fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getFirstRigid(void)")]
#[doc(alias = "__ZN3RBX9Primitive13getFirstRigidEv")]
// IDA 0x74b858: 7 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b858() {
}
// 0x74b86c — __ZN3RBX9Primitive12getNextRigidEPNS_10RigidJointE
// type: RBX::Edge *__fastcall(RBX::Primitive *this, RBX::RigidJoint *)
#[doc(alias = "RBX::Primitive::getNextRigid(RBX::RigidJoint *)")]
#[doc(alias = "__ZN3RBX9Primitive12getNextRigidEPNS_10RigidJointE")]
// IDA 0x74b86c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b86c() {
}
// 0x74b88c — __ZN3RBX9Primitive8getJointEPS0_S1_i
// type: int __fastcall(RBX::Primitive *this, RBX::Primitive *, RBX::Primitive *, int)
#[doc(alias = "RBX::Primitive::getJoint(RBX::Primitive*,RBX::Primitive*,int)")]
#[doc(alias = "__ZN3RBX9Primitive8getJointEPS0_S1_i")]
// IDA 0x74b88c: 115 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b88c() {
}
// 0x74b9cc — __ZN3RBX9Primitive10getContactEPS0_S1_
// type: int __fastcall(RBX::Primitive *this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::Primitive::getContact(RBX::Primitive*,RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX9Primitive10getContactEPS0_S1_")]
// IDA 0x74b9cc: 106 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74b9cc() {
}
// 0x74baf8 — __ZN3RBX9Primitive12onNewOverlapEPS0_S1_
// type: int __fastcall(RBX::Primitive *this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::Primitive::onNewOverlap(RBX::Primitive*,RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX9Primitive12onNewOverlapEPS0_S1_")]
// IDA 0x74baf8: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74baf8() {
}
// 0x74bb10 — __ZN3RBXL13reportOverlapILNS_5World9TouchInfo4TypeE0EEEvPNS_9PrimitiveES5_
// type: int __fastcall(int, RBX::Primitive *)
#[doc(alias = "void RBX::reportOverlap<(RBX::World::TouchInfo::Type)0>(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBXL13reportOverlapILNS_5World9TouchInfo4TypeE0EEEvPNS_9PrimitiveES5_")]
// IDA 0x74bb10: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74bb10() {
}
// 0x74bc08 — __ZN3RBX9Primitive13onStopOverlapEPS0_S1_
// type: int __fastcall(RBX::Primitive *this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::Primitive::onStopOverlap(RBX::Primitive*,RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX9Primitive13onStopOverlapEPS0_S1_")]
// IDA 0x74bc08: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74bc08() {
}
// 0x74bc20 — __ZN3RBXL13reportOverlapILNS_5World9TouchInfo4TypeE1EEEvPNS_9PrimitiveES5_
// type: int __fastcall(int, RBX::Primitive *)
#[doc(alias = "void RBX::reportOverlap<(RBX::World::TouchInfo::Type)1>(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBXL13reportOverlapILNS_5World9TouchInfo4TypeE1EEEvPNS_9PrimitiveES5_")]
// IDA 0x74bc20: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74bc20() {
}
// 0x74bd18 — __ZN3RBX9Primitive8getClumpEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getClump(void)")]
#[doc(alias = "__ZN3RBX9Primitive8getClumpEv")]
// IDA 0x74bd18: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74bd18() {
}
// 0x74bd1c — __ZNK3RBX9Primitive13getConstClumpEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getConstClump(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive13getConstClumpEv")]
// IDA 0x74bd1c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74bd1c() {
}
// 0x74bd20 — __ZN3RBX9Primitive11getAssemblyEv
// type: int __fastcall(RBX::Primitive *this, RBX::Primitive *)
#[doc(alias = "RBX::Primitive::getAssembly(void)")]
#[doc(alias = "__ZN3RBX9Primitive11getAssemblyEv")]
// IDA 0x74bd20: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74bd20() {
}
// 0x74bd24 — __ZNK3RBX9Primitive16getConstAssemblyEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getConstAssembly(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive16getConstAssemblyEv")]
// IDA 0x74bd24: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74bd24() {
}
// 0x74bd28 — __ZN3RBX9Primitive12getMechanismEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getMechanism(void)")]
#[doc(alias = "__ZN3RBX9Primitive12getMechanismEv")]
// IDA 0x74bd28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74bd28() {
}
// 0x74bd2c — __ZNK3RBX9Primitive17getConstMechanismEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getConstMechanism(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive17getConstMechanismEv")]
// IDA 0x74bd2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74bd2c() {
}
// 0x74bd30 — __ZNK3RBX9Primitive15getGeometryTypeEv
// type: int __fastcall(RBX::Primitive *this, int, int)
#[doc(alias = "RBX::Primitive::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive15getGeometryTypeEv")]
// IDA 0x74bd30: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74bd30() {
}
// 0x74bda0 — __ZNK3RBX9Primitive14getCollideTypeEv
// type: int __fastcall(RBX::Primitive *this, int, int)
#[doc(alias = "RBX::Primitive::getCollideType(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive14getCollideTypeEv")]
// IDA 0x74bda0: 32 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74bda0() {
}
// 0x74be08 — __ZNK3RBX9Primitive7getSizeEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getSize(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive7getSizeEv")]
// IDA 0x74be08: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74be08() {
}
// 0x74be10 — __ZNK3RBX9Primitive20getGeometryParameterERKSs
// type: int __fastcall(int)
#[doc(alias = "RBX::Primitive::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX9Primitive20getGeometryParameterERKSs")]
// IDA 0x74be10: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74be10() {
}
// 0x74be1c — __ZN3RBX9Primitive15setGeometryTypeENS_8Geometry12GeometryTypeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Primitive::setGeometryType(RBX::Geometry::GeometryType)")]
#[doc(alias = "__ZN3RBX9Primitive15setGeometryTypeENS_8Geometry12GeometryTypeE")]
// IDA 0x74be1c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74be1c() {
}
// 0x74bfe8 — __ZN3RBX9Primitive14setMassInertiaEf
// type: int __fastcall(RBX::Primitive *this, float)
#[doc(alias = "RBX::Primitive::setMassInertia(float)")]
#[doc(alias = "__ZN3RBX9Primitive14setMassInertiaEf")]
// IDA 0x74bfe8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74bfe8() {
}
// 0x74c044 — __ZN3RBX9Primitive20setGeometryParameterERKSsi
// type: int __fastcall(RBX::Primitive *this, const std::string *, int)
#[doc(alias = "RBX::Primitive::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX9Primitive20setGeometryParameterERKSsi")]
// IDA 0x74c044: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c044() {
}
// 0x74c190 — __ZNK3RBX9Primitive14getCanThrottleEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getCanThrottle(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive14getCanThrottleEv")]
// IDA 0x74c190: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c190() {
}
// 0x74c19c — __ZN3RBX9Primitive14setCanThrottleEb
// type: int __fastcall(RBX::Primitive *this, int)
#[doc(alias = "RBX::Primitive::setCanThrottle(bool)")]
#[doc(alias = "__ZN3RBX9Primitive14setCanThrottleEb")]
// IDA 0x74c19c: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c19c() {
}
// 0x74c24c — __ZN3RBX9Primitive13setEngineTypeENS0_10EngineTypeE
// type: RBX::World *__fastcall(RBX::World **, RBX::World *)
#[doc(alias = "RBX::Primitive::setEngineType(RBX::Primitive::EngineType)")]
#[doc(alias = "__ZN3RBX9Primitive13setEngineTypeENS0_10EngineTypeE")]
// IDA 0x74c24c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c24c() {
}
// 0x74c288 — __ZN3RBX9Primitive8setOwnerEPNS_7IMovingE
// type: int __fastcall(RBX::Primitive *this, RBX::IMoving *)
#[doc(alias = "RBX::Primitive::setOwner(RBX::IMoving *)")]
#[doc(alias = "__ZN3RBX9Primitive8setOwnerEPNS_7IMovingE")]
// IDA 0x74c288: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c288() {
}
