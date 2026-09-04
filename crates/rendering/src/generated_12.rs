//! rendering — next 100 Ogre stubs (EA-sorted strict Ogre:: filter)
//! Filter: Ogre (9839 total, 2373 prior strict Ogre stubbed, +100 this batch) — 0xcbac4c..0xcbee34 after 0xcbac1c (low-EA remaining 7466 before batch, 7366 after; task note 3648 was stale — accurate strict Ogre remaining 7466)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xcbac4c — __ZN4Ogre17VertexDeclarationD0Ev
#[doc(alias = "Ogre::VertexDeclaration::~VertexDeclaration()")]
// was: Ogre::VertexDeclaration::~VertexDeclaration()
// IDA 0xcbac4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbac4c() {
}

// 0xcbad0c — __ZN4Ogre17VertexDeclarationD1Ev
#[doc(alias = "Ogre::VertexDeclaration::~VertexDeclaration()")]
// was: Ogre::VertexDeclaration::~VertexDeclaration()
// IDA 0xcbad0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbad0c() {
}

// 0xcbadc4 — __ZNK4Ogre17VertexDeclaration11getElementsEv
#[doc(alias = "Ogre::VertexDeclaration::getElements(void)const")]
// was: Ogre::VertexDeclaration::getElements(void)const
// IDA 0xcbadc4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbadc4() {
}

// 0xcbadc8 — __ZN4Ogre17VertexDeclaration10addElementEtmNS_17VertexElementTypeENS_21VertexElementSemanticEt
#[doc(alias = "Ogre::VertexDeclaration::addElement(unsigned short,unsigned long,Ogre::VertexElementType,Ogre::VertexElementSemantic,unsigned short)")]
// was: Ogre::VertexDeclaration::addElement(unsigned short,unsigned long,Ogre::VertexElementType,Ogre::VertexElementSemantic,unsigned short)
// IDA 0xcbadc8: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbadc8() {
}

// 0xcbae38 — __ZN4Ogre17VertexDeclaration13insertElementEttmNS_17VertexElementTypeENS_21VertexElementSemanticEt
#[doc(alias = "Ogre::VertexDeclaration::insertElement(unsigned short,unsigned short,unsigned long,Ogre::VertexElementType,Ogre::VertexElementSemantic,unsigned short)")]
// was: Ogre::VertexDeclaration::insertElement(unsigned short,unsigned short,unsigned long,Ogre::VertexElementType,Ogre::VertexElementSemantic,unsigned short)
// IDA 0xcbae38: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbae38() {
}

// 0xcbaebc — __ZNK4Ogre17VertexDeclaration10getElementEt
#[doc(alias = "Ogre::VertexDeclaration::getElement(unsigned short)const")]
// was: Ogre::VertexDeclaration::getElement(unsigned short)const
// IDA 0xcbaebc: 10 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbaebc() {
}

// 0xcbaed0 — __ZN4Ogre17VertexDeclaration13removeElementEt
#[doc(alias = "Ogre::VertexDeclaration::removeElement(unsigned short)")]
// was: Ogre::VertexDeclaration::removeElement(unsigned short)
// IDA 0xcbaed0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbaed0() {
}

// 0xcbaef4 — __ZN4Ogre17VertexDeclaration13removeElementENS_21VertexElementSemanticEt
#[doc(alias = "Ogre::VertexDeclaration::removeElement(Ogre::VertexElementSemantic,unsigned short)")]
// was: Ogre::VertexDeclaration::removeElement(Ogre::VertexElementSemantic,unsigned short)
// IDA 0xcbaef4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbaef4() {
}

// 0xcbaf20 — __ZN4Ogre17VertexDeclaration17removeAllElementsEv
#[doc(alias = "Ogre::VertexDeclaration::removeAllElements(void)")]
// was: Ogre::VertexDeclaration::removeAllElements(void)
// IDA 0xcbaf20: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbaf20() {
}

