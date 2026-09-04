//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe34f7c..0xe3cd18 (100 stubs, 9460 prior -> 9560 covered, 3773 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe34f7c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe34f7c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e34f7c() {
}

// 0xe35020 — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe35020: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e35020() {
}

// 0xe35024 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe35024: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e35024() {
}

// 0xe35028 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe35028: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e35028() {
}

// 0xe35034 — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe35034: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e35034() {
}

// 0xe35040 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)
// IDA 0xe35040: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35040() {
}

// 0xe35220 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)
// IDA 0xe35220: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35220() {
}

// 0xe35374 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *> const&)
// IDA 0xe35374: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35374() {
}

// 0xe35458 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe35458: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35458() {
}

// 0xe354fc — __ZNSt6vectorIPN4Ogre14StaticGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)")]
// was: std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)
// IDA 0xe354fc: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e354fc() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe355f4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>> *)
// IDA 0xe355f4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e355f4() {
}

// 0xe3566c — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe3566c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e3566c() {
}

// 0xe35670 — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe35670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e35670() {
}

// 0xe3567c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe3567c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e3567c() {
}

// 0xe35680 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe35680: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e35680() {
}

// 0xe3568c — __ZNSt6vectorIPN4Ogre14StaticGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)")]
// was: std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)
// IDA 0xe3568c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e3568c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe35784 — __ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS6_EERKf
#[doc(alias = "std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)")]
// was: std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)
// IDA 0xe35784: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e35784() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe35888 — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe35888: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e35888() {
}

// 0xe3588c — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe3588c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3588c() {
}

// 0xe35898 — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISF_E
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)
// IDA 0xe35898: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35898() {
}

// 0xe358c0 — __ZNSt12_Vector_baseIN4Ogre14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe358c0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e358c0() {
}

// 0xe358c4 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>> *)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>> *)
// IDA 0xe358c4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e358c4() {
}

// 0xe358ec — __ZNSt8_Rb_treeImSt4pairIKmmESt10_Select1stIS2_ESt4lessImEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,unsigned long> const&)")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long const,unsigned long> const&)
// IDA 0xe358ec: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e358ec() {
}

// 0xe359e8 — __ZNSt8_Rb_treeImSt4pairIKmmESt10_Select1stIS2_ESt4lessImEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,unsigned long>> *)")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,unsigned long>> *)
// IDA 0xe359e8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e359e8() {
}

// 0xe35a10 — __ZNSt8_Rb_treeImSt4pairIKmmESt10_Select1stIS2_ESt4lessImEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()
// IDA 0xe35a10: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e35a10() {
}

// 0xe35a14 — __ZNSt8_Rb_treeImSt4pairIKmmESt10_Select1stIS2_ESt4lessImEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,unsigned long>,std::_Select1st<std::pair<unsigned long const,unsigned long>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,unsigned long>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()
// IDA 0xe35a14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e35a14() {
}

// 0xe35a20 — __ZNSt6vectorIN4Ogre14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)")]
// was: std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)
// IDA 0xe35a20: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35a20() {
}

// 0xe35c08 — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISF_ERKSF_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xe35c08: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35c08() {
}

// 0xe35e10 — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueERKSF_
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xe35e10: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e35e10() {
}

// 0xe35f0c — __ZNSt12_Vector_baseIN4Ogre14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe35f0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e35f0c() {
}

// 0xe35f18 — __ZNSt6vectorIPN4Ogre14StaticGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)")]
// was: std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)
// IDA 0xe35f18: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e35f18() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe36010 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)
// IDA 0xe36010: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36010() {
}

// 0xe36218 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::StaticGeometry::Region *> const&)
// IDA 0xe36218: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36218() {
}

// 0xe36314 — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe36314: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e36314() {
}

// 0xe36318 — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()
// IDA 0xe36318: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e36318() {
}

// 0xe3631c — __ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre14StaticGeometry6RegionEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,std::_Select1st<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::StaticGeometry::Region *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned int>,false>::~_Rb_tree_impl()
// IDA 0xe3631c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3631c() {
}

// 0xe36328 — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE13_Rb_tree_implISJ_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::SubMesh *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::SubMesh *>,false>::~_Rb_tree_impl()
// IDA 0xe36328: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e36328() {
}

// 0xe3632c — __ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE13_Rb_tree_implISJ_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::SubMesh *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::SubMesh *>,false>::~_Rb_tree_impl()
// IDA 0xe3632c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3632c() {
}

