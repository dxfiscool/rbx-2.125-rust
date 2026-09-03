//! rendering shard 474 — 120 stubs 0xf4ece4..0xf6ac94 EA-sorted asc filtered Ogre|G3D|Render|Adorn|View|Mesh (17333 total, 17212->17332 covered, 1 remaining)
//! Source: ida/export.json (85545 funcs) EA asc filtered not yet in rendering — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf4ece4 — j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev")]
// IDA 0xf4ece4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ece4() {
}

// 0xf4ecf4 — j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv")]
// IDA 0xf4ecf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ecf4() {
}

// 0xf4ed04 — j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm")]
// IDA 0xf4ed04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ed04() {
}

// 0xf4ed34 — j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0xf4ed34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ed34() {
}

// 0xf4ed44 — j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0xf4ed44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ed44() {
}

// 0xf4ef24 — j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::PyramidMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0xf4ef24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ef24() {
}

// 0xf4ef34 — j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0xf4ef34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ef34() {
}

// 0xf4ef44 — j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0xf4ef44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ef44() {
}

// 0xf4ef54 — j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::RightAngleRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0xf4ef54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ef54() {
}

// 0xf4ef64 — j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::POLY::PrismMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0xf4ef64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ef64() {
}

// 0xf4ef74 — j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0xf4ef74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ef74() {
}

// 0xf4f4b4 — j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEC2Ev")]
// IDA 0xf4f4b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f4b4() {
}

// 0xf4f4c4 — j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEdlEPv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEdlEPv")]
// IDA 0xf4f4c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f4c4() {
}

// 0xf4f4d4 — j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEnwEm
// type: int __fastcall(unsigned int)
#[doc(alias = "RBX::Allocator<RBX::POLY::ParallelRampMesh>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY16ParallelRampMeshEEnwEm")]
// IDA 0xf4f4d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f4d4() {
}

// 0xf4f504 — j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0xf4f504: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f504() {
}

// 0xf4f514 — j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0xf4f514: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f514() {
}

// 0xf4f6b4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
// IDA 0xf4f6b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f6b4() {
}

// 0xf4f6c4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
// IDA 0xf4f6c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4f6c4() {
}

// 0xf4f6d4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
// IDA 0xf4f6d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f6d4() {
}

// 0xf4f6e4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
// IDA 0xf4f6e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f6e4() {
}

// 0xf4f6f4 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")]
// IDA 0xf4f6f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f6f4() {
}

// 0xf4f704 — j___ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::POLY::PrismMesh::PrismMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE")]
// IDA 0xf4f704: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f704() {
}

// 0xf4f714 — j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev
// type: int()
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev")]
// IDA 0xf4f714: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f714() {
}

// 0xf4f724 — j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv")]
// IDA 0xf4f724: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f724() {
}

// 0xf4f734 — j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm
// type: int()
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm")]
// IDA 0xf4f734: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f734() {
}

// 0xf4f744 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_")]
// IDA 0xf4f744: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f744() {
}

// 0xf4f754 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_
// type: int()
#[doc(alias = "boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_")]
// IDA 0xf4f754: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f754() {
}

// 0xf4f764 — j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::POLY::PrismMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0xf4f764: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f764() {
}

// 0xf4f774 — j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::POLY::PrismMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0xf4f774: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f774() {
}

// 0xf4f784 — j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY9PrismMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY9PrismMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_")]
// IDA 0xf4f784: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f784() {
}

// 0xf4f794 — j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
// type: int()
#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
// IDA 0xf4f794: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f794() {
}

// 0xf4f7a4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
// IDA 0xf4f7a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f7a4() {
}

// 0xf4f7b4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
// IDA 0xf4f7b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f7b4() {
}

// 0xf4f7c4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
// IDA 0xf4f7c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f7c4() {
}

// 0xf4f7d4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
// IDA 0xf4f7d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f7d4() {
}

// 0xf4f7e4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
// IDA 0xf4f7e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f7e4() {
}

// 0xf4f7f4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
// IDA 0xf4f7f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f7f4() {
}

// 0xf4f804 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
// IDA 0xf4f804: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f804() {
}

