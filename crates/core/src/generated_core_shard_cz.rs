//! core shard CZ — 100 core stubs EA-sorted, next uncovered after CY 0x73bcb0 (strict RBX|boost|std|rbx earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::resize(unsigned long,RBX::SurfaceType)")]
// 0x73bd18 — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE6resizeEmS1_
pub fn stub_73bd18() -> ! {
    todo!("0x73bd18 __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::push_back(RBX::SurfaceType const&)")]
// 0x73bd4c — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE9push_backERKS1_
pub fn stub_73bd4c() -> ! {
    todo!("0x73bd4c __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,RBX::SurfaceType const&)")]
// 0x73bd74 — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_73bd74() -> ! {
    todo!("0x73bd74 __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_allocate(unsigned long)")]
// 0x73be58 — __ZNSt12_Vector_baseIN3RBX11SurfaceTypeESaIS1_EE11_M_allocateEm
pub fn stub_73be58() -> ! {
    todo!("0x73be58 __ZNSt12_Vector_baseIN3RBX11SurfaceTypeESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SurfaceType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SurfaceType *,RBX::SurfaceType *>(RBX::SurfaceType *,RBX::SurfaceType *,RBX::SurfaceType *)")]
// 0x73be70 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SurfaceTypeES5_EET0_T_S7_S6_
pub fn stub_73be70() -> ! {
    todo!("0x73be70 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SurfaceTypeES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,unsigned long,RBX::SurfaceType const&)")]
