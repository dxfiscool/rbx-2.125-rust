//! rendering — generated_499 — 100 stubs global dedup (rendering filtered, EA-sorted asc, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) NOT in /tmp/global_eas.txt — next 100 uncovered EA-sorted asc 0xd7b4d4..0xd7f3f0 (3274 candidates remaining, 90605 global EAs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr). Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xd7b4d4 — __ZNSt8_Rb_treeISsSt4pairIKSsMN4Ogre24RenderSystemCapabilitiesEFvfEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>,std::_Select1st<std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsMN4Ogre24RenderSystemCapabilitiesEFvfEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>,std::_Select1st<std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,void (Ogre::RenderSystemCapabilities::*)(float)>> *)
// IDA 0xd7b4d4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7b4d4() {
}

// 0xd7b54c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Capabilities>,std::_Select1st<std::pair<std::string const,Ogre::Capabilities>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Capabilities>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Capabilities>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre12CapabilitiesEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Capabilities>,std::_Select1st<std::pair<std::string const,Ogre::Capabilities>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Capabilities>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Capabilities>> *)
// IDA 0xd7b54c: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7b54c() {
}

// 0xd7b5f8 — __ZN4Ogre12RenderTargetC2Ev
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::RenderTarget(void)")]
#[doc(alias = "__ZN4Ogre12RenderTargetC2Ev")]
// was: Ogre::RenderTarget::RenderTarget(void)
// IDA 0xd7b5f8: 178 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7b5f8() {
}

// 0xd7b7e4 — __ZN4Ogre12RenderTargetD0Ev
// type: void __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::~RenderTarget()")]
#[doc(alias = "__ZN4Ogre12RenderTargetD0Ev")]
// was: Ogre::RenderTarget::~RenderTarget()
// IDA 0xd7b7e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7b7e4() {
}

// 0xd7b874 — __ZN4Ogre12RenderTargetD1Ev
// type: void __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::~RenderTarget()")]
#[doc(alias = "__ZN4Ogre12RenderTargetD1Ev")]
// was: Ogre::RenderTarget::~RenderTarget()
// IDA 0xd7b874: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7b874() {
}

// 0xd7b880 — __ZN4Ogre12RenderTargetD2Ev
// type: void __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::~RenderTarget()")]
#[doc(alias = "__ZN4Ogre12RenderTargetD2Ev")]
// was: Ogre::RenderTarget::~RenderTarget()
// IDA 0xd7b880: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7b880() {
}

// 0xd7bbd8 — __ZNK4Ogre12RenderTarget7getNameEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getName(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget7getNameEv")]
// was: Ogre::RenderTarget::getName(void)const
// IDA 0xd7bbd8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bbd8() {
}

// 0xd7bbdc — __ZN4Ogre12RenderTarget10getMetricsERjS1_S1_
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, unsigned int *, unsigned int *, unsigned int *)
#[doc(alias = "Ogre::RenderTarget::getMetrics(unsigned int &,unsigned int &,unsigned int &)")]
#[doc(alias = "__ZN4Ogre12RenderTarget10getMetricsERjS1_S1_")]
// was: Ogre::RenderTarget::getMetrics(unsigned int &,unsigned int &,unsigned int &)
// IDA 0xd7bbdc: 7 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bbdc() {
}

// 0xd7bbf0 — __ZNK4Ogre12RenderTarget8getWidthEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getWidth(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget8getWidthEv")]
// was: Ogre::RenderTarget::getWidth(void)const
// IDA 0xd7bbf0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bbf0() {
}

// 0xd7bbf4 — __ZNK4Ogre12RenderTarget9getHeightEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getHeight(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget9getHeightEv")]
// was: Ogre::RenderTarget::getHeight(void)const
// IDA 0xd7bbf4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bbf4() {
}

// 0xd7bbf8 — __ZNK4Ogre12RenderTarget14getColourDepthEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getColourDepth(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget14getColourDepthEv")]
// was: Ogre::RenderTarget::getColourDepth(void)const
// IDA 0xd7bbf8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bbf8() {
}

// 0xd7bbfc — __ZN4Ogre12RenderTarget18setDepthBufferPoolEt
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, unsigned __int16)
#[doc(alias = "Ogre::RenderTarget::setDepthBufferPool(unsigned short)")]
#[doc(alias = "__ZN4Ogre12RenderTarget18setDepthBufferPoolEt")]
// was: Ogre::RenderTarget::setDepthBufferPool(unsigned short)
// IDA 0xd7bbfc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bbfc() {
}

// 0xd7bc14 — __ZNK4Ogre12RenderTarget18getDepthBufferPoolEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getDepthBufferPool(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget18getDepthBufferPoolEv")]
// was: Ogre::RenderTarget::getDepthBufferPool(void)const
// IDA 0xd7bc14: 2 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bc14() {
}

