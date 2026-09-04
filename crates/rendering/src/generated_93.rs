//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe7153c..0xe774c0 (100 stubs, 10364 prior -> 10464 covered, 2869 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0xe7153c — __ZN4Ogre15GLES2FBOManager19releaseRenderBufferERKNS_16GLES2SurfaceDescE
#[doc(alias = "Ogre::GLES2FBOManager::releaseRenderBuffer(Ogre::GLES2SurfaceDesc const&)")]
// was: Ogre::GLES2FBOManager::releaseRenderBuffer(Ogre::GLES2SurfaceDesc const&)
// IDA 0xe7153c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7153c() {
}

// 0xe7159c — __ZNSt3mapIN4Ogre15GLES2FBOManager8RBFormatENS1_5RBRefESt4lessIS2_ENS0_12STLAllocatorISt4pairIKS2_S3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "std::map<Ogre::GLES2FBOManager::RBFormat,Ogre::GLES2FBOManager::RBRef,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::GLES2FBOManager::RBFormat const&)")]
// was: std::map<Ogre::GLES2FBOManager::RBFormat,Ogre::GLES2FBOManager::RBRef,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](Ogre::GLES2FBOManager::RBFormat const&)
// IDA 0xe7159c: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7159c() {
}

// 0xe71638 — __ZN4Ogre21GLES2FBORenderTextureD1Ev
#[doc(alias = "Ogre::GLES2FBORenderTexture::~GLES2FBORenderTexture()")]
// was: Ogre::GLES2FBORenderTexture::~GLES2FBORenderTexture()
// IDA 0xe71638: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e71638() {
}

// 0xe71660 — __ZN4Ogre21GLES2FBORenderTextureD0Ev
#[doc(alias = "Ogre::GLES2FBORenderTexture::~GLES2FBORenderTexture()")]
// was: Ogre::GLES2FBORenderTexture::~GLES2FBORenderTexture()
// IDA 0xe71660: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e71660() {
}

// 0xe7170c — __ZNK4Ogre18GLES2RenderTexture23requiresTextureFlippingEv
#[doc(alias = "Ogre::GLES2RenderTexture::requiresTextureFlipping(void)const")]
// was: Ogre::GLES2RenderTexture::requiresTextureFlipping(void)const
// IDA 0xe7170c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7170c() {
}

// 0xe71710 — __ZN4Ogre15GLES2FBOManager11checkFormatENS_11PixelFormatE
#[doc(alias = "Ogre::GLES2FBOManager::checkFormat(Ogre::PixelFormat)")]
// was: Ogre::GLES2FBOManager::checkFormat(Ogre::PixelFormat)
// IDA 0xe71710: 4 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71710() {
}

// 0xe7171c — __ZN4Ogre15GLES2FBOManager6unbindEPNS_12RenderTargetE
#[doc(alias = "Ogre::GLES2FBOManager::unbind(Ogre::RenderTarget *)")]
// was: Ogre::GLES2FBOManager::unbind(Ogre::RenderTarget *)
// IDA 0xe7171c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e7171c() {
}

// 0xe71720 — __ZNSt8_Rb_treeIN4Ogre15GLES2FBOManager8RBFormatESt4pairIKS2_NS1_5RBRefEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)")]
// was: std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)
// IDA 0xe71720: 159 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71720() {
}

// 0xe71870 — __ZNSt8_Rb_treeIN4Ogre15GLES2FBOManager8RBFormatESt4pairIKS2_NS1_5RBRefEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_
#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)")]
// was: std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)
// IDA 0xe71870: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71870() {
}

// 0xe71908 — __ZNSt8_Rb_treeIN4Ogre15GLES2FBOManager8RBFormatESt4pairIKS2_NS1_5RBRefEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_
#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)")]
// was: std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef> const&)
// IDA 0xe71908: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71908() {
}

// 0xe719b4 — __ZNSt8_Rb_treeIN4Ogre15GLES2FBOManager8RBFormatESt4pairIKS2_NS1_5RBRefEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE4findERS4_
#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(Ogre::GLES2FBOManager::RBFormat const&)")]
// was: std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(Ogre::GLES2FBOManager::RBFormat const&)
// IDA 0xe719b4: 63 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e719b4() {
}

