//! core shard AO — 100 core stubs EA-sorted, next uncovered after AN 0xf6ea78..0x253f70 (low-EA gap continuation).
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x253f70 (earliest low-EA gap).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardName(char const*)")]
// 0x253f70 — __ZN3RBX10RbxDbgInfo14SetGfxCardNameEPKc
pub fn stub_0x253f70() -> ! {
    todo!("0x253f70 __ZN3RBX10RbxDbgInfo14SetGfxCardNameEPKc")
}

#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardDriverVersion(char const*)")]
// 0x253f94 — __ZN3RBX10RbxDbgInfo23SetGfxCardDriverVersionEPKc
pub fn stub_0x253f94() -> ! {
    todo!("0x253f94 __ZN3RBX10RbxDbgInfo23SetGfxCardDriverVersionEPKc")
}

#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardVendor(char const*)")]
// 0x253fb8 — __ZN3RBX10RbxDbgInfo16SetGfxCardVendorEPKc
pub fn stub_0x253fb8() -> ! {
    todo!("0x253fb8 __ZN3RBX10RbxDbgInfo16SetGfxCardVendorEPKc")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr()")]
// 0x2d11b8 — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev
pub fn stub_0x2d11b8() -> ! {
    todo!("0x2d11b8 __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseHover(RBX::UIEvent const&)")]
// 0x2d2a94 — __ZN3RBX15AdvMoveToolBase12onMouseHoverERKNS_7UIEventE
pub fn stub_0x2d2a94() -> ! {
    todo!("0x2d2a94 __ZN3RBX15AdvMoveToolBase12onMouseHoverERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseIdle(RBX::UIEvent const&)")]
// 0x2d2ab0 — __ZN3RBX15AdvMoveToolBase11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2d2ab0() -> ! {
    todo!("0x2d2ab0 __ZN3RBX15AdvMoveToolBase11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseDown(RBX::UIEvent const&)")]
// 0x2d2c3c — __ZN3RBX15AdvMoveToolBase11onMouseDownERKNS_7UIEventE
pub fn stub_0x2d2c3c() -> ! {
    todo!("0x2d2c3c __ZN3RBX15AdvMoveToolBase11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::saveAndModifyPartsTransparency(void)")]
// 0x2d2f40 — __ZN3RBX15AdvMoveToolBase30saveAndModifyPartsTransparencyEv
pub fn stub_0x2d2f40() -> ! {
    todo!("0x2d2f40 __ZN3RBX15AdvMoveToolBase30saveAndModifyPartsTransparencyEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseMove(RBX::UIEvent const&)")]
// 0x2d3174 — __ZN3RBX15AdvMoveToolBase11onMouseMoveERKNS_7UIEventE
pub fn stub_0x2d3174() -> ! {
    todo!("0x2d3174 __ZN3RBX15AdvMoveToolBase11onMouseMoveERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseUp(RBX::UIEvent const&)")]
// 0x2d421c — __ZN3RBX15AdvMoveToolBase9onMouseUpERKNS_7UIEventE
pub fn stub_0x2d421c() -> ! {
    todo!("0x2d421c __ZN3RBX15AdvMoveToolBase9onMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::restoreSavedPartsTransparency(void)")]
// 0x2d427c — __ZN3RBX15AdvMoveToolBase29restoreSavedPartsTransparencyEv
pub fn stub_0x2d427c() -> ! {
    todo!("0x2d427c __ZN3RBX15AdvMoveToolBase29restoreSavedPartsTransparencyEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::onKeyDown(RBX::UIEvent const&)")]
// 0x2d43a4 — __ZN3RBX15AdvMoveToolBase9onKeyDownERKNS_7UIEventE
pub fn stub_0x2d43a4() -> ! {
    todo!("0x2d43a4 __ZN3RBX15AdvMoveToolBase9onKeyDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::render2d(RBX::Adorn *)")]
// 0x2d448c — __ZN3RBX15AdvMoveToolBase8render2dEPNS_5AdornE
pub fn stub_0x2d448c() -> ! {
    todo!("0x2d448c __ZN3RBX15AdvMoveToolBase8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::AdvMoveToolBase::getExtents(RBX::Extents &)const")]
