//! rendering — generated_watchdog_rend_a — 120 stubs Ogre namespace core EA-sorted asc
//! Source: ida/export.json (85545 funcs) filtered Ogre core (6911 candidates) -> 120 lowest EAs (global dedup attempted, 0 remaining so taking lowest)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3ecd0 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE")]
// IDA 0x3ecd0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3ecd0() {
}

// 0x3ecd4 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE")]
// IDA 0x3ecd4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3ecd4() {
}

// 0x3ecd8 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE")]
// IDA 0x3ecd8: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_3ecd8() {
}

// 0x3ecec — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
// type: _DWORD __fastcall(Ogre::WindowEventListener *__hidden this, RenderWindow *)
#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
#[doc(alias = "__ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE")]
// IDA 0x3ecec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3ecec() {
}

// 0xb6f208 — __ZN4Ogre16ShadowRenderableD2Ev
// type: void __fastcall(Ogre::ShadowRenderable *__hidden this)
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
#[doc(alias = "__ZN4Ogre16ShadowRenderableD2Ev")]
// IDA 0xb6f208: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b6f208() {
}

// 0xb70fe0 — __ZNK4Ogre10Renderable12getTechniqueEv
// type: int __fastcall(Ogre::Renderable *this)
#[doc(alias = "Ogre::Renderable::getTechnique(void)const")]
#[doc(alias = "__ZNK4Ogre10Renderable12getTechniqueEv")]
// IDA 0xb70fe0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b70fe0() {
}

// 0xb71000 — __ZNK4Ogre16ShadowRenderable19getSquaredViewDepthEPKNS_6CameraE
// type: int()
#[doc(alias = "Ogre::ShadowRenderable::getSquaredViewDepth(Ogre::Camera const*)const")]
#[doc(alias = "__ZNK4Ogre16ShadowRenderable19getSquaredViewDepthEPKNS_6CameraE")]
// IDA 0xb71000: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b71000() {
}

// 0xb71008 — __ZNK4Ogre16ShadowRenderable9isVisibleEv
// type: int __fastcall(Ogre::ShadowRenderable *this)
#[doc(alias = "Ogre::ShadowRenderable::isVisible(void)const")]
#[doc(alias = "__ZNK4Ogre16ShadowRenderable9isVisibleEv")]
// IDA 0xb71008: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b71008() {
}

// 0xb74238 — __ZN4Ogre28HardwareIndexBufferSharedPtrD1Ev
// type: void __fastcall(Ogre::HardwareIndexBufferSharedPtr *__hidden this)
#[doc(alias = "Ogre::HardwareIndexBufferSharedPtr::~HardwareIndexBufferSharedPtr()")]
#[doc(alias = "__ZN4Ogre28HardwareIndexBufferSharedPtrD1Ev")]
// IDA 0xb74238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b74238() {
}

// 0xb74290 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEED0Ev")]
// IDA 0xb74290: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b74290() {
}

// 0xb74330 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE7destroyEv
// type: int __fastcall(int, void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE7destroyEv")]
// IDA 0xb74330: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b74330() {
}

// 0xb74368 — __ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::HardwareIndexBuffer>::swap(Ogre::SharedPtr<Ogre::HardwareIndexBuffer>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19HardwareIndexBufferEE4swapERS2_")]
// IDA 0xb74368: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b74368() {
}

// 0xb76a08 — __ZN4Ogre16ShadowRenderableD1Ev
// type: void __fastcall(Ogre::ShadowRenderable *__hidden this)
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
#[doc(alias = "__ZN4Ogre16ShadowRenderableD1Ev")]
// IDA 0xb76a08: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b76a08() {
}

// 0xb98608 — __ZN4Ogre7MeshPtrD1Ev
// type: void __fastcall(Ogre::MeshPtr *__hidden this)
#[doc(alias = "Ogre::MeshPtr::~MeshPtr()")]
#[doc(alias = "__ZN4Ogre7MeshPtrD1Ev")]
// IDA 0xb98608: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b98608() {
}

// 0xb98bb0 — __ZN4Ogre9SharedPtrINS_10DataStreamEED1Ev
// type: int __fastcall(int)
#[doc(alias = "Ogre::SharedPtr<Ogre::DataStream>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_10DataStreamEED1Ev")]
// IDA 0xb98bb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b98bb0() {
}

// 0xb99d88 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20GpuProgramParametersEED0Ev")]
// IDA 0xb99d88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b99d88() {
}

// 0xb99e48 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE7destroyEv
// type: void __fastcall(int)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE7destroyEv")]
// IDA 0xb99e48: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b99e48() {
}

// 0xb99f40 — __ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgramParameters>::swap(Ogre::SharedPtr<Ogre::GpuProgramParameters>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_20GpuProgramParametersEE4swapERS2_")]
// IDA 0xb99f40: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b99f40() {
}

// 0xb99f60 — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEED1Ev")]
// IDA 0xb99f60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b99f60() {
}

// 0xb99f90 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEED0Ev")]
// IDA 0xb99f90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b99f90() {
}

// 0xb9a050 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv
// type: void __fastcall(int)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE7destroyEv")]
// IDA 0xb9a050: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a050() {
}