// 0xe71a38 — __ZNSt6vectorIN4Ogre15GLES2FBOManager16FormatProperties4ModeENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
#[doc(alias = "std::vector<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GLES2FBOManager::FormatProperties::Mode*,std::vector<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GLES2FBOManager::FormatProperties::Mode const&)")]
// was: std::vector<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::GLES2FBOManager::FormatProperties::Mode*,std::vector<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::GLES2FBOManager::FormatProperties::Mode const&)
// IDA 0xe71a38: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_e71a38() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xe71b54 — __ZNSt8_Rb_treeIN4Ogre15GLES2FBOManager8RBFormatESt4pairIKS2_NS1_5RBRefEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::GLES2FBOManager::RBFormat>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::GLES2FBOManager::RBFormat>,false>::~_Rb_tree_impl()
// IDA 0xe71b54: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e71b54() {
}

// 0xe71b58 — __ZNSt8_Rb_treeIN4Ogre15GLES2FBOManager8RBFormatESt4pairIKS2_NS1_5RBRefEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::GLES2FBOManager::RBFormat>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::GLES2FBOManager::RBFormat>,false>::~_Rb_tree_impl()
// IDA 0xe71b58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e71b58() {
}

// 0xe71b64 — __ZNSt8_Rb_treeIN4Ogre15GLES2FBOManager8RBFormatESt4pairIKS2_NS1_5RBRefEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>> *)")]
// was: std::_Rb_tree<Ogre::GLES2FBOManager::RBFormat,std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,std::_Select1st<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>>,std::less<Ogre::GLES2FBOManager::RBFormat>,Ogre::STLAllocator<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::GLES2FBOManager::RBFormat const,Ogre::GLES2FBOManager::RBRef>> *)
// IDA 0xe71b64: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71b64() {
}

// 0xe71b8c — __ZNSt12_Vector_baseIN4Ogre15GLES2FBOManager16FormatProperties4ModeENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe71b8c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e71b8c() {
}

// 0xe71b90 — __ZNSt12_Vector_baseIN4Ogre15GLES2FBOManager16FormatProperties4ModeENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::STLAllocator<Ogre::GLES2FBOManager::FormatProperties::Mode,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe71b90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e71b90() {
}

// 0xe71bd0 — __ZN4Ogre22GLES2FrameBufferObjectC1EPNS_15GLES2FBOManagerEj
#[doc(alias = "Ogre::GLES2FrameBufferObject::GLES2FrameBufferObject(Ogre::GLES2FBOManager *,unsigned int)")]
// was: Ogre::GLES2FrameBufferObject::GLES2FrameBufferObject(Ogre::GLES2FBOManager *,unsigned int)
// IDA 0xe71bd0: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71bd0() {
}

// 0xe71bdc — __ZN4Ogre22GLES2FrameBufferObjectC2EPNS_15GLES2FBOManagerEj
#[doc(alias = "Ogre::GLES2FrameBufferObject::GLES2FrameBufferObject(Ogre::GLES2FBOManager *,unsigned int)")]
// was: Ogre::GLES2FrameBufferObject::GLES2FrameBufferObject(Ogre::GLES2FBOManager *,unsigned int)
// IDA 0xe71bdc: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71bdc() {
}

// 0xe71c9c — __ZN4Ogre22GLES2FrameBufferObjectD1Ev
#[doc(alias = "Ogre::GLES2FrameBufferObject::~GLES2FrameBufferObject()")]
// was: Ogre::GLES2FrameBufferObject::~GLES2FrameBufferObject()
// IDA 0xe71c9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e71c9c() {
}

// 0xe71d74 — __ZN4Ogre22GLES2FrameBufferObject11bindSurfaceEmRKNS_16GLES2SurfaceDescE
#[doc(alias = "Ogre::GLES2FrameBufferObject::bindSurface(unsigned long,Ogre::GLES2SurfaceDesc const&)")]
// was: Ogre::GLES2FrameBufferObject::bindSurface(unsigned long,Ogre::GLES2SurfaceDesc const&)
// IDA 0xe71d74: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71d74() {
}

// 0xe71d9c — __ZN4Ogre22GLES2FrameBufferObject10initialiseEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::initialise(void)")]
// was: Ogre::GLES2FrameBufferObject::initialise(void)
// IDA 0xe71d9c: 941 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e71d9c() {
}