// 0x2d45b8 — __ZNK3RBX15AdvMoveToolBase10getExtentsERNS_7ExtentsE
pub fn stub_0x2d45b8() -> ! {
    todo!("0x2d45b8 __ZNK3RBX15AdvMoveToolBase10getExtentsERNS_7ExtentsE")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::render2d(RBX::Adorn *)")]
// 0x2d470c — __ZThn4_N3RBX15AdvMoveToolBase8render2dEPNS_5AdornE
// was: `non-virtual thunk to'RBX::AdvMoveToolBase::render2d(RBX::Adorn *)
pub fn stub_0x2d470c() -> ! {
    todo!("0x2d470c __ZThn4_N3RBX15AdvMoveToolBase8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::AdvMoveToolBase::render3dAdorn(RBX::Adorn *)")]
// 0x2d4714 — __ZN3RBX15AdvMoveToolBase13render3dAdornEPNS_5AdornE
pub fn stub_0x2d4714() -> ! {
    todo!("0x2d4714 __ZN3RBX15AdvMoveToolBase13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::render3dAdorn(RBX::Adorn *)")]
// 0x2d4874 — __ZThn4_N3RBX15AdvMoveToolBase13render3dAdornEPNS_5AdornE
// was: `non-virtual thunk to'RBX::AdvMoveToolBase::render3dAdorn(RBX::Adorn *)
pub fn stub_0x2d4874() -> ! {
    todo!("0x2d4874 __ZThn4_N3RBX15AdvMoveToolBase13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::AdvMoveTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2d496c — __ZN3RBX11AdvMoveTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2d496c() -> ! {
    todo!("0x2d496c __ZN3RBX11AdvMoveTool11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveTool::render2d(RBX::Adorn *)")]
// 0x2d4ac4 — __ZN3RBX11AdvMoveTool8render2dEPNS_5AdornE
pub fn stub_0x2d4ac4() -> ! {
    todo!("0x2d4ac4 __ZN3RBX11AdvMoveTool8render2dEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveTool::render2d(RBX::Adorn *)")]
// 0x2d504c — __ZThn4_N3RBX11AdvMoveTool8render2dEPNS_5AdornE
// was: `non-virtual thunk to'RBX::AdvMoveTool::render2d(RBX::Adorn *)
pub fn stub_0x2d504c() -> ! {
    todo!("0x2d504c __ZThn4_N3RBX11AdvMoveTool8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::DrawAdorn::resizeColor(void)")]
// 0x2d51bc — __ZN3RBX9DrawAdorn11resizeColorEv
pub fn stub_0x2d51bc() -> ! {
    todo!("0x2d51bc __ZN3RBX9DrawAdorn11resizeColorEv")
}

#[doc(alias = "RBX::AdvRotateTool::render2d(RBX::Adorn *)")]
// 0x2d5890 — __ZN3RBX13AdvRotateTool8render2dEPNS_5AdornE
pub fn stub_0x2d5890() -> ! {
    todo!("0x2d5890 __ZN3RBX13AdvRotateTool8render2dEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::AdvRotateTool::render2d(RBX::Adorn *)")]
// 0x2d5a28 — __ZThn4_N3RBX13AdvRotateTool8render2dEPNS_5AdornE
// was: `non-virtual thunk to'RBX::AdvRotateTool::render2d(RBX::Adorn *)
pub fn stub_0x2d5a28() -> ! {
    todo!("0x2d5a28 __ZThn4_N3RBX13AdvRotateTool8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::AdvRotateTool::render3dAdorn(RBX::Adorn *)")]
// 0x2d5a30 — __ZN3RBX13AdvRotateTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2d5a30() -> ! {
    todo!("0x2d5a30 __ZN3RBX13AdvRotateTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::AdvRotateTool::render3dAdorn(RBX::Adorn *)")]
// 0x2d5d98 — __ZThn4_N3RBX13AdvRotateTool13render3dAdornEPNS_5AdornE
// was: `non-virtual thunk to'RBX::AdvRotateTool::render3dAdorn(RBX::Adorn *)
pub fn stub_0x2d5d98() -> ! {
    todo!("0x2d5d98 __ZThn4_N3RBX13AdvRotateTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::AdvRotateTool::~AdvRotateTool()")]
// 0x2d5fd0 — __ZN3RBX13AdvRotateToolD1Ev
pub fn stub_0x2d5fd0() -> ! {
    todo!("0x2d5fd0 __ZN3RBX13AdvRotateToolD1Ev")
}

#[doc(alias = "RBX::AdvRotateTool::~AdvRotateTool()")]
// 0x2d5fd4 — __ZN3RBX13AdvRotateToolD0Ev
pub fn stub_0x2d5fd4() -> ! {
    todo!("0x2d5fd4 __ZN3RBX13AdvRotateToolD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AdvRotateTool::~AdvRotateTool()")]
// 0x2d6074 — __ZThn36_N3RBX13AdvRotateToolD1Ev
// was: `non-virtual thunk to'RBX::AdvRotateTool::~AdvRotateTool()
pub fn stub_0x2d6074() -> ! {
    todo!("0x2d6074 __ZThn36_N3RBX13AdvRotateToolD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AdvRotateTool::~AdvRotateTool()")]
// 0x2d607c — __ZThn36_N3RBX13AdvRotateToolD0Ev
// was: `non-virtual thunk to'RBX::AdvRotateTool::~AdvRotateTool()
pub fn stub_0x2d607c() -> ! {
    todo!("0x2d607c __ZThn36_N3RBX13AdvRotateToolD0Ev")
}

#[doc(alias = "RBX::AdvRunDragger::SnapInfo::updateSurfaceFromHit(void)")]
// 0x2d6390 — __ZN3RBX13AdvRunDragger8SnapInfo20updateSurfaceFromHitEv
pub fn stub_0x2d6390() -> ! {
    todo!("0x2d6390 __ZN3RBX13AdvRunDragger8SnapInfo20updateSurfaceFromHitEv")
}

#[doc(alias = "RBX::AdvRunDragger::SnapInfo::updateHitFromSurface(RBX::RbxRay const&)")]
// 0x2d64ac — __ZN3RBX13AdvRunDragger8SnapInfo20updateHitFromSurfaceERKNS_6RbxRayE
pub fn stub_0x2d64ac() -> ! {
    todo!("0x2d64ac __ZN3RBX13AdvRunDragger8SnapInfo20updateHitFromSurfaceERKNS_6RbxRayE")
}

#[doc(alias = "RBX::AdvRunDragger::SnapInfo::hitOutsideExtents(void)")]
// 0x2d6784 — __ZN3RBX13AdvRunDragger8SnapInfo17hitOutsideExtentsEv
pub fn stub_0x2d6784() -> ! {
    todo!("0x2d6784 __ZN3RBX13AdvRunDragger8SnapInfo17hitOutsideExtentsEv")
}

#[doc(alias = "RBX::AdvRunDragger::AdvRunDragger(void)")]
// 0x2d68d8 — __ZN3RBX13AdvRunDraggerC1Ev
pub fn stub_0x2d68d8() -> ! {
    todo!("0x2d68d8 __ZN3RBX13AdvRunDraggerC1Ev")
}

#[doc(alias = "RBX::AdvRunDragger::AdvRunDragger(void)")]
// 0x2d68dc — __ZN3RBX13AdvRunDraggerC2Ev
pub fn stub_0x2d68dc() -> ! {
    todo!("0x2d68dc __ZN3RBX13AdvRunDraggerC2Ev")
}

#[doc(alias = "RBX::AdvRunDragger::~AdvRunDragger()")]
// 0x2d6ac8 — __ZN3RBX13AdvRunDraggerD1Ev
pub fn stub_0x2d6ac8() -> ! {
    todo!("0x2d6ac8 __ZN3RBX13AdvRunDraggerD1Ev")
}

#[doc(alias = "RBX::AdvRunDragger::~AdvRunDragger()")]
// 0x2d6acc — __ZN3RBX13AdvRunDraggerD2Ev
pub fn stub_0x2d6acc() -> ! {
    todo!("0x2d6acc __ZN3RBX13AdvRunDraggerD2Ev")
}

#[doc(alias = "RBX::AdvRunDragger::snapInfoFromSnapPart(void)")]
// 0x2d6c30 — __ZN3RBX13AdvRunDragger20snapInfoFromSnapPartEv
pub fn stub_0x2d6c30() -> ! {
    todo!("0x2d6c30 __ZN3RBX13AdvRunDragger20snapInfoFromSnapPartEv")
}

#[doc(alias = "RBX::AdvRunDragger::snapPartFromSnapInfo(void)")]
// 0x2d6ed8 — __ZN3RBX13AdvRunDragger20snapPartFromSnapInfoEv
pub fn stub_0x2d6ed8() -> ! {
    todo!("0x2d6ed8 __ZN3RBX13AdvRunDragger20snapPartFromSnapInfoEv")
}

#[doc(alias = "RBX::AdvRunDragger::moveDragPart(void)")]
// 0x2d79c8 — __ZN3RBX13AdvRunDragger12moveDragPartEv
pub fn stub_0x2d79c8() -> ! {
    todo!("0x2d79c8 __ZN3RBX13AdvRunDragger12moveDragPartEv")
}

#[doc(alias = "RBX::AdvRunDragger::getSnapSurfaceCoord(void)")]
// 0x2d83d0 — __ZN3RBX13AdvRunDragger19getSnapSurfaceCoordEv
pub fn stub_0x2d83d0() -> ! {
    todo!("0x2d83d0 __ZN3RBX13AdvRunDragger19getSnapSurfaceCoordEv")
}

#[doc(alias = "RBX::AdvRunDragger::snapDragPart(void)")]
// 0x2d8564 — __ZN3RBX13AdvRunDragger12snapDragPartEv
pub fn stub_0x2d8564() -> ! {
    todo!("0x2d8564 __ZN3RBX13AdvRunDragger12snapDragPartEv")
}

#[doc(alias = "RBX::AdvRunDragger::adjacent(RBX::Primitive *,RBX::Primitive *)")]
// 0x2d8a78 — __ZN3RBX13AdvRunDragger8adjacentEPNS_9PrimitiveES2_
pub fn stub_0x2d8a78() -> ! {
    todo!("0x2d8a78 __ZN3RBX13AdvRunDragger8adjacentEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::AdvRunDragger::fallOffEdge(void)")]
// 0x2d8de4 — __ZN3RBX13AdvRunDragger11fallOffEdgeEv
pub fn stub_0x2d8de4() -> ! {
    todo!("0x2d8de4 __ZN3RBX13AdvRunDragger11fallOffEdgeEv")
}

#[doc(alias = "RBX::AdvRunDragger::fallOffPart(bool &)")]
// 0x2d8e1c — __ZN3RBX13AdvRunDragger11fallOffPartERb
pub fn stub_0x2d8e1c() -> ! {
    todo!("0x2d8e1c __ZN3RBX13AdvRunDragger11fallOffPartERb")
}

#[doc(alias = "RBX::AdvRunDragger::rayHitsCloserPart(void)")]
// 0x2d8f8c — __ZN3RBX13AdvRunDragger17rayHitsCloserPartEv
pub fn stub_0x2d8f8c() -> ! {
    todo!("0x2d8f8c __ZN3RBX13AdvRunDragger17rayHitsCloserPartEv")
}

#[doc(alias = "RBX::AdvRunDragger::tooCloseToCamera(void)")]
// 0x2d90dc — __ZN3RBX13AdvRunDragger16tooCloseToCameraEv
pub fn stub_0x2d90dc() -> ! {
    todo!("0x2d90dc __ZN3RBX13AdvRunDragger16tooCloseToCameraEv")
}

#[doc(alias = "RBX::AdvRunDragger::findSafeY(void)")]
// 0x2d9430 — __ZN3RBX13AdvRunDragger9findSafeYEv
pub fn stub_0x2d9430() -> ! {
    todo!("0x2d9430 __ZN3RBX13AdvRunDragger9findSafeYEv")
}

#[doc(alias = "RBX::AdvRunDragger::snap(RBX::RbxRay const&)")]
// 0x2d9ae0 — __ZN3RBX13AdvRunDragger4snapERKNS_6RbxRayE
pub fn stub_0x2d9ae0() -> ! {
    todo!("0x2d9ae0 __ZN3RBX13AdvRunDragger4snapERKNS_6RbxRayE")
}

#[doc(alias = "RBX::AdvRunDragger::shouldRender3dAdorn(void)const")]
// 0x2d9d4c — __ZNK3RBX13AdvRunDragger19shouldRender3dAdornEv
pub fn stub_0x2d9d4c() -> ! {
    todo!("0x2d9d4c __ZNK3RBX13AdvRunDragger19shouldRender3dAdornEv")
}

#[doc(alias = "RBX::AxisToolBase::onMouseHover(RBX::UIEvent const&)")]
// 0x2da2d0 — __ZN3RBX12AxisToolBase12onMouseHoverERKNS_7UIEventE
pub fn stub_0x2da2d0() -> ! {
    todo!("0x2da2d0 __ZN3RBX12AxisToolBase12onMouseHoverERKNS_7UIEventE")
}

#[doc(alias = "RBX::AxisToolBase::onMouseIdle(RBX::UIEvent const&)")]
// 0x2da2d8 — __ZN3RBX12AxisToolBase11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2da2d8() -> ! {
    todo!("0x2da2d8 __ZN3RBX12AxisToolBase11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::AxisToolBase::onMouseDown(RBX::UIEvent const&)")]
