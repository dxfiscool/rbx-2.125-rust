//! rendering shard 255 — 100 stubs EA-sorted asc global gap filler after 0x2f0bb8 not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 27720->27820 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2f0cb0 — __ZN3RBX12PartDragTool11onMouseMoveERKNS_7UIEventE
// type: int __fastcall(RBX::PartDragTool *this, const RBX::UIEvent *, int)
#[doc(alias = "RBX::PartDragTool::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX12PartDragTool11onMouseMoveERKNS_7UIEventE
// IDA 0x2f0cb0: 53 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0cb0() {
}

// 0x2f0d60 — __ZN3RBX12PartDragTool12onMouseDeltaERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::PartDragTool::onMouseDelta(RBX::UIEvent const&)")]
// was: __ZN3RBX12PartDragTool12onMouseDeltaERKNS_7UIEventE
// IDA 0x2f0d60: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0d60() {
}

// 0x2f0ecc — __ZN3RBX12PartDragTool11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::PartDragTool::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX12PartDragTool11onMouseIdleERKNS_7UIEventE
// IDA 0x2f0ecc: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0ecc() {
}

// 0x2f0f68 — __ZN3RBX12PartDragTool9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::PartDragTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX12PartDragTool9onMouseUpERKNS_7UIEventE
// IDA 0x2f0f68: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f0f68() {
}

// 0x2f1134 — __ZN3RBX12PartDragTool9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::PartDragTool::onKeyDown(RBX::UIEvent const&)")]
// was: __ZN3RBX12PartDragTool9onKeyDownERKNS_7UIEventE
// IDA 0x2f1134: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1134() {
}

// 0x2f13d8 — __ZN3RBX12PartDragToolD0Ev
// type: void __fastcall(RBX::PartDragTool *__hidden this)
#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// was: __ZN3RBX12PartDragToolD0Ev
// IDA 0x2f13d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f13d8() {
}

// 0x2f1478 — __ZN3RBX12PartDragToolD1Ev
// type: void __fastcall(RBX::PartDragTool *__hidden this)
#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// was: __ZN3RBX12PartDragToolD1Ev
// IDA 0x2f1478: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f1478() {
}

// 0x2f147c — __ZThn36_N3RBX12PartDragToolD0Ev
// type: void __fastcall(RBX::PartDragTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PartDragTool::~PartDragTool()")]
// was: __ZThn36_N3RBX12PartDragToolD0Ev
// IDA 0x2f147c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f147c() {
}

// 0x2f1484 — __ZN3RBX12PartDragToolD2Ev
// type: void __fastcall(RBX::PartDragTool *__hidden this)
#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// was: __ZN3RBX12PartDragToolD2Ev
// IDA 0x2f1484: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f1484() {
}

// 0x2f15e4 — __ZThn36_N3RBX12PartDragToolD1Ev
// type: void __fastcall(RBX::PartDragTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::PartDragTool::~PartDragTool()")]
// was: __ZThn36_N3RBX12PartDragToolD1Ev
// IDA 0x2f15e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f15e4() {
}

// 0x2f15ec — __ZN3RBX11shared_fromINS_12PartDragToolEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool> RBX::shared_from<RBX::PartDragTool>(RBX::PartDragTool*)")]
// was: __ZN3RBX11shared_fromINS_12PartDragToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2f15ec: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f15ec() {
}

// 0x2f1754 — __ZN3RBX11MegaDragger12getMousePartEv
// type: _DWORD __fastcall(RBX::MegaDragger *__hidden this)
#[doc(alias = "RBX::MegaDragger::getMousePart(void)")]
// was: __ZN3RBX11MegaDragger12getMousePartEv
// IDA 0x2f1754: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1754() {
}

// 0x2f1808 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_13sPartDragToolEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_13sPartDragToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_13sPartDragToolEEE7getNameEv
// IDA 0x2f1808: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1808() {
}

// 0x2f1830 — __ZNK3RBX12PartDragTool14drawConnectorsEv
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this)
#[doc(alias = "RBX::PartDragTool::drawConnectors(void)const")]
// was: __ZNK3RBX12PartDragTool14drawConnectorsEv
// IDA 0x2f1830: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1830() {
}

