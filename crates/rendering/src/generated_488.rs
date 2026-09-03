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
pub fn stub_0xc554b4() -> ! {
    todo!("0xc554b4 Ogre::VertexAnimationTrack::hasNonZeroKeyFrames(void)const")
}


// 0xc55514 — __ZN4Ogre20VertexAnimationTrack8optimiseEv
// type: void __fastcall(Ogre::VertexAnimationTrack *this)
#[doc(alias = "Ogre::VertexAnimationTrack::optimise(void)")]
// was: __ZN4Ogre20VertexAnimationTrack8optimiseEv
pub fn stub_0xc55514() -> ! {
    todo!("0xc55514 Ogre::VertexAnimationTrack::optimise(void)")
}


// 0xc55518 — __ZN4Ogre20VertexAnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
// type: int __fastcall(int this, const Ogre::KeyFrame *)
#[doc(alias = "Ogre::VertexAnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")]
// was: __ZN4Ogre20VertexAnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
pub fn stub_0xc55518() -> ! {
    todo!("0xc55518 Ogre::VertexAnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")
}


// 0xc5553c — __ZNK4Ogre14AnimationTrack20_keyFrameDataChangedEv
// type: void __fastcall(Ogre::AnimationTrack *this)
#[doc(alias = "Ogre::AnimationTrack::_keyFrameDataChanged(void)const")]
// was: __ZNK4Ogre14AnimationTrack20_keyFrameDataChangedEv
pub fn stub_0xc5553c() -> ! {
    todo!("0xc5553c Ogre::AnimationTrack::_keyFrameDataChanged(void)const")
}


// 0xc55540 — __ZNK4Ogre14AnimationTrack19hasNonZeroKeyFramesEv
// type: int __fastcall(Ogre::AnimationTrack *this)
#[doc(alias = "Ogre::AnimationTrack::hasNonZeroKeyFrames(void)const")]
// was: __ZNK4Ogre14AnimationTrack19hasNonZeroKeyFramesEv
pub fn stub_0xc55540() -> ! {
    todo!("0xc55540 Ogre::AnimationTrack::hasNonZeroKeyFrames(void)const")
}


// 0xc55544 — __ZN4Ogre14AnimationTrack8optimiseEv
// type: void __fastcall(Ogre::AnimationTrack *this)
#[doc(alias = "Ogre::AnimationTrack::optimise(void)")]
// was: __ZN4Ogre14AnimationTrack8optimiseEv
pub fn stub_0xc55544() -> ! {
    todo!("0xc55544 Ogre::AnimationTrack::optimise(void)")
}


// 0xc55548 — __ZN4Ogre14AnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
// type: void()
#[doc(alias = "Ogre::AnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")]
// was: __ZN4Ogre14AnimationTrack18_applyBaseKeyFrameEPKNS_8KeyFrameE
pub fn stub_0xc55548() -> ! {
    todo!("0xc55548 Ogre::AnimationTrack::_applyBaseKeyFrame(Ogre::KeyFrame const*)")
}


// 0xc5554c — __ZN4Ogre14AnimationTrack11setListenerEPNS0_8ListenerE
// type: int __fastcall(int result, int)
#[doc(alias = "Ogre::AnimationTrack::setListener(Ogre::AnimationTrack::Listener *)")]
// was: __ZN4Ogre14AnimationTrack11setListenerEPNS0_8ListenerE
pub fn stub_0xc5554c() -> ! {
    todo!("0xc5554c Ogre::AnimationTrack::setListener(Ogre::AnimationTrack::Listener *)")
}


// 0xc55550 — __ZN4Ogre20VertexAnimationTrackD1Ev
// type: void __fastcall(Ogre::NedPoolingImpl **this)
#[doc(alias = "Ogre::VertexAnimationTrack::~VertexAnimationTrack()")]
// was: __ZN4Ogre20VertexAnimationTrackD1Ev
pub fn stub_0xc55550() -> ! {
    todo!("0xc55550 Ogre::VertexAnimationTrack::~VertexAnimationTrack()")
}


// 0xc5555c — __ZN4Ogre20VertexAnimationTrackD0Ev
// type: void __fastcall(Ogre::NedPoolingImpl **this)
#[doc(alias = "Ogre::VertexAnimationTrack::~VertexAnimationTrack()")]
// was: __ZN4Ogre20VertexAnimationTrackD0Ev
pub fn stub_0xc5555c() -> ! {
    todo!("0xc5555c Ogre::VertexAnimationTrack::~VertexAnimationTrack()")
}