// 0x2da450 — __ZN3RBX12AxisToolBase11onMouseDownERKNS_7UIEventE
pub fn stub_0x2da450() -> ! {
    todo!("0x2da450 __ZN3RBX12AxisToolBase11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::AxisToolBase::onMouseMove(RBX::UIEvent const&)")]
// 0x2da788 — __ZN3RBX12AxisToolBase11onMouseMoveERKNS_7UIEventE
pub fn stub_0x2da788() -> ! {
    todo!("0x2da788 __ZN3RBX12AxisToolBase11onMouseMoveERKNS_7UIEventE")
}

#[doc(alias = "RBX::AxisToolBase::onMouseUp(RBX::UIEvent const&)")]
// 0x2dac1c — __ZN3RBX12AxisToolBase9onMouseUpERKNS_7UIEventE
pub fn stub_0x2dac1c() -> ! {
    todo!("0x2dac1c __ZN3RBX12AxisToolBase9onMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::AxisToolBase::render2d(RBX::Adorn *)")]
// 0x2dac5c — __ZN3RBX12AxisToolBase8render2dEPNS_5AdornE
pub fn stub_0x2dac5c() -> ! {
    todo!("0x2dac5c __ZN3RBX12AxisToolBase8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::AxisToolBase::getExtents(RBX::Extents &)const")]
