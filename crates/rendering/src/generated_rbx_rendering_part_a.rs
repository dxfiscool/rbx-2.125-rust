//! rbx-rendering part A — Ogre namespace EA-sorted (120 stubs)
//! Filter: Ogre namespace, EA-sorted asc, global dedup skipped 120 prior
//! Range: 0xe6f14c..0xe956e4 (120 stubs, 1143 remaining Ogre before, 1023 after)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe6f14c — __ZN4Ogre16DefaultWorkQueueC1ERKSs
#[doc(alias = "Ogre::DefaultWorkQueue::DefaultWorkQueue(std::string const&)")]
// was: Ogre::DefaultWorkQueue::DefaultWorkQueue(std::string const&)
// IDA 0xe6f14c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6f14c() {
}

// 0xe6f168 — __ZN4Ogre16DefaultWorkQueueD0Ev
#[doc(alias = "Ogre::DefaultWorkQueue::~DefaultWorkQueue()")]
// was: Ogre::DefaultWorkQueue::~DefaultWorkQueue()
// IDA 0xe6f168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e6f168() {
}

// 0xe6f228 — __ZN4Ogre16DefaultWorkQueueD1Ev
#[doc(alias = "Ogre::DefaultWorkQueue::~DefaultWorkQueue()")]
// was: Ogre::DefaultWorkQueue::~DefaultWorkQueue()
// IDA 0xe6f228: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e6f228() {
}

// 0xe6f2d8 — __ZN4Ogre16DefaultWorkQueue7startupEb
#[doc(alias = "Ogre::DefaultWorkQueue::startup(bool)")]
// was: Ogre::DefaultWorkQueue::startup(bool)
// IDA 0xe6f2d8: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6f2d8() {
}

// 0xe6f448 — __ZN4Ogre16DefaultWorkQueue8shutdownEv
#[doc(alias = "Ogre::DefaultWorkQueue::shutdown(void)")]
// was: Ogre::DefaultWorkQueue::shutdown(void)
// IDA 0xe6f448: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6f448() {
}

// 0xe6f5a0 — __ZN4Ogre16DefaultWorkQueue13notifyWorkersEv
#[doc(alias = "Ogre::DefaultWorkQueue::notifyWorkers(void)")]
// was: Ogre::DefaultWorkQueue::notifyWorkers(void)
// IDA 0xe6f5a0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e6f5a0() {
}

// 0xe6f5a4 — __ZN4Ogre16DefaultWorkQueue18waitForNextRequestEv
#[doc(alias = "Ogre::DefaultWorkQueue::waitForNextRequest(void)")]
// was: Ogre::DefaultWorkQueue::waitForNextRequest(void)
// IDA 0xe6f5a4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e6f5a4() {
}

// 0xe6f5ac — __ZNK4Ogre20DefaultWorkQueueBase30getResponseProcessingTimeLimitEv
#[doc(alias = "Ogre::DefaultWorkQueueBase::getResponseProcessingTimeLimit(void)const")]
// was: Ogre::DefaultWorkQueueBase::getResponseProcessingTimeLimit(void)const
// IDA 0xe6f5ac: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6f5ac() {
}

// 0xe6f5b0 — __ZN4Ogre20DefaultWorkQueueBase30setResponseProcessingTimeLimitEm
#[doc(alias = "Ogre::DefaultWorkQueueBase::setResponseProcessingTimeLimit(unsigned long)")]
// was: Ogre::DefaultWorkQueueBase::setResponseProcessingTimeLimit(unsigned long)
// IDA 0xe6f5b0: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6f5b0() {
}

// 0xe6f5b4 — __ZNK4Ogre20DefaultWorkQueueBase14isShuttingDownEv
#[doc(alias = "Ogre::DefaultWorkQueueBase::isShuttingDown(void)const")]
// was: Ogre::DefaultWorkQueueBase::isShuttingDown(void)const
// IDA 0xe6f5b4: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e6f5b4() {
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

// 0xe7ed70 — __ZN4Ogre21RenderingAPIExceptionD1Ev
#[doc(alias = "Ogre::RenderingAPIException::~RenderingAPIException()")]
// was: Ogre::RenderingAPIException::~RenderingAPIException()
// IDA 0xe7ed70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7ed70() {
}

// 0xe7ed80 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::GpuProgramParameters>::~SharedPtr()
// IDA 0xe7ed80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7ed80() {
}

