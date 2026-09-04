//! rendering — next 100 Ogre::/G3D:: stubs (EA-sorted strict namespace)
//! Filter: Ogre::|G3D:: (+std<Ogre/G3D>) (10329 total strict, 2558 prior claimed globally, 7771 remaining, this batch 0xd0df90..0xd18074 after 0xd0ddf8)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd0df90 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>> *)")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>> *)
// IDA 0xd0df90: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0df90() {
}

// 0xd0e470 — __ZNSt6vectorIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubMesh **,std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubMesh * const&)")]
// was: std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubMesh **,std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubMesh * const&)
// IDA 0xd0e470: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_d0e470() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd0e568 — __ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage*,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage const&)")]
// was: std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage*,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage const&)
// IDA 0xd0e568: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_d0e568() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd0eb68 — __ZNSt12_Vector_baseIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0eb68: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d0eb68() {
}

// 0xd0eb6c — __ZNSt12_Vector_baseIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0eb6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d0eb6c() {
}

// 0xd0ec24 — __ZNSt12_Vector_baseIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec24: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d0ec24() {
}

// 0xd0ec28 — __ZNSt12_Vector_baseItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ec28() {
}

// 0xd0ec34 — __ZNSt12_Vector_baseIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ec34() {
}

// 0xd0ec40 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xd0ec40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d0ec40() {
}

// 0xd0ec48 — __ZNSt12_Vector_baseIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ec48() {
}

// 0xd0ec54 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()
// IDA 0xd0ec54: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d0ec54() {
}

// 0xd0ec58 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()
// IDA 0xd0ec58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ec58() {
}

// 0xd0ec64 — __ZNSt12_Vector_baseIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ec64() {
}

// 0xd0ec70 — __ZNSt12_Vector_baseIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d0ec70() {
}

// 0xd0ec78 — __ZNSt12_Vector_baseIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec78: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d0ec78() {
}

// 0xd0ec80 — __ZNSt12_Vector_baseIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ec80() {
}

// 0xd0ec90 — __ZNSt12_Vector_baseIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd0ec90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ec90() {
}

// 0xd0eca0 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareVertexBuffer> const&)")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareVertexBuffer> const&)
// IDA 0xd0eca0: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0eca0() {
}

// 0xd0ee20 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()
// IDA 0xd0ee20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ee20() {
}

// 0xd0ef10 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()
// IDA 0xd0ef10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ef10() {
}

// 0xd0f008 — __ZN4Ogre29HardwareVertexBufferSharedPtrD0Ev
#[doc(alias = "Ogre::HardwareVertexBufferSharedPtr::~HardwareVertexBufferSharedPtr()")]
// was: Ogre::HardwareVertexBufferSharedPtr::~HardwareVertexBufferSharedPtr()
// IDA 0xd0f008: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0f008() {
}

// 0xd0f100 — __ZN4Ogre9ExceptionD1Ev
#[doc(alias = "Ogre::Exception::~Exception()")]
// was: Ogre::Exception::~Exception()
// IDA 0xd0f100: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0f100() {
}

// 0xd0f110 — __ZN4Ogre8EdgeDataD2Ev
#[doc(alias = "Ogre::EdgeData::~EdgeData()")]
// was: Ogre::EdgeData::~EdgeData()
// IDA 0xd0f110: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0f110() {
}

// 0xd0f200 — __ZN4Ogre3Log6StreamD2Ev
#[doc(alias = "Ogre::Log::Stream::~Stream()")]
// was: Ogre::Log::Stream::~Stream()
// IDA 0xd0f200: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0f200() {
}

// 0xd0f4f0 — __ZN4Ogre9SharedPtrINS_4MeshEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::operator=(Ogre::SharedPtr<Ogre::Mesh> const&)")]
// was: Ogre::SharedPtr<Ogre::Mesh>::operator=(Ogre::SharedPtr<Ogre::Mesh> const&)
// IDA 0xd0f4f0: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0f4f0() {
}

// 0xd0f670 — __ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeItE
#[doc(alias = "std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<unsigned short> *)")]
// was: std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<unsigned short> *)
// IDA 0xd0f670: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0f670() {
}

