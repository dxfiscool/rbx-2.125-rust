//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe87644..0xe8e39c (100 stubs, 10760 prior -> 10860 covered, 2473 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0xe87644 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::_ConfigOption>>,std::pair<std::string const,Ogre::_ConfigOption> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::_ConfigOption>>,std::pair<std::string const,Ogre::_ConfigOption> const&)
// IDA 0xe87644: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e87644() {
}

// 0xe87824 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::_ConfigOption> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::_ConfigOption> const&)
// IDA 0xe87824: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e87824() {
}

// 0xe8796c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::_ConfigOption> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::_ConfigOption> const&)
// IDA 0xe8796c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8796c() {
}

// 0xe87a50 — __ZNSt4pairIKSsN4Ogre13_ConfigOptionEEC2ERKS3_
#[doc(alias = "std::pair<std::string const,Ogre::_ConfigOption>::pair(std::pair<std::string const,Ogre::_ConfigOption> const&)")]
// was: std::pair<std::string const,Ogre::_ConfigOption>::pair(std::pair<std::string const,Ogre::_ConfigOption> const&)
// IDA 0xe87a50: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e87a50() {
}

// 0xe87bf4 — __ZN4Ogre13_ConfigOptionD2Ev
#[doc(alias = "Ogre::_ConfigOption::~_ConfigOption()")]
// was: Ogre::_ConfigOption::~_ConfigOption()
// IDA 0xe87bf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e87bf4() {
}

// 0xe87d54 — __ZN4Ogre12GLES2SupportD1Ev
#[doc(alias = "Ogre::GLES2Support::~GLES2Support()")]
// was: Ogre::GLES2Support::~GLES2Support()
// IDA 0xe87d54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e87d54() {
}

// 0xe87d60 — __ZN4Ogre12GLES2SupportD0Ev
#[doc(alias = "Ogre::GLES2Support::~GLES2Support()")]
// was: Ogre::GLES2Support::~GLES2Support()
// IDA 0xe87d60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e87d60() {
}

// 0xe87d74 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::_ConfigOption>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::_ConfigOption>> *)
// IDA 0xe87d74: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e87d74() {
}

// 0xe87df4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe87df4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e87df4() {
}

// 0xe87df8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe87df8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e87df8() {
}

// 0xe88388 — __ZN4Ogre11EAGL2WindowC1EPNS_12EAGL2SupportE
#[doc(alias = "Ogre::EAGL2Window::EAGL2Window(Ogre::EAGL2Support *)")]
// was: Ogre::EAGL2Window::EAGL2Window(Ogre::EAGL2Support *)
// IDA 0xe88388: 112 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e88388() {
}

// 0xe884e4 — __ZN4Ogre11EAGL2WindowD0Ev
#[doc(alias = "Ogre::EAGL2Window::~EAGL2Window()")]
// was: Ogre::EAGL2Window::~EAGL2Window()
// IDA 0xe884e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e884e4() {
}

// 0xe885b8 — __ZN4Ogre11EAGL2WindowD1Ev
#[doc(alias = "Ogre::EAGL2Window::~EAGL2Window()")]
// was: Ogre::EAGL2Window::~EAGL2Window()
// IDA 0xe885b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e885b8() {
}

// 0xe88680 — __ZN4Ogre11EAGL2Window7destroyEv
#[doc(alias = "Ogre::EAGL2Window::destroy(void)")]
// was: Ogre::EAGL2Window::destroy(void)
// IDA 0xe88680: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e88680() {
}

// 0xe886f8 — __ZN4Ogre11EAGL2Window13setFullscreenEbjj
#[doc(alias = "Ogre::EAGL2Window::setFullscreen(bool,unsigned int,unsigned int)")]
// was: Ogre::EAGL2Window::setFullscreen(bool,unsigned int,unsigned int)
// IDA 0xe886f8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e886f8() {
}

// 0xe886fc — __ZN4Ogre11EAGL2Window10repositionEii
#[doc(alias = "Ogre::EAGL2Window::reposition(int,int)")]
// was: Ogre::EAGL2Window::reposition(int,int)
// IDA 0xe886fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e886fc() {
}

// 0xe88700 — __ZN4Ogre11EAGL2Window6resizeEjj
#[doc(alias = "Ogre::EAGL2Window::resize(unsigned int,unsigned int)")]
// was: Ogre::EAGL2Window::resize(unsigned int,unsigned int)
// IDA 0xe88700: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e88700() {
}