// 0xe7f358 — __ZN4Ogre24GpuSharedParametersUsageD2Ev
#[doc(alias = "Ogre::GpuSharedParametersUsage::~GpuSharedParametersUsage()")]
// was: Ogre::GpuSharedParametersUsage::~GpuSharedParametersUsage()
// IDA 0xe7f358: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f358() {
}

// 0xe7f710 — __ZN4Ogre21RenderingAPIExceptionD0Ev
#[doc(alias = "Ogre::RenderingAPIException::~RenderingAPIException()")]
// was: Ogre::RenderingAPIException::~RenderingAPIException()
// IDA 0xe7f710: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e7f710() {
}

// 0xe83718 — __ZN4Ogre9SharedPtrINS_19HardwarePixelBufferEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwarePixelBuffer> const&)")]
// was: Ogre::SharedPtr<Ogre::HardwarePixelBuffer>::operator=(Ogre::SharedPtr<Ogre::HardwarePixelBuffer> const&)
// IDA 0xe83718: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e83718() {
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

// 0xe8ea74 — __ZN4Ogre13logObjectInfoERKSsj
#[doc(alias = "Ogre::logObjectInfo(std::string const&,unsigned int)")]
// was: Ogre::logObjectInfo(std::string const&,unsigned int)
// IDA 0xe8ea74: 380 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8ea74() {
}

// 0xe8eeb8 — __ZN4Ogre16GLSLESGpuProgramC1EPNS_13GLSLESProgramE
#[doc(alias = "Ogre::GLSLESGpuProgram::GLSLESGpuProgram(Ogre::GLSLESProgram *)")]
// was: Ogre::GLSLESGpuProgram::GLSLESGpuProgram(Ogre::GLSLESProgram *)
// IDA 0xe8eeb8: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8eeb8() {
}

// 0xe8eec4 — __ZN4Ogre16GLSLESGpuProgramC2EPNS_13GLSLESProgramE
#[doc(alias = "Ogre::GLSLESGpuProgram::GLSLESGpuProgram(Ogre::GLSLESProgram *)")]
// was: Ogre::GLSLESGpuProgram::GLSLESGpuProgram(Ogre::GLSLESProgram *)
// IDA 0xe8eec4: 145 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8eec4() {
}

// 0xe8f054 — __ZN4Ogre16GLSLESGpuProgramD0Ev
#[doc(alias = "Ogre::GLSLESGpuProgram::~GLSLESGpuProgram()")]
// was: Ogre::GLSLESGpuProgram::~GLSLESGpuProgram()
// IDA 0xe8f054: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8f054() {
}

// 0xe8f114 — __ZN4Ogre16GLSLESGpuProgramD1Ev
#[doc(alias = "Ogre::GLSLESGpuProgram::~GLSLESGpuProgram()")]
// was: Ogre::GLSLESGpuProgram::~GLSLESGpuProgram()
// IDA 0xe8f114: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8f114() {
}

// 0xe8f1c4 — __ZN4Ogre16GLSLESGpuProgram8loadImplEv
#[doc(alias = "Ogre::GLSLESGpuProgram::loadImpl(void)")]
// was: Ogre::GLSLESGpuProgram::loadImpl(void)
// IDA 0xe8f1c4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8f1c4() {
}

// 0xe8f1c8 — __ZN4Ogre16GLSLESGpuProgram10unloadImplEv
#[doc(alias = "Ogre::GLSLESGpuProgram::unloadImpl(void)")]
// was: Ogre::GLSLESGpuProgram::unloadImpl(void)
// IDA 0xe8f1c8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8f1c8() {
}

// 0xe8f1cc — __ZN4Ogre16GLSLESGpuProgram14loadFromSourceEv
#[doc(alias = "Ogre::GLSLESGpuProgram::loadFromSource(void)")]
// was: Ogre::GLSLESGpuProgram::loadFromSource(void)
// IDA 0xe8f1cc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e8f1cc() {
}

// 0xe8f1d0 — __ZN4Ogre16GLSLESGpuProgram11bindProgramEv
#[doc(alias = "Ogre::GLSLESGpuProgram::bindProgram(void)")]
// was: Ogre::GLSLESGpuProgram::bindProgram(void)
// IDA 0xe8f1d0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8f1d0() {
}

// 0xe8f230 — __ZN4Ogre16GLSLESGpuProgram13unbindProgramEv
#[doc(alias = "Ogre::GLSLESGpuProgram::unbindProgram(void)")]
// was: Ogre::GLSLESGpuProgram::unbindProgram(void)
// IDA 0xe8f230: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8f230() {
}

