//! rendering — next 100 Ogre stubs (EA-sorted strict Ogre:: filter)
//! Filter: Ogre (9839 total, 6091 prior stubbed unique, +100 this batch) — 0xcb72c8..0xcbac1c after 0xcb6f8c
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xcb72c8 — __ZN4Ogre25HardwareBufferManagerBase20_releaseBufferCopiesEb
#[doc(alias = "Ogre::HardwareBufferManagerBase::_releaseBufferCopies(bool)")]
// was: Ogre::HardwareBufferManagerBase::_releaseBufferCopies(bool)
// IDA 0xcb72c8: 296 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb72c8() {
}

// 0xcb75ac — __ZN4Ogre25HardwareBufferManagerBase25_forceReleaseBufferCopiesERKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::HardwareBufferManagerBase::_forceReleaseBufferCopies(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: Ogre::HardwareBufferManagerBase::_forceReleaseBufferCopies(Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xcb75ac: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb75ac() {
}

// 0xcb75bc — __ZN4Ogre25HardwareBufferManagerBase25_forceReleaseBufferCopiesEPNS_20HardwareVertexBufferE
#[doc(alias = "Ogre::HardwareBufferManagerBase::_forceReleaseBufferCopies(Ogre::HardwareVertexBuffer *)")]
// was: Ogre::HardwareBufferManagerBase::_forceReleaseBufferCopies(Ogre::HardwareVertexBuffer *)
// IDA 0xcb75bc: 234 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb75bc() {
}

// 0xcb7810 — __ZN4Ogre25HardwareBufferManagerBase28_notifyVertexBufferDestroyedEPNS_20HardwareVertexBufferE
#[doc(alias = "Ogre::HardwareBufferManagerBase::_notifyVertexBufferDestroyed(Ogre::HardwareVertexBuffer *)")]
// was: Ogre::HardwareBufferManagerBase::_notifyVertexBufferDestroyed(Ogre::HardwareVertexBuffer *)
// IDA 0xcb7810: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7810() {
}

// 0xcb7868 — __ZN4Ogre25HardwareBufferManagerBase27_notifyIndexBufferDestroyedEPNS_19HardwareIndexBufferE
#[doc(alias = "Ogre::HardwareBufferManagerBase::_notifyIndexBufferDestroyed(Ogre::HardwareIndexBuffer *)")]
// was: Ogre::HardwareBufferManagerBase::_notifyIndexBufferDestroyed(Ogre::HardwareIndexBuffer *)
// IDA 0xcb7868: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7868() {
}

// 0xcb78b4 — __ZN4Ogre25HardwareBufferManagerBase14makeBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::HardwareBufferManagerBase::makeBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::HardwareBufferManagerBase::makeBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xcb78b4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb78b4() {
}

// 0xcb78e0 — __ZN4Ogre21TempBlendedBufferInfoD0Ev
#[doc(alias = "Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()")]
// was: Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()
// IDA 0xcb78e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb78e0() {
}

// 0xcb7970 — __ZN4Ogre21TempBlendedBufferInfoD1Ev
#[doc(alias = "Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()")]
// was: Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()
// IDA 0xcb7970: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb7970() {
}

// 0xcb797c — __ZN4Ogre21TempBlendedBufferInfoD2Ev
#[doc(alias = "Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()")]
// was: Ogre::TempBlendedBufferInfo::~TempBlendedBufferInfo()
// IDA 0xcb797c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb797c() {
}

// 0xcb7dc0 — __ZN4Ogre21TempBlendedBufferInfo11extractFromEPKNS_10VertexDataE
#[doc(alias = "Ogre::TempBlendedBufferInfo::extractFrom(Ogre::VertexData const*)")]
// was: Ogre::TempBlendedBufferInfo::extractFrom(Ogre::VertexData const*)
// IDA 0xcb7dc0: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7dc0() {
}

// 0xcb7e94 — __ZN4Ogre21TempBlendedBufferInfo18checkoutTempCopiesEbb
#[doc(alias = "Ogre::TempBlendedBufferInfo::checkoutTempCopies(bool,bool)")]
// was: Ogre::TempBlendedBufferInfo::checkoutTempCopies(bool,bool)
// IDA 0xcb7e94: 287 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb7e94() {
}