// 0x2dad94 — __ZNK3RBX12AxisToolBase10getExtentsERNS_7ExtentsE
pub fn stub_0x2dad94() -> ! {
    todo!("0x2dad94 __ZNK3RBX12AxisToolBase10getExtentsERNS_7ExtentsE")
}

#[doc(alias = "non-virtual thunk toRBX::AxisToolBase::render2d(RBX::Adorn *)")]
// 0x2daee8 — __ZThn4_N3RBX12AxisToolBase8render2dEPNS_5AdornE
// was: `non-virtual thunk to'RBX::AxisToolBase::render2d(RBX::Adorn *)
pub fn stub_0x2daee8() -> ! {
    todo!("0x2daee8 __ZThn4_N3RBX12AxisToolBase8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::AxisToolBase::render3dAdorn(RBX::Adorn *)")]
// 0x2daef0 — __ZN3RBX12AxisToolBase13render3dAdornEPNS_5AdornE
pub fn stub_0x2daef0() -> ! {
    todo!("0x2daef0 __ZN3RBX12AxisToolBase13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::AxisToolBase::render3dAdorn(RBX::Adorn *)")]
// 0x2db050 — __ZThn4_N3RBX12AxisToolBase13render3dAdornEPNS_5AdornE
// was: `non-virtual thunk to'RBX::AxisToolBase::render3dAdorn(RBX::Adorn *)
pub fn stub_0x2db050() -> ! {
    todo!("0x2db050 __ZThn4_N3RBX12AxisToolBase13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "std::auto_ptr<RBX::MegaDragger>::reset(RBX::MegaDragger*)")]
// 0x2db1c0 — __ZNSt8auto_ptrIN3RBX11MegaDraggerEE5resetEPS1_
pub fn stub_0x2db1c0() -> ! {
    todo!("0x2db1c0 __ZNSt8auto_ptrIN3RBX11MegaDraggerEE5resetEPS1_")
}

#[doc(alias = "RBX::CloneTool::~CloneTool()")]
// 0x2db8b0 — __ZN3RBX9CloneToolD0Ev
pub fn stub_0x2db8b0() -> ! {
    todo!("0x2db8b0 __ZN3RBX9CloneToolD0Ev")
}

#[doc(alias = "RBX::CloneTool::~CloneTool()")]
// 0x2db950 — __ZN3RBX9CloneToolD1Ev
pub fn stub_0x2db950() -> ! {
    todo!("0x2db950 __ZN3RBX9CloneToolD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CloneTool::~CloneTool()")]
// 0x2db954 — __ZThn36_N3RBX9CloneToolD0Ev
// was: `non-virtual thunk to'RBX::CloneTool::~CloneTool()
pub fn stub_0x2db954() -> ! {
    todo!("0x2db954 __ZThn36_N3RBX9CloneToolD0Ev")
}

#[doc(alias = "RBX::CloneTool::~CloneTool()")]
// 0x2db95c — __ZN3RBX9CloneToolD2Ev
pub fn stub_0x2db95c() -> ! {
    todo!("0x2db95c __ZN3RBX9CloneToolD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CloneTool::~CloneTool()")]
// 0x2dba78 — __ZThn36_N3RBX9CloneToolD1Ev
// was: `non-virtual thunk to'RBX::CloneTool::~CloneTool()
pub fn stub_0x2dba78() -> ! {
    todo!("0x2dba78 __ZThn36_N3RBX9CloneToolD1Ev")
}

#[doc(alias = "RBX::CloneTool::onMouseIdle(RBX::UIEvent const&)")]
// 0x2dba80 — __ZN3RBX9CloneTool11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2dba80() -> ! {
    todo!("0x2dba80 __ZN3RBX9CloneTool11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::CloneTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2dbb58 — __ZN3RBX9CloneTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2dbb58() -> ! {
    todo!("0x2dbb58 __ZN3RBX9CloneTool11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::CloneTool::getCursorName(void)const")]
// 0x2dbda0 — __ZNK3RBX9CloneTool13getCursorNameEv
pub fn stub_0x2dbda0() -> ! {
    todo!("0x2dbda0 __ZNK3RBX9CloneTool13getCursorNameEv")
}

#[doc(alias = "RBX::CloneTool::isSticky(void)const")]
// 0x2dbfb0 — __ZNK3RBX9CloneTool8isStickyEv
pub fn stub_0x2dbfb0() -> ! {
    todo!("0x2dbfb0 __ZNK3RBX9CloneTool8isStickyEv")
}

#[doc(alias = "RBX::CloneTool::drawConnectors(void)const")]
// 0x2dc078 — __ZNK3RBX9CloneTool14drawConnectorsEv
pub fn stub_0x2dc078() -> ! {
    todo!("0x2dc078 __ZNK3RBX9CloneTool14drawConnectorsEv")
}

#[doc(alias = "RBX::Dragger::computeExtents(std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> const&)")]
// 0x2dc604 — __ZN3RBX7Dragger14computeExtentsERKSt6vectorIPNS_9PrimitiveESaIS3_EE
pub fn stub_0x2dc604() -> ! {
    todo!("0x2dc604 __ZN3RBX7Dragger14computeExtentsERKSt6vectorIPNS_9PrimitiveESaIS3_EE")
}

#[doc(alias = "RBX::Extents::negativeMaxExtents(void)")]
// 0x2dfc98 — __ZN3RBX7Extents18negativeMaxExtentsEv
pub fn stub_0x2dfc98() -> ! {
    todo!("0x2dfc98 __ZN3RBX7Extents18negativeMaxExtentsEv")
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long)")]
// 0x2dfe04 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm
pub fn stub_0x2dfe04() -> ! {
    todo!("0x2dfe04 __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&)")]
// 0x2dfea0 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_
pub fn stub_0x2dfea0() -> ! {
    todo!("0x2dfea0 __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&)")]
// 0x2dff0c — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x2dff0c() -> ! {
    todo!("0x2dff0c __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long)")]
// 0x2e00a0 — __ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm
pub fn stub_0x2e00a0() -> ! {
    todo!("0x2e00a0 __ZNSt12_Vector_baseIN3RBX7ExtentsESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *)")]
// 0x2e00c4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_
pub fn stub_0x2e00c4() -> ! {
    todo!("0x2e00c4 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7ExtentsES5_EET0_T_S7_S6_")
}

#[doc(alias = "RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const")]
// 0x2e01b0 — __ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm
pub fn stub_0x2e01b0() -> ! {
    todo!("0x2e01b0 __ZNK3RBX4POLY4Edge9getVertexEPKNS0_4FaceEm")
}

#[doc(alias = "RBX::GameTool::onMouseIdle(RBX::UIEvent const&)")]
// 0x2e3044 — __ZN3RBX8GameTool11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2e3044() -> ! {
    todo!("0x2e3044 __ZN3RBX8GameTool11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::GameTool::onMouseHover(RBX::UIEvent const&)")]