// 0xe88800 — __ZN4Ogre11EAGL2Window20windowMovedOrResizedEv
#[doc(alias = "Ogre::EAGL2Window::windowMovedOrResized(void)")]
// was: Ogre::EAGL2Window::windowMovedOrResized(void)
// IDA 0xe88800: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_e88800() {
}

// 0xe88894 — __ZN4Ogre11EAGL2Window12_beginUpdateEv
#[doc(alias = "Ogre::EAGL2Window::_beginUpdate(void)")]
// was: Ogre::EAGL2Window::_beginUpdate(void)
// IDA 0xe88894: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e88894() {
}

// 0xe888bc — __ZN4Ogre11EAGL2Window23initNativeCreatedWindowEPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIKSsSsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::EAGL2Window::initNativeCreatedWindow(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::EAGL2Window::initNativeCreatedWindow(std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xe888bc: 808 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e888bc() {
}

// 0xe89488 — __ZN4Ogre11EAGL2Window6createERKSsjjbPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::EAGL2Window::create(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::EAGL2Window::create(std::string const&,unsigned int,unsigned int,bool,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xe89488: 715 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e89488() {
}

// 0xe89c80 — __ZN4Ogre11EAGL2Window11swapBuffersEb
#[doc(alias = "Ogre::EAGL2Window::swapBuffers(bool)")]
// was: Ogre::EAGL2Window::swapBuffers(bool)
// IDA 0xe89c80: 190 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e89c80() {
}

// 0xe89f88 — __ZN4Ogre11EAGL2Window18getCustomAttributeERKSsPv
#[doc(alias = "Ogre::EAGL2Window::getCustomAttribute(std::string const&,void *)")]
// was: Ogre::EAGL2Window::getCustomAttribute(std::string const&,void *)
// IDA 0xe89f88: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e89f88() {
}

// 0xe8a038 — __ZN4Ogre11EAGL2Window20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE
#[doc(alias = "Ogre::EAGL2Window::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)")]
// was: Ogre::EAGL2Window::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)
// IDA 0xe8a038: 424 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a038() {
}

// 0xe8a530 — __ZNK4Ogre12RenderTarget11getPriorityEv
#[doc(alias = "Ogre::RenderTarget::getPriority(void)const")]
// was: Ogre::RenderTarget::getPriority(void)const
// IDA 0xe8a530: 2 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a530() {
}

// 0xe8a534 — __ZNK4Ogre12RenderWindow8isActiveEv
#[doc(alias = "Ogre::RenderWindow::isActive(void)const")]
// was: Ogre::RenderWindow::isActive(void)const
// IDA 0xe8a534: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a534() {
}

// 0xe8a550 — __ZNK4Ogre12RenderWindow18suggestPixelFormatEv
#[doc(alias = "Ogre::RenderWindow::suggestPixelFormat(void)const")]
// was: Ogre::RenderWindow::suggestPixelFormat(void)const
// IDA 0xe8a550: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a550() {
}

// 0xe8a554 — __ZNK4Ogre11EAGL2Window23requiresTextureFlippingEv
#[doc(alias = "Ogre::EAGL2Window::requiresTextureFlipping(void)const")]
// was: Ogre::EAGL2Window::requiresTextureFlipping(void)const
// IDA 0xe8a554: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a554() {
}

// 0xe8a558 — __ZNK4Ogre12RenderTarget22isHardwareGammaEnabledEv
#[doc(alias = "Ogre::RenderTarget::isHardwareGammaEnabled(void)const")]
// was: Ogre::RenderTarget::isHardwareGammaEnabled(void)const
// IDA 0xe8a558: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a558() {
}

// 0xe8a560 — __ZNK4Ogre12RenderTarget7getFSAAEv
#[doc(alias = "Ogre::RenderTarget::getFSAA(void)const")]
// was: Ogre::RenderTarget::getFSAA(void)const
// IDA 0xe8a560: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a560() {
}

// 0xe8a568 — __ZNK4Ogre11EAGL2Window9isVisibleEv
#[doc(alias = "Ogre::EAGL2Window::isVisible(void)const")]
// was: Ogre::EAGL2Window::isVisible(void)const
// IDA 0xe8a568: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a568() {
}

