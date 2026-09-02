//! core shard CX — 100 core stubs EA-sorted, next uncovered after CW 0x7312d0 (strict RBX|boost|std|rbx earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::CornerWedgePoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
// 0x731338 — __ZNK3RBX15CornerWedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE
pub fn stub_731338() -> ! {
    todo!("0x731338 __ZNK3RBX15CornerWedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE")
}

#[doc(alias = "RBX::CornerWedgePoly::~CornerWedgePoly()")]
// 0x731640 — __ZN3RBX15CornerWedgePolyD1Ev
pub fn stub_731640() -> ! {
    todo!("0x731640 __ZN3RBX15CornerWedgePolyD1Ev")
}

#[doc(alias = "RBX::CornerWedgePoly::~CornerWedgePoly()")]
// 0x731664 — __ZN3RBX15CornerWedgePolyD0Ev
pub fn stub_731664() -> ! {
    todo!("0x731664 __ZN3RBX15CornerWedgePolyD0Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator delete(void *)")]
// 0x731ca8 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv
pub fn stub_731ca8() -> ! {
    todo!("0x731ca8 __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator new(unsigned long)")]
// 0x7322e4 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm
pub fn stub_7322e4() -> ! {
    todo!("0x7322e4 __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::Allocator(void)")]
// 0x732470 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev
pub fn stub_732470() -> ! {
    todo!("0x732470 __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::releaseMemory(void)")]
// 0x7324d4 — __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEE13releaseMemoryEv
pub fn stub_7324d4() -> ! {
    todo!("0x7324d4 __ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Edge::Edge(RBX::Primitive *,RBX::Primitive *)")]
// 0x7328f0 — __ZN3RBX4EdgeC2EPNS_9PrimitiveES2_
pub fn stub_7328f0() -> ! {
    todo!("0x7328f0 __ZN3RBX4EdgeC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::Edge::setPrimitive(int,RBX::Primitive *)")]
// 0x732928 — __ZN3RBX4Edge12setPrimitiveEiPNS_9PrimitiveE
pub fn stub_732928() -> ! {
    todo!("0x732928 __ZN3RBX4Edge12setPrimitiveEiPNS_9PrimitiveE")
}

#[doc(alias = "RBX::EdgeBuffer::~EdgeBuffer()")]
// 0x7329d0 — __ZN3RBX10EdgeBufferD0Ev
pub fn stub_7329d0() -> ! {
    todo!("0x7329d0 __ZN3RBX10EdgeBufferD0Ev")
}

#[doc(alias = "RBX::EdgeBuffer::~EdgeBuffer()")]
// 0x732a70 — __ZN3RBX10EdgeBufferD1Ev
pub fn stub_732a70() -> ! {
    todo!("0x732a70 __ZN3RBX10EdgeBufferD1Ev")
}

#[doc(alias = "RBX::EdgeBuffer::~EdgeBuffer()")]
// 0x732a74 — __ZN3RBX10EdgeBufferD2Ev
pub fn stub_732a74() -> ! {
    todo!("0x732a74 __ZN3RBX10EdgeBufferD2Ev")
}

#[doc(alias = "RBX::EdgeBuffer::afterAssemblyAdded(RBX::Assembly *)")]
// 0x732bd8 — __ZN3RBX10EdgeBuffer18afterAssemblyAddedEPNS_8AssemblyE
pub fn stub_732bd8() -> ! {
    todo!("0x732bd8 __ZN3RBX10EdgeBuffer18afterAssemblyAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::EdgeBuffer::assemblyPrimitiveAdded(RBX::Primitive *)")]
// 0x732c58 — __ZN3RBX10EdgeBuffer22assemblyPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_732c58() -> ! {
    todo!("0x732c58 __ZN3RBX10EdgeBuffer22assemblyPrimitiveAddedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::EdgeBuffer::beforeAssemblyRemoving(RBX::Assembly *)")]
// 0x732cd8 — __ZN3RBX10EdgeBuffer22beforeAssemblyRemovingEPNS_8AssemblyE
pub fn stub_732cd8() -> ! {
    todo!("0x732cd8 __ZN3RBX10EdgeBuffer22beforeAssemblyRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::EdgeBuffer::assemblyPrimitiveRemoved(RBX::Primitive *)")]