// 0x73beac — __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_73beac() -> ! {
    todo!("0x73beac __ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "RBX::Mechanism::Mechanism(void)")]
// 0x73c104 — __ZN3RBX9MechanismC1Ev
pub fn stub_73c104() -> ! {
    todo!("0x73c104 __ZN3RBX9MechanismC1Ev")
}

#[doc(alias = "RBX::Mechanism::Mechanism(void)")]
// 0x73c108 — __ZN3RBX9MechanismC2Ev
pub fn stub_73c108() -> ! {
    todo!("0x73c108 __ZN3RBX9MechanismC2Ev")
}

#[doc(alias = "RBX::Mechanism::~Mechanism()")]
// 0x73c1e8 — __ZN3RBX9MechanismD0Ev
pub fn stub_73c1e8() -> ! {
    todo!("0x73c1e8 __ZN3RBX9MechanismD0Ev")
}

#[doc(alias = "RBX::Mechanism::~Mechanism()")]
// 0x73c288 — __ZN3RBX9MechanismD1Ev
pub fn stub_73c288() -> ! {
    todo!("0x73c288 __ZN3RBX9MechanismD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Mechanism::~Mechanism()")]
// 0x73c28c — __ZThn8_N3RBX9MechanismD0Ev
pub fn stub_73c28c() -> ! {
    todo!("0x73c28c __ZThn8_N3RBX9MechanismD0Ev")
}

#[doc(alias = "RBX::Mechanism::~Mechanism()")]
// 0x73c294 — __ZN3RBX9MechanismD2Ev
pub fn stub_73c294() -> ! {
    todo!("0x73c294 __ZN3RBX9MechanismD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Mechanism::~Mechanism()")]
// 0x73c350 — __ZThn8_N3RBX9MechanismD1Ev
pub fn stub_73c350() -> ! {
    todo!("0x73c350 __ZThn8_N3RBX9MechanismD1Ev")
}

#[doc(alias = "RBX::Mechanism::getConstMechanismPrimitive(void)const")]
// 0x73c358 — __ZNK3RBX9Mechanism26getConstMechanismPrimitiveEv
pub fn stub_73c358() -> ! {
    todo!("0x73c358 __ZNK3RBX9Mechanism26getConstMechanismPrimitiveEv")
}

#[doc(alias = "RBX::Mechanism::getMechanismPrimitive(void)")]
// 0x73c36c — __ZN3RBX9Mechanism21getMechanismPrimitiveEv
pub fn stub_73c36c() -> ! {
    todo!("0x73c36c __ZN3RBX9Mechanism21getMechanismPrimitiveEv")
}

#[doc(alias = "RBX::Mechanism::isComplexMovingMechanism(RBX::Assembly const*)")]
// 0x73c380 — __ZN3RBX9Mechanism24isComplexMovingMechanismEPKNS_8AssemblyE
pub fn stub_73c380() -> ! {
    todo!("0x73c380 __ZN3RBX9Mechanism24isComplexMovingMechanismEPKNS_8AssemblyE")
}

#[doc(alias = "RBX::Mechanism::isMovingAssemblyRoot(RBX::Assembly const*)")]
// 0x73c3e4 — __ZN3RBX9Mechanism20isMovingAssemblyRootEPKNS_8AssemblyE
pub fn stub_73c3e4() -> ! {
    todo!("0x73c3e4 __ZN3RBX9Mechanism20isMovingAssemblyRootEPKNS_8AssemblyE")
}

#[doc(alias = "RBX::Mechanism::getMovingAssemblyRoot(RBX::Assembly *)")]
// 0x73c40c — __ZN3RBX9Mechanism21getMovingAssemblyRootEPNS_8AssemblyE
pub fn stub_73c40c() -> ! {
    todo!("0x73c40c __ZN3RBX9Mechanism21getMovingAssemblyRootEPNS_8AssemblyE")
}

#[doc(alias = "RBX::Mechanism::getConstMovingAssemblyRoot(RBX::Assembly const*)")]
// 0x73c434 — __ZN3RBX9Mechanism26getConstMovingAssemblyRootEPKNS_8AssemblyE
pub fn stub_73c434() -> ! {
    todo!("0x73c434 __ZN3RBX9Mechanism26getConstMovingAssemblyRootEPKNS_8AssemblyE")
}

#[doc(alias = "RBX::Mechanism::getConstRootMovingPrimitive(RBX::Primitive const*)")]
// 0x73c45c — __ZN3RBX9Mechanism27getConstRootMovingPrimitiveEPKNS_9PrimitiveE
pub fn stub_73c45c() -> ! {
    todo!("0x73c45c __ZN3RBX9Mechanism27getConstRootMovingPrimitiveEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::Mechanism::getRootMovingPrimitive(RBX::Primitive *)")]
// 0x73c4d0 — __ZN3RBX9Mechanism22getRootMovingPrimitiveEPNS_9PrimitiveE
pub fn stub_73c4d0() -> ! {
    todo!("0x73c4d0 __ZN3RBX9Mechanism22getRootMovingPrimitiveEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::Mechanism::getPrimitiveMechanism(RBX::Primitive *)")]
// 0x73c4d4 — __ZN3RBX9Mechanism21getPrimitiveMechanismEPNS_9PrimitiveE
pub fn stub_73c4d4() -> ! {
    todo!("0x73c4d4 __ZN3RBX9Mechanism21getPrimitiveMechanismEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::Mechanism::getConstPrimitiveMechanism(RBX::Primitive const*)")]
// 0x73c4fc — __ZN3RBX9Mechanism26getConstPrimitiveMechanismEPKNS_9PrimitiveE
pub fn stub_73c4fc() -> ! {
    todo!("0x73c4fc __ZN3RBX9Mechanism26getConstPrimitiveMechanismEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::Mechanism::getRootAssembly(void)")]
// 0x73c524 — __ZN3RBX9Mechanism15getRootAssemblyEv
pub fn stub_73c524() -> ! {
    todo!("0x73c524 __ZN3RBX9Mechanism15getRootAssemblyEv")
}

#[doc(alias = "RBX::Mechanism::isMechanismRootPrimitive(RBX::Primitive const*)")]
// 0x73c584 — __ZN3RBX9Mechanism24isMechanismRootPrimitiveEPKNS_9PrimitiveE
pub fn stub_73c584() -> ! {
    todo!("0x73c584 __ZN3RBX9Mechanism24isMechanismRootPrimitiveEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::MechToAssemblyStage::MechToAssemblyStage(RBX::IStage *,RBX::World *)")]
// 0x73c6f0 — __ZN3RBX19MechToAssemblyStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_73c6f0() -> ! {
    todo!("0x73c6f0 __ZN3RBX19MechToAssemblyStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::MechToAssemblyStage::MechToAssemblyStage(RBX::IStage *,RBX::World *)")]
// 0x73c6f4 — __ZN3RBX19MechToAssemblyStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_73c6f4() -> ! {
    todo!("0x73c6f4 __ZN3RBX19MechToAssemblyStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::MechToAssemblyStage::~MechToAssemblyStage()")]