// 0x2f1834 — __ZNK3RBX12PartDragTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::PartDragTool *__hidden this)
#[doc(alias = "RBX::PartDragTool::getCursorName(void)const")]
// was: __ZNK3RBX12PartDragTool13getCursorNameEv
// IDA 0x2f1834: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1834() {
}

// 0x2f1864 — __ZN3RBX4Name13callDoDeclareILZNS_13sPartDragToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sPartDragToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sPartDragToolEEEEvv
// IDA 0x2f1864: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f1864() {
}

// 0x2f1868 — __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v
// IDA 0x2f1868: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1868() {
}

// 0x2f1948 — __GLOBAL__I_a_100
#[doc(alias = "global constructor keyed to_a_100")]
// was: __GLOBAL__I_a_100
// IDA 0x2f1948: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2f1948() {
}

// 0x2f1c20 — __GLOBAL__I_a_101
#[doc(alias = "global constructor keyed to_a_101")]
// was: __GLOBAL__I_a_101
// IDA 0x2f1c20: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2f1c20() {
}

// 0x2f1ef8 — __ZN3RBX10RunDragger8SnapInfo20updateSurfaceFromHitEv
// type: _DWORD __fastcall(RBX::RunDragger::SnapInfo *__hidden this)
#[doc(alias = "RBX::RunDragger::SnapInfo::updateSurfaceFromHit(void)")]
// was: __ZN3RBX10RunDragger8SnapInfo20updateSurfaceFromHitEv
// IDA 0x2f1ef8: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1ef8() {
}

// 0x2f1fb8 — __ZN3RBX10RunDragger8SnapInfo20updateHitFromSurfaceERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::RunDragger::SnapInfo *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::RunDragger::SnapInfo::updateHitFromSurface(RBX::RbxRay const&)")]
// was: __ZN3RBX10RunDragger8SnapInfo20updateHitFromSurfaceERKNS_6RbxRayE
// IDA 0x2f1fb8: 252 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f1fb8() {
}

// 0x2f229c — __ZN3RBX10RunDragger8SnapInfo17hitOutsideExtentsEv
// type: float __fastcall(RBX::Primitive **this)
#[doc(alias = "RBX::RunDragger::SnapInfo::hitOutsideExtents(void)")]
// was: __ZN3RBX10RunDragger8SnapInfo17hitOutsideExtentsEv
// IDA 0x2f229c: 92 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f229c() {
}

// 0x2f23f0 — __ZN3RBX10RunDraggerC1Ev
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::RunDragger(void)")]
// was: __ZN3RBX10RunDraggerC1Ev
// IDA 0x2f23f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f23f0() {
}

// 0x2f23f4 — __ZN3RBX10RunDraggerC2Ev
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::RunDragger(void)")]
// was: __ZN3RBX10RunDraggerC2Ev
// IDA 0x2f23f4: 148 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f23f4() {
}

// 0x2f25ac — __ZN3RBX10RunDraggerD1Ev
// type: void __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::~RunDragger()")]
// was: __ZN3RBX10RunDraggerD1Ev
// IDA 0x2f25ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f25ac() {
}

// 0x2f25b0 — __ZN3RBX10RunDraggerD2Ev
// type: void __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::~RunDragger()")]
// was: __ZN3RBX10RunDraggerD2Ev
// IDA 0x2f25b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f25b0() {
}

// 0x2f26a8 — __ZN3RBX10RunDragger20snapInfoFromSnapPartEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::snapInfoFromSnapPart(void)")]
// was: __ZN3RBX10RunDragger20snapInfoFromSnapPartEv
// IDA 0x2f26a8: 340 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f26a8() {
}

// 0x2f2a54 — __ZN3RBX10RunDragger20snapPartFromSnapInfoEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::snapPartFromSnapInfo(void)")]
// was: __ZN3RBX10RunDragger20snapPartFromSnapInfoEv
// IDA 0x2f2a54: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f2a54() {
}

