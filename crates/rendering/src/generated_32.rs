//! rendering generated_32 — Ogre::|G3D:: strict 13333 total, 7378 prior, 100 this batch — 0xd5da10..0xd625e4
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xd5da10 — __ZN4Ogre9RadixSortISt6vectorINS_14RenderablePassENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES2_jED1Ev
#[doc(alias = "Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::~RadixSort()")]
// was: Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::~RadixSort()
// IDA 0xd5da10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d5da10() {
}

// 0xd5dad0 — __ZN4Ogre9RadixSortISt6vectorINS_14RenderablePassENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES2_fED1Ev
#[doc(alias = "Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::~RadixSort()")]
// was: Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::~RadixSort()
// IDA 0xd5dad0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d5dad0() {
}

// 0xd5db90 — __ZN4Ogre9RadixSortISt6vectorINS_14RenderablePassENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES2_jE4sortINS_26QueuedRenderableCollection20RadixSortFunctorPassEEEvRS8_T_
#[doc(alias = "void Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::sort<Ogre::QueuedRenderableCollection::RadixSortFunctorPass>(std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::QueuedRenderableCollection::RadixSortFunctorPass)")]
// was: void Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::sort<Ogre::QueuedRenderableCollection::RadixSortFunctorPass>(std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::QueuedRenderableCollection::RadixSortFunctorPass)
// IDA 0xd5db90: 299 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5db90() {
}

// 0xd5dec4 — __ZN4Ogre9RadixSortISt6vectorINS_14RenderablePassENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES2_fE4sortINS_26QueuedRenderableCollection24RadixSortFunctorDistanceEEEvRS8_T_
#[doc(alias = "void Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::sort<Ogre::QueuedRenderableCollection::RadixSortFunctorDistance>(std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::QueuedRenderableCollection::RadixSortFunctorDistance)")]
// was: void Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::sort<Ogre::QueuedRenderableCollection::RadixSortFunctorDistance>(std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>&,Ogre::QueuedRenderableCollection::RadixSortFunctorDistance)
// IDA 0xd5dec4: 271 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5dec4() {
}

// 0xd5e1bc — __ZNSt6vectorIPN4Ogre10RenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Renderable **,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Renderable * const&)")]
// was: std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Renderable **,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Renderable * const&)
// IDA 0xd5e1bc: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_d5e1bc() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd5e2b4 — __ZNSt8_Rb_treeIPN4Ogre4PassESt4pairIKS2_PSt6vectorIPNS0_10RenderableENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ENS0_26QueuedRenderableCollection13PassGroupLessENS8_ISF_SB_EEE16_M_insert_uniqueERKSF_
// type: int __fastcall(int, int, char *)
#[doc(alias = "std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
// was: std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)
// IDA 0xd5e2b4: 139 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5e2b4() {
}

// 0xd5e40c — __ZNSt12_Vector_baseIPN4Ogre10RenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5e40c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d5e40c() {
}

// 0xd5e410 — __ZNSt12_Vector_baseIPN4Ogre10RenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5e410: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d5e410() {
}

// 0xd5e41c — __ZNSt6vectorIN4Ogre14RenderablePassENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, char *, _QWORD *)
#[doc(alias = "std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderablePass*,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass const&)")]
// was: std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderablePass*,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass const&)
// IDA 0xd5e41c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_d5e41c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd5e538 — __ZSt21__inplace_stable_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_26QueuedRenderableCollection23DepthSortDescendingLessEEvT_SF_T0_
#[doc(alias = "void std::__inplace_stable_sort<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: void std::__inplace_stable_sort<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5e538: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5e538() {
}

// 0xd5e59c — __ZSt22__stable_sort_adaptiveIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES4_iNS2_26QueuedRenderableCollection23DepthSortDescendingLessEEvT_SF_T0_T1_T2_
#[doc(alias = "void std::__stable_sort_adaptive<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: void std::__stable_sort_adaptive<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5e59c: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5e59c() {
}

// 0xd5e624 — __ZSt24__merge_sort_with_bufferIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES4_NS2_26QueuedRenderableCollection23DepthSortDescendingLessEEvT_SF_T0_T1_
#[doc(alias = "void std::__merge_sort_with_buffer<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: void std::__merge_sort_with_buffer<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5e624: 120 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5e624() {
}