// 0x73c7c8 — __ZN3RBX19MechToAssemblyStageD0Ev
pub fn stub_73c7c8() -> ! {
    todo!("0x73c7c8 __ZN3RBX19MechToAssemblyStageD0Ev")
}

#[doc(alias = "RBX::MechToAssemblyStage::~MechToAssemblyStage()")]
// 0x73c880 — __ZN3RBX19MechToAssemblyStageD1Ev
pub fn stub_73c880() -> ! {
    todo!("0x73c880 __ZN3RBX19MechToAssemblyStageD1Ev")
}

#[doc(alias = "RBX::MechToAssemblyStage::onSimulateAssemblyRootAdded(RBX::Assembly *)")]
// 0x73c8a4 — __ZN3RBX19MechToAssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE
pub fn stub_73c8a4() -> ! {
    todo!("0x73c8a4 __ZN3RBX19MechToAssemblyStage27onSimulateAssemblyRootAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::MechToAssemblyStage::onSimulateAssemblyRootRemoving(RBX::Assembly *)")]
// 0x73c958 — __ZN3RBX19MechToAssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE
pub fn stub_73c958() -> ! {
    todo!("0x73c958 __ZN3RBX19MechToAssemblyStage30onSimulateAssemblyRootRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::MechToAssemblyStage::onNoSimulateAssemblyRootAdded(RBX::Assembly *)")]
// 0x73ca18 — __ZN3RBX19MechToAssemblyStage29onNoSimulateAssemblyRootAddedEPNS_8AssemblyE
pub fn stub_73ca18() -> ! {
    todo!("0x73ca18 __ZN3RBX19MechToAssemblyStage29onNoSimulateAssemblyRootAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::MechToAssemblyStage::onNoSimulateAssemblyRootRemoving(RBX::Assembly *)")]
// 0x73cacc — __ZN3RBX19MechToAssemblyStage32onNoSimulateAssemblyRootRemovingEPNS_8AssemblyE
pub fn stub_73cacc() -> ! {
    todo!("0x73cacc __ZN3RBX19MechToAssemblyStage32onNoSimulateAssemblyRootRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::MechToAssemblyStage::onFixedAssemblyAdded(RBX::Assembly *)")]
// 0x73cb8c — __ZN3RBX19MechToAssemblyStage20onFixedAssemblyAddedEPNS_8AssemblyE
pub fn stub_73cb8c() -> ! {
    todo!("0x73cb8c __ZN3RBX19MechToAssemblyStage20onFixedAssemblyAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::MechToAssemblyStage::onFixedAssemblyRemoving(RBX::Assembly *)")]
// 0x73cba8 — __ZN3RBX19MechToAssemblyStage23onFixedAssemblyRemovingEPNS_8AssemblyE
pub fn stub_73cba8() -> ! {
    todo!("0x73cba8 __ZN3RBX19MechToAssemblyStage23onFixedAssemblyRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::MechToAssemblyStage::getStageType(void)const")]
// 0x73cbc4 — __ZNK3RBX19MechToAssemblyStage12getStageTypeEv
pub fn stub_73cbc4() -> ! {
    todo!("0x73cbc4 __ZNK3RBX19MechToAssemblyStage12getStageTypeEv")
}

#[doc(alias = "RBX::POLY::Mesh::addVertex(float,float,float)")]
// 0x73cea8 — __ZN3RBX4POLY4Mesh9addVertexEfff
pub fn stub_73cea8() -> ! {
    todo!("0x73cea8 __ZN3RBX4POLY4Mesh9addVertexEfff")
}

#[doc(alias = "RBX::POLY::Mesh::addFace(unsigned long,unsigned long,unsigned long,unsigned long)")]
// 0x73cf7c — __ZN3RBX4POLY4Mesh7addFaceEmmmm
pub fn stub_73cf7c() -> ! {
    todo!("0x73cf7c __ZN3RBX4POLY4Mesh7addFaceEmmmm")
}

#[doc(alias = "RBX::POLY::Mesh::addFace(unsigned long,unsigned long,unsigned long)")]
// 0x73d4a0 — __ZN3RBX4POLY4Mesh7addFaceEmmm
pub fn stub_73d4a0() -> ! {
    todo!("0x73d4a0 __ZN3RBX4POLY4Mesh7addFaceEmmm")
}