// 0xd7bc18 — __ZNK4Ogre12RenderTarget14getDepthBufferEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getDepthBuffer(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget14getDepthBufferEv")]
// was: Ogre::RenderTarget::getDepthBuffer(void)const
// IDA 0xd7bc18: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bc18() {
}

// 0xd7bc1c — __ZN4Ogre12RenderTarget17attachDepthBufferEPNS_11DepthBufferE
#[doc(alias = "Ogre::RenderTarget::attachDepthBuffer(Ogre::DepthBuffer *)")]
#[doc(alias = "__ZN4Ogre12RenderTarget17attachDepthBufferEPNS_11DepthBufferE")]
// was: Ogre::RenderTarget::attachDepthBuffer(Ogre::DepthBuffer *)
// IDA 0xd7bc1c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bc1c() {
}

// 0xd7bc4c — __ZN4Ogre12RenderTarget17detachDepthBufferEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::detachDepthBuffer(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget17detachDepthBufferEv")]
// was: Ogre::RenderTarget::detachDepthBuffer(void)
// IDA 0xd7bc4c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bc4c() {
}

// 0xd7bc68 — __ZN4Ogre12RenderTarget18_detachDepthBufferEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::_detachDepthBuffer(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget18_detachDepthBufferEv")]
// was: Ogre::RenderTarget::_detachDepthBuffer(void)
// IDA 0xd7bc68: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bc68() {
}

// 0xd7bc70 — __ZN4Ogre12RenderTarget10updateImplEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::updateImpl(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget10updateImplEv")]
// was: Ogre::RenderTarget::updateImpl(void)
// IDA 0xd7bc70: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bc70() {
}

// 0xd7bc98 — __ZN4Ogre12RenderTarget12_beginUpdateEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::_beginUpdate(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget12_beginUpdateEv")]
// was: Ogre::RenderTarget::_beginUpdate(void)
// IDA 0xd7bc98: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bc98() {
}

// 0xd7bcf8 — __ZN4Ogre12RenderTarget10_endUpdateEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::_endUpdate(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget10_endUpdateEv")]
// was: Ogre::RenderTarget::_endUpdate(void)
// IDA 0xd7bcf8: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7bcf8() {
}

// 0xd7c4e0 — __ZNK4Ogre12RenderTarget13getStatisticsERfS1_S1_S1_
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, float *, float *, float *, float *)
#[doc(alias = "Ogre::RenderTarget::getStatistics(float &,float &,float &,float &)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget13getStatisticsERfS1_S1_S1_")]
// was: Ogre::RenderTarget::getStatistics(float &,float &,float &,float &)const
// IDA 0xd7c4e0: 7 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c4e0() {
}

// 0xd7c4f4 — __ZNK4Ogre12RenderTarget13getStatisticsEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getStatistics(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget13getStatisticsEv")]
// was: Ogre::RenderTarget::getStatistics(void)const
// IDA 0xd7c4f4: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c4f4() {
}

// 0xd7c4f8 — __ZNK4Ogre12RenderTarget10getLastFPSEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getLastFPS(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget10getLastFPSEv")]
// was: Ogre::RenderTarget::getLastFPS(void)const
// IDA 0xd7c4f8: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c4f8() {
}

// 0xd7c4fc — __ZNK4Ogre12RenderTarget13getAverageFPSEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getAverageFPS(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget13getAverageFPSEv")]
// was: Ogre::RenderTarget::getAverageFPS(void)const
// IDA 0xd7c4fc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c4fc() {
}

// 0xd7c500 — __ZNK4Ogre12RenderTarget10getBestFPSEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getBestFPS(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget10getBestFPSEv")]
// was: Ogre::RenderTarget::getBestFPS(void)const
// IDA 0xd7c500: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c500() {
}

// 0xd7c504 — __ZNK4Ogre12RenderTarget11getWorstFPSEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getWorstFPS(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget11getWorstFPSEv")]
// was: Ogre::RenderTarget::getWorstFPS(void)const
// IDA 0xd7c504: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c504() {
}

// 0xd7c508 — __ZNK4Ogre12RenderTarget16getTriangleCountEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getTriangleCount(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget16getTriangleCountEv")]
// was: Ogre::RenderTarget::getTriangleCount(void)const
// IDA 0xd7c508: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c508() {
}

// 0xd7c50c — __ZNK4Ogre12RenderTarget13getBatchCountEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getBatchCount(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget13getBatchCountEv")]
// was: Ogre::RenderTarget::getBatchCount(void)const
// IDA 0xd7c50c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c50c() {
}