// 0xf4f814 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: int()
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// IDA 0xf4f814: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f814() {
}

// 0xf4f824 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
// IDA 0xf4f824: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f824() {
}

// 0xf4f834 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
// IDA 0xf4f834: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f834() {
}

// 0xf4f844 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
// IDA 0xf4f844: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4f844() {
}

// 0xf4f854 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
// IDA 0xf4f854: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f854() {
}

// 0xf4f864 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
// IDA 0xf4f864: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f864() {
}

// 0xf4f874 — j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")]
// IDA 0xf4f874: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f874() {
}

// 0xf4f884 — j___ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE")]
// IDA 0xf4f884: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f884() {
}

// 0xf4f894 — j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev
// type: int()
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev")]
// IDA 0xf4f894: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f894() {
}

// 0xf4f8a4 — j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv")]
// IDA 0xf4f8a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f8a4() {
}

// 0xf4f8b4 — j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm
// type: int()
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm")]
// IDA 0xf4f8b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f8b4() {
}

// 0xf4f8c4 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_
#[doc(alias = "boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEC2IS8_EEPT_")]
// IDA 0xf4f8c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f8c4() {
}

// 0xf4f8d4 — j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_
// type: int()
#[doc(alias = "boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY11PyramidMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_")]
// IDA 0xf4f8d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f8d4() {
}

// 0xf4f8e4 — j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::POLY::PyramidMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0xf4f8e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f8e4() {
}

// 0xf4f8f4 — j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::POLY::PyramidMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0xf4f8f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f8f4() {
}

// 0xf4f904 — j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY11PyramidMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token>(RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::Token *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolINS3_13Vector3_2IntsENS3_4POLY11PyramidMeshENS3_21Vector3_2IntsComparerEE5TokenEEEPT_")]
// IDA 0xf4f904: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f904() {
}

// 0xf4f924 — j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
// IDA 0xf4f924: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f924() {
}

// 0xf4f934 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
// IDA 0xf4f934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f934() {
}

// 0xf4f944 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
// IDA 0xf4f944: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f944() {
}

// 0xf4f954 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
// IDA 0xf4f954: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f954() {
}

// 0xf4f964 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
// IDA 0xf4f964: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f964() {
}

// 0xf4f974 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
// IDA 0xf4f974: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f974() {
}

// 0xf4f984 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
// IDA 0xf4f984: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f984() {
}

// 0xf4f994 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
// IDA 0xf4f994: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f994() {
}

// 0xf4f9a4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// IDA 0xf4f9a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f9a4() {
}

// 0xf4f9b4 — j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
// IDA 0xf4f9b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4f9b4() {
}

// 0xf4fa24 — j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev")]
// IDA 0xf4fa24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4fa24() {
}

// 0xf4fa34 — j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv")]
// IDA 0xf4fa34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4fa34() {
}

// 0xf4fa44 — j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm")]
// IDA 0xf4fa44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4fa44() {
}

// 0xf4fa74 — j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::RightAngleRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0xf4fa74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4fa74() {
}

// 0xf4fa84 — j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::RightAngleRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0xf4fa84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4fa84() {
}

// 0xf4ffa4 — j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev")]
// IDA 0xf4ffa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ffa4() {
}

// 0xf4ffb4 — j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv")]
// IDA 0xf4ffb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ffb4() {
}

// 0xf4ffc4 — j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm")]
// IDA 0xf4ffc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ffc4() {
}

// 0xf4fff4 — j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int()
#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0xf4fff4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4fff4() {
}

// 0xf50004 — j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0xf50004: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f50004() {
}

// 0xf52fe4 — j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_13DataModelMeshEEEPKT_v
#[doc(alias = "RBX::DataModelMesh const* RBX::Instance::findConstFirstChildOfType<RBX::DataModelMesh>(void)const")]
#[doc(alias = "j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_13DataModelMeshEEEPKT_v")]
// IDA 0xf52fe4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f52fe4() {
}

// 0xf53e34 — j___ZN5boost10shared_ptrIvE5resetIN3RBX12FileMeshDataEEEvPT_
#[doc(alias = "void boost::shared_ptr<void>::reset<RBX::FileMeshData>(RBX::FileMeshData *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIvE5resetIN3RBX12FileMeshDataEEEvPT_")]
// IDA 0xf53e34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f53e34() {
}