// 0xe8f28c — __ZN4Ogre16GLSLESGpuProgram21bindProgramParametersENS_9SharedPtrINS_20GpuProgramParametersEEEt
#[doc(alias = "Ogre::GLSLESGpuProgram::bindProgramParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short)")]
// was: Ogre::GLSLESGpuProgram::bindProgramParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short)
// IDA 0xe8f28c: 206 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8f28c() {
}

// 0xe8f4a0 — __ZN4Ogre16GLSLESGpuProgram34bindProgramPassIterationParametersENS_9SharedPtrINS_20GpuProgramParametersEEE
#[doc(alias = "Ogre::GLSLESGpuProgram::bindProgramPassIterationParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
// was: Ogre::GLSLESGpuProgram::bindProgramPassIterationParameters(Ogre::SharedPtr<Ogre::GpuProgramParameters>)
// IDA 0xe8f4a0: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8f4a0() {
}

// 0xe8f698 — __ZN4Ogre17GLSLESLinkProgramC1EPNS_16GLSLESGpuProgramES2_
#[doc(alias = "Ogre::GLSLESLinkProgram::GLSLESLinkProgram(Ogre::GLSLESGpuProgram *,Ogre::GLSLESGpuProgram *)")]
// was: Ogre::GLSLESLinkProgram::GLSLESLinkProgram(Ogre::GLSLESGpuProgram *,Ogre::GLSLESGpuProgram *)
// IDA 0xe8f698: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8f698() {
}

// 0xe8f6a4 — __ZN4Ogre17GLSLESLinkProgramC2EPNS_16GLSLESGpuProgramES2_
#[doc(alias = "Ogre::GLSLESLinkProgram::GLSLESLinkProgram(Ogre::GLSLESGpuProgram *,Ogre::GLSLESGpuProgram *)")]
// was: Ogre::GLSLESLinkProgram::GLSLESLinkProgram(Ogre::GLSLESGpuProgram *,Ogre::GLSLESGpuProgram *)
// IDA 0xe8f6a4: 174 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8f6a4() {
}

// 0xe8f8a4 — __ZN4Ogre17GLSLESLinkProgramD0Ev
#[doc(alias = "Ogre::GLSLESLinkProgram::~GLSLESLinkProgram()")]
// was: Ogre::GLSLESLinkProgram::~GLSLESLinkProgram()
// IDA 0xe8f8a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8f8a4() {
}

// 0xe8f958 — __ZN4Ogre17GLSLESLinkProgramD1Ev
#[doc(alias = "Ogre::GLSLESLinkProgram::~GLSLESLinkProgram()")]
// was: Ogre::GLSLESLinkProgram::~GLSLESLinkProgram()
// IDA 0xe8f958: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e8f958() {
}

// 0xe8fa0c — __ZN4Ogre17GLSLESLinkProgram11_useProgramEv
#[doc(alias = "Ogre::GLSLESLinkProgram::_useProgram(void)")]
// was: Ogre::GLSLESLinkProgram::_useProgram(void)
// IDA 0xe8fa0c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8fa0c() {
}

// 0xe8fa20 — __ZN4Ogre17GLSLESLinkProgram8activateEv
#[doc(alias = "Ogre::GLSLESLinkProgram::activate(void)")]
// was: Ogre::GLSLESLinkProgram::activate(void)
// IDA 0xe8fa20: 183 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8fa20() {
}

// 0xe8fc20 — __ZN4Ogre17GLSLESLinkProgram14compileAndLinkEv
#[doc(alias = "Ogre::GLSLESLinkProgram::compileAndLink(void)")]
// was: Ogre::GLSLESLinkProgram::compileAndLink(void)
// IDA 0xe8fc20: 513 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e8fc20() {
}

// 0xe901c8 — __ZN4Ogre17GLSLESLinkProgram14updateUniformsENS_9SharedPtrINS_20GpuProgramParametersEEEtNS_14GpuProgramTypeE
#[doc(alias = "Ogre::GLSLESLinkProgram::updateUniforms(Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short,Ogre::GpuProgramType)")]
// was: Ogre::GLSLESLinkProgram::updateUniforms(Ogre::SharedPtr<Ogre::GpuProgramParameters>,unsigned short,Ogre::GpuProgramType)
// IDA 0xe901c8: 114 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e901c8() {
}