// 0xd7c510 — __ZNK4Ogre12RenderTarget16getBestFrameTimeEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getBestFrameTime(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget16getBestFrameTimeEv")]
// was: Ogre::RenderTarget::getBestFrameTime(void)const
// IDA 0xd7c510: 4 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c510() {
}

// 0xd7c520 — __ZNK4Ogre12RenderTarget17getWorstFrameTimeEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getWorstFrameTime(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget17getWorstFrameTimeEv")]
// was: Ogre::RenderTarget::getWorstFrameTime(void)const
// IDA 0xd7c520: 4 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c520() {
}

// 0xd7c530 — __ZN4Ogre12RenderTarget15resetStatisticsEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::resetStatistics(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget15resetStatisticsEv")]
// was: Ogre::RenderTarget::resetStatistics(void)
// IDA 0xd7c530: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c530() {
}

// 0xd7c568 — __ZN4Ogre12RenderTarget18getCustomAttributeERKSsPv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, const std::string *, void *)
#[doc(alias = "Ogre::RenderTarget::getCustomAttribute(std::string const&,void *)")]
#[doc(alias = "__ZN4Ogre12RenderTarget18getCustomAttributeERKSsPv")]
// was: Ogre::RenderTarget::getCustomAttribute(std::string const&,void *)
// IDA 0xd7c568: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c568() {
}

// 0xd7c71c — __ZN4Ogre12RenderTarget11addListenerEPNS_20RenderTargetListenerE
#[doc(alias = "Ogre::RenderTarget::addListener(Ogre::RenderTargetListener *)")]
#[doc(alias = "__ZN4Ogre12RenderTarget11addListenerEPNS_20RenderTargetListenerE")]
// was: Ogre::RenderTarget::addListener(Ogre::RenderTargetListener *)
// IDA 0xd7c71c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c71c() {
}

// 0xd7c750 — __ZN4Ogre12RenderTarget14removeListenerEPNS_20RenderTargetListenerE
#[doc(alias = "Ogre::RenderTarget::removeListener(Ogre::RenderTargetListener *)")]
#[doc(alias = "__ZN4Ogre12RenderTarget14removeListenerEPNS_20RenderTargetListenerE")]
// was: Ogre::RenderTarget::removeListener(Ogre::RenderTargetListener *)
// IDA 0xd7c750: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c750() {
}

// 0xd7c784 — __ZN4Ogre12RenderTarget18removeAllListenersEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::removeAllListeners(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget18removeAllListenersEv")]
// was: Ogre::RenderTarget::removeAllListeners(void)
// IDA 0xd7c784: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c784() {
}

// 0xd7c78c — __ZN4Ogre12RenderTarget13firePreUpdateEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::firePreUpdate(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget13firePreUpdateEv")]
// was: Ogre::RenderTarget::firePreUpdate(void)
// IDA 0xd7c78c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c78c() {
}

// 0xd7c7b4 — __ZN4Ogre12RenderTarget14firePostUpdateEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::firePostUpdate(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget14firePostUpdateEv")]
// was: Ogre::RenderTarget::firePostUpdate(void)
// IDA 0xd7c7b4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7c7b4() {
}

// 0xd7cb14 — __ZNK4Ogre12RenderTarget8isActiveEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::isActive(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget8isActiveEv")]
// was: Ogre::RenderTarget::isActive(void)const
// IDA 0xd7cb14: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7cb14() {
}

// 0xd7cb1c — __ZN4Ogre12RenderTarget9setActiveEb
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, bool)
#[doc(alias = "Ogre::RenderTarget::setActive(bool)")]
#[doc(alias = "__ZN4Ogre12RenderTarget9setActiveEb")]
// was: Ogre::RenderTarget::setActive(bool)
// IDA 0xd7cb1c: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7cb1c() {
}

// 0xd7cc7c — __ZN4Ogre12RenderTarget30writeContentsToTimestampedFileERKSsS2_
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, const std::string *, const std::string *)
#[doc(alias = "Ogre::RenderTarget::writeContentsToTimestampedFile(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4Ogre12RenderTarget30writeContentsToTimestampedFileERKSsS2_")]
// was: Ogre::RenderTarget::writeContentsToTimestampedFile(std::string const&,std::string const&)
// IDA 0xd7cc7c: 624 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7cc7c() {
}

// 0xd7d324 — __ZN4Ogre12RenderTarget19writeContentsToFileERKSs
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, const std::string *)
#[doc(alias = "Ogre::RenderTarget::writeContentsToFile(std::string const&)")]
#[doc(alias = "__ZN4Ogre12RenderTarget19writeContentsToFileERKSs")]
// was: Ogre::RenderTarget::writeContentsToFile(std::string const&)
// IDA 0xd7d324: 121 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d324() {
}