// 0xd5e754 — __ZSt16__merge_adaptiveIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiS4_NS2_26QueuedRenderableCollection23DepthSortDescendingLessEEvT_SF_SF_T0_SG_T1_SG_T2_
#[doc(alias = "void std::__merge_adaptive<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::RenderablePass *,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: void std::__merge_adaptive<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::RenderablePass *,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5e754: 158 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5e754() {
}

// 0xd5e8e8 — __ZSt5mergeIPN4Ogre14RenderablePassEN9__gnu_cxx17__normal_iteratorIS2_St6vectorIS1_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEESC_NS0_26QueuedRenderableCollection23DepthSortDescendingLessEET1_T_SG_T0_SH_SF_T2_
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::merge<Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: __gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::merge<Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5e8e8: 109 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5e8e8() {
}

// 0xd5e9f8 — __ZSt16__merge_backwardIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES4_SC_NS2_26QueuedRenderableCollection23DepthSortDescendingLessEET1_T_SG_T0_SH_SF_T2_
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__merge_backward<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: __gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__merge_backward<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5e9f8: 168 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5e9f8() {
}

// 0xd5ebb8 — __ZSt11lower_boundIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_26QueuedRenderableCollection23DepthSortDescendingLessEET_SF_SF_RKT0_T1_
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::lower_bound<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass const&,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: __gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::lower_bound<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass const&,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5ebb8: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5ebb8() {
}

// 0xd5ec64 — __ZSt11upper_boundIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES3_NS2_26QueuedRenderableCollection23DepthSortDescendingLessEET_SF_SF_RKT0_T1_
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::upper_bound<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass const&,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: __gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::upper_bound<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass const&,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5ec64: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5ec64() {
}

// 0xd5ed10 — __ZSt17__rotate_adaptiveIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEES4_iET_SD_SD_SD_T1_SE_T0_SE_
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__rotate_adaptive<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,int>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::RenderablePass *,int)")]
// was: __gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::__rotate_adaptive<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,int>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::RenderablePass *,int)
// IDA 0xd5ed10: 138 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5ed10() {
}

// 0xd5ee80 — __ZSt8__rotateIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEEvT_SD_SD_St26random_access_iterator_tag
#[doc(alias = "void std::__rotate<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::random_access_iterator_tag)")]
// was: void std::__rotate<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::random_access_iterator_tag)
// IDA 0xd5ee80: 112 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5ee80() {
}

// 0xd5efb0 — __ZSt5mergeIPN4Ogre14RenderablePassES2_N9__gnu_cxx17__normal_iteratorIS2_St6vectorIS1_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEEENS0_26QueuedRenderableCollection23DepthSortDescendingLessEET1_T_SG_T0_SH_SF_T2_
#[doc(alias = "__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::merge<Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(Ogre::RenderablePass *,Ogre::RenderablePass *,Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: __gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> std::merge<Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(Ogre::RenderablePass *,Ogre::RenderablePass *,Ogre::RenderablePass *,Ogre::RenderablePass *,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5efb0: 109 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5efb0() {
}

// 0xd5f0c0 — __ZSt5mergeIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEESC_S4_NS2_26QueuedRenderableCollection23DepthSortDescendingLessEET1_T_SG_T0_SH_SF_T2_
#[doc(alias = "Ogre::RenderablePass * std::merge<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: Ogre::RenderablePass * std::merge<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderablePass *,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5f0c0: 108 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5f0c0() {
}

// 0xd5f1cc — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEENS2_26QueuedRenderableCollection23DepthSortDescendingLessEEvT_SF_T0_
#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: void std::__insertion_sort<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5f1cc: 124 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5f1cc() {
}

// 0xd5f31c — __ZSt22__merge_without_bufferIN9__gnu_cxx17__normal_iteratorIPN4Ogre14RenderablePassESt6vectorIS3_NS2_12STLAllocatorIS3_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEEEEiNS2_26QueuedRenderableCollection23DepthSortDescendingLessEEvT_SF_SF_T0_SG_T1_
#[doc(alias = "void std::__merge_without_buffer<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)")]
// was: void std::__merge_without_buffer<__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess>(__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,__gnu_cxx::__normal_iterator<Ogre::RenderablePass *,std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int,int,Ogre::QueuedRenderableCollection::DepthSortDescendingLess)
// IDA 0xd5f31c: 119 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5f31c() {
}

// 0xd5f448 — __ZNSt6vectorIN4Ogre14RenderablePassENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
#[doc(alias = "std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd5f448: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5f448() {
}