// 0xb9a150 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::swap(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEE4swapERS2_")]
// IDA 0xb9a150: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a150() {
}

// 0xb9a170 — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::swap(Ogre::SharedPtr<Ogre::GpuNamedConstants>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEE4swapERS2_")]
// IDA 0xb9a170: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a170() {
}

// 0xb9a190 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEED0Ev")]
// IDA 0xb9a190: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9a190() {
}

// 0xb9a230 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE7destroyEv
// type: int __fastcall(int, void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE7destroyEv")]
// IDA 0xb9a230: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a230() {
}

// 0xb9a268 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::swap(Ogre::SharedPtr<Ogre::GpuSharedParameters>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEE4swapERS2_")]
// IDA 0xb9a268: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a268() {
}

// 0xb9a920 — __ZN4Ogre9SharedPtrINS_4MeshEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::~SharedPtr()")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEED0Ev")]
// IDA 0xb9a920: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9a920() {
}

// 0xb9a9c0 — __ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv
// type: int __fastcall(int, void *)
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::destroy(void)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEE7destroyEv")]
// IDA 0xb9a9c0: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a9c0() {
}

// 0xb9a9f8 — __ZN4Ogre9SharedPtrINS_4MeshEE4swapERS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "Ogre::SharedPtr<Ogre::Mesh>::swap(Ogre::SharedPtr<Ogre::Mesh>&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_4MeshEE4swapERS2_")]
// IDA 0xb9a9f8: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9a9f8() {
}

// 0xb9aa2c — __ZN4Ogre4Node11setListenerEPNS0_8ListenerE
// type: int __fastcall(int result, int)
#[doc(alias = "Ogre::Node::setListener(Ogre::Node::Listener *)")]
#[doc(alias = "__ZN4Ogre4Node11setListenerEPNS0_8ListenerE")]
// IDA 0xb9aa2c: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa2c() {
}

// 0xb9aa34 — __ZNK4Ogre4Node11getListenerEv
// type: int __fastcall(Ogre::Node *this)
#[doc(alias = "Ogre::Node::getListener(void)const")]
#[doc(alias = "__ZNK4Ogre4Node11getListenerEv")]
// IDA 0xb9aa34: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa34() {
}

// 0xb9aa3c — __ZN4Ogre4Node10setUserAnyERKNS_3AnyE
// type: void __fastcall(Ogre::Node *this, const Ogre::Any *)
#[doc(alias = "Ogre::Node::setUserAny(Ogre::Any const&)")]
#[doc(alias = "__ZN4Ogre4Node10setUserAnyERKNS_3AnyE")]
// IDA 0xb9aa3c: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa3c() {
}

// 0xb9aa44 — __ZNK4Ogre4Node10getUserAnyEv
// type: _DWORD *__fastcall(Ogre::Node *this)
#[doc(alias = "Ogre::Node::getUserAny(void)const")]
#[doc(alias = "__ZNK4Ogre4Node10getUserAnyEv")]
// IDA 0xb9aa44: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa44() {
}

// 0xb9aa4c — __ZNK4Ogre9SceneNode14isInSceneGraphEv
// type: int __fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::isInSceneGraph(void)const")]
#[doc(alias = "__ZNK4Ogre9SceneNode14isInSceneGraphEv")]
// IDA 0xb9aa4c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa4c() {
}

// 0xb9aa54 — __ZN4Ogre9SceneNode15_notifyRootNodeEv
// type: int __fastcall(int this)
#[doc(alias = "Ogre::SceneNode::_notifyRootNode(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode15_notifyRootNodeEv")]
// IDA 0xb9aa54: 3 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa54() {
}

// 0xb9aa5c — __ZN4Ogre9SceneNode18getAutoTrackTargetEv
// type: int __fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::getAutoTrackTarget(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode18getAutoTrackTargetEv")]
// IDA 0xb9aa5c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa5c() {
}

// 0xb9aa64 — __ZN4Ogre9SceneNode18getAutoTrackOffsetEv
// type: char *__fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::getAutoTrackOffset(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode18getAutoTrackOffsetEv")]
// IDA 0xb9aa64: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa64() {
}

// 0xb9aa6c — __ZN4Ogre9SceneNode26getAutoTrackLocalDirectionEv
// type: char *__fastcall(Ogre::SceneNode *this)
#[doc(alias = "Ogre::SceneNode::getAutoTrackLocalDirection(void)")]
#[doc(alias = "__ZN4Ogre9SceneNode26getAutoTrackLocalDirectionEv")]
// IDA 0xb9aa6c: 2 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9aa6c() {
}

// 0xb9b3bc — __ZN4Ogre17istreamDataStreamC1EPSib
// type: Ogre::istreamDataStream *__fastcall(Ogre::istreamDataStream *this, std::istream *, bool)
#[doc(alias = "Ogre::istreamDataStream::istreamDataStream(std::istream *,bool)")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamC1EPSib")]
// IDA 0xb9b3bc: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9b3bc() {
}

// 0xb9b52c — __ZN4Ogre17istreamDataStreamD0Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this)
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD0Ev")]
// IDA 0xb9b52c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9b52c() {
}