// 0xe36338 — __ZNSt10_List_baseIPN4Ogre14StaticGeometry24OptimisedSubMeshGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe36338: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e36338() {
}

// 0xe3633c — __ZNSt10_List_baseIPN4Ogre14StaticGeometry24OptimisedSubMeshGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::OptimisedSubMeshGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe3633c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3633c() {
}

// 0xe36348 — __ZNSt12_Vector_baseIPN4Ogre14StaticGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe36348: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e36348() {
}

// 0xe36354 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14StaticGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>> *)
// IDA 0xe36354: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36354() {
}

// 0xe36400 — __ZN4Ogre10StringUtil4trimERSsbb
#[doc(alias = "Ogre::StringUtil::trim(std::string &,bool,bool)")]
// was: Ogre::StringUtil::trim(std::string &,bool,bool)
// IDA 0xe36400: 116 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36400() {
}

// 0xe36570 — __ZN4Ogre10StringUtil5splitERKSsS2_jb
#[doc(alias = "Ogre::StringUtil::split(std::string const&,std::string const&,unsigned int,bool)")]
// was: Ogre::StringUtil::split(std::string const&,std::string const&,unsigned int,bool)
// IDA 0xe36570: 514 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36570() {
}

// 0xe36acc — __ZN4Ogre10StringUtil8tokeniseERKSsS2_S2_j
#[doc(alias = "Ogre::StringUtil::tokenise(std::string const&,std::string const&,std::string const&,unsigned int)")]
// was: Ogre::StringUtil::tokenise(std::string const&,std::string const&,std::string const&,unsigned int)
// IDA 0xe36acc: 450 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36acc() {
}

// 0xe36f88 — __ZN4Ogre10StringUtil11toLowerCaseERSs
#[doc(alias = "Ogre::StringUtil::toLowerCase(std::string &)")]
// was: Ogre::StringUtil::toLowerCase(std::string &)
// IDA 0xe36f88: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36f88() {
}

// 0xe36ff8 — __ZN4Ogre10StringUtil11toUpperCaseERSs
#[doc(alias = "Ogre::StringUtil::toUpperCase(std::string &)")]
// was: Ogre::StringUtil::toUpperCase(std::string &)
// IDA 0xe36ff8: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e36ff8() {
}

// 0xe37068 — __ZN4Ogre10StringUtil10startsWithERKSsS2_b
#[doc(alias = "Ogre::StringUtil::startsWith(std::string const&,std::string const&,bool)")]
// was: Ogre::StringUtil::startsWith(std::string const&,std::string const&,bool)
// IDA 0xe37068: 135 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e37068() {
}

// 0xe371e4 — __ZN4Ogre10StringUtil8endsWithERKSsS2_b
#[doc(alias = "Ogre::StringUtil::endsWith(std::string const&,std::string const&,bool)")]
// was: Ogre::StringUtil::endsWith(std::string const&,std::string const&,bool)
// IDA 0xe371e4: 143 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e371e4() {
}

// 0xe3737c — __ZN4Ogre10StringUtil13splitFilenameERKSsRSsS3_
#[doc(alias = "Ogre::StringUtil::splitFilename(std::string const&,std::string &,std::string &)")]
// was: Ogre::StringUtil::splitFilename(std::string const&,std::string &,std::string &)
// IDA 0xe3737c: 194 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3737c() {
}

// 0xe37670 — __ZN4Ogre10StringUtil5matchERKSsS2_b
#[doc(alias = "Ogre::StringUtil::match(std::string const&,std::string const&,bool)")]
// was: Ogre::StringUtil::match(std::string const&,std::string const&,bool)
// IDA 0xe37670: 343 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e37670() {
}

// 0xe379f8 — __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE7reserveEm
#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)")]
// was: std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::reserve(unsigned long)
// IDA 0xe379f8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e379f8() {
}

// 0xe37bf8 — __ZN4Ogre15StringConverter8toStringEfttcSt13_Ios_Fmtflags
#[doc(alias = "Ogre::StringConverter::toString(float,unsigned short,unsigned short,char,std::_Ios_Fmtflags)")]
// was: Ogre::StringConverter::toString(float,unsigned short,unsigned short,char,std::_Ios_Fmtflags)
// IDA 0xe37bf8: 217 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e37bf8() {
}

