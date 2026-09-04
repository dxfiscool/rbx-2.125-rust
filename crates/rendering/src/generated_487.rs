//! rendering — Ogre|G3D|Render|Gfx shard 487
//! This shard: 0xc554b4..0xc58b28 (100 stubs, EA-sorted asc, fresh vs /tmp/global_eas.txt)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xc554b4 — __ZNK4Ogre20VertexAnimationTrack19hasNonZeroKeyFramesEv
#[doc(alias = "Ogre::VertexAnimationTrack::hasNonZeroKeyFrames(void)const")]
// was: Ogre::VertexAnimationTrack::hasNonZeroKeyFrames(void)const
// IDA 0xc554b4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c554b4() {
}

// 0xc55514 — __ZN4Ogre20VertexAnimationTrack8optimiseEv
#[doc(alias = "Ogre::VertexAnimationTrack::optimise(void)")]
// was: Ogre::VertexAnimationTrack::optimise(void)
// IDA 0xc55514: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c55514() {
}

// 0xc55518 — __ZN4Ogre20VertexAnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
#[doc(alias = "Ogre::VertexAnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")]
// was: Ogre::VertexAnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)
// IDA 0xc55518: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c55518() {
}

// 0xc5553c — __ZNK4Ogre14AnimationTrack20_keyFrameDataChangedEv
#[doc(alias = "Ogre::AnimationTrack::_keyFrameDataChanged(void)const")]
// was: Ogre::AnimationTrack::_keyFrameDataChanged(void)const
// IDA 0xc5553c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c5553c() {
}

// 0xc55540 — __ZNK4Ogre14AnimationTrack19hasNonZeroKeyFramesEv
#[doc(alias = "Ogre::AnimationTrack::hasNonZeroKeyFrames(void)const")]
// was: Ogre::AnimationTrack::hasNonZeroKeyFrames(void)const
// IDA 0xc55540: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c55540() {
}

// 0xc55544 — __ZN4Ogre14AnimationTrack8optimiseEv
#[doc(alias = "Ogre::AnimationTrack::optimise(void)")]
// was: Ogre::AnimationTrack::optimise(void)
// IDA 0xc55544: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c55544() {
}

// 0xc55548 — __ZN4Ogre14AnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
#[doc(alias = "Ogre::AnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")]
// was: Ogre::AnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)
// IDA 0xc55548: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c55548() {
}

// 0xc5554c — __ZN4Ogre14AnimationTrack11setListenerEPNS0_8ListenerE
#[doc(alias = "Ogre::AnimationTrack::setListener(Ogre::AnimationTrack::Listener *)")]
// was: Ogre::AnimationTrack::setListener(Ogre::AnimationTrack::Listener *)
// IDA 0xc5554c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5554c() {
}

// 0xc55550 — __ZN4Ogre20VertexAnimationTrackD1Ev
#[doc(alias = "Ogre::VertexAnimationTrack::~VertexAnimationTrack()")]
// was: Ogre::VertexAnimationTrack::~VertexAnimationTrack()
// IDA 0xc55550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c55550() {
}

// 0xc5555c — __ZN4Ogre20VertexAnimationTrackD0Ev
#[doc(alias = "Ogre::VertexAnimationTrack::~VertexAnimationTrack()")]
// was: Ogre::VertexAnimationTrack::~VertexAnimationTrack()
// IDA 0xc5555c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c5555c() {
}

// 0xc555ec — __ZNSt6vectorIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::KeyFrame **,std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::KeyFrame * const&)")]
// was: std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::KeyFrame **,std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::KeyFrame * const&)
// IDA 0xc555ec: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_c555ec() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xc556e4 — __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc556e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c556e4() {
}

// 0xc556e8 — __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc556e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c556e8() {
}

// 0xc55728 — __ZN4Ogre14ArchiveManager12getSingletonEv
#[doc(alias = "Ogre::ArchiveManager::getSingleton(void)")]
// was: Ogre::ArchiveManager::getSingleton(void)
// IDA 0xc55728: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c55728() {
}

// 0xc55738 — __ZN4Ogre14ArchiveManagerC1Ev
#[doc(alias = "Ogre::ArchiveManager::ArchiveManager(void)")]
// was: Ogre::ArchiveManager::ArchiveManager(void)
// IDA 0xc55738: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c55738() {
}