// 0xd7d46c — __ZN4Ogre12RenderTarget20_notifyCameraRemovedEPKNS_6CameraE
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, const Ogre::Camera *)
#[doc(alias = "Ogre::RenderTarget::_notifyCameraRemoved(Ogre::Camera const*)")]
#[doc(alias = "__ZN4Ogre12RenderTarget20_notifyCameraRemovedEPKNS_6CameraE")]
// was: Ogre::RenderTarget::_notifyCameraRemoved(Ogre::Camera const*)
// IDA 0xd7d46c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d46c() {
}

// 0xd7d4a8 — __ZN4Ogre12RenderTarget14setAutoUpdatedEb
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, bool)
#[doc(alias = "Ogre::RenderTarget::setAutoUpdated(bool)")]
#[doc(alias = "__ZN4Ogre12RenderTarget14setAutoUpdatedEb")]
// was: Ogre::RenderTarget::setAutoUpdated(bool)
// IDA 0xd7d4a8: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4a8() {
}

// 0xd7d4b0 — __ZNK4Ogre12RenderTarget13isAutoUpdatedEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::isAutoUpdated(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget13isAutoUpdatedEv")]
// was: Ogre::RenderTarget::isAutoUpdated(void)const
// IDA 0xd7d4b0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4b0() {
}

// 0xd7d4b8 — __ZNK4Ogre12RenderTarget9isPrimaryEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::isPrimary(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget9isPrimaryEv")]
// was: Ogre::RenderTarget::isPrimary(void)const
// IDA 0xd7d4b8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4b8() {
}

// 0xd7d4bc — __ZN4Ogre12RenderTarget8_getImplEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::_getImpl(void)")]
#[doc(alias = "__ZN4Ogre12RenderTarget8_getImplEv")]
// was: Ogre::RenderTarget::_getImpl(void)
// IDA 0xd7d4bc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4bc() {
}

// 0xd7d4c0 — __ZN4Ogre12RenderTarget6updateEb
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, bool)
#[doc(alias = "Ogre::RenderTarget::update(bool)")]
#[doc(alias = "__ZN4Ogre12RenderTarget6updateEb")]
// was: Ogre::RenderTarget::update(bool)
// IDA 0xd7d4c0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4c0() {
}

// 0xd7d4f0 — __ZN4Ogre12RenderTarget11setPriorityEh
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, unsigned __int8)
#[doc(alias = "Ogre::RenderTarget::setPriority(unsigned char)")]
#[doc(alias = "__ZN4Ogre12RenderTarget11setPriorityEh")]
// was: Ogre::RenderTarget::setPriority(unsigned char)
// IDA 0xd7d4f0: 2 insns (STRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4f0() {
}

// 0xd7d4f4 — __ZNK4Ogre12RenderTarget18suggestPixelFormatEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::suggestPixelFormat(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget18suggestPixelFormatEv")]
// was: Ogre::RenderTarget::suggestPixelFormat(void)const
// IDA 0xd7d4f4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4f4() {
}

// 0xd7d4f8 — __ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_
#[doc(alias = "std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKS8_")]
// was: std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::vector(std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xd7d4f8: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d4f8() {
}

// 0xd7d56c — __ZNSt12_Vector_baseIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
// was: std::_Vector_base<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd7d56c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd7d56c() {
}

// 0xd7d570 — __ZNSt12_Vector_baseIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
// was: std::_Vector_base<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xd7d570: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7d570() {
}

// 0xd7d57c — __ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderTargetListener **,std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderTargetListener * const&)")]
#[doc(alias = "__ZNSt6vectorIPN4Ogre20RenderTargetListenerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_")]
// was: std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::RenderTargetListener **,std::vector<Ogre::RenderTargetListener *,Ogre::STLAllocator<Ogre::RenderTargetListener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::RenderTargetListener * const&)
// IDA 0xd7d57c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0xd7d57c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xd7d840 — __ZN4Ogre13RenderTextureC2EPNS_19HardwarePixelBufferEm
// type: _DWORD __fastcall(Ogre::RenderTexture *__hidden this, Ogre::HardwarePixelBuffer *, unsigned int)
#[doc(alias = "Ogre::RenderTexture::RenderTexture(Ogre::HardwarePixelBuffer *,unsigned long)")]
#[doc(alias = "__ZN4Ogre13RenderTextureC2EPNS_19HardwarePixelBufferEm")]
// was: Ogre::RenderTexture::RenderTexture(Ogre::HardwarePixelBuffer *,unsigned long)
// IDA 0xd7d840: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7d840() {
}

