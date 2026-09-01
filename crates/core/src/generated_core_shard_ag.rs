//! core shard AG — 120 core stubs EA-sorted, earliest gap (lowest uncovered) after prior shards.
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 120 uncovered (lowest EA first).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*)")]
// 0x2e7428 — __ZNSt8auto_ptrIN3RBX10RunDraggerEE5resetEPS1_
pub fn stub_0x2e7428() -> ! {
    todo!("0x2e7428 std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*)")
}

#[doc(alias = "std::auto_ptr<RBX::RunDragger>::~auto_ptr()")]
// 0x2e81b8 — __ZNSt8auto_ptrIN3RBX10RunDraggerEED2Ev
pub fn stub_0x2e81b8() -> ! {
    todo!("0x2e81b8 std::auto_ptr<RBX::RunDragger>::~auto_ptr()")
}

#[doc(alias = "__GLOBAL__I_a_95")]
// 0x2e9b08 — __GLOBAL__I_a_95
pub fn stub_0x2e9b08() -> ! {
    todo!("0x2e9b08 __GLOBAL__I_a_95")
}

#[doc(alias = "__GLOBAL__I_a_96")]
// 0x2eac30 — __GLOBAL__I_a_96
pub fn stub_0x2eac30() -> ! {
    todo!("0x2eac30 __GLOBAL__I_a_96")
}

#[doc(alias = "RBX::MegaDragger::~MegaDragger()")]
// 0x2eb0e8 — __ZN3RBX11MegaDraggerD1Ev
pub fn stub_0x2eb0e8() -> ! {
    todo!("0x2eb0e8 RBX::MegaDragger::~MegaDragger()")
}

#[doc(alias = "__ZN3RBX11MegaDraggerD2Ev")]
// 0x2eb0ec — __ZN3RBX11MegaDraggerD2Ev
pub fn stub_0x2eb0ec() -> ! {
    todo!("0x2eb0ec __ZN3RBX11MegaDraggerD2Ev")
}

#[doc(alias = "RBX::MegaDragger::startDragging(void)")]
// 0x2eb224 — __ZN3RBX11MegaDragger13startDraggingEv
pub fn stub_0x2eb224() -> ! {
    todo!("0x2eb224 RBX::MegaDragger::startDragging(void)")
}

#[doc(alias = "RBX::MegaDragger::continueDragging(void)")]
// 0x2eb248 — __ZN3RBX11MegaDragger16continueDraggingEv
pub fn stub_0x2eb248() -> ! {
    todo!("0x2eb248 RBX::MegaDragger::continueDragging(void)")
}

#[doc(alias = "RBX::MegaDragger::finishDragging(void)")]
// 0x2eb2b4 — __ZN3RBX11MegaDragger14finishDraggingEv
pub fn stub_0x2eb2b4() -> ! {
    todo!("0x2eb2b4 RBX::MegaDragger::finishDragging(void)")
}

#[doc(alias = "RBX::MegaDragger::alignAndCleanParts(void)")]
// 0x2eb380 — __ZN3RBX11MegaDragger18alignAndCleanPartsEv
pub fn stub_0x2eb380() -> ! {
    todo!("0x2eb380 RBX::MegaDragger::alignAndCleanParts(void)")
}

#[doc(alias = "RBX::MegaDragger::mousePartAlive(void)")]
// 0x2eb540 — __ZN3RBX11MegaDragger14mousePartAliveEv
pub fn stub_0x2eb540() -> ! {
    todo!("0x2eb540 RBX::MegaDragger::mousePartAlive(void)")
}

#[doc(alias = "RBX::MegaDragger::anyDragPartAlive(void)")]
// 0x2ebf7c — __ZN3RBX11MegaDragger16anyDragPartAliveEv
pub fn stub_0x2ebf7c() -> ! {
    todo!("0x2ebf7c RBX::MegaDragger::anyDragPartAlive(void)")
}

#[doc(alias = "__GLOBAL__I_a_97")]
// 0x2ebf88 — __GLOBAL__I_a_97
pub fn stub_0x2ebf88() -> ! {
    todo!("0x2ebf88 __GLOBAL__I_a_97")
}

#[doc(alias = "RBX::MoveResizeJoinTool::findTargetPV(RBX::UIEvent const&)")]
// 0x2ec2fc — __ZN3RBX18MoveResizeJoinTool12findTargetPVERKNS_7UIEventE
pub fn stub_0x2ec2fc() -> ! {
    todo!("0x2ec2fc RBX::MoveResizeJoinTool::findTargetPV(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseHover(RBX::UIEvent const&)")]