// 0xe90304 — __ZN4Ogre17GLSLESLinkProgram27updatePassIterationUniformsENS_9SharedPtrINS_20GpuProgramParametersEEE
#[doc(alias = "Ogre::GLSLESLinkProgram::updatePassIterationUniforms(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
// was: Ogre::GLSLESLinkProgram::updatePassIterationUniforms(Ogre::SharedPtr<Ogre::GpuProgramParameters>)
// IDA 0xe90304: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e90304() {
}

// 0xe90340 — __ZN4Ogre17GLSLESLinkProgram23extractLayoutQualifiersEv
#[doc(alias = "Ogre::GLSLESLinkProgram::extractLayoutQualifiers(void)")]
// was: Ogre::GLSLESLinkProgram::extractLayoutQualifiers(void)
// IDA 0xe90340: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e90340() {
}

// 0xe9045c — __ZNK4Ogre13CPreprocessor5Token8GetValueERl
#[doc(alias = "Ogre::CPreprocessor::Token::GetValue(long &)const")]
// was: Ogre::CPreprocessor::Token::GetValue(long &)const
// IDA 0xe9045c: 109 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e9045c() {
}

// 0xe9057c — __ZN4Ogre13CPreprocessor5Token8AppendNLEi
#[doc(alias = "Ogre::CPreprocessor::Token::AppendNL(int)")]
// was: Ogre::CPreprocessor::Token::AppendNL(int)
// IDA 0xe9057c: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e9057c() {
}

// 0xe90878 — __ZN4Ogre13CPreprocessor6DefineEPKcmS2_m
#[doc(alias = "Ogre::CPreprocessor::Define(char const*,unsigned long,char const*,unsigned long)")]
// was: Ogre::CPreprocessor::Define(char const*,unsigned long,char const*,unsigned long)
// IDA 0xe90878: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e90878() {
}

// 0xe908c4 — __ZN4Ogre13CPreprocessor5ParseERKNS0_5TokenE
#[doc(alias = "Ogre::CPreprocessor::Parse(Ogre::CPreprocessor::Token const&)")]
// was: Ogre::CPreprocessor::Parse(Ogre::CPreprocessor::Token const&)
// IDA 0xe908c4: 330 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e908c4() {
}

// 0xe90bf4 — __ZN4Ogre13CPreprocessorD1Ev
#[doc(alias = "Ogre::CPreprocessor::~CPreprocessor()")]
// was: Ogre::CPreprocessor::~CPreprocessor()
// IDA 0xe90bf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e90bf4() {
}

// 0xe90c1c — __ZN4OgreL12DefaultErrorEPviPKcS2_m
#[doc(alias = "Ogre::DefaultError(void *,int,char const*,char const*,unsigned long)")]
// was: Ogre::DefaultError(void *,int,char const*,char const*,unsigned long)
// IDA 0xe90c1c: 138 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e90c1c() {
}

// 0xe90db0 — __ZN4Ogre13CPreprocessorD0Ev
#[doc(alias = "Ogre::CPreprocessor::~CPreprocessor()")]
// was: Ogre::CPreprocessor::~CPreprocessor()
// IDA 0xe90db0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e90db0() {
}

// 0xe90ddc — __ZN4Ogre13CPreprocessor8GetTokenEb
#[doc(alias = "Ogre::CPreprocessor::GetToken(bool)")]
// was: Ogre::CPreprocessor::GetToken(bool)
// IDA 0xe90ddc: 534 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e90ddc() {
}

// 0xe9151c — __ZN4Ogre13CPreprocessor12GetArgumentsERiRPNS0_5TokenEb
#[doc(alias = "Ogre::CPreprocessor::GetArguments(int &,Ogre::CPreprocessor::Token *&,bool)")]
// was: Ogre::CPreprocessor::GetArguments(int &,Ogre::CPreprocessor::Token *&,bool)
// IDA 0xe9151c: 546 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e9151c() {
}

// 0xe91a60 — __ZN4Ogre13CPreprocessor13GetExpressionERNS0_5TokenEii
#[doc(alias = "Ogre::CPreprocessor::GetExpression(Ogre::CPreprocessor::Token &,int,int)")]
// was: Ogre::CPreprocessor::GetExpression(Ogre::CPreprocessor::Token &,int,int)
// IDA 0xe91a60: 909 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e91a60() {
}

// 0xe92348 — __ZN4Ogre13CPreprocessor8GetValueERKNS0_5TokenERli
#[doc(alias = "Ogre::CPreprocessor::GetValue(Ogre::CPreprocessor::Token const&,long &,int)")]
// was: Ogre::CPreprocessor::GetValue(Ogre::CPreprocessor::Token const&,long &,int)
// IDA 0xe92348: 341 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e92348() {
}