// 0xf53e44 — j___ZN5boost10shared_ptrIvEC2IN3RBX12FileMeshDataEEEPT_
#[doc(alias = "boost::shared_ptr<void>::shared_ptr<RBX::FileMeshData>(RBX::FileMeshData *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIvEC2IN3RBX12FileMeshDataEEEPT_")]
// IDA 0xf53e44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f53e44() {
}

// 0xf53e54 — j___ZN5boost6detail12shared_countC2IN3RBX12FileMeshDataEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FileMeshData>(RBX::FileMeshData *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX12FileMeshDataEEEPT_")]
// IDA 0xf53e54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f53e54() {
}

// 0xf57df4 — j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::Allocator(void)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEC2Ev")]
// IDA 0xf57df4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f57df4() {
}

// 0xf57e04 — j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator delete(void *)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEdlEPv")]
// IDA 0xf57e04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f57e04() {
}

// 0xf57e14 — j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator new(unsigned long)")]
#[doc(alias = "j___ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEnwEm")]
// IDA 0xf57e14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f57e14() {
}

// 0xf57e44 — j___ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0xf57e44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f57e44() {
}

// 0xf57e54 — j___ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0xf57e54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f57e54() {
}

// 0xf60a34 — j___ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev")]
#[doc(alias = "j___ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0xf60a34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f60a34() {
}

// 0xf60be4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, RBX::Instance *, boost::detail::shared_count *, int, int, void *, int)
#[doc(alias = "boost::shared_ptr<RBX::CylinderMesh> RBX::Creatable<RBX::Instance>::create<RBX::CylinderMesh>(void)")]
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_12CylinderMeshEEEN5boost10shared_ptrIT_EEv")]
// IDA 0xf60be4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f60be4() {
}

// 0xf63ca4 — j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE12getClassNameEv")]
// IDA 0xf63ca4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f63ca4() {
}

// 0xf63cb4 — j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv")]
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0xf63cb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f63cb4() {
}

// 0xf63f24 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BlockMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BlockMesh,RBX::BlockMesh>(boost::shared_ptr<RBX::BlockMesh> const*,RBX::BlockMesh *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BlockMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0xf63f24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f63f24() {
}

// 0xf64b04 — j___ZN3RBX24FastClusterMeshGenerator10isPartHeadEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::FastClusterMeshGenerator *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isPartHead(RBX::PartInstance *)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator10isPartHeadEPNS_12PartInstanceE")]
// IDA 0xf64b04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64b04() {
}

// 0xf64b14 — j___ZN3RBX24FastClusterMeshGenerator11addInstanceEmPNS_12PartInstanceEPNS_5DecalEjNS_22eShadowCullingPriorityEPNS_11AsyncResultE
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::FastClusterMeshGenerator::addInstance(unsigned long,RBX::PartInstance *,RBX::Decal *,unsigned int,RBX::eShadowCullingPriority,RBX::AsyncResult *)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator11addInstanceEmPNS_12PartInstanceEPNS_5DecalEjNS_22eShadowCullingPriorityEPNS_11AsyncResultE")]
// IDA 0xf64b14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64b14() {
}

// 0xf64b44 — j___ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE
// type: int __fastcall(int, bool *, int, _DWORD *)
#[doc(alias = "RBX::FastClusterMeshGenerator::finalizeMerged(RBX::FastCluster *,RBX::eShadowCullingPriority,RBX::FastClusterSharedGeometry &)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator14finalizeMergedEPNS_11FastClusterENS_22eShadowCullingPriorityERNS_25FastClusterSharedGeometryE")]
// IDA 0xf64b44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64b44() {
}

// 0xf64b54 — j___ZN3RBX24FastClusterMeshGenerator15createIndexDataEj
// type: _DWORD __fastcall(RBX::FastClusterMeshGenerator *__hidden this, unsigned int)
#[doc(alias = "RBX::FastClusterMeshGenerator::createIndexData(unsigned int)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator15createIndexDataEj")]
// IDA 0xf64b54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64b54() {
}