// 0xd7d91c — __ZN4Ogre13RenderTextureD0Ev
// type: void __fastcall(Ogre::RenderTexture *__hidden this)
#[doc(alias = "Ogre::RenderTexture::~RenderTexture()")]
#[doc(alias = "__ZN4Ogre13RenderTextureD0Ev")]
// was: Ogre::RenderTexture::~RenderTexture()
// IDA 0xd7d91c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7d91c() {
}

// 0xd7d9e4 — __ZN4Ogre13RenderTextureD1Ev
// type: void __fastcall(Ogre::RenderTexture *__hidden this)
#[doc(alias = "Ogre::RenderTexture::~RenderTexture()")]
#[doc(alias = "__ZN4Ogre13RenderTextureD1Ev")]
// was: Ogre::RenderTexture::~RenderTexture()
// IDA 0xd7d9e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7d9e4() {
}

// 0xd7da9c — __ZN4Ogre13RenderTextureD2Ev
// type: void __fastcall(Ogre::RenderTexture *__hidden this)
#[doc(alias = "Ogre::RenderTexture::~RenderTexture()")]
#[doc(alias = "__ZN4Ogre13RenderTextureD2Ev")]
// was: Ogre::RenderTexture::~RenderTexture()
// IDA 0xd7da9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7da9c() {
}

// 0xd7db54 — __ZN4Ogre13RenderTexture20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE
#[doc(alias = "Ogre::RenderTexture::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)")]
#[doc(alias = "__ZN4Ogre13RenderTexture20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE")]
// was: Ogre::RenderTexture::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)
// IDA 0xd7db54: 175 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7db54() {
}

// 0xd7dd54 — __ZNK4Ogre13RenderTexture18suggestPixelFormatEv
// type: _DWORD __fastcall(Ogre::RenderTexture *__hidden this)
#[doc(alias = "Ogre::RenderTexture::suggestPixelFormat(void)const")]
#[doc(alias = "__ZNK4Ogre13RenderTexture18suggestPixelFormatEv")]
// was: Ogre::RenderTexture::suggestPixelFormat(void)const
// IDA 0xd7dd54: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7dd54() {
}

// 0xd7dd5c — __ZN4Ogre17MultiRenderTargetC2ERKSs
// type: _DWORD __fastcall(Ogre::MultiRenderTarget *__hidden this, const std::string *)
#[doc(alias = "Ogre::MultiRenderTarget::MultiRenderTarget(std::string const&)")]
#[doc(alias = "__ZN4Ogre17MultiRenderTargetC2ERKSs")]
// was: Ogre::MultiRenderTarget::MultiRenderTarget(std::string const&)
// IDA 0xd7dd5c: 95 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7dd5c() {
}

// 0xd7de68 — __ZN4Ogre17MultiRenderTarget20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE
#[doc(alias = "Ogre::MultiRenderTarget::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)")]
#[doc(alias = "__ZN4Ogre17MultiRenderTarget20copyContentsToMemoryERKNS_8PixelBoxENS_12RenderTarget11FrameBufferE")]
// was: Ogre::MultiRenderTarget::copyContentsToMemory(Ogre::PixelBox const&,Ogre::RenderTarget::FrameBuffer)
// IDA 0xd7de68: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7de68() {
}

// 0xd7e018 — __ZN4Ogre17MultiRenderTargetD1Ev
// type: void __fastcall(Ogre::MultiRenderTarget *__hidden this)
#[doc(alias = "Ogre::MultiRenderTarget::~MultiRenderTarget()")]
#[doc(alias = "__ZN4Ogre17MultiRenderTargetD1Ev")]
// was: Ogre::MultiRenderTarget::~MultiRenderTarget()
// IDA 0xd7e018: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7e018() {
}

// 0xd7e0c4 — __ZN4Ogre17MultiRenderTargetD0Ev
// type: void __fastcall(Ogre::MultiRenderTarget *__hidden this)
#[doc(alias = "Ogre::MultiRenderTarget::~MultiRenderTarget()")]
#[doc(alias = "__ZN4Ogre17MultiRenderTargetD0Ev")]
// was: Ogre::MultiRenderTarget::~MultiRenderTarget()
// IDA 0xd7e0c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7e0c4() {
}

// 0xd7e178 — __ZNK4Ogre17MultiRenderTarget18suggestPixelFormatEv
// type: _DWORD __fastcall(Ogre::MultiRenderTarget *__hidden this)
#[doc(alias = "Ogre::MultiRenderTarget::suggestPixelFormat(void)const")]
#[doc(alias = "__ZNK4Ogre17MultiRenderTarget18suggestPixelFormatEv")]
// was: Ogre::MultiRenderTarget::suggestPixelFormat(void)const
// IDA 0xd7e178: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e178() {
}