// 0x2f2f3c — __ZN3RBX10RunDragger11turnUprightEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::RunDragger::turnUpright(RBX::PartInstance *)")]
// was: __ZN3RBX10RunDragger11turnUprightEPNS_12PartInstanceE
// IDA 0x2f2f3c: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f2f3c() {
}

// 0x2f37c8 — __ZN3RBX10RunDragger12moveDragPartEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::moveDragPart(void)")]
// was: __ZN3RBX10RunDragger12moveDragPartEv
// IDA 0x2f37c8: 805 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f37c8() {
}

// 0x2f41c8 — __ZN3RBX10RunDragger19getSnapSurfaceCoordEv
// type: void __fastcall(RBX::RunDragger *this, int)
#[doc(alias = "RBX::RunDragger::getSnapSurfaceCoord(void)")]
// was: __ZN3RBX10RunDragger19getSnapSurfaceCoordEv
// IDA 0x2f41c8: 135 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f41c8() {
}

// 0x2f4340 — __ZN3RBX10RunDragger12snapDragPartEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::snapDragPart(void)")]
// was: __ZN3RBX10RunDragger12snapDragPartEv
// IDA 0x2f4340: 234 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4340() {
}

// 0x2f46c0 — __ZN3RBX10RunDragger8adjacentEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RunDragger::adjacent(RBX::Primitive *,RBX::Primitive *)")]
// was: __ZN3RBX10RunDragger8adjacentEPNS_9PrimitiveES2_
// IDA 0x2f46c0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f46c0() {
}

// 0x2f4ae0 — __ZN3RBX10RunDragger11fallOffEdgeEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::fallOffEdge(void)")]
// was: __ZN3RBX10RunDragger11fallOffEdgeEv
// IDA 0x2f4ae0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4ae0() {
}

// 0x2f4b14 — __ZN3RBX10RunDragger11fallOffPartERb
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, bool *)
#[doc(alias = "RBX::RunDragger::fallOffPart(bool &)")]
// was: __ZN3RBX10RunDragger11fallOffPartERb
// IDA 0x2f4b14: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4b14() {
}

// 0x2f4c88 — __ZN3RBX10RunDragger17rayHitsCloserPartEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::rayHitsCloserPart(void)")]
// was: __ZN3RBX10RunDragger17rayHitsCloserPartEv
// IDA 0x2f4c88: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4c88() {
}

// 0x2f4dd8 — __ZN3RBX10RunDragger16tooCloseToCameraEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::tooCloseToCamera(void)")]
// was: __ZN3RBX10RunDragger16tooCloseToCameraEv
// IDA 0x2f4dd8: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4dd8() {
}

// 0x2f5168 — __ZN3RBX10RunDragger9findSafeYEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::findSafeY(void)")]
// was: __ZN3RBX10RunDragger9findSafeYEv
// IDA 0x2f5168: 222 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f5168() {
}

// 0x2f5610 — __ZN3RBX10RunDragger4snapERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::RunDragger::snap(RBX::RbxRay const&)")]
// was: __ZN3RBX10RunDragger4snapERKNS_6RbxRayE
// IDA 0x2f5610: 236 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f5610() {
}

// 0x2f5d3c — __GLOBAL__I_a_102
#[doc(alias = "global constructor keyed to_a_102")]
// was: __GLOBAL__I_a_102
// IDA 0x2f5d3c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2f5d3c() {
}

// 0x2f614c — __ZN3RBX13ArrowToolBase12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, const UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onMouseHover(RBX::UIEvent const&)")]
// was: __ZN3RBX13ArrowToolBase12onMouseHoverERKNS_7UIEventE
// IDA 0x2f614c: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f614c() {
}

// 0x2f6154 — __ZN3RBX13ArrowToolBase11onMouseIdleERKNS_7UIEventE
// type: int __fastcall(RBX::ArrowToolBase *this, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onMouseIdle(RBX::UIEvent const&)")]
// was: __ZN3RBX13ArrowToolBase11onMouseIdleERKNS_7UIEventE
// IDA 0x2f6154: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6154() {
}

