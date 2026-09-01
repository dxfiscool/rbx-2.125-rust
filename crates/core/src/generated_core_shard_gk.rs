//! core shard GK — 100 core stubs EA-sorted, 0xf4f3c4..0xf4fca4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf4f3b4).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf4f3b4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4f3c4 — j___ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4f3c4() -> ! {
    todo!("0xf4f3c4 j___ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4f3d4 — j___ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4f3d4() -> ! {
    todo!("0xf4f3d4 j___ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4f3e4 — j___ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4f3e4() -> ! {
    todo!("0xf4f3e4 j___ZN5boost14singleton_poolIN3RBX6D6LinkELj252ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "RBX::RevoluteLink::RevoluteLink(void)")]
// 0xf4f3f4 — j___ZN3RBX12RevoluteLinkC2Ev
pub fn stub_f4f3f4() -> ! {
    todo!("0xf4f3f4 j___ZN3RBX12RevoluteLinkC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::RevoluteLink>::Allocator(void)")]
// 0xf4f404 — j___ZN3RBX9AllocatorINS_12RevoluteLinkEEC2Ev
pub fn stub_f4f404() -> ! {
    todo!("0xf4f404 j___ZN3RBX9AllocatorINS_12RevoluteLinkEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::RevoluteLink>::operator delete(void *)")]
// 0xf4f414 — j___ZN3RBX9AllocatorINS_12RevoluteLinkEEdlEPv
pub fn stub_f4f414() -> ! {
    todo!("0xf4f414 j___ZN3RBX9AllocatorINS_12RevoluteLinkEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::RevoluteLink>::operator new(unsigned long)")]
// 0xf4f424 — j___ZN3RBX9AllocatorINS_12RevoluteLinkEEnwEm
pub fn stub_f4f424() -> ! {
    todo!("0xf4f424 j___ZN3RBX9AllocatorINS_12RevoluteLinkEEnwEm")
}

#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4f434 — j___ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4f434() -> ! {
    todo!("0xf4f434 j___ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4f444 — j___ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4f444() -> ! {
    todo!("0xf4f444 j___ZN5boost14singleton_poolIN3RBX12RevoluteLinkELj208ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::Allocator(void)")]
// 0xf4f4b4 — j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEC2Ev
pub fn stub_f4f4b4() -> ! {
    todo!("0xf4f4b4 j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator delete(void *)")]
// 0xf4f4c4 — j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEdlEPv
pub fn stub_f4f4c4() -> ! {
    todo!("0xf4f4c4 j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator new(unsigned long)")]
// 0xf4f4d4 — j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEnwEm
pub fn stub_f4f4d4() -> ! {
    todo!("0xf4f4d4 j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEnwEm")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4f504 — j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4f504() -> ! {
    todo!("0xf4f504 j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4f514 — j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4f514() -> ! {
    todo!("0xf4f514 j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "RBX::FixedArray<RBX::FaceFacePair::VertexStatus,40ul>::push_back(RBX::FaceFacePair::VertexStatus const&)")]
// 0xf4f5d4 — j___ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EE9push_backERKS2_
pub fn stub_f4f5d4() -> ! {
    todo!("0xf4f5d4 j___ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EE9push_backERKS2_")
}

#[doc(alias = "RBX::FixedArray<RBX::FaceFacePair::VertexStatus,40ul>::operator[](unsigned long)")]
// 0xf4f5e4 — j___ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EEixEm
pub fn stub_f4f5e4() -> ! {
    todo!("0xf4f5e4 j___ZN3RBX10FixedArrayINS_12FaceFacePair12VertexStatusELm40EEixEm")
}

#[doc(alias = "RBX::Allocator<RBX::PolyPolyContact>::Allocator(void)")]
// 0xf4f5f4 — j___ZN3RBX9AllocatorINS_15PolyPolyContactEEC2Ev
pub fn stub_f4f5f4() -> ! {
    todo!("0xf4f5f4 j___ZN3RBX9AllocatorINS_15PolyPolyContactEEC2Ev")
}

#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4f604 — j___ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4f604() -> ! {
    todo!("0xf4f604 j___ZN5boost14singleton_poolIN3RBX15PolyPolyContactELj216ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "RBX::BasicSpatialHashPrimitive::~BasicSpatialHashPrimitive()")]
// 0xf4f614 — j___ZN3RBX25BasicSpatialHashPrimitiveD2Ev
pub fn stub_f4f614() -> ! {
    todo!("0xf4f614 j___ZN3RBX25BasicSpatialHashPrimitiveD2Ev")
}

#[doc(alias = "RBX::Body::getPV_Spin_Lock(void)")]
// 0xf4f624 — j___ZN3RBX4Body15getPV_Spin_LockEv
pub fn stub_f4f624() -> ! {
    todo!("0xf4f624 j___ZN3RBX4Body15getPV_Spin_LockEv")
}

#[doc(alias = "RBX::Joint::isSpanningTreeJoint(RBX::Edge const*)")]
// 0xf4f634 — j___ZN3RBX5Joint19isSpanningTreeJointEPKNS_4EdgeE
pub fn stub_f4f634() -> ! {
    todo!("0xf4f634 j___ZN3RBX5Joint19isSpanningTreeJointEPKNS_4EdgeE")
}

#[doc(alias = "RBX::EdgeList::EdgeList(RBX::Primitive *)")]
// 0xf4f644 — j___ZN3RBX8EdgeListC2EPNS_9PrimitiveE
pub fn stub_f4f644() -> ! {
    todo!("0xf4f644 j___ZN3RBX8EdgeListC2EPNS_9PrimitiveE")
}

#[doc(alias = "RBX::EdgeList::~EdgeList()")]
// 0xf4f654 — j___ZN3RBX8EdgeListD2Ev
pub fn stub_f4f654() -> ! {
    todo!("0xf4f654 j___ZN3RBX8EdgeListD2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::operator new(unsigned long)")]
// 0xf4f664 — j___ZN3RBX9AllocatorINS_4BodyEEnwEm
pub fn stub_f4f664() -> ! {
    todo!("0xf4f664 j___ZN3RBX9AllocatorINS_4BodyEEnwEm")
}

#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4f674 — j___ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4f674() -> ! {
    todo!("0xf4f674 j___ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4f684 — j___ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4f684() -> ! {
    todo!("0xf4f684 j___ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "RBX::SurfaceData::isEmpty(void)const")]
// 0xf4f694 — j___ZNK3RBX11SurfaceData7isEmptyEv
pub fn stub_f4f694() -> ! {
    todo!("0xf4f694 j___ZNK3RBX11SurfaceData7isEmptyEv")
}

#[doc(alias = "RBX::EdgeList::getEdge(int)const")]
// 0xf4f6a4 — j___ZNK3RBX8EdgeList7getEdgeEi
pub fn stub_f4f6a4() -> ! {
    todo!("0xf4f6a4 j___ZNK3RBX8EdgeList7getEdgeEi")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
// 0xf4f6b4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
pub fn stub_f4f6b4() -> ! {
    todo!("0xf4f6b4 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
// 0xf4f6c4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
pub fn stub_f4f6c4() -> ! {
    todo!("0xf4f6c4 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
// 0xf4f6d4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
pub fn stub_f4f6d4() -> ! {
    todo!("0xf4f6d4 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
// 0xf4f6e4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
pub fn stub_f4f6e4() -> ! {
    todo!("0xf4f6e4 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
// 0xf4f6f4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
pub fn stub_f4f6f4() -> ! {
    todo!("0xf4f6f4 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")
}

#[doc(alias = "RBX::POLY::PrismMesh::PrismMesh(RBX::Vector3_2Ints const&)")]
// 0xf4f704 — j___ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE
pub fn stub_f4f704() -> ! {
    todo!("0xf4f704 j___ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::Allocator(void)")]
// 0xf4f714 — j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev
pub fn stub_f4f714() -> ! {
    todo!("0xf4f714 j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator delete(void *)")]
// 0xf4f724 — j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv
pub fn stub_f4f724() -> ! {
    todo!("0xf4f724 j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator new(unsigned long)")]
// 0xf4f734 — j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm
pub fn stub_f4f734() -> ! {
    todo!("0xf4f734 j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token *)")]
// 0xf4f744 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_
// was: boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token *)
pub fn stub_f4f744() -> ! {
    todo!("0xf4f744 j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token> const&)")]
// 0xf4f754 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_
// was: boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token> const&)
pub fn stub_f4f754() -> ! {
    todo!("0xf4f754 j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PrismMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4f764 — j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4f764() -> ! {
    todo!("0xf4f764 j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PrismMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4f774 — j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4f774() -> ! {
    todo!("0xf4f774 j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token *)")]
// 0xf4f784 — j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY9PrismMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_
pub fn stub_f4f784() -> ! {
    todo!("0xf4f784 j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY9PrismMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_")
}

#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
// 0xf4f794 — j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
pub fn stub_f4f794() -> ! {
    todo!("0xf4f794 j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
// 0xf4f7a4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
pub fn stub_f4f7a4() -> ! {
    todo!("0xf4f7a4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
// 0xf4f7b4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
pub fn stub_f4f7b4() -> ! {
    todo!("0xf4f7b4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
// 0xf4f7c4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
pub fn stub_f4f7c4() -> ! {
    todo!("0xf4f7c4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
// 0xf4f7d4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
pub fn stub_f4f7d4() -> ! {
    todo!("0xf4f7d4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
// 0xf4f7e4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
pub fn stub_f4f7e4() -> ! {
    todo!("0xf4f7e4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
// 0xf4f7f4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
pub fn stub_f4f7f4() -> ! {
    todo!("0xf4f7f4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
// 0xf4f804 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
pub fn stub_f4f804() -> ! {
    todo!("0xf4f804 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
// 0xf4f814 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
pub fn stub_f4f814() -> ! {
    todo!("0xf4f814 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
// 0xf4f824 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
pub fn stub_f4f824() -> ! {
    todo!("0xf4f824 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
// 0xf4f834 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
pub fn stub_f4f834() -> ! {
    todo!("0xf4f834 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
// 0xf4f844 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
pub fn stub_f4f844() -> ! {
    todo!("0xf4f844 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
// 0xf4f854 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
pub fn stub_f4f854() -> ! {
    todo!("0xf4f854 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
// 0xf4f864 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
pub fn stub_f4f864() -> ! {
    todo!("0xf4f864 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
// 0xf4f874 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
pub fn stub_f4f874() -> ! {
    todo!("0xf4f874 j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")
}

#[doc(alias = "RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&)")]
// 0xf4f884 — j___ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE
pub fn stub_f4f884() -> ! {
    todo!("0xf4f884 j___ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator(void)")]
// 0xf4f894 — j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev
pub fn stub_f4f894() -> ! {
    todo!("0xf4f894 j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator delete(void *)")]
// 0xf4f8a4 — j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv
pub fn stub_f4f8a4() -> ! {
    todo!("0xf4f8a4 j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long)")]
// 0xf4f8b4 — j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm
pub fn stub_f4f8b4() -> ! {
    todo!("0xf4f8b4 j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token *)")]
// 0xf4f8c4 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_
// was: boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token *)
pub fn stub_f4f8c4() -> ! {
    todo!("0xf4f8c4 j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token> const&)")]
// 0xf4f8d4 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_
// was: boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token> const&)
pub fn stub_f4f8d4() -> ! {
    todo!("0xf4f8d4 j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PyramidMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4f8e4 — j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4f8e4() -> ! {
    todo!("0xf4f8e4 j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PyramidMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4f8f4 — j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4f8f4() -> ! {
    todo!("0xf4f8f4 j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token *)")]
// 0xf4f904 — j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY11PyramidMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_
pub fn stub_f4f904() -> ! {
    todo!("0xf4f904 j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY11PyramidMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_")
}

#[doc(alias = "RBX::Vector3_2IntsComparer::operator()(RBX::Vector3_2Ints const&,RBX::Vector3_2Ints const&)const")]
// 0xf4f914 — j___ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_
pub fn stub_f4f914() -> ! {
    todo!("0xf4f914 j___ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_")
}

#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
// 0xf4f924 — j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
pub fn stub_f4f924() -> ! {
    todo!("0xf4f924 j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
// 0xf4f934 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
pub fn stub_f4f934() -> ! {
    todo!("0xf4f934 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
// 0xf4f944 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
pub fn stub_f4f944() -> ! {
    todo!("0xf4f944 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
// 0xf4f954 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
pub fn stub_f4f954() -> ! {
    todo!("0xf4f954 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
// 0xf4f964 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
pub fn stub_f4f964() -> ! {
    todo!("0xf4f964 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
// 0xf4f974 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
pub fn stub_f4f974() -> ! {
    todo!("0xf4f974 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
// 0xf4f984 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
pub fn stub_f4f984() -> ! {
    todo!("0xf4f984 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
// 0xf4f994 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
pub fn stub_f4f994() -> ! {
    todo!("0xf4f994 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
// 0xf4f9a4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
pub fn stub_f4f9a4() -> ! {
    todo!("0xf4f9a4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
// 0xf4f9b4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
pub fn stub_f4f9b4() -> ! {
    todo!("0xf4f9b4 j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator(void)")]
// 0xf4fa24 — j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev
pub fn stub_f4fa24() -> ! {
    todo!("0xf4fa24 j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void *)")]
// 0xf4fa34 — j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv
pub fn stub_f4fa34() -> ! {
    todo!("0xf4fa34 j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long)")]
// 0xf4fa44 — j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm
pub fn stub_f4fa44() -> ! {
    todo!("0xf4fa44 j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::RightAngleRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4fa74 — j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4fa74() -> ! {
    todo!("0xf4fa74 j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::RightAngleRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4fa84 — j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4fa84() -> ! {
    todo!("0xf4fa84 j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "RBX::SendPhysics::nextSimJob(RBX::SimJob *)")]
// 0xf4fb44 — j___ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE
pub fn stub_f4fb44() -> ! {
    todo!("0xf4fb44 j___ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE")
}

#[doc(alias = "unsigned long RBX::fastRemoveShort<RBX::SimJobTracker *>(std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>> &,RBX::SimJobTracker * const&)")]
// 0xf4fb54 — j___ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_
pub fn stub_f4fb54() -> ! {
    todo!("0xf4fb54 j___ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_")
}

#[doc(alias = "std::_Vector_base<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_allocate(unsigned long)")]
// 0xf4fb64 — j___ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm
pub fn stub_f4fb64() -> ! {
    todo!("0xf4fb64 j___ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&)")]
// 0xf4fb74 — j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4fb74() -> ! {
    todo!("0xf4fb74 j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,unsigned long,RBX::SimJobTracker * const&)")]
// 0xf4fb84 — j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f4fb84() -> ! {
    todo!("0xf4fb84 j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::resize(unsigned long,RBX::SimJobTracker *)")]
// 0xf4fb94 — j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_
pub fn stub_f4fb94() -> ! {
    todo!("0xf4fb94 j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::push_back(RBX::SimJobTracker * const&)")]
// 0xf4fba4 — j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_
pub fn stub_f4fba4() -> ! {
    todo!("0xf4fba4 j___ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker *>(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&,std::random_access_iterator_tag)")]
// 0xf4fbb4 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_f4fbb4() -> ! {
    todo!("0xf4fbb4 j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "RBX::Assembly * RBX::IndexedTree::getOneBelowRoot<RBX::Assembly>(void)")]
// 0xf4fbc4 — j___ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v
pub fn stub_f4fbc4() -> ! {
    todo!("0xf4fbc4 j___ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert_unique(std::pair<RBX::Assembly * const,int> const&)")]
// 0xf4fbd4 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_f4fbd4() -> ! {
    todo!("0xf4fbd4 j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,int>> *)")]
// 0xf4fbe4 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f4fbe4() -> ! {
    todo!("0xf4fbe4 j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Assembly * const,int> const&)")]
// 0xf4fbf4 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_f4fbf4() -> ! {
    todo!("0xf4fbf4 j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc>::fastRemove(RBX::Contact*)")]
// 0xf4fc54 — j___ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_
pub fn stub_f4fc54() -> ! {
    todo!("0xf4fc54 j___ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::Body::resetForceAccumulators(void)")]
// 0xf4fc64 — j___ZN3RBX4Body22resetForceAccumulatorsEv
pub fn stub_f4fc64() -> ! {
    todo!("0xf4fc64 j___ZN3RBX4Body22resetForceAccumulatorsEv")
}

#[doc(alias = "RBX::Body::resetImpulseAccumulators(void)")]
// 0xf4fc74 — j___ZN3RBX4Body24resetImpulseAccumulatorsEv
pub fn stub_f4fc74() -> ! {
    todo!("0xf4fc74 j___ZN3RBX4Body24resetImpulseAccumulatorsEv")
}

#[doc(alias = "RBX::IPipelined::inOrDownstreamOfStage(RBX::IStage *)const")]
// 0xf4fc84 — j___ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE
pub fn stub_f4fc84() -> ! {
    todo!("0xf4fc84 j___ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE")
}

#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate_map(unsigned long)")]
// 0xf4fc94 — j___ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_allocate_mapEm
pub fn stub_f4fc94() -> ! {
    todo!("0xf4fc94 j___ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_create_nodes(RBX::Assembly ***,RBX::Assembly ***)")]
// 0xf4fca4 — j___ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_f4fca4() -> ! {
    todo!("0xf4fca4 j___ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_create_nodesEPPS2_S6_")
}