// 0xe8a570 — __ZN4Ogre11EAGL2Window10setVisibleEb
#[doc(alias = "Ogre::EAGL2Window::setVisible(bool)")]
// was: Ogre::EAGL2Window::setVisible(bool)
// IDA 0xe8a570: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a570() {
}

// 0xe8a578 — __ZNK4Ogre12RenderWindow8isHiddenEv
#[doc(alias = "Ogre::RenderWindow::isHidden(void)const")]
// was: Ogre::RenderWindow::isHidden(void)const
// IDA 0xe8a578: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a578() {
}

// 0xe8a580 — __ZN4Ogre12RenderWindow15setVSyncEnabledEb
#[doc(alias = "Ogre::RenderWindow::setVSyncEnabled(bool)")]
// was: Ogre::RenderWindow::setVSyncEnabled(bool)
// IDA 0xe8a580: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8a580() {
}

// 0xe8a588 — __ZN4Ogre12RenderWindow16setVSyncIntervalEj
#[doc(alias = "Ogre::RenderWindow::setVSyncInterval(unsigned int)")]
// was: Ogre::RenderWindow::setVSyncInterval(unsigned int)
// IDA 0xe8a588: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8a588() {
}

// 0xe8a590 — __ZNK4Ogre11EAGL2Window8isClosedEv
#[doc(alias = "Ogre::EAGL2Window::isClosed(void)const")]
// was: Ogre::EAGL2Window::isClosed(void)const
// IDA 0xe8a590: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a590() {
}

// 0xe8a598 — __ZN4Ogre12RenderWindow12isDeviceLostEv
#[doc(alias = "Ogre::RenderWindow::isDeviceLost(void)")]
// was: Ogre::RenderWindow::isDeviceLost(void)
// IDA 0xe8a598: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a598() {
}

// 0xe8a5a0 — __ZN4Ogre12RenderWindow10getGPUTimeEv
#[doc(alias = "Ogre::RenderWindow::getGPUTime(void)")]
// was: Ogre::RenderWindow::getGPUTime(void)
// IDA 0xe8a5a0: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a5a0() {
}

// 0xe8a5b8 — __ZN4Ogre12RenderWindow16setFrameCallbackEPNS_18IFrameDataCallbackE
#[doc(alias = "Ogre::RenderWindow::setFrameCallback(Ogre::IFrameDataCallback *)")]
// was: Ogre::RenderWindow::setFrameCallback(Ogre::IFrameDataCallback *)
// IDA 0xe8a5b8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8a5b8() {
}

// 0xe8a5c0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre13_ConfigOptionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::_ConfigOption>,std::_Select1st<std::pair<std::string const,Ogre::_ConfigOption>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::_ConfigOption>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe8a5c0: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a5c0() {
}

// 0xe8a698 — __ZN4Ogre14EAGLES2ContextC1EP11CAEAGLLayerP14EAGLSharegroup
#[doc(alias = "Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)")]
// was: Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)
// IDA 0xe8a698: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a698() {
}

// 0xe8a6a4 — __ZN4Ogre14EAGLES2ContextC2EP11CAEAGLLayerP14EAGLSharegroup
#[doc(alias = "Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)")]
// was: Ogre::EAGLES2Context::EAGLES2Context(CAEAGLLayer *,EAGLSharegroup *)
// IDA 0xe8a6a4: 242 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8a6a4() {
}

// 0xe8a970 — __ZN4Ogre14EAGLES2ContextD0Ev
#[doc(alias = "Ogre::EAGLES2Context::~EAGLES2Context()")]
// was: Ogre::EAGLES2Context::~EAGLES2Context()
// IDA 0xe8a970: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8a970() {
}

// 0xe8aab4 — __ZN4Ogre14EAGLES2ContextD1Ev
#[doc(alias = "Ogre::EAGLES2Context::~EAGLES2Context()")]
// was: Ogre::EAGLES2Context::~EAGLES2Context()
// IDA 0xe8aab4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8aab4() {
}

// 0xe8abf8 — __ZN4Ogre14EAGLES2Context18destroyFramebufferEv
#[doc(alias = "Ogre::EAGLES2Context::destroyFramebuffer(void)")]
// was: Ogre::EAGLES2Context::destroyFramebuffer(void)
// IDA 0xe8abf8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8abf8() {
}

