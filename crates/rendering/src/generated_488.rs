//! rendering shard 488 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xc554b4..0xc58b28, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter (Ogre|G3D|GLES|ViewRbxGfx|RBX+Render), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xc554b4 — __ZNK4Ogre20VertexAnimationTrack19hasNonZeroKeyFramesEv
// type: bool __fastcall(Ogre::VertexAnimationTrack *this)
#[doc(alias = "Ogre::VertexAnimationTrack::hasNonZeroKeyFrames(void)const")]
// was: __ZNK4Ogre20VertexAnimationTrack19hasNonZeroKeyFramesEv
// IDA 0xc554b4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc554b4() {
}


// 0xc55514 — __ZN4Ogre20VertexAnimationTrack8optimiseEv
// type: void __fastcall(Ogre::VertexAnimationTrack *this)
#[doc(alias = "Ogre::VertexAnimationTrack::optimise(void)")]
// was: __ZN4Ogre20VertexAnimationTrack8optimiseEv
// IDA 0xc55514: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc55514() {
}


// 0xc55518 — __ZN4Ogre20VertexAnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
// type: int __fastcall(int this, const Ogre::KeyFrame *)
#[doc(alias = "Ogre::VertexAnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")]
// was: __ZN4Ogre20VertexAnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
// IDA 0xc55518: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc55518() {
}


// 0xc5553c — __ZNK4Ogre14AnimationTrack20_keyFrameDataChangedEv
// type: void __fastcall(Ogre::AnimationTrack *this)
#[doc(alias = "Ogre::AnimationTrack::_keyFrameDataChanged(void)const")]
// was: __ZNK4Ogre14AnimationTrack20_keyFrameDataChangedEv
// IDA 0xc5553c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc5553c() {
}


// 0xc55540 — __ZNK4Ogre14AnimationTrack19hasNonZeroKeyFramesEv
// type: int __fastcall(Ogre::AnimationTrack *this)
#[doc(alias = "Ogre::AnimationTrack::hasNonZeroKeyFrames(void)const")]
// was: __ZNK4Ogre14AnimationTrack19hasNonZeroKeyFramesEv
// IDA 0xc55540: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc55540() {
}


// 0xc55544 — __ZN4Ogre14AnimationTrack8optimiseEv
// type: void __fastcall(Ogre::AnimationTrack *this)
#[doc(alias = "Ogre::AnimationTrack::optimise(void)")]
// was: __ZN4Ogre14AnimationTrack8optimiseEv
// IDA 0xc55544: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc55544() {
}


// 0xc55548 — __ZN4Ogre14AnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
// type: void()
#[doc(alias = "Ogre::AnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")]
// was: __ZN4Ogre14AnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
// IDA 0xc55548: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc55548() {
}


// 0xc5554c — __ZN4Ogre14AnimationTrack11setListenerEPNS0_8ListenerE
// type: int __fastcall(int result, int)
#[doc(alias = "Ogre::AnimationTrack::setListener(Ogre::AnimationTrack::Listener *)")]
// was: __ZN4Ogre14AnimationTrack11setListenerEPNS0_8ListenerE
// IDA 0xc5554c: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5554c() {
}


// 0xc55550 — __ZN4Ogre20VertexAnimationTrackD1Ev
// type: void __fastcall(Ogre::NedPoolingImpl **this)
#[doc(alias = "Ogre::VertexAnimationTrack::~VertexAnimationTrack()")]
// was: __ZN4Ogre20VertexAnimationTrackD1Ev
// IDA 0xc55550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc55550() {
}


// 0xc5555c — __ZN4Ogre20VertexAnimationTrackD0Ev
// type: void __fastcall(Ogre::NedPoolingImpl **this)
#[doc(alias = "Ogre::VertexAnimationTrack::~VertexAnimationTrack()")]
// was: __ZN4Ogre20VertexAnimationTrackD0Ev
// IDA 0xc5555c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc5555c() {
}