// 0x2ed9dc — __ZN3RBX18MoveResizeJoinTool12onMouseHoverERKNS_7UIEventE
pub fn stub_0x2ed9dc() -> ! {
    todo!("0x2ed9dc RBX::MoveResizeJoinTool::onMouseHover(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseIdle(RBX::UIEvent const&)")]
// 0x2eda60 — __ZN3RBX18MoveResizeJoinTool11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2eda60() -> ! {
    todo!("0x2eda60 RBX::MoveResizeJoinTool::onMouseIdle(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2edbcc — __ZN3RBX18MoveResizeJoinTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2edbcc() -> ! {
    todo!("0x2edbcc RBX::MoveResizeJoinTool::onMouseDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::moveIncrement(void)")]
// 0x2ede04 — __ZN3RBX18MoveResizeJoinTool13moveIncrementEv
pub fn stub_0x2ede04() -> ! {
    todo!("0x2ede04 RBX::MoveResizeJoinTool::moveIncrement(void)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::onKeyDown(RBX::UIEvent const&)")]
// 0x2edf9c — __ZN3RBX18MoveResizeJoinTool9onKeyDownERKNS_7UIEventE
pub fn stub_0x2edf9c() -> ! {
    todo!("0x2edf9c RBX::MoveResizeJoinTool::onKeyDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseMove(RBX::UIEvent const&)")]
// 0x2ee084 — __ZN3RBX18MoveResizeJoinTool11onMouseMoveERKNS_7UIEventE
pub fn stub_0x2ee084() -> ! {
    todo!("0x2ee084 RBX::MoveResizeJoinTool::onMouseMove(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::capturedDrag(float)")]
// 0x2ee324 — __ZN3RBX18MoveResizeJoinTool12capturedDragEf
pub fn stub_0x2ee324() -> ! {
    todo!("0x2ee324 RBX::MoveResizeJoinTool::capturedDrag(float)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::onMouseUp(RBX::UIEvent const&)")]
// 0x2ee4e4 — __ZN3RBX18MoveResizeJoinTool9onMouseUpERKNS_7UIEventE
pub fn stub_0x2ee4e4() -> ! {
    todo!("0x2ee4e4 RBX::MoveResizeJoinTool::onMouseUp(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::MoveResizeJoinTool::~MoveResizeJoinTool()")]
// 0x2ee818 — __ZN3RBX18MoveResizeJoinToolD1Ev
pub fn stub_0x2ee818() -> ! {
    todo!("0x2ee818 RBX::MoveResizeJoinTool::~MoveResizeJoinTool()")
}

#[doc(alias = "__ZN3RBX18MoveResizeJoinToolD0Ev")]
// 0x2ee900 — __ZN3RBX18MoveResizeJoinToolD0Ev
pub fn stub_0x2ee900() -> ! {
    todo!("0x2ee900 __ZN3RBX18MoveResizeJoinToolD0Ev")
}

#[doc(alias = "__ZThn36_N3RBX18MoveResizeJoinToolD1Ev")]
// 0x2ee9f8 — __ZThn36_N3RBX18MoveResizeJoinToolD1Ev
pub fn stub_0x2ee9f8() -> ! {
    todo!("0x2ee9f8 __ZThn36_N3RBX18MoveResizeJoinToolD1Ev")
}

#[doc(alias = "__ZThn36_N3RBX18MoveResizeJoinToolD0Ev")]
// 0x2eeadc — __ZThn36_N3RBX18MoveResizeJoinToolD0Ev
pub fn stub_0x2eeadc() -> ! {
    todo!("0x2eeadc __ZThn36_N3RBX18MoveResizeJoinToolD0Ev")
}

#[doc(alias = "__GLOBAL__I_a_98")]
// 0x2eebd8 — __GLOBAL__I_a_98
pub fn stub_0x2eebd8() -> ! {
    todo!("0x2eebd8 __GLOBAL__I_a_98")
}

#[doc(alias = "RBX::NullTool::~NullTool()")]
// 0x2eef84 — __ZN3RBX8NullToolD0Ev
pub fn stub_0x2eef84() -> ! {
    todo!("0x2eef84 RBX::NullTool::~NullTool()")
}