// 0xcb815c — __ZNK4Ogre21TempBlendedBufferInfo17buffersCheckedOutEbb
#[doc(alias = "Ogre::TempBlendedBufferInfo::buffersCheckedOut(bool,bool)const")]
// was: Ogre::TempBlendedBufferInfo::buffersCheckedOut(bool,bool)const
// IDA 0xcb815c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb815c() {
}

// 0xcb81ac — __ZN4Ogre21TempBlendedBufferInfo14bindTempCopiesEPNS_10VertexDataEb
#[doc(alias = "Ogre::TempBlendedBufferInfo::bindTempCopies(Ogre::VertexData *,bool)")]
// was: Ogre::TempBlendedBufferInfo::bindTempCopies(Ogre::VertexData *,bool)
// IDA 0xcb81ac: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb81ac() {
}

// 0xcb8214 — __ZN4Ogre21TempBlendedBufferInfo14licenseExpiredEPNS_14HardwareBufferE
#[doc(alias = "Ogre::TempBlendedBufferInfo::licenseExpired(Ogre::HardwareBuffer *)")]
// was: Ogre::TempBlendedBufferInfo::licenseExpired(Ogre::HardwareBuffer *)
// IDA 0xcb8214: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8214() {
}

// 0xcb8270 — __ZN4Ogre21HardwareBufferManager18createVertexBufferEmmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::HardwareBufferManager::createVertexBuffer(unsigned long,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::HardwareBufferManager::createVertexBuffer(unsigned long,unsigned long,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xcb8270: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8270() {
}

// 0xcb8290 — __ZN4Ogre21HardwareBufferManager17createIndexBufferENS_19HardwareIndexBuffer9IndexTypeEmNS_14HardwareBuffer5UsageEb
#[doc(alias = "Ogre::HardwareBufferManager::createIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)")]
// was: Ogre::HardwareBufferManager::createIndexBuffer(Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool)
// IDA 0xcb8290: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8290() {
}

// 0xcb82b0 — __ZN4Ogre21HardwareBufferManager26createRenderToVertexBufferEv
#[doc(alias = "Ogre::HardwareBufferManager::createRenderToVertexBuffer(void)")]
// was: Ogre::HardwareBufferManager::createRenderToVertexBuffer(void)
// IDA 0xcb82b0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82b0() {
}

// 0xcb82c0 — __ZN4Ogre21HardwareBufferManager23createVertexDeclarationEv
#[doc(alias = "Ogre::HardwareBufferManager::createVertexDeclaration(void)")]
// was: Ogre::HardwareBufferManager::createVertexDeclaration(void)
// IDA 0xcb82c0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82c0() {
}

// 0xcb82d0 — __ZN4Ogre21HardwareBufferManager24destroyVertexDeclarationEPNS_17VertexDeclarationE
#[doc(alias = "Ogre::HardwareBufferManager::destroyVertexDeclaration(Ogre::VertexDeclaration *)")]
// was: Ogre::HardwareBufferManager::destroyVertexDeclaration(Ogre::VertexDeclaration *)
// IDA 0xcb82d0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82d0() {
}

// 0xcb82e0 — __ZN4Ogre21HardwareBufferManager25createVertexBufferBindingEv
#[doc(alias = "Ogre::HardwareBufferManager::createVertexBufferBinding(void)")]
// was: Ogre::HardwareBufferManager::createVertexBufferBinding(void)
// IDA 0xcb82e0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82e0() {
}

// 0xcb82f0 — __ZN4Ogre21HardwareBufferManager26destroyVertexBufferBindingEPNS_19VertexBufferBindingE
#[doc(alias = "Ogre::HardwareBufferManager::destroyVertexBufferBinding(Ogre::VertexBufferBinding *)")]
// was: Ogre::HardwareBufferManager::destroyVertexBufferBinding(Ogre::VertexBufferBinding *)
// IDA 0xcb82f0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb82f0() {
}

