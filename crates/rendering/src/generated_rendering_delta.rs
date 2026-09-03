//! rendering — generated_rendering_delta — 120 stubs watchdog w14 global dedup (Ogre|Gfx|Render filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) filtered Ogre|Gfx|rendering NOT in /tmp/global_eas.txt — next 120 uncovered EA-sorted asc 0xd0d9f4..0xd1b2b0 (5274 candidates remaining, 87653 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0xd0d9f4 — __ZN4Ogre12STLAllocatorINS_12MeshLodUsageENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE9constructEPS1_RKS1_
// type: int __fastcall(int, int, int, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::construct(Ogre::MeshLodUsage*,Ogre::MeshLodUsage const&)")]
#[doc(alias = "__ZN4Ogre12STLAllocatorINS_12MeshLodUsageENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE9constructEPS1_RKS1_")]
// was: Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::construct(Ogre::MeshLodUsage*,Ogre::MeshLodUsage const&)
pub fn stub_d0d9f4() -> ! {
    todo!("0xd0d9f4 Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::construct(Ogre::MeshLodUsage*,Ogre::MeshLodUsage const&)")
}

// 0xd0db40 — __ZNSt6vectorItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPtS6_EEmRKt
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned short *,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned short const&)")]
#[doc(alias = "__ZNSt6vectorItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPtS6_EEmRKt")]
// was: std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned short *,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned short const&)
pub fn stub_d0db40() -> ! {
    todo!("0xd0db40 std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<unsigned short *,std::vector<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,unsigned short const&)")
}

// 0xd0dce0 — __ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS3_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS3_Lb0EED0Ev")]
// was: std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
pub fn stub_d0dce0() -> ! {
    todo!("0xd0dce0 std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")
}

// 0xd0dcec — __ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE13_Rb_tree_implISC_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE13_Rb_tree_implISC_Lb0EED1Ev")]
// was: std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()
pub fn stub_d0dcec() -> ! {
    todo!("0xd0dcec std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()")
}

// 0xd0dcf0 — __ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE13_Rb_tree_implISC_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE13_Rb_tree_implISC_Lb0EED0Ev")]
// was: std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()
pub fn stub_d0dcf0() -> ! {
    todo!("0xd0dcf0 std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<float>,false>::~_Rb_tree_impl()")
}

// 0xd0dcfc — __ZNSt6vectorIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pose **,std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pose * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pose **,std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pose * const&)
pub fn stub_d0dcfc() -> ! {
    todo!("0xd0dcfc std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Pose **,std::vector<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Pose * const&)")
}

// 0xd0ddf8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Animation *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Animation *> const&)
pub fn stub_d0ddf8() -> ! {
    todo!("0xd0ddf8 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Animation *> const&)")
}

// 0xd0df90 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>> *)
pub fn stub_d0df90() -> ! {
    todo!("0xd0df90 std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>> *)")
}

// 0xd0e470 — __ZNSt6vectorIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubMesh **,std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubMesh * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubMesh **,std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubMesh * const&)
pub fn stub_d0e470() -> ! {
    todo!("0xd0e470 std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubMesh **,std::vector<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubMesh * const&)")
}

// 0xd0e568 — __ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
#[doc(alias = "std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage*,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage const&)")]
#[doc(alias = "__ZNSt6vectorIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_")]
// was: std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage*,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage const&)
pub fn stub_d0e568() -> ! {
    todo!("0xd0e568 std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshLodUsage*,std::vector<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshLodUsage const&)")
}