// 0xc555ec — __ZNSt6vectorIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: _DWORD *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::KeyFrame **,std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::KeyFrame * const&)")]
// was: __ZNSt6vectorIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// IDA 0xc555ec: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xc555ec() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0xc556e4 — __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// type: void()
#[doc(alias = "std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// IDA 0xc556e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc556e4() {
}


// 0xc556e8 — __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// IDA 0xc556e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc556e8() {
}


// 0xc55728 — __ZN4Ogre14ArchiveManager12getSingletonEv
// type: int __fastcall(Ogre::ArchiveManager *this)
#[doc(alias = "Ogre::ArchiveManager::getSingleton(void)")]
// was: __ZN4Ogre14ArchiveManager12getSingletonEv
// IDA 0xc55728: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc55728() {
}


// 0xc55738 — __ZN4Ogre14ArchiveManagerC1Ev
// type: int __fastcall(int this)
#[doc(alias = "Ogre::ArchiveManager::ArchiveManager(void)")]
// was: __ZN4Ogre14ArchiveManagerC1Ev
// IDA 0xc55738: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc55738() {
}


// 0xc557a0 — __ZN4Ogre14ArchiveManager4loadERKSsS2_
// type: int __fastcall(Ogre::ArchiveManager *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::ArchiveManager::load(std::string const&,std::string const&)")]
// was: __ZN4Ogre14ArchiveManager4loadERKSsS2_
// IDA 0xc557a0: 223 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc557a0() {
}


// 0xc55a18 — __ZN4Ogre14ArchiveManagerD0Ev
// type: void __fastcall(Ogre::ArchiveManager *__hidden this)
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: __ZN4Ogre14ArchiveManagerD0Ev
// IDA 0xc55a18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc55a18() {
}


// 0xc55aa8 — __ZN4Ogre14ArchiveManagerD1Ev
// type: void __fastcall(Ogre::ArchiveManager *__hidden this)
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: __ZN4Ogre14ArchiveManagerD1Ev
// IDA 0xc55aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc55aa8() {
}


// 0xc55ab4 — __ZN4Ogre14ArchiveManagerD2Ev
// type: void __fastcall(Ogre::ArchiveManager *__hidden this)
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: __ZN4Ogre14ArchiveManagerD2Ev
// IDA 0xc55ab4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc55ab4() {
}


// 0xc55dfc — __ZN4Ogre14ArchiveManager17addArchiveFactoryEPNS_14ArchiveFactoryE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::ArchiveManager::addArchiveFactory(Ogre::ArchiveFactory *)")]
// was: __ZN4Ogre14ArchiveManager17addArchiveFactoryEPNS_14ArchiveFactoryE
// IDA 0xc55dfc: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc55dfc() {
}


// 0xc56120 — __ZNSt3mapISsPN4Ogre7ArchiveESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: 
#[doc(alias = "std::map<std::string,Ogre::Archive *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsPN4Ogre7ArchiveESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// IDA 0xc56120: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56120() {
}


// 0xc562dc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// IDA 0xc562dc: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc562dc() {
}


// 0xc563c0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// IDA 0xc563c0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc563c0() {
}


// 0xc56514 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Archive *>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0xc56514: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56514() {
}


// 0xc5658c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Archive *>>,std::pair<std::string const,Ogre::Archive *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// IDA 0xc5658c: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5658c() {
}


// 0xc5676c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Archive *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// IDA 0xc5676c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5676c() {
}


// 0xc568c0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Archive *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// IDA 0xc568c0: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc568c0() {
}


// 0xc569a4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// IDA 0xc569a4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc569a4() {
}


// 0xc56a48 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// IDA 0xc56a48: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56a48() {
}


// 0xc56aec — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// IDA 0xc56aec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc56aec() {
}


// 0xc56af0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// IDA 0xc56af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc56af0() {
}


// 0xc56afc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// IDA 0xc56afc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc56afc() {
}


// 0xc56b00 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// IDA 0xc56b00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc56b00() {
}


// 0xc56b0c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ArchiveFactory *>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0xc56b0c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56b0c() {
}