// 0xc557a0 — __ZN4Ogre14ArchiveManager4loadERKSsS2_
#[doc(alias = "Ogre::ArchiveManager::load(std::string const&,std::string const&)")]
// was: Ogre::ArchiveManager::load(std::string const&,std::string const&)
// IDA 0xc557a0: 223 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c557a0() {
}

// 0xc55a18 — __ZN4Ogre14ArchiveManagerD0Ev
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: Ogre::ArchiveManager::~ArchiveManager()
// IDA 0xc55a18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c55a18() {
}

// 0xc55aa8 — __ZN4Ogre14ArchiveManagerD1Ev
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: Ogre::ArchiveManager::~ArchiveManager()
// IDA 0xc55aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c55aa8() {
}

// 0xc55ab4 — __ZN4Ogre14ArchiveManagerD2Ev
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: Ogre::ArchiveManager::~ArchiveManager()
// IDA 0xc55ab4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c55ab4() {
}

// 0xc55dfc — __ZN4Ogre14ArchiveManager17addArchiveFactoryEPNS_14ArchiveFactoryE
#[doc(alias = "Ogre::ArchiveManager::addArchiveFactory(Ogre::ArchiveFactory *)")]
// was: Ogre::ArchiveManager::addArchiveFactory(Ogre::ArchiveFactory *)
// IDA 0xc55dfc: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c55dfc() {
}

// 0xc56120 — __ZNSt3mapISsPN4Ogre7ArchiveESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::Archive *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::Archive *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xc56120: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56120() {
}

// 0xc562dc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ArchiveFactory *> const&)
// IDA 0xc562dc: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c562dc() {
}

// 0xc563c0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ArchiveFactory *> const&)
// IDA 0xc563c0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c563c0() {
}

// 0xc56514 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Archive *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Archive *>> *)
// IDA 0xc56514: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56514() {
}

// 0xc5658c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Archive *>>,std::pair<std::string const,Ogre::Archive *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Archive *>>,std::pair<std::string const,Ogre::Archive *> const&)
// IDA 0xc5658c: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5658c() {
}

// 0xc5676c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Archive *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Archive *> const&)
// IDA 0xc5676c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5676c() {
}

// 0xc568c0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Archive *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Archive *> const&)
// IDA 0xc568c0: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c568c0() {
}

// 0xc569a4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xc569a4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c569a4() {
}

// 0xc56a48 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xc56a48: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56a48() {
}

// 0xc56aec — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc56aec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c56aec() {
}

// 0xc56af0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc56af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c56af0() {
}

// 0xc56afc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc56afc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c56afc() {
}

// 0xc56b00 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc56b00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c56b00() {
}

// 0xc56b0c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ArchiveFactory *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ArchiveFactory *>> *)
// IDA 0xc56b0c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56b0c() {
}

// 0xc56bb8 — __ZN4Ogre19AutoParamDataSourceC1Ev
#[doc(alias = "Ogre::AutoParamDataSource::AutoParamDataSource(void)")]
// was: Ogre::AutoParamDataSource::AutoParamDataSource(void)
// IDA 0xc56bb8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56bb8() {
}

// 0xc56bc4 — __ZN4Ogre19AutoParamDataSourceC2Ev
#[doc(alias = "Ogre::AutoParamDataSource::AutoParamDataSource(void)")]
// was: Ogre::AutoParamDataSource::AutoParamDataSource(void)
// IDA 0xc56bc4: 234 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56bc4() {
}

// 0xc56e7c — __ZN4Ogre19AutoParamDataSourceD0Ev
#[doc(alias = "Ogre::AutoParamDataSource::~AutoParamDataSource()")]
// was: Ogre::AutoParamDataSource::~AutoParamDataSource()
// IDA 0xc56e7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c56e7c() {
}

// 0xc56f20 — __ZN4Ogre19AutoParamDataSourceD1Ev
#[doc(alias = "Ogre::AutoParamDataSource::~AutoParamDataSource()")]
// was: Ogre::AutoParamDataSource::~AutoParamDataSource()
// IDA 0xc56f20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c56f20() {
}

// 0xc56f44 — __ZN4Ogre19AutoParamDataSource20setCurrentRenderableEPKNS_10RenderableE
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentRenderable(Ogre::Renderable const*)")]
// was: Ogre::AutoParamDataSource::setCurrentRenderable(Ogre::Renderable const*)
// IDA 0xc56f44: 26 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56f44() {
}