// 0x732d58 — __ZN3RBX10EdgeBuffer24assemblyPrimitiveRemovedEPNS_9PrimitiveE
pub fn stub_732d58() -> ! {
    todo!("0x732d58 __ZN3RBX10EdgeBuffer24assemblyPrimitiveRemovedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::EdgeBuffer::pushEdgeIfOk(RBX::Edge *)")]
// 0x732dd4 — __ZN3RBX10EdgeBuffer12pushEdgeIfOkEPNS_4EdgeE
pub fn stub_732dd4() -> ! {
    todo!("0x732dd4 __ZN3RBX10EdgeBuffer12pushEdgeIfOkEPNS_4EdgeE")
}

#[doc(alias = "RBX::EdgeBuffer::pushKinematicOk(RBX::Edge *)")]
// 0x732ed4 — __ZN3RBX10EdgeBuffer15pushKinematicOkEPNS_4EdgeE
pub fn stub_732ed4() -> ! {
    todo!("0x732ed4 __ZN3RBX10EdgeBuffer15pushKinematicOkEPNS_4EdgeE")
}

#[doc(alias = "RBX::EdgeBuffer::pushSpringOk(RBX::Edge *)")]
// 0x732fbc — __ZN3RBX10EdgeBuffer12pushSpringOkEPNS_4EdgeE
pub fn stub_732fbc() -> ! {
    todo!("0x732fbc __ZN3RBX10EdgeBuffer12pushSpringOkEPNS_4EdgeE")
}

#[doc(alias = "RBX::EdgeBuffer::onEdgeAdded(RBX::Edge *)")]
// 0x733084 — __ZN3RBX10EdgeBuffer11onEdgeAddedEPNS_4EdgeE
pub fn stub_733084() -> ! {
    todo!("0x733084 __ZN3RBX10EdgeBuffer11onEdgeAddedEPNS_4EdgeE")
}

#[doc(alias = "RBX::EdgeBuffer::onEdgeRemoving(RBX::Edge *)")]
// 0x733168 — __ZN3RBX10EdgeBuffer14onEdgeRemovingEPNS_4EdgeE
pub fn stub_733168() -> ! {
    todo!("0x733168 __ZN3RBX10EdgeBuffer14onEdgeRemovingEPNS_4EdgeE")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,RBX::Edge *>,std::_Select1st<std::pair<RBX::Assembly * const,RBX::Edge *>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,RBX::Edge *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,RBX::Edge *>> *)")]
// 0x733260 — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_PNS0_4EdgeEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_733260() -> ! {
    todo!("0x733260 __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_PNS0_4EdgeEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "RBX::EdgeStage::EdgeStage(RBX::IStage *,RBX::World *)")]
// 0x733488 — __ZN3RBX9EdgeStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_733488() -> ! {
    todo!("0x733488 __ZN3RBX9EdgeStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::EdgeStage::EdgeStage(RBX::IStage *,RBX::World *)")]
// 0x73348c — __ZN3RBX9EdgeStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_73348c() -> ! {
    todo!("0x73348c __ZN3RBX9EdgeStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::EdgeStage::onPrimitiveAdded(RBX::Primitive *)")]
// 0x733560 — __ZN3RBX9EdgeStage16onPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_733560() -> ! {
    todo!("0x733560 __ZN3RBX9EdgeStage16onPrimitiveAddedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::EdgeStage::onPrimitiveRemoving(RBX::Primitive *)")]
// 0x73357c — __ZN3RBX9EdgeStage19onPrimitiveRemovingEPNS_9PrimitiveE
pub fn stub_73357c() -> ! {
    todo!("0x73357c __ZN3RBX9EdgeStage19onPrimitiveRemovingEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::EdgeStage::onEdgeAdded(RBX::Edge *)")]
// 0x733598 — __ZN3RBX9EdgeStage11onEdgeAddedEPNS_4EdgeE
pub fn stub_733598() -> ! {
    todo!("0x733598 __ZN3RBX9EdgeStage11onEdgeAddedEPNS_4EdgeE")
}

#[doc(alias = "RBX::EdgeStage::onEdgeRemoving(RBX::Edge *)")]
// 0x7335b4 — __ZN3RBX9EdgeStage14onEdgeRemovingEPNS_4EdgeE
pub fn stub_7335b4() -> ! {
    todo!("0x7335b4 __ZN3RBX9EdgeStage14onEdgeRemovingEPNS_4EdgeE")
}