// 0x2e3080 — __ZN3RBX8GameTool12onMouseHoverERKNS_7UIEventE
pub fn stub_0x2e3080() -> ! {
    todo!("0x2e3080 __ZN3RBX8GameTool12onMouseHoverERKNS_7UIEventE")
}

#[doc(alias = "RBX::GameTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2e30f4 — __ZN3RBX8GameTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2e30f4() -> ! {
    todo!("0x2e30f4 __ZN3RBX8GameTool11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::GameTool::~GameTool()")]
// 0x2e3234 — __ZN3RBX8GameToolD0Ev
pub fn stub_0x2e3234() -> ! {
    todo!("0x2e3234 __ZN3RBX8GameToolD0Ev")
}

#[doc(alias = "RBX::GameTool::~GameTool()")]
// 0x2e32d4 — __ZN3RBX8GameToolD1Ev
pub fn stub_0x2e32d4() -> ! {
    todo!("0x2e32d4 __ZN3RBX8GameToolD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GameTool::~GameTool()")]
// 0x2e32d8 — __ZThn36_N3RBX8GameToolD0Ev
// was: `non-virtual thunk to'RBX::GameTool::~GameTool()
pub fn stub_0x2e32d8() -> ! {
    todo!("0x2e32d8 __ZThn36_N3RBX8GameToolD0Ev")
}