// 0xb9b5e0 — __ZN4Ogre17istreamDataStreamD1Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this)
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD1Ev")]
// IDA 0xb9b5e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_b9b5e0() {
}

// 0xb9b5e4 — __ZN4Ogre17istreamDataStreamD2Ev
// type: void __fastcall(Ogre::istreamDataStream *__hidden this)
#[doc(alias = "Ogre::istreamDataStream::~istreamDataStream()")]
#[doc(alias = "__ZN4Ogre17istreamDataStreamD2Ev")]
// IDA 0xb9b5e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_b9b5e4() {
}

// 0xb9b744 — __ZN4Ogre17istreamDataStream4readEPvm
// type: int __fastcall(std::istream **this, char *, int)
#[doc(alias = "Ogre::istreamDataStream::read(void *,unsigned long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4readEPvm")]
// IDA 0xb9b744: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9b744() {
}

// 0xb9b758 — __ZN4Ogre17istreamDataStream8readLineEPcmRKSs
// type: unsigned int __fastcall(std::istream **this, char *, unsigned int, char **)
#[doc(alias = "Ogre::istreamDataStream::readLine(char *,unsigned long,std::string const&)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream8readLineEPcmRKSs")]
// IDA 0xb9b758: 393 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9b758() {
}

// 0xb9bbd0 — __ZN4Ogre17istreamDataStream4skipEl
// type: int __fastcall(Ogre::istreamDataStream *this, int)
#[doc(alias = "Ogre::istreamDataStream::skip(long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4skipEl")]
// IDA 0xb9bbd0: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bbd0() {
}

// 0xb9bbf8 — __ZN4Ogre17istreamDataStream4seekEm
// type: int __fastcall(Ogre::istreamDataStream *this, int)
#[doc(alias = "Ogre::istreamDataStream::seek(unsigned long)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream4seekEm")]
// IDA 0xb9bbf8: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bbf8() {
}

// 0xb9bc20 — __ZNK4Ogre17istreamDataStream4tellEv
// type: int __fastcall(Ogre::istreamDataStream *this)
#[doc(alias = "Ogre::istreamDataStream::tell(void)const")]
#[doc(alias = "__ZNK4Ogre17istreamDataStream4tellEv")]
// IDA 0xb9bc20: 26 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bc20() {
}

// 0xb9bc64 — __ZNK4Ogre17istreamDataStream3eofEv
// type: int __fastcall(Ogre::istreamDataStream *this)
#[doc(alias = "Ogre::istreamDataStream::eof(void)const")]
#[doc(alias = "__ZNK4Ogre17istreamDataStream3eofEv")]
// IDA 0xb9bc64: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bc64() {
}

// 0xb9bc78 — __ZN4Ogre17istreamDataStream5closeEv
// type: int __fastcall(Ogre::istreamDataStream *this)
#[doc(alias = "Ogre::istreamDataStream::close(void)")]
#[doc(alias = "__ZN4Ogre17istreamDataStream5closeEv")]
// IDA 0xb9bc78: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9bc78() {
}

// 0xb9c320 — __ZN4Ogre7QuadricC1Ev
// type: int __fastcall(int this)
#[doc(alias = "Ogre::Quadric::Quadric(void)")]
#[doc(alias = "__ZN4Ogre7QuadricC1Ev")]
// IDA 0xb9c320: 16 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c320() {
}

// 0xb9c344 — __ZN4Ogre7Quadric9setOriginERKNS_7Vector3E
// type: int __fastcall(int this, const Vector3 *)
#[doc(alias = "Ogre::Quadric::setOrigin(Ogre::Vector3 const&)")]
#[doc(alias = "__ZN4Ogre7Quadric9setOriginERKNS_7Vector3E")]
// IDA 0xb9c344: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c344() {
}

// 0xb9c358 — __ZN4Ogre7Quadric14createCylinderEPNS_12SceneManagerERKSsPNS_12ManualObjectEfffii
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, float32_t, float32_t, int, int)
#[doc(alias = "Ogre::Quadric::createCylinder(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric14createCylinderEPNS_12SceneManagerERKSsPNS_12ManualObjectEfffii")]
// IDA 0xb9c358: 2824 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9c358() {
}

// 0xb9e7e8 — __ZN4Ogre7Quadric10createDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffii
// type: int __fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float, float, int, int)
#[doc(alias = "Ogre::Quadric::createDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric10createDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffii")]
// IDA 0xb9e7e8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9e7e8() {
}

// 0xb9e830 — __ZN4Ogre7Quadric17createPartialDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffiiff
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, float32_t, int, int, float32_t, float32_t)
#[doc(alias = "Ogre::Quadric::createPartialDisk(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,float,int,int,float,float)")]
#[doc(alias = "__ZN4Ogre7Quadric17createPartialDiskEPNS_12SceneManagerERKSsPNS_12ManualObjectEffiiff")]
// IDA 0xb9e830: 2903 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b9e830() {
}