#[doc(alias = "RBX::POLY::Mesh::addFace(int,int *,bool)")]
// 0x73e120 — __ZN3RBX4POLY4Mesh7addFaceEiPib
pub fn stub_73e120() -> ! {
    todo!("0x73e120 __ZN3RBX4POLY4Mesh7addFaceEiPib")
}

#[doc(alias = "RBX::POLY::Mesh::findOrMakeEdge(unsigned long,unsigned long)")]
// 0x73ebac — __ZN3RBX4POLY4Mesh14findOrMakeEdgeEmm
pub fn stub_73ebac() -> ! {
    todo!("0x73ebac __ZN3RBX4POLY4Mesh14findOrMakeEdgeEmm")
}

#[doc(alias = "RBX::POLY::Face::initPlane(void)")]
// 0x73ed38 — __ZN3RBX4POLY4Face9initPlaneEv
pub fn stub_73ed38() -> ! {
    todo!("0x73ed38 __ZN3RBX4POLY4Face9initPlaneEv")
}

#[doc(alias = "RBX::POLY::Vertex::findEdge(RBX::POLY::Vertex const*)")]
// 0x73ed88 — __ZN3RBX4POLY6Vertex8findEdgeEPKS1_
pub fn stub_73ed88() -> ! {
    todo!("0x73ed88 __ZN3RBX4POLY6Vertex8findEdgeEPKS1_")
}

#[doc(alias = "RBX::POLY::Mesh::addEdge(RBX::POLY::Vertex *,RBX::POLY::Vertex *)")]
// 0x73ee28 — __ZN3RBX4POLY4Mesh7addEdgeEPNS0_6VertexES3_
pub fn stub_73ee28() -> ! {
    todo!("0x73ee28 __ZN3RBX4POLY4Mesh7addEdgeEPNS0_6VertexES3_")
}

#[doc(alias = "RBX::POLY::Vertex::getFace(unsigned long)const")]
// 0x73f16c — __ZNK3RBX4POLY6Vertex7getFaceEm
pub fn stub_73f16c() -> ! {
    todo!("0x73f16c __ZNK3RBX4POLY6Vertex7getFaceEm")
}

#[doc(alias = "RBX::POLY::Face::Face(unsigned long,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
// 0x73f17c — __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_
pub fn stub_73f17c() -> ! {
    todo!("0x73f17c __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_")
}

#[doc(alias = "RBX::POLY::Face::Face(unsigned long,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
// 0x73f288 — __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_S3_
pub fn stub_73f288() -> ! {
    todo!("0x73f288 __ZN3RBX4POLY4FaceC2EmPNS0_4EdgeES3_S3_S3_")
}

#[doc(alias = "RBX::POLY::Face::Face(unsigned long,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> &)")]
// 0x73f3a8 — __ZN3RBX4POLY4FaceC2EmRSt6vectorIPNS0_4EdgeESaIS4_EE
pub fn stub_73f3a8() -> ! {
    todo!("0x73f3a8 __ZN3RBX4POLY4FaceC2EmRSt6vectorIPNS0_4EdgeESaIS4_EE")
}

#[doc(alias = "RBX::POLY::Face::getCentroid(void)const")]
// 0x73fa80 — __ZNK3RBX4POLY4Face11getCentroidEv
pub fn stub_73fa80() -> ! {
    todo!("0x73fa80 __ZNK3RBX4POLY4Face11getCentroidEv")
}