// 0xc555ec — __ZNSt6vectorIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: _DWORD *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::KeyFrame **,std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::KeyFrame * const&)")]
// was: __ZNSt6vectorIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
pub fn stub_0xc555ec() -> ! {
    todo!("0xc555ec std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::KeyFrame **,std::vector<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::KeyFrame * const&)")
}


// 0xc556e4 — __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
// type: void()
#[doc(alias = "std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
pub fn stub_0xc556e4() -> ! {
    todo!("0xc556e4 std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}


// 0xc556e8 — __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
// type: void __fastcall(void *)
#[doc(alias = "std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: __ZNSt12_Vector_baseIPN4Ogre8KeyFrameENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
pub fn stub_0xc556e8() -> ! {
    todo!("0xc556e8 std::_Vector_base<Ogre::KeyFrame *,Ogre::STLAllocator<Ogre::KeyFrame *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")
}


// 0xc55728 — __ZN4Ogre14ArchiveManager12getSingletonEv
// type: int __fastcall(Ogre::ArchiveManager *this)
#[doc(alias = "Ogre::ArchiveManager::getSingleton(void)")]
// was: __ZN4Ogre14ArchiveManager12getSingletonEv
pub fn stub_0xc55728() -> ! {
    todo!("0xc55728 Ogre::ArchiveManager::getSingleton(void)")
}


// 0xc55738 — __ZN4Ogre14ArchiveManagerC1Ev
// type: int __fastcall(int this)
#[doc(alias = "Ogre::ArchiveManager::ArchiveManager(void)")]
// was: __ZN4Ogre14ArchiveManagerC1Ev
pub fn stub_0xc55738() -> ! {
    todo!("0xc55738 Ogre::ArchiveManager::ArchiveManager(void)")
}


// 0xc557a0 — __ZN4Ogre14ArchiveManager4loadERKSsS2_
// type: int __fastcall(Ogre::ArchiveManager *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::ArchiveManager::load(std::string const&,std::string const&)")]
// was: __ZN4Ogre14ArchiveManager4loadERKSsS2_
pub fn stub_0xc557a0() -> ! {
    todo!("0xc557a0 Ogre::ArchiveManager::load(std::string const&,std::string const&)")
}


// 0xc55a18 — __ZN4Ogre14ArchiveManagerD0Ev
// type: void __fastcall(Ogre::ArchiveManager *__hidden this)
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: __ZN4Ogre14ArchiveManagerD0Ev
pub fn stub_0xc55a18() -> ! {
    todo!("0xc55a18 Ogre::ArchiveManager::~ArchiveManager()")
}


// 0xc55aa8 — __ZN4Ogre14ArchiveManagerD1Ev
// type: void __fastcall(Ogre::ArchiveManager *__hidden this)
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: __ZN4Ogre14ArchiveManagerD1Ev
pub fn stub_0xc55aa8() -> ! {
    todo!("0xc55aa8 Ogre::ArchiveManager::~ArchiveManager()")
}


// 0xc55ab4 — __ZN4Ogre14ArchiveManagerD2Ev
// type: void __fastcall(Ogre::ArchiveManager *__hidden this)
#[doc(alias = "Ogre::ArchiveManager::~ArchiveManager()")]
// was: __ZN4Ogre14ArchiveManagerD2Ev
pub fn stub_0xc55ab4() -> ! {
    todo!("0xc55ab4 Ogre::ArchiveManager::~ArchiveManager()")
}


// 0xc55dfc — __ZN4Ogre14ArchiveManager17addArchiveFactoryEPNS_14ArchiveFactoryE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::ArchiveManager::addArchiveFactory(Ogre::ArchiveFactory *)")]
// was: __ZN4Ogre14ArchiveManager17addArchiveFactoryEPNS_14ArchiveFactoryE
pub fn stub_0xc55dfc() -> ! {
    todo!("0xc55dfc Ogre::ArchiveManager::addArchiveFactory(Ogre::ArchiveFactory *)")
}


// 0xc56120 — __ZNSt3mapISsPN4Ogre7ArchiveESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: 
#[doc(alias = "std::map<std::string,Ogre::Archive *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: __ZNSt3mapISsPN4Ogre7ArchiveESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
pub fn stub_0xc56120() -> ! {
    todo!("0xc56120 std::map<std::string,Ogre::Archive *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}


// 0xc562dc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
pub fn stub_0xc562dc() -> ! {
    todo!("0xc562dc std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::ArchiveFactory *> const&)")
}


// 0xc563c0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ArchiveFactory *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
pub fn stub_0xc563c0() -> ! {
    todo!("0xc563c0 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::ArchiveFactory *> const&)")
}


// 0xc56514 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Archive *>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0xc56514() -> ! {
    todo!("0xc56514 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Archive *>> *)")
}


// 0xc5658c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Archive *>>,std::pair<std::string const,Ogre::Archive *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0xc5658c() -> ! {
    todo!("0xc5658c std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Archive *>>,std::pair<std::string const,Ogre::Archive *> const&)")
}


// 0xc5676c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Archive *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
pub fn stub_0xc5676c() -> ! {
    todo!("0xc5676c std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Archive *> const&)")
}


// 0xc568c0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Archive *> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
pub fn stub_0xc568c0() -> ! {
    todo!("0xc568c0 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Archive *> const&)")
}


// 0xc569a4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
pub fn stub_0xc569a4() -> ! {
    todo!("0xc569a4 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}


// 0xc56a48 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
pub fn stub_0xc56a48() -> ! {
    todo!("0xc56a48 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}


// 0xc56aec — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
pub fn stub_0xc56aec() -> ! {
    todo!("0xc56aec std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}


// 0xc56af0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre7ArchiveEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
pub fn stub_0xc56af0() -> ! {
    todo!("0xc56af0 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Archive *>,std::_Select1st<std::pair<std::string const,Ogre::Archive *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Archive *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}


// 0xc56afc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
pub fn stub_0xc56afc() -> ! {
    todo!("0xc56afc std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}


// 0xc56b00 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
pub fn stub_0xc56b00() -> ! {
    todo!("0xc56b00 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")
}


// 0xc56b0c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: 
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ArchiveFactory *>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre14ArchiveFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0xc56b0c() -> ! {
    todo!("0xc56b0c std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ArchiveFactory *>,std::_Select1st<std::pair<std::string const,Ogre::ArchiveFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ArchiveFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ArchiveFactory *>> *)")
}


// 0xc56bb8 — __ZN4Ogre19AutoParamDataSourceC1Ev
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::AutoParamDataSource(void)")]
// was: __ZN4Ogre19AutoParamDataSourceC1Ev
pub fn stub_0xc56bb8() -> ! {
    todo!("0xc56bb8 Ogre::AutoParamDataSource::AutoParamDataSource(void)")
}


// 0xc56bc4 — __ZN4Ogre19AutoParamDataSourceC2Ev
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::AutoParamDataSource(void)")]
// was: __ZN4Ogre19AutoParamDataSourceC2Ev
pub fn stub_0xc56bc4() -> ! {
    todo!("0xc56bc4 Ogre::AutoParamDataSource::AutoParamDataSource(void)")
}


// 0xc56e7c — __ZN4Ogre19AutoParamDataSourceD0Ev
// type: void __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::~AutoParamDataSource()")]
// was: __ZN4Ogre19AutoParamDataSourceD0Ev
pub fn stub_0xc56e7c() -> ! {
    todo!("0xc56e7c Ogre::AutoParamDataSource::~AutoParamDataSource()")
}


// 0xc56f20 — __ZN4Ogre19AutoParamDataSourceD1Ev
// type: void __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::~AutoParamDataSource()")]
// was: __ZN4Ogre19AutoParamDataSourceD1Ev
pub fn stub_0xc56f20() -> ! {
    todo!("0xc56f20 Ogre::AutoParamDataSource::~AutoParamDataSource()")
}


// 0xc56f44 — __ZN4Ogre19AutoParamDataSource20setCurrentRenderableEPKNS_10RenderableE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentRenderable(Ogre::Renderable const*)")]
// was: __ZN4Ogre19AutoParamDataSource20setCurrentRenderableEPKNS_10RenderableE
pub fn stub_0xc56f44() -> ! {
    todo!("0xc56f44 Ogre::AutoParamDataSource::setCurrentRenderable(Ogre::Renderable const*)")
}


// 0xc56f94 — __ZN4Ogre19AutoParamDataSource16setCurrentCameraEPKNS_6CameraEb
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, const Ogre::Camera *, bool)
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentCamera(Ogre::Camera const*,bool)")]
// was: __ZN4Ogre19AutoParamDataSource16setCurrentCameraEPKNS_6CameraEb
pub fn stub_0xc56f94() -> ! {
    todo!("0xc56f94 Ogre::AutoParamDataSource::setCurrentCamera(Ogre::Camera const*,bool)")
}


// 0xc56ffc — __ZN4Ogre19AutoParamDataSource19setCurrentLightListEPKNS_12HashedVectorIPNS_5LightEEE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentLightList(Ogre::HashedVector<Ogre::Light *> const*)")]
// was: __ZN4Ogre19AutoParamDataSource19setCurrentLightListEPKNS_12HashedVectorIPNS_5LightEEE
pub fn stub_0xc56ffc() -> ! {
    todo!("0xc56ffc Ogre::AutoParamDataSource::setCurrentLightList(Ogre::HashedVector<Ogre::Light *> const*)")
}


// 0xc57048 — __ZNK4Ogre19AutoParamDataSource14getLightNumberEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightNumber(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getLightNumberEm
pub fn stub_0xc57048() -> ! {
    todo!("0xc57048 Ogre::AutoParamDataSource::getLightNumber(unsigned long)const")
}


// 0xc57078 — __ZNK4Ogre19AutoParamDataSource21getLightDiffuseColourEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightDiffuseColour(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getLightDiffuseColourEm
pub fn stub_0xc57078() -> ! {
    todo!("0xc57078 Ogre::AutoParamDataSource::getLightDiffuseColour(unsigned long)const")
}


// 0xc570a8 — __ZNK4Ogre19AutoParamDataSource22getLightSpecularColourEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightSpecularColour(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource22getLightSpecularColourEm
pub fn stub_0xc570a8() -> ! {
    todo!("0xc570a8 Ogre::AutoParamDataSource::getLightSpecularColour(unsigned long)const")
}


// 0xc570d8 — __ZNK4Ogre19AutoParamDataSource30getLightDiffuseColourWithPowerEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightDiffuseColourWithPower(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource30getLightDiffuseColourWithPowerEm
pub fn stub_0xc570d8() -> ! {
    todo!("0xc570d8 Ogre::AutoParamDataSource::getLightDiffuseColourWithPower(unsigned long)const")
}


// 0xc57140 — __ZNK4Ogre19AutoParamDataSource31getLightSpecularColourWithPowerEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightSpecularColourWithPower(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource31getLightSpecularColourWithPowerEm
pub fn stub_0xc57140() -> ! {
    todo!("0xc57140 Ogre::AutoParamDataSource::getLightSpecularColourWithPower(unsigned long)const")
}


// 0xc571a8 — __ZNK4Ogre19AutoParamDataSource16getLightPositionEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightPosition(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource16getLightPositionEm
pub fn stub_0xc571a8() -> ! {
    todo!("0xc571a8 Ogre::AutoParamDataSource::getLightPosition(unsigned long)const")
}


// 0xc571d4 — __ZNK4Ogre19AutoParamDataSource18getLightAs4DVectorEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightAs4DVector(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getLightAs4DVectorEm
pub fn stub_0xc571d4() -> ! {
    todo!("0xc571d4 Ogre::AutoParamDataSource::getLightAs4DVector(unsigned long)const")
}


// 0xc57200 — __ZNK4Ogre19AutoParamDataSource17getLightDirectionEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightDirection(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource17getLightDirectionEm
pub fn stub_0xc57200() -> ! {
    todo!("0xc57200 Ogre::AutoParamDataSource::getLightDirection(unsigned long)const")
}


// 0xc57230 — __ZNK4Ogre19AutoParamDataSource18getLightPowerScaleEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightPowerScale(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getLightPowerScaleEm
pub fn stub_0xc57230() -> ! {
    todo!("0xc57230 Ogre::AutoParamDataSource::getLightPowerScale(unsigned long)const")
}


// 0xc57260 — __ZNK4Ogre19AutoParamDataSource19getLightAttenuationEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightAttenuation(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getLightAttenuationEm
pub fn stub_0xc57260() -> ! {
    todo!("0xc57260 Ogre::AutoParamDataSource::getLightAttenuation(unsigned long)const")
}


// 0xc572b8 — __ZNK4Ogre19AutoParamDataSource18getSpotlightParamsEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightParams(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getSpotlightParamsEm
pub fn stub_0xc572b8() -> ! {
    todo!("0xc572b8 Ogre::AutoParamDataSource::getSpotlightParams(unsigned long)const")
}


// 0xc57374 — __ZN4Ogre19AutoParamDataSource20setMainCamBoundsInfoEPNS_24VisibleObjectsBoundsInfoE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setMainCamBoundsInfo(Ogre::VisibleObjectsBoundsInfo *)")]
// was: __ZN4Ogre19AutoParamDataSource20setMainCamBoundsInfoEPNS_24VisibleObjectsBoundsInfoE
pub fn stub_0xc57374() -> ! {
    todo!("0xc57374 Ogre::AutoParamDataSource::setMainCamBoundsInfo(Ogre::VisibleObjectsBoundsInfo *)")
}


// 0xc57384 — __ZN4Ogre19AutoParamDataSource22setCurrentSceneManagerEPKNS_12SceneManagerE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentSceneManager(Ogre::SceneManager const*)")]
// was: __ZN4Ogre19AutoParamDataSource22setCurrentSceneManagerEPKNS_12SceneManagerE
pub fn stub_0xc57384() -> ! {
    todo!("0xc57384 Ogre::AutoParamDataSource::setCurrentSceneManager(Ogre::SceneManager const*)")
}


// 0xc5738c — __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)")]
// was: __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
pub fn stub_0xc5738c() -> ! {
    todo!("0xc5738c Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)")
}


// 0xc573a4 — __ZNK4Ogre19AutoParamDataSource14getWorldMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getWorldMatrixEv
pub fn stub_0xc573a4() -> ! {
    todo!("0xc573a4 Ogre::AutoParamDataSource::getWorldMatrix(void)const")
}


// 0xc5745c — __ZNK4Ogre19AutoParamDataSource19getWorldMatrixCountEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrixCount(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getWorldMatrixCountEv
pub fn stub_0xc5745c() -> ! {
    todo!("0xc5745c Ogre::AutoParamDataSource::getWorldMatrixCount(void)const")
}


// 0xc57474 — __ZNK4Ogre19AutoParamDataSource19getWorldMatrixArrayEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldMatrixArray(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getWorldMatrixArrayEv
pub fn stub_0xc57474() -> ! {
    todo!("0xc57474 Ogre::AutoParamDataSource::getWorldMatrixArray(void)const")
}


// 0xc5748c — __ZNK4Ogre19AutoParamDataSource13getViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource13getViewMatrixEv
pub fn stub_0xc5748c() -> ! {
    todo!("0xc5748c Ogre::AutoParamDataSource::getViewMatrix(void)const")
}


// 0xc57588 — __ZNK4Ogre19AutoParamDataSource23getViewProjectionMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getViewProjectionMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource23getViewProjectionMatrixEv
pub fn stub_0xc57588() -> ! {
    todo!("0xc57588 Ogre::AutoParamDataSource::getViewProjectionMatrix(void)const")
}


// 0xc5760c — __ZNK4Ogre19AutoParamDataSource19getProjectionMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getProjectionMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getProjectionMatrixEv
pub fn stub_0xc5760c() -> ! {
    todo!("0xc5760c Ogre::AutoParamDataSource::getProjectionMatrix(void)const")
}


// 0xc57710 — __ZNK4Ogre19AutoParamDataSource18getWorldViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource18getWorldViewMatrixEv
pub fn stub_0xc57710() -> ! {
    todo!("0xc57710 Ogre::AutoParamDataSource::getWorldViewMatrix(void)const")
}


// 0xc57794 — __ZNK4Ogre19AutoParamDataSource22getWorldViewProjMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getWorldViewProjMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource22getWorldViewProjMatrixEv
pub fn stub_0xc57794() -> ! {
    todo!("0xc57794 Ogre::AutoParamDataSource::getWorldViewProjMatrix(void)const")
}


// 0xc57818 — __ZNK4Ogre19AutoParamDataSource21getInverseWorldMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseWorldMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getInverseWorldMatrixEv
pub fn stub_0xc57818() -> ! {
    todo!("0xc57818 Ogre::AutoParamDataSource::getInverseWorldMatrix(void)const")
}


// 0xc57880 — __ZNK4Ogre19AutoParamDataSource25getInverseWorldViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseWorldViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource25getInverseWorldViewMatrixEv
pub fn stub_0xc57880() -> ! {
    todo!("0xc57880 Ogre::AutoParamDataSource::getInverseWorldViewMatrix(void)const")
}


// 0xc578e8 — __ZNK4Ogre19AutoParamDataSource20getInverseViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getInverseViewMatrixEv
pub fn stub_0xc578e8() -> ! {
    todo!("0xc578e8 Ogre::AutoParamDataSource::getInverseViewMatrix(void)const")
}


// 0xc57950 — __ZNK4Ogre19AutoParamDataSource30getInverseTransposeWorldMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTransposeWorldMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource30getInverseTransposeWorldMatrixEv
pub fn stub_0xc57950() -> ! {
    todo!("0xc57950 Ogre::AutoParamDataSource::getInverseTransposeWorldMatrix(void)const")
}


// 0xc57a5c — __ZNK4Ogre19AutoParamDataSource34getInverseTransposeWorldViewMatrixEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTransposeWorldViewMatrix(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource34getInverseTransposeWorldViewMatrixEv
pub fn stub_0xc57a5c() -> ! {
    todo!("0xc57a5c Ogre::AutoParamDataSource::getInverseTransposeWorldViewMatrix(void)const")
}


// 0xc57b64 — __ZNK4Ogre19AutoParamDataSource17getCameraPositionEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCameraPosition(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource17getCameraPositionEv
pub fn stub_0xc57b64() -> ! {
    todo!("0xc57b64 Ogre::AutoParamDataSource::getCameraPosition(void)const")
}


// 0xc57bf4 — __ZNK4Ogre19AutoParamDataSource28getCameraPositionObjectSpaceEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCameraPositionObjectSpace(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource28getCameraPositionObjectSpaceEv
pub fn stub_0xc57bf4() -> ! {
    todo!("0xc57bf4 Ogre::AutoParamDataSource::getCameraPositionObjectSpace(void)const")
}


// 0xc57da4 — __ZNK4Ogre19AutoParamDataSource20getLodCameraPositionEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getLodCameraPosition(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getLodCameraPositionEv
pub fn stub_0xc57da4() -> ! {
    todo!("0xc57da4 Ogre::AutoParamDataSource::getLodCameraPosition(void)const")
}


// 0xc57e40 — __ZNK4Ogre19AutoParamDataSource31getLodCameraPositionObjectSpaceEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getLodCameraPositionObjectSpace(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource31getLodCameraPositionObjectSpaceEv
pub fn stub_0xc57e40() -> ! {
    todo!("0xc57e40 Ogre::AutoParamDataSource::getLodCameraPositionObjectSpace(void)const")
}


// 0xc58014 — __ZN4Ogre19AutoParamDataSource21setAmbientLightColourERKNS_11ColourValueE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setAmbientLightColour(Ogre::ColourValue const&)")]
// was: __ZN4Ogre19AutoParamDataSource21setAmbientLightColourERKNS_11ColourValueE
pub fn stub_0xc58014() -> ! {
    todo!("0xc58014 Ogre::AutoParamDataSource::setAmbientLightColour(Ogre::ColourValue const&)")
}


// 0xc58024 — __ZNK4Ogre19AutoParamDataSource13getLightCountEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getLightCount(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource13getLightCountEv
pub fn stub_0xc58024() -> ! {
    todo!("0xc58024 Ogre::AutoParamDataSource::getLightCount(void)const")
}


// 0xc58040 — __ZNK4Ogre19AutoParamDataSource20getLightCastsShadowsEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getLightCastsShadows(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getLightCastsShadowsEm
pub fn stub_0xc58040() -> ! {
    todo!("0xc58040 Ogre::AutoParamDataSource::getLightCastsShadows(unsigned long)const")
}


// 0xc58080 — __ZNK4Ogre19AutoParamDataSource21getAmbientLightColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getAmbientLightColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getAmbientLightColourEv
pub fn stub_0xc58080() -> ! {
    todo!("0xc58080 Ogre::AutoParamDataSource::getAmbientLightColour(void)const")
}


// 0xc58088 — __ZN4Ogre19AutoParamDataSource14setCurrentPassEPKNS_4PassE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentPass(Ogre::Pass const*)")]
// was: __ZN4Ogre19AutoParamDataSource14setCurrentPassEPKNS_4PassE
pub fn stub_0xc58088() -> ! {
    todo!("0xc58088 Ogre::AutoParamDataSource::setCurrentPass(Ogre::Pass const*)")
}


// 0xc58090 — __ZNK4Ogre19AutoParamDataSource14getCurrentPassEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCurrentPass(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getCurrentPassEv
pub fn stub_0xc58090() -> ! {
    todo!("0xc58090 Ogre::AutoParamDataSource::getCurrentPass(void)const")
}


// 0xc58098 — __ZNK4Ogre19AutoParamDataSource14getTextureSizeEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureSize(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource14getTextureSizeEm
pub fn stub_0xc58098() -> ! {
    todo!("0xc58098 Ogre::AutoParamDataSource::getTextureSize(unsigned long)const")
}


// 0xc58128 — __ZNK4Ogre19AutoParamDataSource21getInverseTextureSizeEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getInverseTextureSize(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getInverseTextureSizeEm
pub fn stub_0xc58128() -> ! {
    todo!("0xc58128 Ogre::AutoParamDataSource::getInverseTextureSize(unsigned long)const")
}


// 0xc58174 — __ZNK4Ogre19AutoParamDataSource20getPackedTextureSizeEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getPackedTextureSize(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource20getPackedTextureSizeEm
pub fn stub_0xc58174() -> ! {
    todo!("0xc58174 Ogre::AutoParamDataSource::getPackedTextureSize(unsigned long)const")
}


// 0xc581b4 — __ZNK4Ogre19AutoParamDataSource23getSurfaceAmbientColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceAmbientColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource23getSurfaceAmbientColourEv
pub fn stub_0xc581b4() -> ! {
    todo!("0xc581b4 Ogre::AutoParamDataSource::getSurfaceAmbientColour(void)const")
}


// 0xc581c4 — __ZNK4Ogre19AutoParamDataSource23getSurfaceDiffuseColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceDiffuseColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource23getSurfaceDiffuseColourEv
pub fn stub_0xc581c4() -> ! {
    todo!("0xc581c4 Ogre::AutoParamDataSource::getSurfaceDiffuseColour(void)const")
}


// 0xc581d4 — __ZNK4Ogre19AutoParamDataSource24getSurfaceSpecularColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceSpecularColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource24getSurfaceSpecularColourEv
pub fn stub_0xc581d4() -> ! {
    todo!("0xc581d4 Ogre::AutoParamDataSource::getSurfaceSpecularColour(void)const")
}


// 0xc581e4 — __ZNK4Ogre19AutoParamDataSource24getSurfaceEmissiveColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceEmissiveColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource24getSurfaceEmissiveColourEv
pub fn stub_0xc581e4() -> ! {
    todo!("0xc581e4 Ogre::AutoParamDataSource::getSurfaceEmissiveColour(void)const")
}


// 0xc581f4 — __ZNK4Ogre19AutoParamDataSource19getSurfaceShininessEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getSurfaceShininess(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource19getSurfaceShininessEv
pub fn stub_0xc581f4() -> ! {
    todo!("0xc581f4 Ogre::AutoParamDataSource::getSurfaceShininess(void)const")
}


// 0xc58204 — __ZNK4Ogre19AutoParamDataSource28getDerivedAmbientLightColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getDerivedAmbientLightColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource28getDerivedAmbientLightColourEv
pub fn stub_0xc58204() -> ! {
    todo!("0xc58204 Ogre::AutoParamDataSource::getDerivedAmbientLightColour(void)const")
}


// 0xc58268 — __ZNK4Ogre19AutoParamDataSource21getDerivedSceneColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getDerivedSceneColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource21getDerivedSceneColourEv
pub fn stub_0xc58268() -> ! {
    todo!("0xc58268 Ogre::AutoParamDataSource::getDerivedSceneColour(void)const")
}


// 0xc582cc — __ZN4Ogre19AutoParamDataSource6setFogENS_7FogModeERKNS_11ColourValueEfff
// type: int __fastcall(int, int, int, int, float, float)
#[doc(alias = "Ogre::AutoParamDataSource::setFog(Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")]
// was: __ZN4Ogre19AutoParamDataSource6setFogENS_7FogModeERKNS_11ColourValueEfff
pub fn stub_0xc582cc() -> ! {
    todo!("0xc582cc Ogre::AutoParamDataSource::setFog(Ogre::FogMode,Ogre::ColourValue const&,float,float,float)")
}


// 0xc58328 — __ZNK4Ogre19AutoParamDataSource12getFogColourEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getFogColour(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource12getFogColourEv
pub fn stub_0xc58328() -> ! {
    todo!("0xc58328 Ogre::AutoParamDataSource::getFogColour(void)const")
}


// 0xc58330 — __ZNK4Ogre19AutoParamDataSource12getFogParamsEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getFogParams(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource12getFogParamsEv
pub fn stub_0xc58330() -> ! {
    todo!("0xc58330 Ogre::AutoParamDataSource::getFogParams(void)const")
}


// 0xc58338 — __ZN4Ogre19AutoParamDataSource19setTextureProjectorEPKNS_7FrustumEm
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setTextureProjector(Ogre::Frustum const*,unsigned long)")]
// was: __ZN4Ogre19AutoParamDataSource19setTextureProjectorEPKNS_7FrustumEm
pub fn stub_0xc58338() -> ! {
    todo!("0xc58338 Ogre::AutoParamDataSource::setTextureProjector(Ogre::Frustum const*,unsigned long)")
}


// 0xc5836c — __ZNK4Ogre19AutoParamDataSource24getTextureViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource24getTextureViewProjMatrixEm
pub fn stub_0xc5836c() -> ! {
    todo!("0xc5836c Ogre::AutoParamDataSource::getTextureViewProjMatrix(unsigned long)const")
}


// 0xc5849c — __ZNK4Ogre19AutoParamDataSource29getTextureWorldViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureWorldViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource29getTextureWorldViewProjMatrixEm
pub fn stub_0xc5849c() -> ! {
    todo!("0xc5849c Ogre::AutoParamDataSource::getTextureWorldViewProjMatrix(unsigned long)const")
}


// 0xc5855c — __ZNK4Ogre19AutoParamDataSource26getSpotlightViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource26getSpotlightViewProjMatrixEm
pub fn stub_0xc5855c() -> ! {
    todo!("0xc5855c Ogre::AutoParamDataSource::getSpotlightViewProjMatrix(unsigned long)const")
}


// 0xc58a08 — __ZNK4Ogre19AutoParamDataSource31getSpotlightWorldViewProjMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getSpotlightWorldViewProjMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource31getSpotlightWorldViewProjMatrixEm
pub fn stub_0xc58a08() -> ! {
    todo!("0xc58a08 Ogre::AutoParamDataSource::getSpotlightWorldViewProjMatrix(unsigned long)const")
}


// 0xc58ae4 — __ZNK4Ogre19AutoParamDataSource25getTextureTransformMatrixEm
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this, unsigned int)
#[doc(alias = "Ogre::AutoParamDataSource::getTextureTransformMatrix(unsigned long)const")]
// was: __ZNK4Ogre19AutoParamDataSource25getTextureTransformMatrixEm
pub fn stub_0xc58ae4() -> ! {
    todo!("0xc58ae4 Ogre::AutoParamDataSource::getTextureTransformMatrix(unsigned long)const")
}


// 0xc58b18 — __ZN4Ogre19AutoParamDataSource22setCurrentRenderTargetEPKNS_12RenderTargetE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentRenderTarget(Ogre::RenderTarget const*)")]
// was: __ZN4Ogre19AutoParamDataSource22setCurrentRenderTargetEPKNS_12RenderTargetE
pub fn stub_0xc58b18() -> ! {
    todo!("0xc58b18 Ogre::AutoParamDataSource::setCurrentRenderTarget(Ogre::RenderTarget const*)")
}


// 0xc58b20 — __ZNK4Ogre19AutoParamDataSource22getCurrentRenderTargetEv
// type: _DWORD __fastcall(Ogre::AutoParamDataSource *__hidden this)
#[doc(alias = "Ogre::AutoParamDataSource::getCurrentRenderTarget(void)const")]
// was: __ZNK4Ogre19AutoParamDataSource22getCurrentRenderTargetEv
pub fn stub_0xc58b20() -> ! {
    todo!("0xc58b20 Ogre::AutoParamDataSource::getCurrentRenderTarget(void)const")
}


// 0xc58b28 — __ZN4Ogre19AutoParamDataSource18setCurrentViewportEPKNS_8ViewportE
// type: 
#[doc(alias = "Ogre::AutoParamDataSource::setCurrentViewport(Ogre::Viewport const*)")]
// was: __ZN4Ogre19AutoParamDataSource18setCurrentViewportEPKNS_8ViewportE
pub fn stub_0xc58b28() -> ! {
    todo!("0xc58b28 Ogre::AutoParamDataSource::setCurrentViewport(Ogre::Viewport const*)")
}