// 0xc56bb8 — __ZN4Ogre19AutoParamDataSourceC1Ev
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::AutoParamDataSource(void)")]
// was: __ZN4Ogre19AutoParamDataSourceC1Ev
// IDA 0xc56bb8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56bb8() {
}


// 0xc56bc4 — __ZN4Ogre19AutoParamDataSourceC2Ev
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::AutoParamDataSource(void)")]
// was: __ZN4Ogre19AutoParamDataSourceC2Ev
// IDA 0xc56bc4: 234 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56bc4() {
}


// 0xc56e7c — __ZN4Ogre19AutoParamDataSourceD0Ev
// type: void __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::~AutoParamDataSource()")]
// was: __ZN4Ogre19AutoParamDataSourceD0Ev
// IDA 0xc56e7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc56e7c() {
}


// 0xc56f20 — __ZN4Ogre19AutoParamDataSourceD1Ev
// type: void __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::~AutoParamDataSource()")]
// was: __ZN4Ogre19AutoParamDataSourceD1Ev
// IDA 0xc56f20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc56f20() {
}


// 0xc56f44 — __ZN4Ogre19AutoParamDataSource20setCurrentRenderableEPKNS_10RenderableE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentRenderable(Ogre::Renderable const*)")]
// was: __ZN4Ogre19AutoParamDataSource20setCurrentRenderableEPKNS_10RenderableE
// IDA 0xc56f44: 26 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56f44() {
}


// 0xc56f94 — __ZN4Ogre19AutoParamDataSource16setCurrentCameraEPKNS_6CameraEb
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, const Ogre::Camera *, bool)
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentCamera(Ogre::Camera const*,bool)")]
// was: __ZN4Ogre19AutoParamDataSource16setCurrentCameraEPKNS_6CameraEb
// IDA 0xc56f94: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56f94() {
}


// 0xc56ffc — __ZN4Ogre19AutoParamDataSource19setCurrentLightListEPKNS_12HashedVectorIPNS_5LightEEE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentLightList(Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre19AutoParamDataSource19setCurrentLightListEPKNS_12HashedVectorIPNS_5LightEEE
// IDA 0xc56ffc: 29 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc56ffc() {
}


// 0xc57048 — __ZNK4Ogre19AutoParamDataSource14getLightNumberEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightNumber(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getLightNumberEm
// IDA 0xc57048: 16 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57048() {
}


// 0xc57078 — __ZNK4Ogre19AutoParamDataSource21getLightDiffuseColourEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightDiffuseColour(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getLightDiffuseColourEm
// IDA 0xc57078: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57078() {
}


// 0xc570a8 — __ZNK4Ogre19AutoParamDataSource22getLightSpecularColourEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightSpecularColour(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource22getLightSpecularColourEm
// IDA 0xc570a8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc570a8() {
}


// 0xc570d8 — __ZNK4Ogre19AutoParamDataSource30getLightDiffuseColourWithPowerEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightDiffuseColourWithPower(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource30getLightDiffuseColourWithPowerEm
// IDA 0xc570d8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc570d8() {
}


// 0xc57140 — __ZNK4Ogre19AutoParamDataSource31getLightSpecularColourWithPowerEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightSpecularColourWithPower(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource31getLightSpecularColourWithPowerEm
// IDA 0xc57140: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57140() {
}


// 0xc571a8 — __ZNK4Ogre19AutoParamDataSource16getLightPositionEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightPosition(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource16getLightPositionEm
// IDA 0xc571a8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc571a8() {
}


// 0xc571d4 — __ZNK4Ogre19AutoParamDataSource18getLightAs4DVectorEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightAs4DVector(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getLightAs4DVectorEm
// IDA 0xc571d4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc571d4() {
}


// 0xc57200 — __ZNK4Ogre19AutoParamDataSource17getLightDirectionEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightDirection(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource17getLightDirectionEm
// IDA 0xc57200: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57200() {
}