// 0xcbaf44 — __ZN4Ogre17VertexDeclaration13modifyElementEttmNS_17VertexElementTypeENS_21VertexElementSemanticEt
#[doc(alias = "Ogre::VertexDeclaration::modifyElement(unsigned short,unsigned short,unsigned long,Ogre::VertexElementType,Ogre::VertexElementSemantic,unsigned short)")]
// was: Ogre::VertexDeclaration::modifyElement(unsigned short,unsigned short,unsigned long,Ogre::VertexElementType,Ogre::VertexElementSemantic,unsigned short)
// IDA 0xcbaf44: 19 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbaf44() {
}

// 0xcbaf74 — __ZNK4Ogre17VertexDeclaration21findElementBySemanticENS_21VertexElementSemanticEt
#[doc(alias = "Ogre::VertexDeclaration::findElementBySemantic(Ogre::VertexElementSemantic,unsigned short)const")]
// was: Ogre::VertexDeclaration::findElementBySemantic(Ogre::VertexElementSemantic,unsigned short)const
// IDA 0xcbaf74: 18 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbaf74() {
}

// 0xcbafa8 — __ZNK4Ogre17VertexDeclaration20findElementsBySourceEt
#[doc(alias = "Ogre::VertexDeclaration::findElementsBySource(unsigned short)const")]
// was: Ogre::VertexDeclaration::findElementsBySource(unsigned short)const
// IDA 0xcbafa8: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbafa8() {
}

// 0xcbb0d4 — __ZNK4Ogre17VertexDeclaration13getVertexSizeEt
#[doc(alias = "Ogre::VertexDeclaration::getVertexSize(unsigned short)const")]
// was: Ogre::VertexDeclaration::getVertexSize(unsigned short)const
// IDA 0xcbb0d4: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb0d4() {
}

// 0xcbb110 — __ZNK4Ogre17VertexDeclaration5cloneEPNS_25HardwareBufferManagerBaseE
#[doc(alias = "Ogre::VertexDeclaration::clone(Ogre::HardwareBufferManagerBase *)const")]
// was: Ogre::VertexDeclaration::clone(Ogre::HardwareBufferManagerBase *)const
// IDA 0xcbb110: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb110() {
}

// 0xcbb160 — __ZN4Ogre17VertexDeclaration17vertexElementLessERKNS_13VertexElementES3_
#[doc(alias = "Ogre::VertexDeclaration::vertexElementLess(Ogre::VertexElement const&,Ogre::VertexElement const&)")]
// was: Ogre::VertexDeclaration::vertexElementLess(Ogre::VertexElement const&,Ogre::VertexElement const&)
// IDA 0xcbb160: 22 insns (LDRH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb160() {
}

// 0xcbb190 — __ZN4Ogre17VertexDeclaration17closeGapsInSourceEv
#[doc(alias = "Ogre::VertexDeclaration::closeGapsInSource(void)")]
// was: Ogre::VertexDeclaration::closeGapsInSource(void)
// IDA 0xcbb190: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb190() {
}

// 0xcbb218 — __ZNK4Ogre17VertexDeclaration27getAutoOrganisedDeclarationEbbb
#[doc(alias = "Ogre::VertexDeclaration::getAutoOrganisedDeclaration(bool,bool,bool)const")]
// was: Ogre::VertexDeclaration::getAutoOrganisedDeclaration(bool,bool,bool)const
// IDA 0xcbb218: 132 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb218() {
}

// 0xcbb36c — __ZNK4Ogre17VertexDeclaration12getMaxSourceEv
#[doc(alias = "Ogre::VertexDeclaration::getMaxSource(void)const")]
// was: Ogre::VertexDeclaration::getMaxSource(void)const
// IDA 0xcbb36c: 14 insns (LDR.W..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb36c() {
}

// 0xcbb38c — __ZNK4Ogre17VertexDeclaration28getNextFreeTextureCoordinateEv
#[doc(alias = "Ogre::VertexDeclaration::getNextFreeTextureCoordinate(void)const")]
// was: Ogre::VertexDeclaration::getNextFreeTextureCoordinate(void)const
// IDA 0xcbb38c: 14 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb38c() {
}

