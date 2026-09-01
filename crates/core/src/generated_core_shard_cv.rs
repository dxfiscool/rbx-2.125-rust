//! core shard CV — 100 core stubs EA-sorted, next uncovered after CU 0x7207c4 (strict RBX|boost|std|rbx earliest gap).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::BlockBlockContact::computeIsColliding(float)")]
// 0x720898 — __ZN3RBX17BlockBlockContact18computeIsCollidingEf
pub fn stub_720898() -> ! {
    todo!("0x720898 __ZN3RBX17BlockBlockContact18computeIsCollidingEf")
}

#[doc(alias = "RBX::BlockBlockContact::computeIsColliding(float,bool &)")]
// 0x7208b0 — __ZN3RBX17BlockBlockContact18computeIsCollidingEfRb
pub fn stub_7208b0() -> ! {
    todo!("0x7208b0 __ZN3RBX17BlockBlockContact18computeIsCollidingEfRb")
}

#[doc(alias = "RBX::BlockBlockContact::stepContact(void)")]
// 0x72090c — __ZN3RBX17BlockBlockContact11stepContactEv
pub fn stub_72090c() -> ! {
    todo!("0x72090c __ZN3RBX17BlockBlockContact11stepContactEv")
}

#[doc(alias = "RBX::BlockBlockContactData::stepContactFFlag(void)")]
// 0x720988 — __ZN3RBX21BlockBlockContactData16stepContactFFlagEv
pub fn stub_720988() -> ! {
    todo!("0x720988 __ZN3RBX21BlockBlockContactData16stepContactFFlagEv")
}