// 0xd7e17c — __ZN4Ogre17MultiRenderTarget11bindSurfaceEmPNS_13RenderTextureE
// type: _DWORD __fastcall(Ogre::MultiRenderTarget *__hidden this, unsigned int, Ogre::RenderTexture *)
#[doc(alias = "Ogre::MultiRenderTarget::bindSurface(unsigned long,Ogre::RenderTexture *)")]
#[doc(alias = "__ZN4Ogre17MultiRenderTarget11bindSurfaceEmPNS_13RenderTextureE")]
// was: Ogre::MultiRenderTarget::bindSurface(unsigned long,Ogre::RenderTexture *)
// IDA 0xd7e17c: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e17c() {
}

// 0xd7e1f8 — __ZN4Ogre17MultiRenderTarget13unbindSurfaceEm
// type: _DWORD __fastcall(Ogre::MultiRenderTarget *__hidden this, unsigned int)
#[doc(alias = "Ogre::MultiRenderTarget::unbindSurface(unsigned long)")]
#[doc(alias = "__ZN4Ogre17MultiRenderTarget13unbindSurfaceEm")]
// was: Ogre::MultiRenderTarget::unbindSurface(unsigned long)
// IDA 0xd7e1f8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e1f8() {
}

// 0xd7e24c — __ZN4Ogre12RenderWindowC2Ev
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::RenderWindow(void)")]
#[doc(alias = "__ZN4Ogre12RenderWindowC2Ev")]
// was: Ogre::RenderWindow::RenderWindow(void)
// IDA 0xd7e24c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e24c() {
}

// 0xd7e270 — __ZN4Ogre12RenderWindow10getMetricsERjS1_S1_RiS2_
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this, unsigned int *, unsigned int *, unsigned int *, int *, int *)
#[doc(alias = "Ogre::RenderWindow::getMetrics(unsigned int &,unsigned int &,unsigned int &,int &,int &)")]
#[doc(alias = "__ZN4Ogre12RenderWindow10getMetricsERjS1_S1_RiS2_")]
// was: Ogre::RenderWindow::getMetrics(unsigned int &,unsigned int &,unsigned int &,int &,int &)
// IDA 0xd7e270: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e270() {
}

// 0xd7e2a0 — __ZNK4Ogre12RenderWindow12isFullScreenEv
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::isFullScreen(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderWindow12isFullScreenEv")]
// was: Ogre::RenderWindow::isFullScreen(void)const
// IDA 0xd7e2a0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e2a0() {
}

// 0xd7e2a8 — __ZNK4Ogre12RenderWindow9isPrimaryEv
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::isPrimary(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderWindow9isPrimaryEv")]
// was: Ogre::RenderWindow::isPrimary(void)const
// IDA 0xd7e2a8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e2a8() {
}

// 0xd7e2b4 — __ZN4Ogre12RenderWindowD1Ev
// type: void __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::~RenderWindow()")]
#[doc(alias = "__ZN4Ogre12RenderWindowD1Ev")]
// was: Ogre::RenderWindow::~RenderWindow()
// IDA 0xd7e2b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7e2b4() {
}

// 0xd7e2c0 — __ZN4Ogre12RenderWindowD0Ev
// type: void __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::~RenderWindow()")]
#[doc(alias = "__ZN4Ogre12RenderWindowD0Ev")]
// was: Ogre::RenderWindow::~RenderWindow()
// IDA 0xd7e2c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7e2c0() {
}

// 0xd7e350 — __ZN4Ogre12RenderTarget11swapBuffersEb
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, bool)
#[doc(alias = "Ogre::RenderTarget::swapBuffers(bool)")]
#[doc(alias = "__ZN4Ogre12RenderTarget11swapBuffersEb")]
// was: Ogre::RenderTarget::swapBuffers(bool)
// IDA 0xd7e350: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd7e350() {
}

// 0xd7e358 — __ZNK4Ogre12RenderTarget11getFSAAHintEv
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "Ogre::RenderTarget::getFSAAHint(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderTarget11getFSAAHintEv")]
// was: Ogre::RenderTarget::getFSAAHint(void)const
// IDA 0xd7e358: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e358() {
}

// 0xd7e35c — __ZN4Ogre12RenderWindow13setFullscreenEbjj
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this, bool, unsigned int, unsigned int)
#[doc(alias = "Ogre::RenderWindow::setFullscreen(bool,unsigned int,unsigned int)")]
#[doc(alias = "__ZN4Ogre12RenderWindow13setFullscreenEbjj")]
// was: Ogre::RenderWindow::setFullscreen(bool,unsigned int,unsigned int)
// IDA 0xd7e35c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd7e35c() {
}