// 0xd5f544 — __ZN4Ogre9RadixSortISt6vectorINS_14RenderablePassENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEES2_fE9finalPassEif
#[doc(alias = "Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::finalPass(int,float)")]
// was: Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::finalPass(int,float)
// IDA 0xd5f544: 87 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5f544() {
}

// 0xd5f640 — __ZNSt6vectorIN4Ogre9RadixSortIS_INS0_14RenderablePassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES2_fE9SortEntryENS3_ISA_S6_EEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSA_SC_EEmRKSA_
#[doc(alias = "std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry*,std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry const&)")]
// was: std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry*,std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry const&)
// IDA 0xd5f640: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5f640() {
}

// 0xd5f828 — __ZNSt6vectorIN4Ogre9RadixSortIS_INS0_14RenderablePassENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES2_jE9SortEntryENS3_ISA_S6_EEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSA_SC_EEmRKSA_
#[doc(alias = "std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry*,std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry const&)")]
// was: std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry*,std::vector<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry const&)
// IDA 0xd5f828: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5f828() {
}

// 0xd5fa10 — __ZNSt12_Vector_baseIN4Ogre14RenderablePassENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5fa10: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d5fa10() {
}

// 0xd5fa14 — __ZNSt12_Vector_baseIN4Ogre14RenderablePassENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Vector_base<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5fa14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d5fa14() {
}

// 0xd5fa20 — __ZNSt8_Rb_treeIPN4Ogre4PassESt4pairIKS2_PSt6vectorIPNS0_10RenderableENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ENS0_26QueuedRenderableCollection13PassGroupLessENS8_ISF_SB_EEE13_Rb_tree_implISJ_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::QueuedRenderableCollection::PassGroupLess,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::QueuedRenderableCollection::PassGroupLess,false>::~_Rb_tree_impl()
// IDA 0xd5fa20: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d5fa20() {
}

// 0xd5fa24 — __ZNSt8_Rb_treeIPN4Ogre4PassESt4pairIKS2_PSt6vectorIPNS0_10RenderableENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ENS0_26QueuedRenderableCollection13PassGroupLessENS8_ISF_SB_EEE13_Rb_tree_implISJ_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::QueuedRenderableCollection::PassGroupLess,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<Ogre::QueuedRenderableCollection::PassGroupLess,false>::~_Rb_tree_impl()
// IDA 0xd5fa24: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5fa24() {
}

// 0xd5fa30 — __ZNSt12_Vector_baseIN4Ogre9RadixSortISt6vectorINS0_14RenderablePassENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES3_fE9SortEntryENS4_ISB_S7_EEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5fa30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d5fa30() {
}

// 0xd5fa34 — __ZNSt12_Vector_baseIN4Ogre9RadixSortISt6vectorINS0_14RenderablePassENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES3_fE9SortEntryENS4_ISB_S7_EEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,float>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5fa34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d5fa34() {
}

// 0xd5fa40 — __ZNSt12_Vector_baseIN4Ogre9RadixSortISt6vectorINS0_14RenderablePassENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES3_jE9SortEntryENS4_ISB_S7_EEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5fa40: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d5fa40() {
}

// 0xd5fa44 — __ZNSt12_Vector_baseIN4Ogre9RadixSortISt6vectorINS0_14RenderablePassENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEES3_jE9SortEntryENS4_ISB_S7_EEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::STLAllocator<Ogre::RadixSort<std::vector<Ogre::RenderablePass,Ogre::STLAllocator<Ogre::RenderablePass,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::RenderablePass,unsigned int>::SortEntry,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd5fa44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d5fa44() {
}

// 0xd5fa50 — __ZNSt8_Rb_treeIPN4Ogre4PassESt4pairIKS2_PSt6vectorIPNS0_10RenderableENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ENS0_26QueuedRenderableCollection13PassGroupLessENS8_ISF_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISF_E
#[doc(alias = "std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
// was: std::_Rb_tree<Ogre::Pass *,std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,Ogre::QueuedRenderableCollection::PassGroupLess,Ogre::STLAllocator<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::Pass * const,std::vector<Ogre::Renderable *,Ogre::STLAllocator<Ogre::Renderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)
// IDA 0xd5fa50: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5fa50() {
}

// 0xd5fbe0 — __ZN4Ogre12RenderSystemC2Ev
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::RenderSystem(void)")]
// was: Ogre::RenderSystem::RenderSystem(void)
// IDA 0xd5fbe0: 558 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d5fbe0() {
}