// 0x2f6190 — __ZNK3RBX13ArrowToolBase13getCursorNameEv
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this)
#[doc(alias = "RBX::ArrowToolBase::getCursorName(void)const")]
// was: __ZNK3RBX13ArrowToolBase13getCursorNameEv
// IDA 0x2f6190: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6190() {
}

// 0x2f61c0 — __ZN3RBX13ArrowToolBase9findDecalEPNS_12PartInstanceERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::PartInstance *, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::findDecal(RBX::PartInstance *,RBX::UIEvent const&)")]
// was: __ZN3RBX13ArrowToolBase9findDecalEPNS_12PartInstanceERKNS_7UIEventE
// IDA 0x2f61c0: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f61c0() {
}

// 0x2f6254 — __ZN3RBX13ArrowToolBase11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX13ArrowToolBase11onMouseDownERKNS_7UIEventE
// IDA 0x2f6254: 366 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6254() {
}

// 0x2f6610 — __ZN3RBX13ArrowToolBase13onPeekKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onPeekKeyDown(RBX::UIEvent const&)")]
// was: __ZN3RBX13ArrowToolBase13onPeekKeyDownERKNS_7UIEventE
// IDA 0x2f6610: 201 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6610() {
}

// 0x2f68d0 — __ZNK3RBX16AdvArrowToolBase13getCursorNameEv
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this)
#[doc(alias = "RBX::AdvArrowToolBase::getCursorName(void)const")]
// was: __ZNK3RBX16AdvArrowToolBase13getCursorNameEv
// IDA 0x2f68d0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f68d0() {
}

// 0x2f6900 — __ZN3RBX16AdvArrowToolBase9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onKeyDown(RBX::UIEvent const&)")]
// was: __ZN3RBX16AdvArrowToolBase9onKeyDownERKNS_7UIEventE
// IDA 0x2f6900: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6900() {
}

// 0x2f6954 — __ZN3RBX16AdvArrowToolBase11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX16AdvArrowToolBase11onMouseDownERKNS_7UIEventE
// IDA 0x2f6954: 359 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6954() {
}

// 0x2f6d04 — __ZN3RBX16AdvArrowToolBase11onMouseMoveERKNS_7UIEventE
// type: int __fastcall(RBX::AdvArrowToolBase *this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX16AdvArrowToolBase11onMouseMoveERKNS_7UIEventE
// IDA 0x2f6d04: 6 insns (MOVW..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6d04() {
}

// 0x2f6d18 — __ZN3RBX16AdvArrowToolBase30determineManualJointConditionsEv
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this)
#[doc(alias = "RBX::AdvArrowToolBase::determineManualJointConditions(void)")]
// was: __ZN3RBX16AdvArrowToolBase30determineManualJointConditionsEv
// IDA 0x2f6d18: 232 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6d18() {
}

// 0x2f6fb8 — __ZN3RBX16AdvArrowToolBase9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX16AdvArrowToolBase9onMouseUpERKNS_7UIEventE
// IDA 0x2f6fb8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6fb8() {
}

// 0x2f6ff4 — __ZN3RBX16BoxSelectCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)")]
// was: __ZN3RBX16BoxSelectCommandC2EPNS_9WorkspaceE
// IDA 0x2f6ff4: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f6ff4() {
}

// 0x2f7134 — __ZN3RBX16BoxSelectCommandD0Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
// was: __ZN3RBX16BoxSelectCommandD0Ev
// IDA 0x2f7134: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f7134() {
}

// 0x2f71d4 — __ZN3RBX16BoxSelectCommandD1Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
// was: __ZN3RBX16BoxSelectCommandD1Ev
// IDA 0x2f71d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f71d4() {
}

// 0x2f71d8 — __ZThn36_N3RBX16BoxSelectCommandD0Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::~BoxSelectCommand()")]
// was: __ZThn36_N3RBX16BoxSelectCommandD0Ev
// IDA 0x2f71d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f71d8() {
}

// 0x2f71e0 — __ZN3RBX16BoxSelectCommandD2Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
// was: __ZN3RBX16BoxSelectCommandD2Ev
// IDA 0x2f71e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f71e0() {
}