// 0xd7e360 — __ZN4Ogre12RenderWindow20windowMovedOrResizedEv
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::windowMovedOrResized(void)")]
#[doc(alias = "__ZN4Ogre12RenderWindow20windowMovedOrResizedEv")]
// was: Ogre::RenderWindow::windowMovedOrResized(void)
// IDA 0xd7e360: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd7e360() {
}

// 0xd7e364 — __ZNK4Ogre12RenderWindow9isVisibleEv
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderWindow9isVisibleEv")]
// was: Ogre::RenderWindow::isVisible(void)const
// IDA 0xd7e364: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e364() {
}

// 0xd7e368 — __ZN4Ogre12RenderWindow10setVisibleEb
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this, bool)
#[doc(alias = "Ogre::RenderWindow::setVisible(bool)")]
#[doc(alias = "__ZN4Ogre12RenderWindow10setVisibleEb")]
// was: Ogre::RenderWindow::setVisible(bool)
// IDA 0xd7e368: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd7e368() {
}

// 0xd7e370 — __ZN4Ogre12RenderWindow9setHiddenEb
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this, bool)
#[doc(alias = "Ogre::RenderWindow::setHidden(bool)")]
#[doc(alias = "__ZN4Ogre12RenderWindow9setHiddenEb")]
// was: Ogre::RenderWindow::setHidden(bool)
// IDA 0xd7e370: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xd7e370() {
}

// 0xd7e378 — __ZNK4Ogre12RenderWindow14isVSyncEnabledEv
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::isVSyncEnabled(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderWindow14isVSyncEnabledEv")]
// was: Ogre::RenderWindow::isVSyncEnabled(void)const
// IDA 0xd7e378: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e378() {
}

// 0xd7e380 — __ZNK4Ogre12RenderWindow16getVSyncIntervalEv
// type: _DWORD __fastcall(Ogre::RenderWindow *__hidden this)
#[doc(alias = "Ogre::RenderWindow::getVSyncInterval(void)const")]
#[doc(alias = "__ZNK4Ogre12RenderWindow16getVSyncIntervalEv")]
// was: Ogre::RenderWindow::getVSyncInterval(void)const
// IDA 0xd7e380: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e380() {
}

// 0xd7e3c4 — __ZN4Ogre8ResourceC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, Ogre::ResourceManager *, const std::string *, unsigned __int64, const std::string *, bool, Ogre::ManualResourceLoader *)
#[doc(alias = "Ogre::Resource::Resource(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
#[doc(alias = "__ZN4Ogre8ResourceC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE")]
// was: Ogre::Resource::Resource(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xd7e3c4: 168 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e3c4() {
}

// 0xd7e58c — __ZN4Ogre8ResourceD0Ev
// type: void __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::~Resource()")]
#[doc(alias = "__ZN4Ogre8ResourceD0Ev")]
// was: Ogre::Resource::~Resource()
// IDA 0xd7e58c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7e58c() {
}

// 0xd7e61c — __ZN4Ogre8ResourceD1Ev
// type: void __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::~Resource()")]
#[doc(alias = "__ZN4Ogre8ResourceD1Ev")]
// was: Ogre::Resource::~Resource()
// IDA 0xd7e61c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7e61c() {
}

// 0xd7e628 — __ZN4Ogre8ResourceD2Ev
// type: void __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::~Resource()")]
#[doc(alias = "__ZN4Ogre8ResourceD2Ev")]
// was: Ogre::Resource::~Resource()
// IDA 0xd7e628: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xd7e628() {
}

// 0xd7e7bc — __ZN4Ogre8Resource15escalateLoadingEv
// type: _DWORD __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::escalateLoading(void)")]
#[doc(alias = "__ZN4Ogre8Resource15escalateLoadingEv")]
// was: Ogre::Resource::escalateLoading(void)
// IDA 0xd7e7bc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e7bc() {
}

// 0xd7e7dc — __ZN4Ogre8Resource7prepareEb
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, bool)
#[doc(alias = "Ogre::Resource::prepare(bool)")]
#[doc(alias = "__ZN4Ogre8Resource7prepareEb")]
// was: Ogre::Resource::prepare(bool)
// IDA 0xd7e7dc: 359 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7e7dc() {
}

// 0xd7ebb0 — __ZN4Ogre8Resource4loadEb
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, bool)
#[doc(alias = "Ogre::Resource::load(bool)")]
#[doc(alias = "__ZN4Ogre8Resource4loadEb")]
// was: Ogre::Resource::load(bool)
// IDA 0xd7ebb0: 440 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7ebb0() {
}