// 0xd0f698 — __ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>> *)")]
// was: std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>> *)
// IDA 0xd0f698: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0f698() {
}

// 0xd0f6c0 — __ZN4Ogre9SharedPtrINS_8SkeletonEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Skeleton>::operator=(Ogre::SharedPtr<Ogre::Skeleton> const&)")]
// was: Ogre::SharedPtr<Ogre::Skeleton>::operator=(Ogre::SharedPtr<Ogre::Skeleton> const&)
// IDA 0xd0f6c0: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0f6c0() {
}

// 0xd0f840 — __ZN4Ogre9SharedPtrINS_8SkeletonEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()
// IDA 0xd0f840: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0f840() {
}

// 0xd0f930 — __ZN4Ogre9SharedPtrINS_8SkeletonEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()
// IDA 0xd0f930: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0f930() {
}

// 0xd0fa28 — __ZN4Ogre21InvalidStateExceptionD0Ev
#[doc(alias = "Ogre::InvalidStateException::~InvalidStateException()")]
// was: Ogre::InvalidStateException::~InvalidStateException()
// IDA 0xd0fa28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0fa28() {
}

// 0xd0fa40 — __ZN4Ogre12MeshLodUsageD2Ev
#[doc(alias = "Ogre::MeshLodUsage::~MeshLodUsage()")]
// was: Ogre::MeshLodUsage::~MeshLodUsage()
// IDA 0xd0fa40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0fa40() {
}

// 0xd0fba8 — __ZN4Ogre11SkeletonPtrD0Ev
#[doc(alias = "Ogre::SkeletonPtr::~SkeletonPtr()")]
// was: Ogre::SkeletonPtr::~SkeletonPtr()
// IDA 0xd0fba8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0fba8() {
}

// 0xd0fcd0 — __ZN4Ogre11MeshManager15getSingletonPtrEv
#[doc(alias = "Ogre::MeshManager::getSingletonPtr(void)")]
// was: Ogre::MeshManager::getSingletonPtr(void)
// IDA 0xd0fcd0: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0fcd0() {
}

// 0xd0fce0 — __ZN4Ogre11MeshManager12getSingletonEv
#[doc(alias = "Ogre::MeshManager::getSingleton(void)")]
// was: Ogre::MeshManager::getSingleton(void)
// IDA 0xd0fce0: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0fce0() {
}

// 0xd0fcf0 — __ZN4Ogre11MeshManagerC1Ev
#[doc(alias = "Ogre::MeshManager::MeshManager(void)")]
// was: Ogre::MeshManager::MeshManager(void)
// IDA 0xd0fcf0: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d0fcf0() {
}

// 0xd0fe68 — __ZN4Ogre11MeshManagerD0Ev
#[doc(alias = "Ogre::MeshManager::~MeshManager()")]
// was: Ogre::MeshManager::~MeshManager()
// IDA 0xd0fe68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0fe68() {
}

// 0xd0fef8 — __ZN4Ogre11MeshManagerD1Ev
#[doc(alias = "Ogre::MeshManager::~MeshManager()")]
// was: Ogre::MeshManager::~MeshManager()
// IDA 0xd0fef8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0fef8() {
}

// 0xd0ff04 — __ZThn152_N4Ogre11MeshManagerD0Ev
#[doc(alias = "non-virtual thunk toOgre::MeshManager::~MeshManager()")]
// was: non-virtual thunk to Ogre::MeshManager::~MeshManager()
// IDA 0xd0ff04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ff04() {
}

// 0xd0ff98 — __ZN4Ogre11MeshManagerD2Ev
#[doc(alias = "Ogre::MeshManager::~MeshManager()")]
// was: Ogre::MeshManager::~MeshManager()
// IDA 0xd0ff98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d0ff98() {
}

// 0xd100a8 — __ZThn152_N4Ogre11MeshManagerD1Ev
#[doc(alias = "non-virtual thunk toOgre::MeshManager::~MeshManager()")]
// was: non-virtual thunk to Ogre::MeshManager::~MeshManager()
// IDA 0xd100a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d100a8() {
}