// 0xe8ac58 — __ZN4Ogre14EAGLES2Context17createFramebufferEv
#[doc(alias = "Ogre::EAGLES2Context::createFramebuffer(void)")]
// was: Ogre::EAGLES2Context::createFramebuffer(void)
// IDA 0xe8ac58: 529 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8ac58() {
}

// 0xe8b298 — __ZN4Ogre14EAGLES2Context10setCurrentEv
#[doc(alias = "Ogre::EAGLES2Context::setCurrent(void)")]
// was: Ogre::EAGLES2Context::setCurrent(void)
// IDA 0xe8b298: 163 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b298() {
}

// 0xe8b488 — __ZN4Ogre14EAGLES2Context10endCurrentEv
#[doc(alias = "Ogre::EAGLES2Context::endCurrent(void)")]
// was: Ogre::EAGLES2Context::endCurrent(void)
// IDA 0xe8b488: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8b488() {
}

// 0xe8b48c — __ZNK4Ogre14EAGLES2Context5cloneEv
#[doc(alias = "Ogre::EAGLES2Context::clone(void)const")]
// was: Ogre::EAGLES2Context::clone(void)const
// IDA 0xe8b48c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8b48c() {
}

// 0xe8b490 — __ZNK4Ogre14EAGLES2Context10getContextEv
#[doc(alias = "Ogre::EAGLES2Context::getContext(void)const")]
// was: Ogre::EAGLES2Context::getContext(void)const
// IDA 0xe8b490: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b490() {
}

// 0xe8b494 — __ZN4Ogre12GLES2Context14releaseContextEv
#[doc(alias = "Ogre::GLES2Context::releaseContext(void)")]
// was: Ogre::GLES2Context::releaseContext(void)
// IDA 0xe8b494: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8b494() {
}

// 0xe8b4cc — __ZN4Ogre24GLSLESLinkProgramManager12getSingletonEv
#[doc(alias = "Ogre::GLSLESLinkProgramManager::getSingleton(void)")]
// was: Ogre::GLSLESLinkProgramManager::getSingleton(void)
// IDA 0xe8b4cc: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b4cc() {
}

// 0xe8b4dc — __ZN4Ogre24GLSLESLinkProgramManagerC1Ev
#[doc(alias = "Ogre::GLSLESLinkProgramManager::GLSLESLinkProgramManager(void)")]
// was: Ogre::GLSLESLinkProgramManager::GLSLESLinkProgramManager(void)
// IDA 0xe8b4dc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b4dc() {
}

// 0xe8b53c — __ZN4Ogre24GLSLESLinkProgramManagerD1Ev
#[doc(alias = "Ogre::GLSLESLinkProgramManager::~GLSLESLinkProgramManager()")]
// was: Ogre::GLSLESLinkProgramManager::~GLSLESLinkProgramManager()
// IDA 0xe8b53c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8b53c() {
}

// 0xe8b548 — __ZN4Ogre24GLSLESLinkProgramManagerD2Ev
#[doc(alias = "Ogre::GLSLESLinkProgramManager::~GLSLESLinkProgramManager()")]
// was: Ogre::GLSLESLinkProgramManager::~GLSLESLinkProgramManager()
// IDA 0xe8b548: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8b548() {
}

// 0xe8b678 — __ZN4Ogre24GLSLESLinkProgramManager20getActiveLinkProgramEv
#[doc(alias = "Ogre::GLSLESLinkProgramManager::getActiveLinkProgram(void)")]
// was: Ogre::GLSLESLinkProgramManager::getActiveLinkProgram(void)
// IDA 0xe8b678: 195 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b678() {
}

// 0xe8b870 — __ZN4Ogre24GLSLESLinkProgramManager23setActiveFragmentShaderEPNS_16GLSLESGpuProgramE
#[doc(alias = "Ogre::GLSLESLinkProgramManager::setActiveFragmentShader(Ogre::GLSLESGpuProgram *)")]
// was: Ogre::GLSLESLinkProgramManager::setActiveFragmentShader(Ogre::GLSLESGpuProgram *)
// IDA 0xe8b870: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b870() {
}

// 0xe8b880 — __ZN4Ogre24GLSLESLinkProgramManager21setActiveVertexShaderEPNS_16GLSLESGpuProgramE
#[doc(alias = "Ogre::GLSLESLinkProgramManager::setActiveVertexShader(Ogre::GLSLESGpuProgram *)")]
// was: Ogre::GLSLESLinkProgramManager::setActiveVertexShader(Ogre::GLSLESGpuProgram *)
// IDA 0xe8b880: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b880() {
}

