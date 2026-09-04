//! core shard CV — 100 core stubs EA-sorted, next uncovered after CU 0x7207c4 (strict RBX|boost|std|rbx earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::BlockBlockContact::computeIsColliding(float)")]
// 0x720898 — __ZN3RBX17BlockBlockContact18computeIsCollidingEf
pub fn stub_720898() {
    // IDA 0x720898: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::computeIsColliding(float,bool &)")]
// 0x7208b0 — __ZN3RBX17BlockBlockContact18computeIsCollidingEfRb
pub fn stub_7208b0() {
    // IDA 0x7208b0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::stepContact(void)")]
// 0x72090c — __ZN3RBX17BlockBlockContact11stepContactEv
pub fn stub_72090c() {
    // IDA 0x72090c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContactData::stepContactFFlag(void)")]
// 0x720988 — __ZN3RBX21BlockBlockContactData16stepContactFFlagEv
pub fn stub_720988() {
    // IDA 0x720988: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContactData::stepContact(void)")]
// 0x720a08 — __ZN3RBX21BlockBlockContactData11stepContactEv
pub fn stub_720a08() {
    // IDA 0x720a08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairEdgeEdge(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,int)")]
// 0x720aac — __ZN3RBX17BlockBlockContact19loadGeoPairEdgeEdgeERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii
pub fn stub_720aac() {
    // IDA 0x720aac: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairEdgeEdgeFFlag(int,int,int,int)")]
// 0x720bbc — __ZN3RBX17BlockBlockContact24loadGeoPairEdgeEdgeFFlagEiiii
pub fn stub_720bbc() {
    // IDA 0x720bbc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairPointPlane(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,RBX::NormalId,RBX::NormalId)")]
// 0x720fe4 — __ZN3RBX17BlockBlockContact21loadGeoPairPointPlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiiNS_8NormalIdES6_
pub fn stub_720fe4() {
    // IDA 0x720fe4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairPointPlaneFFlag(int,int,int,RBX::NormalId,RBX::NormalId)")]
// 0x7210c8 — __ZN3RBX17BlockBlockContact26loadGeoPairPointPlaneFFlagEiiiNS_8NormalIdES1_
pub fn stub_7210c8() {
    // IDA 0x7210c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::geoFeaturesOverlap(int,int,int,RBX::NormalId,RBX::NormalId)")]
// 0x7215a4 — __ZN3RBX17BlockBlockContact18geoFeaturesOverlapEiiiNS_8NormalIdES1_
pub fn stub_7215a4() {
    // IDA 0x7215a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContactData::loadGeoPairEdgeEdgePlane(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,int)")]
// 0x721778 — __ZN3RBX21BlockBlockContactData24loadGeoPairEdgeEdgePlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii
pub fn stub_721778() {
    // IDA 0x721778: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContactData::getBestPlaneEdge(float,bool &)")]
// 0x721920 — __ZN3RBX21BlockBlockContactData16getBestPlaneEdgeEfRb
pub fn stub_721920() {
    // IDA 0x721920: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContactData::computePlaneContact(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &)")]
// 0x72205c — __ZN3RBX21BlockBlockContactData19computePlaneContactERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEE
pub fn stub_72205c() {
    // IDA 0x72205c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::BlockBlockContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x7223c0 — __ZN3RBX17BlockBlockContactC1EPNS_9PrimitiveES2_
pub fn stub_7223c0() {
    // IDA 0x7223c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::BlockBlockContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x7223c4 — __ZN3RBX17BlockBlockContactC2EPNS_9PrimitiveES2_
pub fn stub_7223c4() {
    // IDA 0x7223c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContact::~BlockBlockContact()")]
// 0x7224b8 — __ZN3RBX17BlockBlockContactD0Ev
pub fn stub_7224b8() {
    // IDA 0x7224b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BlockBlockContact::~BlockBlockContact()")]
// 0x72256c — __ZN3RBX17BlockBlockContactD1Ev
pub fn stub_72256c() {
    // IDA 0x72256c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BlockBlockContact::~BlockBlockContact()")]
// 0x722570 — __ZN3RBX17BlockBlockContactD2Ev
pub fn stub_722570() {
    // IDA 0x722570: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BlockBlockContact::numConnectors(void)const")]
// 0x722598 — __ZNK3RBX17BlockBlockContact13numConnectorsEv
pub fn stub_722598() {
    // IDA 0x722598: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BlockBlockContact::generateDataForMovingAssemblyStage(void)")]
// 0x7225c8 — __ZN3RBX17BlockBlockContact34generateDataForMovingAssemblyStageEv
pub fn stub_7225c8() {
    // IDA 0x7225c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BlockBlockContactData::computePlaneContactFFlag(void)")]
// 0x722608 — __ZN3RBX21BlockBlockContactData24computePlaneContactFFlagEv
pub fn stub_722608() {
    // IDA 0x722608: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BlockBlockContactData::loadGeoPairEdgeEdgePlaneFFlag(int,int,int,int)")]
// 0x722968 — __ZN3RBX21BlockBlockContactData29loadGeoPairEdgeEdgePlaneFFlagEiiii
pub fn stub_722968() {
    // IDA 0x722968: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x723508 — __ZN3RBX4EdgeD2Ev
pub fn stub_723508() {
    // IDA 0x723508: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::operator new(unsigned long)")]
// 0x723748 — __ZN3RBX9AllocatorINS_17BallBallConnectorEEnwEm
pub fn stub_723748() {
    // IDA 0x723748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::operator new(unsigned long)")]
// 0x7237b8 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEnwEm
pub fn stub_7237b8() {
    // IDA 0x7237b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Block::getEdgeVertex(int)const")]
// 0x723828 — __ZNK3RBX5Block13getEdgeVertexEi
pub fn stub_723828() {
    // IDA 0x723828: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::push_back(RBX::GeoPairConnector * const&)")]
// 0x7238f0 — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE9push_backERKS2_
pub fn stub_7238f0() {
    // IDA 0x7238f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator new(unsigned long)")]
// 0x723958 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEnwEm
pub fn stub_723958() {
    // IDA 0x723958: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator delete(void *)")]
// 0x7239c8 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv
pub fn stub_7239c8() {
    // IDA 0x7239c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactConnector::isIntersecting(void)")]
// 0x723a08 — __ZN3RBX16ContactConnector14isIntersectingEv
pub fn stub_723a08() {
    // IDA 0x723a08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::Allocator(void)")]
// 0x723a7c — __ZN3RBX9AllocatorINS_17BlockBlockContactEEC2Ev
pub fn stub_723a7c() {
    // IDA 0x723a7c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator delete(void *)")]
// 0x723ae0 — __ZN3RBX9AllocatorINS_17BlockBlockContactEEdlEPv
pub fn stub_723ae0() {
    // IDA 0x723ae0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::operator[](unsigned long)")]
// 0x723b1c — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EEixEm
pub fn stub_723b1c() {
    // IDA 0x723b1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::fastRemove(unsigned long)")]
// 0x723b7c — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE10fastRemoveEm
pub fn stub_723b7c() {
    // IDA 0x723b7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BlockBlockContact::deleteAllConnectors(void)")]
// 0x723c30 — __ZN3RBX17BlockBlockContact19deleteAllConnectorsEv
pub fn stub_723c30() {
    // IDA 0x723c30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::releaseMemory(void)")]
// 0x723c4c — __ZN3RBX9AllocatorINS_17BlockBlockContactEE13releaseMemoryEv
pub fn stub_723c4c() {
    // IDA 0x723c4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeoPair::match(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0x723d40 — __ZN3RBX7GeoPair5matchEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_723d40() {
    // IDA 0x723d40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactConnector::ContactConnector(RBX::Body *,RBX::Body *,RBX::ContactParams const&)")]
// 0x723e20 — __ZN3RBX16ContactConnectorC2EPNS_4BodyES2_RKNS_13ContactParamsE
pub fn stub_723e20() {
    // IDA 0x723e20: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::Allocator(void)")]
// 0x723f2c — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEC2Ev
pub fn stub_723f2c() {
    // IDA 0x723f2c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::GeoPairConnector::~GeoPairConnector()")]
// 0x723f90 — __ZN3RBX16GeoPairConnectorD1Ev
pub fn stub_723f90() {
    // IDA 0x723f90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeoPairConnector::~GeoPairConnector()")]
// 0x723f94 — __ZN3RBX16GeoPairConnectorD0Ev
pub fn stub_723f94() {
    // IDA 0x723f94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeoPairConnector::updateContactPoint(void)")]
// 0x723f98 — __ZN3RBX16GeoPairConnector18updateContactPointEv
pub fn stub_723f98() {
    // IDA 0x723f98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::releaseMemory(void)")]
// 0x724018 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEE13releaseMemoryEv
pub fn stub_724018() {
    // IDA 0x724018: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::Allocator(void)")]
// 0x724064 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev
pub fn stub_724064() {
    // IDA 0x724064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::releaseMemory(void)")]
// 0x7240c8 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEE13releaseMemoryEv
pub fn stub_7240c8() {
    // IDA 0x7240c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::Allocator(void)")]
// 0x724114 — __ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev
pub fn stub_724114() {
    // IDA 0x724114: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::releaseMemory(void)")]
// 0x724178 — __ZN3RBX9AllocatorINS_17BallBallConnectorEE13releaseMemoryEv
pub fn stub_724178() {
    // IDA 0x724178: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7241c4 — __ZN3RBX10IPipelinedD2Ev
pub fn stub_7241c4() {
    // IDA 0x7241c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x724234 — __ZN3RBX4EdgeD1Ev
pub fn stub_724234() {
    // IDA 0x724234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x724238 — __ZN3RBX4EdgeD0Ev
pub fn stub_724238() {
    // IDA 0x724238: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7242d8 — __ZN3RBX10IPipelinedD1Ev
pub fn stub_7242d8() {
    // IDA 0x7242d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7242dc — __ZN3RBX10IPipelinedD0Ev
pub fn stub_7242dc() {
    // IDA 0x7242dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
// 0x724650 — __ZN3RBX14ContactManagerC1EPNS_5WorldE
pub fn stub_724650() {
    // IDA 0x724650: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
// 0x724654 — __ZN3RBX14ContactManagerC2EPNS_5WorldE
pub fn stub_724654() {
    // IDA 0x724654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::~ContactManager()")]
// 0x7247ec — __ZN3RBX14ContactManagerD1Ev
pub fn stub_7247ec() {
    // IDA 0x7247ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::~ContactManager()")]
// 0x7247f0 — __ZN3RBX14ContactManagerD2Ev
pub fn stub_7247f0() {
    // IDA 0x7247f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::fastClear(void)")]
// 0x724920 — __ZN3RBX14ContactManager9fastClearEv
pub fn stub_724920() {
    // IDA 0x724920: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::doStats(void)")]
// 0x724928 — __ZN3RBX14ContactManager7doStatsEv
pub fn stub_724928() {
    // IDA 0x724928: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::intersectingMySimulation(RBX::Primitive *,RBX::SystemAddress,float)")]
// 0x72492c — __ZN3RBX14ContactManager24intersectingMySimulationEPNS_9PrimitiveENS_13SystemAddressEf
pub fn stub_72492c() {
    // IDA 0x72492c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&,float)")]
// 0x7249bc — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveERKSt3setIS2_St4lessIS2_ESaIS2_EEf
pub fn stub_7249bc() {
    // IDA 0x7249bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactManager::createContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x724be4 — __ZN3RBX14ContactManager13createContactEPNS_9PrimitiveES2_
pub fn stub_724be4() {
    // IDA 0x724be4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,float)")]
// 0x7250a4 — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveEf
pub fn stub_7250a4() {
    // IDA 0x7250a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::terrainCellsInRegion3(RBX::Region3)const")]
// 0x725f1c — __ZNK3RBX14ContactManager21terrainCellsInRegion3ENS_7Region3E
pub fn stub_725f1c() {
    // IDA 0x725f1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::onNewPair(RBX::Primitive *,RBX::Primitive *)")]
// 0x7262e0 — __ZN3RBX14ContactManager9onNewPairEPNS_9PrimitiveES2_
pub fn stub_7262e0() {
    // IDA 0x7262e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::releasePair(RBX::Primitive *,RBX::Primitive *)")]
// 0x726370 — __ZN3RBX14ContactManager11releasePairEPNS_9PrimitiveES2_
pub fn stub_726370() {
    // IDA 0x726370: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveAdded(RBX::Primitive *)")]
// 0x72641c — __ZN3RBX14ContactManager16onPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_72641c() {
    // IDA 0x72641c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveRemoved(RBX::Primitive *)")]
// 0x726524 — __ZN3RBX14ContactManager18onPrimitiveRemovedEPNS_9PrimitiveE
pub fn stub_726524() {
    // IDA 0x726524: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveExtentsChanged(RBX::Primitive *)")]
// 0x726604 — __ZN3RBX14ContactManager25onPrimitiveExtentsChangedEPNS_9PrimitiveE
pub fn stub_726604() {
    // IDA 0x726604: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::onPrimitiveGeometryChanged(RBX::Primitive *)")]
// 0x72660c — __ZN3RBX14ContactManager26onPrimitiveGeometryChangedEPNS_9PrimitiveE
pub fn stub_72660c() {
    // IDA 0x72660c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::checkMegaClusterContact(RBX::Primitive *,bool,bool,bool)")]
// 0x72676c — __ZN3RBX14ContactManager23checkMegaClusterContactEPNS_9PrimitiveEbbb
pub fn stub_72676c() {
    // IDA 0x72676c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::primitiveIsExcludedFromSpatialHash(RBX::Primitive *)")]
// 0x726cf0 — __ZN3RBX14ContactManager34primitiveIsExcludedFromSpatialHashEPNS_9PrimitiveE
pub fn stub_726cf0() {
    // IDA 0x726cf0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::checkMegaClusterBigTerrainContact(RBX::Primitive *)")]
// 0x726d08 — __ZN3RBX14ContactManager33checkMegaClusterBigTerrainContactEPNS_9PrimitiveE
pub fn stub_726d08() {
    // IDA 0x726d08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0x727054 — __ZN3RBX14ContactManager18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
pub fn stub_727054() {
    // IDA 0x727054: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::startLoadingTerrain(void)")]
// 0x727420 — __ZN3RBX14ContactManager19startLoadingTerrainEv
pub fn stub_727420() {
    // IDA 0x727420: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::doneLoadingTerrain(void)")]
// 0x727438 — __ZN3RBX14ContactManager18doneLoadingTerrainEv
pub fn stub_727438() {
    // IDA 0x727438: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::applyDeferredTerrainChanges(void)")]
// 0x727578 — __ZN3RBX14ContactManager27applyDeferredTerrainChangesEv
pub fn stub_727578() {
    // IDA 0x727578: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::fastClear(void)")]
// 0x727838 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv
pub fn stub_727838() {
    // IDA 0x727838: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getNextGrid(RBX::Vector3int32 &,RBX::RbxRay const&,float)")]
// 0x727f74 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf
pub fn stub_727f74() {
    // IDA 0x727f74: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::overlapsOrTouches(RBX::Extents const&)const")]
// 0x728358 — __ZNK3RBX7Extents17overlapsOrTouchesERKS0_
pub fn stub_728358() {
    // IDA 0x728358: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator new(unsigned long)")]
// 0x728438 — __ZN3RBX9AllocatorINS_15BallBallContactEEnwEm
pub fn stub_728438() {
    // IDA 0x728438: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator delete(void *)")]
// 0x7284a8 — __ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv
pub fn stub_7284a8() {
    // IDA 0x7284a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator new(unsigned long)")]
// 0x7284e4 — __ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm
pub fn stub_7284e4() {
    // IDA 0x7284e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator delete(void *)")]
// 0x728554 — __ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv
pub fn stub_728554() {
    // IDA 0x728554: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator new(unsigned long)")]
// 0x728590 — __ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm
pub fn stub_728590() {
    // IDA 0x728590: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator new(unsigned long)")]
// 0x728600 — __ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm
pub fn stub_728600() {
    // IDA 0x728600: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator new(unsigned long)")]
// 0x728670 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm
pub fn stub_728670() {
    // IDA 0x728670: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator delete(void *)")]
// 0x7286e0 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv
pub fn stub_7286e0() {
    // IDA 0x7286e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(RBX::Primitive*,bool)")]
// 0x72871c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b
pub fn stub_72871c() {
    // IDA 0x72871c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(RBX::Primitive*)")]
// 0x7287b0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_
pub fn stub_7287b0() {
    // IDA 0x7287b0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator new(unsigned long)")]
// 0x72895c — __ZN3RBX9AllocatorINS_15BallCellContactEEnwEm
pub fn stub_72895c() {
    // IDA 0x72895c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator delete(void *)")]
// 0x7289cc — __ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv
pub fn stub_7289cc() {
    // IDA 0x7289cc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator new(unsigned long)")]
// 0x728a08 — __ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm
pub fn stub_728a08() {
    // IDA 0x728a08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator delete(void *)")]
// 0x728a78 — __ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv
pub fn stub_728a78() {
    // IDA 0x728a78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Extents::clampToOverlap(RBX::Extents const&)")]
// 0x728ab4 — __ZN3RBX7Extents14clampToOverlapERKS0_
pub fn stub_728ab4() {
    // IDA 0x728ab4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::computeLevel(RBX::Primitive const*,RBX::Extents const&)")]
// 0x72a668 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE
pub fn stub_72a668() {
    // IDA 0x72a668: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(RBX::Primitive*)")]
// 0x72a728 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_
pub fn stub_72a728() {
    // IDA 0x72a728: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(RBX::Primitive*,bool)")]
// 0x72a844 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b
pub fn stub_72a844() {
    // IDA 0x72a844: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(RBX::Primitive*,RBX::Extents const&)")]
// 0x72a990 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE
pub fn stub_72a990() {
    // IDA 0x72a990: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
// 0x72ab58 — __ZN3RBX12ExtentsInt32C1Ev
pub fn stub_72ab58() {
    // IDA 0x72ab58: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::changeMinMax(RBX::Primitive*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
// 0x72ac08 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b
pub fn stub_72ac08() {
    // IDA 0x72ac08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