// 0xba0b70 — __ZN4Ogre7Quadric12createSphereEPNS_12SceneManagerERKSsPNS_12ManualObjectEfii
// type: Ogre::ManualObject *__fastcall(Ogre::Quadric *this, Ogre::SceneManager *, const std::string *, Ogre::ManualObject *, float32_t, int, int)
#[doc(alias = "Ogre::Quadric::createSphere(Ogre::SceneManager *,std::string const&,Ogre::ManualObject *,float,int,int)")]
#[doc(alias = "__ZN4Ogre7Quadric12createSphereEPNS_12SceneManagerERKSsPNS_12ManualObjectEfii")]
// IDA 0xba0b70: 3827 insns (PUSH..TBH.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba0b70() {
}

// 0xba4494 — __ZNK4Ogre10RbxArchive15isCaseSensitiveEv
// type: int __fastcall(Ogre::RbxArchive *this)
#[doc(alias = "Ogre::RbxArchive::isCaseSensitive(void)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive15isCaseSensitiveEv")]
// IDA 0xba4494: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba4494() {
}

// 0xba4498 — __ZNK4Ogre10RbxArchive17doStaticFindFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
// type: void __fastcall(int, const char **, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, char, char, char, char, int, char, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "Ogre::RbxArchive::doStaticFindFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive17doStaticFindFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE")]
// IDA 0xba4498: 499 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba4498() {
}

// 0xba4a18 — __ZNK4Ogre10RbxArchive9findFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE
// type: void __fastcall(struct _Unwind_Exception *, std::string *, struct _Unwind_Exception *, int, int, int)
#[doc(alias = "Ogre::RbxArchive::findFiles(std::string const&,bool,bool,std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> *,std::vector*<Ogre::FileInfo,Ogre::STLAllocator<std::vector*,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive9findFilesERKSsbbPSt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEPS3_INS_8FileInfoENS4_ISB_S7_EEE")]
// IDA 0xba4a18: 862 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba4a18() {
}

// 0xba5874 — __ZN4OgreL16concatenate_pathERKSsS1_
// type: void __fastcall(Ogre *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::concatenate_path(std::string const&,std::string const&)")]
#[doc(alias = "__ZN4OgreL16concatenate_pathERKSsS1_")]
// IDA 0xba5874: 167 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba5874() {
}

// 0xba5a54 — __ZN4Ogre10RbxArchiveD0Ev
// type: void __fastcall(Ogre::RbxArchive *__hidden this)
#[doc(alias = "Ogre::RbxArchive::~RbxArchive()")]
#[doc(alias = "__ZN4Ogre10RbxArchiveD0Ev")]
// IDA 0xba5a54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba5a54() {
}

// 0xba5af0 — __ZN4Ogre10RbxArchiveD1Ev
// type: void __fastcall(Ogre::RbxArchive *__hidden this)
#[doc(alias = "Ogre::RbxArchive::~RbxArchive()")]
#[doc(alias = "__ZN4Ogre10RbxArchiveD1Ev")]
// IDA 0xba5af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba5af0() {
}

// 0xba5b88 — __ZN4Ogre10RbxArchive4loadEv
// type: void __fastcall(Ogre::RbxArchive *this)
#[doc(alias = "Ogre::RbxArchive::load(void)")]
#[doc(alias = "__ZN4Ogre10RbxArchive4loadEv")]
// IDA 0xba5b88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ba5b88() {
}

// 0xba5b8c — __ZN4Ogre10RbxArchive6unloadEv
// type: void __fastcall(Ogre::RbxArchive *this)
#[doc(alias = "Ogre::RbxArchive::unload(void)")]
#[doc(alias = "__ZN4Ogre10RbxArchive6unloadEv")]
// IDA 0xba5b8c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_ba5b8c() {
}

// 0xba5b90 — __ZNK4Ogre10RbxArchive4openERKSsb
// type: void __fastcall(Ogre::RbxArchive *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::RbxArchive::open(std::string const&,bool)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive4openERKSsb")]
// IDA 0xba5b90: 619 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba5b90() {
}

// 0xba6244 — __ZN4Ogre10RbxArchive4listEbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::list(bool,bool)")]
#[doc(alias = "__ZN4Ogre10RbxArchive4listEbb")]
// IDA 0xba6244: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6244() {
}

// 0xba6460 — __ZN4Ogre10RbxArchive12listFileInfoEbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::listFileInfo(bool,bool)")]
#[doc(alias = "__ZN4Ogre10RbxArchive12listFileInfoEbb")]
// IDA 0xba6460: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6460() {
}

// 0xba667c — __ZN4Ogre10RbxArchive4findERKSsbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, std::string *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::find(std::string const&,bool,bool)")]
#[doc(alias = "__ZN4Ogre10RbxArchive4findERKSsbb")]
// IDA 0xba667c: 147 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba667c() {
}

// 0xba67f8 — __ZNK4Ogre10RbxArchive12findFileInfoERKSsbb
// type: void __fastcall(Ogre::RbxArchive *this, struct _Unwind_Exception *, std::string *, struct _Unwind_Exception *, int)
#[doc(alias = "Ogre::RbxArchive::findFileInfo(std::string const&,bool,bool)const")]
#[doc(alias = "__ZNK4Ogre10RbxArchive12findFileInfoERKSsbb")]
// IDA 0xba67f8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba67f8() {
}