// 0xe37e70 — __ZN4Ogre15StringConverter8toStringEitcSt13_Ios_Fmtflags
#[doc(alias = "Ogre::StringConverter::toString(int,unsigned short,char,std::_Ios_Fmtflags)")]
// was: Ogre::StringConverter::toString(int,unsigned short,char,std::_Ios_Fmtflags)
// IDA 0xe37e70: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e37e70() {
}

// 0xe380d0 — __ZN4Ogre15StringConverter8toStringEjtcSt13_Ios_Fmtflags
#[doc(alias = "Ogre::StringConverter::toString(unsigned int,unsigned short,char,std::_Ios_Fmtflags)")]
// was: Ogre::StringConverter::toString(unsigned int,unsigned short,char,std::_Ios_Fmtflags)
// IDA 0xe380d0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e380d0() {
}

// 0xe38330 — __ZN4Ogre15StringConverter8toStringEmtcSt13_Ios_Fmtflags
#[doc(alias = "Ogre::StringConverter::toString(unsigned long,unsigned short,char,std::_Ios_Fmtflags)")]
// was: Ogre::StringConverter::toString(unsigned long,unsigned short,char,std::_Ios_Fmtflags)
// IDA 0xe38330: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e38330() {
}

// 0xe38590 — __ZN4Ogre15StringConverter8toStringEltcSt13_Ios_Fmtflags
#[doc(alias = "Ogre::StringConverter::toString(long,unsigned short,char,std::_Ios_Fmtflags)")]
// was: Ogre::StringConverter::toString(long,unsigned short,char,std::_Ios_Fmtflags)
// IDA 0xe38590: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e38590() {
}

// 0xe387f0 — __ZN4Ogre15StringConverter8toStringERKNS_7Vector3E
#[doc(alias = "Ogre::StringConverter::toString(Ogre::Vector3 const&)")]
// was: Ogre::StringConverter::toString(Ogre::Vector3 const&)
// IDA 0xe387f0: 209 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e387f0() {
}

// 0xe38a60 — __ZN4Ogre15StringConverter8toStringEbb
#[doc(alias = "Ogre::StringConverter::toString(bool,bool)")]
// was: Ogre::StringConverter::toString(bool,bool)
// IDA 0xe38a60: 90 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e38a60() {
}

// 0xe38b60 — __ZN4Ogre15StringConverter8toStringERKNS_11ColourValueE
#[doc(alias = "Ogre::StringConverter::toString(Ogre::ColourValue const&)")]
// was: Ogre::StringConverter::toString(Ogre::ColourValue const&)
// IDA 0xe38b60: 226 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e38b60() {
}

// 0xe38e00 — __ZN4Ogre15StringConverter8toStringERKSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::StringConverter::toString(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::StringConverter::toString(std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe38e00: 202 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e38e00() {
}

// 0xe39044 — __ZN4Ogre15StringConverter9parseRealERKSsf
#[doc(alias = "Ogre::StringConverter::parseReal(std::string const&,float)")]
// was: Ogre::StringConverter::parseReal(std::string const&,float)
// IDA 0xe39044: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e39044() {
}

// 0xe3923c — __ZN4Ogre15StringConverter8parseIntERKSsi
#[doc(alias = "Ogre::StringConverter::parseInt(std::string const&,int)")]
// was: Ogre::StringConverter::parseInt(std::string const&,int)
// IDA 0xe3923c: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3923c() {
}

// 0xe39430 — __ZN4Ogre15StringConverter16parseUnsignedIntERKSsj
#[doc(alias = "Ogre::StringConverter::parseUnsignedInt(std::string const&,unsigned int)")]
// was: Ogre::StringConverter::parseUnsignedInt(std::string const&,unsigned int)
// IDA 0xe39430: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e39430() {
}

// 0xe39624 — __ZN4Ogre15StringConverter9parseLongERKSsl
#[doc(alias = "Ogre::StringConverter::parseLong(std::string const&,long)")]
// was: Ogre::StringConverter::parseLong(std::string const&,long)
// IDA 0xe39624: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e39624() {
}

// 0xe39818 — __ZN4Ogre15StringConverter17parseUnsignedLongERKSsm
#[doc(alias = "Ogre::StringConverter::parseUnsignedLong(std::string const&,unsigned long)")]
// was: Ogre::StringConverter::parseUnsignedLong(std::string const&,unsigned long)
// IDA 0xe39818: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e39818() {
}

// 0xe39a0c — __ZN4Ogre15StringConverter9parseBoolERKSsb
#[doc(alias = "Ogre::StringConverter::parseBool(std::string const&,bool)")]
// was: Ogre::StringConverter::parseBool(std::string const&,bool)
// IDA 0xe39a0c: 409 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e39a0c() {
}