// 0xd601d4 — __ZN4Ogre12RenderSystemD0Ev
// type: void __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::~RenderSystem()")]
// was: Ogre::RenderSystem::~RenderSystem()
// IDA 0xd601d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d601d4() {
}

// 0xd60264 — __ZN4Ogre12RenderSystemD1Ev
// type: void __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::~RenderSystem()")]
// was: Ogre::RenderSystem::~RenderSystem()
// IDA 0xd60264: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d60264() {
}

// 0xd60270 — __ZN4Ogre12RenderSystemD2Ev
// type: void __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::~RenderSystem()")]
// was: Ogre::RenderSystem::~RenderSystem()
// IDA 0xd60270: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_d60270() {
}

// 0xd60764 — __ZN4Ogre12RenderSystem18_initRenderTargetsEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_initRenderTargets(void)")]
// was: Ogre::RenderSystem::_initRenderTargets(void)
// IDA 0xd60764: 16 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d60764() {
}

// 0xd60788 — __ZN4Ogre12RenderSystem23_updateAllRenderTargetsEb
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, bool)
#[doc(alias = "Ogre::RenderSystem::_updateAllRenderTargets(bool)")]
// was: Ogre::RenderSystem::_updateAllRenderTargets(bool)
// IDA 0xd60788: 30 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d60788() {
}

// 0xd607cc — __ZN4Ogre12RenderSystem27_swapAllRenderTargetBuffersEb
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, bool)
#[doc(alias = "Ogre::RenderSystem::_swapAllRenderTargetBuffers(bool)")]
// was: Ogre::RenderSystem::_swapAllRenderTargetBuffers(bool)
// IDA 0xd607cc: 30 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d607cc() {
}

// 0xd60810 — __ZN4Ogre12RenderSystem11_initialiseEbRKSs
#[doc(alias = "Ogre::RenderSystem::_initialise(bool,std::string const&)")]
// was: Ogre::RenderSystem::_initialise(bool,std::string const&)
// IDA 0xd60810: 5 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d60810() {
}

// 0xd60820 — __ZN4Ogre12RenderSystem33useCustomRenderSystemCapabilitiesEPNS_24RenderSystemCapabilitiesE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, Ogre::RenderSystemCapabilities *)
#[doc(alias = "Ogre::RenderSystem::useCustomRenderSystemCapabilities(Ogre::RenderSystemCapabilities *)")]
// was: Ogre::RenderSystem::useCustomRenderSystemCapabilities(Ogre::RenderSystemCapabilities *)
// IDA 0xd60820: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d60820() {
}

// 0xd60a00 — __ZN4Ogre12RenderSystem20_createRenderWindowsERKSt6vectorINS_23RenderWindowDescriptionENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEERS1_IPNS_12RenderWindowENS3_ISC_S6_EEE
#[doc(alias = "Ogre::RenderSystem::_createRenderWindows(std::vector<Ogre::RenderWindowDescription,Ogre::STLAllocator<Ogre::RenderWindowDescription,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,std::vector&<Ogre::RenderWindow *,Ogre::STLAllocator<Ogre::RenderWindow,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)")]
// was: Ogre::RenderSystem::_createRenderWindows(std::vector<Ogre::RenderWindowDescription,Ogre::STLAllocator<Ogre::RenderWindowDescription,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&,std::vector&<Ogre::RenderWindow *,Ogre::STLAllocator<Ogre::RenderWindow,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)
// IDA 0xd60a00: 546 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d60a00() {
}

// 0xd6103c — __ZN4Ogre12RenderSystem19destroyRenderWindowERKSs
#[doc(alias = "Ogre::RenderSystem::destroyRenderWindow(std::string const&)")]
// was: Ogre::RenderSystem::destroyRenderWindow(std::string const&)
// IDA 0xd6103c: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d6103c() {
}

// 0xd61048 — __ZN4Ogre12RenderSystem20destroyRenderTextureERKSs
#[doc(alias = "Ogre::RenderSystem::destroyRenderTexture(std::string const&)")]
// was: Ogre::RenderSystem::destroyRenderTexture(std::string const&)
// IDA 0xd61048: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61048() {
}

// 0xd61054 — __ZN4Ogre12RenderSystem19destroyRenderTargetERKSs
#[doc(alias = "Ogre::RenderSystem::destroyRenderTarget(std::string const&)")]
// was: Ogre::RenderSystem::destroyRenderTarget(std::string const&)
// IDA 0xd61054: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61054() {
}