// 0xe926a0 — __ZN4Ogre13CPreprocessor11GetArgumentERNS0_5TokenEb
#[doc(alias = "Ogre::CPreprocessor::GetArgument(Ogre::CPreprocessor::Token &,bool)")]
// was: Ogre::CPreprocessor::GetArgument(Ogre::CPreprocessor::Token &,bool)
// IDA 0xe926a0: 260 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e926a0() {
}

// 0xe92958 — __ZN4Ogre13CPreprocessor12HandleDefineERNS0_5TokenEi
#[doc(alias = "Ogre::CPreprocessor::HandleDefine(Ogre::CPreprocessor::Token &,int)")]
// was: Ogre::CPreprocessor::HandleDefine(Ogre::CPreprocessor::Token &,int)
// IDA 0xe92958: 258 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e92958() {
}

// 0xe92bd4 — __ZN4Ogre13CPreprocessor11HandleUnDefERNS0_5TokenEi
#[doc(alias = "Ogre::CPreprocessor::HandleUnDef(Ogre::CPreprocessor::Token &,int)")]
// was: Ogre::CPreprocessor::HandleUnDef(Ogre::CPreprocessor::Token &,int)
// IDA 0xe92bd4: 203 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e92bd4() {
}

// 0xe92ddc — __ZN4Ogre13CPreprocessor11HandleIfDefERNS0_5TokenEi
#[doc(alias = "Ogre::CPreprocessor::HandleIfDef(Ogre::CPreprocessor::Token &,int)")]
// was: Ogre::CPreprocessor::HandleIfDef(Ogre::CPreprocessor::Token &,int)
// IDA 0xe92ddc: 234 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e92ddc() {
}

// 0xe93038 — __ZN4Ogre13CPreprocessor13ExpandDefinedEPS0_iPNS0_5TokenE
#[doc(alias = "Ogre::CPreprocessor::ExpandDefined(Ogre::CPreprocessor*,int,Ogre::CPreprocessor::Token *)")]
// was: Ogre::CPreprocessor::ExpandDefined(Ogre::CPreprocessor*,int,Ogre::CPreprocessor::Token *)
// IDA 0xe93038: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e93038() {
}

// 0xe930dc — __ZN4Ogre13CPreprocessor8HandleIfERNS0_5TokenEi
#[doc(alias = "Ogre::CPreprocessor::HandleIf(Ogre::CPreprocessor::Token &,int)")]
// was: Ogre::CPreprocessor::HandleIf(Ogre::CPreprocessor::Token &,int)
// IDA 0xe930dc: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e930dc() {
}

// 0xe931e8 — __ZN4Ogre13CPreprocessor15HandleDirectiveERNS0_5TokenEi
#[doc(alias = "Ogre::CPreprocessor::HandleDirective(Ogre::CPreprocessor::Token &,int)")]
// was: Ogre::CPreprocessor::HandleDirective(Ogre::CPreprocessor::Token &,int)
// IDA 0xe931e8: 447 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e931e8() {
}

// 0xe93660 — __ZN4Ogre13CPreprocessor6DefineEPKcml
#[doc(alias = "Ogre::CPreprocessor::Define(char const*,unsigned long,long)")]
// was: Ogre::CPreprocessor::Define(char const*,unsigned long,long)
// IDA 0xe93660: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e93660() {
}

// 0xe93724 — __ZN4Ogre13CPreprocessor5ParseEPKcmRm
#[doc(alias = "Ogre::CPreprocessor::Parse(char const*,unsigned long,unsigned long &)")]
// was: Ogre::CPreprocessor::Parse(char const*,unsigned long,unsigned long &)
// IDA 0xe93724: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e93724() {
}

// 0xe938a4 — __ZN4Ogre13GLSLESProgramC1EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::GLSLESProgram::GLSLESProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::GLSLESProgram::GLSLESProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe938a4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e938a4() {
}

// 0xe938c0 — __ZN4Ogre13GLSLESProgramC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
#[doc(alias = "Ogre::GLSLESProgram::GLSLESProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
// was: Ogre::GLSLESProgram::GLSLESProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)
// IDA 0xe938c0: 388 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e938c0() {
}

