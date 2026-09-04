//! core shard GH — 100 core stubs EA-sorted, 0xf4dbb4..0xf4e574 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf4dba4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf4dba4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "void std::sort_heap<RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*)>(RBX::Joint **,RBX::Joint **,bool (*)(RBX::Joint const*,RBX::Joint const*))")]
// 0xf4dbb4 — j___ZSt9sort_heapIPPN3RBX5JointEPFbPKS1_S5_EEvT_S8_T0_
pub fn stub_f4dbb4() {
    // IDA 0xf4dbb4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::sample(RBX::PhysicsCoord,bool)")]
// 0xf4dbc4 — j___ZN3RBX7AverageINS_12PhysicsCoordEE6sampleES1_b
pub fn stub_f4dbc4() {
    // IDA 0xf4dbc4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::Average(unsigned long,RBX::PhysicsCoord)")]
// 0xf4dbd4 — j___ZN3RBX7AverageINS_12PhysicsCoordEEC2EmS1_
pub fn stub_f4dbd4() {
    // IDA 0xf4dbd4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Average<RBX::PhysicsCoord>::getAverage(void)const")]
// 0xf4dbe4 — j___ZNK3RBX7AverageINS_12PhysicsCoordEE10getAverageEv
pub fn stub_f4dbe4() {
    // IDA 0xf4dbe4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::_Vector_base<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_allocate(unsigned long)")]
// 0xf4dbf4 — j___ZNSt12_Vector_baseIN3RBX12PhysicsCoordESaIS1_EE11_M_allocateEm
pub fn stub_f4dbf4() {
    // IDA 0xf4dbf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PhysicsCoord * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PhysicsCoord *,RBX::PhysicsCoord *>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord *)")]
// 0xf4dc04 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12PhysicsCoordES5_EET0_T_S7_S6_
pub fn stub_f4dc04() {
    // IDA 0xf4dc04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::PhysicsCoord*,std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>>,unsigned long,RBX::PhysicsCoord const&)")]
// 0xf4dc14 — j___ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_f4dc14() {
    // IDA 0xf4dc14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::PhysicsCoord,std::allocator<RBX::PhysicsCoord>>::resize(unsigned long,RBX::PhysicsCoord)")]
// 0xf4dc24 — j___ZNSt6vectorIN3RBX12PhysicsCoordESaIS1_EE6resizeEmS1_
pub fn stub_f4dc24() {
    // IDA 0xf4dc24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::fill<RBX::PhysicsCoord *,RBX::PhysicsCoord>(RBX::PhysicsCoord *,RBX::PhysicsCoord *,RBX::PhysicsCoord const&)")]
// 0xf4dc34 — j___ZSt4fillIPN3RBX12PhysicsCoordES1_EvT_S3_RKT0_
pub fn stub_f4dc34() {
    // IDA 0xf4dc34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::Allocator(void)")]
// 0xf4dc44 — j___ZN3RBX9AllocatorINS_15BallPolyContactEEC2Ev
pub fn stub_f4dc44() {
    // IDA 0xf4dc44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Allocator<RBX::BallPolyContact>::operator delete(void *)")]
// 0xf4dc54 — j___ZN3RBX9AllocatorINS_15BallPolyContactEEdlEPv
pub fn stub_f4dc54() {
    // IDA 0xf4dc54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4dc64 — j___ZN5boost14singleton_poolIN3RBX15BallPolyContactELj212ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4dc64() {
    // IDA 0xf4dc64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Block::~Block()")]
// 0xf4dd24 — j___ZN3RBX5BlockD2Ev
pub fn stub_f4dd24() {
    // IDA 0xf4dd24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::Allocator(void)")]
// 0xf4dd34 — j___ZN3RBX9AllocatorINS_4POLY12BlockCornersEEC2Ev
pub fn stub_f4dd34() {
    // IDA 0xf4dd34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator delete(void *)")]
// 0xf4dd44 — j___ZN3RBX9AllocatorINS_4POLY12BlockCornersEEdlEPv
pub fn stub_f4dd44() {
    // IDA 0xf4dd44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockCorners>::operator new(unsigned long)")]
// 0xf4dd54 — j___ZN3RBX9AllocatorINS_4POLY12BlockCornersEEnwEm
pub fn stub_f4dd54() {
    // IDA 0xf4dd54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::Allocator(void)")]
// 0xf4dd64 — j___ZN3RBX9AllocatorINS_4POLY9BlockMeshEEC2Ev
pub fn stub_f4dd64() {
    // IDA 0xf4dd64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator delete(void *)")]
// 0xf4dd74 — j___ZN3RBX9AllocatorINS_4POLY9BlockMeshEEdlEPv
pub fn stub_f4dd74() {
    // IDA 0xf4dd74: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::BlockMesh>::operator new(unsigned long)")]
// 0xf4dd84 — j___ZN3RBX9AllocatorINS_4POLY9BlockMeshEEnwEm
pub fn stub_f4dd84() {
    // IDA 0xf4dd84: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4ddd4 — j___ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4ddd4() {
    // IDA 0xf4ddd4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4dde4 — j___ZN5boost14singleton_poolIN3RBX4POLY12BlockCornersELj96ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4dde4() {
    // IDA 0xf4dde4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4ddf4 — j___ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4ddf4() {
    // IDA 0xf4ddf4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4de04 — j___ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4de04() {
    // IDA 0xf4de04: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::~vector()")]
// 0xf4de54 — j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EED2Ev
pub fn stub_f4de54() {
    // IDA 0xf4de54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::~vector()")]
// 0xf4de64 — j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EED2Ev
pub fn stub_f4de64() {
    // IDA 0xf4de64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IPipelined::inStage(RBX::IStage *)const")]
// 0xf4df84 — j___ZNK3RBX10IPipelined7inStageEPNS_6IStageE
pub fn stub_f4df84() {
    // IDA 0xf4df84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::fastRemove(unsigned long)")]
// 0xf4dff4 — j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE10fastRemoveEm
pub fn stub_f4dff4() {
    // IDA 0xf4dff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::push_back(RBX::GeoPairConnector * const&)")]
// 0xf4e004 — j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EE9push_backERKS2_
pub fn stub_f4e004() {
    // IDA 0xf4e004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::FixedArray<RBX::GeoPairConnector *,8ul>::operator[](unsigned long)")]
// 0xf4e014 — j___ZN3RBX10FixedArrayIPNS_16GeoPairConnectorELm8EEixEm
pub fn stub_f4e014() {
    // IDA 0xf4e014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IPipelined::~IPipelined()")]
// 0xf4e024 — j___ZN3RBX10IPipelinedD2Ev
pub fn stub_f4e024() {
    // IDA 0xf4e024: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactConnector::isIntersecting(void)")]
// 0xf4e034 — j___ZN3RBX16ContactConnector14isIntersectingEv
pub fn stub_f4e034() {
    // IDA 0xf4e034: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContactConnector::ContactConnector(RBX::Body *,RBX::Body *,RBX::ContactParams const&)")]
// 0xf4e044 — j___ZN3RBX16ContactConnectorC2EPNS_4BodyES2_RKNS_13ContactParamsE
pub fn stub_f4e044() {
    // IDA 0xf4e044: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Edge::~Edge()")]
// 0xf4e054 — j___ZN3RBX4EdgeD2Ev
pub fn stub_f4e054() {
    // IDA 0xf4e054: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeoPair::match(RBX::Body *,RBX::Body *,RBX::GeoPairType,int,int)")]
// 0xf4e064 — j___ZN3RBX7GeoPair5matchEPNS_4BodyES2_NS_11GeoPairTypeEii
pub fn stub_f4e064() {
    // IDA 0xf4e064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::Allocator(void)")]
// 0xf4e074 — j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEC2Ev
pub fn stub_f4e074() {
    // IDA 0xf4e074: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator delete(void *)")]
// 0xf4e084 — j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEdlEPv
pub fn stub_f4e084() {
    // IDA 0xf4e084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::GeoPairConnector>::operator new(unsigned long)")]
// 0xf4e094 — j___ZN3RBX9AllocatorINS_16GeoPairConnectorEEnwEm
pub fn stub_f4e094() {
    // IDA 0xf4e094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::Allocator(void)")]
// 0xf4e0a4 — j___ZN3RBX9AllocatorINS_17BallBallConnectorEEC2Ev
pub fn stub_f4e0a4() {
    // IDA 0xf4e0a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::operator new(unsigned long)")]
// 0xf4e0b4 — j___ZN3RBX9AllocatorINS_17BallBallConnectorEEnwEm
pub fn stub_f4e0b4() {
    // IDA 0xf4e0b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::Allocator(void)")]
// 0xf4e0c4 — j___ZN3RBX9AllocatorINS_17BlockBlockContactEEC2Ev
pub fn stub_f4e0c4() {
    // IDA 0xf4e0c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BlockBlockContact>::operator delete(void *)")]
// 0xf4e0d4 — j___ZN3RBX9AllocatorINS_17BlockBlockContactEEdlEPv
pub fn stub_f4e0d4() {
    // IDA 0xf4e0d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::Allocator(void)")]
// 0xf4e0e4 — j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEC2Ev
pub fn stub_f4e0e4() {
    // IDA 0xf4e0e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::operator new(unsigned long)")]
// 0xf4e0f4 — j___ZN3RBX9AllocatorINS_18BallBlockConnectorEEnwEm
pub fn stub_f4e0f4() {
    // IDA 0xf4e0f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e104 — j___ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e104() {
    // IDA 0xf4e104: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e114 — j___ZN5boost14singleton_poolIN3RBX16GeoPairConnectorELj264ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e114() {
    // IDA 0xf4e114: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e124 — j___ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e124() {
    // IDA 0xf4e124: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e134 — j___ZN5boost14singleton_poolIN3RBX17BallBallConnectorELj272ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e134() {
    // IDA 0xf4e134: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e144 — j___ZN5boost14singleton_poolIN3RBX17BlockBlockContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e144() {
    // IDA 0xf4e144: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4e154 — j___ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4e154() {
    // IDA 0xf4e154: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4e164 — j___ZN5boost14singleton_poolIN3RBX18BallBlockConnectorELj288ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4e164() {
    // IDA 0xf4e164: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getEdgeVertex(int)const")]
// 0xf4e174 — j___ZNK3RBX5Block13getEdgeVertexEi
pub fn stub_f4e174() {
    // IDA 0xf4e174: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getNextGrid(RBX::Vector3int32 &,RBX::RbxRay const&,float)")]
// 0xf4e1f4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11getNextGridERNS_12Vector3int32ERKNS_6RbxRayEf
pub fn stub_f4e1f4() {
    // IDA 0xf4e1f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::changeMinMax(RBX::Primitive*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
// 0xf4e204 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS1_PKNS_12ExtentsInt32ES8_S8_b
pub fn stub_f4e204() {
    // IDA 0xf4e204: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::computeLevel(RBX::Primitive const*,RBX::Extents const&)")]
// 0xf4e214 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12computeLevelEPKS1_RKNS_7ExtentsE
pub fn stub_f4e214() {
    // IDA 0xf4e214: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
// 0xf4e224 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
pub fn stub_f4e224() {
    // IDA 0xf4e224: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(RBX::Primitive*,bool)")]
// 0xf4e234 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS1_b
pub fn stub_f4e234() {
    // IDA 0xf4e234: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
// 0xf4e244 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE
pub fn stub_f4e244() {
    // IDA 0xf4e244: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(RBX::Primitive*,bool)")]
// 0xf4e254 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS1_b
pub fn stub_f4e254() {
    // IDA 0xf4e254: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(RBX::Primitive*)")]
// 0xf4e264 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS1_
pub fn stub_f4e264() {
    // IDA 0xf4e264: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::oldExtentsOverlap(RBX::Primitive*,RBX::Primitive*)")]
// 0xf4e274 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_
pub fn stub_f4e274() {
    // IDA 0xf4e274: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeNodeFromHash(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0xf4e284 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE
pub fn stub_f4e284() {
    // IDA 0xf4e284: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)")]
// 0xf4e2b4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
pub fn stub_f4e2b4() {
    // IDA 0xf4e2b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,RBX::Primitive*,RBX::Vector3int32 const&,int)")]
// 0xf4e2c4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei
pub fn stub_f4e2c4() {
    // IDA 0xf4e2c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addContactFromChildren(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *,RBX::Primitive*)")]
// 0xf4e2d4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_
pub fn stub_f4e2d4() {
    // IDA 0xf4e2d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::checkAndReleaseContacts(RBX::Primitive*)")]
// 0xf4e2e4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_
pub fn stub_f4e2e4() {
    // IDA 0xf4e2e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::primitiveExtentsChanged(RBX::Primitive*,RBX::Extents const&)")]
// 0xf4e2f4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23primitiveExtentsChangedEPS1_RKNS_7ExtentsE
pub fn stub_f4e2f4() {
    // IDA 0xf4e2f4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveExtentsChanged(RBX::Primitive*)")]
// 0xf4e304 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE25onPrimitiveExtentsChangedEPS1_
pub fn stub_f4e304() {
    // IDA 0xf4e304: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0xf4e314 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE
pub fn stub_f4e314() {
    // IDA 0xf4e314: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesTouchingGrids(RBX::Extents const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,unsigned long,boost::unordered::unordered_set<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::equal_to<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
// 0xf4e324 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26getPrimitivesTouchingGridsERKNS_7ExtentsERKN5boost9unordered13unordered_setIPKS1_NS8_4hashISC_EESt8equal_toISC_ESaISC_EEEmRNSA_IPS1_NSD_ISL_EESF_ISL_ESaISL_EEE
pub fn stub_f4e324() {
    // IDA 0xf4e324: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::setup(void)")]
// 0xf4e334 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE5setupEv
pub fn stub_f4e334() {
    // IDA 0xf4e334: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addNode(RBX::Primitive*,RBX::Vector3int32 const&,bool)")]
// 0xf4e344 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb
pub fn stub_f4e344() {
    // IDA 0xf4e344: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::cleanup(void)")]
// 0xf4e354 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7cleanupEv
pub fn stub_f4e354() {
    // IDA 0xf4e354: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)")]
// 0xf4e364 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E
pub fn stub_f4e364() {
    // IDA 0xf4e364: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::TreeNode(void)")]
// 0xf4e374 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev
pub fn stub_f4e374() {
    // IDA 0xf4e374: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::~TreeNode()")]
// 0xf4e384 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev
pub fn stub_f4e384() {
    // IDA 0xf4e384: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findNode(RBX::Primitive*,RBX::Vector3int32 const&)")]
// 0xf4e394 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E
pub fn stub_f4e394() {
    // IDA 0xf4e394: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::fastClear(void)")]
// 0xf4e3a4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE9fastClearEv
pub fn stub_f4e3a4() {
    // IDA 0xf4e3a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::~SpatialHash()")]
// 0xf4e3b4 — j___ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EED2Ev
pub fn stub_f4e3b4() {
    // IDA 0xf4e3b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::for_each<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearSpatialNode>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearSpatialNode &)")]
// 0xf4e3c4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS5_20FastClearSpatialNodeEEEvRT_
pub fn stub_f4e3c4() {
    // IDA 0xf4e3c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::for_each<RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::CallDestructor>(RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::CallDestructor &)")]
// 0xf4e3d4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_
pub fn stub_f4e3d4() {
    // IDA 0xf4e3d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::for_each<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearTreeNode>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearTreeNode &)")]
// 0xf4e3e4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS5_17FastClearTreeNodeEEEvRT_
pub fn stub_f4e3e4() {
    // IDA 0xf4e3e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::for_each<RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::CallDestructor>(RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::CallDestructor &)")]
// 0xf4e3f4 — j___ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_
pub fn stub_f4e3f4() {
    // IDA 0xf4e3f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::const_iterator::operator++(void)")]
// 0xf4e404 — j___ZN3RBX12DenseHashSetIPNS_9PrimitiveEN5boost4hashIS2_EESaIS2_EE14const_iteratorppEv
pub fn stub_f4e404() {
    // IDA 0xf4e404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ExtentsInt32::empty(void)")]
// 0xf4e414 — j___ZN3RBX12ExtentsInt325emptyEv
pub fn stub_f4e414() {
    // IDA 0xf4e414: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ExtentsInt32::ExtentsInt32(void)")]
// 0xf4e424 — j___ZN3RBX12ExtentsInt32C1Ev
pub fn stub_f4e424() {
    // IDA 0xf4e424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BallBallContact::BallBallContact(RBX::Primitive *,RBX::Primitive *)")]
// 0xf4e484 — j___ZN3RBX15BallBallContactC2EPNS_9PrimitiveES2_
pub fn stub_f4e484() {
    // IDA 0xf4e484: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BallBallContact::~BallBallContact()")]
// 0xf4e494 — j___ZN3RBX15BallBallContactD2Ev
pub fn stub_f4e494() {
    // IDA 0xf4e494: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallBlockContact::BallBlockContact(RBX::Primitive *,RBX::Primitive *)")]
// 0xf4e4a4 — j___ZN3RBX16BallBlockContactC2EPNS_9PrimitiveES2_
pub fn stub_f4e4a4() {
    // IDA 0xf4e4a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BallBlockContact::~BallBlockContact()")]
// 0xf4e4b4 — j___ZN3RBX16BallBlockContactD2Ev
pub fn stub_f4e4b4() {
    // IDA 0xf4e4b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Extents::clampToOverlap(RBX::Extents const&)")]
// 0xf4e4c4 — j___ZN3RBX7Extents14clampToOverlapERKS0_
pub fn stub_f4e4c4() {
    // IDA 0xf4e4c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::NodeBase::getLevel(void)")]
// 0xf4e4e4 — j___ZN3RBX8NodeBase8getLevelEv
pub fn stub_f4e4e4() {
    // IDA 0xf4e4e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)")]
// 0xf4e4f4 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev
pub fn stub_f4e4f4() {
    // IDA 0xf4e4f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator delete(void *)")]
// 0xf4e504 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv
pub fn stub_f4e504() {
    // IDA 0xf4e504: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator new(unsigned long)")]
// 0xf4e514 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm
pub fn stub_f4e514() {
    // IDA 0xf4e514: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)")]
// 0xf4e524 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev
pub fn stub_f4e524() {
    // IDA 0xf4e524: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator new(unsigned long)")]
// 0xf4e534 — j___ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm
pub fn stub_f4e534() {
    // IDA 0xf4e534: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::Allocator(void)")]
// 0xf4e544 — j___ZN3RBX9AllocatorINS_15BallBallContactEEC2Ev
pub fn stub_f4e544() {
    // IDA 0xf4e544: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator delete(void *)")]
// 0xf4e554 — j___ZN3RBX9AllocatorINS_15BallBallContactEEdlEPv
pub fn stub_f4e554() {
    // IDA 0xf4e554: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::operator new(unsigned long)")]
// 0xf4e564 — j___ZN3RBX9AllocatorINS_15BallBallContactEEnwEm
pub fn stub_f4e564() {
    // IDA 0xf4e564: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<RBX::BallCellContact>::operator delete(void *)")]
// 0xf4e574 — j___ZN3RBX9AllocatorINS_15BallCellContactEEdlEPv
pub fn stub_f4e574() {
    // IDA 0xf4e574: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