// 0xd6106c — __ZN4Ogre12RenderSystem18attachRenderTargetERNS_12RenderTargetE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, Ogre::RenderTarget *)
#[doc(alias = "Ogre::RenderSystem::attachRenderTarget(Ogre::RenderTarget &)")]
// was: Ogre::RenderSystem::attachRenderTarget(Ogre::RenderTarget &)
// IDA 0xd6106c: 152 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d6106c() {
}

// 0xd61218 — __ZN4Ogre12RenderSystem15getRenderTargetERKSs
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, const std::string *)
#[doc(alias = "Ogre::RenderSystem::getRenderTarget(std::string const&)")]
// was: Ogre::RenderSystem::getRenderTarget(std::string const&)
// IDA 0xd61218: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61218() {
}

// 0xd61238 — __ZN4Ogre12RenderSystem18detachRenderTargetERKSs
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, const std::string *)
#[doc(alias = "Ogre::RenderSystem::detachRenderTarget(std::string const&)")]
// was: Ogre::RenderSystem::detachRenderTarget(std::string const&)
// IDA 0xd61238: 67 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61238() {
}

// 0xd612fc — __ZN4Ogre12RenderSystem12_getViewportEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_getViewport(void)")]
// was: Ogre::RenderSystem::_getViewport(void)
// IDA 0xd612fc: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d612fc() {
}

// 0xd61304 — __ZN4Ogre12RenderSystem23_setTextureUnitSettingsEmRNS_16TextureUnitStateE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, unsigned int, Ogre::TextureUnitState *)
#[doc(alias = "Ogre::RenderSystem::_setTextureUnitSettings(unsigned long,Ogre::TextureUnitState &)")]
// was: Ogre::RenderSystem::_setTextureUnitSettings(unsigned long,Ogre::TextureUnitState &)
// IDA 0xd61304: 211 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61304() {
}

// 0xd61534 — __ZN4Ogre12RenderSystem11_setTextureEmbRKSs
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, unsigned int, bool, const std::string *)
#[doc(alias = "Ogre::RenderSystem::_setTexture(unsigned long,bool,std::string const&)")]
// was: Ogre::RenderSystem::_setTexture(unsigned long,bool,std::string const&)
// IDA 0xd61534: 213 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61534() {
}

// 0xd6174c — __ZN4Ogre12RenderSystem17_setVertexTextureEmRKNS_10TexturePtrE
#[doc(alias = "Ogre::RenderSystem::_setVertexTexture(unsigned long,Ogre::TexturePtr const&)")]
// was: Ogre::RenderSystem::_setVertexTexture(unsigned long,Ogre::TexturePtr const&)
// IDA 0xd6174c: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d6174c() {
}

// 0xd61900 — __ZN4Ogre12RenderSystem19_disableTextureUnitEm
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, unsigned int)
#[doc(alias = "Ogre::RenderSystem::_disableTextureUnit(unsigned long)")]
// was: Ogre::RenderSystem::_disableTextureUnit(unsigned long)
// IDA 0xd61900: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61900() {
}

// 0xd61920 — __ZN4Ogre12RenderSystem24_disableTextureUnitsFromEm
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, unsigned int)
#[doc(alias = "Ogre::RenderSystem::_disableTextureUnitsFrom(unsigned long)")]
// was: Ogre::RenderSystem::_disableTextureUnitsFrom(unsigned long)
// IDA 0xd61920: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61920() {
}

// 0xd61970 — __ZN4Ogre12RenderSystem24_setTextureUnitFilteringEmNS_13FilterOptionsES1_S1_
#[doc(alias = "Ogre::RenderSystem::_setTextureUnitFiltering(unsigned long,Ogre::FilterOptions,Ogre::FilterOptions,Ogre::FilterOptions)")]
// was: Ogre::RenderSystem::_setTextureUnitFiltering(unsigned long,Ogre::FilterOptions,Ogre::FilterOptions,Ogre::FilterOptions)
// IDA 0xd61970: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61970() {
}

// 0xd619b4 — __ZNK4Ogre12RenderSystem15_getCullingModeEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_getCullingMode(void)const")]
// was: Ogre::RenderSystem::_getCullingMode(void)const
// IDA 0xd619b4: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d619b4() {
}

// 0xd619bc — __ZNK4Ogre12RenderSystem23getWaitForVerticalBlankEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::getWaitForVerticalBlank(void)const")]
// was: Ogre::RenderSystem::getWaitForVerticalBlank(void)const
// IDA 0xd619bc: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d619bc() {
}