#[doc(alias = "RBX::EdgeStage::~EdgeStage()")]
// 0x7335c8 — __ZN3RBX9EdgeStageD1Ev
pub fn stub_7335c8() -> ! {
    todo!("0x7335c8 __ZN3RBX9EdgeStageD1Ev")
}

#[doc(alias = "RBX::EdgeStage::~EdgeStage()")]
// 0x7335ec — __ZN3RBX9EdgeStageD0Ev
pub fn stub_7335ec() -> ! {
    todo!("0x7335ec __ZN3RBX9EdgeStageD0Ev")
}

#[doc(alias = "RBX::EdgeStage::getStageType(void)const")]
// 0x7336a4 — __ZNK3RBX9EdgeStage12getStageTypeEv
pub fn stub_7336a4() -> ! {
    todo!("0x7336a4 __ZNK3RBX9EdgeStage12getStageTypeEv")
}

#[doc(alias = "RBX::GlueJoint::GlueJoint(void)")]
// 0x733770 — __ZN3RBX9GlueJointC1Ev
pub fn stub_733770() -> ! {
    todo!("0x733770 __ZN3RBX9GlueJointC1Ev")
}

#[doc(alias = "RBX::GlueJoint::GlueJoint(void)")]
// 0x7337b0 — __ZN3RBX9GlueJointC2Ev
pub fn stub_7337b0() -> ! {
    todo!("0x7337b0 __ZN3RBX9GlueJointC2Ev")
}

#[doc(alias = "RBX::GlueJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
// 0x7337f0 — __ZN3RBX9GlueJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
pub fn stub_7337f0() -> ! {
    todo!("0x7337f0 __ZN3RBX9GlueJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")
}

#[doc(alias = "RBX::GlueJoint::getMaxForce(void)")]
// 0x733c38 — __ZN3RBX9GlueJoint11getMaxForceEv
pub fn stub_733c38() -> ! {
    todo!("0x733c38 __ZN3RBX9GlueJoint11getMaxForceEv")
}

#[doc(alias = "RBX::GlueJoint::putInKernel(RBX::Kernel *)")]
// 0x733c58 — __ZN3RBX9GlueJoint11putInKernelEPNS_6KernelE
pub fn stub_733c58() -> ! {
    todo!("0x733c58 __ZN3RBX9GlueJoint11putInKernelEPNS_6KernelE")
}

#[doc(alias = "RBX::ManualGlueJoint::putInKernel(RBX::Kernel *)")]
// 0x733e7c — __ZN3RBX15ManualGlueJoint11putInKernelEPNS_6KernelE
pub fn stub_733e7c() -> ! {
    todo!("0x733e7c __ZN3RBX15ManualGlueJoint11putInKernelEPNS_6KernelE")
}

#[doc(alias = "RBX::ManualGlueJoint::computeIntersectingSurfacePoints(void)")]
// 0x733e94 — __ZN3RBX15ManualGlueJoint32computeIntersectingSurfacePointsEv
pub fn stub_733e94() -> ! {
    todo!("0x733e94 __ZN3RBX15ManualGlueJoint32computeIntersectingSurfacePointsEv")
}