// 0xcbb3ac — __ZN4Ogre19VertexBufferBindingC1Ev
#[doc(alias = "Ogre::VertexBufferBinding::VertexBufferBinding(void)")]
// was: Ogre::VertexBufferBinding::VertexBufferBinding(void)
// IDA 0xcbb3ac: 20 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb3ac() {
}

// 0xcbb3e4 — __ZN4Ogre19VertexBufferBindingD0Ev
#[doc(alias = "Ogre::VertexBufferBinding::~VertexBufferBinding()")]
// was: Ogre::VertexBufferBinding::~VertexBufferBinding()
// IDA 0xcbb3e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbb3e4() {
}

// 0xcbb4d4 — __ZN4Ogre19VertexBufferBindingD1Ev
#[doc(alias = "Ogre::VertexBufferBinding::~VertexBufferBinding()")]
// was: Ogre::VertexBufferBinding::~VertexBufferBinding()
// IDA 0xcbb4d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbb4d4() {
}

// 0xcbb5b8 — __ZN4Ogre19VertexBufferBinding10setBindingEtRKNS_29HardwareVertexBufferSharedPtrE
#[doc(alias = "Ogre::VertexBufferBinding::setBinding(unsigned short,Ogre::HardwareVertexBufferSharedPtr const&)")]
// was: Ogre::VertexBufferBinding::setBinding(unsigned short,Ogre::HardwareVertexBufferSharedPtr const&)
// IDA 0xcbb5b8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb5b8() {
}

// 0xcbb5ec — __ZN4Ogre19VertexBufferBinding12unsetBindingEt
#[doc(alias = "Ogre::VertexBufferBinding::unsetBinding(unsigned short)")]
// was: Ogre::VertexBufferBinding::unsetBinding(unsigned short)
// IDA 0xcbb5ec: 256 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb5ec() {
}

// 0xcbb8d8 — __ZN4Ogre19VertexBufferBinding16unsetAllBindingsEv
#[doc(alias = "Ogre::VertexBufferBinding::unsetAllBindings(void)")]
// was: Ogre::VertexBufferBinding::unsetAllBindings(void)
// IDA 0xcbb8d8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb8d8() {
}

// 0xcbb8f8 — __ZNK4Ogre19VertexBufferBinding11getBindingsEv
#[doc(alias = "Ogre::VertexBufferBinding::getBindings(void)const")]
// was: Ogre::VertexBufferBinding::getBindings(void)const
// IDA 0xcbb8f8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb8f8() {
}

// 0xcbb8fc — __ZNK4Ogre19VertexBufferBinding9getBufferEt
#[doc(alias = "Ogre::VertexBufferBinding::getBuffer(unsigned short)const")]
// was: Ogre::VertexBufferBinding::getBuffer(unsigned short)const
// IDA 0xcbb8fc: 175 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbb8fc() {
}

// 0xcbbb00 — __ZNK4Ogre19VertexBufferBinding13isBufferBoundEt
#[doc(alias = "Ogre::VertexBufferBinding::isBufferBound(unsigned short)const")]
// was: Ogre::VertexBufferBinding::isBufferBound(unsigned short)const
// IDA 0xcbbb00: 26 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbbb00() {
}

// 0xcbbb40 — __ZNK4Ogre19VertexBufferBinding17getLastBoundIndexEv
#[doc(alias = "Ogre::VertexBufferBinding::getLastBoundIndex(void)const")]
// was: Ogre::VertexBufferBinding::getLastBoundIndex(void)const
// IDA 0xcbbb40: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbbb40() {
}

// 0xcbbb58 — __ZNK4Ogre19VertexBufferBinding7hasGapsEv
#[doc(alias = "Ogre::VertexBufferBinding::hasGaps(void)const")]
// was: Ogre::VertexBufferBinding::hasGaps(void)const
// IDA 0xcbbb58: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbbb58() {
}