// 0xd619c4 — __ZN4Ogre12RenderSystem17setDepthBufferForEPNS_12RenderTargetE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, Ogre::RenderTarget *)
#[doc(alias = "Ogre::RenderSystem::setDepthBufferFor(Ogre::RenderTarget *)")]
// was: Ogre::RenderSystem::setDepthBufferFor(Ogre::RenderTarget *)
// IDA 0xd619c4: 221 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d619c4() {
}

// 0xd61c1c — __ZN4Ogre12RenderSystem8shutdownEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::shutdown(void)")]
// was: Ogre::RenderSystem::shutdown(void)
// IDA 0xd61c1c: 110 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61c1c() {
}

// 0xd61d48 — __ZN4Ogre12RenderSystem19_beginGeometryCountEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_beginGeometryCount(void)")]
// was: Ogre::RenderSystem::_beginGeometryCount(void)
// IDA 0xd61d48: 5 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61d48() {
}

// 0xd61d58 — __ZNK4Ogre12RenderSystem13_getFaceCountEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_getFaceCount(void)const")]
// was: Ogre::RenderSystem::_getFaceCount(void)const
// IDA 0xd61d58: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61d58() {
}

// 0xd61d60 — __ZNK4Ogre12RenderSystem14_getBatchCountEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_getBatchCount(void)const")]
// was: Ogre::RenderSystem::_getBatchCount(void)const
// IDA 0xd61d60: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61d60() {
}

// 0xd61d68 — __ZNK4Ogre12RenderSystem15_getVertexCountEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_getVertexCount(void)const")]
// was: Ogre::RenderSystem::_getVertexCount(void)const
// IDA 0xd61d68: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61d68() {
}

// 0xd61d70 — __ZN4Ogre12RenderSystem18convertColourValueERKNS_11ColourValueEPj
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, const Ogre::ColourValue *, unsigned int *)
#[doc(alias = "Ogre::RenderSystem::convertColourValue(Ogre::ColourValue const&,unsigned int *)")]
// was: Ogre::RenderSystem::convertColourValue(Ogre::ColourValue const&,unsigned int *)
// IDA 0xd61d70: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61d70() {
}

// 0xd61d8c — __ZN4Ogre12RenderSystem17_setWorldMatricesEPKNS_7Matrix4Et
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, const Ogre::Matrix4 *, unsigned __int16)
#[doc(alias = "Ogre::RenderSystem::_setWorldMatrices(Ogre::Matrix4 const*,unsigned short)")]
// was: Ogre::RenderSystem::_setWorldMatrices(Ogre::Matrix4 const*,unsigned short)
// IDA 0xd61d8c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61d8c() {
}

// 0xd61da4 — __ZN4Ogre12RenderSystem7_renderERKNS_15RenderOperationE
#[doc(alias = "Ogre::RenderSystem::_render(Ogre::RenderOperation const&)")]
// was: Ogre::RenderSystem::_render(Ogre::RenderOperation const&)
// IDA 0xd61da4: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61da4() {
}

// 0xd61e4c — __ZN4Ogre12RenderSystem22setInvertVertexWindingEb
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, bool)
#[doc(alias = "Ogre::RenderSystem::setInvertVertexWinding(bool)")]
// was: Ogre::RenderSystem::setInvertVertexWinding(bool)
// IDA 0xd61e4c: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61e4c() {
}

// 0xd61e54 — __ZNK4Ogre12RenderSystem22getInvertVertexWindingEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::getInvertVertexWinding(void)const")]
// was: Ogre::RenderSystem::getInvertVertexWinding(void)const
// IDA 0xd61e54: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61e54() {
}

// 0xd61e5c — __ZN4Ogre12RenderSystem12addClipPlaneERKNS_5PlaneE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, const Ogre::Plane *)
#[doc(alias = "Ogre::RenderSystem::addClipPlane(Ogre::Plane const&)")]
// was: Ogre::RenderSystem::addClipPlane(Ogre::Plane const&)
// IDA 0xd61e5c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61e5c() {
}

// 0xd61ea4 — __ZN4Ogre12RenderSystem12addClipPlaneEffff
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, float, float, float, float)
#[doc(alias = "Ogre::RenderSystem::addClipPlane(float,float,float,float)")]
// was: Ogre::RenderSystem::addClipPlane(float,float,float,float)
// IDA 0xd61ea4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61ea4() {
}