// 0xcb8300 — __ZN4Ogre21HardwareBufferManager33registerVertexBufferSourceAndCopyERKNS_29HardwareVertexBufferSharedPtrES3_
#[doc(alias = "Ogre::HardwareBufferManager::registerVertexBufferSourceAndCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: Ogre::HardwareBufferManager::registerVertexBufferSourceAndCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xcb8300: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8300() {
}

// 0xcb8310 — __ZN4Ogre21HardwareBufferManager24allocateVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrENS_25HardwareBufferManagerBase17BufferLicenseTypeEPNS_22HardwareBufferLicenseeEb
#[doc(alias = "Ogre::HardwareBufferManager::allocateVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareBufferManagerBase::BufferLicenseType,Ogre::HardwareBufferLicensee *,bool)")]
// was: Ogre::HardwareBufferManager::allocateVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&,Ogre::HardwareBufferManagerBase::BufferLicenseType,Ogre::HardwareBufferLicensee *,bool)
// IDA 0xcb8310: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8310() {
}

// 0xcb8330 — __ZN4Ogre21HardwareBufferManager23releaseVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::HardwareBufferManager::releaseVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: Ogre::HardwareBufferManager::releaseVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xcb8330: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8330() {
}

// 0xcb8340 — __ZN4Ogre21HardwareBufferManager21touchVertexBufferCopyERKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::HardwareBufferManager::touchVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: Ogre::HardwareBufferManager::touchVertexBufferCopy(Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xcb8340: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8340() {
}

// 0xcb8350 — __ZN4Ogre21HardwareBufferManager23_freeUnusedBufferCopiesEv
#[doc(alias = "Ogre::HardwareBufferManager::_freeUnusedBufferCopies(void)")]
// was: Ogre::HardwareBufferManager::_freeUnusedBufferCopies(void)
// IDA 0xcb8350: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8350() {
}

// 0xcb8360 — __ZN4Ogre21HardwareBufferManager20_releaseBufferCopiesEb
#[doc(alias = "Ogre::HardwareBufferManager::_releaseBufferCopies(bool)")]
// was: Ogre::HardwareBufferManager::_releaseBufferCopies(bool)
// IDA 0xcb8360: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8360() {
}

// 0xcb8370 — __ZN4Ogre21HardwareBufferManager25_forceReleaseBufferCopiesERKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::HardwareBufferManager::_forceReleaseBufferCopies(Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: Ogre::HardwareBufferManager::_forceReleaseBufferCopies(Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xcb8370: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8370() {
}

// 0xcb8380 — __ZN4Ogre21HardwareBufferManager25_forceReleaseBufferCopiesEPNS_20HardwareVertexBufferE
#[doc(alias = "Ogre::HardwareBufferManager::_forceReleaseBufferCopies(Ogre::HardwareVertexBuffer *)")]
// was: Ogre::HardwareBufferManager::_forceReleaseBufferCopies(Ogre::HardwareVertexBuffer *)
// IDA 0xcb8380: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8380() {
}

// 0xcb8390 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS6_ESI_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::_Rb_tree_iterator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>)
// IDA 0xcb8390: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8390() {
}

// 0xcb83f8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xcb83f8: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb83f8() {
}

// 0xcb84f4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xcb84f4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb84f4() {
}

// 0xcb851c — __ZNSt10_List_baseIN4Ogre29HardwareVertexBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::HardwareVertexBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwareVertexBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::HardwareVertexBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwareVertexBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xcb851c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb851c() {
}

// 0xcb8520 — __ZNSt10_List_baseIN4Ogre29HardwareVertexBufferSharedPtrENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::HardwareVertexBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwareVertexBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::HardwareVertexBufferSharedPtr,Ogre::STLAllocator<Ogre::HardwareVertexBufferSharedPtr,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xcb8520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8520() {
}

// 0xcb852c — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)
// IDA 0xcb852c: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb852c() {
}

// 0xcb8628 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)
// IDA 0xcb8628: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8628() {
}