// 0xcbbb7c — __ZN4Ogre19VertexBufferBinding9closeGapsERSt3mapIttSt4lessItENS_12STLAllocatorISt4pairIKttENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::VertexBufferBinding::closeGaps(std::map<unsigned short,unsigned short,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)")]
// was: Ogre::VertexBufferBinding::closeGaps(std::map<unsigned short,unsigned short,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> &)
// IDA 0xcbbb7c: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbbb7c() {
}

// 0xcbbd88 — __ZNK4Ogre19VertexBufferBinding18getHasInstanceDataEv
#[doc(alias = "Ogre::VertexBufferBinding::getHasInstanceData(void)const")]
// was: Ogre::VertexBufferBinding::getHasInstanceData(void)const
// IDA 0xcbbd88: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbbd88() {
}

// 0xcbbdb0 — __ZN4Ogre29HardwareVertexBufferSharedPtrC1EPNS_20HardwareVertexBufferE
#[doc(alias = "Ogre::HardwareVertexBufferSharedPtr::HardwareVertexBufferSharedPtr(Ogre::HardwareVertexBuffer *)")]
// was: Ogre::HardwareVertexBufferSharedPtr::HardwareVertexBufferSharedPtr(Ogre::HardwareVertexBuffer *)
// IDA 0xcbbdb0: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbbdb0() {
}

// 0xcbbe04 — __ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE4sortIPFbRKS1_SA_EEEvT_
#[doc(alias = "void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::sort<bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&)>(bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&))")]
// was: void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::sort<bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&)>(bool (*)(Ogre::VertexElement const&,Ogre::VertexElement const&))
// IDA 0xcbbe04: 356 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbbe04() {
}

// 0xcbc14c — __ZNSt3mapItN4Ogre29HardwareVertexBufferSharedPtrESt4lessItENS0_12STLAllocatorISt4pairIKtS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(alias = "std::map<unsigned short,Ogre::HardwareVertexBufferSharedPtr,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)")]
// was: std::map<unsigned short,Ogre::HardwareVertexBufferSharedPtr,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](unsigned short const&)
// IDA 0xcbc14c: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc14c() {
}

// 0xcbc318 — __ZN4Ogre14HardwareBuffer4lockEmmNS0_11LockOptionsE
#[doc(alias = "Ogre::HardwareBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)")]
// was: Ogre::HardwareBuffer::lock(unsigned long,unsigned long,Ogre::HardwareBuffer::LockOptions)
// IDA 0xcbc318: 191 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc318() {
}

// 0xcbc53c — __ZN4Ogre14HardwareBuffer6unlockEv
#[doc(alias = "Ogre::HardwareBuffer::unlock(void)")]
// was: Ogre::HardwareBuffer::unlock(void)
// IDA 0xcbc53c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc53c() {
}

// 0xcbc57c — __ZN4Ogre14HardwareBuffer8copyDataERS0_mmmb
#[doc(alias = "Ogre::HardwareBuffer::copyData(Ogre::HardwareBuffer&,unsigned long,unsigned long,unsigned long,bool)")]
// was: Ogre::HardwareBuffer::copyData(Ogre::HardwareBuffer&,unsigned long,unsigned long,unsigned long,bool)
// IDA 0xcbc57c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc57c() {
}

// 0xcbc5c0 — __ZN4Ogre14HardwareBuffer8copyDataERS0_
#[doc(alias = "Ogre::HardwareBuffer::copyData(Ogre::HardwareBuffer&)")]
// was: Ogre::HardwareBuffer::copyData(Ogre::HardwareBuffer&)
// IDA 0xcbc5c0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc5c0() {
}

// 0xcbc5e8 — __ZN4Ogre14HardwareBuffer17_updateFromShadowEv
#[doc(alias = "Ogre::HardwareBuffer::_updateFromShadow(void)")]
// was: Ogre::HardwareBuffer::_updateFromShadow(void)
// IDA 0xcbc5e8: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc5e8() {
}

// 0xcbc664 — __ZNK4Ogre19VertexBufferBinding14getBufferCountEv
#[doc(alias = "Ogre::VertexBufferBinding::getBufferCount(void)const")]
// was: Ogre::VertexBufferBinding::getBufferCount(void)const
// IDA 0xcbc664: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc664() {
}