#[doc(alias = "RBX::GameTool::~GameTool()")]
// 0x2e32e0 — __ZN3RBX8GameToolD2Ev
pub fn stub_0x2e32e0() -> ! {
    todo!("0x2e32e0 __ZN3RBX8GameToolD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GameTool::~GameTool()")]
// 0x2e33e4 — __ZThn36_N3RBX8GameToolD1Ev
// was: `non-virtual thunk to'RBX::GameTool::~GameTool()
pub fn stub_0x2e33e4() -> ! {
    todo!("0x2e33e4 __ZThn36_N3RBX8GameToolD1Ev")
}

#[doc(alias = "RBX::GameTool::isSticky(void)const")]
// 0x2e3414 — __ZNK3RBX8GameTool8isStickyEv
pub fn stub_0x2e3414() -> ! {
    todo!("0x2e3414 __ZNK3RBX8GameTool8isStickyEv")
}

#[doc(alias = "RBX::GameTool::drawConnectors(void)const")]
// 0x2e34dc — __ZNK3RBX8GameTool14drawConnectorsEv
pub fn stub_0x2e34dc() -> ! {
    todo!("0x2e34dc __ZNK3RBX8GameTool14drawConnectorsEv")
}

#[doc(alias = "RBX::GameTool::getCursorName(void)const")]
// 0x2e34e0 — __ZNK3RBX8GameTool13getCursorNameEv
pub fn stub_0x2e34e0() -> ! {
    todo!("0x2e34e0 __ZNK3RBX8GameTool13getCursorNameEv")
}