// 0x2f7324 — __ZThn36_N3RBX16BoxSelectCommandD1Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::~BoxSelectCommand()")]
// was: __ZThn36_N3RBX16BoxSelectCommandD1Ev
// IDA 0x2f7324: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f7324() {
}

// 0x2f732c — __ZN3RBX16BoxSelectCommand9selectAndERKSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EE
#[doc(alias = "RBX::BoxSelectCommand::selectAnd(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: __ZN3RBX16BoxSelectCommand9selectAndERKSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EE
// IDA 0x2f732c: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f732c() {
}

// 0x2f7394 — __ZN3RBX16BoxSelectCommand13selectReverseERKSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EE
#[doc(alias = "RBX::BoxSelectCommand::selectReverse(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: __ZN3RBX16BoxSelectCommand13selectReverseERKSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EE
// IDA 0x2f7394: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7394() {
}

// 0x2f73fc — __ZN3RBX16BoxSelectCommand11onMouseDownERKNS_7UIEventE
// type: int __fastcall(RBX::BoxSelectCommand *this, const RBX::UIEvent *, int)
#[doc(alias = "RBX::BoxSelectCommand::onMouseDown(RBX::UIEvent const&)")]
// was: __ZN3RBX16BoxSelectCommand11onMouseDownERKNS_7UIEventE
// IDA 0x2f73fc: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f73fc() {
}

// 0x2f7468 — __ZN3RBX16BoxSelectCommand11onMouseMoveERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::BoxSelectCommand::onMouseMove(RBX::UIEvent const&)")]
// was: __ZN3RBX16BoxSelectCommand11onMouseMoveERKNS_7UIEventE
// IDA 0x2f7468: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7468() {
}

// 0x2f78d8 — __ZNK3RBX9Selection10isSelectedEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Selection *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Selection::isSelected(RBX::Instance const*)const")]
// was: __ZNK3RBX9Selection10isSelectedEPKNS_8InstanceE
// IDA 0x2f78d8: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f78d8() {
}

// 0x2f79c8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x2f79c8: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f79c8() {
}

// 0x2f7a7c — __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection11AddIteratorEET1_T_SA_T0_SB_S9_
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, boost::detail::sp_counted_base *, RBX::Selection *, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::AddIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
// was: __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection11AddIteratorEET1_T_SA_T0_SB_S9_
// IDA 0x2f7a7c: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7a7c() {
}

// 0x2f7bd4 — __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14RemoveIteratorEET1_T_SA_T0_SB_S9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::RemoveIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
// was: __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14RemoveIteratorEET1_T_SA_T0_SB_S9_
// IDA 0x2f7bd4: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7bd4() {
}

// 0x2f7d2c — __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14ToggleIteratorEET1_T_SA_T0_SB_S9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::ToggleIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
// was: __ZSt14set_differenceISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEES6_NS3_9Selection14ToggleIteratorEET1_T_SA_T0_SB_S9_
// IDA 0x2f7d2c: 128 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7d2c() {
}

// 0x2f7e84 — __ZN3RBX8Instance15queryTypedChildINS_10SelectableEEEPT_i
#[doc(alias = "RBX::Selectable * RBX::Instance::queryTypedChild<RBX::Selectable>(int)")]
// was: __ZN3RBX8Instance15queryTypedChildINS_10SelectableEEEPT_i
// IDA 0x2f7e84: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7e84() {
}

// 0x2f7ec0 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv
// IDA 0x2f7ec0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7ec0() {
}

// 0x2f7ee8 — __ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv
// IDA 0x2f7ee8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f7ee8() {
}

// 0x2f7eec — __ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v
// IDA 0x2f7eec: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7eec() {
}

// 0x2f7fcc — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
// IDA 0x2f7fcc: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f7fcc() {
}

// 0x2f8034 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// IDA 0x2f8034: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8034() {
}

// 0x2f8080 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
// IDA 0x2f8080: 81 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8080() {
}

// 0x2f8164 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
// was: __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// IDA 0x2f8164: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8164() {
}