// 0xd0eb68 — __ZNSt12_Vector_baseIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0eb68() -> ! {
    todo!("0xd0eb68 std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0eb6c — __ZNSt12_Vector_baseIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0eb6c() -> ! {
    todo!("0xd0eb6c std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec24 — __ZNSt12_Vector_baseIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec24() -> ! {
    todo!("0xd0ec24 std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec28 — __ZNSt12_Vector_baseItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseItN4Ogre12STLAllocatorItNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec28() -> ! {
    todo!("0xd0ec28 std::_Vector_base<unsigned short,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec34 — __ZNSt12_Vector_baseIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre4PoseENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec34() -> ! {
    todo!("0xd0ec34 std::_Vector_base<Ogre::Pose *,Ogre::STLAllocator<Ogre::Pose *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec40 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre9AnimationEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
pub fn stub_d0ec40() -> ! {
    todo!("0xd0ec40 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Animation *>,std::_Select1st<std::pair<std::string const,Ogre::Animation *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Animation *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}

// 0xd0ec48 — __ZNSt12_Vector_baseIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre12MeshLodUsageENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec48() -> ! {
    todo!("0xd0ec48 std::_Vector_base<Ogre::MeshLodUsage,Ogre::STLAllocator<Ogre::MeshLodUsage,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec54 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()
pub fn stub_d0ec54() -> ! {
    todo!("0xd0ec54 std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")
}

// 0xd0ec58 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmN4Ogre22VertexBoneAssignment_sEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev")]
// was: std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()
pub fn stub_d0ec58() -> ! {
    todo!("0xd0ec58 std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,std::_Select1st<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long>,false>::~_Rb_tree_impl()")
}

// 0xd0ec64 — __ZNSt12_Vector_baseIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre7SubMeshENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec64() -> ! {
    todo!("0xd0ec64 std::_Vector_base<Ogre::SubMesh *,Ogre::STLAllocator<Ogre::SubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec70 — __ZNSt12_Vector_baseIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre8EdgeData8TriangleENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec70() -> ! {
    todo!("0xd0ec70 std::_Vector_base<Ogre::EdgeData::Triangle,Ogre::STLAllocator<Ogre::EdgeData::Triangle,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec78 — __ZNSt12_Vector_baseIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIcN4Ogre12STLAllocatorIcNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec78() -> ! {
    todo!("0xd0ec78 std::_Vector_base<char,Ogre::STLAllocator<char,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec80 — __ZNSt12_Vector_baseIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre8EdgeData9EdgeGroupENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec80() -> ! {
    todo!("0xd0ec80 std::_Vector_base<Ogre::EdgeData::EdgeGroup,Ogre::STLAllocator<Ogre::EdgeData::EdgeGroup,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0ec90 — __ZNSt12_Vector_baseIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIN4Ogre7Vector4ENS0_12STLAllocatorIS1_NS0_27CategorisedAlignAllocPolicyILNS0_14MemoryCategoryE1ELm0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d0ec90() -> ! {
    todo!("0xd0ec90 std::_Vector_base<Ogre::Vector4,Ogre::STLAllocator<Ogre::Vector4,Ogre::CategorisedAlignAllocPolicy<(Ogre::MemoryCategory)1,0ul>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd0eca0 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareVertexBuffer> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEEaSERKS2_")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareVertexBuffer> const&)
pub fn stub_d0eca0() -> ! {
    todo!("0xd0eca0 Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareVertexBuffer> const&)")
}

// 0xd0ee20 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED1Ev")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()
pub fn stub_d0ee20() -> ! {
    todo!("0xd0ee20 Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()")
}

// 0xd0ef10 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEED0Ev")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()
pub fn stub_d0ef10() -> ! {
    todo!("0xd0ef10 Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::~SharedPtr()")
}

// 0xd0f008 — __ZN4Ogre29HardwareVertexBufferSharedPtrD0Ev
// type: void __fastcall(Ogre::HardwareVertexBufferSharedPtr *__hidden this)
#[doc(alias = "Ogre::HardwareVertexBufferSharedPtr::~HardwareVertexBufferSharedPtr()")]
#[doc(alias = "__ZN4Ogre29HardwareVertexBufferSharedPtrD0Ev")]
// was: Ogre::HardwareVertexBufferSharedPtr::~HardwareVertexBufferSharedPtr()
pub fn stub_d0f008() -> ! {
    todo!("0xd0f008 Ogre::HardwareVertexBufferSharedPtr::~HardwareVertexBufferSharedPtr()")
}

// 0xd0f100 — __ZN4Ogre9ExceptionD1Ev
// type: void __fastcall(Ogre::Exception *__hidden this)
#[doc(alias = "Ogre::Exception::~Exception()")]
#[doc(alias = "__ZN4Ogre9ExceptionD1Ev")]
// was: Ogre::Exception::~Exception()
pub fn stub_d0f100() -> ! {
    todo!("0xd0f100 Ogre::Exception::~Exception()")
}

// 0xd0f110 — __ZN4Ogre8EdgeDataD2Ev
// type: void __fastcall(Ogre::EdgeData *__hidden this)
#[doc(alias = "Ogre::EdgeData::~EdgeData()")]
#[doc(alias = "__ZN4Ogre8EdgeDataD2Ev")]
// was: Ogre::EdgeData::~EdgeData()
pub fn stub_d0f110() -> ! {
    todo!("0xd0f110 Ogre::EdgeData::~EdgeData()")
}

// 0xd0f200 — __ZN4Ogre3Log6StreamD2Ev
// type: void __fastcall(Ogre::Log::Stream *__hidden this)
#[doc(alias = "Ogre::Log::Stream::~Stream()")]
#[doc(alias = "__ZN4Ogre3Log6StreamD2Ev")]
// was: Ogre::Log::Stream::~Stream()
pub fn stub_d0f200() -> ! {
    todo!("0xd0f200 Ogre::Log::Stream::~Stream()")
}

// 0xd0f4f0 — __ZN4Ogre9SharedPtrINS_4MeshEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::operator=(Ogre::SharedPtr<Ogre::Mesh> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEEaSERKS2_")]
// was: Ogre::SharedPtr<Ogre::Mesh>::operator=(Ogre::SharedPtr<Ogre::Mesh> const&)
pub fn stub_d0f4f0() -> ! {
    todo!("0xd0f4f0 Ogre::SharedPtr<Ogre::Mesh>::operator=(Ogre::SharedPtr<Ogre::Mesh> const&)")
}

// 0xd0f670 — __ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeItE
#[doc(alias = "std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<unsigned short> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIttSt9_IdentityItESt4lessItEN4Ogre12STLAllocatorItNS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeItE")]
// was: std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<unsigned short> *)
pub fn stub_d0f670() -> ! {
    todo!("0xd0f670 std::_Rb_tree<unsigned short,unsigned short,std::_Identity<unsigned short>,std::less<unsigned short>,Ogre::STLAllocator<unsigned short,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<unsigned short> *)")
}

// 0xd0f698 — __ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIfSt4pairIKfSt17_Rb_tree_iteratorIS0_IKmN4Ogre22VertexBoneAssignment_sEEEESt10_Select1stIS8_ESt4lessIfENS4_12STLAllocatorIS8_NS4_22CategorisedAllocPolicyILNS4_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// was: std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>> *)
pub fn stub_d0f698() -> ! {
    todo!("0xd0f698 std::_Rb_tree<float,std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,std::_Select1st<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,std::_Rb_tree_iterator<std::pair<unsigned long const,Ogre::VertexBoneAssignment_s>>>> *)")
}

// 0xd0f6c0 — __ZN4Ogre9SharedPtrINS_8SkeletonEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::Skeleton>::operator=(Ogre::SharedPtr<Ogre::Skeleton> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8SkeletonEEaSERKS2_")]
// was: Ogre::SharedPtr<Ogre::Skeleton>::operator=(Ogre::SharedPtr<Ogre::Skeleton> const&)
pub fn stub_d0f6c0() -> ! {
    todo!("0xd0f6c0 Ogre::SharedPtr<Ogre::Skeleton>::operator=(Ogre::SharedPtr<Ogre::Skeleton> const&)")
}

// 0xd0f840 — __ZN4Ogre9SharedPtrINS_8SkeletonEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8SkeletonEED1Ev")]
// was: Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()
pub fn stub_d0f840() -> ! {
    todo!("0xd0f840 Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()")
}

// 0xd0f930 — __ZN4Ogre9SharedPtrINS_8SkeletonEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_8SkeletonEED0Ev")]
// was: Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()
pub fn stub_d0f930() -> ! {
    todo!("0xd0f930 Ogre::SharedPtr<Ogre::Skeleton>::~SharedPtr()")
}

// 0xd0fa28 — __ZN4Ogre21InvalidStateExceptionD0Ev
// type: void __fastcall(Ogre::InvalidStateException *__hidden this)
#[doc(alias = "Ogre::InvalidStateException::~InvalidStateException()")]
#[doc(alias = "__ZN4Ogre21InvalidStateExceptionD0Ev")]
// was: Ogre::InvalidStateException::~InvalidStateException()
pub fn stub_d0fa28() -> ! {
    todo!("0xd0fa28 Ogre::InvalidStateException::~InvalidStateException()")
}

// 0xd0fa40 — __ZN4Ogre12MeshLodUsageD2Ev
// type: void __fastcall(Ogre::MeshLodUsage *__hidden this)
#[doc(alias = "Ogre::MeshLodUsage::~MeshLodUsage()")]
#[doc(alias = "__ZN4Ogre12MeshLodUsageD2Ev")]
// was: Ogre::MeshLodUsage::~MeshLodUsage()
pub fn stub_d0fa40() -> ! {
    todo!("0xd0fa40 Ogre::MeshLodUsage::~MeshLodUsage()")
}

// 0xd0fba8 — __ZN4Ogre11SkeletonPtrD0Ev
// type: void __fastcall(Ogre::SkeletonPtr *__hidden this)
#[doc(alias = "Ogre::SkeletonPtr::~SkeletonPtr()")]
#[doc(alias = "__ZN4Ogre11SkeletonPtrD0Ev")]
// was: Ogre::SkeletonPtr::~SkeletonPtr()
pub fn stub_d0fba8() -> ! {
    todo!("0xd0fba8 Ogre::SkeletonPtr::~SkeletonPtr()")
}

// 0xd0fcd0 — __ZN4Ogre11MeshManager15getSingletonPtrEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::getSingletonPtr(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager15getSingletonPtrEv")]
// was: Ogre::MeshManager::getSingletonPtr(void)
pub fn stub_d0fcd0() -> ! {
    todo!("0xd0fcd0 Ogre::MeshManager::getSingletonPtr(void)")
}

// 0xd0fce0 — __ZN4Ogre11MeshManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::getSingleton(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager12getSingletonEv")]
// was: Ogre::MeshManager::getSingleton(void)
pub fn stub_d0fce0() -> ! {
    todo!("0xd0fce0 Ogre::MeshManager::getSingleton(void)")
}

// 0xd0fcf0 — __ZN4Ogre11MeshManagerC1Ev
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::MeshManager(void)")]
#[doc(alias = "__ZN4Ogre11MeshManagerC1Ev")]
// was: Ogre::MeshManager::MeshManager(void)
pub fn stub_d0fcf0() -> ! {
    todo!("0xd0fcf0 Ogre::MeshManager::MeshManager(void)")
}

// 0xd0fe68 — __ZN4Ogre11MeshManagerD0Ev
// type: void __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::~MeshManager()")]
#[doc(alias = "__ZN4Ogre11MeshManagerD0Ev")]
// was: Ogre::MeshManager::~MeshManager()
pub fn stub_d0fe68() -> ! {
    todo!("0xd0fe68 Ogre::MeshManager::~MeshManager()")
}

// 0xd0fef8 — __ZN4Ogre11MeshManagerD1Ev
// type: void __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::~MeshManager()")]
#[doc(alias = "__ZN4Ogre11MeshManagerD1Ev")]
// was: Ogre::MeshManager::~MeshManager()
pub fn stub_d0fef8() -> ! {
    todo!("0xd0fef8 Ogre::MeshManager::~MeshManager()")
}

// 0xd0ff04 — __ZThn152_N4Ogre11MeshManagerD0Ev
// type: void __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::MeshManager::~MeshManager()")]
#[doc(alias = "__ZThn152_N4Ogre11MeshManagerD0Ev")]
// was: `non-virtual thunk to'Ogre::MeshManager::~MeshManager()
pub fn stub_d0ff04() -> ! {
    todo!("0xd0ff04 non-virtual thunk toOgre::MeshManager::~MeshManager()")
}

// 0xd0ff98 — __ZN4Ogre11MeshManagerD2Ev
// type: void __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::~MeshManager()")]
#[doc(alias = "__ZN4Ogre11MeshManagerD2Ev")]
// was: Ogre::MeshManager::~MeshManager()
pub fn stub_d0ff98() -> ! {
    todo!("0xd0ff98 Ogre::MeshManager::~MeshManager()")
}

// 0xd100a8 — __ZThn152_N4Ogre11MeshManagerD1Ev
// type: void __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "non-virtual thunk toOgre::MeshManager::~MeshManager()")]
#[doc(alias = "__ZThn152_N4Ogre11MeshManagerD1Ev")]
// was: `non-virtual thunk to'Ogre::MeshManager::~MeshManager()
pub fn stub_d100a8() -> ! {
    todo!("0xd100a8 non-virtual thunk toOgre::MeshManager::~MeshManager()")
}

// 0xd100b4 — __ZN4Ogre11MeshManager11_initialiseEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::_initialise(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager11_initialiseEv")]
// was: Ogre::MeshManager::_initialise(void)
pub fn stub_d100b4() -> ! {
    todo!("0xd100b4 Ogre::MeshManager::_initialise(void)")
}

// 0xd100cc — __ZN4Ogre11MeshManager17createPrefabPlaneEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::createPrefabPlane(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager17createPrefabPlaneEv")]
// was: Ogre::MeshManager::createPrefabPlane(void)
pub fn stub_d100cc() -> ! {
    todo!("0xd100cc Ogre::MeshManager::createPrefabPlane(void)")
}

// 0xd103f4 — __ZN4Ogre11MeshManager16createPrefabCubeEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::createPrefabCube(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager16createPrefabCubeEv")]
// was: Ogre::MeshManager::createPrefabCube(void)
pub fn stub_d103f4() -> ! {
    todo!("0xd103f4 Ogre::MeshManager::createPrefabCube(void)")
}

// 0xd10718 — __ZN4Ogre11MeshManager18createPrefabSphereEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::createPrefabSphere(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager18createPrefabSphereEv")]
// was: Ogre::MeshManager::createPrefabSphere(void)
pub fn stub_d10718() -> ! {
    todo!("0xd10718 Ogre::MeshManager::createPrefabSphere(void)")
}

// 0xd10a3c — __ZN4Ogre11MeshManager16createOrRetrieveERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEENS_14HardwareBuffer5UsageESJ_bb
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, struct _Unwind_Exception *, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int)
#[doc(alias = "Ogre::MeshManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
#[doc(alias = "__ZN4Ogre11MeshManager16createOrRetrieveERKSsS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEENS_14HardwareBuffer5UsageESJ_bb")]
// was: Ogre::MeshManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
pub fn stub_d10a3c() -> ! {
    todo!("0xd10a3c Ogre::MeshManager::createOrRetrieve(std::string const&,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")
}

// 0xd10c5c — __ZN4Ogre11MeshManager4loadERKSsS2_NS_14HardwareBuffer5UsageES4_bb
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::MeshManager::load(std::string const&,std::string const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
#[doc(alias = "__ZN4Ogre11MeshManager4loadERKSsS2_NS_14HardwareBuffer5UsageES4_bb")]
// was: Ogre::MeshManager::load(std::string const&,std::string const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
pub fn stub_d10c5c() -> ! {
    todo!("0xd10c5c Ogre::MeshManager::load(std::string const&,std::string const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")
}

// 0xd10e7c — __ZN4Ogre11MeshManager12createManualERKSsS2_PNS_20ManualResourceLoaderE
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this, const std::string *, const std::string *, Ogre::ManualResourceLoader *)
#[doc(alias = "Ogre::MeshManager::createManual(std::string const&,std::string const&,Ogre::ManualResourceLoader *)")]
#[doc(alias = "__ZN4Ogre11MeshManager12createManualERKSsS2_PNS_20ManualResourceLoaderE")]
// was: Ogre::MeshManager::createManual(std::string const&,std::string const&,Ogre::ManualResourceLoader *)
pub fn stub_d10e7c() -> ! {
    todo!("0xd10e7c Ogre::MeshManager::createManual(std::string const&,std::string const&,Ogre::ManualResourceLoader *)")
}

// 0xd10ffc — __ZN4Ogre11MeshManager11createPlaneERKSsS2_RKNS_5PlaneEffiibtffRKNS_7Vector3ENS_14HardwareBuffer5UsageESA_bb
// type: int __fastcall(int, int, int, int, int, float, float, int, int, int, int, float, float, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::MeshManager::createPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
#[doc(alias = "__ZN4Ogre11MeshManager11createPlaneERKSsS2_RKNS_5PlaneEffiibtffRKNS_7Vector3ENS_14HardwareBuffer5UsageESA_bb")]
// was: Ogre::MeshManager::createPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
pub fn stub_d10ffc() -> ! {
    todo!("0xd10ffc Ogre::MeshManager::createPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")
}

// 0xd1122c — __ZN4Ogre11MeshManager17createCurvedPlaneERKSsS2_RKNS_5PlaneEfffiibtffRKNS_7Vector3ENS_14HardwareBuffer5UsageESA_bb
// type: int __fastcall(int, int, int, int, int, float, float, float, int, int, int, int, float, float, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::MeshManager::createCurvedPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")]
#[doc(alias = "__ZN4Ogre11MeshManager17createCurvedPlaneERKSsS2_RKNS_5PlaneEfffiibtffRKNS_7Vector3ENS_14HardwareBuffer5UsageESA_bb")]
// was: Ogre::MeshManager::createCurvedPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)
pub fn stub_d1122c() -> ! {
    todo!("0xd1122c Ogre::MeshManager::createCurvedPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool)")
}

// 0xd11464 — __ZN4Ogre11MeshManager25createCurvedIllusionPlaneERKSsS2_RKNS_5PlaneEfffiibtffRKNS_7Vector3ERKNS_10QuaternionENS_14HardwareBuffer5UsageESD_bbi
// type: int __fastcall(int, int, int, int, int, float, float, float, int, int, int, int, float, float, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::MeshManager::createCurvedIllusionPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool,int)")]
#[doc(alias = "__ZN4Ogre11MeshManager25createCurvedIllusionPlaneERKSsS2_RKNS_5PlaneEfffiibtffRKNS_7Vector3ERKNS_10QuaternionENS_14HardwareBuffer5UsageESD_bbi")]
// was: Ogre::MeshManager::createCurvedIllusionPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool,int)
pub fn stub_d11464() -> ! {
    todo!("0xd11464 Ogre::MeshManager::createCurvedIllusionPlane(std::string const&,std::string const&,Ogre::Plane const&,float,float,float,int,int,bool,unsigned short,float,float,Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::HardwareBuffer::Usage,Ogre::HardwareBuffer::Usage,bool,bool,int)")
}

// 0xd116ac — __ZN4Ogre11MeshManager15tesselate2DMeshEPNS_7SubMeshEttbNS_14HardwareBuffer5UsageEb
// type: int __fastcall(int, int, int, int, int, Ogre::NedPoolingImpl *, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::MeshManager::tesselate2DMesh(Ogre::SubMesh *,unsigned short,unsigned short,bool,Ogre::HardwareBuffer::Usage,bool)")]
#[doc(alias = "__ZN4Ogre11MeshManager15tesselate2DMeshEPNS_7SubMeshEttbNS_14HardwareBuffer5UsageEb")]
// was: Ogre::MeshManager::tesselate2DMesh(Ogre::SubMesh *,unsigned short,unsigned short,bool,Ogre::HardwareBuffer::Usage,bool)
pub fn stub_d116ac() -> ! {
    todo!("0xd116ac Ogre::MeshManager::tesselate2DMesh(Ogre::SubMesh *,unsigned short,unsigned short,bool,Ogre::HardwareBuffer::Usage,bool)")
}

// 0xd11a14 — __ZN4Ogre11MeshManager11getListenerEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::getListener(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager11getListenerEv")]
// was: Ogre::MeshManager::getListener(void)
pub fn stub_d11a14() -> ! {
    todo!("0xd11a14 Ogre::MeshManager::getListener(void)")
}

// 0xd11a1c — __ZN4Ogre11MeshManager12loadResourceEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this, Ogre::Resource *)
#[doc(alias = "Ogre::MeshManager::loadResource(Ogre::Resource *)")]
#[doc(alias = "__ZN4Ogre11MeshManager12loadResourceEPNS_8ResourceE")]
// was: Ogre::MeshManager::loadResource(Ogre::Resource *)
pub fn stub_d11a1c() -> ! {
    todo!("0xd11a1c Ogre::MeshManager::loadResource(Ogre::Resource *)")
}

// 0xd11ec8 — __ZN4Ogre11MeshManager15loadManualPlaneEPNS_4MeshERNS0_15MeshBuildParamsE
#[doc(alias = "Ogre::MeshManager::loadManualPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")]
#[doc(alias = "__ZN4Ogre11MeshManager15loadManualPlaneEPNS_4MeshERNS0_15MeshBuildParamsE")]
// was: Ogre::MeshManager::loadManualPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)
pub fn stub_d11ec8() -> ! {
    todo!("0xd11ec8 Ogre::MeshManager::loadManualPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")
}

// 0xd12a3c — __ZN4Ogre11MeshManager29loadManualCurvedIllusionPlaneEPNS_4MeshERNS0_15MeshBuildParamsE
// type: int __fastcall(int, int)
#[doc(alias = "Ogre::MeshManager::loadManualCurvedIllusionPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")]
#[doc(alias = "__ZN4Ogre11MeshManager29loadManualCurvedIllusionPlaneEPNS_4MeshERNS0_15MeshBuildParamsE")]
// was: Ogre::MeshManager::loadManualCurvedIllusionPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)
pub fn stub_d12a3c() -> ! {
    todo!("0xd12a3c Ogre::MeshManager::loadManualCurvedIllusionPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")
}

// 0xd136f0 — __ZN4Ogre11MeshManager21loadManualCurvedPlaneEPNS_4MeshERNS0_15MeshBuildParamsE
#[doc(alias = "Ogre::MeshManager::loadManualCurvedPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")]
#[doc(alias = "__ZN4Ogre11MeshManager21loadManualCurvedPlaneEPNS_4MeshERNS0_15MeshBuildParamsE")]
// was: Ogre::MeshManager::loadManualCurvedPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)
pub fn stub_d136f0() -> ! {
    todo!("0xd136f0 Ogre::MeshManager::loadManualCurvedPlane(Ogre::Mesh *,Ogre::MeshManager::MeshBuildParams &)")
}

// 0xd14398 — __ZThn152_N4Ogre11MeshManager12loadResourceEPNS_8ResourceE
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this, Ogre::Resource *)
#[doc(alias = "non-virtual thunk toOgre::MeshManager::loadResource(Ogre::Resource *)")]
#[doc(alias = "__ZThn152_N4Ogre11MeshManager12loadResourceEPNS_8ResourceE")]
// was: `non-virtual thunk to'Ogre::MeshManager::loadResource(Ogre::Resource *)
pub fn stub_d14398() -> ! {
    todo!("0xd14398 non-virtual thunk toOgre::MeshManager::loadResource(Ogre::Resource *)")
}

// 0xd143a4 — __ZN4Ogre11MeshManager35setPrepareAllMeshesForShadowVolumesEb
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this, bool)
#[doc(alias = "Ogre::MeshManager::setPrepareAllMeshesForShadowVolumes(bool)")]
#[doc(alias = "__ZN4Ogre11MeshManager35setPrepareAllMeshesForShadowVolumesEb")]
// was: Ogre::MeshManager::setPrepareAllMeshesForShadowVolumes(bool)
pub fn stub_d143a4() -> ! {
    todo!("0xd143a4 Ogre::MeshManager::setPrepareAllMeshesForShadowVolumes(bool)")
}

// 0xd143ac — __ZN4Ogre11MeshManager35getPrepareAllMeshesForShadowVolumesEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::getPrepareAllMeshesForShadowVolumes(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager35getPrepareAllMeshesForShadowVolumesEv")]
// was: Ogre::MeshManager::getPrepareAllMeshesForShadowVolumes(void)
pub fn stub_d143ac() -> ! {
    todo!("0xd143ac Ogre::MeshManager::getPrepareAllMeshesForShadowVolumes(void)")
}

// 0xd143b4 — __ZN4Ogre11MeshManager22getBoundsPaddingFactorEv
// type: _DWORD __fastcall(Ogre::MeshManager *__hidden this)
#[doc(alias = "Ogre::MeshManager::getBoundsPaddingFactor(void)")]
#[doc(alias = "__ZN4Ogre11MeshManager22getBoundsPaddingFactorEv")]
// was: Ogre::MeshManager::getBoundsPaddingFactor(void)
pub fn stub_d143b4() -> ! {
    todo!("0xd143b4 Ogre::MeshManager::getBoundsPaddingFactor(void)")
}

// 0xd143bc — __ZN4Ogre11MeshManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, Ogre::ManualResourceLoader *, int, int, int)
#[doc(alias = "Ogre::MeshManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
#[doc(alias = "__ZN4Ogre11MeshManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
// was: Ogre::MeshManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
pub fn stub_d143bc() -> ! {
    todo!("0xd143bc Ogre::MeshManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xd14494 — __ZNSt3mapIPN4Ogre8ResourceENS0_11MeshManager15MeshBuildParamsESt4lessIS2_ENS0_12STLAllocatorISt4pairIKS2_S4_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS9_
#[doc(alias = "std::map<Ogre::Resource *,Ogre::MeshManager::MeshBuildParams,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Resource * const&)")]
#[doc(alias = "__ZNSt3mapIPN4Ogre8ResourceENS0_11MeshManager15MeshBuildParamsESt4lessIS2_ENS0_12STLAllocatorISt4pairIKS2_S4_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS9_")]
// was: std::map<Ogre::Resource *,Ogre::MeshManager::MeshBuildParams,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Resource * const&)
pub fn stub_d14494() -> ! {
    todo!("0xd14494 std::map<Ogre::Resource *,Ogre::MeshManager::MeshBuildParams,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::Resource * const&)")
}

// 0xd1455c — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)
pub fn stub_d1455c() -> ! {
    todo!("0xd1455c std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")
}

// 0xd14610 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)
pub fn stub_d14610() -> ! {
    todo!("0xd14610 std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")
}

// 0xd14760 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)
pub fn stub_d14760() -> ! {
    todo!("0xd14760 std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams> const&)")
}

// 0xd147cc — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()
pub fn stub_d147cc() -> ! {
    todo!("0xd147cc std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()")
}

// 0xd147d0 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()
pub fn stub_d147d0() -> ! {
    todo!("0xd147d0 std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::Resource *>,false>::~_Rb_tree_impl()")
}

// 0xd147dc — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEE7destroyEv")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::destroy(void)
pub fn stub_d147dc() -> ! {
    todo!("0xd147dc Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::destroy(void)")
}

// 0xd14814 — __ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareVertexBuffer>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20HardwareVertexBufferEE4swapERS2_")]
// was: Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareVertexBuffer>&)
pub fn stub_d14814() -> ! {
    todo!("0xd14814 Ogre::SharedPtr<Ogre::HardwareVertexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareVertexBuffer>&)")
}

// 0xd14830 — __ZNK4Ogre7Matrix411concatenateERKS0_
// type: _DWORD __fastcall(Ogre::Matrix4 *__hidden this, const Ogre::Matrix4 *)
#[doc(alias = "Ogre::Matrix4::concatenate(Ogre::Matrix4 const&)const")]
#[doc(alias = "__ZNK4Ogre7Matrix411concatenateERKS0_")]
// was: Ogre::Matrix4::concatenate(Ogre::Matrix4 const&)const
pub fn stub_d14830() -> ! {
    todo!("0xd14830 Ogre::Matrix4::concatenate(Ogre::Matrix4 const&)const")
}

// 0xd14af0 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED1Ev")]
// was: Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()
pub fn stub_d14af0() -> ! {
    todo!("0xd14af0 Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")
}

// 0xd14be0 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareIndexBuffer> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEEaSERKS2_")]
// was: Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareIndexBuffer> const&)
pub fn stub_d14be0() -> ! {
    todo!("0xd14be0 Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwareIndexBuffer> const&)")
}

// 0xd14d60 — __ZN4Ogre9SharedPtrINS_4MeshEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEED1Ev")]
// was: Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()
pub fn stub_d14d60() -> ! {
    todo!("0xd14d60 Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")
}

// 0xd14e10 — __ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8ResourceESt4pairIKS2_NS0_11MeshManager15MeshBuildParamsEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// was: std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>> *)
pub fn stub_d14e10() -> ! {
    todo!("0xd14e10 std::_Rb_tree<Ogre::Resource *,std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,std::_Select1st<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>>,std::less<Ogre::Resource *>,Ogre::STLAllocator<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Resource * const,Ogre::MeshManager::MeshBuildParams>> *)")
}

// 0xd14e6c — __ZN4Ogre14MeshSerializerC1Ev
// type: _DWORD __fastcall(Ogre::MeshSerializer *__hidden this)
#[doc(alias = "Ogre::MeshSerializer::MeshSerializer(void)")]
#[doc(alias = "__ZN4Ogre14MeshSerializerC1Ev")]
// was: Ogre::MeshSerializer::MeshSerializer(void)
pub fn stub_d14e6c() -> ! {
    todo!("0xd14e6c Ogre::MeshSerializer::MeshSerializer(void)")
}

// 0xd14e78 — __ZN4Ogre14MeshSerializerC2Ev
// type: _DWORD __fastcall(Ogre::MeshSerializer *__hidden this)
#[doc(alias = "Ogre::MeshSerializer::MeshSerializer(void)")]
#[doc(alias = "__ZN4Ogre14MeshSerializerC2Ev")]
// was: Ogre::MeshSerializer::MeshSerializer(void)
pub fn stub_d14e78() -> ! {
    todo!("0xd14e78 Ogre::MeshSerializer::MeshSerializer(void)")
}

// 0xd15800 — __ZN4Ogre14MeshSerializerD0Ev
// type: void __fastcall(Ogre::MeshSerializer *__hidden this)
#[doc(alias = "Ogre::MeshSerializer::~MeshSerializer()")]
#[doc(alias = "__ZN4Ogre14MeshSerializerD0Ev")]
// was: Ogre::MeshSerializer::~MeshSerializer()
pub fn stub_d15800() -> ! {
    todo!("0xd15800 Ogre::MeshSerializer::~MeshSerializer()")
}

// 0xd15890 — __ZN4Ogre14MeshSerializerD1Ev
// type: void __fastcall(Ogre::MeshSerializer *__hidden this)
#[doc(alias = "Ogre::MeshSerializer::~MeshSerializer()")]
#[doc(alias = "__ZN4Ogre14MeshSerializerD1Ev")]
// was: Ogre::MeshSerializer::~MeshSerializer()
pub fn stub_d15890() -> ! {
    todo!("0xd15890 Ogre::MeshSerializer::~MeshSerializer()")
}

// 0xd1589c — __ZN4Ogre14MeshSerializerD2Ev
// type: void __fastcall(Ogre::MeshSerializer *__hidden this)
#[doc(alias = "Ogre::MeshSerializer::~MeshSerializer()")]
#[doc(alias = "__ZN4Ogre14MeshSerializerD2Ev")]
// was: Ogre::MeshSerializer::~MeshSerializer()
pub fn stub_d1589c() -> ! {
    todo!("0xd1589c Ogre::MeshSerializer::~MeshSerializer()")
}

// 0xd16388 — __ZN4Ogre14MeshSerializer11setListenerEPNS_22MeshSerializerListenerE
#[doc(alias = "Ogre::MeshSerializer::setListener(Ogre::MeshSerializerListener *)")]
#[doc(alias = "__ZN4Ogre14MeshSerializer11setListenerEPNS_22MeshSerializerListenerE")]
// was: Ogre::MeshSerializer::setListener(Ogre::MeshSerializerListener *)
pub fn stub_d16388() -> ! {
    todo!("0xd16388 Ogre::MeshSerializer::setListener(Ogre::MeshSerializerListener *)")
}

// 0xd1638c — __ZNSt6vectorIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshSerializer::MeshVersionData **,std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshSerializer::MeshVersionData * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
// was: std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshSerializer::MeshVersionData **,std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshSerializer::MeshVersionData * const&)
pub fn stub_d1638c() -> ! {
    todo!("0xd1638c std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::MeshSerializer::MeshVersionData **,std::vector<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::MeshSerializer::MeshVersionData * const&)")
}

// 0xd16484 — __ZNSt12_Vector_baseIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d16484() -> ! {
    todo!("0xd16484 std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd16488 — __ZNSt12_Vector_baseIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre14MeshSerializer15MeshVersionDataENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
pub fn stub_d16488() -> ! {
    todo!("0xd16488 std::_Vector_base<Ogre::MeshSerializer::MeshVersionData *,Ogre::STLAllocator<Ogre::MeshSerializer::MeshVersionData *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}

// 0xd164c8 — __ZN4Ogre18MeshSerializerImplC1Ev
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this)
#[doc(alias = "Ogre::MeshSerializerImpl::MeshSerializerImpl(void)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImplC1Ev")]
// was: Ogre::MeshSerializerImpl::MeshSerializerImpl(void)
pub fn stub_d164c8() -> ! {
    todo!("0xd164c8 Ogre::MeshSerializerImpl::MeshSerializerImpl(void)")
}

// 0xd1658c — __ZN4Ogre18MeshSerializerImplD0Ev
// type: void __fastcall(Ogre::MeshSerializerImpl *__hidden this)
#[doc(alias = "Ogre::MeshSerializerImpl::~MeshSerializerImpl()")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImplD0Ev")]
// was: Ogre::MeshSerializerImpl::~MeshSerializerImpl()
pub fn stub_d1658c() -> ! {
    todo!("0xd1658c Ogre::MeshSerializerImpl::~MeshSerializerImpl()")
}

// 0xd1661c — __ZN4Ogre18MeshSerializerImplD1Ev
// type: void __fastcall(Ogre::MeshSerializerImpl *__hidden this)
#[doc(alias = "Ogre::MeshSerializerImpl::~MeshSerializerImpl()")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImplD1Ev")]
// was: Ogre::MeshSerializerImpl::~MeshSerializerImpl()
pub fn stub_d1661c() -> ! {
    todo!("0xd1661c Ogre::MeshSerializerImpl::~MeshSerializerImpl()")
}

// 0xd16680 — __ZN4Ogre18MeshSerializerImpl9writeMeshEPKNS_4MeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeMesh(Ogre::Mesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl9writeMeshEPKNS_4MeshE")]
// was: Ogre::MeshSerializerImpl::writeMesh(Ogre::Mesh const*)
pub fn stub_d16680() -> ! {
    todo!("0xd16680 Ogre::MeshSerializerImpl::writeMesh(Ogre::Mesh const*)")
}

// 0xd17258 — __ZN4Ogre18MeshSerializerImpl21writeSubMeshNameTableEPKNS_4MeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshNameTable(Ogre::Mesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl21writeSubMeshNameTableEPKNS_4MeshE")]
// was: Ogre::MeshSerializerImpl::writeSubMeshNameTable(Ogre::Mesh const*)
pub fn stub_d17258() -> ! {
    todo!("0xd17258 Ogre::MeshSerializerImpl::writeSubMeshNameTable(Ogre::Mesh const*)")
}

// 0xd172c4 — __ZN4Ogre18MeshSerializerImpl12writeSubMeshEPKNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::SubMesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMesh(Ogre::SubMesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl12writeSubMeshEPKNS_7SubMeshE")]
// was: Ogre::MeshSerializerImpl::writeSubMesh(Ogre::SubMesh const*)
pub fn stub_d172c4() -> ! {
    todo!("0xd172c4 Ogre::MeshSerializerImpl::writeSubMesh(Ogre::SubMesh const*)")
}

// 0xd176d4 — __ZN4Ogre18MeshSerializerImpl13writeExtremesEPKNS_4MeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeExtremes(Ogre::Mesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl13writeExtremesEPKNS_4MeshE")]
// was: Ogre::MeshSerializerImpl::writeExtremes(Ogre::Mesh const*)
pub fn stub_d176d4() -> ! {
    todo!("0xd176d4 Ogre::MeshSerializerImpl::writeExtremes(Ogre::Mesh const*)")
}

// 0xd17914 — __ZN4Ogre18MeshSerializerImpl20writeSubMeshExtremesEtPKNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, char *, const Ogre::SubMesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshExtremes(unsigned short,Ogre::SubMesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl20writeSubMeshExtremesEtPKNS_7SubMeshE")]
// was: Ogre::MeshSerializerImpl::writeSubMeshExtremes(unsigned short,Ogre::SubMesh const*)
pub fn stub_d17914() -> ! {
    todo!("0xd17914 Ogre::MeshSerializerImpl::writeSubMeshExtremes(unsigned short,Ogre::SubMesh const*)")
}

// 0xd17990 — __ZN4Ogre18MeshSerializerImpl26writeSubMeshTextureAliasesEPKNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::SubMesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshTextureAliases(Ogre::SubMesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl26writeSubMeshTextureAliasesEPKNS_7SubMeshE")]
// was: Ogre::MeshSerializerImpl::writeSubMeshTextureAliases(Ogre::SubMesh const*)
pub fn stub_d17990() -> ! {
    todo!("0xd17990 Ogre::MeshSerializerImpl::writeSubMeshTextureAliases(Ogre::SubMesh const*)")
}

// 0xd17bdc — __ZN4Ogre18MeshSerializerImpl21writeSubMeshOperationEPKNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::SubMesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshOperation(Ogre::SubMesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl21writeSubMeshOperationEPKNS_7SubMeshE")]
// was: Ogre::MeshSerializerImpl::writeSubMeshOperation(Ogre::SubMesh const*)
pub fn stub_d17bdc() -> ! {
    todo!("0xd17bdc Ogre::MeshSerializerImpl::writeSubMeshOperation(Ogre::SubMesh const*)")
}

// 0xd17c10 — __ZN4Ogre18MeshSerializerImpl13writeGeometryEPKNS_10VertexDataE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::MeshSerializerImpl::writeGeometry(Ogre::VertexData const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl13writeGeometryEPKNS_10VertexDataE")]
// was: Ogre::MeshSerializerImpl::writeGeometry(Ogre::VertexData const*)
pub fn stub_d17c10() -> ! {
    todo!("0xd17c10 Ogre::MeshSerializerImpl::writeGeometry(Ogre::VertexData const*)")
}

// 0xd17f48 — __ZN4Ogre18MeshSerializerImpl24calcSubMeshNameTableSizeEPKNS_4MeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshNameTableSize(Ogre::Mesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl24calcSubMeshNameTableSizeEPKNS_4MeshE")]
// was: Ogre::MeshSerializerImpl::calcSubMeshNameTableSize(Ogre::Mesh const*)
pub fn stub_d17f48() -> ! {
    todo!("0xd17f48 Ogre::MeshSerializerImpl::calcSubMeshNameTableSize(Ogre::Mesh const*)")
}

// 0xd17f88 — __ZN4Ogre18MeshSerializerImpl12calcMeshSizeEPKNS_4MeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::calcMeshSize(Ogre::Mesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl12calcMeshSizeEPKNS_4MeshE")]
// was: Ogre::MeshSerializerImpl::calcMeshSize(Ogre::Mesh const*)
pub fn stub_d17f88() -> ! {
    todo!("0xd17f88 Ogre::MeshSerializerImpl::calcMeshSize(Ogre::Mesh const*)")
}

// 0xd18074 — __ZN4Ogre18MeshSerializerImpl15calcSubMeshSizeEPKNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::SubMesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshSize(Ogre::SubMesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl15calcSubMeshSizeEPKNS_7SubMeshE")]
// was: Ogre::MeshSerializerImpl::calcSubMeshSize(Ogre::SubMesh const*)
pub fn stub_d18074() -> ! {
    todo!("0xd18074 Ogre::MeshSerializerImpl::calcSubMeshSize(Ogre::SubMesh const*)")
}

// 0xd18118 — __ZN4Ogre18MeshSerializerImpl24calcSubMeshOperationSizeEPKNS_7SubMeshE
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshOperationSize(Ogre::SubMesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl24calcSubMeshOperationSizeEPKNS_7SubMeshE")]
// was: Ogre::MeshSerializerImpl::calcSubMeshOperationSize(Ogre::SubMesh const*)
pub fn stub_d18118() -> ! {
    todo!("0xd18118 Ogre::MeshSerializerImpl::calcSubMeshOperationSize(Ogre::SubMesh const*)")
}

// 0xd1811c — __ZN4Ogre18MeshSerializerImpl29calcSubMeshTextureAliasesSizeEPKNS_7SubMeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::SubMesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::calcSubMeshTextureAliasesSize(Ogre::SubMesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl29calcSubMeshTextureAliasesSizeEPKNS_7SubMeshE")]
// was: Ogre::MeshSerializerImpl::calcSubMeshTextureAliasesSize(Ogre::SubMesh const*)
pub fn stub_d1811c() -> ! {
    todo!("0xd1811c Ogre::MeshSerializerImpl::calcSubMeshTextureAliasesSize(Ogre::SubMesh const*)")
}

// 0xd1814c — __ZN4Ogre18MeshSerializerImpl16calcGeometrySizeEPKNS_10VertexDataE
#[doc(alias = "Ogre::MeshSerializerImpl::calcGeometrySize(Ogre::VertexData const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl16calcGeometrySizeEPKNS_10VertexDataE")]
// was: Ogre::MeshSerializerImpl::calcGeometrySize(Ogre::VertexData const*)
pub fn stub_d1814c() -> ! {
    todo!("0xd1814c Ogre::MeshSerializerImpl::calcGeometrySize(Ogre::VertexData const*)")
}

// 0xd19ad0 — __ZN4Ogre18MeshSerializerImpl17writeSkeletonLinkERKSs
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const std::string *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeSkeletonLink(std::string const&)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl17writeSkeletonLinkERKSs")]
// was: Ogre::MeshSerializerImpl::writeSkeletonLink(std::string const&)
pub fn stub_d19ad0() -> ! {
    todo!("0xd19ad0 Ogre::MeshSerializerImpl::writeSkeletonLink(std::string const&)")
}

// 0xd19c30 — __ZN4Ogre18MeshSerializerImpl20calcSkeletonLinkSizeERKSs
#[doc(alias = "Ogre::MeshSerializerImpl::calcSkeletonLinkSize(std::string const&)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl20calcSkeletonLinkSizeERKSs")]
// was: Ogre::MeshSerializerImpl::calcSkeletonLinkSize(std::string const&)
pub fn stub_d19c30() -> ! {
    todo!("0xd19c30 Ogre::MeshSerializerImpl::calcSkeletonLinkSize(std::string const&)")
}

// 0xd19c3c — __ZN4Ogre18MeshSerializerImpl23writeMeshBoneAssignmentERKNS_22VertexBoneAssignment_sE
#[doc(alias = "Ogre::MeshSerializerImpl::writeMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl23writeMeshBoneAssignmentERKNS_22VertexBoneAssignment_sE")]
// was: Ogre::MeshSerializerImpl::writeMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)
pub fn stub_d19c3c() -> ! {
    todo!("0xd19c3c Ogre::MeshSerializerImpl::writeMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)")
}

// 0xd19c7c — __ZN4Ogre18MeshSerializerImpl26writeSubMeshBoneAssignmentERKNS_22VertexBoneAssignment_sE
#[doc(alias = "Ogre::MeshSerializerImpl::writeSubMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl26writeSubMeshBoneAssignmentERKNS_22VertexBoneAssignment_sE")]
// was: Ogre::MeshSerializerImpl::writeSubMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)
pub fn stub_d19c7c() -> ! {
    todo!("0xd19c7c Ogre::MeshSerializerImpl::writeSubMeshBoneAssignment(Ogre::VertexBoneAssignment_s const&)")
}

// 0xd19d44 — __ZN4Ogre18MeshSerializerImpl22calcBoneAssignmentSizeEv
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this)
#[doc(alias = "Ogre::MeshSerializerImpl::calcBoneAssignmentSize(void)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl22calcBoneAssignmentSizeEv")]
// was: Ogre::MeshSerializerImpl::calcBoneAssignmentSize(void)
pub fn stub_d19d44() -> ! {
    todo!("0xd19d44 Ogre::MeshSerializerImpl::calcBoneAssignmentSize(void)")
}

// 0xd19d48 — __ZN4Ogre18MeshSerializerImpl12writeLodInfoEPKNS_4MeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodInfo(Ogre::Mesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl12writeLodInfoEPKNS_4MeshE")]
// was: Ogre::MeshSerializerImpl::writeLodInfo(Ogre::Mesh const*)
pub fn stub_d19d48() -> ! {
    todo!("0xd19d48 Ogre::MeshSerializerImpl::writeLodInfo(Ogre::Mesh const*)")
}

// 0xd19dc0 — __ZN4Ogre18MeshSerializerImpl15writeLodSummaryEtbPKNS_11LodStrategyE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, unsigned __int16, bool, const Ogre::LodStrategy *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodSummary(unsigned short,bool,Ogre::LodStrategy const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl15writeLodSummaryEtbPKNS_11LodStrategyE")]
// was: Ogre::MeshSerializerImpl::writeLodSummary(unsigned short,bool,Ogre::LodStrategy const*)
pub fn stub_d19dc0() -> ! {
    todo!("0xd19dc0 Ogre::MeshSerializerImpl::writeLodSummary(unsigned short,bool,Ogre::LodStrategy const*)")
}

// 0xd19e00 — __ZN4Ogre18MeshSerializerImpl19writeLodUsageManualERKNS_12MeshLodUsageE
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodUsageManual(Ogre::MeshLodUsage const&)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl19writeLodUsageManualERKNS_12MeshLodUsageE")]
// was: Ogre::MeshSerializerImpl::writeLodUsageManual(Ogre::MeshLodUsage const&)
pub fn stub_d19e00() -> ! {
    todo!("0xd19e00 Ogre::MeshSerializerImpl::writeLodUsageManual(Ogre::MeshLodUsage const&)")
}

// 0xd19e50 — __ZN4Ogre18MeshSerializerImpl22writeLodUsageGeneratedEPKNS_4MeshERKNS_12MeshLodUsageEt
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *, const Ogre::MeshLodUsage *, unsigned __int16)
#[doc(alias = "Ogre::MeshSerializerImpl::writeLodUsageGenerated(Ogre::Mesh const*,Ogre::MeshLodUsage const&,unsigned short)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl22writeLodUsageGeneratedEPKNS_4MeshERKNS_12MeshLodUsageEt")]
// was: Ogre::MeshSerializerImpl::writeLodUsageGenerated(Ogre::Mesh const*,Ogre::MeshLodUsage const&,unsigned short)
pub fn stub_d19e50() -> ! {
    todo!("0xd19e50 Ogre::MeshSerializerImpl::writeLodUsageGenerated(Ogre::Mesh const*,Ogre::MeshLodUsage const&,unsigned short)")
}

// 0xd1a154 — __ZN4Ogre18MeshSerializerImpl15writeBoundsInfoEPKNS_4MeshE
// type: _DWORD __fastcall(Ogre::MeshSerializerImpl *__hidden this, const Ogre::Mesh *)
#[doc(alias = "Ogre::MeshSerializerImpl::writeBoundsInfo(Ogre::Mesh const*)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl15writeBoundsInfoEPKNS_4MeshE")]
// was: Ogre::MeshSerializerImpl::writeBoundsInfo(Ogre::Mesh const*)
pub fn stub_d1a154() -> ! {
    todo!("0xd1a154 Ogre::MeshSerializerImpl::writeBoundsInfo(Ogre::Mesh const*)")
}

// 0xd1b288 — __ZN4Ogre18MeshSerializerImpl20flipFromLittleEndianEPvmmRKSt4listINS_13VertexElementENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::MeshSerializerImpl::flipFromLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl20flipFromLittleEndianEPvmmRKSt4listINS_13VertexElementENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
// was: Ogre::MeshSerializerImpl::flipFromLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_d1b288() -> ! {
    todo!("0xd1b288 Ogre::MeshSerializerImpl::flipFromLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xd1b2b0 — __ZN4Ogre18MeshSerializerImpl18flipToLittleEndianEPvmmRKSt4listINS_13VertexElementENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::MeshSerializerImpl::flipToLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZN4Ogre18MeshSerializerImpl18flipToLittleEndianEPvmmRKSt4listINS_13VertexElementENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
// was: Ogre::MeshSerializerImpl::flipToLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
pub fn stub_d1b2b0() -> ! {
    todo!("0xd1b2b0 Ogre::MeshSerializerImpl::flipToLittleEndian(void *,unsigned long,unsigned long,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}