#[doc(alias = "RBX::GrabTool::onMouseIdle(RBX::UIEvent const&)")]
// 0x2e38e8 — __ZN3RBX8GrabTool11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2e38e8() -> ! {
    todo!("0x2e38e8 __ZN3RBX8GrabTool11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::GrabTool::onMouseHover(RBX::UIEvent const&)")]
// 0x2e38f0 — __ZN3RBX8GrabTool12onMouseHoverERKNS_7UIEventE
pub fn stub_0x2e38f0() -> ! {
    todo!("0x2e38f0 __ZN3RBX8GrabTool12onMouseHoverERKNS_7UIEventE")
}

#[doc(alias = "RBX::GrabTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2e395c — __ZN3RBX8GrabTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2e395c() -> ! {
    todo!("0x2e395c __ZN3RBX8GrabTool11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::GrabTool::~GrabTool()")]
// 0x2e3aa8 — __ZN3RBX8GrabToolD0Ev
pub fn stub_0x2e3aa8() -> ! {
    todo!("0x2e3aa8 __ZN3RBX8GrabToolD0Ev")
}

#[doc(alias = "RBX::GrabTool::~GrabTool()")]
// 0x2e3b48 — __ZN3RBX8GrabToolD1Ev
pub fn stub_0x2e3b48() -> ! {
    todo!("0x2e3b48 __ZN3RBX8GrabToolD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GrabTool::~GrabTool()")]