// 0xf64b74 — j___ZN3RBX24FastClusterMeshGenerator16isPartCompositedEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::FastClusterMeshGenerator *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isPartComposited(RBX::PartInstance *)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator16isPartCompositedEPNS_12PartInstanceE")]
// IDA 0xf64b74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64b74() {
}

// 0xf64b84 — j___ZN3RBX24FastClusterMeshGenerator18generateShadowDataERKNS0_5BatchEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEEb
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *, int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
#[doc(alias = "RBX::FastClusterMeshGenerator::generateShadowData(RBX::FastClusterMeshGenerator::Batch const&,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,bool)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator18generateShadowDataERKNS0_5BatchEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEEb")]
// IDA 0xf64b84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64b84() {
}

// 0xf64ba4 — j___ZN3RBX24FastClusterMeshGenerator20getRelativeTransformEPNS_12PartInstanceES2_
// type: void __fastcall(RBX::FastClusterMeshGenerator *this, RBX::PartInstance *, RBX::PartInstance *, RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::getRelativeTransform(RBX::PartInstance *,RBX::PartInstance *)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator20getRelativeTransformEPNS_12PartInstanceES2_")]
// IDA 0xf64ba4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64ba4() {
}

// 0xf64bb4 — j___ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb
// type: _DWORD __fastcall(RBX::FastClusterMeshGenerator *__hidden this, bool)
#[doc(alias = "RBX::FastClusterMeshGenerator::getVertexDeclaration(bool)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator20getVertexDeclarationEb")]
// IDA 0xf64bb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64bb4() {
}

// 0xf64bc4 — j___ZN3RBX24FastClusterMeshGenerator20isBodyPartCompositedEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::FastClusterMeshGenerator *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::isBodyPartComposited(RBX::PartInstance *)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator20isBodyPartCompositedEPNS_12PartInstanceE")]
// IDA 0xf64bc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64bc4() {
}

// 0xf64be4 — j___ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_
// type: int __fastcall(int, __int64 *)
#[doc(alias = "RBX::FastClusterMeshGenerator::Batch::Batch(RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator5BatchC2ERKS1_")]
// IDA 0xf64be4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64be4() {
}

// 0xf64bf4 — j___ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::FastClusterMeshGenerator *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE")]
// IDA 0xf64bf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64bf4() {
}

// 0xf64c34 — j___ZN3RBX24FastClusterMeshGeneratorD1Ev
// type: void __fastcall(RBX::FastClusterMeshGenerator *__hidden this)
#[doc(alias = "RBX::FastClusterMeshGenerator::~FastClusterMeshGenerator()")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGeneratorD1Ev")]
// IDA 0xf64c34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f64c34() {
}

// 0xf64c84 — j___ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_List_base<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::~_List_base()")]
#[doc(alias = "j___ZNSt10_List_baseIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EED2Ev")]
// IDA 0xf64c84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f64c84() {
}

// 0xf64c94 — j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX24FastClusterMeshGenerator13BatchInstanceES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance * std::__copy_backward_normal<false,false>::__copy_b_n<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *)")]
#[doc(alias = "j___ZNSt22__copy_backward_normalILb0ELb0EE10__copy_b_nIPN3RBX24FastClusterMeshGenerator13BatchInstanceES5_EET0_T_S7_S6_")]
// IDA 0xf64c94: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f64c94() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf64cb4 — j___ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<RBX::FastClusterMeshGenerator::Batch,std::allocator<RBX::FastClusterMeshGenerator::Batch>>::_M_insert(std::_List_iterator<RBX::FastClusterMeshGenerator::Batch>,RBX::FastClusterMeshGenerator::Batch const&)")]
#[doc(alias = "j___ZNSt4listIN3RBX24FastClusterMeshGenerator5BatchESaIS2_EE9_M_insertESt14_List_iteratorIS2_ERKS2_")]
// IDA 0xf64cb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64cb4() {
}

// 0xf64d14 — j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0xf64d14: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f64d14() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf64d24 — j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EEC2ERKS4_
// type: int __fastcall(int *, int *, int, int)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>::vector(std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>> const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator13BatchInstanceESaIS2_EEC2ERKS4_")]
// IDA 0xf64d24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64d24() {
}