// 0xd100b4 — __ZN4Ogre11MeshManager11_initialiseEv
#[doc(alias = "Ogre::MeshManager::_initialise(void)")]
// was: Ogre::MeshManager::_initialise(void)
// IDA 0xd100b4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d100b4() {
}

// 0xd100cc — __ZN4Ogre11MeshManager17createPrefabPlaneEv
#[doc(alias = "Ogre::MeshManager::createPrefabPlane(void)")]
// was: Ogre::MeshManager::createPrefabPlane(void)
// IDA 0xd100cc: 311 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d100cc() {
}

// 0xd103f4 — __ZN4Ogre11MeshManager16createPrefabCubeEv
#[doc(alias = "Ogre::MeshManager::createPrefabCube(void)")]
// was: Ogre::MeshManager::createPrefabCube(void)
// IDA 0xd103f4: 309 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d103f4() {
}

// 0xd10718 — __ZN4Ogre11MeshManager18createPrefabSphereEv
#[doc(alias = "Ogre::MeshManager::createPrefabSphere(void)")]
// was: Ogre::MeshManager::createPrefabSphere(void)
// IDA 0xd10718: 309 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d10718() {
}

// 0xd10a3c — __ZN4Ogre11MeshManager16createOrRetrieveERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEENS_14HardwareBuffer5UsageESJ_bb
#[doc(alias = "Ogre::MeshManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: Ogre::MeshManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
// IDA 0xd10a3c: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d10a3c() {
}

// 0xd10c5c — __ZN4Ogre11MeshManager4loadERKSsS2_NS_14HardwareBuffer5UsageES4_bb
#[doc(alias = "Ogre::MeshManager::load(std::string const&,std::string const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: Ogre::MeshManager::load(std::string const&,std::string const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
// IDA 0xd10c5c: 218 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d10c5c() {
}

// 0xd10e7c — __ZN4Ogre11MeshManager12createManualERKSsS2_PNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::MeshManager::createManual(std::string const&,std::string const&,Ogre::ManualResourceLoader *)")]
// was: Ogre::MeshManager::createManual(std::string const&,std::string const&,Ogre::ManualResourceLoader *)
// IDA 0xd10e7c: 147 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d10e7c() {
}

// 0xd10ffc — __ZN4Ogre11MeshManager11createPlaneERKSsS2_RKNS_5PlaneEffiibtffRKNS_7Vector3ENS_14HardwareBuffer5UsageESA_bb
#[doc(alias = "Ogre::MeshManager::createPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: Ogre::MeshManager::createPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
// IDA 0xd10ffc: 212 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d10ffc() {
}

// 0xd1122c — __ZN4Ogre11MeshManager17createCurvedPlaneERKSsS2_RKNS_5PlaneEfffiibtffRKNS_7Vector3ENS_14HardwareBuffer5UsageESA_bb
#[doc(alias = "Ogre::MeshManager::createCurvedPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: Ogre::MeshManager::createCurvedPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
// IDA 0xd1122c: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1122c() {
}

// 0xd11464 — __ZN4Ogre11MeshManager25createCurvedIllusionPlaneERKSsS2_RKNS_5PlaneEfffiibtffRKNS_7Vector3ERKNS_10QuaternionENS_14HardwareBuffer5UsageESD_bbi
#[doc(alias = "Ogre::MeshManager::createCurvedIllusionPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool,int)")]
// was: Ogre::MeshManager::createCurvedIllusionPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool,int)
// IDA 0xd11464: 221 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d11464() {
}

// 0xd116ac — __ZN4Ogre11MeshManager15tesselate2DMeshEPNS_7SubMeshEttbNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::MeshManager::tesselate2DMesh(Ogre::SubMesh *,unsigned short,unsigned short,bool,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::MeshManager::tesselate2DMesh(Ogre::SubMesh *,unsigned short,unsigned short,bool,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xd116ac: 361 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d116ac() {
}

// 0xd11a14 — __ZN4Ogre11MeshManager11getListenerEv
#[doc(alias = "Ogre::MeshManager::getListener(void)")]
// was: Ogre::MeshManager::getListener(void)
// IDA 0xd11a14: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d11a14() {
}