// 0xe39ea4 — __ZN4Ogre15StringConverter12parseVector3ERKSsRKNS_7Vector3E
#[doc(alias = "Ogre::StringConverter::parseVector3(std::string const&,Ogre::Vector3 const&)")]
// was: Ogre::StringConverter::parseVector3(std::string const&,Ogre::Vector3 const&)
// IDA 0xe39ea4: 231 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e39ea4() {
}

// 0xe3a130 — __ZN4Ogre15StringConverter12parseVector4ERKSsRKNS_7Vector4E
#[doc(alias = "Ogre::StringConverter::parseVector4(std::string const&,Ogre::Vector4 const&)")]
// was: Ogre::StringConverter::parseVector4(std::string const&,Ogre::Vector4 const&)
// IDA 0xe3a130: 239 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3a130() {
}

// 0xe3a3d4 — __ZN4Ogre15StringConverter16parseColourValueERKSsRKNS_11ColourValueE
#[doc(alias = "Ogre::StringConverter::parseColourValue(std::string const&,Ogre::ColourValue const&)")]
// was: Ogre::StringConverter::parseColourValue(std::string const&,Ogre::ColourValue const&)
// IDA 0xe3a3d4: 274 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3a3d4() {
}

// 0xe3a6d8 — __ZN4Ogre15StringConverter8isNumberERKSs
#[doc(alias = "Ogre::StringConverter::isNumber(std::string const&)")]
// was: Ogre::StringConverter::isNumber(std::string const&)
// IDA 0xe3a6d8: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3a6d8() {
}

// 0xe3a918 — __ZN4Ogre15StringInterface12setParameterERKSsS2_
#[doc(alias = "Ogre::StringInterface::setParameter(std::string const&,std::string const&)")]
// was: Ogre::StringInterface::setParameter(std::string const&,std::string const&)
// IDA 0xe3a918: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3a918() {
}

// 0xe3a954 — __ZN4Ogre15StringInterface16setParameterListERKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::StringInterface::setParameterList(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::StringInterface::setParameterList(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe3a954: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3a954() {
}

// 0xe3a980 — __ZN4Ogre15StringInterface17cleanupDictionaryEv
#[doc(alias = "Ogre::StringInterface::cleanupDictionary(void)")]
// was: Ogre::StringInterface::cleanupDictionary(void)
// IDA 0xe3a980: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3a980() {
}

// 0xe3a9a8 — __ZNSt3mapISsN4Ogre15ParamDictionaryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "std::map<std::string,Ogre::ParamDictionary,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~map()")]
// was: std::map<std::string,Ogre::ParamDictionary,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~map()
// IDA 0xe3a9a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3a9a8() {
}

// 0xe3aa3c — __ZN4Ogre15StringInterfaceD1Ev
#[doc(alias = "Ogre::StringInterface::~StringInterface()")]
// was: Ogre::StringInterface::~StringInterface()
// IDA 0xe3aa3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3aa3c() {
}

// 0xe3aa98 — __ZN4Ogre15StringInterfaceD0Ev
#[doc(alias = "Ogre::StringInterface::~StringInterface()")]
// was: Ogre::StringInterface::~StringInterface()
// IDA 0xe3aa98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3aa98() {
}

// 0xe3aaf8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamDictionary>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamDictionary>> *)
// IDA 0xe3aaf8: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3aaf8() {
}

// 0xe3ac04 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ParamCommand *>> *)
// IDA 0xe3ac04: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3ac04() {
}

// 0xe3ac7c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe3ac7c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e3ac7c() {
}

// 0xe3ac80 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre15ParamDictionaryEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamDictionary>,std::_Select1st<std::pair<std::string const,Ogre::ParamDictionary>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamDictionary>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe3ac80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3ac80() {
}

// 0xe3ac8c — __ZNSt6vectorIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED2Ev
#[doc(alias = "std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
// was: std::vector<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()
// IDA 0xe3ac8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3ac8c() {
}

// 0xe3ada8 — __ZNSt12_Vector_baseIN4Ogre12ParameterDefENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::ParameterDef,Ogre::STLAllocator<Ogre::ParameterDef,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe3ada8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3ada8() {
}

// 0xe3adb4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12ParamCommandEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ParamCommand *>,std::_Select1st<std::pair<std::string const,Ogre::ParamCommand *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ParamCommand *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe3adb4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3adb4() {
}