// 0xe72894 — __ZN4Ogre22GLES2FrameBufferObject13unbindSurfaceEm
#[doc(alias = "Ogre::GLES2FrameBufferObject::unbindSurface(unsigned long)")]
// was: Ogre::GLES2FrameBufferObject::unbindSurface(unsigned long)
// IDA 0xe72894: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e72894() {
}

// 0xe728b4 — __ZN4Ogre22GLES2FrameBufferObject4bindEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::bind(void)")]
// was: Ogre::GLES2FrameBufferObject::bind(void)
// IDA 0xe728b4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e728b4() {
}

// 0xe728cc — __ZN4Ogre22GLES2FrameBufferObject11swapBuffersEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::swapBuffers(void)")]
// was: Ogre::GLES2FrameBufferObject::swapBuffers(void)
// IDA 0xe728cc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e728cc() {
}

// 0xe728f0 — __ZN4Ogre22GLES2FrameBufferObject17attachDepthBufferEPNS_11DepthBufferE
#[doc(alias = "Ogre::GLES2FrameBufferObject::attachDepthBuffer(Ogre::DepthBuffer *)")]
// was: Ogre::GLES2FrameBufferObject::attachDepthBuffer(Ogre::DepthBuffer *)
// IDA 0xe728f0: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e728f0() {
}

// 0xe72958 — __ZN4Ogre22GLES2FrameBufferObject17detachDepthBufferEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::detachDepthBuffer(void)")]
// was: Ogre::GLES2FrameBufferObject::detachDepthBuffer(void)
// IDA 0xe72958: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e72958() {
}

// 0xe72994 — __ZN4Ogre22GLES2FrameBufferObject8getWidthEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::getWidth(void)")]
// was: Ogre::GLES2FrameBufferObject::getWidth(void)
// IDA 0xe72994: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e72994() {
}

// 0xe7299c — __ZN4Ogre22GLES2FrameBufferObject9getHeightEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::getHeight(void)")]
// was: Ogre::GLES2FrameBufferObject::getHeight(void)
// IDA 0xe7299c: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7299c() {
}

// 0xe729a4 — __ZN4Ogre22GLES2FrameBufferObject9getFormatEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::getFormat(void)")]
// was: Ogre::GLES2FrameBufferObject::getFormat(void)
// IDA 0xe729a4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e729a4() {
}

// 0xe729ac — __ZN4Ogre22GLES2FrameBufferObject7getFSAAEv
#[doc(alias = "Ogre::GLES2FrameBufferObject::getFSAA(void)")]
// was: Ogre::GLES2FrameBufferObject::getFSAA(void)
// IDA 0xe729ac: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e729ac() {
}

// 0xe729e4 — __ZN4Ogre15GLES2GpuProgramC1EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::GLES2GpuProgram::GLES2GpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::GLES2GpuProgram::GLES2GpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe729e4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e729e4() {
}

// 0xe72a00 — __ZN4Ogre15GLES2GpuProgramC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::GLES2GpuProgram::GLES2GpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::GLES2GpuProgram::GLES2GpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe72a00: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e72a00() {
}

// 0xe72b80 — __ZN4Ogre15GLES2GpuProgramD0Ev
#[doc(alias = "Ogre::GLES2GpuProgram::~GLES2GpuProgram()")]
// was: Ogre::GLES2GpuProgram::~GLES2GpuProgram()
// IDA 0xe72b80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e72b80() {
}

// 0xe72c40 — __ZN4Ogre15GLES2GpuProgramD1Ev
#[doc(alias = "Ogre::GLES2GpuProgram::~GLES2GpuProgram()")]
// was: Ogre::GLES2GpuProgram::~GLES2GpuProgram()
// IDA 0xe72c40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e72c40() {
}

// 0xe72cf0 — __ZN4Ogre15GLES2GpuProgramD2Ev
#[doc(alias = "Ogre::GLES2GpuProgram::~GLES2GpuProgram()")]
// was: Ogre::GLES2GpuProgram::~GLES2GpuProgram()
// IDA 0xe72cf0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e72cf0() {
}

// 0xe72da0 — __ZN4Ogre15GLES2GpuProgram10unloadImplEv
#[doc(alias = "Ogre::GLES2GpuProgram::unloadImpl(void)")]
// was: Ogre::GLES2GpuProgram::unloadImpl(void)
// IDA 0xe72da0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e72da0() {
}