// 0xc56f94 — __ZN4Ogre19AutoParamDataSource16setCurrentCameraEPKNS_6CameraEb
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentCamera(Ogre::Camera const*,bool)")]
// was: Ogre::AutoParamDataSource::setCurrentCamera(Ogre::Camera const*,bool)
// IDA 0xc56f94: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56f94() {
}

// 0xc56ffc — __ZN4Ogre19AutoParamDataSource19setCurrentLightListEPKNS_12HashedVectorIPNS_5LightEEE
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentLightList(Ogre::HashedVector<Ogre::Light *> const*)")]
// was: Ogre::AutoParamDataSource::setCurrentLightList(Ogre::HashedVector<Ogre::Light *> const*)
// IDA 0xc56ffc: 29 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c56ffc() {
}

// 0xc57048 — __ZNK4Ogre19AutoParamDataSource14getLightNumberEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightNumber(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightNumber(unsigned long)const
// IDA 0xc57048: 16 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57048() {
}

// 0xc57078 — __ZNK4Ogre19AutoParamDataSource21getLightDiffuseColourEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightDiffuseColour(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightDiffuseColour(unsigned long)const
// IDA 0xc57078: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57078() {
}

// 0xc570a8 — __ZNK4Ogre19AutoParamDataSource22getLightSpecularColourEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightSpecularColour(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightSpecularColour(unsigned long)const
// IDA 0xc570a8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c570a8() {
}

// 0xc570d8 — __ZNK4Ogre19AutoParamDataSource30getLightDiffuseColourWithPowerEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightDiffuseColourWithPower(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightDiffuseColourWithPower(unsigned long)const
// IDA 0xc570d8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c570d8() {
}

// 0xc57140 — __ZNK4Ogre19AutoParamDataSource31getLightSpecularColourWithPowerEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightSpecularColourWithPower(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightSpecularColourWithPower(unsigned long)const
// IDA 0xc57140: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57140() {
}

// 0xc571a8 — __ZNK4Ogre19AutoParamDataSource16getLightPositionEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightPosition(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightPosition(unsigned long)const
// IDA 0xc571a8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c571a8() {
}

// 0xc571d4 — __ZNK4Ogre19AutoParamDataSource18getLightAs4DVectorEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightAs4DVector(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightAs4DVector(unsigned long)const
// IDA 0xc571d4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c571d4() {
}

// 0xc57200 — __ZNK4Ogre19AutoParamDataSource17getLightDirectionEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightDirection(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightDirection(unsigned long)const
// IDA 0xc57200: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57200() {
}

// 0xc57230 — __ZNK4Ogre19AutoParamDataSource18getLightPowerScaleEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightPowerScale(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightPowerScale(unsigned long)const
// IDA 0xc57230: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57230() {
}

// 0xc57260 — __ZNK4Ogre19AutoParamDataSource19getLightAttenuationEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightAttenuation(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightAttenuation(unsigned long)const
// IDA 0xc57260: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57260() {
}

// 0xc572b8 — __ZNK4Ogre19AutoParamDataSource18getSpotlightParamsEm
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightParams(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getSpotlightParams(unsigned long)const
// IDA 0xc572b8: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c572b8() {
}

// 0xc57374 — __ZN4Ogre19AutoParamDataSource20setMainCamBoundsInfoEPNS_24VisibleObjectsBoundsInfoE
#[doc(alias = "Ogre::AutoParamDataSource::setMainCamBoundsInfo(Ogre::VisibleObjectsBoundsInfo *)")]
// was: Ogre::AutoParamDataSource::setMainCamBoundsInfo(Ogre::VisibleObjectsBoundsInfo *)
// IDA 0xc57374: 6 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57374() {
}

// 0xc57384 — __ZN4Ogre19AutoParamDataSource22setCurrentSceneManagerEPKNS_12SceneManagerE
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentSceneManager(Ogre::SceneManager const*)")]
// was: Ogre::AutoParamDataSource::setCurrentSceneManager(Ogre::SceneManager const*)
// IDA 0xc57384: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57384() {
}

// 0xc5738c — __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
#[doc(alias = "Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)")]
// was: Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)
// IDA 0xc5738c: 8 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5738c() {
}

// 0xc573a4 — __ZNK4Ogre19AutoParamDataSource14getWorldMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getWorldMatrix(void)const
// IDA 0xc573a4: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c573a4() {
}