// 0xcb8694 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKS7_
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense> const&)
// IDA 0xcb8694: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8694() {
}

// 0xcb874c — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexBufferBinding *> *)")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexBufferBinding *> *)
// IDA 0xcb874c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb874c() {
}

// 0xcb8774 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexDeclaration *> *)")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::VertexDeclaration *> *)
// IDA 0xcb8774: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8774() {
}

// 0xcb879c — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>,std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>)")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>,std::_Rb_tree_iterator<Ogre::VertexBufferBinding *>)
// IDA 0xcb879c: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb879c() {
}

// 0xcb8800 — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexBufferBinding * const&)")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexBufferBinding * const&)
// IDA 0xcb8800: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8800() {
}

// 0xcb88f8 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexDeclaration *>,std::_Rb_tree_iterator<Ogre::VertexDeclaration *>)")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::VertexDeclaration *>,std::_Rb_tree_iterator<Ogre::VertexDeclaration *>)
// IDA 0xcb88f8: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb88f8() {
}

// 0xcb895c — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexDeclaration * const&)")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::VertexDeclaration * const&)
// IDA 0xcb895c: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb895c() {
}

// 0xcb8a54 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareIndexBuffer *> *)")]
// was: std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareIndexBuffer *> *)
// IDA 0xcb8a54: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8a54() {
}

// 0xcb8a7c — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareVertexBuffer *> *)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::HardwareVertexBuffer *> *)
// IDA 0xcb8a7c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8a7c() {
}

// 0xcb8aa4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8aa4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8aa4() {
}

// 0xcb8aa8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISB_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8aa8() {
}

// 0xcb8ab4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ab4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ab4() {
}

// 0xcb8ab8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_29HardwareVertexBufferSharedPtrEESt10_Select1stIS6_ESt4lessIS2_ENS0_12STLAllocatorIS6_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ab8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ab8() {
}

// 0xcb8ac4 — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexBufferBinding *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexBufferBinding *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ac4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ac4() {
}

// 0xcb8ac8 — __ZNSt8_Rb_treeIPN4Ogre19VertexBufferBindingES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexBufferBinding *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::VertexBufferBinding *,Ogre::VertexBufferBinding *,std::_Identity<Ogre::VertexBufferBinding *>,std::less<Ogre::VertexBufferBinding *>,Ogre::STLAllocator<Ogre::VertexBufferBinding *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexBufferBinding *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ac8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ac8() {
}

// 0xcb8ad4 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexDeclaration *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexDeclaration *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ad4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ad4() {
}

// 0xcb8ad8 — __ZNSt8_Rb_treeIPN4Ogre17VertexDeclarationES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexDeclaration *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::VertexDeclaration *,Ogre::VertexDeclaration *,std::_Identity<Ogre::VertexDeclaration *>,std::less<Ogre::VertexDeclaration *>,Ogre::STLAllocator<Ogre::VertexDeclaration *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::VertexDeclaration *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ad8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ad8() {
}

// 0xcb8ae4 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareIndexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareIndexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ae4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8ae4() {
}

// 0xcb8ae8 — __ZNSt8_Rb_treeIPN4Ogre19HardwareIndexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareIndexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareIndexBuffer *,Ogre::HardwareIndexBuffer *,std::_Identity<Ogre::HardwareIndexBuffer *>,std::less<Ogre::HardwareIndexBuffer *>,Ogre::STLAllocator<Ogre::HardwareIndexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareIndexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8ae8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ae8() {
}

// 0xcb8af4 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8af4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb8af4() {
}

// 0xcb8af8 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,Ogre::HardwareVertexBuffer *,std::_Identity<Ogre::HardwareVertexBuffer *>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<Ogre::HardwareVertexBuffer *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<Ogre::HardwareVertexBuffer *>,false>::~_Rb_tree_impl()
// IDA 0xcb8af8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8af8() {
}