// 0xcbc668 — __ZNK4Ogre19VertexBufferBinding12getNextIndexEv
#[doc(alias = "Ogre::VertexBufferBinding::getNextIndex(void)const")]
// was: Ogre::VertexBufferBinding::getNextIndex(void)const
// IDA 0xcbc668: 5 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc668() {
}

// 0xcbc674 — __ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,unsigned short>>,std::pair<unsigned short const,unsigned short> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,unsigned short>>,std::pair<unsigned short const,unsigned short> const&)
// IDA 0xcbc674: 208 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc674() {
}

// 0xcbc86c — __ZNSt8_Rb_treeItSt4pairIKttESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,unsigned short> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,unsigned short>,std::_Select1st<std::pair<unsigned short const,unsigned short>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,unsigned short>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,unsigned short> const&)
// IDA 0xcbc86c: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc86c() {
}

// 0xcbc964 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xcbc964: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc964() {
}

// 0xcbc98c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>> *)
// IDA 0xcbc98c: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbc98c() {
}

// 0xcbca88 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xcbca88: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbca88() {
}

// 0xcbcb3c — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xcbcb3c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbcb3c() {
}

// 0xcbcbe4 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr> const&)
// IDA 0xcbcbe4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbcbe4() {
}

// 0xcbcc50 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xcbcc50: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbcc50() {
}

// 0xcbcc54 — __ZNSt8_Rb_treeItSt4pairIKtN4Ogre29HardwareVertexBufferSharedPtrEESt10_Select1stIS4_ESt4lessItENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,std::_Select1st<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,Ogre::HardwareVertexBufferSharedPtr>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xcbcc54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbcc54() {
}

// 0xcbcc60 — __ZN4Ogre14HardwareBufferD1Ev
#[doc(alias = "Ogre::HardwareBuffer::~HardwareBuffer()")]
// was: Ogre::HardwareBuffer::~HardwareBuffer()
// IDA 0xcbcc60: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbcc60() {
}

// 0xcbcc64 — __ZN4Ogre14HardwareBufferD0Ev
#[doc(alias = "Ogre::HardwareBuffer::~HardwareBuffer()")]
// was: Ogre::HardwareBuffer::~HardwareBuffer()
// IDA 0xcbcc64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbcc64() {
}

// 0xcbcd24 — __ZN4Ogre19HighLevelGpuProgramC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::HighLevelGpuProgram::HighLevelGpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::HighLevelGpuProgram::HighLevelGpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xcbcd24: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbcd24() {
}

// 0xcbcd84 — __ZN4Ogre19HighLevelGpuProgram8loadImplEv
#[doc(alias = "Ogre::HighLevelGpuProgram::loadImpl(void)")]
// was: Ogre::HighLevelGpuProgram::loadImpl(void)
// IDA 0xcbcd84: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbcd84() {
}

// 0xcbcdc4 — __ZN4Ogre19HighLevelGpuProgram10unloadImplEv
#[doc(alias = "Ogre::HighLevelGpuProgram::unloadImpl(void)")]
// was: Ogre::HighLevelGpuProgram::unloadImpl(void)
// IDA 0xcbcdc4: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbcdc4() {
}

// 0xcbce38 — __ZN4Ogre19HighLevelGpuProgramD0Ev
#[doc(alias = "Ogre::HighLevelGpuProgram::~HighLevelGpuProgram()")]
// was: Ogre::HighLevelGpuProgram::~HighLevelGpuProgram()
// IDA 0xcbce38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbce38() {
}

// 0xcbcec8 — __ZN4Ogre19HighLevelGpuProgramD1Ev
#[doc(alias = "Ogre::HighLevelGpuProgram::~HighLevelGpuProgram()")]
// was: Ogre::HighLevelGpuProgram::~HighLevelGpuProgram()
// IDA 0xcbcec8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbcec8() {
}