// 0xc57230 — __ZNK4Ogre19AutoParamDataSource18getLightPowerScaleEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightPowerScale(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getLightPowerScaleEm
// IDA 0xc57230: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57230() {
}


// 0xc57260 — __ZNK4Ogre19AutoParamDataSource19getLightAttenuationEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightAttenuation(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getLightAttenuationEm
// IDA 0xc57260: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57260() {
}


// 0xc572b8 — __ZNK4Ogre19AutoParamDataSource18getSpotlightParamsEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightParams(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getSpotlightParamsEm
// IDA 0xc572b8: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc572b8() {
}


// 0xc57374 — __ZN4Ogre19AutoParamDataSource20setMainCamBoundsInfoEPNS_24VisibleObjectsBoundsInfoE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setMainCamBoundsInfo(Ogre::VisibleObjectsBoundsInfo *)")]
// was: __ZN4Ogre19AutoParamDataSource20setMainCamBoundsInfoEPNS_24VisibleObjectsBoundsInfoE
// IDA 0xc57374: 6 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57374() {
}


// 0xc57384 — __ZN4Ogre19AutoParamDataSource22setCurrentSceneManagerEPKNS_12SceneManagerE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentSceneManager(Ogre::SceneManager const*)")]
// was: __ZN4Ogre19AutoParamDataSource22setCurrentSceneManagerEPKNS_12SceneManagerE
// IDA 0xc57384: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57384() {
}


// 0xc5738c — __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)")]
// was: __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
// IDA 0xc5738c: 8 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5738c() {
}


// 0xc573a4 — __ZNK4Ogre19AutoParamDataSource14getWorldMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getWorldMatrixEv
// IDA 0xc573a4: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc573a4() {
}


// 0xc5745c — __ZNK4Ogre19AutoParamDataSource19getWorldMatrixCountEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrixCount(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getWorldMatrixCountEv
// IDA 0xc5745c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5745c() {
}


// 0xc57474 — __ZNK4Ogre19AutoParamDataSource19getWorldMatrixArrayEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrixArray(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getWorldMatrixArrayEv
// IDA 0xc57474: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57474() {
}


// 0xc5748c — __ZNK4Ogre19AutoParamDataSource13getViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource13getViewMatrixEv
// IDA 0xc5748c: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5748c() {
}


// 0xc57588 — __ZNK4Ogre19AutoParamDataSource23getViewProjectionMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getViewProjectionMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource23getViewProjectionMatrixEv
// IDA 0xc57588: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57588() {
}


// 0xc5760c — __ZNK4Ogre19AutoParamDataSource19getProjectionMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getProjectionMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getProjectionMatrixEv
// IDA 0xc5760c: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5760c() {
}


// 0xc57710 — __ZNK4Ogre19AutoParamDataSource18getWorldViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getWorldViewMatrixEv
// IDA 0xc57710: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57710() {
}


// 0xc57794 — __ZNK4Ogre19AutoParamDataSource22getWorldViewProjMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldViewProjMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource22getWorldViewProjMatrixEv
// IDA 0xc57794: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57794() {
}


// 0xc57818 — __ZNK4Ogre19AutoParamDataSource21getInverseWorldMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseWorldMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getInverseWorldMatrixEv
// IDA 0xc57818: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57818() {
}


// 0xc57880 — __ZNK4Ogre19AutoParamDataSource25getInverseWorldViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseWorldViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource25getInverseWorldViewMatrixEv
// IDA 0xc57880: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57880() {
}


// 0xc578e8 — __ZNK4Ogre19AutoParamDataSource20getInverseViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getInverseViewMatrixEv
// IDA 0xc578e8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc578e8() {
}


// 0xc57950 — __ZNK4Ogre19AutoParamDataSource30getInverseTransposeWorldMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTransposeWorldMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource30getInverseTransposeWorldMatrixEv
// IDA 0xc57950: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57950() {
}


// 0xc57a5c — __ZNK4Ogre19AutoParamDataSource34getInverseTransposeWorldViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTransposeWorldViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource34getInverseTransposeWorldViewMatrixEv
// IDA 0xc57a5c: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57a5c() {
}