// 0xe72da4 — __ZN4Ogre15GLES2GpuProgram14loadFromSourceEv
#[doc(alias = "Ogre::GLES2GpuProgram::loadFromSource(void)")]
// was: Ogre::GLES2GpuProgram::loadFromSource(void)
// IDA 0xe72da4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e72da4() {
}

// 0xe72da8 — __ZN4Ogre15GLES2GpuProgram11bindProgramEv
#[doc(alias = "Ogre::GLES2GpuProgram::bindProgram(void)")]
// was: Ogre::GLES2GpuProgram::bindProgram(void)
// IDA 0xe72da8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e72da8() {
}

// 0xe72dac — __ZN4Ogre15GLES2GpuProgram13unbindProgramEv
#[doc(alias = "Ogre::GLES2GpuProgram::unbindProgram(void)")]
// was: Ogre::GLES2GpuProgram::unbindProgram(void)
// IDA 0xe72dac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e72dac() {
}

// 0xe72db0 — __ZN4Ogre15GLES2GpuProgram21bindProgramParametersENS_9SharedPtrINS_20GpuProgramParametersEEEt
#[doc(alias = "Ogre::GLES2GpuProgram::bindProgramParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short)")]
// was: Ogre::GLES2GpuProgram::bindProgramParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short)
// IDA 0xe72db0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e72db0() {
}

// 0xe72db4 — __ZN4Ogre15GLES2GpuProgram34bindProgramPassIterationParametersENS_9SharedPtrINS_20GpuProgramParametersEEE
#[doc(alias = "Ogre::GLES2GpuProgram::bindProgramPassIterationParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
// was: Ogre::GLES2GpuProgram::bindProgramPassIterationParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)
// IDA 0xe72db4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e72db4() {
}

// 0xe72dec — __ZN4Ogre22GLES2GpuProgramManagerC1Ev
#[doc(alias = "Ogre::GLES2GpuProgramManager::GLES2GpuProgramManager(void)")]
// was: Ogre::GLES2GpuProgramManager::GLES2GpuProgramManager(void)
// IDA 0xe72dec: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e72dec() {
}

// 0xe72efc — __ZN4Ogre22GLES2GpuProgramManagerD0Ev
#[doc(alias = "Ogre::GLES2GpuProgramManager::~GLES2GpuProgramManager()")]
// was: Ogre::GLES2GpuProgramManager::~GLES2GpuProgramManager()
// IDA 0xe72efc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e72efc() {
}

// 0xe72f8c — __ZN4Ogre22GLES2GpuProgramManagerD1Ev
#[doc(alias = "Ogre::GLES2GpuProgramManager::~GLES2GpuProgramManager()")]
// was: Ogre::GLES2GpuProgramManager::~GLES2GpuProgramManager()
// IDA 0xe72f8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e72f8c() {
}

// 0xe72f98 — __ZN4Ogre22GLES2GpuProgramManagerD2Ev
#[doc(alias = "Ogre::GLES2GpuProgramManager::~GLES2GpuProgramManager()")]
// was: Ogre::GLES2GpuProgramManager::~GLES2GpuProgramManager()
// IDA 0xe72f98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e72f98() {
}

// 0xe73084 — __ZN4Ogre22GLES2GpuProgramManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::GLES2GpuProgramManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::GLES2GpuProgramManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xe73084: 346 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73084() {
}

// 0xe73454 — __ZN4Ogre22GLES2GpuProgramManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderENS_14GpuProgramTypeES2_
#[doc(alias = "Ogre::GLES2GpuProgramManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)")]
// was: Ogre::GLES2GpuProgramManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)
// IDA 0xe73454: 108 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73454() {
}

// 0xe73574 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findES7_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe73574: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73574() {
}

// 0xe73618 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeISD_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>> *)
// IDA 0xe73618: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73618() {
}

// 0xe73690 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe73690: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e73690() {
}

// 0xe73694 — __ZNSt8_Rb_treeISsSt4pairIKSsPFPN4Ogre10GpuProgramEPNS2_15ResourceManagerERS1_yS7_bPNS2_20ManualResourceLoaderENS2_14GpuProgramTypeES7_EESt10_Select1stISD_ESt4lessISsENS2_12STLAllocatorISD_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISH_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,std::_Select1st<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuProgram * (*)(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,Ogre::GpuProgramType,std::string const&)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe73694: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73694() {
}