// 0xd61ed0 — __ZN4Ogre12RenderSystem13setClipPlanesERKSt6vectorINS_5PlaneENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::RenderSystem::setClipPlanes(std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: Ogre::RenderSystem::setClipPlanes(std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd61ed0: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61ed0() {
}

// 0xd61f60 — __ZN4Ogre12RenderSystem15resetClipPlanesEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::resetClipPlanes(void)")]
// was: Ogre::RenderSystem::resetClipPlanes(void)
// IDA 0xd61f60: 7 insns (LDRD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61f60() {
}

// 0xd61f74 — __ZN4Ogre12RenderSystem20_notifyCameraRemovedEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, const Ogre::Camera *)
#[doc(alias = "Ogre::RenderSystem::_notifyCameraRemoved(Ogre::Camera const*)")]
// was: Ogre::RenderSystem::_notifyCameraRemoved(Ogre::Camera const*)
// IDA 0xd61f74: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61f74() {
}

// 0xd61f9c — __ZN4Ogre12RenderSystem30updatePassIterationRenderStateEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::updatePassIterationRenderState(void)")]
// was: Ogre::RenderSystem::updatePassIterationRenderState(void)
// IDA 0xd61f9c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d61f9c() {
}

// 0xd62004 — __ZN4Ogre12RenderSystem11addListenerEPNS0_8ListenerE
#[doc(alias = "Ogre::RenderSystem::addListener(Ogre::RenderSystem::Listener *)")]
// was: Ogre::RenderSystem::addListener(Ogre::RenderSystem::Listener *)
// IDA 0xd62004: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d62004() {
}

// 0xd62024 — __ZN4Ogre12RenderSystem14removeListenerEPNS0_8ListenerE
#[doc(alias = "Ogre::RenderSystem::removeListener(Ogre::RenderSystem::Listener *)")]
// was: Ogre::RenderSystem::removeListener(Ogre::RenderSystem::Listener *)
// IDA 0xd62024: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d62024() {
}

// 0xd6205c — __ZN4Ogre12RenderSystem9fireEventERKSsPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::RenderSystem::fireEvent(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::RenderSystem::fireEvent(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xd6205c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d6205c() {
}

// 0xd6208c — __ZN4Ogre12RenderSystem29destroyHardwareOcclusionQueryEPNS_22HardwareOcclusionQueryE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, Ogre::HardwareOcclusionQuery *)
#[doc(alias = "Ogre::RenderSystem::destroyHardwareOcclusionQuery(Ogre::HardwareOcclusionQuery *)")]
// was: Ogre::RenderSystem::destroyHardwareOcclusionQuery(Ogre::HardwareOcclusionQuery *)
// IDA 0xd6208c: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d6208c() {
}

// 0xd620cc — __ZN4Ogre12RenderSystem14bindGpuProgramEPNS_10GpuProgramE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, Ogre::GpuProgram *)
#[doc(alias = "Ogre::RenderSystem::bindGpuProgram(Ogre::GpuProgram *)")]
// was: Ogre::RenderSystem::bindGpuProgram(Ogre::GpuProgram *)
// IDA 0xd620cc: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d620cc() {
}

// 0xd62118 — __ZN4Ogre12RenderSystem16unbindGpuProgramENS_14GpuProgramTypeE
#[doc(alias = "Ogre::RenderSystem::unbindGpuProgram(Ogre::GpuProgramType)")]
// was: Ogre::RenderSystem::unbindGpuProgram(Ogre::GpuProgramType)
// IDA 0xd62118: 23 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d62118() {
}

// 0xd62154 — __ZN4Ogre12RenderSystem17isGpuProgramBoundENS_14GpuProgramTypeE
#[doc(alias = "Ogre::RenderSystem::isGpuProgramBound(Ogre::GpuProgramType)")]
// was: Ogre::RenderSystem::isGpuProgramBound(Ogre::GpuProgramType)
// IDA 0xd62154: 18 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d62154() {
}

// 0xd62180 — __ZN4Ogre12RenderSystem31_setTextureProjectionRelativeToEbRKNS_7Vector3E
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, bool, const Vector3 *)
#[doc(alias = "Ogre::RenderSystem::_setTextureProjectionRelativeTo(bool,Ogre::Vector3 const&)")]
// was: Ogre::RenderSystem::_setTextureProjectionRelativeTo(bool,Ogre::Vector3 const&)
// IDA 0xd62180: 8 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d62180() {
}