// 0xc57b64 — __ZNK4Ogre19AutoParamDataSource17getCameraPositionEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCameraPosition(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource17getCameraPositionEv
// IDA 0xc57b64: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57b64() {
}


// 0xc57bf4 — __ZNK4Ogre19AutoParamDataSource28getCameraPositionObjectSpaceEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCameraPositionObjectSpace(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource28getCameraPositionObjectSpaceEv
// IDA 0xc57bf4: 121 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57bf4() {
}


// 0xc57da4 — __ZNK4Ogre19AutoParamDataSource20getLodCameraPositionEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getLodCameraPosition(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getLodCameraPositionEv
// IDA 0xc57da4: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57da4() {
}


// 0xc57e40 — __ZNK4Ogre19AutoParamDataSource31getLodCameraPositionObjectSpaceEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getLodCameraPositionObjectSpace(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource31getLodCameraPositionObjectSpaceEv
// IDA 0xc57e40: 133 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc57e40() {
}


// 0xc58014 — __ZN4Ogre19AutoParamDataSource21setAmbientLightColourERKNS_11ColourValueE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setAmbientLightColour(Ogre::ColourValue const&)")]
// was: __ZN4Ogre19AutoParamDataSource21setAmbientLightColourERKNS_11ColourValueE
// IDA 0xc58014: 5 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58014() {
}


// 0xc58024 — __ZNK4Ogre19AutoParamDataSource13getLightCountEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getLightCount(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource13getLightCountEv
// IDA 0xc58024: 9 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58024() {
}


// 0xc58040 — __ZNK4Ogre19AutoParamDataSource20getLightCastsShadowsEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightCastsShadows(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getLightCastsShadowsEm
// IDA 0xc58040: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58040() {
}


// 0xc58080 — __ZNK4Ogre19AutoParamDataSource21getAmbientLightColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getAmbientLightColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getAmbientLightColourEv
// IDA 0xc58080: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58080() {
}


// 0xc58088 — __ZN4Ogre19AutoParamDataSource14setCurrentPassEPKNS_4PassE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentPass(Ogre::Pass const*)")]
// was: __ZN4Ogre19AutoParamDataSource14setCurrentPassEPKNS_4PassE
// IDA 0xc58088: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58088() {
}


// 0xc58090 — __ZNK4Ogre19AutoParamDataSource14getCurrentPassEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCurrentPass(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getCurrentPassEv
// IDA 0xc58090: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58090() {
}


// 0xc58098 — __ZNK4Ogre19AutoParamDataSource14getTextureSizeEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureSize(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getTextureSizeEm
// IDA 0xc58098: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58098() {
}


// 0xc58128 — __ZNK4Ogre19AutoParamDataSource21getInverseTextureSizeEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTextureSize(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getInverseTextureSizeEm
// IDA 0xc58128: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58128() {
}


// 0xc58174 — __ZNK4Ogre19AutoParamDataSource20getPackedTextureSizeEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getPackedTextureSize(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getPackedTextureSizeEm
// IDA 0xc58174: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58174() {
}


// 0xc581b4 — __ZNK4Ogre19AutoParamDataSource23getSurfaceAmbientColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceAmbientColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource23getSurfaceAmbientColourEv
// IDA 0xc581b4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc581b4() {
}


// 0xc581c4 — __ZNK4Ogre19AutoParamDataSource23getSurfaceDiffuseColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceDiffuseColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource23getSurfaceDiffuseColourEv
// IDA 0xc581c4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc581c4() {
}


// 0xc581d4 — __ZNK4Ogre19AutoParamDataSource24getSurfaceSpecularColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceSpecularColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource24getSurfaceSpecularColourEv
// IDA 0xc581d4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc581d4() {
}


// 0xc581e4 — __ZNK4Ogre19AutoParamDataSource24getSurfaceEmissiveColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceEmissiveColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource24getSurfaceEmissiveColourEv
// IDA 0xc581e4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc581e4() {
}