#[doc(alias = "RBX::Face::size(void)const")]
// 0x7343c4 — __ZNK3RBX4Face4sizeEv
pub fn stub_7343c4() -> ! {
    todo!("0x7343c4 __ZNK3RBX4Face4sizeEv")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::operator new(unsigned long)")]
// 0x73443c — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEnwEm
pub fn stub_73443c() -> ! {
    todo!("0x73443c __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEnwEm")
}

#[doc(alias = "RBX::GlueJoint::~GlueJoint()")]
// 0x734524 — __ZN3RBX9GlueJointD1Ev
pub fn stub_734524() -> ! {
    todo!("0x734524 __ZN3RBX9GlueJointD1Ev")
}

#[doc(alias = "RBX::GlueJoint::~GlueJoint()")]
// 0x734528 — __ZN3RBX9GlueJointD0Ev
pub fn stub_734528() -> ! {
    todo!("0x734528 __ZN3RBX9GlueJointD0Ev")
}

#[doc(alias = "RBX::GlueJoint::getJointType(void)const")]
// 0x7345c8 — __ZNK3RBX9GlueJoint12getJointTypeEv
pub fn stub_7345c8() -> ! {
    todo!("0x7345c8 __ZNK3RBX9GlueJoint12getJointTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::GlueJoint::~GlueJoint()")]
// 0x7345cc — __ZThn32_N3RBX9GlueJointD1Ev
pub fn stub_7345cc() -> ! {
    todo!("0x7345cc __ZThn32_N3RBX9GlueJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GlueJoint::~GlueJoint()")]
// 0x7345d4 — __ZThn32_N3RBX9GlueJointD0Ev
pub fn stub_7345d4() -> ! {
    todo!("0x7345d4 __ZThn32_N3RBX9GlueJointD0Ev")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::Allocator(void)")]
// 0x7346b0 — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEC2Ev
pub fn stub_7346b0() -> ! {
    todo!("0x7346b0 __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::releaseMemory(void)")]
// 0x734714 — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEE13releaseMemoryEv
pub fn stub_734714() -> ! {
    todo!("0x734714 __ZN3RBX9AllocatorINS_20NormalBreakConnectorEE13releaseMemoryEv")
}

#[doc(alias = "RBX::GroundStage::GroundStage(RBX::IStage *,RBX::World *)")]
// 0x73492c — __ZN3RBX11GroundStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_73492c() -> ! {
    todo!("0x73492c __ZN3RBX11GroundStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::GroundStage::GroundStage(RBX::IStage *,RBX::World *)")]
// 0x734930 — __ZN3RBX11GroundStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_734930() -> ! {
    todo!("0x734930 __ZN3RBX11GroundStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::GroundStage::~GroundStage()")]
// 0x734a04 — __ZN3RBX11GroundStageD0Ev
pub fn stub_734a04() -> ! {
    todo!("0x734a04 __ZN3RBX11GroundStageD0Ev")
}

#[doc(alias = "RBX::GroundStage::~GroundStage()")]
// 0x734abc — __ZN3RBX11GroundStageD1Ev
pub fn stub_734abc() -> ! {
    todo!("0x734abc __ZN3RBX11GroundStageD1Ev")
}

#[doc(alias = "RBX::GroundStage::onPrimitiveAdded(RBX::Primitive *)")]
// 0x734ae0 — __ZN3RBX11GroundStage16onPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_734ae0() -> ! {
    todo!("0x734ae0 __ZN3RBX11GroundStage16onPrimitiveAddedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::GroundStage::addGroundJoint(RBX::Primitive *,bool)")]
// 0x734b70 — __ZN3RBX11GroundStage14addGroundJointEPNS_9PrimitiveEb
pub fn stub_734b70() -> ! {
    todo!("0x734b70 __ZN3RBX11GroundStage14addGroundJointEPNS_9PrimitiveEb")
}

#[doc(alias = "RBX::GroundStage::onPrimitiveRemoving(RBX::Primitive *)")]
// 0x734d14 — __ZN3RBX11GroundStage19onPrimitiveRemovingEPNS_9PrimitiveE
pub fn stub_734d14() -> ! {
    todo!("0x734d14 __ZN3RBX11GroundStage19onPrimitiveRemovingEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::GroundStage::removeGroundJoint(RBX::Primitive *,bool)")]
// 0x734df4 — __ZN3RBX11GroundStage17removeGroundJointEPNS_9PrimitiveEb
pub fn stub_734df4() -> ! {
    todo!("0x734df4 __ZN3RBX11GroundStage17removeGroundJointEPNS_9PrimitiveEb")
}

#[doc(alias = "RBX::GroundStage::onPrimitiveFixedChanging(RBX::Primitive *)")]
// 0x734f30 — __ZN3RBX11GroundStage24onPrimitiveFixedChangingEPNS_9PrimitiveE
pub fn stub_734f30() -> ! {
    todo!("0x734f30 __ZN3RBX11GroundStage24onPrimitiveFixedChangingEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::GroundStage::onPrimitiveFixedChanged(RBX::Primitive *)")]
// 0x734f60 — __ZN3RBX11GroundStage23onPrimitiveFixedChangedEPNS_9PrimitiveE
pub fn stub_734f60() -> ! {
    todo!("0x734f60 __ZN3RBX11GroundStage23onPrimitiveFixedChangedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::GroundStage::rebuildFreeGround(RBX::Primitive *)")]
// 0x734fa4 — __ZN3RBX11GroundStage17rebuildFreeGroundEPNS_9PrimitiveE
pub fn stub_734fa4() -> ! {
    todo!("0x734fa4 __ZN3RBX11GroundStage17rebuildFreeGroundEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::GroundStage::rebuildOthers(RBX::Primitive *)")]