#[doc(alias = "__ZN3RBX8NullToolD1Ev")]
// 0x2ef024 — __ZN3RBX8NullToolD1Ev
pub fn stub_0x2ef024() -> ! {
    todo!("0x2ef024 __ZN3RBX8NullToolD1Ev")
}

#[doc(alias = "__ZThn36_N3RBX8NullToolD0Ev")]
// 0x2ef028 — __ZThn36_N3RBX8NullToolD0Ev
pub fn stub_0x2ef028() -> ! {
    todo!("0x2ef028 __ZThn36_N3RBX8NullToolD0Ev")
}

#[doc(alias = "__ZN3RBX8NullToolD2Ev")]
// 0x2ef030 — __ZN3RBX8NullToolD2Ev
pub fn stub_0x2ef030() -> ! {
    todo!("0x2ef030 __ZN3RBX8NullToolD2Ev")
}

#[doc(alias = "__ZThn36_N3RBX8NullToolD1Ev")]
// 0x2ef124 — __ZThn36_N3RBX8NullToolD1Ev
pub fn stub_0x2ef124() -> ! {
    todo!("0x2ef124 __ZThn36_N3RBX8NullToolD1Ev")
}

#[doc(alias = "RBX::NewNullTool::~NewNullTool()")]
// 0x2ef22c — __ZN3RBX11NewNullToolD0Ev
pub fn stub_0x2ef22c() -> ! {
    todo!("0x2ef22c RBX::NewNullTool::~NewNullTool()")
}

#[doc(alias = "__ZN3RBX11NewNullToolD1Ev")]
// 0x2ef2f4 — __ZN3RBX11NewNullToolD1Ev
pub fn stub_0x2ef2f4() -> ! {
    todo!("0x2ef2f4 __ZN3RBX11NewNullToolD1Ev")
}

#[doc(alias = "__ZThn36_N3RBX11NewNullToolD0Ev")]
// 0x2ef328 — __ZThn36_N3RBX11NewNullToolD0Ev
pub fn stub_0x2ef328() -> ! {
    todo!("0x2ef328 __ZThn36_N3RBX11NewNullToolD0Ev")
}

#[doc(alias = "__ZThn36_N3RBX11NewNullToolD1Ev")]
// 0x2ef330 — __ZThn36_N3RBX11NewNullToolD1Ev
pub fn stub_0x2ef330() -> ! {
    todo!("0x2ef330 __ZThn36_N3RBX11NewNullToolD1Ev")
}

#[doc(alias = "RBX::NewNullTool::onMouseIdle(RBX::UIEvent const&)")]
// 0x2ef48c — __ZN3RBX11NewNullTool11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2ef48c() -> ! {
    todo!("0x2ef48c RBX::NewNullTool::onMouseIdle(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::NewNullTool::updateClickDetectorHover(RBX::UIEvent const&)")]
// 0x2ef694 — __ZN3RBX11NewNullTool24updateClickDetectorHoverERKNS_7UIEventE
pub fn stub_0x2ef694() -> ! {
    todo!("0x2ef694 RBX::NewNullTool::updateClickDetectorHover(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::NewNullTool::onMouseHover(RBX::UIEvent const&)")]
// 0x2ef888 — __ZN3RBX11NewNullTool12onMouseHoverERKNS_7UIEventE
pub fn stub_0x2ef888() -> ! {
    todo!("0x2ef888 RBX::NewNullTool::onMouseHover(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::NewNullTool::onRightMouseDown(RBX::UIEvent const&)")]