// 0xba6974 — __ZN4Ogre10RbxArchive12makeFullPathERKSs
// type: void __fastcall(Ogre::RbxArchive *this, const std::string *, const std::string *)
#[doc(alias = "Ogre::RbxArchive::makeFullPath(std::string const&)")]
#[doc(alias = "__ZN4Ogre10RbxArchive12makeFullPathERKSs")]
// IDA 0xba6974: 329 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6974() {
}

// 0xba6f04 — __ZN4Ogre10RbxArchive6existsERKSs
// type: bool __fastcall(Ogre::RbxArchive *this, const std::string *)
#[doc(alias = "Ogre::RbxArchive::exists(std::string const&)")]
#[doc(alias = "__ZN4Ogre10RbxArchive6existsERKSs")]
// IDA 0xba6f04: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba6f04() {
}

// 0xba7028 — __ZN4Ogre10RbxArchive15getModifiedTimeERKSs
// type: __darwin_time_t __fastcall(Ogre::RbxArchive *this, const std::string *)
#[doc(alias = "Ogre::RbxArchive::getModifiedTime(std::string const&)")]
#[doc(alias = "__ZN4Ogre10RbxArchive15getModifiedTimeERKSs")]
// IDA 0xba7028: 101 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7028() {
}

// 0xba7150 — __ZN4Ogre17RbxArchiveFactoryC2EPN3RBX15ContentProviderE
// type: Ogre::RbxArchiveFactory *__fastcall(Ogre::RbxArchiveFactory *this, RBX::ContentProvider *)
#[doc(alias = "Ogre::RbxArchiveFactory::RbxArchiveFactory(RBX::ContentProvider *)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactoryC2EPN3RBX15ContentProviderE")]
// IDA 0xba7150: 130 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7150() {
}

// 0xba72c4 — __ZN4Ogre17RbxArchiveFactory18getArchiveTypeNameEPN3RBX15ContentProviderE
// type: void __fastcall(Ogre::RbxArchiveFactory *this, RBX::ContentProvider *)
#[doc(alias = "Ogre::RbxArchiveFactory::getArchiveTypeName(RBX::ContentProvider *)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory18getArchiveTypeNameEPN3RBX15ContentProviderE")]
// IDA 0xba72c4: 172 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba72c4() {
}

// 0xba74cc — __ZNK4Ogre17RbxArchiveFactory7getTypeEv
// type: char *__fastcall(Ogre::RbxArchiveFactory *this)
#[doc(alias = "Ogre::RbxArchiveFactory::getType(void)const")]
#[doc(alias = "__ZNK4Ogre17RbxArchiveFactory7getTypeEv")]
// IDA 0xba74cc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba74cc() {
}

// 0xba74d0 — __ZN4Ogre17RbxArchiveFactory9singletonEv
// type: int __fastcall(Ogre::RbxArchiveFactory *this)
#[doc(alias = "Ogre::RbxArchiveFactory::singleton(void)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory9singletonEv")]
// IDA 0xba74d0: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba74d0() {
}

// 0xba75a8 — __ZN4Ogre17RbxArchiveFactoryD0Ev
// type: void __fastcall(Ogre::RbxArchiveFactory *__hidden this)
#[doc(alias = "Ogre::RbxArchiveFactory::~RbxArchiveFactory()")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactoryD0Ev")]
// IDA 0xba75a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba75a8() {
}

// 0xba7618 — __ZN4Ogre17RbxArchiveFactoryD1Ev
// type: void __fastcall(Ogre::RbxArchiveFactory *__hidden this)
#[doc(alias = "Ogre::RbxArchiveFactory::~RbxArchiveFactory()")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactoryD1Ev")]
// IDA 0xba7618: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba7618() {
}

// 0xba7684 — __ZN4Ogre17RbxArchiveFactory10destroyAllEv
// type: int __fastcall(Ogre::RbxArchiveFactory *this)
#[doc(alias = "Ogre::RbxArchiveFactory::destroyAll(void)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory10destroyAllEv")]
// IDA 0xba7684: 9 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7684() {
}

// 0xba769c — __ZN4Ogre17RbxArchiveFactory14createInstanceERKSs
// type: Ogre::NedPoolingImpl *__fastcall(RBX::ContentProvider **this, const std::string *)
#[doc(alias = "Ogre::RbxArchiveFactory::createInstance(std::string const&)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory14createInstanceERKSs")]
// IDA 0xba769c: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba769c() {
}

// 0xba7888 — __ZN4Ogre17RbxArchiveFactory15destroyInstanceEPNS_7ArchiveE
// type: int __fastcall(int, int)
#[doc(alias = "Ogre::RbxArchiveFactory::destroyInstance(Ogre::Archive *)")]
#[doc(alias = "__ZN4Ogre17RbxArchiveFactory15destroyInstanceEPNS_7ArchiveE")]
// IDA 0xba7888: 7 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7888() {
}

// 0xba7f20 — __ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerE
// type: Ogre::RbxCullableSceneNode *__fastcall(Ogre::RbxCullableSceneNode *this, Ogre::SceneManager *)
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerE")]
// IDA 0xba7f20: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7f20() {
}