// 0x73506c — __ZN3RBX11GroundStage13rebuildOthersEPNS_9PrimitiveE
pub fn stub_73506c() -> ! {
    todo!("0x73506c __ZN3RBX11GroundStage13rebuildOthersEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::GroundStage::onEdgeAdded(RBX::Edge *)")]
// 0x73509c — __ZN3RBX11GroundStage11onEdgeAddedEPNS_4EdgeE
pub fn stub_73509c() -> ! {
    todo!("0x73509c __ZN3RBX11GroundStage11onEdgeAddedEPNS_4EdgeE")
}

#[doc(alias = "RBX::GroundStage::onKernelJointAdded(RBX::KernelJoint *)")]
// 0x7350fc — __ZN3RBX11GroundStage18onKernelJointAddedEPNS_11KernelJointE
pub fn stub_7350fc() -> ! {
    todo!("0x7350fc __ZN3RBX11GroundStage18onKernelJointAddedEPNS_11KernelJointE")
}

#[doc(alias = "RBX::GroundStage::checkForFreeGroundJoint(RBX::RigidJoint *)")]
// 0x7351ec — __ZN3RBX11GroundStage23checkForFreeGroundJointEPNS_10RigidJointE
pub fn stub_7351ec() -> ! {
    todo!("0x7351ec __ZN3RBX11GroundStage23checkForFreeGroundJointEPNS_10RigidJointE")
}

#[doc(alias = "RBX::GroundStage::onEdgeRemoving(RBX::Edge *)")]
// 0x7352e8 — __ZN3RBX11GroundStage14onEdgeRemovingEPNS_4EdgeE
pub fn stub_7352e8() -> ! {
    todo!("0x7352e8 __ZN3RBX11GroundStage14onEdgeRemovingEPNS_4EdgeE")
}

#[doc(alias = "RBX::GroundStage::onKernelJointRemoving(RBX::KernelJoint *)")]
// 0x735348 — __ZN3RBX11GroundStage21onKernelJointRemovingEPNS_11KernelJointE
pub fn stub_735348() -> ! {
    todo!("0x735348 __ZN3RBX11GroundStage21onKernelJointRemovingEPNS_11KernelJointE")
}

#[doc(alias = "RBX::GroundStage::heaviestRigidToGround(RBX::Primitive *)")]
// 0x735438 — __ZN3RBX11GroundStage21heaviestRigidToGroundEPNS_9PrimitiveE
pub fn stub_735438() -> ! {
    todo!("0x735438 __ZN3RBX11GroundStage21heaviestRigidToGroundEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::GroundStage::getStageType(void)const")]
// 0x7354ec — __ZNK3RBX11GroundStage12getStageTypeEv
pub fn stub_7354ec() -> ! {
    todo!("0x7354ec __ZNK3RBX11GroundStage12getStageTypeEv")
}

#[doc(alias = "RBX::FreeJoint::FreeJoint(RBX::Primitive *)")]
// 0x7354f0 — __ZN3RBX9FreeJointC2EPNS_9PrimitiveE
pub fn stub_7354f0() -> ! {
    todo!("0x7354f0 __ZN3RBX9FreeJointC2EPNS_9PrimitiveE")
}

#[doc(alias = "RBX::FreeJoint::~FreeJoint()")]
// 0x7355bc — __ZN3RBX9FreeJointD1Ev
pub fn stub_7355bc() -> ! {
    todo!("0x7355bc __ZN3RBX9FreeJointD1Ev")
}

#[doc(alias = "RBX::FreeJoint::~FreeJoint()")]
// 0x7355c0 — __ZN3RBX9FreeJointD0Ev
pub fn stub_7355c0() -> ! {
    todo!("0x7355c0 __ZN3RBX9FreeJointD0Ev")
}

#[doc(alias = "RBX::FreeJoint::getJointType(void)const")]
// 0x735660 — __ZNK3RBX9FreeJoint12getJointTypeEv
pub fn stub_735660() -> ! {
    todo!("0x735660 __ZNK3RBX9FreeJoint12getJointTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::FreeJoint::~FreeJoint()")]
// 0x735664 — __ZThn32_N3RBX9FreeJointD1Ev
pub fn stub_735664() -> ! {
    todo!("0x735664 __ZThn32_N3RBX9FreeJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::FreeJoint::~FreeJoint()")]