// 0xd7f034 — __ZN4Ogre8Resource11_dirtyStateEv
// type: _DWORD __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::_dirtyState(void)")]
#[doc(alias = "__ZN4Ogre8Resource11_dirtyStateEv")]
// was: Ogre::Resource::_dirtyState(void)
// IDA 0xd7f034: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f034() {
}

// 0xd7f03c — __ZN4Ogre8Resource20changeGroupOwnershipERKSs
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, const std::string *)
#[doc(alias = "Ogre::Resource::changeGroupOwnership(std::string const&)")]
#[doc(alias = "__ZN4Ogre8Resource20changeGroupOwnershipERKSs")]
// was: Ogre::Resource::changeGroupOwnership(std::string const&)
// IDA 0xd7f03c: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f03c() {
}

// 0xd7f1a4 — __ZN4Ogre8Resource6unloadEv
// type: _DWORD __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::unload(void)")]
#[doc(alias = "__ZN4Ogre8Resource6unloadEv")]
// was: Ogre::Resource::unload(void)
// IDA 0xd7f1a4: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f1a4() {
}

// 0xd7f208 — __ZN4Ogre8Resource6reloadEv
// type: _DWORD __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::reload(void)")]
#[doc(alias = "__ZN4Ogre8Resource6reloadEv")]
// was: Ogre::Resource::reload(void)
// IDA 0xd7f208: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f208() {
}

// 0xd7f250 — __ZN4Ogre8Resource11addListenerEPNS0_8ListenerE
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, Ogre::Resource::Listener *)
#[doc(alias = "Ogre::Resource::addListener(Ogre::Resource::Listener *)")]
#[doc(alias = "__ZN4Ogre8Resource11addListenerEPNS0_8ListenerE")]
// was: Ogre::Resource::addListener(Ogre::Resource::Listener *)
// IDA 0xd7f250: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f250() {
}

// 0xd7f268 — __ZN4Ogre8Resource14removeListenerEPNS0_8ListenerE
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, Ogre::Resource::Listener *)
#[doc(alias = "Ogre::Resource::removeListener(Ogre::Resource::Listener *)")]
#[doc(alias = "__ZN4Ogre8Resource14removeListenerEPNS0_8ListenerE")]
// was: Ogre::Resource::removeListener(Ogre::Resource::Listener *)
// IDA 0xd7f268: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f268() {
}

// 0xd7f2b4 — __ZN4Ogre8Resource20_fireLoadingCompleteEb
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, bool)
#[doc(alias = "Ogre::Resource::_fireLoadingComplete(bool)")]
#[doc(alias = "__ZN4Ogre8Resource20_fireLoadingCompleteEb")]
// was: Ogre::Resource::_fireLoadingComplete(bool)
// IDA 0xd7f2b4: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f2b4() {
}

// 0xd7f2f8 — __ZN4Ogre8Resource22_firePreparingCompleteEb
// type: _DWORD __fastcall(Ogre::Resource *__hidden this, bool)
#[doc(alias = "Ogre::Resource::_firePreparingComplete(bool)")]
#[doc(alias = "__ZN4Ogre8Resource22_firePreparingCompleteEb")]
// was: Ogre::Resource::_firePreparingComplete(bool)
// IDA 0xd7f2f8: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f2f8() {
}

// 0xd7f33c — __ZN4Ogre8Resource22_fireUnloadingCompleteEv
// type: _DWORD __fastcall(Ogre::Resource *__hidden this)
#[doc(alias = "Ogre::Resource::_fireUnloadingComplete(void)")]
#[doc(alias = "__ZN4Ogre8Resource22_fireUnloadingCompleteEv")]
// was: Ogre::Resource::_fireUnloadingComplete(void)
// IDA 0xd7f33c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f33c() {
}

// 0xd7f364 — __ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS3_ESF_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Resource::Listener *>,std::_Rb_tree_iterator<Ogre::Resource::Listener *>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS3_ESF_")]
// was: std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Resource::Listener *>,std::_Rb_tree_iterator<Ogre::Resource::Listener *>)
// IDA 0xd7f364: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f364() {
}

// 0xd7f3c8 — __ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E
#[doc(alias = "std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Resource::Listener *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS3_E")]
// was: std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Resource::Listener *> *)
// IDA 0xd7f3c8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f3c8() {
}

// 0xd7f3f0 — __ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Resource::Listener * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN4Ogre8Resource8ListenerES3_St9_IdentityIS3_ESt4lessIS3_ENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS3_")]
// was: std::_Rb_tree<Ogre::Resource::Listener *,Ogre::Resource::Listener *,std::_Identity<Ogre::Resource::Listener *>,std::less<Ogre::Resource::Listener *>,Ogre::STLAllocator<Ogre::Resource::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Resource::Listener * const&)
// IDA 0xd7f3f0: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xd7f3f0() {
}