// 0xcb8b04 — __ZNSt8_Rb_treeIPN4Ogre20HardwareVertexBufferESt4pairIKS2_NS0_25HardwareBufferManagerBase19VertexBufferLicenseEESt10_Select1stIS7_ESt4lessIS2_ENS0_12STLAllocatorIS7_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)")]
// was: std::_Rb_tree<Ogre::HardwareVertexBuffer *,std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,std::_Select1st<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>>,std::less<Ogre::HardwareVertexBuffer *>,Ogre::STLAllocator<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<Ogre::HardwareVertexBuffer * const,Ogre::HardwareBufferManagerBase::VertexBufferLicense>> *)
// IDA 0xcb8b04: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8b04() {
}

// 0xcb8b60 — __ZN4Ogre19HardwareIndexBufferC2EPNS_25HardwareBufferManagerBaseENS0_9IndexTypeEmNS_14HardwareBuffer5UsageEbb
#[doc(alias = "Ogre::HardwareIndexBuffer::HardwareIndexBuffer(Ogre::HardwareBufferManagerBase *,Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: Ogre::HardwareIndexBuffer::HardwareIndexBuffer(Ogre::HardwareBufferManagerBase *,Ogre::HardwareIndexBuffer::IndexType,unsigned long,Ogre::HardwareBuffer::Usage,bool,bool)
// IDA 0xcb8b60: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8b60() {
}

// 0xcb8ccc — __ZN4Ogre19HardwareIndexBufferD0Ev
#[doc(alias = "Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()")]
// was: Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()
// IDA 0xcb8ccc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8ccc() {
}

// 0xcb8d88 — __ZN4Ogre19HardwareIndexBufferD1Ev
#[doc(alias = "Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()")]
// was: Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()
// IDA 0xcb8d88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8d88() {
}

// 0xcb8e38 — __ZN4Ogre19HardwareIndexBufferD2Ev
#[doc(alias = "Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()")]
// was: Ogre::HardwareIndexBuffer::~HardwareIndexBuffer()
// IDA 0xcb8e38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8e38() {
}

// 0xcb8ee8 — __ZN4Ogre28HardwareIndexBufferSharedPtrC1EPNS_19HardwareIndexBufferE
#[doc(alias = "Ogre::HardwareIndexBufferSharedPtr::HardwareIndexBufferSharedPtr(Ogre::HardwareIndexBuffer *)")]
// was: Ogre::HardwareIndexBufferSharedPtr::HardwareIndexBufferSharedPtr(Ogre::HardwareIndexBuffer *)
// IDA 0xcb8ee8: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8ee8() {
}

// 0xcb8f70 — __ZN4Ogre22HardwareOcclusionQueryC2Ev
#[doc(alias = "Ogre::HardwareOcclusionQuery::HardwareOcclusionQuery(void)")]
// was: Ogre::HardwareOcclusionQuery::HardwareOcclusionQuery(void)
// IDA 0xcb8f70: 8 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb8f70() {
}

// 0xcb8f88 — __ZN4Ogre22HardwareOcclusionQueryD0Ev
#[doc(alias = "Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()")]
// was: Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()
// IDA 0xcb8f88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb8f88() {
}

// 0xcb9014 — __ZN4Ogre22HardwareOcclusionQueryD1Ev
#[doc(alias = "Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()")]
// was: Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()
// IDA 0xcb9014: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb9014() {
}

// 0xcb9018 — __ZN4Ogre22HardwareOcclusionQueryD2Ev
#[doc(alias = "Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()")]
// was: Ogre::HardwareOcclusionQuery::~HardwareOcclusionQuery()
// IDA 0xcb9018: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb9018() {
}

// 0xcb9050 — __ZN4Ogre19HardwarePixelBufferC2EmmmNS_11PixelFormatENS_14HardwareBuffer5UsageEbb
#[doc(alias = "Ogre::HardwarePixelBuffer::HardwarePixelBuffer(unsigned long,unsigned long,unsigned long,Ogre::PixelFormat,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: Ogre::HardwarePixelBuffer::HardwarePixelBuffer(unsigned long,unsigned long,unsigned long,Ogre::PixelFormat,Ogre::HardwareBuffer::Usage,bool,bool)
// IDA 0xcb9050: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9050() {
}

