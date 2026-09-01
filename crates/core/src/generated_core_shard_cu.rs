//! core shard CU — 100 core stubs EA-sorted, next uncovered after CT 0x719f5c (strict RBX|boost|std|rbx earliest gap).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Ball::getSurfaceVertInBody(unsigned long,int)const")]
// 0x719fb4 — __ZNK3RBX4Ball20getSurfaceVertInBodyEmi
pub fn stub_719fb4() -> ! {
    todo!("0x719fb4 __ZNK3RBX4Ball20getSurfaceVertInBodyEmi")
}

#[doc(alias = "RBX::Ball::getNumVertsInSurface(unsigned long)const")]
// 0x71a194 — __ZNK3RBX4Ball20getNumVertsInSurfaceEm
pub fn stub_71a194() -> ! {
    todo!("0x71a194 __ZNK3RBX4Ball20getNumVertsInSurfaceEm")
}

#[doc(alias = "RBX::Ball::getSurfaceCoordInBody(unsigned long)const")]
// 0x71a230 — __ZNK3RBX4Ball21getSurfaceCoordInBodyEm
pub fn stub_71a230() -> ! {
    todo!("0x71a230 __ZNK3RBX4Ball21getSurfaceCoordInBodyEm")
}

#[doc(alias = "RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x71a334 — __ZN3RBX15BallPolyContactC1EPNS_9PrimitiveES2_
pub fn stub_71a334() -> ! {
    todo!("0x71a334 __ZN3RBX15BallPolyContactC1EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BallPolyContact::BallPolyContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x71a338 — __ZN3RBX15BallPolyContactC2EPNS_9PrimitiveES2_
pub fn stub_71a338() -> ! {
    todo!("0x71a338 __ZN3RBX15BallPolyContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::BallPolyContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x71a4d0 — __ZN3RBX15BallPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_71a4d0() -> ! {
    todo!("0x71a4d0 __ZN3RBX15BallPolyContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::BallPolyContact::newBallPlaneConnector(RBX::POLY::Face const*)")]
// 0x71a7b0 — __ZN3RBX15BallPolyContact21newBallPlaneConnectorEPKNS_4POLY4FaceE
pub fn stub_71a7b0() -> ! {
    todo!("0x71a7b0 __ZN3RBX15BallPolyContact21newBallPlaneConnectorEPKNS_4POLY4FaceE")
}

#[doc(alias = "RBX::BallPolyContact::newBallEdgeConnector(RBX::POLY::Edge const*)")]
// 0x71aa04 — __ZN3RBX15BallPolyContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE
pub fn stub_71aa04() -> ! {
    todo!("0x71aa04 __ZN3RBX15BallPolyContact20newBallEdgeConnectorEPKNS_4POLY4EdgeE")
}

#[doc(alias = "RBX::BallPolyContact::newBallVertexConnector(RBX::POLY::Vertex const*)")]
// 0x71ad7c — __ZN3RBX15BallPolyContact22newBallVertexConnectorEPKNS_4POLY6VertexE
pub fn stub_71ad7c() -> ! {
    todo!("0x71ad7c __ZN3RBX15BallPolyContact22newBallVertexConnectorEPKNS_4POLY6VertexE")
}

#[doc(alias = "RBX::BallPolyContact::generateDataForMovingAssemblyStage(void)")]
// 0x71af10 — __ZN3RBX15BallPolyContact34generateDataForMovingAssemblyStageEv
pub fn stub_71af10() -> ! {
    todo!("0x71af10 __ZN3RBX15BallPolyContact34generateDataForMovingAssemblyStageEv")
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::Allocator(void)")]
// 0x71af14 — __ZN3RBX9AllocatorINS_15BallPolyContactEEC2Ev
pub fn stub_71af14() -> ! {
    todo!("0x71af14 __ZN3RBX9AllocatorINS_15BallPolyContactEEC2Ev")
}

#[doc(alias = "RBX::BallPolyContact::~BallPolyContact()")]
// 0x71af78 — __ZN3RBX15BallPolyContactD1Ev
pub fn stub_71af78() -> ! {
    todo!("0x71af78 __ZN3RBX15BallPolyContactD1Ev")
}

#[doc(alias = "RBX::BallPolyContact::~BallPolyContact()")]
// 0x71af7c — __ZN3RBX15BallPolyContactD0Ev
pub fn stub_71af7c() -> ! {
    todo!("0x71af7c __ZN3RBX15BallPolyContactD0Ev")
}

#[doc(alias = "RBX::PolyContact::numConnectors(void)const")]
// 0x71b030 — __ZNK3RBX11PolyContact13numConnectorsEv
pub fn stub_71b030() -> ! {
    todo!("0x71b030 __ZNK3RBX11PolyContact13numConnectorsEv")
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::releaseMemory(void)")]
// 0x71b038 — __ZN3RBX9AllocatorINS_15BallPolyContactEE13releaseMemoryEv
pub fn stub_71b038() -> ! {
    todo!("0x71b038 __ZN3RBX9AllocatorINS_15BallPolyContactEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator delete(void *)")]
// 0x71b084 — __ZN3RBX9AllocatorINS_15BallPolyContactEEdlEPv
pub fn stub_71b084() -> ! {
    todo!("0x71b084 __ZN3RBX9AllocatorINS_15BallPolyContactEEdlEPv")
}

#[doc(alias = "RBX::Block::init(void)")]
// 0x71b460 — __ZN3RBX5Block4initEv
pub fn stub_71b460() -> ! {
    todo!("0x71b460 __ZN3RBX5Block4initEv")
}

#[doc(alias = "RBX::Block::buildMesh(void)")]
// 0x71b4a8 — __ZN3RBX5Block9buildMeshEv
pub fn stub_71b4a8() -> ! {
    todo!("0x71b4a8 __ZN3RBX5Block9buildMeshEv")
}

#[doc(alias = "RBX::Block::getMomentHollow(float)const")]
// 0x71b72c — __ZNK3RBX5Block15getMomentHollowEf
pub fn stub_71b72c() -> ! {
    todo!("0x71b72c __ZNK3RBX5Block15getMomentHollowEf")
}

#[doc(alias = "RBX::Block::getVolume(void)const")]
// 0x71bb08 — __ZNK3RBX5Block9getVolumeEv
pub fn stub_71bb08() -> ! {
    todo!("0x71bb08 __ZNK3RBX5Block9getVolumeEv")
}

#[doc(alias = "RBX::Block::getSurfaceCoordInBody(unsigned long)const")]
// 0x71c050 — __ZNK3RBX5Block21getSurfaceCoordInBodyEm
pub fn stub_71c050() -> ! {
    todo!("0x71c050 __ZNK3RBX5Block21getSurfaceCoordInBodyEm")
}

#[doc(alias = "RBX::Block::~Block()")]
// 0x71c3f0 — __ZN3RBX5BlockD1Ev
pub fn stub_71c3f0() -> ! {
    todo!("0x71c3f0 __ZN3RBX5BlockD1Ev")
}

#[doc(alias = "RBX::Block::~Block()")]
// 0x71c3f4 — __ZN3RBX5BlockD0Ev
pub fn stub_71c3f4() -> ! {
    todo!("0x71c3f4 __ZN3RBX5BlockD0Ev")
}

#[doc(alias = "RBX::Block::getGeometryType(void)const")]
// 0x71c494 — __ZNK3RBX5Block15getGeometryTypeEv
pub fn stub_71c494() -> ! {
    todo!("0x71c494 __ZNK3RBX5Block15getGeometryTypeEv")
}

#[doc(alias = "RBX::Block::getCollideType(void)const")]
// 0x71c498 — __ZNK3RBX5Block14getCollideTypeEv
pub fn stub_71c498() -> ! {
    todo!("0x71c498 __ZNK3RBX5Block14getCollideTypeEv")
}

#[doc(alias = "RBX::Geometry::setGeometryParameter(std::string const&,int)")]
// 0x71c49c — __ZN3RBX8Geometry20setGeometryParameterERKSsi
pub fn stub_71c49c() -> ! {
    todo!("0x71c49c __ZN3RBX8Geometry20setGeometryParameterERKSsi")
}

#[doc(alias = "RBX::Geometry::getGeometryParameter(std::string const&)const")]
// 0x71c4f4 — __ZNK3RBX8Geometry20getGeometryParameterERKSs
pub fn stub_71c4f4() -> ! {
    todo!("0x71c4f4 __ZNK3RBX8Geometry20getGeometryParameterERKSs")
}

#[doc(alias = "RBX::Poly::getRadius(void)const")]
// 0x71c548 — __ZNK3RBX4Poly9getRadiusEv
pub fn stub_71c548() -> ! {
    todo!("0x71c548 __ZNK3RBX4Poly9getRadiusEv")
}

#[doc(alias = "RBX::Poly::getNumSurfaces(void)const")]
// 0x71c54c — __ZNK3RBX4Poly14getNumSurfacesEv
pub fn stub_71c54c() -> ! {
    todo!("0x71c54c __ZNK3RBX4Poly14getNumSurfacesEv")
}

#[doc(alias = "RBX::Geometry::getFaceFromLegacyNormalId(RBX::NormalId)const")]
// 0x71c564 — __ZNK3RBX8Geometry25getFaceFromLegacyNormalIdENS_8NormalIdE
pub fn stub_71c564() -> ! {
    todo!("0x71c564 __ZNK3RBX8Geometry25getFaceFromLegacyNormalIdENS_8NormalIdE")
}

#[doc(alias = "RBX::Geometry::isGeometryOrthogonal(void)const")]
// 0x71c568 — __ZNK3RBX8Geometry20isGeometryOrthogonalEv
pub fn stub_71c568() -> ! {
    todo!("0x71c568 __ZNK3RBX8Geometry20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::Block::getMoment(float)const")]
// 0x71c56c — __ZNK3RBX5Block9getMomentEf
pub fn stub_71c56c() -> ! {
    todo!("0x71c56c __ZNK3RBX5Block9getMomentEf")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator delete(void *)")]
// 0x71ca14 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEdlEPv
pub fn stub_71ca14() -> ! {
    todo!("0x71ca14 __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator new(unsigned long)")]
// 0x71d050 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEnwEm
pub fn stub_71d050() -> ! {
    todo!("0x71d050 __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::Allocator(void)")]
// 0x71d14c — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEC2Ev
pub fn stub_71d14c() -> ! {
    todo!("0x71d14c __ZN3RBX9AllocatorINS_4POLY12BlockCornersEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::releaseMemory(void)")]
// 0x71d1b0 — __ZN3RBX9AllocatorINS_4POLY12BlockCornersEE13releaseMemoryEv
pub fn stub_71d1b0() -> ! {
    todo!("0x71d1b0 __ZN3RBX9AllocatorINS_4POLY12BlockCornersEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator delete(void *)")]
// 0x71da2c — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEdlEPv
pub fn stub_71da2c() -> ! {
    todo!("0x71da2c __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEdlEPv")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::~vector()")]
// 0x71da68 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EED2Ev
pub fn stub_71da68() -> ! {
    todo!("0x71da68 __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EED2Ev")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::~vector()")]
// 0x71daa0 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EED2Ev
pub fn stub_71daa0() -> ! {
    todo!("0x71daa0 __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EED2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator new(unsigned long)")]
// 0x71e0b0 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEnwEm
pub fn stub_71e0b0() -> ! {
    todo!("0x71e0b0 __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::Allocator(void)")]
// 0x71e230 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEC2Ev
pub fn stub_71e230() -> ! {
    todo!("0x71e230 __ZN3RBX9AllocatorINS_4POLY9BlockMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::releaseMemory(void)")]
// 0x71e294 — __ZN3RBX9AllocatorINS_4POLY9BlockMeshEE13releaseMemoryEv
pub fn stub_71e294() -> ! {
    todo!("0x71e294 __ZN3RBX9AllocatorINS_4POLY9BlockMeshEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Block::~Block()")]
// 0x71e3bc — __ZN3RBX5BlockD2Ev
pub fn stub_71e3bc() -> ! {
    todo!("0x71e3bc __ZN3RBX5BlockD2Ev")
}

#[doc(alias = "RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *)")]
// 0x71e5cc — __ZN3RBX10CleanStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_71e5cc() -> ! {
    todo!("0x71e5cc __ZN3RBX10CleanStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::CleanStage::CleanStage(RBX::IStage *,RBX::World *)")]
// 0x71e5d0 — __ZN3RBX10CleanStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_71e5d0() -> ! {
    todo!("0x71e5d0 __ZN3RBX10CleanStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::CleanStage::onPrimitiveAdded(RBX::Primitive *)")]
// 0x71e6a4 — __ZN3RBX10CleanStage16onPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_71e6a4() -> ! {
    todo!("0x71e6a4 __ZN3RBX10CleanStage16onPrimitiveAddedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::CleanStage::onPrimitiveRemoving(RBX::Primitive *)")]
// 0x71e6c0 — __ZN3RBX10CleanStage19onPrimitiveRemovingEPNS_9PrimitiveE
pub fn stub_71e6c0() -> ! {
    todo!("0x71e6c0 __ZN3RBX10CleanStage19onPrimitiveRemovingEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::CleanStage::onJointPrimitiveNulling(RBX::Joint *,RBX::Primitive *)")]
// 0x71e6dc — __ZN3RBX10CleanStage23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE
pub fn stub_71e6dc() -> ! {
    todo!("0x71e6dc __ZN3RBX10CleanStage23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::CleanStage::onJointPrimitiveSet(RBX::Joint *,RBX::Primitive *)")]
// 0x71e7fc — __ZN3RBX10CleanStage19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE
pub fn stub_71e7fc() -> ! {
    todo!("0x71e7fc __ZN3RBX10CleanStage19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::CleanStage::onEdgeAdded(RBX::Edge *)")]
// 0x71e87c — __ZN3RBX10CleanStage11onEdgeAddedEPNS_4EdgeE
pub fn stub_71e87c() -> ! {
    todo!("0x71e87c __ZN3RBX10CleanStage11onEdgeAddedEPNS_4EdgeE")
}

#[doc(alias = "RBX::CleanStage::onEdgeRemoving(RBX::Edge *)")]
// 0x71e8b0 — __ZN3RBX10CleanStage14onEdgeRemovingEPNS_4EdgeE
pub fn stub_71e8b0() -> ! {
    todo!("0x71e8b0 __ZN3RBX10CleanStage14onEdgeRemovingEPNS_4EdgeE")
}

#[doc(alias = "RBX::IPipelined::inStage(RBX::IStage *)const")]
// 0x71e984 — __ZNK3RBX10IPipelined7inStageEPNS_6IStageE
pub fn stub_71e984() -> ! {
    todo!("0x71e984 __ZNK3RBX10IPipelined7inStageEPNS_6IStageE")
}

#[doc(alias = "RBX::CleanStage::~CleanStage()")]
// 0x71ea28 — __ZN3RBX10CleanStageD1Ev
pub fn stub_71ea28() -> ! {
    todo!("0x71ea28 __ZN3RBX10CleanStageD1Ev")
}

#[doc(alias = "RBX::CleanStage::~CleanStage()")]
// 0x71ea4c — __ZN3RBX10CleanStageD0Ev
pub fn stub_71ea4c() -> ! {
    todo!("0x71ea4c __ZN3RBX10CleanStageD0Ev")
}

#[doc(alias = "RBX::CleanStage::getStageType(void)const")]
// 0x71eb04 — __ZNK3RBX10CleanStage12getStageTypeEv
pub fn stub_71eb04() -> ! {
    todo!("0x71eb04 __ZNK3RBX10CleanStage12getStageTypeEv")
}

#[doc(alias = "RBX::Clump::Clump(void)")]
// 0x71ebd0 — __ZN3RBX5ClumpC1Ev
pub fn stub_71ebd0() -> ! {
    todo!("0x71ebd0 __ZN3RBX5ClumpC1Ev")
}

#[doc(alias = "RBX::Clump::~Clump()")]
// 0x71ebec — __ZN3RBX5ClumpD0Ev
pub fn stub_71ebec() -> ! {
    todo!("0x71ebec __ZN3RBX5ClumpD0Ev")
}

#[doc(alias = "RBX::Clump::~Clump()")]
// 0x71ec8c — __ZN3RBX5ClumpD1Ev
pub fn stub_71ec8c() -> ! {
    todo!("0x71ec8c __ZN3RBX5ClumpD1Ev")
}

#[doc(alias = "RBX::Clump::isClumpRootPrimitive(RBX::Primitive const*)")]
// 0x71ec9c — __ZN3RBX5Clump20isClumpRootPrimitiveEPKNS_9PrimitiveE
pub fn stub_71ec9c() -> ! {
    todo!("0x71ec9c __ZN3RBX5Clump20isClumpRootPrimitiveEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::Clump::getPrimitiveClump(RBX::Primitive *)")]
// 0x71ecac — __ZN3RBX5Clump17getPrimitiveClumpEPNS_9PrimitiveE
pub fn stub_71ecac() -> ! {
    todo!("0x71ecac __ZN3RBX5Clump17getPrimitiveClumpEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::Clump::getConstPrimitiveClump(RBX::Primitive const*)")]
// 0x71ecb4 — __ZN3RBX5Clump22getConstPrimitiveClumpEPKNS_9PrimitiveE
pub fn stub_71ecb4() -> ! {
    todo!("0x71ecb4 __ZN3RBX5Clump22getConstPrimitiveClumpEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::IndexedTree::onParentChanging(void)")]
// 0x71f024 — __ZN3RBX11IndexedTree16onParentChangingEv
pub fn stub_71f024() -> ! {
    todo!("0x71f024 __ZN3RBX11IndexedTree16onParentChangingEv")
}

#[doc(alias = "RBX::IndexedTree::onChildAdding(RBX::IndexedTree*)")]
// 0x71f028 — __ZN3RBX11IndexedTree13onChildAddingEPS0_
pub fn stub_71f028() -> ! {
    todo!("0x71f028 __ZN3RBX11IndexedTree13onChildAddingEPS0_")
}

#[doc(alias = "RBX::IndexedTree::onChildAdded(RBX::IndexedTree*)")]
// 0x71f02c — __ZN3RBX11IndexedTree12onChildAddedEPS0_
pub fn stub_71f02c() -> ! {
    todo!("0x71f02c __ZN3RBX11IndexedTree12onChildAddedEPS0_")
}

#[doc(alias = "RBX::IndexedTree::onChildRemoving(RBX::IndexedTree*)")]
// 0x71f030 — __ZN3RBX11IndexedTree15onChildRemovingEPS0_
pub fn stub_71f030() -> ! {
    todo!("0x71f030 __ZN3RBX11IndexedTree15onChildRemovingEPS0_")
}

#[doc(alias = "RBX::IndexedTree::onChildRemoved(RBX::IndexedTree*)")]
// 0x71f034 — __ZN3RBX11IndexedTree14onChildRemovedEPS0_
pub fn stub_71f034() -> ! {
    todo!("0x71f034 __ZN3RBX11IndexedTree14onChildRemovedEPS0_")
}

#[doc(alias = "RBX::IndexedTree::onAncestorChanged(void)")]
// 0x71f038 — __ZN3RBX11IndexedTree17onAncestorChangedEv
pub fn stub_71f038() -> ! {
    todo!("0x71f038 __ZN3RBX11IndexedTree17onAncestorChangedEv")
}

#[doc(alias = "RBX::IndexedMesh::onLowersChanged(void)")]
// 0x71f03c — __ZN3RBX11IndexedMesh15onLowersChangedEv
pub fn stub_71f03c() -> ! {
    todo!("0x71f03c __ZN3RBX11IndexedMesh15onLowersChangedEv")
}

#[doc(alias = "RBX::BlockBlockContact::pairHitRatio(void)")]
// 0x71f648 — __ZN3RBX17BlockBlockContact12pairHitRatioEv
pub fn stub_71f648() -> ! {
    todo!("0x71f648 __ZN3RBX17BlockBlockContact12pairHitRatioEv")
}

#[doc(alias = "RBX::BlockBlockContact::featureHitRatio(void)")]
// 0x71f684 — __ZN3RBX17BlockBlockContact15featureHitRatioEv
pub fn stub_71f684() -> ! {
    todo!("0x71f684 __ZN3RBX17BlockBlockContact15featureHitRatioEv")
}

#[doc(alias = "RBX::Contact::getBody(int)")]
// 0x71f6c0 — __ZN3RBX7Contact7getBodyEi
pub fn stub_71f6c0() -> ! {
    todo!("0x71f6c0 __ZN3RBX7Contact7getBodyEi")
}

#[doc(alias = "RBX::Contact::Contact(RBX::Primitive *,RBX::Primitive *)")]
// 0x71f6cc — __ZN3RBX7ContactC2EPNS_9PrimitiveES2_
pub fn stub_71f6cc() -> ! {
    todo!("0x71f6cc __ZN3RBX7ContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::Contact::~Contact()")]
// 0x71f6fc — __ZN3RBX7ContactD0Ev
pub fn stub_71f6fc() -> ! {
    todo!("0x71f6fc __ZN3RBX7ContactD0Ev")
}

#[doc(alias = "RBX::Contact::~Contact()")]
// 0x71f79c — __ZN3RBX7ContactD1Ev
pub fn stub_71f79c() -> ! {
    todo!("0x71f79c __ZN3RBX7ContactD1Ev")
}

#[doc(alias = "RBX::Contact::~Contact()")]
// 0x71f7a0 — __ZN3RBX7ContactD2Ev
pub fn stub_71f7a0() -> ! {
    todo!("0x71f7a0 __ZN3RBX7ContactD2Ev")
}

#[doc(alias = "RBX::Contact::primitiveMovedExternally(void)")]
// 0x71f890 — __ZN3RBX7Contact24primitiveMovedExternallyEv
pub fn stub_71f890() -> ! {
    todo!("0x71f890 __ZN3RBX7Contact24primitiveMovedExternallyEv")
}

#[doc(alias = "RBX::Contact::step(int)")]
// 0x71f8d4 — __ZN3RBX7Contact4stepEi
pub fn stub_71f8d4() -> ! {
    todo!("0x71f8d4 __ZN3RBX7Contact4stepEi")
}

#[doc(alias = "RBX::Contact::computeIsAdjacentUi(float)")]
// 0x71f9e4 — __ZN3RBX7Contact19computeIsAdjacentUiEf
pub fn stub_71f9e4() -> ! {
    todo!("0x71f9e4 __ZN3RBX7Contact19computeIsAdjacentUiEf")
}

#[doc(alias = "RBX::Contact::computeIsCollidingUi(float)")]
// 0x71fa14 — __ZN3RBX7Contact20computeIsCollidingUiEf
pub fn stub_71fa14() -> ! {
    todo!("0x71fa14 __ZN3RBX7Contact20computeIsCollidingUiEf")
}

#[doc(alias = "RBX::calculateFriction(float,float)")]
// 0x71fa34 — __ZN3RBX17calculateFrictionEff
pub fn stub_71fa34() -> ! {
    todo!("0x71fa34 __ZN3RBX17calculateFrictionEff")
}

#[doc(alias = "RBX::Contact::onPrimitiveContactParametersChanged(void)")]
// 0x71fac4 — __ZN3RBX7Contact35onPrimitiveContactParametersChangedEv
pub fn stub_71fac4() -> ! {
    todo!("0x71fac4 __ZN3RBX7Contact35onPrimitiveContactParametersChangedEv")
}

#[doc(alias = "RBX::Contact::deleteConnector(RBX::ContactConnector *)")]
// 0x71fbb8 — __ZN3RBX7Contact15deleteConnectorEPNS_16ContactConnectorE
pub fn stub_71fbb8() -> ! {
    todo!("0x71fbb8 __ZN3RBX7Contact15deleteConnectorEPNS_16ContactConnectorE")
}

#[doc(alias = "RBX::Contact::generateDataForMovingAssemblyStage(void)")]
// 0x71fbdc — __ZN3RBX7Contact34generateDataForMovingAssemblyStageEv
pub fn stub_71fbdc() -> ! {
    todo!("0x71fbdc __ZN3RBX7Contact34generateDataForMovingAssemblyStageEv")
}

#[doc(alias = "RBX::BallBallContact::getConnector(int)")]
// 0x71fc04 — __ZN3RBX15BallBallContact12getConnectorEi
pub fn stub_71fc04() -> ! {
    todo!("0x71fc04 __ZN3RBX15BallBallContact12getConnectorEi")
}

#[doc(alias = "RBX::BallBallContact::deleteAllConnectors(void)")]
// 0x71fc08 — __ZN3RBX15BallBallContact19deleteAllConnectorsEv
pub fn stub_71fc08() -> ! {
    todo!("0x71fc08 __ZN3RBX15BallBallContact19deleteAllConnectorsEv")
}

#[doc(alias = "RBX::BallBallContact::computeIsColliding(float)")]
// 0x71fc24 — __ZN3RBX15BallBallContact18computeIsCollidingEf
pub fn stub_71fc24() -> ! {
    todo!("0x71fc24 __ZN3RBX15BallBallContact18computeIsCollidingEf")
}

#[doc(alias = "RBX::BallBallContact::stepContact(void)")]
// 0x71fcfc — __ZN3RBX15BallBallContact11stepContactEv
pub fn stub_71fcfc() -> ! {
    todo!("0x71fcfc __ZN3RBX15BallBallContact11stepContactEv")
}

#[doc(alias = "RBX::BallBallContact::generateDataForMovingAssemblyStage(void)")]
// 0x71fec4 — __ZN3RBX15BallBallContact34generateDataForMovingAssemblyStageEv
pub fn stub_71fec4() -> ! {
    todo!("0x71fec4 __ZN3RBX15BallBallContact34generateDataForMovingAssemblyStageEv")
}

#[doc(alias = "RBX::BallBlockContact::getConnector(int)")]
// 0x71fec8 — __ZN3RBX16BallBlockContact12getConnectorEi
pub fn stub_71fec8() -> ! {
    todo!("0x71fec8 __ZN3RBX16BallBlockContact12getConnectorEi")
}

#[doc(alias = "RBX::BallBlockContact::deleteAllConnectors(void)")]
// 0x71fecc — __ZN3RBX16BallBlockContact19deleteAllConnectorsEv
pub fn stub_71fecc() -> ! {
    todo!("0x71fecc __ZN3RBX16BallBlockContact19deleteAllConnectorsEv")
}

#[doc(alias = "RBX::BallBlockContact::computeIsColliding(float)")]
// 0x71fee8 — __ZN3RBX16BallBlockContact18computeIsCollidingEf
pub fn stub_71fee8() -> ! {
    todo!("0x71fee8 __ZN3RBX16BallBlockContact18computeIsCollidingEf")
}

#[doc(alias = "RBX::BallBlockContact::stepContact(void)")]
// 0x7200f8 — __ZN3RBX16BallBlockContact11stepContactEv
pub fn stub_7200f8() -> ! {
    todo!("0x7200f8 __ZN3RBX16BallBlockContact11stepContactEv")
}

#[doc(alias = "RBX::BallBlockContact::generateDataForMovingAssemblyStage(void)")]
// 0x72034c — __ZN3RBX16BallBlockContact34generateDataForMovingAssemblyStageEv
pub fn stub_72034c() -> ! {
    todo!("0x72034c __ZN3RBX16BallBlockContact34generateDataForMovingAssemblyStageEv")
}

#[doc(alias = "RBX::BlockBlockContact::getConnector(int)")]
// 0x720354 — __ZN3RBX17BlockBlockContact12getConnectorEi
pub fn stub_720354() -> ! {
    todo!("0x720354 __ZN3RBX17BlockBlockContact12getConnectorEi")
}

#[doc(alias = "RBX::BlockBlockContact::deleteAllConnectorsOrig(void)")]
// 0x720388 — __ZN3RBX17BlockBlockContact23deleteAllConnectorsOrigEv
pub fn stub_720388() -> ! {
    todo!("0x720388 __ZN3RBX17BlockBlockContact23deleteAllConnectorsOrigEv")
}

#[doc(alias = "RBX::BlockBlockContact::deleteAllConnectorsFFlag(void)")]
// 0x720414 — __ZN3RBX17BlockBlockContact24deleteAllConnectorsFFlagEv
pub fn stub_720414() -> ! {
    todo!("0x720414 __ZN3RBX17BlockBlockContact24deleteAllConnectorsFFlagEv")
}

#[doc(alias = "RBX::BlockBlockContact::findGeoPairConnector(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0x7204b8 — __ZN3RBX17BlockBlockContact20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_7204b8() -> ! {
    todo!("0x7204b8 __ZN3RBX17BlockBlockContact20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii")
}

#[doc(alias = "RBX::BlockBlockContactData::findGeoPairConnector(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0x720548 — __ZN3RBX21BlockBlockContactData20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_720548() -> ! {
    todo!("0x720548 __ZN3RBX21BlockBlockContactData20findGeoPairConnectorEPNS_4BodyES2_NS_11GeoPairTypeEii")
}

#[doc(alias = "RBX::BlockBlockContact::findGeoPairConnectorFFlag(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0x720734 — __ZN3RBX17BlockBlockContact25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_720734() -> ! {
    todo!("0x720734 __ZN3RBX17BlockBlockContact25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii")
}

#[doc(alias = "RBX::BlockBlockContactData::findGeoPairConnectorFFlag(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0x7207c4 — __ZN3RBX21BlockBlockContactData25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_7207c4() -> ! {
    todo!("0x7207c4 __ZN3RBX21BlockBlockContactData25findGeoPairConnectorFFlagEPNS_4BodyES2_NS_11GeoPairTypeEii")
}