// 0xd11a1c — __ZN4Ogre11MeshManager12loadResourceEPNS_8ResourceE
#[doc(alias = "Ogre::MeshManager::loadResource(Ogre::Resource *)")]
// was: Ogre::MeshManager::loadResource(Ogre::Resource *)
// IDA 0xd11a1c: 413 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d11a1c() {
}

// 0xd11ec8 — __ZN4Ogre11MeshManager15loadManualPlaneEPNS_4MeshERNS0_15MeshBuildParamsE
#[doc(alias = "Ogre::MeshManager::loadManualPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")]
// was: Ogre::MeshManager::loadManualPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)
// IDA 0xd11ec8: 957 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d11ec8() {
}

// 0xd12a3c — __ZN4Ogre11MeshManager29loadManualCurvedIllusionPlaneEPNS_4MeshERNS0_15MeshBuildParamsE
#[doc(alias = "Ogre::MeshManager::loadManualCurvedIllusionPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")]
// was: Ogre::MeshManager::loadManualCurvedIllusionPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)
// IDA 0xd12a3c: 1059 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d12a3c() {
}

// 0xd136f0 — __ZN4Ogre11MeshManager21loadManualCurvedPlaneEPNS_4MeshERNS0_15MeshBuildParamsE
#[doc(alias = "Ogre::MeshManager::loadManualCurvedPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")]
// was: Ogre::MeshManager::loadManualCurvedPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)
// IDA 0xd136f0: 1039 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d136f0() {
}

// 0xd14398 — __ZThn152_N4Ogre11MeshManager12loadResourceEPNS_8ResourceE
#[doc(alias = "non-virtual thunk toOgre::MeshManager::loadResource(Ogre::Resource *)")]
// was: non-virtual thunk to Ogre::MeshManager::loadResource(Ogre::Resource *)
// IDA 0xd14398: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14398() {
}

// 0xd143a4 — __ZN4Ogre11MeshManager35setPrepareAllMeshesForShadowVolumesEb
#[doc(alias = "Ogre::MeshManager::setPrepareAllMeshesForShadowVolumes(bool)")]
// was: Ogre::MeshManager::setPrepareAllMeshesForShadowVolumes(bool)
// IDA 0xd143a4: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d143a4() {
}

// 0xd143ac — __ZN4Ogre11MeshManager35getPrepareAllMeshesForShadowVolumesEv
#[doc(alias = "Ogre::MeshManager::getPrepareAllMeshesForShadowVolumes(void)")]
// was: Ogre::MeshManager::getPrepareAllMeshesForShadowVolumes(void)
// IDA 0xd143ac: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d143ac() {
}

// 0xd143b4 — __ZN4Ogre11MeshManager22getBoundsPaddingFactorEv
#[doc(alias = "Ogre::MeshManager::getBoundsPaddingFactor(void)")]
// was: Ogre::MeshManager::getBoundsPaddingFactor(void)
// IDA 0xd143b4: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d143b4() {
}

// 0xd143bc — __ZN4Ogre11MeshManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::MeshManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::MeshManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xd143bc: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d143bc() {
}

// 0xd14494 — __ZNSt3mapIPN4Ogre8ResourceENS0_11MeshManager15MeshBuildParamsESt4lessIS2_ENS0_12STLAllocatorISt4pairIKS2_S4_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS9_
#[doc(alias = "std::map<Ogre::Resource *,Ogre::MeshManager::MeshBuildParams,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Resource * const&)")]
// was: std::map<Ogre::Resource *,Ogre::MeshManager::MeshBuildParams,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Resource * const&)
// IDA 0xd14494: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14494() {
}

// 0xd1455c — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)
// IDA 0xd1455c: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d1455c() {
}

// 0xd14610 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)
// IDA 0xd14610: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14610() {
}

// 0xd14760 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)
// IDA 0xd14760: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14760() {
}

// 0xd147cc — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()
// IDA 0xd147cc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d147cc() {
}

// 0xd147d0 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()
// IDA 0xd147d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d147d0() {
}

// 0xd147dc — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::destroy(void)
// IDA 0xd147dc: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d147dc() {
}