// 0xc5745c — __ZNK4Ogre19AutoParamDataSource19getWorldMatrixCountEv
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrixCount(void)const")]
// was: Ogre::AutoParamDataSource::getWorldMatrixCount(void)const
// IDA 0xc5745c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5745c() {
}

// 0xc57474 — __ZNK4Ogre19AutoParamDataSource19getWorldMatrixArrayEv
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrixArray(void)const")]
// was: Ogre::AutoParamDataSource::getWorldMatrixArray(void)const
// IDA 0xc57474: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57474() {
}

// 0xc5748c — __ZNK4Ogre19AutoParamDataSource13getViewMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getViewMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getViewMatrix(void)const
// IDA 0xc5748c: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5748c() {
}

// 0xc57588 — __ZNK4Ogre19AutoParamDataSource23getViewProjectionMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getViewProjectionMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getViewProjectionMatrix(void)const
// IDA 0xc57588: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57588() {
}

// 0xc5760c — __ZNK4Ogre19AutoParamDataSource19getProjectionMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getProjectionMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getProjectionMatrix(void)const
// IDA 0xc5760c: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5760c() {
}

// 0xc57710 — __ZNK4Ogre19AutoParamDataSource18getWorldViewMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getWorldViewMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getWorldViewMatrix(void)const
// IDA 0xc57710: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57710() {
}

// 0xc57794 — __ZNK4Ogre19AutoParamDataSource22getWorldViewProjMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getWorldViewProjMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getWorldViewProjMatrix(void)const
// IDA 0xc57794: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57794() {
}

// 0xc57818 — __ZNK4Ogre19AutoParamDataSource21getInverseWorldMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getInverseWorldMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getInverseWorldMatrix(void)const
// IDA 0xc57818: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57818() {
}

// 0xc57880 — __ZNK4Ogre19AutoParamDataSource25getInverseWorldViewMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getInverseWorldViewMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getInverseWorldViewMatrix(void)const
// IDA 0xc57880: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57880() {
}

// 0xc578e8 — __ZNK4Ogre19AutoParamDataSource20getInverseViewMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getInverseViewMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getInverseViewMatrix(void)const
// IDA 0xc578e8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c578e8() {
}

// 0xc57950 — __ZNK4Ogre19AutoParamDataSource30getInverseTransposeWorldMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTransposeWorldMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getInverseTransposeWorldMatrix(void)const
// IDA 0xc57950: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57950() {
}

// 0xc57a5c — __ZNK4Ogre19AutoParamDataSource34getInverseTransposeWorldViewMatrixEv
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTransposeWorldViewMatrix(void)const")]
// was: Ogre::AutoParamDataSource::getInverseTransposeWorldViewMatrix(void)const
// IDA 0xc57a5c: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57a5c() {
}

// 0xc57b64 — __ZNK4Ogre19AutoParamDataSource17getCameraPositionEv
#[doc(alias = "Ogre::AutoParamDataSource::getCameraPosition(void)const")]
// was: Ogre::AutoParamDataSource::getCameraPosition(void)const
// IDA 0xc57b64: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57b64() {
}

// 0xc57bf4 — __ZNK4Ogre19AutoParamDataSource28getCameraPositionObjectSpaceEv
#[doc(alias = "Ogre::AutoParamDataSource::getCameraPositionObjectSpace(void)const")]
// was: Ogre::AutoParamDataSource::getCameraPositionObjectSpace(void)const
// IDA 0xc57bf4: 121 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57bf4() {
}

// 0xc57da4 — __ZNK4Ogre19AutoParamDataSource20getLodCameraPositionEv
#[doc(alias = "Ogre::AutoParamDataSource::getLodCameraPosition(void)const")]
// was: Ogre::AutoParamDataSource::getLodCameraPosition(void)const
// IDA 0xc57da4: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57da4() {
}

// 0xc57e40 — __ZNK4Ogre19AutoParamDataSource31getLodCameraPositionObjectSpaceEv
#[doc(alias = "Ogre::AutoParamDataSource::getLodCameraPositionObjectSpace(void)const")]
// was: Ogre::AutoParamDataSource::getLodCameraPositionObjectSpace(void)const
// IDA 0xc57e40: 133 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c57e40() {
}