#[doc(alias = "RBX::BlockBlockContactData::stepContact(void)")]
// 0x720a08 — __ZN3RBX21BlockBlockContactData11stepContactEv
pub fn stub_720a08() -> ! {
    todo!("0x720a08 __ZN3RBX21BlockBlockContactData11stepContactEv")
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairEdgeEdge(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,int)")]
// 0x720aac — __ZN3RBX17BlockBlockContact19loadGeoPairEdgeEdgeERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii
pub fn stub_720aac() -> ! {
    todo!("0x720aac __ZN3RBX17BlockBlockContact19loadGeoPairEdgeEdgeERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii")
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairEdgeEdgeFFlag(int,int,int,int)")]
// 0x720bbc — __ZN3RBX17BlockBlockContact24loadGeoPairEdgeEdgeFFlagEiiii
pub fn stub_720bbc() -> ! {
    todo!("0x720bbc __ZN3RBX17BlockBlockContact24loadGeoPairEdgeEdgeFFlagEiiii")
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairPointPlane(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,RBX::NormalId,RBX::NormalId)")]
// 0x720fe4 — __ZN3RBX17BlockBlockContact21loadGeoPairPointPlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiiNS_8NormalIdES6_
pub fn stub_720fe4() -> ! {
    todo!("0x720fe4 __ZN3RBX17BlockBlockContact21loadGeoPairPointPlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiiNS_8NormalIdES6_")
}

#[doc(alias = "RBX::BlockBlockContact::loadGeoPairPointPlaneFFlag(int,int,int,RBX::NormalId,RBX::NormalId)")]
// 0x7210c8 — __ZN3RBX17BlockBlockContact26loadGeoPairPointPlaneFFlagEiiiNS_8NormalIdES1_
pub fn stub_7210c8() -> ! {
    todo!("0x7210c8 __ZN3RBX17BlockBlockContact26loadGeoPairPointPlaneFFlagEiiiNS_8NormalIdES1_")
}

#[doc(alias = "RBX::BlockBlockContact::geoFeaturesOverlap(int,int,int,RBX::NormalId,RBX::NormalId)")]
// 0x7215a4 — __ZN3RBX17BlockBlockContact18geoFeaturesOverlapEiiiNS_8NormalIdES1_
pub fn stub_7215a4() -> ! {
    todo!("0x7215a4 __ZN3RBX17BlockBlockContact18geoFeaturesOverlapEiiiNS_8NormalIdES1_")
}

#[doc(alias = "RBX::BlockBlockContactData::loadGeoPairEdgeEdgePlane(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,int,int,int,int)")]
// 0x721778 — __ZN3RBX21BlockBlockContactData24loadGeoPairEdgeEdgePlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii
pub fn stub_721778() -> ! {
    todo!("0x721778 __ZN3RBX21BlockBlockContactData24loadGeoPairEdgeEdgePlaneERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEEiiii")
}

#[doc(alias = "RBX::BlockBlockContactData::getBestPlaneEdge(float,bool &)")]
// 0x721920 — __ZN3RBX21BlockBlockContactData16getBestPlaneEdgeEfRb
pub fn stub_721920() -> ! {
    todo!("0x721920 __ZN3RBX21BlockBlockContactData16getBestPlaneEdgeEfRb")
}

#[doc(alias = "RBX::BlockBlockContactData::computePlaneContact(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &)")]
// 0x72205c — __ZN3RBX21BlockBlockContactData19computePlaneContactERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEE
pub fn stub_72205c() -> ! {
    todo!("0x72205c __ZN3RBX21BlockBlockContactData19computePlaneContactERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEE")
}

#[doc(alias = "RBX::BlockBlockContact::BlockBlockContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x7223c0 — __ZN3RBX17BlockBlockContactC1EPNS_9PrimitiveES2_
pub fn stub_7223c0() -> ! {
    todo!("0x7223c0 __ZN3RBX17BlockBlockContactC1EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BlockBlockContact::BlockBlockContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x7223c4 — __ZN3RBX17BlockBlockContactC2EPNS_9PrimitiveES2_
pub fn stub_7223c4() -> ! {
    todo!("0x7223c4 __ZN3RBX17BlockBlockContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BlockBlockContact::~BlockBlockContact()")]
// 0x7224b8 — __ZN3RBX17BlockBlockContactD0Ev
pub fn stub_7224b8() -> ! {
    todo!("0x7224b8 __ZN3RBX17BlockBlockContactD0Ev")
}

#[doc(alias = "RBX::BlockBlockContact::~BlockBlockContact()")]
// 0x72256c — __ZN3RBX17BlockBlockContactD1Ev
pub fn stub_72256c() -> ! {
    todo!("0x72256c __ZN3RBX17BlockBlockContactD1Ev")
}

#[doc(alias = "RBX::BlockBlockContact::~BlockBlockContact()")]
// 0x722570 — __ZN3RBX17BlockBlockContactD2Ev
pub fn stub_722570() -> ! {
    todo!("0x722570 __ZN3RBX17BlockBlockContactD2Ev")
}

#[doc(alias = "RBX::BlockBlockContact::numConnectors(void)const")]
// 0x722598 — __ZNK3RBX17BlockBlockContact13numConnectorsEv
pub fn stub_722598() -> ! {
    todo!("0x722598 __ZNK3RBX17BlockBlockContact13numConnectorsEv")
}

#[doc(alias = "RBX::BlockBlockContact::generateDataForMovingAssemblyStage(void)")]
// 0x7225c8 — __ZN3RBX17BlockBlockContact34generateDataForMovingAssemblyStageEv
pub fn stub_7225c8() -> ! {
    todo!("0x7225c8 __ZN3RBX17BlockBlockContact34generateDataForMovingAssemblyStageEv")
}

#[doc(alias = "RBX::BlockBlockContactData::computePlaneContactFFlag(void)")]
// 0x722608 — __ZN3RBX21BlockBlockContactData24computePlaneContactFFlagEv
pub fn stub_722608() -> ! {
    todo!("0x722608 __ZN3RBX21BlockBlockContactData24computePlaneContactFFlagEv")
}

#[doc(alias = "RBX::BlockBlockContactData::loadGeoPairEdgeEdgePlaneFFlag(int,int,int,int)")]
// 0x722968 — __ZN3RBX21BlockBlockContactData29loadGeoPairEdgeEdgePlaneFFlagEiiii
pub fn stub_722968() -> ! {
    todo!("0x722968 __ZN3RBX21BlockBlockContactData29loadGeoPairEdgeEdgePlaneFFlagEiiii")
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x723508 — __ZN3RBX4EdgeD2Ev
pub fn stub_723508() -> ! {
    todo!("0x723508 __ZN3RBX4EdgeD2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::operator new(unsigned long)")]
// 0x723748 — __ZN3RBX9AllocatorINS_17BallBallConnectorEEnwEm
pub fn stub_723748() -> ! {
    todo!("0x723748 __ZN3RBX9AllocatorINS_17BallBallConnectorEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::operator new(unsigned long)")]
// 0x7237b8 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEnwEm
pub fn stub_7237b8() -> ! {
    todo!("0x7237b8 __ZN3RBX9AllocatorINS_18BallBlockConnectorEEnwEm")
}

#[doc(alias = "RBX::Block::getEdgeVertex(int)const")]
// 0x723828 — __ZNK3RBX5Block13getEdgeVertexEi
pub fn stub_723828() -> ! {
    todo!("0x723828 __ZNK3RBX5Block13getEdgeVertexEi")
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::push_back(RBX::GeoPairConnector * const&)")]
// 0x7238f0 — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE9push_backERKS2_
pub fn stub_7238f0() -> ! {
    todo!("0x7238f0 __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE9push_backERKS2_")
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator new(unsigned long)")]
// 0x723958 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEnwEm
pub fn stub_723958() -> ! {
    todo!("0x723958 __ZN3RBX9AllocatorINS_16GeoPairConnectorEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator delete(void *)")]
// 0x7239c8 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv
pub fn stub_7239c8() -> ! {
    todo!("0x7239c8 __ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv")
}

#[doc(alias = "RBX::ContactConnector::isIntersecting(void)")]
// 0x723a08 — __ZN3RBX16ContactConnector14isIntersectingEv
pub fn stub_723a08() -> ! {
    todo!("0x723a08 __ZN3RBX16ContactConnector14isIntersectingEv")
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::Allocator(void)")]
// 0x723a7c — __ZN3RBX9AllocatorINS_17BlockBlockContactEEC2Ev
pub fn stub_723a7c() -> ! {
    todo!("0x723a7c __ZN3RBX9AllocatorINS_17BlockBlockContactEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator delete(void *)")]
// 0x723ae0 — __ZN3RBX9AllocatorINS_17BlockBlockContactEEdlEPv
pub fn stub_723ae0() -> ! {
    todo!("0x723ae0 __ZN3RBX9AllocatorINS_17BlockBlockContactEEdlEPv")
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::operator[](unsigned long)")]
// 0x723b1c — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EEixEm
pub fn stub_723b1c() -> ! {
    todo!("0x723b1c __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EEixEm")
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::fastRemove(unsigned long)")]
// 0x723b7c — __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE10fastRemoveEm
pub fn stub_723b7c() -> ! {
    todo!("0x723b7c __ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE10fastRemoveEm")
}

#[doc(alias = "RBX::BlockBlockContact::deleteAllConnectors(void)")]
// 0x723c30 — __ZN3RBX17BlockBlockContact19deleteAllConnectorsEv
pub fn stub_723c30() -> ! {
    todo!("0x723c30 __ZN3RBX17BlockBlockContact19deleteAllConnectorsEv")
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::releaseMemory(void)")]
// 0x723c4c — __ZN3RBX9AllocatorINS_17BlockBlockContactEE13releaseMemoryEv
pub fn stub_723c4c() -> ! {
    todo!("0x723c4c __ZN3RBX9AllocatorINS_17BlockBlockContactEE13releaseMemoryEv")
}

#[doc(alias = "RBX::GeoPair::match(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0x723d40 — __ZN3RBX7GeoPair5matchEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_723d40() -> ! {
    todo!("0x723d40 __ZN3RBX7GeoPair5matchEPNS_4BodyES2_NS_11GeoPairTypeEii")
}

#[doc(alias = "RBX::ContactConnector::ContactConnector(RBX::Body *,RBX::Body *,RBX::ContactParams const&)")]
// 0x723e20 — __ZN3RBX16ContactConnectorC2EPNS_4BodyES2_RKNS_13ContactParamsE
pub fn stub_723e20() -> ! {
    todo!("0x723e20 __ZN3RBX16ContactConnectorC2EPNS_4BodyES2_RKNS_13ContactParamsE")
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::Allocator(void)")]
// 0x723f2c — __ZN3RBX9AllocatorINS_16GeoPairConnectorEEC2Ev
pub fn stub_723f2c() -> ! {
    todo!("0x723f2c __ZN3RBX9AllocatorINS_16GeoPairConnectorEEC2Ev")
}

#[doc(alias = "RBX::GeoPairConnector::~GeoPairConnector()")]
// 0x723f90 — __ZN3RBX16GeoPairConnectorD1Ev
pub fn stub_723f90() -> ! {
    todo!("0x723f90 __ZN3RBX16GeoPairConnectorD1Ev")
}

#[doc(alias = "RBX::GeoPairConnector::~GeoPairConnector()")]
// 0x723f94 — __ZN3RBX16GeoPairConnectorD0Ev
pub fn stub_723f94() -> ! {
    todo!("0x723f94 __ZN3RBX16GeoPairConnectorD0Ev")
}

#[doc(alias = "RBX::GeoPairConnector::updateContactPoint(void)")]
// 0x723f98 — __ZN3RBX16GeoPairConnector18updateContactPointEv
pub fn stub_723f98() -> ! {
    todo!("0x723f98 __ZN3RBX16GeoPairConnector18updateContactPointEv")
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::releaseMemory(void)")]
// 0x724018 — __ZN3RBX9AllocatorINS_16GeoPairConnectorEE13releaseMemoryEv
pub fn stub_724018() -> ! {
    todo!("0x724018 __ZN3RBX9AllocatorINS_16GeoPairConnectorEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::Allocator(void)")]
// 0x724064 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev
pub fn stub_724064() -> ! {
    todo!("0x724064 __ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::releaseMemory(void)")]
// 0x7240c8 — __ZN3RBX9AllocatorINS_18BallBlockConnectorEE13releaseMemoryEv
pub fn stub_7240c8() -> ! {
    todo!("0x7240c8 __ZN3RBX9AllocatorINS_18BallBlockConnectorEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::Allocator(void)")]
// 0x724114 — __ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev
pub fn stub_724114() -> ! {
    todo!("0x724114 __ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::releaseMemory(void)")]
// 0x724178 — __ZN3RBX9AllocatorINS_17BallBallConnectorEE13releaseMemoryEv
pub fn stub_724178() -> ! {
    todo!("0x724178 __ZN3RBX9AllocatorINS_17BallBallConnectorEE13releaseMemoryEv")
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7241c4 — __ZN3RBX10IPipelinedD2Ev
pub fn stub_7241c4() -> ! {
    todo!("0x7241c4 __ZN3RBX10IPipelinedD2Ev")
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x724234 — __ZN3RBX4EdgeD1Ev
pub fn stub_724234() -> ! {
    todo!("0x724234 __ZN3RBX4EdgeD1Ev")
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0x724238 — __ZN3RBX4EdgeD0Ev
pub fn stub_724238() -> ! {
    todo!("0x724238 __ZN3RBX4EdgeD0Ev")
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7242d8 — __ZN3RBX10IPipelinedD1Ev
pub fn stub_7242d8() -> ! {
    todo!("0x7242d8 __ZN3RBX10IPipelinedD1Ev")
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0x7242dc — __ZN3RBX10IPipelinedD0Ev
pub fn stub_7242dc() -> ! {
    todo!("0x7242dc __ZN3RBX10IPipelinedD0Ev")
}

#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
// 0x724650 — __ZN3RBX14ContactManagerC1EPNS_5WorldE
pub fn stub_724650() -> ! {
    todo!("0x724650 __ZN3RBX14ContactManagerC1EPNS_5WorldE")
}

#[doc(alias = "RBX::ContactManager::ContactManager(RBX::World *)")]
// 0x724654 — __ZN3RBX14ContactManagerC2EPNS_5WorldE
pub fn stub_724654() -> ! {
    todo!("0x724654 __ZN3RBX14ContactManagerC2EPNS_5WorldE")
}

#[doc(alias = "RBX::ContactManager::~ContactManager()")]
// 0x7247ec — __ZN3RBX14ContactManagerD1Ev
pub fn stub_7247ec() -> ! {
    todo!("0x7247ec __ZN3RBX14ContactManagerD1Ev")
}

#[doc(alias = "RBX::ContactManager::~ContactManager()")]
// 0x7247f0 — __ZN3RBX14ContactManagerD2Ev
pub fn stub_7247f0() -> ! {
    todo!("0x7247f0 __ZN3RBX14ContactManagerD2Ev")
}

#[doc(alias = "RBX::ContactManager::fastClear(void)")]
// 0x724920 — __ZN3RBX14ContactManager9fastClearEv
pub fn stub_724920() -> ! {
    todo!("0x724920 __ZN3RBX14ContactManager9fastClearEv")
}

#[doc(alias = "RBX::ContactManager::doStats(void)")]
// 0x724928 — __ZN3RBX14ContactManager7doStatsEv
pub fn stub_724928() -> ! {
    todo!("0x724928 __ZN3RBX14ContactManager7doStatsEv")
}

#[doc(alias = "RBX::ContactManager::intersectingMySimulation(RBX::Primitive *,RBX::SystemAddress,float)")]
// 0x72492c — __ZN3RBX14ContactManager24intersectingMySimulationEPNS_9PrimitiveENS_13SystemAddressEf
pub fn stub_72492c() -> ! {
    todo!("0x72492c __ZN3RBX14ContactManager24intersectingMySimulationEPNS_9PrimitiveENS_13SystemAddressEf")
}

#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&,float)")]
// 0x7249bc — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveERKSt3setIS2_St4lessIS2_ESaIS2_EEf
pub fn stub_7249bc() -> ! {
    todo!("0x7249bc __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveERKSt3setIS2_St4lessIS2_ESaIS2_EEf")
}

#[doc(alias = "RBX::ContactManager::createContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x724be4 — __ZN3RBX14ContactManager13createContactEPNS_9PrimitiveES2_
pub fn stub_724be4() -> ! {
    todo!("0x724be4 __ZN3RBX14ContactManager13createContactEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::ContactManager::intersectingOthers(RBX::Primitive *,float)")]
// 0x7250a4 — __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveEf
pub fn stub_7250a4() -> ! {
    todo!("0x7250a4 __ZN3RBX14ContactManager18intersectingOthersEPNS_9PrimitiveEf")
}

#[doc(alias = "RBX::ContactManager::terrainCellsInRegion3(RBX::Region3)const")]
// 0x725f1c — __ZNK3RBX14ContactManager21terrainCellsInRegion3ENS_7Region3E
pub fn stub_725f1c() -> ! {
    todo!("0x725f1c __ZNK3RBX14ContactManager21terrainCellsInRegion3ENS_7Region3E")
}

#[doc(alias = "RBX::ContactManager::onNewPair(RBX::Primitive *,RBX::Primitive *)")]
// 0x7262e0 — __ZN3RBX14ContactManager9onNewPairEPNS_9PrimitiveES2_
pub fn stub_7262e0() -> ! {
    todo!("0x7262e0 __ZN3RBX14ContactManager9onNewPairEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::ContactManager::releasePair(RBX::Primitive *,RBX::Primitive *)")]
// 0x726370 — __ZN3RBX14ContactManager11releasePairEPNS_9PrimitiveES2_
pub fn stub_726370() -> ! {
    todo!("0x726370 __ZN3RBX14ContactManager11releasePairEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::ContactManager::onPrimitiveAdded(RBX::Primitive *)")]
// 0x72641c — __ZN3RBX14ContactManager16onPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_72641c() -> ! {
    todo!("0x72641c __ZN3RBX14ContactManager16onPrimitiveAddedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactManager::onPrimitiveRemoved(RBX::Primitive *)")]
// 0x726524 — __ZN3RBX14ContactManager18onPrimitiveRemovedEPNS_9PrimitiveE
pub fn stub_726524() -> ! {
    todo!("0x726524 __ZN3RBX14ContactManager18onPrimitiveRemovedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactManager::onPrimitiveExtentsChanged(RBX::Primitive *)")]
// 0x726604 — __ZN3RBX14ContactManager25onPrimitiveExtentsChangedEPNS_9PrimitiveE
pub fn stub_726604() -> ! {
    todo!("0x726604 __ZN3RBX14ContactManager25onPrimitiveExtentsChangedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactManager::onPrimitiveGeometryChanged(RBX::Primitive *)")]
// 0x72660c — __ZN3RBX14ContactManager26onPrimitiveGeometryChangedEPNS_9PrimitiveE
pub fn stub_72660c() -> ! {
    todo!("0x72660c __ZN3RBX14ContactManager26onPrimitiveGeometryChangedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactManager::checkMegaClusterContact(RBX::Primitive *,bool,bool,bool)")]
// 0x72676c — __ZN3RBX14ContactManager23checkMegaClusterContactEPNS_9PrimitiveEbbb
pub fn stub_72676c() -> ! {
    todo!("0x72676c __ZN3RBX14ContactManager23checkMegaClusterContactEPNS_9PrimitiveEbbb")
}

#[doc(alias = "RBX::ContactManager::primitiveIsExcludedFromSpatialHash(RBX::Primitive *)")]
// 0x726cf0 — __ZN3RBX14ContactManager34primitiveIsExcludedFromSpatialHashEPNS_9PrimitiveE
pub fn stub_726cf0() -> ! {
    todo!("0x726cf0 __ZN3RBX14ContactManager34primitiveIsExcludedFromSpatialHashEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactManager::checkMegaClusterBigTerrainContact(RBX::Primitive *)")]
// 0x726d08 — __ZN3RBX14ContactManager33checkMegaClusterBigTerrainContactEPNS_9PrimitiveE
pub fn stub_726d08() -> ! {
    todo!("0x726d08 __ZN3RBX14ContactManager33checkMegaClusterBigTerrainContactEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactManager::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0x727054 — __ZN3RBX14ContactManager18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
pub fn stub_727054() -> ! {
    todo!("0x727054 __ZN3RBX14ContactManager18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::ContactManager::startLoadingTerrain(void)")]
// 0x727420 — __ZN3RBX14ContactManager19startLoadingTerrainEv
pub fn stub_727420() -> ! {
    todo!("0x727420 __ZN3RBX14ContactManager19startLoadingTerrainEv")
}

#[doc(alias = "RBX::ContactManager::doneLoadingTerrain(void)")]
// 0x727438 — __ZN3RBX14ContactManager18doneLoadingTerrainEv
pub fn stub_727438() -> ! {
    todo!("0x727438 __ZN3RBX14ContactManager18doneLoadingTerrainEv")
}

#[doc(alias = "RBX::ContactManager::applyDeferredTerrainChanges(void)")]
// 0x727578 — __ZN3RBX14ContactManager27applyDeferredTerrainChangesEv
pub fn stub_727578() -> ! {
    todo!("0x727578 __ZN3RBX14ContactManager27applyDeferredTerrainChangesEv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::fastClear(void)")]
// 0x727838 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv
pub fn stub_727838() -> ! {
    todo!("0x727838 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getNextGrid(RBX::Vector3int32 &,RBX::RbxRay const&,float)")]
// 0x727f74 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf
pub fn stub_727f74() -> ! {
    todo!("0x727f74 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf")
}

#[doc(alias = "RBX::Extents::overlapsOrTouches(RBX::Extents const&)const")]
// 0x728358 — __ZNK3RBX7Extents17overlapsOrTouchesERKS0_
pub fn stub_728358() -> ! {
    todo!("0x728358 __ZNK3RBX7Extents17overlapsOrTouchesERKS0_")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator new(unsigned long)")]
// 0x728438 — __ZN3RBX9AllocatorINS_15BallBallContactEEnwEm
pub fn stub_728438() -> ! {
    todo!("0x728438 __ZN3RBX9AllocatorINS_15BallBallContactEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator delete(void *)")]
// 0x7284a8 — __ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv
pub fn stub_7284a8() -> ! {
    todo!("0x7284a8 __ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator new(unsigned long)")]
// 0x7284e4 — __ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm
pub fn stub_7284e4() -> ! {
    todo!("0x7284e4 __ZN3RBX9AllocatorINS_16BallBlockContactEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::operator delete(void *)")]
// 0x728554 — __ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv
pub fn stub_728554() -> ! {
    todo!("0x728554 __ZN3RBX9AllocatorINS_16BallBlockContactEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator new(unsigned long)")]
// 0x728590 — __ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm
pub fn stub_728590() -> ! {
    todo!("0x728590 __ZN3RBX9AllocatorINS_15BallPolyContactEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator new(unsigned long)")]
// 0x728600 — __ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm
pub fn stub_728600() -> ! {
    todo!("0x728600 __ZN3RBX9AllocatorINS_17BlockBlockContactEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator new(unsigned long)")]
// 0x728670 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm
pub fn stub_728670() -> ! {
    todo!("0x728670 __ZN3RBX9AllocatorINS_15PolyPolyContactEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::operator delete(void *)")]
// 0x7286e0 — __ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv
pub fn stub_7286e0() -> ! {
    todo!("0x7286e0 __ZN3RBX9AllocatorINS_15PolyPolyContactEEdlEPv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(RBX::Primitive*,bool)")]
// 0x72871c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b
pub fn stub_72871c() -> ! {
    todo!("0x72871c __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(RBX::Primitive*)")]
// 0x7287b0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_
pub fn stub_7287b0() -> ! {
    todo!("0x7287b0 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_")
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator new(unsigned long)")]
// 0x72895c — __ZN3RBX9AllocatorINS_15BallCellContactEEnwEm
pub fn stub_72895c() -> ! {
    todo!("0x72895c __ZN3RBX9AllocatorINS_15BallCellContactEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator delete(void *)")]
// 0x7289cc — __ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv
pub fn stub_7289cc() -> ! {
    todo!("0x7289cc __ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator new(unsigned long)")]
// 0x728a08 — __ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm
pub fn stub_728a08() -> ! {
    todo!("0x728a08 __ZN3RBX9AllocatorINS_15PolyCellContactEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::operator delete(void *)")]
// 0x728a78 — __ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv
pub fn stub_728a78() -> ! {
    todo!("0x728a78 __ZN3RBX9AllocatorINS_15PolyCellContactEEdlEPv")
}

#[doc(alias = "RBX::Extents::clampToOverlap(RBX::Extents const&)")]
// 0x728ab4 — __ZN3RBX7Extents14clampToOverlapERKS0_
pub fn stub_728ab4() -> ! {
    todo!("0x728ab4 __ZN3RBX7Extents14clampToOverlapERKS0_")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::computeLevel(RBX::Primitive const*,RBX::Extents const&)")]
// 0x72a668 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE
pub fn stub_72a668() -> ! {
    todo!("0x72a668 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(RBX::Primitive*)")]
// 0x72a728 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_
pub fn stub_72a728() -> ! {
    todo!("0x72a728 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(RBX::Primitive*,bool)")]
// 0x72a844 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b
pub fn stub_72a844() -> ! {
    todo!("0x72a844 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(RBX::Primitive*,RBX::Extents const&)")]
// 0x72a990 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE
pub fn stub_72a990() -> ! {
    todo!("0x72a990 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE")
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
// 0x72ab58 — __ZN3RBX12ExtentsInt32C1Ev
pub fn stub_72ab58() -> ! {
    todo!("0x72ab58 __ZN3RBX12ExtentsInt32C1Ev")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::changeMinMax(RBX::Primitive*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
// 0x72ac08 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b
pub fn stub_72ac08() -> ! {
    todo!("0x72ac08 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b")
}