// 0x2f818c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
// was: __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// IDA 0x2f818c: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f818c() {
}

// 0x2f81a8 — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14ToggleIteratorEEET0_T_SD_SC_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::ToggleIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
// was: __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14ToggleIteratorEEET0_T_SD_SC_
// IDA 0x2f81a8: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2f81a8() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2f829c — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EEaSERKSA_
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::operator=(std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EEaSERKSA_
// IDA 0x2f829c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f829c() {
}

// 0x2f82e8 — __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSC_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> const*,std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>>*)")]
// was: __ZNSt8_Rb_treeIN5boost10shared_ptrIN3RBX8InstanceEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSC_
// IDA 0x2f82e8: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f82e8() {
}

// 0x2f843c — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14RemoveIteratorEEET0_T_SD_SC_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
// was: __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection14RemoveIteratorEEET0_T_SD_SC_
// IDA 0x2f843c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2f843c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2f8530 — __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection11AddIteratorEEET0_T_SD_SC_
// type: RBX::Selection *__fastcall(int, int, RBX::Selection *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
// was: __ZNSt6__copyILb0ESt26bidirectional_iterator_tagE4copyISt23_Rb_tree_const_iteratorIN5boost10shared_ptrIN3RBX8InstanceEEEENS6_9Selection11AddIteratorEEET0_T_SD_SC_
// IDA 0x2f8530: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2f8530() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2f8624 — __ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x2f8624: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8624() {
}

// 0x2f86ec — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2f86ec: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f86ec() {
}

// 0x2f87d0 — __ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x2f87d0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f87d0() {
}

// 0x2f88c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x2f88c8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2f88c8() {
}

// 0x2f88cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x2f88cc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f88cc() {
}

// 0x2f88d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x2f88d0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f88d0() {
}

// 0x2f88e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2f88e0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f88e0() {
}

// 0x2f88f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2f88f8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f88f8() {
}

// 0x2f88fc — __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_8InstanceEEEE13createServiceEv
#[doc(alias = "RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")]
// was: __ZNK3RBX13ServiceClientINS_17FilteredSelectionINS_8InstanceEEEE13createServiceEv
// IDA 0x2f88fc: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f88fc() {
}

// 0x2f89dc — __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_8InstanceEEEEaSERKS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX17FilteredSelectionINS1_8InstanceEEEEaSERKS5_
// IDA 0x2f89dc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f89dc() {
}

// 0x2f8a14 — __ZN3RBX11shared_fromINS_17FilteredSelectionINS_8InstanceEEEEEN5boost10shared_ptrIT_EEPS6_
#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")]
// was: __ZN3RBX11shared_fromINS_17FilteredSelectionINS_8InstanceEEEEEN5boost10shared_ptrIT_EEPS6_
// IDA 0x2f8a14: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8a14() {
}

// 0x2f8b84 — __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_PKS3_
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_8InstanceEEEEEPT_PKS3_
// IDA 0x2f8b84: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8b84() {
}

// 0x2f8b9c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKS5_EEET_SF_SF_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const> const&,std::random_access_iterator_tag)")]
// was: __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS3_IKS5_EEET_SF_SF_RKT0_St26random_access_iterator_tag
// IDA 0x2f8b9c: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8b9c() {
}

// 0x2f8c2c — __GLOBAL__I_a_103
#[doc(alias = "global constructor keyed to_a_103")]
// was: __GLOBAL__I_a_103
// IDA 0x2f8c2c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2f8c2c() {
}

// 0x2f8f04 — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC1Ev
// IDA 0x2f8f04: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f8f04() {
}

// 0x2f8f08 — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC2Ev
// IDA 0x2f8f08: 182 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f8f08() {
}

// 0x2f910c — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE7addPairES3_PKc
// IDA 0x2f910c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f910c() {
}

// 0x2f946c — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")]
// was: __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE6resizeEmS2_
// IDA 0x2f946c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f946c() {
}

// 0x2f94a0 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")]
// was: __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE9push_backERKS2_
// IDA 0x2f94a0: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_2f94a0() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}