// 0xc58014 — __ZN4Ogre19AutoParamDataSource21setAmbientLightColourERKNS_11ColourValueE
#[doc(alias = "Ogre::AutoParamDataSource::setAmbientLightColour(Ogre::ColourValue const&)")]
// was: Ogre::AutoParamDataSource::setAmbientLightColour(Ogre::ColourValue const&)
// IDA 0xc58014: 5 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58014() {
}

// 0xc58024 — __ZNK4Ogre19AutoParamDataSource13getLightCountEv
#[doc(alias = "Ogre::AutoParamDataSource::getLightCount(void)const")]
// was: Ogre::AutoParamDataSource::getLightCount(void)const
// IDA 0xc58024: 9 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58024() {
}

// 0xc58040 — __ZNK4Ogre19AutoParamDataSource20getLightCastsShadowsEm
#[doc(alias = "Ogre::AutoParamDataSource::getLightCastsShadows(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getLightCastsShadows(unsigned long)const
// IDA 0xc58040: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58040() {
}

// 0xc58080 — __ZNK4Ogre19AutoParamDataSource21getAmbientLightColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getAmbientLightColour(void)const")]
// was: Ogre::AutoParamDataSource::getAmbientLightColour(void)const
// IDA 0xc58080: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58080() {
}

// 0xc58088 — __ZN4Ogre19AutoParamDataSource14setCurrentPassEPKNS_4PassE
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentPass(Ogre::Pass const*)")]
// was: Ogre::AutoParamDataSource::setCurrentPass(Ogre::Pass const*)
// IDA 0xc58088: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58088() {
}

// 0xc58090 — __ZNK4Ogre19AutoParamDataSource14getCurrentPassEv
#[doc(alias = "Ogre::AutoParamDataSource::getCurrentPass(void)const")]
// was: Ogre::AutoParamDataSource::getCurrentPass(void)const
// IDA 0xc58090: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58090() {
}

// 0xc58098 — __ZNK4Ogre19AutoParamDataSource14getTextureSizeEm
#[doc(alias = "Ogre::AutoParamDataSource::getTextureSize(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getTextureSize(unsigned long)const
// IDA 0xc58098: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58098() {
}

// 0xc58128 — __ZNK4Ogre19AutoParamDataSource21getInverseTextureSizeEm
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTextureSize(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getInverseTextureSize(unsigned long)const
// IDA 0xc58128: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58128() {
}

// 0xc58174 — __ZNK4Ogre19AutoParamDataSource20getPackedTextureSizeEm
#[doc(alias = "Ogre::AutoParamDataSource::getPackedTextureSize(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getPackedTextureSize(unsigned long)const
// IDA 0xc58174: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58174() {
}

// 0xc581b4 — __ZNK4Ogre19AutoParamDataSource23getSurfaceAmbientColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceAmbientColour(void)const")]
// was: Ogre::AutoParamDataSource::getSurfaceAmbientColour(void)const
// IDA 0xc581b4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c581b4() {
}

// 0xc581c4 — __ZNK4Ogre19AutoParamDataSource23getSurfaceDiffuseColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceDiffuseColour(void)const")]
// was: Ogre::AutoParamDataSource::getSurfaceDiffuseColour(void)const
// IDA 0xc581c4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c581c4() {
}

// 0xc581d4 — __ZNK4Ogre19AutoParamDataSource24getSurfaceSpecularColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceSpecularColour(void)const")]
// was: Ogre::AutoParamDataSource::getSurfaceSpecularColour(void)const
// IDA 0xc581d4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c581d4() {
}

// 0xc581e4 — __ZNK4Ogre19AutoParamDataSource24getSurfaceEmissiveColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceEmissiveColour(void)const")]
// was: Ogre::AutoParamDataSource::getSurfaceEmissiveColour(void)const
// IDA 0xc581e4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c581e4() {
}

// 0xc581f4 — __ZNK4Ogre19AutoParamDataSource19getSurfaceShininessEv
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceShininess(void)const")]
// was: Ogre::AutoParamDataSource::getSurfaceShininess(void)const
// IDA 0xc581f4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c581f4() {
}

// 0xc58204 — __ZNK4Ogre19AutoParamDataSource28getDerivedAmbientLightColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getDerivedAmbientLightColour(void)const")]
// was: Ogre::AutoParamDataSource::getDerivedAmbientLightColour(void)const
// IDA 0xc58204: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58204() {
}