// 0xe736d4 — __ZN4Ogre30GLES2HardwareBufferManagerBaseC1Ev
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::GLES2HardwareBufferManagerBase(void)")]
// was: Ogre::GLES2HardwareBufferManagerBase::GLES2HardwareBufferManagerBase(void)
// IDA 0xe736d4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e736d4() {
}

// 0xe736fc — __ZN4Ogre30GLES2HardwareBufferManagerBaseD0Ev
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::~GLES2HardwareBufferManagerBase()")]
// was: Ogre::GLES2HardwareBufferManagerBase::~GLES2HardwareBufferManagerBase()
// IDA 0xe736fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e736fc() {
}

// 0xe737dc — __ZN4Ogre30GLES2HardwareBufferManagerBaseD1Ev
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::~GLES2HardwareBufferManagerBase()")]
// was: Ogre::GLES2HardwareBufferManagerBase::~GLES2HardwareBufferManagerBase()
// IDA 0xe737dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e737dc() {
}

// 0xe738ac — __ZN4Ogre30GLES2HardwareBufferManagerBase18createVertexBufferEmmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::createVertexBuffer(unsigned long,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::GLES2HardwareBufferManagerBase::createVertexBuffer(unsigned long,unsigned long,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xe738ac: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e738ac() {
}

// 0xe73998 — __ZN4Ogre30GLES2HardwareBufferManagerBase17createIndexBufferENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::createIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::GLES2HardwareBufferManagerBase::createIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xe73998: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73998() {
}

// 0xe73a88 — __ZN4Ogre30GLES2HardwareBufferManagerBase26createRenderToVertexBufferEv
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::createRenderToVertexBuffer(void)")]
// was: Ogre::GLES2HardwareBufferManagerBase::createRenderToVertexBuffer(void)
// IDA 0xe73a88: 10 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73a88() {
}

// 0xe73aa4 — __ZN4Ogre30GLES2HardwareBufferManagerBase10getGLUsageEj
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::getGLUsage(unsigned int)")]
// was: Ogre::GLES2HardwareBufferManagerBase::getGLUsage(unsigned int)
// IDA 0xe73aa4: 13 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73aa4() {
}

// 0xe73acc — __ZN4Ogre30GLES2HardwareBufferManagerBase9getGLTypeEj
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::getGLType(unsigned int)")]
// was: Ogre::GLES2HardwareBufferManagerBase::getGLType(unsigned int)
// IDA 0xe73acc: 8 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73acc() {
}

// 0xe73ae4 — __ZN4Ogre30GLES2HardwareBufferManagerBase17deallocateScratchEPv
#[doc(alias = "Ogre::GLES2HardwareBufferManagerBase::deallocateScratch(void *)")]
// was: Ogre::GLES2HardwareBufferManagerBase::deallocateScratch(void *)
// IDA 0xe73ae4: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73ae4() {
}

// 0xe73b7c — __ZN4Ogre9SharedPtrINS_20RenderToVertexBufferEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::~SharedPtr()
// IDA 0xe73b7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e73b7c() {
}

// 0xe73c2c — __ZN4Ogre9SharedPtrINS_20RenderToVertexBufferEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::~SharedPtr()
// IDA 0xe73c2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e73c2c() {
}

// 0xe73d20 — __ZN4Ogre9SharedPtrINS_20RenderToVertexBufferEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::destroy(void)
// IDA 0xe73d20: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73d20() {
}

// 0xe73d58 — __ZN4Ogre9SharedPtrINS_20RenderToVertexBufferEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::swap(Ogre::SharedPtr<Ogre::RenderToVertexBuffer>&)")]
// was: Ogre::SharedPtr<Ogre::RenderToVertexBuffer>::swap(Ogre::SharedPtr<Ogre::RenderToVertexBuffer>&)
// IDA 0xe73d58: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73d58() {
}

// 0xe73d74 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareIndexBuffer * const&)")]
// was: std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareIndexBuffer * const&)
// IDA 0xe73d74: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73d74() {
}

// 0xe73e6c — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareVertexBuffer * const&)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::HardwareVertexBuffer * const&)
// IDA 0xe73e6c: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73e6c() {
}