// 0x2e3b4c — __ZThn36_N3RBX8GrabToolD0Ev
// was: `non-virtual thunk to'RBX::GrabTool::~GrabTool()
pub fn stub_0x2e3b4c() -> ! {
    todo!("0x2e3b4c __ZThn36_N3RBX8GrabToolD0Ev")
}

#[doc(alias = "RBX::GrabTool::~GrabTool()")]
// 0x2e3b54 — __ZN3RBX8GrabToolD2Ev
pub fn stub_0x2e3b54() -> ! {
    todo!("0x2e3b54 __ZN3RBX8GrabToolD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GrabTool::~GrabTool()")]
// 0x2e3c58 — __ZThn36_N3RBX8GrabToolD1Ev
// was: `non-virtual thunk to'RBX::GrabTool::~GrabTool()
pub fn stub_0x2e3c58() -> ! {
    todo!("0x2e3c58 __ZThn36_N3RBX8GrabToolD1Ev")
}

#[doc(alias = "RBX::GrabTool::isSticky(void)const")]
// 0x2e3c88 — __ZNK3RBX8GrabTool8isStickyEv
pub fn stub_0x2e3c88() -> ! {
    todo!("0x2e3c88 __ZNK3RBX8GrabTool8isStickyEv")
}

#[doc(alias = "RBX::GrabTool::drawConnectors(void)const")]
// 0x2e3d50 — __ZNK3RBX8GrabTool14drawConnectorsEv
pub fn stub_0x2e3d50() -> ! {
    todo!("0x2e3d50 __ZNK3RBX8GrabTool14drawConnectorsEv")
}

#[doc(alias = "RBX::GrabTool::getCursorName(void)const")]
// 0x2e3d54 — __ZNK3RBX8GrabTool13getCursorNameEv
pub fn stub_0x2e3d54() -> ! {
    todo!("0x2e3d54 __ZNK3RBX8GrabTool13getCursorNameEv")
}