// 0xcb90f4 — __ZN4Ogre19HardwarePixelBufferD0Ev
#[doc(alias = "Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()")]
// was: Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()
// IDA 0xcb90f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb90f4() {
}

// 0xcb9190 — __ZN4Ogre19HardwarePixelBufferD1Ev
#[doc(alias = "Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()")]
// was: Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()
// IDA 0xcb9190: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb9190() {
}

// 0xcb91a0 — __ZN4Ogre19HardwarePixelBufferD2Ev
#[doc(alias = "Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()")]
// was: Ogre::HardwarePixelBuffer::~HardwarePixelBuffer()
// IDA 0xcb91a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb91a0() {
}

// 0xcb91b0 — __ZN4Ogre19HardwarePixelBuffer4lockEmmNS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::HardwarePixelBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: Ogre::HardwarePixelBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xcb91b0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb91b0() {
}

// 0xcb91e4 — __ZN4Ogre19HardwarePixelBuffer4lockERKNS_3BoxENS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::HardwarePixelBuffer::lock(Ogre::Box const&,Ogre::HardwareBuffer::LockOptions)")]
// was: Ogre::HardwarePixelBuffer::lock(Ogre::Box const&,Ogre::HardwareBuffer::LockOptions)
// IDA 0xcb91e4: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb91e4() {
}

// 0xcb9280 — __ZN4Ogre19HardwarePixelBuffer14getCurrentLockEv
#[doc(alias = "Ogre::HardwarePixelBuffer::getCurrentLock(void)")]
// was: Ogre::HardwarePixelBuffer::getCurrentLock(void)
// IDA 0xcb9280: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9280() {
}

// 0xcb9284 — __ZN4Ogre19HardwarePixelBuffer8lockImplEmmNS_14HardwareBuffer11LockOptionsE
#[doc(alias = "Ogre::HardwarePixelBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: Ogre::HardwarePixelBuffer::lockImpl(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xcb9284: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9284() {
}

// 0xcb9434 — __ZN4Ogre19HardwarePixelBuffer4blitERKNS_28HardwarePixelBufferSharedPtrERKNS_3BoxES6_
#[doc(alias = "Ogre::HardwarePixelBuffer::blit(Ogre::HardwarePixelBufferSharedPtr const&,Ogre::Box const&,Ogre::Box const&)")]
// was: Ogre::HardwarePixelBuffer::blit(Ogre::HardwarePixelBufferSharedPtr const&,Ogre::Box const&,Ogre::Box const&)
// IDA 0xcb9434: 374 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9434() {
}

// 0xcb983c — __ZN4Ogre19HardwarePixelBuffer4blitERKNS_28HardwarePixelBufferSharedPtrE
#[doc(alias = "Ogre::HardwarePixelBuffer::blit(Ogre::HardwarePixelBufferSharedPtr const&)")]
// was: Ogre::HardwarePixelBuffer::blit(Ogre::HardwarePixelBufferSharedPtr const&)
// IDA 0xcb983c: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb983c() {
}

// 0xcb9888 — __ZN4Ogre19HardwarePixelBuffer8readDataEmmPv
#[doc(alias = "Ogre::HardwarePixelBuffer::readData(unsigned long,unsigned long,void *)")]
// was: Ogre::HardwarePixelBuffer::readData(unsigned long,unsigned long,void *)
// IDA 0xcb9888: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9888() {
}

// 0xcb9a38 — __ZN4Ogre19HardwarePixelBuffer9writeDataEmmPKvb
#[doc(alias = "Ogre::HardwarePixelBuffer::writeData(unsigned long,unsigned long,void const*,bool)")]
// was: Ogre::HardwarePixelBuffer::writeData(unsigned long,unsigned long,void const*,bool)
// IDA 0xcb9a38: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9a38() {
}

// 0xcb9be8 — __ZN4Ogre19HardwarePixelBuffer15getRenderTargetEm
#[doc(alias = "Ogre::HardwarePixelBuffer::getRenderTarget(unsigned long)")]
// was: Ogre::HardwarePixelBuffer::getRenderTarget(unsigned long)
// IDA 0xcb9be8: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9be8() {
}