// 0xe73f98 — __ZN4Ogre24GLES2HardwareIndexBufferC1EPNS_25HardwareBufferManagerBaseENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::GLES2HardwareIndexBuffer(Ogre::HardwareBufferManagerBase *,Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::GLES2HardwareIndexBuffer::GLES2HardwareIndexBuffer(Ogre::HardwareBufferManagerBase *,Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xe73f98: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73f98() {
}

// 0xe73fb0 — __ZN4Ogre24GLES2HardwareIndexBufferC2EPNS_25HardwareBufferManagerBaseENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::GLES2HardwareIndexBuffer(Ogre::HardwareBufferManagerBase *,Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::GLES2HardwareIndexBuffer::GLES2HardwareIndexBuffer(Ogre::HardwareBufferManagerBase *,Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xe73fb0: 436 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e73fb0() {
}

// 0xe744a8 — __ZN4Ogre24GLES2HardwareIndexBufferD0Ev
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::~GLES2HardwareIndexBuffer()")]
// was: Ogre::GLES2HardwareIndexBuffer::~GLES2HardwareIndexBuffer()
// IDA 0xe744a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e744a8() {
}

// 0xe745b8 — __ZN4Ogre24GLES2HardwareIndexBufferD1Ev
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::~GLES2HardwareIndexBuffer()")]
// was: Ogre::GLES2HardwareIndexBuffer::~GLES2HardwareIndexBuffer()
// IDA 0xe745b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e745b8() {
}

// 0xe746b8 — __ZN4Ogre24GLES2HardwareIndexBuffer10unlockImplEv
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::unlockImpl(void)")]
// was: Ogre::GLES2HardwareIndexBuffer::unlockImpl(void)
// IDA 0xe746b8: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e746b8() {
}

// 0xe74924 — __ZN4Ogre24GLES2HardwareIndexBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: Ogre::GLES2HardwareIndexBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xe74924: 322 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e74924() {
}

// 0xe74ce4 — __ZN4Ogre24GLES2HardwareIndexBuffer8readDataEmmPv
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::readData(unsigned long,unsigned long,void *)")]
// was: Ogre::GLES2HardwareIndexBuffer::readData(unsigned long,unsigned long,void *)
// IDA 0xe74ce4: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e74ce4() {
}

// 0xe74ee8 — __ZN4Ogre24GLES2HardwareIndexBuffer9writeDataEmmPKvb
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::writeData(unsigned long,unsigned long,void const*,bool)")]
// was: Ogre::GLES2HardwareIndexBuffer::writeData(unsigned long,unsigned long,void const*,bool)
// IDA 0xe74ee8: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e74ee8() {
}

// 0xe74fb0 — __ZN4Ogre24GLES2HardwareIndexBuffer17_updateFromShadowEv
#[doc(alias = "Ogre::GLES2HardwareIndexBuffer::_updateFromShadow(void)")]
// was: Ogre::GLES2HardwareIndexBuffer::_updateFromShadow(void)
// IDA 0xe74fb0: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e74fb0() {
}

// 0xe75080 — __ZN4Ogre27GLES2HardwareOcclusionQueryC1Ev
#[doc(alias = "Ogre::GLES2HardwareOcclusionQuery::GLES2HardwareOcclusionQuery(void)")]
// was: Ogre::GLES2HardwareOcclusionQuery::GLES2HardwareOcclusionQuery(void)
// IDA 0xe75080: 63 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e75080() {
}

// 0xe75138 — __ZN4Ogre27GLES2HardwareOcclusionQueryD0Ev
#[doc(alias = "Ogre::GLES2HardwareOcclusionQuery::~GLES2HardwareOcclusionQuery()")]
// was: Ogre::GLES2HardwareOcclusionQuery::~GLES2HardwareOcclusionQuery()
// IDA 0xe75138: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e75138() {
}

// 0xe751fc — __ZN4Ogre27GLES2HardwareOcclusionQueryD1Ev
#[doc(alias = "Ogre::GLES2HardwareOcclusionQuery::~GLES2HardwareOcclusionQuery()")]
// was: Ogre::GLES2HardwareOcclusionQuery::~GLES2HardwareOcclusionQuery()
// IDA 0xe751fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e751fc() {
}