// 0x73566c — __ZThn32_N3RBX9FreeJointD0Ev
pub fn stub_73566c() -> ! {
    todo!("0x73566c __ZThn32_N3RBX9FreeJointD0Ev")
}

#[doc(alias = "RBX::AnchorJoint::AnchorJoint(RBX::Primitive *)")]
// 0x735710 — __ZN3RBX11AnchorJointC2EPNS_9PrimitiveE
pub fn stub_735710() -> ! {
    todo!("0x735710 __ZN3RBX11AnchorJointC2EPNS_9PrimitiveE")
}

#[doc(alias = "RBX::AnchorJoint::~AnchorJoint()")]
// 0x7357dc — __ZN3RBX11AnchorJointD1Ev
pub fn stub_7357dc() -> ! {
    todo!("0x7357dc __ZN3RBX11AnchorJointD1Ev")
}

#[doc(alias = "RBX::AnchorJoint::~AnchorJoint()")]
// 0x7357e0 — __ZN3RBX11AnchorJointD0Ev
pub fn stub_7357e0() -> ! {
    todo!("0x7357e0 __ZN3RBX11AnchorJointD0Ev")
}

#[doc(alias = "RBX::AnchorJoint::getJointType(void)const")]
// 0x735880 — __ZNK3RBX11AnchorJoint12getJointTypeEv
pub fn stub_735880() -> ! {
    todo!("0x735880 __ZNK3RBX11AnchorJoint12getJointTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::AnchorJoint::~AnchorJoint()")]
// 0x735884 — __ZThn32_N3RBX11AnchorJointD1Ev
pub fn stub_735884() -> ! {
    todo!("0x735884 __ZThn32_N3RBX11AnchorJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AnchorJoint::~AnchorJoint()")]
// 0x73588c — __ZThn32_N3RBX11AnchorJointD0Ev
pub fn stub_73588c() -> ! {
    todo!("0x73588c __ZThn32_N3RBX11AnchorJointD0Ev")
}

#[doc(alias = "RBX::HumanoidStage::HumanoidStage(RBX::IStage *,RBX::World *)")]
// 0x735a2c — __ZN3RBX13HumanoidStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_735a2c() -> ! {
    todo!("0x735a2c __ZN3RBX13HumanoidStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::HumanoidStage::HumanoidStage(RBX::IStage *,RBX::World *)")]
// 0x735a30 — __ZN3RBX13HumanoidStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_735a30() -> ! {
    todo!("0x735a30 __ZN3RBX13HumanoidStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::HumanoidStage::~HumanoidStage()")]
// 0x735b18 — __ZN3RBX13HumanoidStageD0Ev
pub fn stub_735b18() -> ! {
    todo!("0x735b18 __ZN3RBX13HumanoidStageD0Ev")
}

#[doc(alias = "RBX::HumanoidStage::~HumanoidStage()")]
// 0x735bb8 — __ZN3RBX13HumanoidStageD1Ev
pub fn stub_735bb8() -> ! {
    todo!("0x735bb8 __ZN3RBX13HumanoidStageD1Ev")
}

#[doc(alias = "RBX::HumanoidStage::~HumanoidStage()")]
// 0x735bbc — __ZN3RBX13HumanoidStageD2Ev
pub fn stub_735bbc() -> ! {
    todo!("0x735bbc __ZN3RBX13HumanoidStageD2Ev")
}

#[doc(alias = "RBX::HumanoidStage::toHumanoid(RBX::Assembly *)")]
// 0x735d20 — __ZN3RBX13HumanoidStage10toHumanoidEPNS_8AssemblyE
pub fn stub_735d20() -> ! {
    todo!("0x735d20 __ZN3RBX13HumanoidStage10toHumanoidEPNS_8AssemblyE")
}

#[doc(alias = "RBX::HumanoidStage::fromHumanoid(RBX::Assembly *)")]
// 0x735de4 — __ZN3RBX13HumanoidStage12fromHumanoidEPNS_8AssemblyE
pub fn stub_735de4() -> ! {
    todo!("0x735de4 __ZN3RBX13HumanoidStage12fromHumanoidEPNS_8AssemblyE")
}