// 0x2efb14 — __ZN3RBX11NewNullTool16onRightMouseDownERKNS_7UIEventE
pub fn stub_0x2efb14() -> ! {
    todo!("0x2efb14 RBX::NewNullTool::onRightMouseDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::NewNullTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2efc0c — __ZN3RBX11NewNullTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2efc0c() -> ! {
    todo!("0x2efc0c RBX::NewNullTool::onMouseDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::NewNullTool::onRightMouseUp(RBX::UIEvent const&)")]
// 0x2efd44 — __ZN3RBX11NewNullTool14onRightMouseUpERKNS_7UIEventE
pub fn stub_0x2efd44() -> ! {
    todo!("0x2efd44 RBX::NewNullTool::onRightMouseUp(RBX::UIEvent const&)")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_9sNullToolEEE7getNameEv")]
// 0x2f0060 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sNullToolEEE7getNameEv
pub fn stub_0x2f0060() -> ! {
    todo!("0x2f0060 __ZNK3RBX5NamedINS_12MouseCommandELZNS_9sNullToolEEE7getNameEv")
}

#[doc(alias = "RBX::NullTool::onMouseUp(RBX::UIEvent const&)")]
// 0x2f0088 — __ZN3RBX8NullTool9onMouseUpERKNS_7UIEventE
pub fn stub_0x2f0088() -> ! {
    todo!("0x2f0088 RBX::NullTool::onMouseUp(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::NullTool::isSticky(void)const")]
// 0x2f015c — __ZNK3RBX8NullTool8isStickyEv
pub fn stub_0x2f015c() -> ! {
    todo!("0x2f015c RBX::NullTool::isSticky(void)const")
}

#[doc(alias = "RBX::NullTool::getCursorName(void)const")]
// 0x2f0224 — __ZNK3RBX8NullTool13getCursorNameEv
pub fn stub_0x2f0224() -> ! {
    todo!("0x2f0224 RBX::NullTool::getCursorName(void)const")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_12sNewNullToolEEE7getNameEv")]
// 0x2f0240 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_12sNewNullToolEEE7getNameEv
pub fn stub_0x2f0240() -> ! {
    todo!("0x2f0240 __ZNK3RBX5NamedINS_12MouseCommandELZNS_12sNewNullToolEEE7getNameEv")
}

#[doc(alias = "RBX::NewNullTool::onMouseUp(RBX::UIEvent const&)")]
// 0x2f0268 — __ZN3RBX11NewNullTool9onMouseUpERKNS_7UIEventE
pub fn stub_0x2f0268() -> ! {
    todo!("0x2f0268 RBX::NewNullTool::onMouseUp(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::NewNullTool::isSticky(void)const")]
// 0x2f033c — __ZNK3RBX11NewNullTool8isStickyEv
pub fn stub_0x2f033c() -> ! {
    todo!("0x2f033c RBX::NewNullTool::isSticky(void)const")
}

#[doc(alias = "RBX::NewNullTool::getCursorName(void)const")]
// 0x2f0404 — __ZNK3RBX11NewNullTool13getCursorNameEv
pub fn stub_0x2f0404() -> ! {
    todo!("0x2f0404 RBX::NewNullTool::getCursorName(void)const")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sNewNullToolEEEEvv")]
// 0x2f0418 — __ZN3RBX4Name13callDoDeclareILZNS_12sNewNullToolEEEEvv
pub fn stub_0x2f0418() -> ! {
    todo!("0x2f0418 __ZN3RBX4Name13callDoDeclareILZNS_12sNewNullToolEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")]
// 0x2f041c — __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v
pub fn stub_0x2f041c() -> ! {
    todo!("0x2f041c __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")
}

#[doc(alias = "__GLOBAL__I_a_99")]
// 0x2f0664 — __GLOBAL__I_a_99
pub fn stub_0x2f0664() -> ! {
    todo!("0x2f0664 __GLOBAL__I_a_99")
}

#[doc(alias = "RBX::MegaDragger::getMousePart(void)")]
// 0x2f1754 — __ZN3RBX11MegaDragger12getMousePartEv
pub fn stub_0x2f1754() -> ! {
    todo!("0x2f1754 RBX::MegaDragger::getMousePart(void)")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_13sPartDragToolEEE7getNameEv")]
// 0x2f1808 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_13sPartDragToolEEE7getNameEv
pub fn stub_0x2f1808() -> ! {
    todo!("0x2f1808 __ZNK3RBX5NamedINS_12MouseCommandELZNS_13sPartDragToolEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sPartDragToolEEEEvv")]
// 0x2f1864 — __ZN3RBX4Name13callDoDeclareILZNS_13sPartDragToolEEEEvv
pub fn stub_0x2f1864() -> ! {
    todo!("0x2f1864 __ZN3RBX4Name13callDoDeclareILZNS_13sPartDragToolEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")]
// 0x2f1868 — __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v
pub fn stub_0x2f1868() -> ! {
    todo!("0x2f1868 __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")
}

#[doc(alias = "__GLOBAL__I_a_100")]
// 0x2f1948 — __GLOBAL__I_a_100
pub fn stub_0x2f1948() -> ! {
    todo!("0x2f1948 __GLOBAL__I_a_100")
}

#[doc(alias = "__GLOBAL__I_a_101")]
// 0x2f1c20 — __GLOBAL__I_a_101
pub fn stub_0x2f1c20() -> ! {
    todo!("0x2f1c20 __GLOBAL__I_a_101")
}

#[doc(alias = "RBX::RunDragger::SnapInfo::updateSurfaceFromHit(void)")]
// 0x2f1ef8 — __ZN3RBX10RunDragger8SnapInfo20updateSurfaceFromHitEv
pub fn stub_0x2f1ef8() -> ! {
    todo!("0x2f1ef8 RBX::RunDragger::SnapInfo::updateSurfaceFromHit(void)")
}

#[doc(alias = "RBX::RunDragger::SnapInfo::updateHitFromSurface(RBX::RbxRay const&)")]
// 0x2f1fb8 — __ZN3RBX10RunDragger8SnapInfo20updateHitFromSurfaceERKNS_6RbxRayE
pub fn stub_0x2f1fb8() -> ! {
    todo!("0x2f1fb8 RBX::RunDragger::SnapInfo::updateHitFromSurface(RBX::RbxRay const&)")
}

#[doc(alias = "RBX::RunDragger::SnapInfo::hitOutsideExtents(void)")]
// 0x2f229c — __ZN3RBX10RunDragger8SnapInfo17hitOutsideExtentsEv
pub fn stub_0x2f229c() -> ! {
    todo!("0x2f229c RBX::RunDragger::SnapInfo::hitOutsideExtents(void)")
}

#[doc(alias = "RBX::RunDragger::RunDragger(void)")]
// 0x2f23f0 — __ZN3RBX10RunDraggerC1Ev
pub fn stub_0x2f23f0() -> ! {
    todo!("0x2f23f0 RBX::RunDragger::RunDragger(void)")
}

#[doc(alias = "__ZN3RBX10RunDraggerC2Ev")]
// 0x2f23f4 — __ZN3RBX10RunDraggerC2Ev
pub fn stub_0x2f23f4() -> ! {
    todo!("0x2f23f4 __ZN3RBX10RunDraggerC2Ev")
}

#[doc(alias = "RBX::RunDragger::~RunDragger()")]
// 0x2f25ac — __ZN3RBX10RunDraggerD1Ev
pub fn stub_0x2f25ac() -> ! {
    todo!("0x2f25ac RBX::RunDragger::~RunDragger()")
}

#[doc(alias = "__ZN3RBX10RunDraggerD2Ev")]
// 0x2f25b0 — __ZN3RBX10RunDraggerD2Ev
pub fn stub_0x2f25b0() -> ! {
    todo!("0x2f25b0 __ZN3RBX10RunDraggerD2Ev")
}

#[doc(alias = "RBX::RunDragger::snapInfoFromSnapPart(void)")]
// 0x2f26a8 — __ZN3RBX10RunDragger20snapInfoFromSnapPartEv
pub fn stub_0x2f26a8() -> ! {
    todo!("0x2f26a8 RBX::RunDragger::snapInfoFromSnapPart(void)")
}

#[doc(alias = "RBX::RunDragger::snapPartFromSnapInfo(void)")]
// 0x2f2a54 — __ZN3RBX10RunDragger20snapPartFromSnapInfoEv
pub fn stub_0x2f2a54() -> ! {
    todo!("0x2f2a54 RBX::RunDragger::snapPartFromSnapInfo(void)")
}

#[doc(alias = "RBX::RunDragger::moveDragPart(void)")]
// 0x2f37c8 — __ZN3RBX10RunDragger12moveDragPartEv
pub fn stub_0x2f37c8() -> ! {
    todo!("0x2f37c8 RBX::RunDragger::moveDragPart(void)")
}

#[doc(alias = "RBX::RunDragger::getSnapSurfaceCoord(void)")]
// 0x2f41c8 — __ZN3RBX10RunDragger19getSnapSurfaceCoordEv
pub fn stub_0x2f41c8() -> ! {
    todo!("0x2f41c8 RBX::RunDragger::getSnapSurfaceCoord(void)")
}

#[doc(alias = "RBX::RunDragger::snapDragPart(void)")]
// 0x2f4340 — __ZN3RBX10RunDragger12snapDragPartEv
pub fn stub_0x2f4340() -> ! {
    todo!("0x2f4340 RBX::RunDragger::snapDragPart(void)")
}

#[doc(alias = "RBX::RunDragger::adjacent(RBX::Primitive *,RBX::Primitive *)")]
// 0x2f46c0 — __ZN3RBX10RunDragger8adjacentEPNS_9PrimitiveES2_
pub fn stub_0x2f46c0() -> ! {
    todo!("0x2f46c0 RBX::RunDragger::adjacent(RBX::Primitive *,RBX::Primitive *)")
}

#[doc(alias = "RBX::RunDragger::fallOffEdge(void)")]
// 0x2f4ae0 — __ZN3RBX10RunDragger11fallOffEdgeEv
pub fn stub_0x2f4ae0() -> ! {
    todo!("0x2f4ae0 RBX::RunDragger::fallOffEdge(void)")
}

#[doc(alias = "RBX::RunDragger::fallOffPart(bool &)")]
// 0x2f4b14 — __ZN3RBX10RunDragger11fallOffPartERb
pub fn stub_0x2f4b14() -> ! {
    todo!("0x2f4b14 RBX::RunDragger::fallOffPart(bool &)")
}

#[doc(alias = "RBX::RunDragger::rayHitsCloserPart(void)")]
// 0x2f4c88 — __ZN3RBX10RunDragger17rayHitsCloserPartEv
pub fn stub_0x2f4c88() -> ! {
    todo!("0x2f4c88 RBX::RunDragger::rayHitsCloserPart(void)")
}

#[doc(alias = "RBX::RunDragger::tooCloseToCamera(void)")]
// 0x2f4dd8 — __ZN3RBX10RunDragger16tooCloseToCameraEv
pub fn stub_0x2f4dd8() -> ! {
    todo!("0x2f4dd8 RBX::RunDragger::tooCloseToCamera(void)")
}

#[doc(alias = "RBX::RunDragger::findSafeY(void)")]
// 0x2f5168 — __ZN3RBX10RunDragger9findSafeYEv
pub fn stub_0x2f5168() -> ! {
    todo!("0x2f5168 RBX::RunDragger::findSafeY(void)")
}

#[doc(alias = "RBX::RunDragger::snap(RBX::RbxRay const&)")]
// 0x2f5610 — __ZN3RBX10RunDragger4snapERKNS_6RbxRayE
pub fn stub_0x2f5610() -> ! {
    todo!("0x2f5610 RBX::RunDragger::snap(RBX::RbxRay const&)")
}

#[doc(alias = "__GLOBAL__I_a_102")]
// 0x2f5d3c — __GLOBAL__I_a_102
pub fn stub_0x2f5d3c() -> ! {
    todo!("0x2f5d3c __GLOBAL__I_a_102")
}

#[doc(alias = "RBX::ArrowToolBase::onMouseHover(RBX::UIEvent const&)")]
// 0x2f614c — __ZN3RBX13ArrowToolBase12onMouseHoverERKNS_7UIEventE
pub fn stub_0x2f614c() -> ! {
    todo!("0x2f614c RBX::ArrowToolBase::onMouseHover(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::ArrowToolBase::onMouseIdle(RBX::UIEvent const&)")]
// 0x2f6154 — __ZN3RBX13ArrowToolBase11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2f6154() -> ! {
    todo!("0x2f6154 RBX::ArrowToolBase::onMouseIdle(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::ArrowToolBase::getCursorName(void)const")]
// 0x2f6190 — __ZNK3RBX13ArrowToolBase13getCursorNameEv
pub fn stub_0x2f6190() -> ! {
    todo!("0x2f6190 RBX::ArrowToolBase::getCursorName(void)const")
}

#[doc(alias = "RBX::ArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
// 0x2f6254 — __ZN3RBX13ArrowToolBase11onMouseDownERKNS_7UIEventE
pub fn stub_0x2f6254() -> ! {
    todo!("0x2f6254 RBX::ArrowToolBase::onMouseDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::ArrowToolBase::onPeekKeyDown(RBX::UIEvent const&)")]
// 0x2f6610 — __ZN3RBX13ArrowToolBase13onPeekKeyDownERKNS_7UIEventE
pub fn stub_0x2f6610() -> ! {
    todo!("0x2f6610 RBX::ArrowToolBase::onPeekKeyDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::AdvArrowToolBase::getCursorName(void)const")]
// 0x2f68d0 — __ZNK3RBX16AdvArrowToolBase13getCursorNameEv
pub fn stub_0x2f68d0() -> ! {
    todo!("0x2f68d0 RBX::AdvArrowToolBase::getCursorName(void)const")
}

#[doc(alias = "RBX::AdvArrowToolBase::onKeyDown(RBX::UIEvent const&)")]
// 0x2f6900 — __ZN3RBX16AdvArrowToolBase9onKeyDownERKNS_7UIEventE
pub fn stub_0x2f6900() -> ! {
    todo!("0x2f6900 RBX::AdvArrowToolBase::onKeyDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::AdvArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
// 0x2f6954 — __ZN3RBX16AdvArrowToolBase11onMouseDownERKNS_7UIEventE
pub fn stub_0x2f6954() -> ! {
    todo!("0x2f6954 RBX::AdvArrowToolBase::onMouseDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::AdvArrowToolBase::onMouseMove(RBX::UIEvent const&)")]
// 0x2f6d04 — __ZN3RBX16AdvArrowToolBase11onMouseMoveERKNS_7UIEventE
pub fn stub_0x2f6d04() -> ! {
    todo!("0x2f6d04 RBX::AdvArrowToolBase::onMouseMove(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::AdvArrowToolBase::determineManualJointConditions(void)")]
// 0x2f6d18 — __ZN3RBX16AdvArrowToolBase30determineManualJointConditionsEv
pub fn stub_0x2f6d18() -> ! {
    todo!("0x2f6d18 RBX::AdvArrowToolBase::determineManualJointConditions(void)")
}

#[doc(alias = "RBX::AdvArrowToolBase::onMouseUp(RBX::UIEvent const&)")]
// 0x2f6fb8 — __ZN3RBX16AdvArrowToolBase9onMouseUpERKNS_7UIEventE
pub fn stub_0x2f6fb8() -> ! {
    todo!("0x2f6fb8 RBX::AdvArrowToolBase::onMouseUp(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
// 0x2f7134 — __ZN3RBX16BoxSelectCommandD0Ev
pub fn stub_0x2f7134() -> ! {
    todo!("0x2f7134 RBX::BoxSelectCommand::~BoxSelectCommand()")
}

#[doc(alias = "__ZN3RBX16BoxSelectCommandD1Ev")]
// 0x2f71d4 — __ZN3RBX16BoxSelectCommandD1Ev
pub fn stub_0x2f71d4() -> ! {
    todo!("0x2f71d4 __ZN3RBX16BoxSelectCommandD1Ev")
}

#[doc(alias = "__ZThn36_N3RBX16BoxSelectCommandD0Ev")]
// 0x2f71d8 — __ZThn36_N3RBX16BoxSelectCommandD0Ev
pub fn stub_0x2f71d8() -> ! {
    todo!("0x2f71d8 __ZThn36_N3RBX16BoxSelectCommandD0Ev")
}

#[doc(alias = "__ZN3RBX16BoxSelectCommandD2Ev")]
// 0x2f71e0 — __ZN3RBX16BoxSelectCommandD2Ev
pub fn stub_0x2f71e0() -> ! {
    todo!("0x2f71e0 __ZN3RBX16BoxSelectCommandD2Ev")
}

#[doc(alias = "__ZThn36_N3RBX16BoxSelectCommandD1Ev")]
// 0x2f7324 — __ZThn36_N3RBX16BoxSelectCommandD1Ev
pub fn stub_0x2f7324() -> ! {
    todo!("0x2f7324 __ZThn36_N3RBX16BoxSelectCommandD1Ev")
}

#[doc(alias = "RBX::BoxSelectCommand::onMouseDown(RBX::UIEvent const&)")]
// 0x2f73fc — __ZN3RBX16BoxSelectCommand11onMouseDownERKNS_7UIEventE
pub fn stub_0x2f73fc() -> ! {
    todo!("0x2f73fc RBX::BoxSelectCommand::onMouseDown(RBX::UIEvent const&)")
}

#[doc(alias = "RBX::BoxSelectCommand::onMouseMove(RBX::UIEvent const&)")]
// 0x2f7468 — __ZN3RBX16BoxSelectCommand11onMouseMoveERKNS_7UIEventE
pub fn stub_0x2f7468() -> ! {
    todo!("0x2f7468 RBX::BoxSelectCommand::onMouseMove(RBX::UIEvent const&)")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv")]
// 0x2f7ec0 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv
pub fn stub_0x2f7ec0() -> ! {
    todo!("0x2f7ec0 __ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv")]
// 0x2f7ee8 — __ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv
pub fn stub_0x2f7ee8() -> ! {
    todo!("0x2f7ee8 __ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")]
// 0x2f7eec — __ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v
pub fn stub_0x2f7eec() -> ! {
    todo!("0x2f7eec __ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")
}

#[doc(alias = "__GLOBAL__I_a_103")]
// 0x2f8c2c — __GLOBAL__I_a_103
pub fn stub_0x2f8c2c() -> ! {
    todo!("0x2f8c2c __GLOBAL__I_a_103")
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")]
// 0x2f946c — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE6resizeEmS2_
pub fn stub_0x2f946c() -> ! {
    todo!("0x2f946c std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")]
// 0x2f94a0 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE9push_backERKS2_
pub fn stub_0x2f94a0() -> ! {
    todo!("0x2f94a0 std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")]
// 0x2f94c8 — __ZNSt3mapIPKN3RBX4NameENS0_6Action10ActionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0x2f94c8() -> ! {
    todo!("0x2f94c8 std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
// 0x2f9520 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0x2f9520() -> ! {
    todo!("0x2f9520 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
// 0x2f95d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0x2f95d4() -> ! {
    todo!("0x2f95d4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
// 0x2f962c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x2f962c() -> ! {
    todo!("0x2f962c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")]
// 0x2f9694 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x2f9694() -> ! {
    todo!("0x2f9694 std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")
}

#[doc(alias = "std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")]
// 0x2f9778 — __ZNSt12_Vector_baseIN3RBX6Action10ActionTypeESaIS2_EE11_M_allocateEm
pub fn stub_0x2f9778() -> ! {
    todo!("0x2f9778 std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")
}

#[doc(alias = "RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")]
// 0x2f9790 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Action10ActionTypeES6_EET0_T_S8_S7_
pub fn stub_0x2f9790() -> ! {
    todo!("0x2f9790 RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&)")]
// 0x2f97cc — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0x2f97cc() -> ! {
    todo!("0x2f97cc std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&)")
}

#[doc(alias = "__GLOBAL__I_a_104")]
// 0x2f995c — __GLOBAL__I_a_104
pub fn stub_0x2f995c() -> ! {
    todo!("0x2f995c __GLOBAL__I_a_104")
}

#[doc(alias = "RBX::StringConverter<RBX::AnimationId>::convertToValue(std::string const&,RBX::AnimationId&)")]
// 0x2f9a24 — __ZN3RBX15StringConverterINS_11AnimationIdEE14convertToValueERKSsRS1_
pub fn stub_0x2f9a24() -> ! {
    todo!("0x2f9a24 RBX::StringConverter<RBX::AnimationId>::convertToValue(std::string const&,RBX::AnimationId&)")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&)")]
// 0x2fa39c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11AnimationIdEEERS3_RKT_
pub fn stub_0x2fa39c() -> ! {
    todo!("0x2fa39c rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&)")
}

#[doc(alias = "RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x2fa6a8 — __ZN3rbx8any_castIN3RBX11AnimationIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x2fa6a8() -> ! {
    todo!("0x2fa6a8 RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

#[doc(alias = "RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x2fa700 — __ZN3rbx8any_castIRN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x2fa700() -> ! {
    todo!("0x2fa700 RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void)")]
// 0x2fa7f0 — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE9singletonEv
pub fn stub_0x2fa7f0() -> ! {
    todo!("0x2fa7f0 rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::construct_func(char const*,char *)")]
// 0x2fa85c — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE14construct_funcEPKcPc
pub fn stub_0x2fa85c() -> ! {
    todo!("0x2fa85c rbx::implementation::typed_holder<RBX::AnimationId>::construct_func(char const*,char *)")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::destruct_func(char *)")]
// 0x2fa878 — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE13destruct_funcEPc
pub fn stub_0x2fa878() -> ! {
    todo!("0x2fa878 rbx::implementation::typed_holder<RBX::AnimationId>::destruct_func(char *)")
}

#[doc(alias = "__GLOBAL__I_a_105")]
// 0x2fa87c — __GLOBAL__I_a_105
pub fn stub_0x2fa87c() -> ! {
    todo!("0x2fa87c __GLOBAL__I_a_105")
}