// 0xba7f94 — __ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerERKSs
// type: _DWORD __fastcall(Ogre::RbxCullableSceneNode *__hidden this, Ogre::SceneManager *, const std::string *)
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeC1EPNS_12SceneManagerERKSs")]
// IDA 0xba7f94: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba7f94() {
}

// 0xba8008 — __ZN4Ogre20RbxCullableSceneNodeC2EPNS_12SceneManagerERKSs
// type: Ogre::RbxCullableSceneNode *__fastcall(Ogre::RbxCullableSceneNode *this, Ogre::SceneManager *, const std::string *)
#[doc(alias = "Ogre::RbxCullableSceneNode::RbxCullableSceneNode(Ogre::SceneManager *,std::string const&)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeC2EPNS_12SceneManagerERKSs")]
// IDA 0xba8008: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8008() {
}

// 0xba807c — __ZN4Ogre20RbxCullableSceneNodeD0Ev
// type: void __fastcall(Ogre::RbxCullableSceneNode *__hidden this)
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeD0Ev")]
// IDA 0xba807c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba807c() {
}

// 0xba8130 — __ZN4Ogre20RbxCullableSceneNodeD1Ev
// type: void __fastcall(Ogre::RbxCullableSceneNode *this, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeD1Ev")]
// IDA 0xba8130: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_ba8130() {
}

// 0xba8134 — __ZN4Ogre20RbxCullableSceneNodeD2Ev
// type: void __fastcall(Ogre::RbxCullableSceneNode *this, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::~RbxCullableSceneNode()")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNodeD2Ev")]
// IDA 0xba8134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba8134() {
}

// 0xba827c — __ZN4Ogre20RbxCullableSceneNode27calculateSqDistanceToCameraEPKNS_6CameraE
// type: __int32 __fastcall(Ogre::RbxCullableSceneNode *this, const Ogre::Camera *)
#[doc(alias = "Ogre::RbxCullableSceneNode::calculateSqDistanceToCamera(Ogre::Camera const*)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode27calculateSqDistanceToCameraEPKNS_6CameraE")]
// IDA 0xba827c: 115 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba827c() {
}

// 0xba841c — __ZN4Ogre20RbxCullableSceneNode8IsCulledEPKNS_6CameraEb
// type: int __fastcall(Ogre::RbxCullableSceneNode *this, const Ogre::Camera *, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::IsCulled(Ogre::Camera const*,bool)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode8IsCulledEPKNS_6CameraEb")]
// IDA 0xba841c: 93 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba841c() {
}

// 0xba854c — __ZN4Ogre20RbxCullableSceneNode17ShouldCastShadowsEPKNS_6CameraE
// type: bool __fastcall(Ogre::RbxCullableSceneNode *this, const Ogre::Camera *)
#[doc(alias = "Ogre::RbxCullableSceneNode::ShouldCastShadows(Ogre::Camera const*)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode17ShouldCastShadowsEPKNS_6CameraE")]
// IDA 0xba854c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba854c() {
}

// 0xba8594 — __ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbN3RBX15IntersectResultE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool,RBX::IntersectResult)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbbN3RBX15IntersectResultE")]
// IDA 0xba8594: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8594() {
}

// 0xba85e4 — __ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb
// type: int __fastcall(Ogre::RbxCullableSceneNode *this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::VisibleObjectsBoundsInfo *, int, int, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::_findVisibleObjects(Ogre::Camera *,Ogre::RenderQueue *,Ogre::VisibleObjectsBoundsInfo *,bool,bool,bool)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode19_findVisibleObjectsEPNS_6CameraEPNS_11RenderQueueEPNS_24VisibleObjectsBoundsInfoEbbb")]
// IDA 0xba85e4: 142 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba85e4() {
}

// 0xba8750 — __ZN4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv
// type: int __fastcall(Ogre::RbxCullableSceneNode *this, int)
#[doc(alias = "Ogre::RbxCullableSceneNode::getFastFuzzyExtents(void)")]
#[doc(alias = "__ZN4Ogre20RbxCullableSceneNode19getFastFuzzyExtentsEv")]
// IDA 0xba8750: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8750() {
}

// 0xba8e18 — __ZN4Ogre9RbxEntityC1Ev
// type: Ogre::RbxEntity *__fastcall(Ogre::RbxEntity *this)
#[doc(alias = "Ogre::RbxEntity::RbxEntity(void)")]
#[doc(alias = "__ZN4Ogre9RbxEntityC1Ev")]
// IDA 0xba8e18: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8e18() {
}

// 0xba8eb4 — __ZN4Ogre9RbxEntity7setMeshENS_7MeshPtrE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "Ogre::RbxEntity::setMesh(Ogre::MeshPtr)")]
#[doc(alias = "__ZN4Ogre9RbxEntity7setMeshENS_7MeshPtrE")]
// IDA 0xba8eb4: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8eb4() {
}

// 0xba8f9c — __ZN4Ogre9RbxEntity16clearSubEntitiesEv
// type: int __fastcall(int this)
#[doc(alias = "Ogre::RbxEntity::clearSubEntities(void)")]
#[doc(alias = "__ZN4Ogre9RbxEntity16clearSubEntitiesEv")]
// IDA 0xba8f9c: 3 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8f9c() {
}