// 0xe8b890 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)")]
// was: std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)
// IDA 0xe8b890: 315 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8b890() {
}

// 0xe8bb94 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)")]
// was: std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *> const&)
// IDA 0xe8bb94: 152 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8bb94() {
}

// 0xe8bd04 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe8bd04: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8bd04() {
}

// 0xe8bd08 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe8bd08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8bd08() {
}

// 0xe8bd14 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()
// IDA 0xe8bd14: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8bd14() {
}

// 0xe8bd18 — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned long long>,false>::~_Rb_tree_impl()
// IDA 0xe8bd18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8bd18() {
}

// 0xe8bd24 — __ZNSt8_Rb_treeISsSt4pairIKSsjESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned int>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,unsigned int>,std::_Select1st<std::pair<std::string const,unsigned int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,unsigned int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned int>> *)
// IDA 0xe8bd24: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8bd24() {
}

// 0xe8bd9c — __ZNSt8_Rb_treeIySt4pairIKyPN4Ogre17GLSLESLinkProgramEESt10_Select1stIS5_ESt4lessIyENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>> *)")]
// was: std::_Rb_tree<unsigned long long,std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,std::_Select1st<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>>,std::less<unsigned long long>,Ogre::STLAllocator<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long long const,Ogre::GLSLESLinkProgram *>> *)
// IDA 0xe8bd9c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8bd9c() {
}

// 0xe8bdf8 — __ZN4Ogre19GLSLESProgramCommonC2EPNS_16GLSLESGpuProgramES2_
#[doc(alias = "Ogre::GLSLESProgramCommon::GLSLESProgramCommon(Ogre::GLSLESGpuProgram *,Ogre::GLSLESGpuProgram *)")]
// was: Ogre::GLSLESProgramCommon::GLSLESProgramCommon(Ogre::GLSLESGpuProgram *,Ogre::GLSLESGpuProgram *)
// IDA 0xe8bdf8: 1271 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8bdf8() {
}

// 0xe8cc50 — __ZN4Ogre19GLSLESProgramCommonD0Ev
#[doc(alias = "Ogre::GLSLESProgramCommon::~GLSLESProgramCommon()")]
// was: Ogre::GLSLESProgramCommon::~GLSLESProgramCommon()
// IDA 0xe8cc50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8cc50() {
}

// 0xe8cc64 — __ZN4Ogre19GLSLESProgramCommonD1Ev
#[doc(alias = "Ogre::GLSLESProgramCommon::~GLSLESProgramCommon()")]
// was: Ogre::GLSLESProgramCommon::~GLSLESProgramCommon()
// IDA 0xe8cc64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8cc64() {
}

// 0xe8cc70 — __ZN4Ogre19GLSLESProgramCommonD2Ev
#[doc(alias = "Ogre::GLSLESProgramCommon::~GLSLESProgramCommon()")]
// was: Ogre::GLSLESProgramCommon::~GLSLESProgramCommon()
// IDA 0xe8cc70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8cc70() {
}

// 0xe8cd70 — __ZN4Ogre19GLSLESProgramCommon15getCombinedNameEv
#[doc(alias = "Ogre::GLSLESProgramCommon::getCombinedName(void)")]
// was: Ogre::GLSLESProgramCommon::getCombinedName(void)
// IDA 0xe8cd70: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8cd70() {
}

// 0xe8cecc — __ZN4Ogre19GLSLESProgramCommon24getAttributeSemanticEnumESs
#[doc(alias = "Ogre::GLSLESProgramCommon::getAttributeSemanticEnum(std::string)")]
// was: Ogre::GLSLESProgramCommon::getAttributeSemanticEnum(std::string)
// IDA 0xe8cecc: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8cecc() {
}

// 0xe8cee4 — __ZN4Ogre19GLSLESProgramCommon26getAttributeSemanticStringENS_21VertexElementSemanticE
#[doc(alias = "Ogre::GLSLESProgramCommon::getAttributeSemanticString(Ogre::VertexElementSemantic)")]
// was: Ogre::GLSLESProgramCommon::getAttributeSemanticString(Ogre::VertexElementSemantic)
// IDA 0xe8cee4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8cee4() {
}