// 0xf64d34 — j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(_DWORD)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::Bone*,std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>>,RBX::FastClusterMeshGenerator::Bone const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0xf64d34: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f64d34() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf64d44 — j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::FastClusterMeshGenerator::Bone,std::allocator<RBX::FastClusterMeshGenerator::Bone>>::reserve(unsigned long)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX24FastClusterMeshGenerator4BoneESaIS2_EE7reserveEm")]
// IDA 0xf64d44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64d44() {
}

// 0xf64e94 — j___ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX24FastClusterMeshGenerator13BatchInstanceESt6vectorIS4_SaIS4_EEEEPS4_ET0_T_SD_SC_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance* std::__uninitialized_copy_aux<__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*>(__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,__gnu_cxx::__normal_iterator<RBX::FastClusterMeshGenerator::BatchInstance const*,std::vector<RBX::FastClusterMeshGenerator::BatchInstance,std::allocator<RBX::FastClusterMeshGenerator::BatchInstance>>>,RBX::FastClusterMeshGenerator::BatchInstance*,std::__false_type)")]
#[doc(alias = "j___ZSt24__uninitialized_copy_auxIN9__gnu_cxx17__normal_iteratorIPKN3RBX24FastClusterMeshGenerator13BatchInstanceESt6vectorIS4_SaIS4_EEEEPS4_ET0_T_SD_SC_St12__false_type")]
// IDA 0xf64e94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64e94() {
}

// 0xf64ea4 — j___ZSt24__uninitialized_copy_auxIPN3RBX24FastClusterMeshGenerator13BatchInstanceES3_ET0_T_S5_S4_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "RBX::FastClusterMeshGenerator::BatchInstance * std::__uninitialized_copy_aux<RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *>(RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,RBX::FastClusterMeshGenerator::BatchInstance *,std::__false_type)")]
#[doc(alias = "j___ZSt24__uninitialized_copy_auxIPN3RBX24FastClusterMeshGenerator13BatchInstanceES3_ET0_T_S5_S4_St12__false_type")]
// IDA 0xf64ea4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64ea4() {
}

// 0xf64f74 — j___ZN3RBX13DataModelUtil11getFileMeshEPNS_13DataModelMeshE
// type: _DWORD __fastcall(RBX::DataModelUtil *__hidden this, RBX::DataModelMesh *)
#[doc(alias = "RBX::DataModelUtil::getFileMesh(RBX::DataModelMesh *)")]
#[doc(alias = "j___ZN3RBX13DataModelUtil11getFileMeshEPNS_13DataModelMeshE")]
// IDA 0xf64f74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f64f74() {
}

// 0xf65884 — j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v
#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")]
#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")]
// IDA 0xf65884: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f65884() {
}

// 0xf66054 — j___ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_8FileMeshELZNS_9sFileMeshEENS_14FactoryProductIS2_NS_13DataModelMeshELZNS_9sFileMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// IDA 0xf66054: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66054() {
}

// 0xf66074 — j___ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_9BevelMeshELZNS_10sBevelMeshEENS_17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// IDA 0xf66074: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66074() {
}

// 0xf660d4 — j___ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "j___ZN3RBX14FactoryProductINS_12CylinderMeshENS_9BevelMeshELZNS_13sCylinderMeshEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0xf660d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f660d4() {
}

// 0xf660e4 — j___ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev")]
#[doc(alias = "j___ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0xf660e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f660e4() {
}

// 0xf660f4 — j___ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "j___ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "j___ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0xf660f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f660f4() {
}

// 0xf66104 — j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v
// type: int(void)
#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")]
#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")]
// IDA 0xf66104: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66104() {
}

// 0xf664b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CylinderMesh,RBX::CylinderMesh>(boost::shared_ptr<RBX::CylinderMesh> const*,RBX::CylinderMesh *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0xf664b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f664b4() {
}

// 0xf6ac84 — j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,RBX::FileMeshFace const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// IDA 0xf6ac84: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f6ac84() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf6ac94 — j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,unsigned long,RBX::FileMeshFace const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// IDA 0xf6ac94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f6ac94() {
}