// 0xd14814 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareVertexBuffer>&)")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareVertexBuffer>&)
// IDA 0xd14814: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14814() {
}

// 0xd14830 — __ZNK4Ogre7Matrix411concatenateERKS0_
#[doc(alias = "Ogre::Matrix4::concatenate(Ogre::Matrix4 const&)const")]
// was: Ogre::Matrix4::concatenate(Ogre::Matrix4 const&)const
// IDA 0xd14830: 179 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14830() {
}

// 0xd14af0 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()
// IDA 0xd14af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d14af0() {
}

// 0xd14be0 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareIndexBuffer> const&)")]
// was: Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareIndexBuffer> const&)
// IDA 0xd14be0: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14be0() {
}

// 0xd14d60 — __ZN4Ogre9SharedPtrINS_4MeshEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()
// IDA 0xd14d60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d14d60() {
}

// 0xd14e10 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>> *)")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>> *)
// IDA 0xd14e10: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14e10() {
}

// 0xd14e6c — __ZN4Ogre14MeshSerializerC1Ev
#[doc(alias = "Ogre::MeshSerializer::MeshSerializer(void)")]
// was: Ogre::MeshSerializer::MeshSerializer(void)
// IDA 0xd14e6c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14e6c() {
}

// 0xd14e78 — __ZN4Ogre14MeshSerializerC2Ev
#[doc(alias = "Ogre::MeshSerializer::MeshSerializer(void)")]
// was: Ogre::MeshSerializer::MeshSerializer(void)
// IDA 0xd14e78: 905 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d14e78() {
}

// 0xd15800 — __ZN4Ogre14MeshSerializerD0Ev
#[doc(alias = "Ogre::MeshSerializer::~MeshSerializer()")]
// was: Ogre::MeshSerializer::~MeshSerializer()
// IDA 0xd15800: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d15800() {
}

// 0xd15890 — __ZN4Ogre14MeshSerializerD1Ev
#[doc(alias = "Ogre::MeshSerializer::~MeshSerializer()")]
// was: Ogre::MeshSerializer::~MeshSerializer()
// IDA 0xd15890: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d15890() {
}

// 0xd1589c — __ZN4Ogre14MeshSerializerD2Ev
#[doc(alias = "Ogre::MeshSerializer::~MeshSerializer()")]
// was: Ogre::MeshSerializer::~MeshSerializer()
// IDA 0xd1589c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1589c() {
}

// 0xd159d8 — __ZN4Ogre14MeshSerializer10importMeshERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshE
#[doc(alias = "Ogre::MeshSerializer::importMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)")]
// was: Ogre::MeshSerializer::importMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *)
// IDA 0xd159d8: 842 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d159d8() {
}

// 0xd16388 — __ZN4Ogre14MeshSerializer11setListenerEPNS_22MeshSerializerListenerE
#[doc(alias = "Ogre::MeshSerializer::setListener(Ogre::MeshSerializerListener *)")]
// was: Ogre::MeshSerializer::setListener(Ogre::MeshSerializerListener *)
// IDA 0xd16388: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d16388() {
}

// 0xd1638c — __ZNSt6vectorIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshSerializer::MeshVersionData **,std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshSerializer::MeshVersionData * const&)")]
// was: std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshSerializer::MeshVersionData **,std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshSerializer::MeshVersionData * const&)
// IDA 0xd1638c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_d1638c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd16484 — __ZNSt12_Vector_baseIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd16484: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d16484() {
}

// 0xd16488 — __ZNSt12_Vector_baseIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd16488: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d16488() {
}

// 0xd164c8 — __ZN4Ogre18MeshSerializerImplC1Ev
#[doc(alias = "Ogre::MeshSerializerImpl::MeshSerializerImpl(void)")]
// was: Ogre::MeshSerializerImpl::MeshSerializerImpl(void)
// IDA 0xd164c8: 65 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d164c8() {
}

// 0xd1658c — __ZN4Ogre18MeshSerializerImplD0Ev
#[doc(alias = "Ogre::MeshSerializerImpl::~MeshSerializerImpl()")]
// was: Ogre::MeshSerializerImpl::~MeshSerializerImpl()
// IDA 0xd1658c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1658c() {
}