#[doc(alias = "RBX::HumanoidStage::onAssemblyAdded(RBX::Assembly *)")]
// 0x735ea4 — __ZN3RBX13HumanoidStage15onAssemblyAddedEPNS_8AssemblyE
pub fn stub_735ea4() -> ! {
    todo!("0x735ea4 __ZN3RBX13HumanoidStage15onAssemblyAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::HumanoidStage::onAssemblyRemoving(RBX::Assembly *)")]
// 0x735edc — __ZN3RBX13HumanoidStage18onAssemblyRemovingEPNS_8AssemblyE
pub fn stub_735edc() -> ! {
    todo!("0x735edc __ZN3RBX13HumanoidStage18onAssemblyRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::HumanoidStage::getStageType(void)const")]
// 0x735f10 — __ZNK3RBX13HumanoidStage12getStageTypeEv
pub fn stub_735f10() -> ! {
    todo!("0x735f10 __ZNK3RBX13HumanoidStage12getStageTypeEv")
}

#[doc(alias = "RBX::IMoving::IMoving(void)")]
// 0x736044 — __ZN3RBX7IMovingC2Ev
pub fn stub_736044() -> ! {
    todo!("0x736044 __ZN3RBX7IMovingC2Ev")
}

#[doc(alias = "RBX::IMoving::makeMoving(void)")]
// 0x736060 — __ZN3RBX7IMoving10makeMovingEv
pub fn stub_736060() -> ! {
    todo!("0x736060 __ZN3RBX7IMoving10makeMovingEv")
}

#[doc(alias = "RBX::IMoving::setMovingManager(RBX::IMovingManager *)")]
// 0x7360d8 — __ZN3RBX7IMoving16setMovingManagerEPNS_14IMovingManagerE
pub fn stub_7360d8() -> ! {
    todo!("0x7360d8 __ZN3RBX7IMoving16setMovingManagerEPNS_14IMovingManagerE")
}

#[doc(alias = "RBX::IMovingManager::remove(RBX::IMoving *)")]
// 0x736108 — __ZN3RBX14IMovingManager6removeEPNS_7IMovingE
pub fn stub_736108() -> ! {
    todo!("0x736108 __ZN3RBX14IMovingManager6removeEPNS_7IMovingE")
}

#[doc(alias = "RBX::IMoving::checkSleep(void)")]
// 0x73614c — __ZN3RBX7IMoving10checkSleepEv
pub fn stub_73614c() -> ! {
    todo!("0x73614c __ZN3RBX7IMoving10checkSleepEv")
}

#[doc(alias = "RBX::IMoving::notifyMoved(void)")]
// 0x736214 — __ZN3RBX7IMoving11notifyMovedEv
pub fn stub_736214() -> ! {
    todo!("0x736214 __ZN3RBX7IMoving11notifyMovedEv")
}

#[doc(alias = "RBX::IMoving::forceSleep(void)")]
// 0x736250 — __ZN3RBX7IMoving10forceSleepEv
pub fn stub_736250() -> ! {
    todo!("0x736250 __ZN3RBX7IMoving10forceSleepEv")
}

#[doc(alias = "RBX::IMovingManager::IMovingManager(void)")]
// 0x736264 — __ZN3RBX14IMovingManagerC2Ev
pub fn stub_736264() -> ! {
    todo!("0x736264 __ZN3RBX14IMovingManagerC2Ev")
}

#[doc(alias = "RBX::IMovingManager::~IMovingManager()")]
// 0x73628c — __ZN3RBX14IMovingManagerD0Ev
pub fn stub_73628c() -> ! {
    todo!("0x73628c __ZN3RBX14IMovingManagerD0Ev")
}

#[doc(alias = "RBX::IMovingManager::~IMovingManager()")]
// 0x73632c — __ZN3RBX14IMovingManagerD1Ev
pub fn stub_73632c() -> ! {
    todo!("0x73632c __ZN3RBX14IMovingManagerD1Ev")
}

#[doc(alias = "RBX::IMovingManager::~IMovingManager()")]
// 0x736330 — __ZN3RBX14IMovingManagerD2Ev
pub fn stub_736330() -> ! {
    todo!("0x736330 __ZN3RBX14IMovingManagerD2Ev")
}

#[doc(alias = "RBX::IMovingManager::onMovingHeartbeat(void)")]
// 0x73649c — __ZN3RBX14IMovingManager17onMovingHeartbeatEv
pub fn stub_73649c() -> ! {
    todo!("0x73649c __ZN3RBX14IMovingManager17onMovingHeartbeatEv")
}