// 0xcbced4 — __ZN4Ogre19HighLevelGpuProgramD2Ev
#[doc(alias = "Ogre::HighLevelGpuProgram::~HighLevelGpuProgram()")]
// was: Ogre::HighLevelGpuProgram::~HighLevelGpuProgram()
// IDA 0xcbced4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbced4() {
}

// 0xcbcff0 — __ZN4Ogre19HighLevelGpuProgram16createParametersEv
#[doc(alias = "Ogre::HighLevelGpuProgram::createParameters(void)")]
// was: Ogre::HighLevelGpuProgram::createParameters(void)
// IDA 0xcbcff0: 175 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbcff0() {
}

// 0xcbd1ac — __ZN4Ogre19HighLevelGpuProgram13loadHighLevelEv
#[doc(alias = "Ogre::HighLevelGpuProgram::loadHighLevel(void)")]
// was: Ogre::HighLevelGpuProgram::loadHighLevel(void)
// IDA 0xcbd1ac: 270 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd1ac() {
}

// 0xcbd460 — __ZN4Ogre19HighLevelGpuProgram15unloadHighLevelEv
#[doc(alias = "Ogre::HighLevelGpuProgram::unloadHighLevel(void)")]
// was: Ogre::HighLevelGpuProgram::unloadHighLevel(void)
// IDA 0xcbd460: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd460() {
}

// 0xcbd490 — __ZN4Ogre19HighLevelGpuProgram17loadHighLevelImplEv
#[doc(alias = "Ogre::HighLevelGpuProgram::loadHighLevelImpl(void)")]
// was: Ogre::HighLevelGpuProgram::loadHighLevelImpl(void)
// IDA 0xcbd490: 211 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd490() {
}

// 0xcbd6c8 — __ZNK4Ogre19HighLevelGpuProgram22getConstantDefinitionsEv
#[doc(alias = "Ogre::HighLevelGpuProgram::getConstantDefinitions(void)const")]
// was: Ogre::HighLevelGpuProgram::getConstantDefinitions(void)const
// IDA 0xcbd6c8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd6c8() {
}

// 0xcbd6ec — __ZN4Ogre19HighLevelGpuProgram22populateParameterNamesENS_9SharedPtrINS_20GpuProgramParametersEEE
#[doc(alias = "Ogre::HighLevelGpuProgram::populateParameterNames(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
// was: Ogre::HighLevelGpuProgram::populateParameterNames(Ogre::SharedPtr<Ogre::GpuProgramParameters>)
// IDA 0xcbd6ec: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd6ec() {
}

// 0xcbd750 — __ZN4Ogre26HighLevelGpuProgramManager15getSingletonPtrEv
#[doc(alias = "Ogre::HighLevelGpuProgramManager::getSingletonPtr(void)")]
// was: Ogre::HighLevelGpuProgramManager::getSingletonPtr(void)
// IDA 0xcbd750: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd750() {
}

// 0xcbd760 — __ZN4Ogre26HighLevelGpuProgramManager12getSingletonEv
#[doc(alias = "Ogre::HighLevelGpuProgramManager::getSingleton(void)")]
// was: Ogre::HighLevelGpuProgramManager::getSingleton(void)
// IDA 0xcbd760: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd760() {
}

// 0xcbd770 — __ZN4Ogre26HighLevelGpuProgramManagerC1Ev
#[doc(alias = "Ogre::HighLevelGpuProgramManager::HighLevelGpuProgramManager(void)")]
// was: Ogre::HighLevelGpuProgramManager::HighLevelGpuProgramManager(void)
// IDA 0xcbd770: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd770() {
}

// 0xcbd77c — __ZN4Ogre26HighLevelGpuProgramManagerC2Ev
#[doc(alias = "Ogre::HighLevelGpuProgramManager::HighLevelGpuProgramManager(void)")]
// was: Ogre::HighLevelGpuProgramManager::HighLevelGpuProgramManager(void)
// IDA 0xcbd77c: 177 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd77c() {
}