// 0xc581f4 — __ZNK4Ogre19AutoParamDataSource19getSurfaceShininessEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceShininess(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getSurfaceShininessEv
// IDA 0xc581f4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc581f4() {
}


// 0xc58204 — __ZNK4Ogre19AutoParamDataSource28getDerivedAmbientLightColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getDerivedAmbientLightColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource28getDerivedAmbientLightColourEv
// IDA 0xc58204: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58204() {
}


// 0xc58268 — __ZNK4Ogre19AutoParamDataSource21getDerivedSceneColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getDerivedSceneColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getDerivedSceneColourEv
// IDA 0xc58268: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58268() {
}


// 0xc582cc — __ZN4Ogre19AutoParamDataSource6setFogENS_7FogModeERKNS_11ColourValueEfff
// type: int __fastcall(int, int, int, int, float, float)
#[doc(alias = "Ogre::AutoParamDataSource::setFog(Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")]
// was: __ZN4Ogre19AutoParamDataSource6setFogENS_7FogModeERKNS_11ColourValueEfff
// IDA 0xc582cc: 25 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc582cc() {
}


// 0xc58328 — __ZNK4Ogre19AutoParamDataSource12getFogColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getFogColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource12getFogColourEv
// IDA 0xc58328: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58328() {
}


// 0xc58330 — __ZNK4Ogre19AutoParamDataSource12getFogParamsEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getFogParams(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource12getFogParamsEv
// IDA 0xc58330: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58330() {
}


// 0xc58338 — __ZN4Ogre19AutoParamDataSource19setTextureProjectorEPKNS_7FrustumEm
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setTextureProjector(Ogre::Frustum const*,unsigned long)")]
// was: __ZN4Ogre19AutoParamDataSource19setTextureProjectorEPKNS_7FrustumEm
// IDA 0xc58338: 15 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58338() {
}


// 0xc5836c — __ZNK4Ogre19AutoParamDataSource24getTextureViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource24getTextureViewProjMatrixEm
// IDA 0xc5836c: 103 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5836c() {
}


// 0xc5849c — __ZNK4Ogre19AutoParamDataSource29getTextureWorldViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureWorldViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource29getTextureWorldViewProjMatrixEm
// IDA 0xc5849c: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5849c() {
}


// 0xc5855c — __ZNK4Ogre19AutoParamDataSource26getSpotlightViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource26getSpotlightViewProjMatrixEm
// IDA 0xc5855c: 349 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc5855c() {
}


// 0xc58a08 — __ZNK4Ogre19AutoParamDataSource31getSpotlightWorldViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightWorldViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource31getSpotlightWorldViewProjMatrixEm
// IDA 0xc58a08: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58a08() {
}


// 0xc58ae4 — __ZNK4Ogre19AutoParamDataSource25getTextureTransformMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureTransformMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource25getTextureTransformMatrixEm
// IDA 0xc58ae4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58ae4() {
}


// 0xc58b18 — __ZN4Ogre19AutoParamDataSource22setCurrentRenderTargetEPKNS_12RenderTargetE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentRenderTarget(Ogre::RenderTarget const*)")]
// was: __ZN4Ogre19AutoParamDataSource22setCurrentRenderTargetEPKNS_12RenderTargetE
// IDA 0xc58b18: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58b18() {
}


// 0xc58b20 — __ZNK4Ogre19AutoParamDataSource22getCurrentRenderTargetEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCurrentRenderTarget(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource22getCurrentRenderTargetEv
// IDA 0xc58b20: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58b20() {
}


// 0xc58b28 — __ZN4Ogre19AutoParamDataSource18setCurrentViewportEPKNS_8ViewportE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentViewport(Ogre::Viewport const*)")]
// was: __ZN4Ogre19AutoParamDataSource18setCurrentViewportEPKNS_8ViewportE
// IDA 0xc58b28: 3 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc58b28() {
}