// 0xcb9d98 — __ZN4Ogre28HardwarePixelBufferSharedPtrC1EPNS_19HardwarePixelBufferE
#[doc(alias = "Ogre::HardwarePixelBufferSharedPtr::HardwarePixelBufferSharedPtr(Ogre::HardwarePixelBuffer *)")]
// was: Ogre::HardwarePixelBufferSharedPtr::HardwarePixelBufferSharedPtr(Ogre::HardwarePixelBuffer *)
// IDA 0xcb9d98: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb9d98() {
}

// 0xcb9dec — __ZN4Ogre19HardwarePixelBuffer14_clearSliceRTTEm
#[doc(alias = "Ogre::HardwarePixelBuffer::_clearSliceRTT(unsigned long)")]
// was: Ogre::HardwarePixelBuffer::_clearSliceRTT(unsigned long)
// IDA 0xcb9dec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cb9dec() {
}

// 0xcb9df0 — __ZN4Ogre28HardwarePixelBufferSharedPtrD1Ev
#[doc(alias = "Ogre::HardwarePixelBufferSharedPtr::~HardwarePixelBufferSharedPtr()")]
// was: Ogre::HardwarePixelBufferSharedPtr::~HardwarePixelBufferSharedPtr()
// IDA 0xcb9df0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb9df0() {
}

// 0xcb9ee0 — __ZN4Ogre28HardwarePixelBufferSharedPtrD0Ev
#[doc(alias = "Ogre::HardwarePixelBufferSharedPtr::~HardwarePixelBufferSharedPtr()")]
// was: Ogre::HardwarePixelBufferSharedPtr::~HardwarePixelBufferSharedPtr()
// IDA 0xcb9ee0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cb9ee0() {
}

// 0xcba008 — __ZN4Ogre20HardwareVertexBufferC2EPNS_25HardwareBufferManagerBaseEmmNS_14HardwareBuffer5UsageEbb
#[doc(alias = "Ogre::HardwareVertexBuffer::HardwareVertexBuffer(Ogre::HardwareBufferManagerBase *,unsigned long,unsigned long,Ogre::HardwareBuffer::Usage,bool,bool)")]
// was: Ogre::HardwareVertexBuffer::HardwareVertexBuffer(Ogre::HardwareBufferManagerBase *,unsigned long,unsigned long,Ogre::HardwareBuffer::Usage,bool,bool)
// IDA 0xcba008: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba008() {
}

// 0xcba168 — __ZN4Ogre20HardwareVertexBufferD0Ev
#[doc(alias = "Ogre::HardwareVertexBuffer::~HardwareVertexBuffer()")]
// was: Ogre::HardwareVertexBuffer::~HardwareVertexBuffer()
// IDA 0xcba168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cba168() {
}

// 0xcba224 — __ZN4Ogre20HardwareVertexBufferD1Ev
#[doc(alias = "Ogre::HardwareVertexBuffer::~HardwareVertexBuffer()")]
// was: Ogre::HardwareVertexBuffer::~HardwareVertexBuffer()
// IDA 0xcba224: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cba224() {
}

// 0xcba2d4 — __ZN4Ogre20HardwareVertexBufferD2Ev
#[doc(alias = "Ogre::HardwareVertexBuffer::~HardwareVertexBuffer()")]
// was: Ogre::HardwareVertexBuffer::~HardwareVertexBuffer()
// IDA 0xcba2d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cba2d4() {
}

// 0xcba384 — __ZN4Ogre20HardwareVertexBuffer36checkIfVertexInstanceDataIsSupportedEv
#[doc(alias = "Ogre::HardwareVertexBuffer::checkIfVertexInstanceDataIsSupported(void)")]
// was: Ogre::HardwareVertexBuffer::checkIfVertexInstanceDataIsSupported(void)
// IDA 0xcba384: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba384() {
}