// 0xcbd96c — __ZN4Ogre26HighLevelGpuProgramManager10addFactoryEPNS_26HighLevelGpuProgramFactoryE
#[doc(alias = "Ogre::HighLevelGpuProgramManager::addFactory(Ogre::HighLevelGpuProgramFactory *)")]
// was: Ogre::HighLevelGpuProgramManager::addFactory(Ogre::HighLevelGpuProgramFactory *)
// IDA 0xcbd96c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbd96c() {
}

// 0xcbd98c — __ZN4Ogre26HighLevelGpuProgramManagerD0Ev
#[doc(alias = "Ogre::HighLevelGpuProgramManager::~HighLevelGpuProgramManager()")]
// was: Ogre::HighLevelGpuProgramManager::~HighLevelGpuProgramManager()
// IDA 0xcbd98c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbd98c() {
}

// 0xcbda1c — __ZN4Ogre26HighLevelGpuProgramManagerD1Ev
#[doc(alias = "Ogre::HighLevelGpuProgramManager::~HighLevelGpuProgramManager()")]
// was: Ogre::HighLevelGpuProgramManager::~HighLevelGpuProgramManager()
// IDA 0xcbda1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbda1c() {
}

// 0xcbda28 — __ZN4Ogre26HighLevelGpuProgramManagerD2Ev
#[doc(alias = "Ogre::HighLevelGpuProgramManager::~HighLevelGpuProgramManager()")]
// was: Ogre::HighLevelGpuProgramManager::~HighLevelGpuProgramManager()
// IDA 0xcbda28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbda28() {
}

// 0xcbdb4c — __ZN4Ogre26HighLevelGpuProgramManager13removeFactoryEPNS_26HighLevelGpuProgramFactoryE
#[doc(alias = "Ogre::HighLevelGpuProgramManager::removeFactory(Ogre::HighLevelGpuProgramFactory *)")]
// was: Ogre::HighLevelGpuProgramManager::removeFactory(Ogre::HighLevelGpuProgramFactory *)
// IDA 0xcbdb4c: 50 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbdb4c() {
}

// 0xcbdbd8 — __ZN4Ogre26HighLevelGpuProgramManager10createImplERKSsyS2_bPNS_20ManualResourceLoaderEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::HighLevelGpuProgramManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::HighLevelGpuProgramManager::createImpl(std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xcbdbd8: 231 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbdbd8() {
}

// 0xcbde78 — __ZN4Ogre26HighLevelGpuProgramManager13createProgramERKSsS2_S2_NS_14GpuProgramTypeE
#[doc(alias = "Ogre::HighLevelGpuProgramManager::createProgram(std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)")]
// was: Ogre::HighLevelGpuProgramManager::createProgram(std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)
// IDA 0xcbde78: 269 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbde78() {
}

// 0xcbe11c — __ZN4Ogre26HighLevelGpuProgramFactoryD2Ev
#[doc(alias = "Ogre::HighLevelGpuProgramFactory::~HighLevelGpuProgramFactory()")]
// was: Ogre::HighLevelGpuProgramFactory::~HighLevelGpuProgramFactory()
// IDA 0xcbe11c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbe11c() {
}

// 0xcbe120 — __ZNSt3mapISsPN4Ogre26HighLevelGpuProgramFactoryESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
#[doc(alias = "std::map<std::string,Ogre::HighLevelGpuProgramFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::HighLevelGpuProgramFactory *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xcbe120: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbe120() {
}

// 0xcbe2dc — __ZN4Ogre22HighLevelGpuProgramPtrD1Ev
#[doc(alias = "Ogre::HighLevelGpuProgramPtr::~HighLevelGpuProgramPtr()")]
// was: Ogre::HighLevelGpuProgramPtr::~HighLevelGpuProgramPtr()
// IDA 0xcbe2dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbe2dc() {
}

// 0xcbe3cc — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xcbe3cc: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbe3cc() {
}

// 0xcbe470 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xcbe470: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbe470() {
}

// 0xcbe650 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xcbe650: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbe650() {
}

// 0xcbe7a4 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *> const&)
// IDA 0xcbe7a4: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbe7a4() {
}

// 0xcbe888 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xcbe888: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbe888() {
}