// 0xd62198 — __ZN4Ogre12RenderSystem11_pauseFrameEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_pauseFrame(void)")]
// was: Ogre::RenderSystem::_pauseFrame(void)
// IDA 0xd62198: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d62198() {
}

// 0xd621ac — __ZN4Ogre12RenderSystem12_resumeFrameEPNS0_19RenderSystemContextE
#[doc(alias = "Ogre::RenderSystem::_resumeFrame(Ogre::RenderSystem::RenderSystemContext *)")]
// was: Ogre::RenderSystem::_resumeFrame(Ogre::RenderSystem::RenderSystemContext *)
// IDA 0xd621ac: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d621ac() {
}

// 0xd621c8 — __ZNK4Ogre12RenderSystem33_getDefaultViewportMaterialSchemeEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::_getDefaultViewportMaterialScheme(void)const")]
// was: Ogre::RenderSystem::_getDefaultViewportMaterialScheme(void)const
// IDA 0xd621c8: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d621c8() {
}

// 0xd622cc — __ZN4Ogre12RenderSystem10startFrameEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::RenderSystem::startFrame(Ogre::RenderWindow *)")]
// was: Ogre::RenderSystem::startFrame(Ogre::RenderWindow *)
// IDA 0xd622cc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d622cc() {
}

// 0xd622d0 — __ZN4Ogre12RenderSystem19beginProfilingEventEPKc
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, const char *)
#[doc(alias = "Ogre::RenderSystem::beginProfilingEvent(char const*)")]
// was: Ogre::RenderSystem::beginProfilingEvent(char const*)
// IDA 0xd622d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d622d0() {
}

// 0xd622d4 — __ZN4Ogre12RenderSystem17endProfilingEventEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::endProfilingEvent(void)")]
// was: Ogre::RenderSystem::endProfilingEvent(void)
// IDA 0xd622d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_d622d4() {
}

// 0xd622d8 — __ZNSt3mapItSt6vectorIPN4Ogre11DepthBufferENS1_12STLAllocatorIS3_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEESt4lessItENS4_ISt4pairIKtS9_ES7_EEEixERSD_
#[doc(alias = "std::map<unsigned short,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)")]
// was: std::map<unsigned short,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)
// IDA 0xd622d8: 104 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d622d8() {
}

// 0xd623f8 — __ZNSt6vectorIN4Ogre5PlaneENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
#[doc(alias = "std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::Plane,Ogre::STLAllocator<Ogre::Plane,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd623f8: 163 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d623f8() {
}

// 0xd625ac — __ZN4Ogre12RenderSystem23getRenderTargetIteratorEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::getRenderTargetIterator(void)")]
// was: Ogre::RenderSystem::getRenderTargetIterator(void)
// IDA 0xd625ac: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d625ac() {
}

// 0xd625b8 — __ZNK4Ogre12RenderSystem33areFixedFunctionLightsInViewSpaceEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::areFixedFunctionLightsInViewSpace(void)const")]
// was: Ogre::RenderSystem::areFixedFunctionLightsInViewSpace(void)const
// IDA 0xd625b8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d625b8() {
}

// 0xd625bc — __ZNK4Ogre12RenderSystem16getDriverVersionEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::getDriverVersion(void)const")]
// was: Ogre::RenderSystem::getDriverVersion(void)const
// IDA 0xd625bc: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d625bc() {
}

// 0xd625c4 — __ZN4Ogre12RenderSystem28setCurrentPassIterationCountEm
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, unsigned int)
#[doc(alias = "Ogre::RenderSystem::setCurrentPassIterationCount(unsigned long)")]
// was: Ogre::RenderSystem::setCurrentPassIterationCount(unsigned long)
// IDA 0xd625c4: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d625c4() {
}

// 0xd625cc — __ZN4Ogre12RenderSystem18setDeriveDepthBiasEbfff
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this, bool, float, float, float)
#[doc(alias = "Ogre::RenderSystem::setDeriveDepthBias(bool,float,float,float)")]
// was: Ogre::RenderSystem::setDeriveDepthBias(bool,float,float,float)
// IDA 0xd625cc: 6 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d625cc() {
}

// 0xd625e4 — __ZNK4Ogre12RenderSystem21getRenderSystemEventsEv
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "Ogre::RenderSystem::getRenderSystemEvents(void)const")]
// was: Ogre::RenderSystem::getRenderSystemEvents(void)const
// IDA 0xd625e4: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_d625e4() {
}