// 0xba8fa8 — __ZN4Ogre9RbxEntity15appendSubEntityEPNS_12RbxSubEntityE
// type: int __fastcall(int this, Ogre::RbxSubEntity *)
#[doc(alias = "Ogre::RbxEntity::appendSubEntity(Ogre::RbxSubEntity *)")]
#[doc(alias = "__ZN4Ogre9RbxEntity15appendSubEntityEPNS_12RbxSubEntityE")]
// IDA 0xba8fa8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8fa8() {
}

// 0xba8fdc — __ZN4Ogre9RbxEntity18_updateRenderQueueEPNS_11RenderQueueE
// type: int __fastcall(Ogre::RbxEntity *this, Ogre::RenderQueue *)
#[doc(alias = "Ogre::RbxEntity::_updateRenderQueue(Ogre::RenderQueue *)")]
#[doc(alias = "__ZN4Ogre9RbxEntity18_updateRenderQueueEPNS_11RenderQueueE")]
// IDA 0xba8fdc: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba8fdc() {
}

// 0xba905c — __ZN4Ogre9RbxEntity20_notifyCurrentCameraEPNS_6CameraE
// type: int __fastcall(unsigned __int8 *)
#[doc(alias = "Ogre::RbxEntity::_notifyCurrentCamera(Ogre::Camera *)")]
#[doc(alias = "__ZN4Ogre9RbxEntity20_notifyCurrentCameraEPNS_6CameraE")]
// IDA 0xba905c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba905c() {
}

// 0xba9080 — __ZN4Ogre9RbxEntity13setVisibleAllEb
// type: int __fastcall(Ogre::RbxEntity *this, int)
#[doc(alias = "Ogre::RbxEntity::setVisibleAll(bool)")]
#[doc(alias = "__ZN4Ogre9RbxEntity13setVisibleAllEb")]
// IDA 0xba9080: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba9080() {
}

// 0xba90b4 — __ZN4Ogre9RbxEntity4cullEPKNS_6CameraE
// type: void __fastcall(Ogre::RbxEntity *this, const Ogre::Camera *)
#[doc(alias = "Ogre::RbxEntity::cull(Ogre::Camera const*)")]
#[doc(alias = "__ZN4Ogre9RbxEntity4cullEPKNS_6CameraE")]
// IDA 0xba90b4: 181 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba90b4() {
}

// 0xba92b8 — __ZN4Ogre9RbxEntityD0Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZN4Ogre9RbxEntityD0Ev")]
// IDA 0xba92b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba92b8() {
}

// 0xba936c — __ZN4Ogre9RbxEntityD1Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZN4Ogre9RbxEntityD1Ev")]
// IDA 0xba936c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_ba936c() {
}

// 0xba94e0 — __ZN4Ogre9RbxEntityD2Ev
// type: void __fastcall(Ogre::RbxEntity *__hidden this)
#[doc(alias = "Ogre::RbxEntity::~RbxEntity()")]
#[doc(alias = "__ZN4Ogre9RbxEntityD2Ev")]
// IDA 0xba94e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_ba94e0() {
}

// 0xba9dbc — __ZN4Ogre11RootManager9GetOrInitENS_11GraphicsAPIERKSs
// type: Ogre::NedPoolingImpl *__fastcall(Ogre::NedPoolingImpl **, Ogre::NedPoolingImpl *)
#[doc(alias = "Ogre::RootManager::GetOrInit(Ogre::GraphicsAPI,std::string const&)")]
#[doc(alias = "__ZN4Ogre11RootManager9GetOrInitENS_11GraphicsAPIERKSs")]
// IDA 0xba9dbc: 774 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_ba9dbc() {
}

// 0xbaae98 — __ZN4Ogre11RootManagerD2Ev
// type: void __fastcall(Ogre::RootManager *__hidden this)
#[doc(alias = "Ogre::RootManager::~RootManager()")]
#[doc(alias = "__ZN4Ogre11RootManagerD2Ev")]
// IDA 0xbaae98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_baae98() {
}

// 0xbab8f8 — __ZN4Ogre11RootManager16cleanUpResourcesERNS_15ResourceManagerERKSsS4_RKSt6vectorISsSaISsEE
// type: void __fastcall(_DWORD *, const std::string *, const void **, _DWORD *)
#[doc(alias = "Ogre::RootManager::cleanUpResources(Ogre::ResourceManager &,std::string const&,std::string const&,std::vector<std::string,std::allocator<std::string>> const&)")]
#[doc(alias = "__ZN4Ogre11RootManager16cleanUpResourcesERNS_15ResourceManagerERKSsS4_RKSt6vectorISsSaISsEE")]
// IDA 0xbab8f8: 1348 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bab8f8() {
}

// 0xbac7fc — __ZN4Ogre11RootManager14printResourcesERNS_15ResourceManagerERKSs
// type: void __fastcall(Ogre::RootManager *this, Ogre::ResourceManager *, const std::string *)
#[doc(alias = "Ogre::RootManager::printResources(Ogre::ResourceManager &,std::string const&)")]
#[doc(alias = "__ZN4Ogre11RootManager14printResourcesERNS_15ResourceManagerERKSs")]
// IDA 0xbac7fc: 718 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bac7fc() {
}