// 0xcbe88c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xcbe88c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbe88c() {
}

// 0xcbe898 — __ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::~SharedPtr()
// IDA 0xcbe898: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbe898() {
}

// 0xcbe948 — __ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::~SharedPtr()
// IDA 0xcbe948: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbe948() {
}

// 0xcbea3c — __ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::destroy(void)
// IDA 0xcbea3c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbea3c() {
}

// 0xcbea74 — __ZN4Ogre9SharedPtrINS_19HighLevelGpuProgramEE4swapERS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::swap(Ogre::SharedPtr<Ogre::HighLevelGpuProgram>&)")]
// was: Ogre::SharedPtr<Ogre::HighLevelGpuProgram>::swap(Ogre::SharedPtr<Ogre::HighLevelGpuProgram>&)
// IDA 0xcbea74: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbea74() {
}

// 0xcbea90 — __ZN4Ogre22HighLevelGpuProgramPtrD0Ev
#[doc(alias = "Ogre::HighLevelGpuProgramPtr::~HighLevelGpuProgramPtr()")]
// was: Ogre::HighLevelGpuProgramPtr::~HighLevelGpuProgramPtr()
// IDA 0xcbea90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbea90() {
}

// 0xcbeb84 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre26HighLevelGpuProgramFactoryEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,std::_Select1st<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::HighLevelGpuProgramFactory *>> *)
// IDA 0xcbeb84: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbeb84() {
}

// 0xcbebfc — __ZN4Ogre18NullProgramFactoryD1Ev
#[doc(alias = "Ogre::NullProgramFactory::~NullProgramFactory()")]
// was: Ogre::NullProgramFactory::~NullProgramFactory()
// IDA 0xcbebfc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbebfc() {
}

// 0xcbec00 — __ZN4Ogre18NullProgramFactoryD0Ev
#[doc(alias = "Ogre::NullProgramFactory::~NullProgramFactory()")]
// was: Ogre::NullProgramFactory::~NullProgramFactory()
// IDA 0xcbec00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbec00() {
}

// 0xcbec8c — __ZNK4Ogre18NullProgramFactory11getLanguageEv
#[doc(alias = "Ogre::NullProgramFactory::getLanguage(void)const")]
// was: Ogre::NullProgramFactory::getLanguage(void)const
// IDA 0xcbec8c: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbec8c() {
}

// 0xcbec98 — __ZN4Ogre18NullProgramFactory6createEPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::NullProgramFactory::create(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::NullProgramFactory::create(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xcbec98: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbec98() {
}

// 0xcbed80 — __ZN4Ogre18NullProgramFactory7destroyEPNS_19HighLevelGpuProgramE
#[doc(alias = "Ogre::NullProgramFactory::destroy(Ogre::HighLevelGpuProgram *)")]
// was: Ogre::NullProgramFactory::destroy(Ogre::HighLevelGpuProgram *)
// IDA 0xcbed80: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbed80() {
}

// 0xcbed94 — __ZN4Ogre11NullProgramD1Ev
#[doc(alias = "Ogre::NullProgram::~NullProgram()")]
// was: Ogre::NullProgram::~NullProgram()
// IDA 0xcbed94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbed94() {
}

// 0xcbeda0 — __ZN4Ogre11NullProgramD0Ev
#[doc(alias = "Ogre::NullProgram::~NullProgram()")]
// was: Ogre::NullProgram::~NullProgram()
// IDA 0xcbeda0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_cbeda0() {
}

// 0xcbee30 — __ZN4Ogre11NullProgram12setParameterERKSsS2_
#[doc(alias = "Ogre::NullProgram::setParameter(std::string const&,std::string const&)")]
// was: Ogre::NullProgram::setParameter(std::string const&,std::string const&)
// IDA 0xcbee30: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cbee30() {
}

// 0xcbee34 — __ZN4Ogre8Resource12postLoadImplEv
#[doc(alias = "Ogre::Resource::postLoadImpl(void)")]
// was: Ogre::Resource::postLoadImpl(void)
// IDA 0xcbee34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_cbee34() {
}