// 0xcba3a0 — __ZN4Ogre20HardwareVertexBuffer17setIsInstanceDataEb
#[doc(alias = "Ogre::HardwareVertexBuffer::setIsInstanceData(bool)")]
// was: Ogre::HardwareVertexBuffer::setIsInstanceData(bool)
// IDA 0xcba3a0: 164 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba3a0() {
}

// 0xcba588 — __ZN4Ogre20HardwareVertexBuffer23setInstanceDataStepRateEm
#[doc(alias = "Ogre::HardwareVertexBuffer::setInstanceDataStepRate(unsigned long)")]
// was: Ogre::HardwareVertexBuffer::setInstanceDataStepRate(unsigned long)
// IDA 0xcba588: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba588() {
}

// 0xcba75c — __ZNK4Ogre13VertexElement7getSizeEv
#[doc(alias = "Ogre::VertexElement::getSize(void)const")]
// was: Ogre::VertexElement::getSize(void)const
// IDA 0xcba75c: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba75c() {
}

// 0xcba778 — __ZN4Ogre13VertexElement11getTypeSizeENS_17VertexElementTypeE
#[doc(alias = "Ogre::VertexElement::getTypeSize(Ogre::VertexElementType)")]
// was: Ogre::VertexElement::getTypeSize(Ogre::VertexElementType)
// IDA 0xcba778: 8 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba778() {
}

// 0xcba790 — __ZN4Ogre13VertexElement12getTypeCountENS_17VertexElementTypeE
#[doc(alias = "Ogre::VertexElement::getTypeCount(Ogre::VertexElementType)")]
// was: Ogre::VertexElement::getTypeCount(Ogre::VertexElementType)
// IDA 0xcba790: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba790() {
}

// 0xcba990 — __ZN4Ogre13VertexElement17multiplyTypeCountENS_17VertexElementTypeEt
#[doc(alias = "Ogre::VertexElement::multiplyTypeCount(Ogre::VertexElementType,unsigned short)")]
// was: Ogre::VertexElement::multiplyTypeCount(Ogre::VertexElementType,unsigned short)
// IDA 0xcba990: 179 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cba990() {
}

// 0xcbaba0 — __ZN4Ogre13VertexElement30getBestColourVertexElementTypeEv
#[doc(alias = "Ogre::VertexElement::getBestColourVertexElementType(void)")]
// was: Ogre::VertexElement::getBestColourVertexElementType(void)
// IDA 0xcbaba0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbaba0() {
}

// 0xcbabd4 — __ZN4Ogre13VertexElement18convertColourValueENS_17VertexElementTypeES1_Pj
#[doc(alias = "Ogre::VertexElement::convertColourValue(Ogre::VertexElementType,Ogre::VertexElementType,unsigned int *)")]
// was: Ogre::VertexElement::convertColourValue(Ogre::VertexElementType,Ogre::VertexElementType,unsigned int *)
// IDA 0xcbabd4: 10 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbabd4() {
}

// 0xcbabf0 — __ZN4Ogre13VertexElement18convertColourValueERKNS_11ColourValueENS_17VertexElementTypeE
#[doc(alias = "Ogre::VertexElement::convertColourValue(Ogre::ColourValue const&,Ogre::VertexElementType)")]
// was: Ogre::VertexElement::convertColourValue(Ogre::ColourValue const&,Ogre::VertexElementType)
// IDA 0xcbabf0: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbabf0() {
}

// 0xcbac04 — __ZN4Ogre13VertexElement11getBaseTypeENS_17VertexElementTypeE
#[doc(alias = "Ogre::VertexElement::getBaseType(Ogre::VertexElementType)")]
// was: Ogre::VertexElement::getBaseType(Ogre::VertexElementType)
// IDA 0xcbac04: 8 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbac04() {
}

// 0xcbac1c — __ZN4Ogre17VertexDeclarationC1Ev
#[doc(alias = "Ogre::VertexDeclaration::VertexDeclaration(void)")]
// was: Ogre::VertexDeclaration::VertexDeclaration(void)
// IDA 0xcbac1c: 16 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbac1c() {
}
