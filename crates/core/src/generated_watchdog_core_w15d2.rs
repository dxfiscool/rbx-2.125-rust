//! core — watchdog core w15d2
//! Generated from ida/export.json — 120 stubs, core namespace (RBX/DataModel/Instance + SharedPtr/Signals/TaskScheduler fallback).
//! Source: ida/export.json (85545 funcs), filtered RBX/DataModel/Instance OR core (SharedPtr/signals/core) EA-ascending, excludes committed global EAs.
//! Range 0xd627c0..0xf679e4. Uses rbx_core::SharedPtr (not boost::shared_ptr).
#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd627c0 — __ZNSt6vectorIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "__ZNSt6vectorIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
#[doc(alias = "std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::DepthBuffer **,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::DepthBuffer * const&)")]
pub fn stub_0xd627c0() {
    // IDA 0xd627c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd628b8 — __ZNSt12_Vector_baseIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias = "std::_Vector_base<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
pub fn stub_0xd628b8() {
    // IDA 0xd628b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd628bc — __ZNSt6vectorIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(alias = "__ZNSt6vectorIPN4Ogre11DepthBufferENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_")]
#[doc(alias = "std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_0xd628bc() {
    // IDA 0xd628bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62930 — __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSC_
// type: _Rb_tree_node_base *__fastcall(int, int, int, _WORD *, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKSC_")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xd62930() {
    // IDA 0xd62930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62a68 — __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE16_M_insert_uniqueERKSC_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE16_M_insert_uniqueERKSC_")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
pub fn stub_0xd62a68() {
    // IDA 0xd62a68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62ad4 — __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
pub fn stub_0xd62ad4() {
    // IDA 0xd62ad4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd62b90 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::RenderTarget *> const&)")]
pub fn stub_0xd62b90() {
    // IDA 0xd62b90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd62c74 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::RenderTarget *> const&)")]
pub fn stub_0xd62c74() {
    // IDA 0xd62c74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd62dc8 — __ZNSt10_List_baseIPN4Ogre22HardwareOcclusionQueryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre22HardwareOcclusionQueryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
#[doc(alias = "std::_List_base<Ogre::HardwareOcclusionQuery *,Ogre::STLAllocator<Ogre::HardwareOcclusionQuery *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd62dc8() {
    // IDA 0xd62dc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62dcc — __ZNSt10_List_baseIPN4Ogre22HardwareOcclusionQueryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre22HardwareOcclusionQueryENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
#[doc(alias = "std::_List_base<Ogre::HardwareOcclusionQuery *,Ogre::STLAllocator<Ogre::HardwareOcclusionQuery *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd62dcc() {
    // IDA 0xd62dcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62dd8 — __ZNSt10_List_baseIPN4Ogre12RenderSystem8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre12RenderSystem8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev")]
#[doc(alias = "std::_List_base<Ogre::RenderSystem::Listener *,Ogre::STLAllocator<Ogre::RenderSystem::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd62dd8() {
    // IDA 0xd62dd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62ddc — __ZNSt10_List_baseIPN4Ogre12RenderSystem8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "__ZNSt10_List_baseIPN4Ogre12RenderSystem8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev")]
#[doc(alias = "std::_List_base<Ogre::RenderSystem::Listener *,Ogre::STLAllocator<Ogre::RenderSystem::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
pub fn stub_0xd62ddc() {
    // IDA 0xd62ddc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62de8 — __ZNSt8_Rb_treeIhSt4pairIKhPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessIhENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "__ZNSt8_Rb_treeIhSt4pairIKhPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessIhENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<unsigned char,std::pair<unsigned char const,Ogre::RenderTarget *>,std::_Select1st<std::pair<unsigned char const,Ogre::RenderTarget *>>,std::less<unsigned char>,Ogre::STLAllocator<std::pair<unsigned char const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned char>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd62de8() {
    // IDA 0xd62de8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62dec — __ZNSt8_Rb_treeIhSt4pairIKhPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessIhENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "__ZNSt8_Rb_treeIhSt4pairIKhPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessIhENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<unsigned char,std::pair<unsigned char const,Ogre::RenderTarget *>,std::_Select1st<std::pair<unsigned char const,Ogre::RenderTarget *>>,std::less<unsigned char>,Ogre::STLAllocator<std::pair<unsigned char const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned char>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd62dec() {
    // IDA 0xd62dec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62df8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd62df8() {
    // IDA 0xd62df8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62dfc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre12RenderTargetEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderTarget *>,std::_Select1st<std::pair<std::string const,Ogre::RenderTarget *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderTarget *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd62dfc() {
    // IDA 0xd62dfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62e08 — __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED1Ev
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd62e08() {
    // IDA 0xd62e08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62e0c — __ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED0Ev
#[doc(alias = "__ZNSt8_Rb_treeItSt4pairIKtSt6vectorIPN4Ogre11DepthBufferENS3_12STLAllocatorIS5_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISC_ESt4lessItENS6_ISC_S9_EEE13_Rb_tree_implISG_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::vector<Ogre::DepthBuffer *,Ogre::STLAllocator<Ogre::DepthBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd62e0c() {
    // IDA 0xd62e0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62e7c — __ZN4Ogre24RenderSystemCapabilitiesC1Ev
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilitiesC1Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilities::RenderSystemCapabilities(void)")]
pub fn stub_0xd62e7c() {
    // IDA 0xd62e7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd62ef0 — __ZN4Ogre24RenderSystemCapabilitiesD0Ev
// type: void __fastcall(Ogre::RenderSystemCapabilities *__hidden this)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilitiesD0Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilities::~RenderSystemCapabilities()")]
pub fn stub_0xd62ef0() {
    // IDA 0xd62ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd63014 — __ZN4Ogre24RenderSystemCapabilitiesD1Ev
// type: void __fastcall(Ogre::RenderSystemCapabilities *__hidden this)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilitiesD1Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilities::~RenderSystemCapabilities()")]
pub fn stub_0xd63014() {
    // IDA 0xd63014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd63130 — __ZN4Ogre24RenderSystemCapabilities3logEPNS_3LogE
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, Ogre::Log *)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities3logEPNS_3LogE")]
#[doc(alias = "Ogre::RenderSystemCapabilities::log(Ogre::Log *)")]
pub fn stub_0xd63130() {
    // IDA 0xd63130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a1c8 — __ZN4Ogre24RenderSystemCapabilities14vendorToStringENS_9GPUVendorE
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities14vendorToStringENS_9GPUVendorE")]
#[doc(alias = "Ogre::RenderSystemCapabilities::vendorToString(Ogre::GPUVendor)")]
pub fn stub_0xd6a1c8() {
    // IDA 0xd6a1c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a1ec — __ZN4Ogre24RenderSystemCapabilities16vendorFromStringERKSs
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, const std::string *)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities16vendorFromStringERKSs")]
#[doc(alias = "Ogre::RenderSystemCapabilities::vendorFromString(std::string const&)")]
pub fn stub_0xd6a1ec() {
    // IDA 0xd6a1ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a360 — __ZN4Ogre24RenderSystemCapabilities17initVendorStringsEv
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities17initVendorStringsEv")]
#[doc(alias = "Ogre::RenderSystemCapabilities::initVendorStrings(void)")]
pub fn stub_0xd6a360() {
    // IDA 0xd6a360: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd6a578 — __ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev
#[doc(alias = "__ZNSt6vectorISsN4Ogre12STLAllocatorISsNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEED1Ev")]
#[doc(alias = "std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~vector()")]
pub fn stub_0xd6a578() {
    // IDA 0xd6a578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a664 — __ZNK4Ogre24RenderSystemCapabilities13calculateSizeEv
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this)
#[doc(alias = "__ZNK4Ogre24RenderSystemCapabilities13calculateSizeEv")]
#[doc(alias = "Ogre::RenderSystemCapabilities::calculateSize(void)const")]
pub fn stub_0xd6a664() {
    // IDA 0xd6a664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a6d4 — __ZN4Ogre31RenderSystemCapabilitiesManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManager12getSingletonEv")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::getSingleton(void)")]
pub fn stub_0xd6a6d4() {
    // IDA 0xd6a6d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a6e4 — __ZN4Ogre31RenderSystemCapabilitiesManagerC1Ev
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManagerC1Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::RenderSystemCapabilitiesManager(void)")]
pub fn stub_0xd6a6e4() {
    // IDA 0xd6a6e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a6f0 — __ZN4Ogre31RenderSystemCapabilitiesManagerC2Ev
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManagerC2Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::RenderSystemCapabilitiesManager(void)")]
pub fn stub_0xd6a6f0() {
    // IDA 0xd6a6f0: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd6a89c — __ZN4Ogre31RenderSystemCapabilitiesManagerD0Ev
// type: void __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManagerD0Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::~RenderSystemCapabilitiesManager()")]
pub fn stub_0xd6a89c() {
    // IDA 0xd6a89c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a92c — __ZN4Ogre31RenderSystemCapabilitiesManagerD1Ev
// type: void __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManagerD1Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::~RenderSystemCapabilitiesManager()")]
pub fn stub_0xd6a92c() {
    // IDA 0xd6a92c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6a938 — __ZN4Ogre31RenderSystemCapabilitiesManagerD2Ev
// type: void __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManagerD2Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::~RenderSystemCapabilitiesManager()")]
pub fn stub_0xd6a938() {
    // IDA 0xd6a938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6aad4 — __ZN4Ogre31RenderSystemCapabilitiesManager28parseCapabilitiesFromArchiveERKSsS2_b
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this, const std::string *, const std::string *, bool)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManager28parseCapabilitiesFromArchiveERKSsS2_b")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::parseCapabilitiesFromArchive(std::string const&,std::string const&,bool)")]
pub fn stub_0xd6aad4() {
    // IDA 0xd6aad4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6acfc — __ZN4Ogre31RenderSystemCapabilitiesManager22loadParsedCapabilitiesERKSs
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this, const std::string *)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManager22loadParsedCapabilitiesERKSs")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::loadParsedCapabilities(std::string const&)")]
pub fn stub_0xd6acfc() {
    // IDA 0xd6acfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6ad0c — __ZN4Ogre31RenderSystemCapabilitiesManager28_addRenderSystemCapabilitiesERKSsPNS_24RenderSystemCapabilitiesE
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesManager *__hidden this, const std::string *, Ogre::RenderSystemCapabilities *)
#[doc(alias = "__ZN4Ogre31RenderSystemCapabilitiesManager28_addRenderSystemCapabilitiesERKSsPNS_24RenderSystemCapabilitiesE")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesManager::_addRenderSystemCapabilities(std::string const&,Ogre::RenderSystemCapabilities *)")]
pub fn stub_0xd6ad0c() {
    // IDA 0xd6ad0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6ae2c — __ZNSt3mapISsPN4Ogre24RenderSystemCapabilitiesESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "__ZNSt3mapISsPN4Ogre24RenderSystemCapabilitiesESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_")]
#[doc(alias = "std::map<std::string,Ogre::RenderSystemCapabilities *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xd6ae2c() {
    // IDA 0xd6ae2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd6afe8 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilities *>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilities *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::RenderSystemCapabilities *> const&)")]
pub fn stub_0xd6afe8() {
    // IDA 0xd6afe8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd6b0cc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilities *>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilities *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::RenderSystemCapabilities *> const&)")]
pub fn stub_0xd6b0cc() {
    // IDA 0xd6b0cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd6b220 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilities *>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilities *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>>,std::pair<std::string const,Ogre::RenderSystemCapabilities *> const&)")]
pub fn stub_0xd6b220() {
    // IDA 0xd6b220: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd6b400 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilities *>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilities *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd6b400() {
    // IDA 0xd6b400: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6b404 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilities *>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilities *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
pub fn stub_0xd6b404() {
    // IDA 0xd6b404: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6b410 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre24RenderSystemCapabilitiesEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::RenderSystemCapabilities *>,std::_Select1st<std::pair<std::string const,Ogre::RenderSystemCapabilities *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::RenderSystemCapabilities *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::RenderSystemCapabilities *>> *)")]
pub fn stub_0xd6b410() {
    // IDA 0xd6b410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6b4bc — __ZN4Ogre34RenderSystemCapabilitiesSerializerC1Ev
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesSerializer *__hidden this)
#[doc(alias = "__ZN4Ogre34RenderSystemCapabilitiesSerializerC1Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesSerializer::RenderSystemCapabilitiesSerializer(void)")]
pub fn stub_0xd6b4bc() {
    // IDA 0xd6b4bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6b4c8 — __ZN4Ogre34RenderSystemCapabilitiesSerializerC2Ev
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesSerializer *__hidden this)
#[doc(alias = "__ZN4Ogre34RenderSystemCapabilitiesSerializerC2Ev")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesSerializer::RenderSystemCapabilitiesSerializer(void)")]
pub fn stub_0xd6b4c8() {
    // IDA 0xd6b4c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd6b790 — __ZN4Ogre34RenderSystemCapabilitiesSerializer25initialiaseDispatchTablesEv
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesSerializer *__hidden this)
#[doc(alias = "__ZN4Ogre34RenderSystemCapabilitiesSerializer25initialiaseDispatchTablesEv")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesSerializer::initialiaseDispatchTables(void)")]
pub fn stub_0xd6b790() {
    // IDA 0xd6b790: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd77760 — __ZNK4Ogre34RenderSystemCapabilitiesSerializer13logParseErrorERKSs
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilitiesSerializer *__hidden this, const std::string *)
#[doc(alias = "__ZNK4Ogre34RenderSystemCapabilitiesSerializer13logParseErrorERKSs")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesSerializer::logParseError(std::string const&)const")]
pub fn stub_0xd77760() {
    // IDA 0xd77760: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd78000 — __ZN4Ogre34RenderSystemCapabilitiesSerializer22parseCapabilitiesLinesERSt6vectorISt4pairISsiENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, Ogre::NedPoolingImpl *, int, int, char, char, int, int, int, int, char, char, char, char, char, char, char, char, char, char, char, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre34RenderSystemCapabilitiesSerializer22parseCapabilitiesLinesERSt6vectorISt4pairISsiENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE")]
#[doc(alias = "Ogre::RenderSystemCapabilitiesSerializer::parseCapabilitiesLines(std::vector<std::pair<std::string,int>,Ogre::STLAllocator<std::pair<std::string,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
pub fn stub_0xd78000() {
    // IDA 0xd78000: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd78960 — __ZN4Ogre12STLAllocatorISt4pairISsiENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "__ZN4Ogre12STLAllocatorISt4pairISsiENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev")]
#[doc(alias = "Ogre::STLAllocator<std::pair<std::string,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
pub fn stub_0xd78960() {
    // IDA 0xd78960: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd78964 — __ZN4Ogre24RenderSystemCapabilities28parseDriverVersionFromStringERKSs
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, const std::string *)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities28parseDriverVersionFromStringERKSs")]
#[doc(alias = "Ogre::RenderSystemCapabilities::parseDriverVersionFromString(std::string const&)")]
pub fn stub_0xd78964() {
    // IDA 0xd78964: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd78998 — __ZN4Ogre24RenderSystemCapabilities19setRenderSystemNameERKSs
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, const std::string *)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities19setRenderSystemNameERKSs")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setRenderSystemName(std::string const&)")]
pub fn stub_0xd78998() {
    // IDA 0xd78998: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd789a4 — __ZN4Ogre24RenderSystemCapabilities21parseVendorFromStringERKSs
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, const std::string *)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities21parseVendorFromStringERKSs")]
#[doc(alias = "Ogre::RenderSystemCapabilities::parseVendorFromString(std::string const&)")]
pub fn stub_0xd789a4() {
    // IDA 0xd789a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xd789b8 — __ZN4Ogre24RenderSystemCapabilities18setNumTextureUnitsEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities18setNumTextureUnitsEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setNumTextureUnits(unsigned short)")]
pub fn stub_0xd789b8() {
    // IDA 0xd789b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd789bc — __ZN4Ogre24RenderSystemCapabilities24setStencilBufferBitDepthEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities24setStencilBufferBitDepthEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setStencilBufferBitDepth(unsigned short)")]
pub fn stub_0xd789bc() {
    // IDA 0xd789bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd789c0 — __ZN4Ogre24RenderSystemCapabilities25setNumVertexBlendMatricesEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities25setNumVertexBlendMatricesEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setNumVertexBlendMatrices(unsigned short)")]
pub fn stub_0xd789c0() {
    // IDA 0xd789c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xd789c4 — __ZN4Ogre24RenderSystemCapabilities24setNumMultiRenderTargetsEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities24setNumMultiRenderTargetsEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setNumMultiRenderTargets(unsigned short)")]
pub fn stub_0xd789c4() {
    // IDA 0xd789c4: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd789cc — __ZN4Ogre24RenderSystemCapabilities34setVertexProgramConstantFloatCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities34setVertexProgramConstantFloatCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setVertexProgramConstantFloatCount(unsigned short)")]
pub fn stub_0xd789cc() {
    // IDA 0xd789cc: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd789d4 — __ZN4Ogre24RenderSystemCapabilities32setVertexProgramConstantIntCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities32setVertexProgramConstantIntCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setVertexProgramConstantIntCount(unsigned short)")]
pub fn stub_0xd789d4() {
    // IDA 0xd789d4: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd789dc — __ZN4Ogre24RenderSystemCapabilities33setVertexProgramConstantBoolCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities33setVertexProgramConstantBoolCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setVertexProgramConstantBoolCount(unsigned short)")]
pub fn stub_0xd789dc() {
    // IDA 0xd789dc: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd789e4 — __ZN4Ogre24RenderSystemCapabilities36setFragmentProgramConstantFloatCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities36setFragmentProgramConstantFloatCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setFragmentProgramConstantFloatCount(unsigned short)")]
pub fn stub_0xd789e4() {
    // IDA 0xd789e4: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd789ec — __ZN4Ogre24RenderSystemCapabilities34setFragmentProgramConstantIntCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities34setFragmentProgramConstantIntCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setFragmentProgramConstantIntCount(unsigned short)")]
pub fn stub_0xd789ec() {
    // IDA 0xd789ec: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd789f4 — __ZN4Ogre24RenderSystemCapabilities35setFragmentProgramConstantBoolCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities35setFragmentProgramConstantBoolCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setFragmentProgramConstantBoolCount(unsigned short)")]
pub fn stub_0xd789f4() {
    // IDA 0xd789f4: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd789fc — __ZN4Ogre24RenderSystemCapabilities36setGeometryProgramConstantFloatCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities36setGeometryProgramConstantFloatCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setGeometryProgramConstantFloatCount(unsigned short)")]
pub fn stub_0xd789fc() {
    // IDA 0xd789fc: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd78a04 — __ZN4Ogre24RenderSystemCapabilities34setGeometryProgramConstantIntCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities34setGeometryProgramConstantIntCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setGeometryProgramConstantIntCount(unsigned short)")]
pub fn stub_0xd78a04() {
    // IDA 0xd78a04: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd78a0c — __ZN4Ogre24RenderSystemCapabilities35setGeometryProgramConstantBoolCountEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities35setGeometryProgramConstantBoolCountEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setGeometryProgramConstantBoolCount(unsigned short)")]
pub fn stub_0xd78a0c() {
    // IDA 0xd78a0c: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd78a14 — __ZN4Ogre24RenderSystemCapabilities24setNumVertexTextureUnitsEt
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities24setNumVertexTextureUnitsEt")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setNumVertexTextureUnits(unsigned short)")]
pub fn stub_0xd78a14() {
    // IDA 0xd78a14: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd78a1c — __ZN4Ogre24RenderSystemCapabilities25setNonPOW2TexturesLimitedEb
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, bool)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities25setNonPOW2TexturesLimitedEb")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setNonPOW2TexturesLimited(bool)")]
pub fn stub_0xd78a1c() {
    // IDA 0xd78a1c: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xd78a24 — __ZN4Ogre24RenderSystemCapabilities27setVertexTextureUnitsSharedEb
// type: _DWORD __fastcall(Ogre::RenderSystemCapabilities *__hidden this, bool)
#[doc(alias = "__ZN4Ogre24RenderSystemCapabilities27setVertexTextureUnitsSharedEb")]
#[doc(alias = "Ogre::RenderSystemCapabilities::setVertexTextureUnitsShared(bool)")]
pub fn stub_0xd78a24() {
    // IDA 0xd78a24: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xf662d4 — j___ZN3RBX9LightGrid31lightingUpdatePointLightScratchILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef
// type: int __fastcall(int, int, int, int, float, int, float)
#[doc(alias = "j___ZN3RBX9LightGrid31lightingUpdatePointLightScratchILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef")]
#[doc(alias = "void RBX::LightGrid::lightingUpdatePointLightScratch<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
pub fn stub_0xf662d4() {
    // IDA 0xf662d4: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xf662e4 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf662e4() {
    // IDA 0xf662e4: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xf662f4 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf662f4() {
    // IDA 0xf662f4: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xf66304 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf66304() {
    // IDA 0xf66304: Ogre render-engine type owned by the rendering crate — carrier no-op in core.
}

// 0xf66314 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb0ELb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<false,true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf66314() {
    // IDA 0xf66314: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66324 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb0ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,false,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf66324() {
    // IDA 0xf66324: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66334 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb0ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,false,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf66334() {
    // IDA 0xf66334: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66344 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf66344() {
    // IDA 0xf66344: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66354 — j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "j___ZN3RBX9LightGrid32lightingComputeShadowMaskImplLUTILb1ELb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E")]
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskImplLUT<true,true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
pub fn stub_0xf66354() {
    // IDA 0xf66354: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66364 — j___ZN3RBX9LightGrid34lightingUpdateSpotLightScratchSIMDILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef
// type: int __fastcall(int, int, int, int, float, int, float, int, float)
#[doc(alias = "j___ZN3RBX9LightGrid34lightingUpdateSpotLightScratchSIMDILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef")]
#[doc(alias = "void RBX::LightGrid::lightingUpdateSpotLightScratchSIMD<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
pub fn stub_0xf66364() {
    // IDA 0xf66364: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66374 — j___ZN3RBX9LightGrid34lightingUpdateSpotLightScratchSIMDILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef
// type: int __fastcall(int, int, int, int, float, int, float, int, float)
#[doc(alias = "j___ZN3RBX9LightGrid34lightingUpdateSpotLightScratchSIMDILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfS8_fRKNS5_11Color3uint8Ef")]
#[doc(alias = "void RBX::LightGrid::lightingUpdateSpotLightScratchSIMD<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
pub fn stub_0xf66374() {
    // IDA 0xf66374: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66384 — j___ZN3RBX9LightGrid35lightingUpdatePointLightScratchSIMDILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef
// type: int __fastcall(int, int, int, int, float, int, char)
#[doc(alias = "j___ZN3RBX9LightGrid35lightingUpdatePointLightScratchSIMDILb0EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef")]
#[doc(alias = "void RBX::LightGrid::lightingUpdatePointLightScratchSIMD<false>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
pub fn stub_0xf66384() {
    // IDA 0xf66384: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66394 — j___ZN3RBX9LightGrid35lightingUpdatePointLightScratchSIMDILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef
// type: int __fastcall(int, int, int, int, float, int, char)
#[doc(alias = "j___ZN3RBX9LightGrid35lightingUpdatePointLightScratchSIMDILb1EEEvRKNS_12Vector3int32ES4_RKN3G3D7Vector3EfRKNS5_11Color3uint8Ef")]
#[doc(alias = "void RBX::LightGrid::lightingUpdatePointLightScratchSIMD<true>(RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&,float,G3D::Color3uint8 const&,float)")]
pub fn stub_0xf66394() {
    // IDA 0xf66394: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66484 — j___ZNK3RBX27CornerWedgeDistanceFunctionclERKN3G3D7Vector3E
#[doc(alias = "j___ZNK3RBX27CornerWedgeDistanceFunctionclERKN3G3D7Vector3E")]
#[doc(alias = "RBX::CornerWedgeDistanceFunction::operator()(G3D::Vector3 const&)const")]
pub fn stub_0xf66484() {
    // IDA 0xf66484: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0xf66524 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS2_PKNS_12ExtentsInt32ES9_S9_b
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12changeMinMaxEPS2_PKNS_12ExtentsInt32ES9_S9_b")]
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::changeMinMax(Ogre::RbxCullableSceneNode*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,RBX::ExtentsInt32 const*,bool)")]
pub fn stub_0xf66524() {
    // IDA 0xf66524: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf66534 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS5_11SpatialNodeEPS2_RKNS_12Vector3int32Ei
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS5_11SpatialNodeEPS2_RKNS_12Vector3int32Ei")]
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&,int)")]
pub fn stub_0xf66534() {
    // IDA 0xf66534: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf66544 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS5_11SpatialNodeE
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS5_11SpatialNodeE")]
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
pub fn stub_0xf66544() {
    // IDA 0xf66544: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf66554 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, void *)
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E")]
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)")]
pub fn stub_0xf66554() {
    // IDA 0xf66554: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf66564 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS2_RKNS_12Vector3int32E
// type: int __fastcall(int, int, int)
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS2_RKNS_12Vector3int32E")]
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findNode(Ogre::RbxCullableSceneNode*,RBX::Vector3int32 const&)")]
pub fn stub_0xf66564() {
    // IDA 0xf66564: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf66574 — j___ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev")]
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)")]
pub fn stub_0xf66574() {
    // IDA 0xf66574: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf66584 — j___ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev
#[doc(alias = "j___ZN3RBX9AllocatorINS_11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev")]
#[doc(alias = "RBX::Allocator<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)")]
pub fn stub_0xf66584() {
    // IDA 0xf66584: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf665a4 — j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEE9constructIiiNS1_12Vector3int32EEEPS8_RT_RT0_RKT1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeENS1_16roblox_allocatorEE9constructIiiNS1_12Vector3int32EEEPS8_RT_RT0_RKT1_")]
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode* boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::construct<int,int,RBX::Vector3int32>(int &,int &,RBX::Vector3int32 const&)")]
pub fn stub_0xf665a4() {
    // IDA 0xf665a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf665b4 — j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE7destroyEPS8_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE7destroyEPS8_")]
#[doc(alias = "boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::destroy(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode*)")]
pub fn stub_0xf665b4() {
    // IDA 0xf665b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf665c4 — j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE9constructEv
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost11object_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeENS1_16roblox_allocatorEE9constructEv")]
#[doc(alias = "boost::object_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::construct(void)")]
pub fn stub_0xf665c4() {
    // IDA 0xf665c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf665d4 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE11SpatialNodeELj32ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0xf665d4() {
    // IDA 0xf665d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf665e4 — j___ZN5boost14singleton_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "j___ZN5boost14singleton_poolIN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS1_7ContactENS1_14ContactManagerELi4EE8TreeNodeELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0xf665e4() {
    // IDA 0xf665e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0xf67824 — j___ZNSt3mapISsPN4Ogre17InstancedGeometry14GeometryBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "j___ZNSt3mapISsPN4Ogre17InstancedGeometry14GeometryBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_")]
#[doc(alias = "std::map<std::string,Ogre::InstancedGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf67824() {
    // IDA 0xf67824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67834 — j___ZNSt3mapISsPN4Ogre17InstancedGeometry14MaterialBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "j___ZNSt3mapISsPN4Ogre17InstancedGeometry14MaterialBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_")]
#[doc(alias = "std::map<std::string,Ogre::InstancedGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_0xf67834() {
    // IDA 0xf67834: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67844 — j___ZNSt6vectorIN4Ogre17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "j___ZNSt6vectorIN4Ogre17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_")]
#[doc(alias = "std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::InstancedGeometry::SubMeshLodGeometryLink const&)")]
pub fn stub_0xf67844() {
    // IDA 0xf67844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67864 — j___ZNSt6vectorIPN4Ogre17InstancedGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "j___ZNSt6vectorIPN4Ogre17InstancedGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
#[doc(alias = "std::vector<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::QueuedSubMesh **,std::vector<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::QueuedSubMesh * const&)")]
pub fn stub_0xf67864() {
    // IDA 0xf67864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67874 — j___ZNSt6vectorIPN4Ogre17InstancedGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "j___ZNSt6vectorIPN4Ogre17InstancedGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
#[doc(alias = "std::vector<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::GeometryBucket **,std::vector<Ogre::InstancedGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::GeometryBucket * const&)")]
pub fn stub_0xf67874() {
    // IDA 0xf67874: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67884 — j___ZNSt6vectorIPN4Ogre17InstancedGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "j___ZNSt6vectorIPN4Ogre17InstancedGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
#[doc(alias = "std::vector<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::QueuedGeometry **,std::vector<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::InstancedGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::QueuedGeometry * const&)")]
pub fn stub_0xf67884() {
    // IDA 0xf67884: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67894 — j___ZNSt6vectorIPN4Ogre17InstancedGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "j___ZNSt6vectorIPN4Ogre17InstancedGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_")]
#[doc(alias = "std::vector<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::InstancedGeometry::LODBucket **,std::vector<Ogre::InstancedGeometry::LODBucket *,Ogre::STLAllocator<Ogre::InstancedGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::InstancedGeometry::LODBucket * const&)")]
pub fn stub_0xf67894() {
    // IDA 0xf67894: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf678a4 — j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueERKSF_
// type: int __fastcall(char *)
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueERKSF_")]
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_0xf678a4() {
    // IDA 0xf678a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf678b4 — j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISF_ERKSF_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE16_M_insert_uniqueESt17_Rb_tree_iteratorISF_ERKSF_")]
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *> const&)")]
pub fn stub_0xf678b4() {
    // IDA 0xf678b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf678c4 — j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISF_E
#[doc(alias = "j___ZNSt8_Rb_treeIPN4Ogre7SubMeshESt4pairIKS2_PSt6vectorINS0_17InstancedGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEESt10_Select1stISF_ESt4lessIS2_ENS8_ISF_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISF_E")]
#[doc(alias = "std::_Rb_tree<Ogre::SubMesh *,std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,std::_Select1st<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>>,std::less<Ogre::SubMesh *>,Ogre::STLAllocator<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::SubMesh * const,std::vector<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::InstancedGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *>> *)")]
pub fn stub_0xf678c4() {
    // IDA 0xf678c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf678d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)")]
pub fn stub_0xf678d4() {
    // IDA 0xf678d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf678e4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)")]
pub fn stub_0xf678e4() {
    // IDA 0xf678e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf678f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf678f4() {
    // IDA 0xf678f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67904 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>> *)")]
pub fn stub_0xf67904() {
    // IDA 0xf67904: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67914 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14GeometryBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstancedGeometry::GeometryBucket *> const&)")]
pub fn stub_0xf67914() {
    // IDA 0xf67914: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67924 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)")]
pub fn stub_0xf67924() {
    // IDA 0xf67924: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67934 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)")]
pub fn stub_0xf67934() {
    // IDA 0xf67934: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67944 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_0xf67944() {
    // IDA 0xf67944: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67954 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>> *)")]
pub fn stub_0xf67954() {
    // IDA 0xf67954: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67964 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre17InstancedGeometry14MaterialBucketEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,std::_Select1st<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::InstancedGeometry::MaterialBucket *> const&)")]
pub fn stub_0xf67964() {
    // IDA 0xf67964: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67974 — j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(char *)
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *> const&)")]
pub fn stub_0xf67974() {
    // IDA 0xf67974: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67984 — j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *> const&)")]
pub fn stub_0xf67984() {
    // IDA 0xf67984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf67994 — j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjPN4Ogre17InstancedGeometry13BatchInstanceEESt10_Select1stIS6_ESt4lessIjENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,std::_Select1st<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>>,std::less<unsigned int>,Ogre::STLAllocator<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,Ogre::InstancedGeometry::BatchInstance *>> *)")]
pub fn stub_0xf67994() {
    // IDA 0xf67994: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf679c4 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
// type: int __fastcall(char *)
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *> const&)")]
pub fn stub_0xf679c4() {
    // IDA 0xf679c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf679d4 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *> const&)")]
pub fn stub_0xf679d4() {
    // IDA 0xf679d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf679e4 — j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "j___ZNSt8_Rb_treeItSt4pairIKtPN4Ogre17InstancedGeometry15InstancedObjectEESt10_Select1stIS6_ESt4lessItENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,std::_Select1st<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::InstancedGeometry::InstancedObject *>> *)")]
pub fn stub_0xf679e4() {
    // IDA 0xf679e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