#[doc(alias = "RBX::POLY::Vertex::recoverEdge(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*)")]
// 0x73ff5c — __ZN3RBX4POLY6Vertex11recoverEdgeEPKS1_S3_
pub fn stub_73ff5c() -> ! {
    todo!("0x73ff5c __ZN3RBX4POLY6Vertex11recoverEdgeEPKS1_S3_")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::reserve(unsigned long)")]
// 0x7400a0 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE7reserveEm
pub fn stub_7400a0() -> ! {
    todo!("0x7400a0 __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::reserve(unsigned long)")]
// 0x74014c — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE7reserveEm
pub fn stub_74014c() -> ! {
    todo!("0x74014c __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::reserve(unsigned long)")]
// 0x7401f8 — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE7reserveEm
pub fn stub_7401f8() -> ! {
    todo!("0x7401f8 __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::push_back(RBX::POLY::Vertex const&)")]
// 0x740290 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE9push_backERKS2_
pub fn stub_740290() -> ! {
    todo!("0x740290 __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::push_back(RBX::POLY::Face const&)")]
// 0x7402dc — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE9push_backERKS2_
pub fn stub_7402dc() -> ! {
    todo!("0x7402dc __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "RBX::POLY::Edge::addFace(RBX::POLY::Face const*)")]
// 0x740314 — __ZN3RBX4POLY4Edge7addFaceEPKNS0_4FaceE
pub fn stub_740314() -> ! {
    todo!("0x740314 __ZN3RBX4POLY4Edge7addFaceEPKNS0_4FaceE")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::push_back(RBX::POLY::Edge * const&)")]
// 0x7403c0 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE9push_backERKS3_
pub fn stub_7403c0() -> ! {
    todo!("0x7403c0 __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::push_back(RBX::POLY::Edge const&)")]
// 0x7403ec — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE9push_backERKS2_
pub fn stub_7403ec() -> ! {
    todo!("0x7403ec __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "RBX::POLY::Vertex::addEdge(RBX::POLY::Edge *)")]
// 0x74041c — __ZN3RBX4POLY6Vertex7addEdgeEPNS0_4EdgeE
pub fn stub_74041c() -> ! {
    todo!("0x74041c __ZN3RBX4POLY6Vertex7addEdgeEPNS0_4EdgeE")
}

#[doc(alias = "RBX::POLY::Edge::getVertexFace(RBX::POLY::Vertex const*)const")]
// 0x74049c — __ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE
pub fn stub_74049c() -> ! {
    todo!("0x74049c __ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE")
}

#[doc(alias = "RBX::POLY::Edge::otherFace(RBX::POLY::Face const*)const")]
// 0x740508 — __ZNK3RBX4POLY4Edge9otherFaceEPKNS0_4FaceE
pub fn stub_740508() -> ! {
    todo!("0x740508 __ZNK3RBX4POLY4Edge9otherFaceEPKNS0_4FaceE")
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Edge*,std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>>,RBX::POLY::Edge const&)")]
// 0x7405cc — __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7405cc() -> ! {
    todo!("0x7405cc __ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_allocate(unsigned long)")]
// 0x740718 — __ZNSt12_Vector_baseIN3RBX4POLY4EdgeESaIS2_EE11_M_allocateEm
pub fn stub_740718() -> ! {
    todo!("0x740718 __ZNSt12_Vector_baseIN3RBX4POLY4EdgeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::POLY::Edge * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Edge *,RBX::POLY::Edge *>(RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
// 0x74073c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4EdgeES6_EET0_T_S8_S7_
pub fn stub_74073c() -> ! {
    todo!("0x74073c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4EdgeES6_EET0_T_S8_S7_")
}

#[doc(alias = "__gnu_cxx::new_allocator<RBX::POLY::Face>::construct(RBX::POLY::Face*,RBX::POLY::Face const&)")]
// 0x74079c — __ZN9__gnu_cxx13new_allocatorIN3RBX4POLY4FaceEE9constructEPS3_RKS3_
pub fn stub_74079c() -> ! {
    todo!("0x74079c __ZN9__gnu_cxx13new_allocatorIN3RBX4POLY4FaceEE9constructEPS3_RKS3_")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Face*,std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>>,RBX::POLY::Face const&)")]
// 0x7407e4 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7407e4() -> ! {
    todo!("0x7407e4 __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::POLY::Face::operator=(RBX::POLY::Face const&)")]
// 0x740bf8 — __ZN3RBX4POLY4FaceaSERKS1_
pub fn stub_740bf8() -> ! {
    todo!("0x740bf8 __ZN3RBX4POLY4FaceaSERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_allocate(unsigned long)")]
// 0x740c20 — __ZNSt12_Vector_baseIN3RBX4POLY4FaceESaIS2_EE11_M_allocateEm
pub fn stub_740c20() -> ! {
    todo!("0x740c20 __ZNSt12_Vector_baseIN3RBX4POLY4FaceESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::operator=(std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> const&)")]
// 0x740c44 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEaSERKS5_
pub fn stub_740c44() -> ! {
    todo!("0x740c44 __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEaSERKS5_")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_M_allocate(unsigned long)")]
// 0x740cdc — __ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EE11_M_allocateEm
pub fn stub_740cdc() -> ! {
    todo!("0x740cdc __ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "RBX::POLY::Face * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Face *,RBX::POLY::Face *>(RBX::POLY::Face *,RBX::POLY::Face *,RBX::POLY::Face *)")]
// 0x740cf4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4FaceES6_EET0_T_S8_S7_
pub fn stub_740cf4() -> ! {
    todo!("0x740cf4 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4FaceES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::vector(std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> const&)")]
// 0x740d50 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEC2ERKS5_
pub fn stub_740d50() -> ! {
    todo!("0x740d50 __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEC2ERKS5_")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_Vector_base(unsigned long,std::allocator<RBX::POLY::Edge *> const&)")]
// 0x740d88 — __ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EEC2EmRKS4_
pub fn stub_740d88() -> ! {
    todo!("0x740d88 __ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EEC2EmRKS4_")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Vertex*,std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>>,RBX::POLY::Vertex const&)")]
// 0x740db8 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_740db8() -> ! {
    todo!("0x740db8 __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_allocate(unsigned long)")]
// 0x741158 — __ZNSt12_Vector_baseIN3RBX4POLY6VertexESaIS2_EE11_M_allocateEm
pub fn stub_741158() -> ! {
    todo!("0x741158 __ZNSt12_Vector_baseIN3RBX4POLY6VertexESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::POLY::Vertex * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Vertex *,RBX::POLY::Vertex *>(RBX::POLY::Vertex *,RBX::POLY::Vertex *,RBX::POLY::Vertex *)")]
// 0x74117c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY6VertexES6_EET0_T_S8_S7_
pub fn stub_74117c() -> ! {
    todo!("0x74117c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY6VertexES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_erase_at_end(RBX::POLY::Face*)")]
// 0x741214 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_741214() -> ! {
    todo!("0x741214 __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_erase_at_end(RBX::POLY::Vertex*)")]
// 0x741244 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_741244() -> ! {
    todo!("0x741244 __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "RBX::POLY::Face* std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_allocate_and_copy<RBX::POLY::Face*>(unsigned long,RBX::POLY::Face*,RBX::POLY::Face*)")]
// 0x741274 — __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_
pub fn stub_741274() -> ! {
    todo!("0x741274 __ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_")
}

#[doc(alias = "RBX::POLY::Vertex* std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_allocate_and_copy<RBX::POLY::Vertex*>(unsigned long,RBX::POLY::Vertex*,RBX::POLY::Vertex*)")]
// 0x741410 — __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_
pub fn stub_741410() -> ! {
    todo!("0x741410 __ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge * const&)")]
// 0x741570 — __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_741570() -> ! {
    todo!("0x741570 __ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge *>(__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge * const&,std::random_access_iterator_tag)")]
// 0x741650 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX4POLY4EdgeESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
pub fn stub_741650() -> ! {
    todo!("0x741650 __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX4POLY4EdgeESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "RBX::Motor6DJoint::Motor6DJoint(void)")]
// 0x7417a8 — __ZN3RBX12Motor6DJointC1Ev
pub fn stub_7417a8() -> ! {
    todo!("0x7417a8 __ZN3RBX12Motor6DJointC1Ev")
}

#[doc(alias = "RBX::Motor6DJoint::Motor6DJoint(void)")]
// 0x7417ac — __ZN3RBX12Motor6DJointC2Ev
pub fn stub_7417ac() -> ! {
    todo!("0x7417ac __ZN3RBX12Motor6DJointC2Ev")
}

#[doc(alias = "RBX::Motor6DJoint::~Motor6DJoint()")]
// 0x741920 — __ZN3RBX12Motor6DJointD0Ev
pub fn stub_741920() -> ! {
    todo!("0x741920 __ZN3RBX12Motor6DJointD0Ev")
}

#[doc(alias = "RBX::Motor6DJoint::~Motor6DJoint()")]
// 0x7419c0 — __ZN3RBX12Motor6DJointD1Ev
pub fn stub_7419c0() -> ! {
    todo!("0x7419c0 __ZN3RBX12Motor6DJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Motor6DJoint::~Motor6DJoint()")]
// 0x7419c4 — __ZThn32_N3RBX12Motor6DJointD0Ev
pub fn stub_7419c4() -> ! {
    todo!("0x7419c4 __ZThn32_N3RBX12Motor6DJointD0Ev")
}

#[doc(alias = "RBX::Motor6DJoint::~Motor6DJoint()")]
// 0x7419cc — __ZN3RBX12Motor6DJointD2Ev
pub fn stub_7419cc() -> ! {
    todo!("0x7419cc __ZN3RBX12Motor6DJointD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Motor6DJoint::~Motor6DJoint()")]
// 0x741ac8 — __ZThn32_N3RBX12Motor6DJointD1Ev
pub fn stub_741ac8() -> ! {
    todo!("0x741ac8 __ZThn32_N3RBX12Motor6DJointD1Ev")
}

#[doc(alias = "RBX::Motor6DJoint::getParentId(void)const")]
// 0x741ad0 — __ZNK3RBX12Motor6DJoint11getParentIdEv
pub fn stub_741ad0() -> ! {
    todo!("0x741ad0 __ZNK3RBX12Motor6DJoint11getParentIdEv")
}

#[doc(alias = "RBX::Motor6DJoint::resetLink(void)")]
// 0x741bf8 — __ZN3RBX12Motor6DJoint9resetLinkEv
pub fn stub_741bf8() -> ! {
    todo!("0x741bf8 __ZN3RBX12Motor6DJoint9resetLinkEv")
}

#[doc(alias = "RBX::Motor6DJoint::getCurrentZAngle(void)const")]
// 0x741c44 — __ZNK3RBX12Motor6DJoint16getCurrentZAngleEv
pub fn stub_741c44() -> ! {
    todo!("0x741c44 __ZNK3RBX12Motor6DJoint16getCurrentZAngleEv")
}

#[doc(alias = "RBX::Motor6DJoint::stepUi(double)")]
// 0x741c50 — __ZN3RBX12Motor6DJoint6stepUiEd
pub fn stub_741c50() -> ! {
    todo!("0x741c50 __ZN3RBX12Motor6DJoint6stepUiEd")
}

#[doc(alias = "RBX::Motor6DJoint::setCurrentZAngle(float)")]
// 0x74202c — __ZN3RBX12Motor6DJoint16setCurrentZAngleEf
pub fn stub_74202c() -> ! {
    todo!("0x74202c __ZN3RBX12Motor6DJoint16setCurrentZAngleEf")
}

#[doc(alias = "RBX::Motor6DJoint::isAligned(void)")]
// 0x7420e0 — __ZN3RBX12Motor6DJoint9isAlignedEv
pub fn stub_7420e0() -> ! {
    todo!("0x7420e0 __ZN3RBX12Motor6DJoint9isAlignedEv")
}

#[doc(alias = "RBX::Allocator<RBX::D6Link>::operator new(unsigned long)")]
// 0x7421d8 — __ZN3RBX9AllocatorINS_6D6LinkEEnwEm
pub fn stub_7421d8() -> ! {
    todo!("0x7421d8 __ZN3RBX9AllocatorINS_6D6LinkEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::D6Link>::operator delete(void *)")]
// 0x742248 — __ZN3RBX9AllocatorINS_6D6LinkEEdlEPv
pub fn stub_742248() -> ! {
    todo!("0x742248 __ZN3RBX9AllocatorINS_6D6LinkEEdlEPv")
}

#[doc(alias = "RBX::Motor6DJoint::getJointType(void)const")]
// 0x74231c — __ZNK3RBX12Motor6DJoint12getJointTypeEv
pub fn stub_74231c() -> ! {
    todo!("0x74231c __ZNK3RBX12Motor6DJoint12getJointTypeEv")
}

#[doc(alias = "RBX::Motor6DJoint::isBroken(void)const")]
// 0x742320 — __ZNK3RBX12Motor6DJoint8isBrokenEv
pub fn stub_742320() -> ! {
    todo!("0x742320 __ZNK3RBX12Motor6DJoint8isBrokenEv")
}

#[doc(alias = "RBX::Motor6DJoint::canStepUi(void)const")]
// 0x742324 — __ZNK3RBX12Motor6DJoint9canStepUiEv
pub fn stub_742324() -> ! {
    todo!("0x742324 __ZNK3RBX12Motor6DJoint9canStepUiEv")
}