// 0xc58268 — __ZNK4Ogre19AutoParamDataSource21getDerivedSceneColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getDerivedSceneColour(void)const")]
// was: Ogre::AutoParamDataSource::getDerivedSceneColour(void)const
// IDA 0xc58268: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58268() {
}

// 0xc582cc — __ZN4Ogre19AutoParamDataSource6setFogENS_7FogModeERKNS_11ColourValueEfff
#[doc(alias = "Ogre::AutoParamDataSource::setFog(Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")]
// was: Ogre::AutoParamDataSource::setFog(Ogre::FogMode,Ogre::ColourValue const&,float,float,float)
// IDA 0xc582cc: 25 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c582cc() {
}

// 0xc58328 — __ZNK4Ogre19AutoParamDataSource12getFogColourEv
#[doc(alias = "Ogre::AutoParamDataSource::getFogColour(void)const")]
// was: Ogre::AutoParamDataSource::getFogColour(void)const
// IDA 0xc58328: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58328() {
}

// 0xc58330 — __ZNK4Ogre19AutoParamDataSource12getFogParamsEv
#[doc(alias = "Ogre::AutoParamDataSource::getFogParams(void)const")]
// was: Ogre::AutoParamDataSource::getFogParams(void)const
// IDA 0xc58330: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58330() {
}

// 0xc58338 — __ZN4Ogre19AutoParamDataSource19setTextureProjectorEPKNS_7FrustumEm
#[doc(alias = "Ogre::AutoParamDataSource::setTextureProjector(Ogre::Frustum const*,unsigned long)")]
// was: Ogre::AutoParamDataSource::setTextureProjector(Ogre::Frustum const*,unsigned long)
// IDA 0xc58338: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58338() {
}

// 0xc5836c — __ZNK4Ogre19AutoParamDataSource24getTextureViewProjMatrixEm
#[doc(alias = "Ogre::AutoParamDataSource::getTextureViewProjMatrix(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getTextureViewProjMatrix(unsigned long)const
// IDA 0xc5836c: 103 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5836c() {
}

// 0xc5849c — __ZNK4Ogre19AutoParamDataSource29getTextureWorldViewProjMatrixEm
#[doc(alias = "Ogre::AutoParamDataSource::getTextureWorldViewProjMatrix(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getTextureWorldViewProjMatrix(unsigned long)const
// IDA 0xc5849c: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5849c() {
}

// 0xc5855c — __ZNK4Ogre19AutoParamDataSource26getSpotlightViewProjMatrixEm
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightViewProjMatrix(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getSpotlightViewProjMatrix(unsigned long)const
// IDA 0xc5855c: 349 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5855c() {
}

// 0xc58a08 — __ZNK4Ogre19AutoParamDataSource31getSpotlightWorldViewProjMatrixEm
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightWorldViewProjMatrix(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getSpotlightWorldViewProjMatrix(unsigned long)const
// IDA 0xc58a08: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58a08() {
}

// 0xc58ae4 — __ZNK4Ogre19AutoParamDataSource25getTextureTransformMatrixEm
#[doc(alias = "Ogre::AutoParamDataSource::getTextureTransformMatrix(unsigned long)const")]
// was: Ogre::AutoParamDataSource::getTextureTransformMatrix(unsigned long)const
// IDA 0xc58ae4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58ae4() {
}

// 0xc58b18 — __ZN4Ogre19AutoParamDataSource22setCurrentRenderTargetEPKNS_12RenderTargetE
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentRenderTarget(Ogre::RenderTarget const*)")]
// was: Ogre::AutoParamDataSource::setCurrentRenderTarget(Ogre::RenderTarget const*)
// IDA 0xc58b18: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58b18() {
}

// 0xc58b20 — __ZNK4Ogre19AutoParamDataSource22getCurrentRenderTargetEv
#[doc(alias = "Ogre::AutoParamDataSource::getCurrentRenderTarget(void)const")]
// was: Ogre::AutoParamDataSource::getCurrentRenderTarget(void)const
// IDA 0xc58b20: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58b20() {
}

// 0xc58b28 — __ZN4Ogre19AutoParamDataSource18setCurrentViewportEPKNS_8ViewportE
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentViewport(Ogre::Viewport const*)")]
// was: Ogre::AutoParamDataSource::setCurrentViewport(Ogre::Viewport const*)
// IDA 0xc58b28: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c58b28() {
}