// 0xe8cf0c — __ZN4Ogre19GLSLESProgramCommon17getAttributeIndexENS_21VertexElementSemanticEj
#[doc(alias = "Ogre::GLSLESProgramCommon::getAttributeIndex(Ogre::VertexElementSemantic,unsigned int)")]
// was: Ogre::GLSLESProgramCommon::getAttributeIndex(Ogre::VertexElementSemantic,unsigned int)
// IDA 0xe8cf0c: 277 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8cf0c() {
}

// 0xe8d21c — __ZN4Ogre19GLSLESProgramCommon16isAttributeValidENS_21VertexElementSemanticEj
#[doc(alias = "Ogre::GLSLESProgramCommon::isAttributeValid(Ogre::VertexElementSemantic,unsigned int)")]
// was: Ogre::GLSLESProgramCommon::isAttributeValid(Ogre::VertexElementSemantic,unsigned int)
// IDA 0xe8d21c: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8d21c() {
}

// 0xe8d234 — __ZN4Ogre19GLSLESProgramCommon21getMicrocodeFromCacheEv
#[doc(alias = "Ogre::GLSLESProgramCommon::getMicrocodeFromCache(void)")]
// was: Ogre::GLSLESProgramCommon::getMicrocodeFromCache(void)
// IDA 0xe8d234: 487 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8d234() {
}

// 0xe8d72c — __ZNSt3mapISsN4Ogre21VertexElementSemanticESt4lessISsENS0_12STLAllocatorISt4pairIKSsS1_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS6_
#[doc(alias = "std::map<std::string,Ogre::VertexElementSemantic,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
// was: std::map<std::string,Ogre::VertexElementSemantic,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)
// IDA 0xe8d72c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8d72c() {
}

// 0xe8d8e8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::pair<std::string const,Ogre::VertexElementSemantic> const&)
// IDA 0xe8d8e8: 184 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8d8e8() {
}

// 0xe8dac8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::VertexElementSemantic> const&)
// IDA 0xe8dac8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8dac8() {
}

// 0xe8dc1c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexElementSemantic> const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::VertexElementSemantic> const&)
// IDA 0xe8dc1c: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8dc1c() {
}

// 0xe8dd00 — __ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe8dd00: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8dd00() {
}

// 0xe8dd04 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe8dd04: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8dd04() {
}

// 0xe8dd08 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xe8dd08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8dd08() {
}

// 0xe8dd14 — __ZNSt12_Vector_baseIN4Ogre18GLUniformReferenceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::GLUniformReference,Ogre::STLAllocator<Ogre::GLUniformReference,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xe8dd14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8dd14() {
}

// 0xe8dd20 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21VertexElementSemanticEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexElementSemantic>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::VertexElementSemantic>,std::_Select1st<std::pair<std::string const,Ogre::VertexElementSemantic>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::VertexElementSemantic>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::VertexElementSemantic>> *)
// IDA 0xe8dd20: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8dd20() {
}

// 0xe8ddcc — __ZN4Ogre20GLSLESProgramFactoryC1Ev
#[doc(alias = "Ogre::GLSLESProgramFactory::GLSLESProgramFactory(void)")]
// was: Ogre::GLSLESProgramFactory::GLSLESProgramFactory(void)
// IDA 0xe8ddcc: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8ddcc() {
}

// 0xe8ddd8 — __ZN4Ogre20GLSLESProgramFactoryC2Ev
#[doc(alias = "Ogre::GLSLESProgramFactory::GLSLESProgramFactory(void)")]
// was: Ogre::GLSLESProgramFactory::GLSLESProgramFactory(void)
// IDA 0xe8ddd8: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8ddd8() {
}

// 0xe8df18 — __ZN4Ogre20GLSLESProgramFactoryD0Ev
#[doc(alias = "Ogre::GLSLESProgramFactory::~GLSLESProgramFactory()")]
// was: Ogre::GLSLESProgramFactory::~GLSLESProgramFactory()
// IDA 0xe8df18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8df18() {
}

// 0xe8dfa8 — __ZN4Ogre20GLSLESProgramFactoryD1Ev
#[doc(alias = "Ogre::GLSLESProgramFactory::~GLSLESProgramFactory()")]
// was: Ogre::GLSLESProgramFactory::~GLSLESProgramFactory()
// IDA 0xe8dfa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8dfa8() {
}

