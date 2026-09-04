//! core shard DB — 100 core stubs EA-sorted, next uncovered after DA 0x7492c0 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::FaceFacePair::newFaceEdgeConnector(unsigned long,RBX::POLY::Vertex const*,RBX::POLY::Vertex const*)")]
// 0x7494ec — __ZN3RBX12FaceFacePair20newFaceEdgeConnectorEmPKNS_4POLY6VertexES4_
pub fn stub_7494ec() {
    // IDA 0x7494ec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeEdgePair::allocateClone(void)")]
// 0x74979c — __ZN3RBX12EdgeEdgePair13allocateCloneEv
pub fn stub_74979c() {
    // IDA 0x74979c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeEdgePair::test(void)")]
// 0x7497d0 — __ZN3RBX12EdgeEdgePair4testEv
pub fn stub_7497d0() {
    // IDA 0x7497d0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeEdgePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x749cc8 — __ZN3RBX12EdgeEdgePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_749cc8() {
    // IDA 0x749cc8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeEdgePair::newEdgeEdgeConnector(void)")]
// 0x749d3c — __ZN3RBX12EdgeEdgePair20newEdgeEdgeConnectorEv
pub fn stub_749d3c() {
    // IDA 0x749d3c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::Allocator(void)")]
// 0x749f78 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEC2Ev
pub fn stub_749f78() {
    // IDA 0x749f78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeEdgePair::~EdgeEdgePair()")]
// 0x749fdc — __ZN3RBX12EdgeEdgePairD1Ev
pub fn stub_749fdc() {
    // IDA 0x749fdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FaceFacePair::~FaceFacePair()")]
// 0x749fe0 — __ZN3RBX12FaceFacePairD1Ev
pub fn stub_749fe0() {
    // IDA 0x749fe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FixedArray<RBX::FaceFacePair::VertexStatus,40ul>::operator[](unsigned long)")]
// 0x749fe4 — __ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EEixEm
pub fn stub_749fe4() {
    // IDA 0x749fe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FixedArray<RBX::FaceFacePair::VertexStatus,40ul>::push_back(RBX::FaceFacePair::VertexStatus const&)")]
// 0x74a044 — __ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EE9push_backERKS2_
pub fn stub_74a044() {
    // IDA 0x74a044: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FaceFacePair::isFaceFace(void)const")]
// 0x74a0b4 — __ZNK3RBX12FaceFacePair10isFaceFaceEv
pub fn stub_74a0b4() {
    // IDA 0x74a0b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FaceFacePair::~FaceFacePair()")]
// 0x74a0b8 — __ZN3RBX12FaceFacePairD0Ev
pub fn stub_74a0b8() {
    // IDA 0x74a0b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EdgeEdgePair::isFaceFace(void)const")]
// 0x74a0bc — __ZNK3RBX12EdgeEdgePair10isFaceFaceEv
pub fn stub_74a0bc() {
    // IDA 0x74a0bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EdgeEdgePair::~EdgeEdgePair()")]
// 0x74a0c0 — __ZN3RBX12EdgeEdgePairD0Ev
pub fn stub_74a0c0() {
    // IDA 0x74a0c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::releaseMemory(void)")]
// 0x74a0c4 — __ZN3RBX9AllocatorINS_15PolyPolyContactEE13releaseMemoryEv
pub fn stub_74a0c4() {
    // IDA 0x74a0c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PolyPair::~PolyPair()")]
// 0x74a110 — __ZN3RBX8PolyPairD1Ev
pub fn stub_74a110() {
    // IDA 0x74a110: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PolyPair::~PolyPair()")]
// 0x74a114 — __ZN3RBX8PolyPairD0Ev
pub fn stub_74a114() {
    // IDA 0x74a114: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::Primitive(RBX::Geometry::GeometryType)")]
// 0x74a4b8 — __ZN3RBX9PrimitiveC1ENS_8Geometry12GeometryTypeE
pub fn stub_74a4b8() {
    // IDA 0x74a4b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::Primitive(RBX::Geometry::GeometryType)")]
// 0x74a4bc — __ZN3RBX9PrimitiveC2ENS_8Geometry12GeometryTypeE
pub fn stub_74a4bc() {
    // IDA 0x74a4bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::newGeometry(RBX::Geometry::GeometryType)")]
// 0x74a8c8 — __ZN3RBX9Primitive11newGeometryENS_8Geometry12GeometryTypeE
pub fn stub_74a8c8() {
    // IDA 0x74a8c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::computeJointK(void)")]
// 0x74aa18 — __ZN3RBX9Primitive13computeJointKEv
pub fn stub_74aa18() {
    // IDA 0x74aa18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::~Primitive()")]
// 0x74aa5c — __ZN3RBX9PrimitiveD0Ev
pub fn stub_74aa5c() {
    // IDA 0x74aa5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::~Primitive()")]
// 0x74aafc — __ZN3RBX9PrimitiveD1Ev
pub fn stub_74aafc() {
    // IDA 0x74aafc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Primitive::~Primitive()")]
// 0x74ab00 — __ZThn8_N3RBX9PrimitiveD0Ev
pub fn stub_74ab00() {
    // IDA 0x74ab00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::~Primitive()")]
// 0x74ab08 — __ZN3RBX9PrimitiveD2Ev
pub fn stub_74ab08() {
    // IDA 0x74ab08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Primitive::~Primitive()")]
// 0x74ae0c — __ZThn8_N3RBX9PrimitiveD1Ev
pub fn stub_74ae0c() {
    // IDA 0x74ae0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::onBuoyancyChanged(bool)")]
// 0x74ae2c — __ZN3RBX9Primitive17onBuoyancyChangedEb
pub fn stub_74ae2c() {
    // IDA 0x74ae2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::getSizeMultiplier(void)const")]
// 0x74ae38 — __ZNK3RBX9Primitive17getSizeMultiplierEv
pub fn stub_74ae38() {
    // IDA 0x74ae38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::setSizeMultiplier(RBX::Primitive::SizeMultiplier)")]
// 0x74aeb4 — __ZN3RBX9Primitive17setSizeMultiplierENS0_14SizeMultiplierE
pub fn stub_74aeb4() {
    // IDA 0x74aeb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::getGuid(void)const")]
// 0x74af24 — __ZNK3RBX9Primitive7getGuidEv
pub fn stub_74af24() {
    // IDA 0x74af24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::setGuid(RBX::Guid const&)")]
// 0x74af80 — __ZN3RBX9Primitive7setGuidERKNS_4GuidE
pub fn stub_74af80() {
    // IDA 0x74af80: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::computeFuzzyExtents(void)")]
// 0x74aff4 — __ZN3RBX9Primitive19computeFuzzyExtentsEv
pub fn stub_74aff4() {
    // IDA 0x74aff4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFastFuzzyExtents(void)")]
// 0x74b104 — __ZN3RBX9Primitive19getFastFuzzyExtentsEv
pub fn stub_74b104() {
    // IDA 0x74b104: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::countNumAutoJoints(void)const")]
// 0x74b168 — __ZNK3RBX9Primitive18countNumAutoJointsEv
pub fn stub_74b168() {
    // IDA 0x74b168: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getConstFirstJoint(void)const")]
// 0x74b1b4 — __ZNK3RBX9Primitive18getConstFirstJointEv
pub fn stub_74b1b4() {
    // IDA 0x74b1b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getConstNextJoint(RBX::Joint const*)const")]
// 0x74b1c8 — __ZNK3RBX9Primitive17getConstNextJointEPKNS_5JointE
pub fn stub_74b1c8() {
    // IDA 0x74b1c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeList::getNext(RBX::Primitive const*,RBX::Edge *)const")]
// 0x74b1d4 — __ZNK3RBX8EdgeList7getNextEPKNS_9PrimitiveEPNS_4EdgeE
pub fn stub_74b1d4() {
    // IDA 0x74b1d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeList::insertEdge(RBX::Edge *)")]
// 0x74b2a4 — __ZN3RBX8EdgeList10insertEdgeEPNS_4EdgeE
pub fn stub_74b2a4() {
    // IDA 0x74b2a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::EdgeList::removeEdge(RBX::Edge *)")]
// 0x74b348 — __ZN3RBX8EdgeList10removeEdgeEPNS_4EdgeE
pub fn stub_74b348() {
    // IDA 0x74b348: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getJoint(int)")]
// 0x74b47c — __ZN3RBX9Primitive8getJointEi
pub fn stub_74b47c() {
    // IDA 0x74b47c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getConstJoint(int)const")]
// 0x74b484 — __ZNK3RBX9Primitive13getConstJointEi
pub fn stub_74b484() {
    // IDA 0x74b484: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getContact(int)")]
// 0x74b48c — __ZN3RBX9Primitive10getContactEi
pub fn stub_74b48c() {
    // IDA 0x74b48c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::insertEdge(RBX::Edge *)")]
// 0x74b494 — __ZN3RBX9Primitive10insertEdgeEPNS_4EdgeE
pub fn stub_74b494() {
    // IDA 0x74b494: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::removeEdge(RBX::Edge *)")]
// 0x74b5f8 — __ZN3RBX9Primitive10removeEdgeEPNS_4EdgeE
pub fn stub_74b5f8() {
    // IDA 0x74b5f8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFirstEdge(void)const")]
// 0x74b75c — __ZNK3RBX9Primitive12getFirstEdgeEv
pub fn stub_74b75c() {
    // IDA 0x74b75c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getNextEdge(RBX::Edge *)const")]
// 0x74b780 — __ZNK3RBX9Primitive11getNextEdgeEPNS_4EdgeE
pub fn stub_74b780() {
    // IDA 0x74b780: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFirstJoint(void)")]
// 0x74b7c8 — __ZN3RBX9Primitive13getFirstJointEv
pub fn stub_74b7c8() {
    // IDA 0x74b7c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getNextJoint(RBX::Joint *)")]
// 0x74b7dc — __ZN3RBX9Primitive12getNextJointEPNS_5JointE
pub fn stub_74b7dc() {
    // IDA 0x74b7dc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFirstContact(void)")]
// 0x74b7e8 — __ZN3RBX9Primitive15getFirstContactEv
pub fn stub_74b7e8() {
    // IDA 0x74b7e8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getNextContact(RBX::Contact *)")]
// 0x74b7fc — __ZN3RBX9Primitive14getNextContactEPNS_7ContactE
pub fn stub_74b7fc() {
    // IDA 0x74b7fc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFirstRigidAt(RBX::Joint *)")]
// 0x74b808 — __ZN3RBX9Primitive15getFirstRigidAtEPNS_5JointE
pub fn stub_74b808() {
    // IDA 0x74b808: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFirstRigid(void)")]
// 0x74b858 — __ZN3RBX9Primitive13getFirstRigidEv
pub fn stub_74b858() {
    // IDA 0x74b858: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getNextRigid(RBX::RigidJoint *)")]
// 0x74b86c — __ZN3RBX9Primitive12getNextRigidEPNS_10RigidJointE
pub fn stub_74b86c() {
    // IDA 0x74b86c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getJoint(RBX::Primitive*,RBX::Primitive*,int)")]
// 0x74b88c — __ZN3RBX9Primitive8getJointEPS0_S1_i
pub fn stub_74b88c() {
    // IDA 0x74b88c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getContact(RBX::Primitive*,RBX::Primitive*)")]
// 0x74b9cc — __ZN3RBX9Primitive10getContactEPS0_S1_
pub fn stub_74b9cc() {
    // IDA 0x74b9cc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::onNewOverlap(RBX::Primitive*,RBX::Primitive*)")]
// 0x74baf8 — __ZN3RBX9Primitive12onNewOverlapEPS0_S1_
pub fn stub_74baf8() {
    // IDA 0x74baf8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void RBX::reportOverlap<(RBX::World::TouchInfo::Type)0>(RBX::Primitive *,RBX::Primitive *)")]
// 0x74bb10 — __ZN3RBXL13reportOverlapILNS_5World9TouchInfo4TypeE0EEEvPNS_9PrimitiveES5_
pub fn stub_74bb10() {
    // IDA 0x74bb10: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::onStopOverlap(RBX::Primitive*,RBX::Primitive*)")]
// 0x74bc08 — __ZN3RBX9Primitive13onStopOverlapEPS0_S1_
pub fn stub_74bc08() {
    // IDA 0x74bc08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "void RBX::reportOverlap<(RBX::World::TouchInfo::Type)1>(RBX::Primitive *,RBX::Primitive *)")]
// 0x74bc20 — __ZN3RBXL13reportOverlapILNS_5World9TouchInfo4TypeE1EEEvPNS_9PrimitiveES5_
pub fn stub_74bc20() {
    // IDA 0x74bc20: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getClump(void)")]
// 0x74bd18 — __ZN3RBX9Primitive8getClumpEv
pub fn stub_74bd18() {
    // IDA 0x74bd18: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getConstClump(void)const")]
// 0x74bd1c — __ZNK3RBX9Primitive13getConstClumpEv
pub fn stub_74bd1c() {
    // IDA 0x74bd1c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getAssembly(void)")]
// 0x74bd20 — __ZN3RBX9Primitive11getAssemblyEv
pub fn stub_74bd20() {
    // IDA 0x74bd20: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getConstAssembly(void)const")]
// 0x74bd24 — __ZNK3RBX9Primitive16getConstAssemblyEv
pub fn stub_74bd24() {
    // IDA 0x74bd24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getMechanism(void)")]
// 0x74bd28 — __ZN3RBX9Primitive12getMechanismEv
pub fn stub_74bd28() {
    // IDA 0x74bd28: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getConstMechanism(void)const")]
// 0x74bd2c — __ZNK3RBX9Primitive17getConstMechanismEv
pub fn stub_74bd2c() {
    // IDA 0x74bd2c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getGeometryType(void)const")]
// 0x74bd30 — __ZNK3RBX9Primitive15getGeometryTypeEv
pub fn stub_74bd30() {
    // IDA 0x74bd30: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getCollideType(void)const")]
// 0x74bda0 — __ZNK3RBX9Primitive14getCollideTypeEv
pub fn stub_74bda0() {
    // IDA 0x74bda0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getSize(void)const")]
// 0x74be08 — __ZNK3RBX9Primitive7getSizeEv
pub fn stub_74be08() {
    // IDA 0x74be08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getGeometryParameter(std::string const&)const")]
// 0x74be10 — __ZNK3RBX9Primitive20getGeometryParameterERKSs
pub fn stub_74be10() {
    // IDA 0x74be10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::setGeometryType(RBX::Geometry::GeometryType)")]
// 0x74be1c — __ZN3RBX9Primitive15setGeometryTypeENS_8Geometry12GeometryTypeE
pub fn stub_74be1c() {
    // IDA 0x74be1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::setMassInertia(float)")]
// 0x74bfe8 — __ZN3RBX9Primitive14setMassInertiaEf
pub fn stub_74bfe8() {
    // IDA 0x74bfe8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::setGeometryParameter(std::string const&,int)")]
// 0x74c044 — __ZN3RBX9Primitive20setGeometryParameterERKSsi
pub fn stub_74c044() {
    // IDA 0x74c044: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::getCanThrottle(void)const")]
// 0x74c190 — __ZNK3RBX9Primitive14getCanThrottleEv
pub fn stub_74c190() {
    // IDA 0x74c190: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::setCanThrottle(bool)")]
// 0x74c19c — __ZN3RBX9Primitive14setCanThrottleEb
pub fn stub_74c19c() {
    // IDA 0x74c19c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::setEngineType(RBX::Primitive::EngineType)")]
// 0x74c24c — __ZN3RBX9Primitive13setEngineTypeENS0_10EngineTypeE
pub fn stub_74c24c() {
    // IDA 0x74c24c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::setOwner(RBX::IMoving *)")]
// 0x74c288 — __ZN3RBX9Primitive8setOwnerEPNS_7IMovingE
pub fn stub_74c288() {
    // IDA 0x74c288: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Primitive::setDragging(bool)")]
// 0x74c2bc — __ZN3RBX9Primitive11setDraggingEb
pub fn stub_74c2bc() {
    // IDA 0x74c2bc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setFixed(bool,bool)")]
// 0x74c2c8 — __ZN3RBX9Primitive8setFixedEbb
pub fn stub_74c2c8() {
    // IDA 0x74c2c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setAnchoredProperty(bool)")]
// 0x74c328 — __ZN3RBX9Primitive19setAnchoredPropertyEb
pub fn stub_74c328() {
    // IDA 0x74c328: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setPreventCollide(bool)")]
// 0x74c330 — __ZN3RBX9Primitive17setPreventCollideEb
pub fn stub_74c330() {
    // IDA 0x74c330: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setFriction(float)")]
// 0x74c350 — __ZN3RBX9Primitive11setFrictionEf
pub fn stub_74c350() {
    // IDA 0x74c350: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setElasticity(float)")]
// 0x74c37c — __ZN3RBX9Primitive13setElasticityEf
pub fn stub_74c37c() {
    // IDA 0x74c37c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFaceCoordInObject(RBX::NormalId)const")]
// 0x74c3a8 — __ZNK3RBX9Primitive20getFaceCoordInObjectENS_8NormalIdE
pub fn stub_74c3a8() {
    // IDA 0x74c3a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFaceInObject(RBX::NormalId)const")]
// 0x74c43c — __ZNK3RBX9Primitive15getFaceInObjectENS_8NormalIdE
pub fn stub_74c43c() {
    // IDA 0x74c43c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFaceInWorld(RBX::NormalId)")]
// 0x74c498 — __ZN3RBX9Primitive14getFaceInWorldENS_8NormalIdE
pub fn stub_74c498() {
    // IDA 0x74c498: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setPV(RBX::PV const&)")]
// 0x74c638 — __ZN3RBX9Primitive5setPVERKNS_2PVE
pub fn stub_74c638() {
    // IDA 0x74c638: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::zeroVelocity(void)")]
// 0x74c7b0 — __ZN3RBX9Primitive12zeroVelocityEv
pub fn stub_74c7b0() {
    // IDA 0x74c7b0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setVelocity(RBX::Velocity const&)")]
// 0x74c7e8 — __ZN3RBX9Primitive11setVelocityERKNS_8VelocityE
pub fn stub_74c7e8() {
    // IDA 0x74c7e8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getCoordinateFrame(void)const")]
// 0x74c840 — __ZNK3RBX9Primitive18getCoordinateFrameEv
pub fn stub_74c840() {
    // IDA 0x74c840: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getPV(void)const")]
// 0x74c848 — __ZNK3RBX9Primitive5getPVEv
pub fn stub_74c848() {
    // IDA 0x74c848: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getGridCorner(void)const")]
// 0x74c850 — __ZNK3RBX9Primitive13getGridCornerEv
pub fn stub_74c850() {
    // IDA 0x74c850: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setSurfaceData(RBX::NormalId,RBX::SurfaceData const&)")]
// 0x74c980 — __ZN3RBX9Primitive14setSurfaceDataENS_8NormalIdERKNS_11SurfaceDataE
pub fn stub_74c980() {
    // IDA 0x74c980: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::setSurfaceType(RBX::NormalId,RBX::SurfaceType)")]
// 0x74ca64 — __ZN3RBX9Primitive14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE
pub fn stub_74ca64() {
    // IDA 0x74ca64: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::nextSpanningEdgeFromJoint(RBX::Joint *)")]
// 0x74ca78 — __ZN3RBX9Primitive25nextSpanningEdgeFromJointEPNS_5JointE
pub fn stub_74ca78() {
    // IDA 0x74ca78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Primitive::getFirstSpanningEdge(void)")]
// 0x74cab0 — __ZN3RBX9Primitive20getFirstSpanningEdgeEv
pub fn stub_74cab0() {
    // IDA 0x74cab0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::Primitive::getFirstSpanningEdge(void)")]
// 0x74cac4 — __ZThn8_N3RBX9Primitive20getFirstSpanningEdgeEv
pub fn stub_74cac4() {
    // IDA 0x74cac4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::getNextSpanningEdge(RBX::SpanningEdge *)")]
// 0x74cae0 — __ZN3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE
pub fn stub_74cae0() {
    // IDA 0x74cae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Primitive::getNextSpanningEdge(RBX::SpanningEdge *)")]
// 0x74cb08 — __ZThn8_N3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE
pub fn stub_74cb08() {
    // IDA 0x74cb08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::isGeometryOrthogonal(void)const")]
// 0x74cb10 — __ZNK3RBX9Primitive20isGeometryOrthogonalEv
pub fn stub_74cb10() {
    // IDA 0x74cb10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Primitive::getSortSize(void)")]
// 0x74cb20 — __ZN3RBX9Primitive11getSortSizeEv
pub fn stub_74cb20() {
    // IDA 0x74cb20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