// 0xe93d08 — __ZN4Ogre13GLSLESProgramD0Ev
#[doc(alias = "Ogre::GLSLESProgram::~GLSLESProgram()")]
// was: Ogre::GLSLESProgram::~GLSLESProgram()
// IDA 0xe93d08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e93d08() {
}

// 0xe93d98 — __ZN4Ogre13GLSLESProgramD1Ev
#[doc(alias = "Ogre::GLSLESProgram::~GLSLESProgram()")]
// was: Ogre::GLSLESProgram::~GLSLESProgram()
// IDA 0xe93d98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e93d98() {
}

// 0xe93da4 — __ZN4Ogre13GLSLESProgramD2Ev
#[doc(alias = "Ogre::GLSLESProgram::~GLSLESProgram()")]
// was: Ogre::GLSLESProgram::~GLSLESProgram()
// IDA 0xe93da4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e93da4() {
}

// 0xe93ef4 — __ZN4Ogre13GLSLESProgram14loadFromSourceEv
#[doc(alias = "Ogre::GLSLESProgram::loadFromSource(void)")]
// was: Ogre::GLSLESProgram::loadFromSource(void)
// IDA 0xe93ef4: 375 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e93ef4() {
}

// 0xe94314 — __ZN4Ogre13GLSLESProgram7compileEb
#[doc(alias = "Ogre::GLSLESProgram::compile(bool)")]
// was: Ogre::GLSLESProgram::compile(bool)
// IDA 0xe94314: 504 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e94314() {
}

// 0xe948bc — __ZN4Ogre13GLSLESProgram39checkAndFixInvalidDefaultPrecisionErrorERSs
#[doc(alias = "Ogre::GLSLESProgram::checkAndFixInvalidDefaultPrecisionError(std::string &)")]
// was: Ogre::GLSLESProgram::checkAndFixInvalidDefaultPrecisionError(std::string &)
// IDA 0xe948bc: 1012 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e948bc() {
}

// 0xe95410 — __ZN4Ogre13GLSLESProgram18createLowLevelImplEv
#[doc(alias = "Ogre::GLSLESProgram::createLowLevelImpl(void)")]
// was: Ogre::GLSLESProgram::createLowLevelImpl(void)
// IDA 0xe95410: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e95410() {
}

// 0xe955f8 — __ZN4Ogre13GLSLESProgram10unloadImplEv
#[doc(alias = "Ogre::GLSLESProgram::unloadImpl(void)")]
// was: Ogre::GLSLESProgram::unloadImpl(void)
// IDA 0xe955f8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e955f8() {
}

// 0xe95630 — __ZN4Ogre13GLSLESProgram19unloadHighLevelImplEv
#[doc(alias = "Ogre::GLSLESProgram::unloadHighLevelImpl(void)")]
// was: Ogre::GLSLESProgram::unloadHighLevelImpl(void)
// IDA 0xe95630: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e95630() {
}

// 0xe95670 — __ZN4Ogre13GLSLESProgram22populateParameterNamesENS_9SharedPtrINS_20GpuProgramParametersEEE
#[doc(alias = "Ogre::GLSLESProgram::populateParameterNames(Ogre::SharedPtr<Ogre::GpuProgramParameters>)")]
// was: Ogre::GLSLESProgram::populateParameterNames(Ogre::SharedPtr<Ogre::GpuProgramParameters>)
// IDA 0xe95670: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e95670() {
}

// 0xe9568c — __ZNK4Ogre13GLSLESProgram24buildConstantDefinitionsEv
#[doc(alias = "Ogre::GLSLESProgram::buildConstantDefinitions(void)const")]
// was: Ogre::GLSLESProgram::buildConstantDefinitions(void)const
// IDA 0xe9568c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e9568c() {
}

// 0xe956d4 — __ZNK4Ogre13GLSLESProgram22CmdPreprocessorDefines5doGetEPKv
#[doc(alias = "Ogre::GLSLESProgram::CmdPreprocessorDefines::doGet(void const*)const")]
// was: Ogre::GLSLESProgram::CmdPreprocessorDefines::doGet(void const*)const
// IDA 0xe956d4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e956d4() {
}

// 0xe956e4 — __ZN4Ogre13GLSLESProgram22CmdPreprocessorDefines5doSetEPvRKSs
#[doc(alias = "Ogre::GLSLESProgram::CmdPreprocessorDefines::doSet(void *,std::string const&)")]
// was: Ogre::GLSLESProgram::CmdPreprocessorDefines::doSet(void *,std::string const&)
// IDA 0xe956e4: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e956e4() {
}