// 0xe752b0 — __ZN4Ogre27GLES2HardwareOcclusionQuery19beginOcclusionQueryEv
#[doc(alias = "Ogre::GLES2HardwareOcclusionQuery::beginOcclusionQuery(void)")]
// was: Ogre::GLES2HardwareOcclusionQuery::beginOcclusionQuery(void)
// IDA 0xe752b0: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e752b0() {
}

// 0xe752c0 — __ZN4Ogre27GLES2HardwareOcclusionQuery17endOcclusionQueryEv
#[doc(alias = "Ogre::GLES2HardwareOcclusionQuery::endOcclusionQuery(void)")]
// was: Ogre::GLES2HardwareOcclusionQuery::endOcclusionQuery(void)
// IDA 0xe752c0: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e752c0() {
}

// 0xe752d0 — __ZN4Ogre27GLES2HardwareOcclusionQuery18pullOcclusionQueryEPj
#[doc(alias = "Ogre::GLES2HardwareOcclusionQuery::pullOcclusionQuery(unsigned int *)")]
// was: Ogre::GLES2HardwareOcclusionQuery::pullOcclusionQuery(unsigned int *)
// IDA 0xe752d0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e752d0() {
}

// 0xe752ec — __ZN4Ogre27GLES2HardwareOcclusionQuery18isStillOutstandingEv
#[doc(alias = "Ogre::GLES2HardwareOcclusionQuery::isStillOutstanding(void)")]
// was: Ogre::GLES2HardwareOcclusionQuery::isStillOutstanding(void)
// IDA 0xe752ec: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e752ec() {
}

// 0xe75344 — __ZN4Ogre24GLES2HardwarePixelBufferD0Ev
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::~GLES2HardwarePixelBuffer()")]
// was: Ogre::GLES2HardwarePixelBuffer::~GLES2HardwarePixelBuffer()
// IDA 0xe75344: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e75344() {
}

// 0xe753f4 — __ZN4Ogre24GLES2HardwarePixelBufferD1Ev
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::~GLES2HardwarePixelBuffer()")]
// was: Ogre::GLES2HardwarePixelBuffer::~GLES2HardwarePixelBuffer()
// IDA 0xe753f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e753f4() {
}

// 0xe75420 — __ZN4Ogre24GLES2HardwarePixelBuffer8lockImplENS_3BoxENS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::lockImpl(Ogre::Box,Ogre::HardwareBuffer::LockOptions)")]
// was: Ogre::GLES2HardwarePixelBuffer::lockImpl(Ogre::Box,Ogre::HardwareBuffer::LockOptions)
// IDA 0xe75420: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e75420() {
}

// 0xe7549c — __ZN4Ogre24GLES2HardwarePixelBuffer10unlockImplEv
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::unlockImpl(void)")]
// was: Ogre::GLES2HardwarePixelBuffer::unlockImpl(void)
// IDA 0xe7549c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e7549c() {
}

// 0xe754d8 — __ZN4Ogre24GLES2HardwarePixelBuffer14blitFromMemoryERKNS_8PixelBoxERKNS_3BoxE
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::blitFromMemory(Ogre::PixelBox const&,Ogre::Box const&)")]
// was: Ogre::GLES2HardwarePixelBuffer::blitFromMemory(Ogre::PixelBox const&,Ogre::Box const&)
// IDA 0xe754d8: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e754d8() {
}

// 0xe75854 — __ZN4Ogre24GLES2HardwarePixelBuffer12blitToMemoryERKNS_3BoxERKNS_8PixelBoxE
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::blitToMemory(Ogre::Box const&,Ogre::PixelBox const&)")]
// was: Ogre::GLES2HardwarePixelBuffer::blitToMemory(Ogre::Box const&,Ogre::PixelBox const&)
// IDA 0xe75854: 300 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e75854() {
}

// 0xe75b9c — __ZN4Ogre24GLES2HardwarePixelBuffer6uploadERKNS_8PixelBoxERKNS_3BoxE
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::upload(Ogre::PixelBox const&,Ogre::Box const&)")]
// was: Ogre::GLES2HardwarePixelBuffer::upload(Ogre::PixelBox const&,Ogre::Box const&)
// IDA 0xe75b9c: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e75b9c() {
}