// 0xe8dfb4 — __ZN4Ogre20GLSLESProgramFactoryD2Ev
#[doc(alias = "Ogre::GLSLESProgramFactory::~GLSLESProgramFactory()")]
// was: Ogre::GLSLESProgramFactory::~GLSLESProgramFactory()
// IDA 0xe8dfb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8dfb4() {
}

// 0xe8e0b0 — __ZNK4Ogre20GLSLESProgramFactory11getLanguageEv
#[doc(alias = "Ogre::GLSLESProgramFactory::getLanguage(void)const")]
// was: Ogre::GLSLESProgramFactory::getLanguage(void)const
// IDA 0xe8e0b0: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e0b0() {
}

// 0xe8e0bc — __ZN4Ogre20GLSLESProgramFactory6createEPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::GLSLESProgramFactory::create(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::GLSLESProgramFactory::create(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe8e0bc: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e0bc() {
}

// 0xe8e190 — __ZN4Ogre20GLSLESProgramFactory7destroyEPNS_19HighLevelGpuProgramE
#[doc(alias = "Ogre::GLSLESProgramFactory::destroy(Ogre::HighLevelGpuProgram *)")]
// was: Ogre::GLSLESProgramFactory::destroy(Ogre::HighLevelGpuProgram *)
// IDA 0xe8e190: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e190() {
}

// 0xe8e210 — __ZN4Ogre28GLSLESProgramPipelineManager12getSingletonEv
#[doc(alias = "Ogre::GLSLESProgramPipelineManager::getSingleton(void)")]
// was: Ogre::GLSLESProgramPipelineManager::getSingleton(void)
// IDA 0xe8e210: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e210() {
}

// 0xe8e220 — __ZN4Ogre28GLSLESProgramPipelineManagerC1Ev
#[doc(alias = "Ogre::GLSLESProgramPipelineManager::GLSLESProgramPipelineManager(void)")]
// was: Ogre::GLSLESProgramPipelineManager::GLSLESProgramPipelineManager(void)
// IDA 0xe8e220: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e220() {
}

// 0xe8e264 — __ZN4Ogre28GLSLESProgramPipelineManagerD1Ev
#[doc(alias = "Ogre::GLSLESProgramPipelineManager::~GLSLESProgramPipelineManager()")]
// was: Ogre::GLSLESProgramPipelineManager::~GLSLESProgramPipelineManager()
// IDA 0xe8e264: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8e264() {
}

// 0xe8e270 — __ZN4Ogre28GLSLESProgramPipelineManagerD2Ev
#[doc(alias = "Ogre::GLSLESProgramPipelineManager::~GLSLESProgramPipelineManager()")]
// was: Ogre::GLSLESProgramPipelineManager::~GLSLESProgramPipelineManager()
// IDA 0xe8e270: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8e270() {
}

// 0xe8e37c — __ZN4Ogre28GLSLESProgramPipelineManager28setActiveFragmentLinkProgramEPNS_16GLSLESGpuProgramE
#[doc(alias = "Ogre::GLSLESProgramPipelineManager::setActiveFragmentLinkProgram(Ogre::GLSLESGpuProgram *)")]
// was: Ogre::GLSLESProgramPipelineManager::setActiveFragmentLinkProgram(Ogre::GLSLESGpuProgram *)
// IDA 0xe8e37c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e37c() {
}

// 0xe8e38c — __ZN4Ogre28GLSLESProgramPipelineManager26setActiveVertexLinkProgramEPNS_16GLSLESGpuProgramE
#[doc(alias = "Ogre::GLSLESProgramPipelineManager::setActiveVertexLinkProgram(Ogre::GLSLESGpuProgram *)")]
// was: Ogre::GLSLESProgramPipelineManager::setActiveVertexLinkProgram(Ogre::GLSLESGpuProgram *)
// IDA 0xe8e38c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e38c() {
}

// 0xe8e39c — __ZN4Ogre28GLSLESProgramPipelineManager24getActiveProgramPipelineEv
#[doc(alias = "Ogre::GLSLESProgramPipelineManager::getActiveProgramPipeline(void)")]
// was: Ogre::GLSLESProgramPipelineManager::getActiveProgramPipeline(void)
// IDA 0xe8e39c: 195 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8e39c() {
}