// 0xe3aed0 — __ZN4Ogre9SubEntityC1EPNS_6EntityEPNS_7SubMeshE
#[doc(alias = "Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")]
// was: Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)
// IDA 0xe3aed0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3aed0() {
}

// 0xe3aedc — __ZN4Ogre9SubEntityC2EPNS_6EntityEPNS_7SubMeshE
#[doc(alias = "Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)")]
// was: Ogre::SubEntity::SubEntity(Ogre::Entity *,Ogre::SubMesh *)
// IDA 0xe3aedc: 640 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3aedc() {
}

// 0xe3b554 — __ZN4Ogre9SubEntityD0Ev
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
// was: Ogre::SubEntity::~SubEntity()
// IDA 0xe3b554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3b554() {
}

// 0xe3b5e4 — __ZN4Ogre9SubEntityD1Ev
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
// was: Ogre::SubEntity::~SubEntity()
// IDA 0xe3b5e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3b5e4() {
}

// 0xe3b5f0 — __ZN4Ogre9SubEntityD2Ev
#[doc(alias = "Ogre::SubEntity::~SubEntity()")]
// was: Ogre::SubEntity::~SubEntity()
// IDA 0xe3b5f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e3b5f0() {
}

// 0xe3b7cc — __ZN4Ogre9SubEntity10getSubMeshEv
#[doc(alias = "Ogre::SubEntity::getSubMesh(void)")]
// was: Ogre::SubEntity::getSubMesh(void)
// IDA 0xe3b7cc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3b7cc() {
}

// 0xe3b7d0 — __ZNK4Ogre9SubEntity15getMaterialNameEv
#[doc(alias = "Ogre::SubEntity::getMaterialName(void)const")]
// was: Ogre::SubEntity::getMaterialName(void)const
// IDA 0xe3b7d0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3b7d0() {
}

// 0xe3b7d4 — __ZN4Ogre9SubEntity15setMaterialNameERKSsS2_
#[doc(alias = "Ogre::SubEntity::setMaterialName(std::string const&,std::string const&)")]
// was: Ogre::SubEntity::setMaterialName(std::string const&,std::string const&)
// IDA 0xe3b7d4: 975 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3b7d4() {
}

// 0xe3c25c — __ZN4Ogre9SubEntity11setMaterialERKNS_11MaterialPtrE
#[doc(alias = "Ogre::SubEntity::setMaterial(Ogre::MaterialPtr const&)")]
// was: Ogre::SubEntity::setMaterial(Ogre::MaterialPtr const&)
// IDA 0xe3c25c: 373 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3c25c() {
}

// 0xe3c968 — __ZNK4Ogre9SubEntity11getMaterialEv
#[doc(alias = "Ogre::SubEntity::getMaterial(void)const")]
// was: Ogre::SubEntity::getMaterial(void)const
// IDA 0xe3c968: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3c968() {
}

// 0xe3c96c — __ZNK4Ogre9SubEntity12getTechniqueEv
#[doc(alias = "Ogre::SubEntity::getTechnique(void)const")]
// was: Ogre::SubEntity::getTechnique(void)const
// IDA 0xe3c96c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3c96c() {
}

// 0xe3c980 — __ZN4Ogre9SubEntity18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::SubEntity::getRenderOperation(Ogre::RenderOperation &)")]
// was: Ogre::SubEntity::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xe3c980: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3c980() {
}

// 0xe3c9e8 — __ZNK4Ogre9SubEntity18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::SubEntity::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::SubEntity::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xe3c9e8: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3c9e8() {
}

// 0xe3cb10 — __ZNK4Ogre9SubEntity21getNumWorldTransformsEv
#[doc(alias = "Ogre::SubEntity::getNumWorldTransforms(void)const")]
// was: Ogre::SubEntity::getNumWorldTransforms(void)const
// IDA 0xe3cb10: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3cb10() {
}

// 0xe3cb48 — __ZNK4Ogre9SubEntity19getSquaredViewDepthEPKNS_6CameraE
#[doc(alias = "Ogre::SubEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
// was: Ogre::SubEntity::getSquaredViewDepth(Ogre::Camera const*)const
// IDA 0xe3cb48: 132 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3cb48() {
}

// 0xe3cd18 — __ZNK4Ogre9SubEntity9getLightsEv
#[doc(alias = "Ogre::SubEntity::getLights(void)const")]
// was: Ogre::SubEntity::getLights(void)const
// IDA 0xe3cd18: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e3cd18() {
}