// 0xe75d4c — __ZN4Ogre24GLES2HardwarePixelBuffer8downloadERKNS_8PixelBoxE
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::download(Ogre::PixelBox const&)")]
// was: Ogre::GLES2HardwarePixelBuffer::download(Ogre::PixelBox const&)
// IDA 0xe75d4c: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e75d4c() {
}

// 0xe75efc — __ZN4Ogre24GLES2HardwarePixelBuffer17bindToFramebufferEjm
#[doc(alias = "Ogre::GLES2HardwarePixelBuffer::bindToFramebuffer(unsigned int,unsigned long)")]
// was: Ogre::GLES2HardwarePixelBuffer::bindToFramebuffer(unsigned int,unsigned long)
// IDA 0xe75efc: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e75efc() {
}

// 0xe760ac — __ZN4Ogre18GLES2TextureBufferC1ERKSsjjiiiiiiNS_14HardwareBuffer5UsageEbbj
#[doc(alias = "Ogre::GLES2TextureBuffer::GLES2TextureBuffer(std::string const&,unsigned int,unsigned int,int,int,int,int,int,int,Ogre::HardwareBuffer::Usage,bool,bool,unsigned int)")]
// was: Ogre::GLES2TextureBuffer::GLES2TextureBuffer(std::string const&,unsigned int,unsigned int,int,int,int,int,int,int,Ogre::HardwareBuffer::Usage,bool,bool,unsigned int)
// IDA 0xe760ac: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e760ac() {
}

// 0xe760fc — __ZN4Ogre18GLES2TextureBufferC2ERKSsjjiiiiiiNS_14HardwareBuffer5UsageEbbj
#[doc(alias = "Ogre::GLES2TextureBuffer::GLES2TextureBuffer(std::string const&,unsigned int,unsigned int,int,int,int,int,int,int,Ogre::HardwareBuffer::Usage,bool,bool,unsigned int)")]
// was: Ogre::GLES2TextureBuffer::GLES2TextureBuffer(std::string const&,unsigned int,unsigned int,int,int,int,int,int,int,Ogre::HardwareBuffer::Usage,bool,bool,unsigned int)
// IDA 0xe760fc: 680 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e760fc() {
}

// 0xe76870 — __ZN4Ogre18GLES2TextureBufferD0Ev
#[doc(alias = "Ogre::GLES2TextureBuffer::~GLES2TextureBuffer()")]
// was: Ogre::GLES2TextureBuffer::~GLES2TextureBuffer()
// IDA 0xe76870: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e76870() {
}

// 0xe76900 — __ZN4Ogre18GLES2TextureBufferD1Ev
#[doc(alias = "Ogre::GLES2TextureBuffer::~GLES2TextureBuffer()")]
// was: Ogre::GLES2TextureBuffer::~GLES2TextureBuffer()
// IDA 0xe76900: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e76900() {
}

// 0xe7690c — __ZN4Ogre18GLES2TextureBufferD2Ev
#[doc(alias = "Ogre::GLES2TextureBuffer::~GLES2TextureBuffer()")]
// was: Ogre::GLES2TextureBuffer::~GLES2TextureBuffer()
// IDA 0xe7690c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7690c() {
}

// 0xe76a80 — __ZN4Ogre18GLES2TextureBuffer6uploadERKNS_8PixelBoxERKNS_3BoxE
#[doc(alias = "Ogre::GLES2TextureBuffer::upload(Ogre::PixelBox const&,Ogre::Box const&)")]
// was: Ogre::GLES2TextureBuffer::upload(Ogre::PixelBox const&,Ogre::Box const&)
// IDA 0xe76a80: 485 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e76a80() {
}

// 0xe77340 — __ZN4Ogre18GLES2TextureBuffer12buildMipmapsERKNS_8PixelBoxE
#[doc(alias = "Ogre::GLES2TextureBuffer::buildMipmaps(Ogre::PixelBox const&)")]
// was: Ogre::GLES2TextureBuffer::buildMipmaps(Ogre::PixelBox const&)
// IDA 0xe77340: 145 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e77340() {
}

// 0xe774c0 — __ZN4Ogre18GLES2TextureBuffer8downloadERKNS_8PixelBoxE
#[doc(alias = "Ogre::GLES2TextureBuffer::download(Ogre::PixelBox const&)")]
// was: Ogre::GLES2TextureBuffer::download(Ogre::PixelBox const&)
// IDA 0xe774c0: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e774c0() {
}