// 0xbacfcc — __ZN4Ogre12VisualEngineC1Ev
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngineC1Ev")]
// IDA 0xbacfcc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bacfcc() {
}

// 0xbacfd0 — __ZN4Ogre12VisualEngineC2Ev
// type: Ogre::VisualEngine *__fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::VisualEngine(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngineC2Ev")]
// IDA 0xbacfd0: 294 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bacfd0() {
}

// 0xbad2c8 — __ZN4Ogre12VisualEngineD0Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
#[doc(alias = "__ZN4Ogre12VisualEngineD0Ev")]
// IDA 0xbad2c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bad2c8() {
}

// 0xbad368 — __ZN4Ogre12VisualEngineD1Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
#[doc(alias = "__ZN4Ogre12VisualEngineD1Ev")]
// IDA 0xbad368: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bad368() {
}

// 0xbad36c — __ZN4Ogre12VisualEngineD2Ev
// type: void __fastcall(Ogre::VisualEngine *__hidden this)
#[doc(alias = "Ogre::VisualEngine::~VisualEngine()")]
#[doc(alias = "__ZN4Ogre12VisualEngineD2Ev")]
// IDA 0xbad36c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_bad36c() {
}

// 0xbadb50 — __ZN4Ogre12VisualEngine7setViewEPN3RBX8ViewBaseE
// type: void __fastcall(Ogre::VisualEngine *this, RBX::ViewBase *)
#[doc(alias = "Ogre::VisualEngine::setView(RBX::ViewBase *)")]
#[doc(alias = "__ZN4Ogre12VisualEngine7setViewEPN3RBX8ViewBaseE")]
// IDA 0xbadb50: 199 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_badb50() {
}

// 0xbadd64 — __ZN4Ogre14SaveRBXDbgInfoEPKNS_24RenderSystemCapabilitiesE
// type: void __fastcall(Ogre *this, const Ogre::RenderSystemCapabilities *)
#[doc(alias = "Ogre::SaveRBXDbgInfo(Ogre::RenderSystemCapabilities const*)")]
#[doc(alias = "__ZN4Ogre14SaveRBXDbgInfoEPKNS_24RenderSystemCapabilitiesE")]
// IDA 0xbadd64: 316 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_badd64() {
}

// 0xbae0f8 — __ZN4Ogre12VisualEngine14initializeBaseENS_11GraphicsAPIEmiiPN3RBX15CRenderSettingsERKSsmPNS2_9OSContextE
// type: int __fastcall(_DWORD *, int, int, int (*)(const char *, ...), int, int, int, int, const char **)
#[doc(alias = "Ogre::VisualEngine::initializeBase(Ogre::GraphicsAPI,unsigned long,int,int,RBX::CRenderSettings *,std::string const&,unsigned long,RBX::OSContext *)")]
#[doc(alias = "__ZN4Ogre12VisualEngine14initializeBaseENS_11GraphicsAPIEmiiPN3RBX15CRenderSettingsERKSsmPNS2_9OSContextE")]
// IDA 0xbae0f8: 1803 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bae0f8() {
}

// 0xbb0120 — __ZN4Ogre12VisualEngine14initializeLoadEii
// type: void __fastcall(Ogre::VisualEngine *this, int, int)
#[doc(alias = "Ogre::VisualEngine::initializeLoad(int,int)")]
#[doc(alias = "__ZN4Ogre12VisualEngine14initializeLoadEii")]
// IDA 0xbb0120: 149 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb0120() {
}

// 0xbb039c — __ZN4Ogre12VisualEngine14setupResourcesEv
// type: void __fastcall(Ogre::VisualEngine *this, Ogre::VisualEngine *)
#[doc(alias = "Ogre::VisualEngine::setupResources(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngine14setupResourcesEv")]
// IDA 0xbb039c: 1212 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb039c() {
}

// 0xbb1160 — __ZN4Ogre12VisualEngine10setupSceneEii
// type: void __fastcall(Ogre::VisualEngine *this, int, int)
#[doc(alias = "Ogre::VisualEngine::setupScene(int,int)")]
#[doc(alias = "__ZN4Ogre12VisualEngine10setupSceneEii")]
// IDA 0xbb1160: 974 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb1160() {
}

// 0xbb1c18 — __ZN4Ogre12VisualEngine17checkMaterialCapsEv
// type: void __fastcall(Ogre::Root **this)
#[doc(alias = "Ogre::VisualEngine::checkMaterialCaps(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngine17checkMaterialCapsEv")]
// IDA 0xbb1c18: 647 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bb1c18() {
}

// 0xbb2308 — __ZN4Ogre12VisualEngine13reloadShadersEv
// type: int __fastcall(Ogre::VisualEngine *this)
#[doc(alias = "Ogre::VisualEngine::reloadShaders(void)")]
#[doc(alias = "__ZN4Ogre12VisualEngine13reloadShadersEv")]
// IDA 0xbb2308: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bb2308() {
}