// 0xd1661c — __ZN4Ogre18MeshSerializerImplD1Ev
#[doc(alias = "Ogre::MeshSerializerImpl::~MeshSerializerImpl()")]
// was: Ogre::MeshSerializerImpl::~MeshSerializerImpl()
// IDA 0xd1661c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d1661c() {
}

// 0xd16628 — __ZN4Ogre18MeshSerializerImpl10importMeshERNS_9SharedPtrINS_10DataStreamEEEPNS_4MeshEPNS_22MeshSerializerListenerE
#[doc(alias = "Ogre::MeshSerializerImpl::importMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)")]
// was: Ogre::MeshSerializerImpl::importMesh(Ogre::SharedPtr<Ogre::DataStream> &,Ogre::Mesh *,Ogre::MeshSerializerListener *)
// IDA 0xd16628: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d16628() {
}

// 0xd16680 — __ZN4Ogre18MeshSerializerImpl9writeMeshEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeMesh(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writeMesh(Ogre::Mesh const*)
// IDA 0xd16680: 1057 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d16680() {
}

// 0xd17258 — __ZN4Ogre18MeshSerializerImpl21writeSubMeshNameTableEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshNameTable(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writeSubMeshNameTable(Ogre::Mesh const*)
// IDA 0xd17258: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d17258() {
}

// 0xd172c4 — __ZN4Ogre18MeshSerializerImpl12writeSubMeshEPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMesh(Ogre::SubMesh const*)")]
// was: Ogre::MeshSerializerImpl::writeSubMesh(Ogre::SubMesh const*)
// IDA 0xd172c4: 410 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d172c4() {
}

// 0xd176d4 — __ZN4Ogre18MeshSerializerImpl13writeExtremesEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeExtremes(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::writeExtremes(Ogre::Mesh const*)
// IDA 0xd176d4: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d176d4() {
}

// 0xd17914 — __ZN4Ogre18MeshSerializerImpl20writeSubMeshExtremesEtPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshExtremes(unsigned short,Ogre::SubMesh const*)")]
// was: Ogre::MeshSerializerImpl::writeSubMeshExtremes(unsigned short,Ogre::SubMesh const*)
// IDA 0xd17914: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d17914() {
}

// 0xd17990 — __ZN4Ogre18MeshSerializerImpl26writeSubMeshTextureAliasesEPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshTextureAliases(Ogre::SubMesh const*)")]
// was: Ogre::MeshSerializerImpl::writeSubMeshTextureAliases(Ogre::SubMesh const*)
// IDA 0xd17990: 202 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d17990() {
}

// 0xd17bdc — __ZN4Ogre18MeshSerializerImpl21writeSubMeshOperationEPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshOperation(Ogre::SubMesh const*)")]
// was: Ogre::MeshSerializerImpl::writeSubMeshOperation(Ogre::SubMesh const*)
// IDA 0xd17bdc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d17bdc() {
}

// 0xd17c10 — __ZN4Ogre18MeshSerializerImpl13writeGeometryEPKNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl::writeGeometry(Ogre::VertexData const*)")]
// was: Ogre::MeshSerializerImpl::writeGeometry(Ogre::VertexData const*)
// IDA 0xd17c10: 321 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d17c10() {
}

// 0xd17f48 — __ZN4Ogre18MeshSerializerImpl24calcSubMeshNameTableSizeEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshNameTableSize(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::calcSubMeshNameTableSize(Ogre::Mesh const*)
// IDA 0xd17f48: 23 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d17f48() {
}

// 0xd17f88 — __ZN4Ogre18MeshSerializerImpl12calcMeshSizeEPKNS_4MeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcMeshSize(Ogre::Mesh const*)")]
// was: Ogre::MeshSerializerImpl::calcMeshSize(Ogre::Mesh const*)
// IDA 0xd17f88: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d17f88() {
}

// 0xd18074 — __ZN4Ogre18MeshSerializerImpl15calcSubMeshSizeEPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshSize(Ogre::SubMesh const*)")]
// was: Ogre::MeshSerializerImpl::calcSubMeshSize(Ogre::SubMesh const*)
// IDA 0xd18074: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d18074() {
}