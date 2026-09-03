//! rendering — next 150 stubs EA-sorted Ogre|G3D|Gfx|Render|Adorn
//! Filter: Ogre|G3D|Gfx|Render|Adorn (15586 total, 1684 prior stubbed, 150 this batch, 13752 remaining)
//! This shard: 0x3ec30..0x30e130
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x3ec30 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED1Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
// was: boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()
// IDA 0x3ec30: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ec30() {
}

// 0x3ec34 — __ZN5boost10scoped_ptrIN4Ogre10LogManagerEED2Ev
#[doc(alias = "boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()")]
// was: boost::scoped_ptr<Ogre::LogManager>::~scoped_ptr()
// IDA 0x3ec34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ec34() {
}

// 0x3ecd0 — __ZN4Ogre19WindowEventListener11windowMovedEPNS_12RenderWindowE
#[doc(alias = "Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowMoved(Ogre::RenderWindow *)
// IDA 0x3ecd0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3ecd0() {
}

// 0x3ecd4 — __ZN4Ogre19WindowEventListener13windowResizedEPNS_12RenderWindowE
#[doc(alias = "Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowResized(Ogre::RenderWindow *)
// IDA 0x3ecd4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3ecd4() {
}

// 0x3ecd8 — __ZN4Ogre19WindowEventListener13windowClosingEPNS_12RenderWindowE
#[doc(alias = "Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowClosing(Ogre::RenderWindow *)
// IDA 0x3ecd8: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_3ecd8() {
}

// 0x3ecdc — __ZN17QuitEventListener12windowClosedEPN4Ogre12RenderWindowE
#[doc(alias = "QuitEventListener::windowClosed(Ogre::RenderWindow *)")]
// was: QuitEventListener::windowClosed(Ogre::RenderWindow *)
// IDA 0x3ecdc: BX LR default listener — empty virtual in C++, no-op here.
pub fn stub_3ecdc() {
}

// 0x3ecec — __ZN4Ogre19WindowEventListener17windowFocusChangeEPNS_12RenderWindowE
#[doc(alias = "Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)")]
// was: Ogre::WindowEventListener::windowFocusChange(Ogre::RenderWindow *)
// IDA 0x3ecec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3ecec() {
}

// 0x51f40 — -[MainViewController getOgreWindow]
#[doc(alias = "-[MainViewController getOgreWindow]")]
// was: -[MainViewController getOgreWindow]
// IDA 0x51f40: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_51f40() {
}

// 0x51f50 — -[MainViewController setOgreWindow:]
#[doc(alias = "-[MainViewController setOgreWindow:]")]
// was: -[MainViewController setOgreWindow:]
// IDA 0x51f50: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_51f50() {
}

// 0x51f60 — -[MainViewController getOgreView]
#[doc(alias = "-[MainViewController getOgreView]")]
// was: -[MainViewController getOgreView]
// IDA 0x51f60: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_51f60() {
}

// 0x51f70 — -[MainViewController setOgreView:]
#[doc(alias = "-[MainViewController setOgreView:]")]
// was: -[MainViewController setOgreView:]
// IDA 0x51f70: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_51f70() {
}

// 0x51fa0 — -[MainViewController getOgreViewController]
#[doc(alias = "-[MainViewController getOgreViewController]")]
// was: -[MainViewController getOgreViewController]
// IDA 0x51fa0: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_51fa0() {
}

// 0x51fb0 — -[MainViewController setOgreViewController:]
#[doc(alias = "-[MainViewController setOgreViewController:]")]
// was: -[MainViewController setOgreViewController:]
// IDA 0x51fb0: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_51fb0() {
}

// 0x2ce2c4 — __ZN3RBX11AdvDragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE
#[doc(alias = "RBX::AdvDragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::AdvDragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2ce2c4: 209 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce2c4() {
}

// 0x2ce4e8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14AdvLuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvLuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::shared_ptr<RBX::AdvLuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvLuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2ce4e8: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce4e8() {
}

// 0x2cf178 — __ZN3RBX13AdvLuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
#[doc(alias = "RBX::AdvLuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>)")]
// was: RBX::AdvLuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)
// IDA 0x2cf178: 190 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cf178() {
}

// 0x2d0030 — __ZN3RBX13AdvLuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
#[doc(alias = "RBX::AdvLuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
// was: RBX::AdvLuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)
// IDA 0x2d0030: 108 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0030() {
}

// 0x2d03b0 — __ZN3RBX13AdvLuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E
#[doc(alias = "RBX::AdvLuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
// was: RBX::AdvLuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)
// IDA 0x2d03b0: 183 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d03b0() {
}

// 0x2d17c4 — __ZN3RBX14AdvLuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2d17c4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d17c4() {
}

// 0x2d17c8 — __ZN3RBX14AdvLuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2d17c8: 228 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d17c8() {
}

// 0x2d3d4c — __ZNK3RBX15AdvMoveToolBase21getExtentsAndLocationERNS_7ExtentsERN3G3D15CoordinateFrameERb
#[doc(alias = "RBX::AdvMoveToolBase::getExtentsAndLocation(RBX::Extents &,G3D::CoordinateFrame &,bool &)const")]
// was: RBX::AdvMoveToolBase::getExtentsAndLocation(RBX::Extents &,G3D::CoordinateFrame &,bool &)const
// IDA 0x2d3d4c: 396 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d3d4c() {
}

// 0x2d487c — __ZNK3RBX15AdvMoveToolBase13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE
#[doc(alias = "RBX::AdvMoveToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
// was: RBX::AdvMoveToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const
// IDA 0x2d487c: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d487c() {
}

// 0x2d4d38 — __ZN3RBX11AdvMoveTool20getGridXYUsingCameraEPNS_12PartInstanceERN3G3D7Vector3ES5_
#[doc(alias = "RBX::AdvMoveTool::getGridXYUsingCamera(RBX::PartInstance *,G3D::Vector3 &,G3D::Vector3 &)")]
// was: RBX::AdvMoveTool::getGridXYUsingCamera(RBX::PartInstance *,G3D::Vector3 &,G3D::Vector3 &)
// IDA 0x2d4d38: 242 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d4d38() {
}

// 0x2d5da0 — __ZNK3RBX13AdvRotateTool13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE
#[doc(alias = "RBX::AdvRotateTool::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
// was: RBX::AdvRotateTool::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const
// IDA 0x2d5da0: 193 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d5da0() {
}

// 0x2d7074 — __ZN3RBX13AdvRunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorIS6_SaIS6_EE
#[doc(alias = "RBX::AdvRunDragger::initLocal(RBX::Workspace *,rbx_core::WeakPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>)")]
// was: RBX::AdvRunDragger::initLocal(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)
// IDA 0x2d7074: 516 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d7074() {
}

// 0x2d7610 — __ZN3RBX13AdvRunDragger17createSnapSurfaceEPNS_9PrimitiveEPN3G3D5ArrayImLi10ELm32EEE
#[doc(alias = "RBX::AdvRunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")]
// was: RBX::AdvRunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)
// IDA 0x2d7610: 319 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d7610() {
}

// 0x2d89e8 — __ZN3RBX13AdvRunDragger8notTriedEPNS_9PrimitiveERKN3G3D5ArrayIS2_Li10ELm32EEE
#[doc(alias = "RBX::AdvRunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::AdvRunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x2d89e8: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d89e8() {
}

// 0x2d8ab8 — __ZN3RBX13AdvRunDragger11rayHitsPartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEb
#[doc(alias = "RBX::AdvRunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)")]
// was: RBX::AdvRunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)
// IDA 0x2d8ab8: 204 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d8ab8() {
}

// 0x2d8ce4 — __ZN3RBX13AdvRunDragger17bestProximatePartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEMNS_7ContactEFbfE
#[doc(alias = "RBX::AdvRunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))")]
// was: RBX::AdvRunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))
// IDA 0x2d8ce4: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d8ce4() {
}

// 0x2d91b0 — __ZN3RBX13AdvRunDragger8findSnapERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::AdvRunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::AdvRunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x2d91b0: 103 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d91b0() {
}

// 0x2d92dc — __ZN3RBX13AdvRunDragger18findNoSnapPositionERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::AdvRunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")]
// was: RBX::AdvRunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)
// IDA 0x2d92dc: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d92dc() {
}

// 0x2d98c8 — __ZN3RBX13AdvRunDragger32rotatePart90DegAboutSnapFaceAxisEN3G3D7Vector34AxisE
#[doc(alias = "RBX::AdvRunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)")]
// was: RBX::AdvRunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)
// IDA 0x2d98c8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d98c8() {
}

// 0x2da5d0 — __ZNK3RBX12AxisToolBase13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE
#[doc(alias = "RBX::AxisToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
// was: RBX::AxisToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const
// IDA 0x2da5d0: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2da5d0() {
}

// 0x2dbe5c — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12PartDragToolEPNS_12PartInstanceEN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEEEENSC_IT_EET0_T1_T2_T3_
#[doc(alias = "rbx_core::SharedPtr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::shared_ptr<RBX::PartDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::PartDragTool,RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2dbe5c: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dbe5c() {
}

// 0x2dc790 — __ZN3RBX7Dragger14computeExtentsERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::Dragger::computeExtents(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::Dragger::computeExtents(G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x2dc790: 114 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dc790() {
}

// 0x2dca04 — __ZN3RBX7Dragger25intersectingWorldOrOthersERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS_14ContactManagerEff
#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers(G3D::Array<RBX::Primitive *,10,32ul> const&,RBX::ContactManager &,float,float)")]
// was: RBX::Dragger::intersectingWorldOrOthers(G3D::Array<RBX::Primitive *,10,32ul> const&,RBX::ContactManager &,float,float)
// IDA 0x2dca04: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dca04() {
}

// 0x2dca90 — __ZN3RBX7Dragger23intersectingGroundPlaneERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEf
#[doc(alias = "RBX::Dragger::intersectingGroundPlane(G3D::Array<RBX::Primitive *,10,32ul> const&,float)")]
// was: RBX::Dragger::intersectingGroundPlane(G3D::Array<RBX::Primitive *,10,32ul> const&,float)
// IDA 0x2dca90: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dca90() {
}

// 0x2dcb04 — __ZN3RBX7Dragger18movePrimitivesGoalERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERS8_
#[doc(alias = "RBX::Dragger::movePrimitivesGoal(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")]
// was: RBX::Dragger::movePrimitivesGoal(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)
// IDA 0x2dcb04: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dcb04() {
}

// 0x2dcba4 — __ZN3RBX7Dragger14movePrimitivesERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3E
#[doc(alias = "RBX::Dragger::movePrimitives(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&)")]
// was: RBX::Dragger::movePrimitives(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&)
// IDA 0x2dcba4: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dcba4() {
}

// 0x2dcc5c — __ZN3RBX7Dragger19movePrimitivesDeltaERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERS8_
#[doc(alias = "RBX::Dragger::movePrimitivesDelta(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)")]
// was: RBX::Dragger::movePrimitivesDelta(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3&)
// IDA 0x2dcc5c: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dcc5c() {
}

// 0x2dcd50 — __ZN3RBX7Dragger12searchUpFineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::Dragger::searchUpFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
// was: RBX::Dragger::searchUpFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)
// IDA 0x2dcd50: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dcd50() {
}

// 0x2dce48 — __ZN3RBX7Dragger14searchDownFineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::Dragger::searchDownFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
// was: RBX::Dragger::searchDownFine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)
// IDA 0x2dce48: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dce48() {
}

// 0x2dcf50 — __ZN3RBX7Dragger13searchUpGrossERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::Dragger::searchUpGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
// was: RBX::Dragger::searchUpGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)
// IDA 0x2dcf50: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dcf50() {
}

// 0x2dd074 — __ZN3RBX7Dragger15searchDownGrossERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS1_7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::Dragger::searchDownGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)")]
// was: RBX::Dragger::searchDownGross(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 &,RBX::ContactManager &,float)
// IDA 0x2dd074: 114 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dd074() {
}

// 0x2dd1d4 — __ZN3RBX7Dragger18safePlaceAlongLineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ESA_RS8_RNS_14ContactManagerE
#[doc(alias = "RBX::Dragger::safePlaceAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3&,RBX::ContactManager &)")]
// was: RBX::Dragger::safePlaceAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3&,RBX::ContactManager &)
// IDA 0x2dd1d4: 286 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dd1d4() {
}

// 0x2dd588 — __ZN3RBX7Dragger17safeMoveAlongLineERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::Dragger::safeMoveAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
// was: RBX::Dragger::safeMoveAlongLine(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)
// IDA 0x2dd588: 196 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dd588() {
}

// 0x2dd814 — __ZN3RBX7Dragger13safeMoveYDropERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::Dragger::safeMoveYDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
// was: RBX::Dragger::safeMoveYDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)
// IDA 0x2dd814: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dd814() {
}

// 0x2dd924 — __ZN3RBX7Dragger17safeMoveYDrop_EXTERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::Dragger::safeMoveYDrop_EXT(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
// was: RBX::Dragger::safeMoveYDrop_EXT(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &,float)
// IDA 0x2dd924: 381 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dd924() {
}

// 0x2ddd90 — __ZN3RBX7Dragger14safeMoveNoDropERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Vector3ERNS_14ContactManagerE
#[doc(alias = "RBX::Dragger::safeMoveNoDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &)")]
// was: RBX::Dragger::safeMoveNoDrop(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Vector3 const&,RBX::ContactManager &)
// IDA 0x2ddd90: 95 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ddd90() {
}

// 0x2ddec0 — __ZN3RBX7Dragger10safeRotateERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Matrix3ERNS_14ContactManagerE
#[doc(alias = "RBX::Dragger::safeRotate(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")]
// was: RBX::Dragger::safeRotate(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)
// IDA 0x2ddec0: 227 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ddec0() {
}

// 0x2de150 — __ZN3RBX7Dragger11safeRotate2ERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS1_7Matrix3ERNS_14ContactManagerE
#[doc(alias = "RBX::Dragger::safeRotate2(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)")]
// was: RBX::Dragger::safeRotate2(G3D::Array<RBX::Primitive *,10,32ul> const&,G3D::Matrix3 const&,RBX::ContactManager &)
// IDA 0x2de150: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2de150() {
}

// 0x2de1d0 — __ZN3RBX7Dragger29intersectingWorldOrOthers_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRKNS6_7Vector3E
#[doc(alias = "RBX::Dragger::intersectingWorldOrOthers_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 const&)")]
// was: RBX::Dragger::intersectingWorldOrOthers_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 const&)
// IDA 0x2de1d0: 327 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2de1d0() {
}

// 0x2de578 — __ZN3RBX7Dragger17searchUpGross_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
#[doc(alias = "RBX::Dragger::searchUpGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
// was: RBX::Dragger::searchUpGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)
// IDA 0x2de578: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2de578() {
}

// 0x2de6ac — __ZN3RBX7Dragger19searchDownGross_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
#[doc(alias = "RBX::Dragger::searchDownGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
// was: RBX::Dragger::searchDownGross_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)
// IDA 0x2de6ac: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2de6ac() {
}

// 0x2de7e0 — __ZN3RBX7Dragger18searchDownFine_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
#[doc(alias = "RBX::Dragger::searchDownFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
// was: RBX::Dragger::searchDownFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)
// IDA 0x2de7e0: 101 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2de7e0() {
}

// 0x2de92c — __ZN3RBX7Dragger16searchUpFine_EXTERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKN5boost9unordered13unordered_setIPKS8_NSD_4hashISH_EESt8equal_toISH_ESaISH_EEERNS_14ContactManagerEfRNS6_7Vector3E
#[doc(alias = "RBX::Dragger::searchUpFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)")]
// was: RBX::Dragger::searchUpFine_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Array<RBX::Primitive *,10,32ul> const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,RBX::ContactManager &,float,G3D::Vector3 &)
// IDA 0x2de92c: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2de92c() {
}

// 0x2dea44 — __ZN3RBX7Dragger27intersectingGroundPlane_EXTERKSt6vectorINS_7ExtentsESaIS2_EERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEfRKNS7_7Vector3E
#[doc(alias = "RBX::Dragger::intersectingGroundPlane_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> const&,G3D::Array<RBX::Primitive *,10,32ul> const&,float,G3D::Vector3 const&)")]
// was: RBX::Dragger::intersectingGroundPlane_EXT(std::vector<RBX::Extents,std::allocator<RBX::Extents>> const&,G3D::Array<RBX::Primitive *,10,32ul> const&,float,G3D::Vector3 const&)
// IDA 0x2dea44: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dea44() {
}

// 0x2deb24 — __ZN3RBX7Dragger14isIntersectingEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
#[doc(alias = "RBX::Dragger::isIntersecting(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
// was: RBX::Dragger::isIntersecting(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)
// IDA 0x2deb24: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2deb24() {
}

// 0x2deb94 — __ZN3RBX7Dragger25checkBallBallIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
#[doc(alias = "RBX::Dragger::checkBallBallIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
// was: RBX::Dragger::checkBallBallIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)
// IDA 0x2deb94: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2deb94() {
}

// 0x2decd4 — __ZN3RBX7Dragger25checkBallPolyIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
#[doc(alias = "RBX::Dragger::checkBallPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
// was: RBX::Dragger::checkBallPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)
// IDA 0x2decd4: 447 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2decd4() {
}

// 0x2df2b8 — __ZN3RBX7Dragger25checkPolyPolyIntersectionEPKNS_9PrimitiveERKN3G3D15CoordinateFrameES3_S7_
#[doc(alias = "RBX::Dragger::checkPolyPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)")]
// was: RBX::Dragger::checkPolyPolyIntersection(RBX::Primitive const*,G3D::CoordinateFrame const&,RBX::Primitive const*,G3D::CoordinateFrame const&)
// IDA 0x2df2b8: 650 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2df2b8() {
}

// 0x2dfc24 — __ZN3RBX7Dragger11moveExtentsERSt6vectorINS_7ExtentsESaIS2_EERKN3G3D7Vector3E
#[doc(alias = "RBX::Dragger::moveExtents(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Vector3 const&)")]
// was: RBX::Dragger::moveExtents(std::vector<RBX::Extents,std::allocator<RBX::Extents>> &,G3D::Vector3 const&)
// IDA 0x2dfc24: 32 insns (LDM.W..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dfc24() {
}

// 0x2dfda8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)")]
// was: G3D::Array<RBX::Primitive *,10,32ul>::append(RBX::Primitive * const&)
// IDA 0x2dfda8: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dfda8() {
}

// 0x2dfed8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE4initEiRKNS_23ReferenceCountedPointerINS_13MemoryManagerEEE
#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)")]
// was: G3D::Array<RBX::Primitive *,10,32ul>::init(int,G3D::ReferenceCountedPointer<G3D::MemoryManager> const&)
// IDA 0x2dfed8: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2dfed8() {
}

// 0x2e02d0 — __ZN3G3D6SphereD0Ev
#[doc(alias = "G3D::Sphere::~Sphere()")]
// was: G3D::Sphere::~Sphere()
// IDA 0x2e02d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e02d0() {
}

// 0x2e06d0 — __ZN3RBX8DragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE
#[doc(alias = "RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::DragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2e06d0: 186 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e06d0() {
}

// 0x2e08bc — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11LuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::shared_ptr<RBX::LuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2e08bc: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e08bc() {
}

// 0x2e0f38 — __ZN3RBX13DragUtilities13safeMoveYDropERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKN3G3D7Vector3ERNS_14ContactManagerEf
#[doc(alias = "RBX::DragUtilities::safeMoveYDrop(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)")]
// was: RBX::DragUtilities::safeMoveYDrop(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Vector3 const&,RBX::ContactManager &,float)
// IDA 0x2e0f38: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e0f38() {
}

// 0x2e10d8 — __ZN3RBX13DragUtilities17partsToPrimitivesERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::DragUtilities::partsToPrimitives(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)")]
// was: RBX::DragUtilities::partsToPrimitives(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::Array<RBX::Primitive *,10,32ul> &)
// IDA 0x2e10d8: 209 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e10d8() {
}

// 0x2e13f0 — __ZN3RBX13DragUtilities16hitObjectOrPlaneERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb
#[doc(alias = "RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
// was: RBX::DragUtilities::hitObjectOrPlane(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)
// IDA 0x2e13f0: 193 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e13f0() {
}

// 0x2e1628 — __ZN3RBX13DragUtilities9hitObjectERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EERKNS_6RbxRayERKNS_14ContactManagerERN3G3D7Vector3Eb
#[doc(alias = "RBX::DragUtilities::hitObject(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)")]
// was: RBX::DragUtilities::hitObject(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::RbxRay const&,RBX::ContactManager const&,G3D::Vector3 &,bool)
// IDA 0x2e1628: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1628() {
}

// 0x2e1708 — __ZN3RBX13DragUtilities9hitObjectERKNS_14ContactManagerERKNS_6RbxRayEPKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERNS7_7Vector3Eb
#[doc(alias = "RBX::DragUtilities::hitObject(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)")]
// was: RBX::DragUtilities::hitObject(RBX::ContactManager const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive *,10,32ul> const*,G3D::Vector3 &,bool)
// IDA 0x2e1708: 109 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e1708() {
}

// 0x2e2300 — __ZN3RBX13DragUtilities12moveAndCleanEPNS_12PartInstanceERKN3G3D7Vector3E
#[doc(alias = "RBX::DragUtilities::moveAndClean(RBX::PartInstance *,G3D::Vector3 const&)")]
// was: RBX::DragUtilities::moveAndClean(RBX::PartInstance *,G3D::Vector3 const&)
// IDA 0x2e2300: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e2300() {
}

// 0x2e24f0 — __ZN3RBX13DragUtilities4moveERKSt6vectorIN5boost8weak_ptrINS_12PartInstanceEEESaIS5_EEN3G3D15CoordinateFrameESB_
#[doc(alias = "RBX::DragUtilities::move(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)")]
// was: RBX::DragUtilities::move(std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,G3D::CoordinateFrame,G3D::CoordinateFrame)
// IDA 0x2e24f0: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e24f0() {
}

// 0x2e26d4 — __ZN3RBX13DragUtilities6toGridERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::DragUtilities::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: RBX::DragUtilities::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0x2e26d4: 93 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e26d4() {
}

// 0x2e51d0 — __ZN3RBX10LuaDragger15mouseDownPublicEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS2_IKSt6vectorIS4_SaIS4_EEEE
#[doc(alias = "RBX::LuaDragger::mouseDownPublic(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
// was: RBX::LuaDragger::mouseDownPublic(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
// IDA 0x2e51d0: 446 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e51d0() {
}

// 0x2e5b88 — __ZN3RBX10LuaDragger10axisRotateEN3G3D7Vector34AxisE
#[doc(alias = "RBX::LuaDragger::axisRotate(G3D::Vector3::Axis)")]
// was: RBX::LuaDragger::axisRotate(G3D::Vector3::Axis)
// IDA 0x2e5b88: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e5b88() {
}

// 0x2e6070 — __ZN3RBX10LuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
#[doc(alias = "RBX::LuaDragger::mouseDown(rbx_core::SharedPtr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>)")]
// was: RBX::LuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)
// IDA 0x2e6070: 224 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e6070() {
}

// 0x2e67a4 — __ZN3RBX10LuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
#[doc(alias = "RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
// was: RBX::LuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)
// IDA 0x2e67a4: 108 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e67a4() {
}

// 0x2e6b88 — __ZN3RBX10LuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E
#[doc(alias = "RBX::LuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
// was: RBX::LuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)
// IDA 0x2e6b88: 183 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e6b88() {
}

// 0x2e700c — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()
// IDA 0x2e700c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e700c() {
}

// 0x2e712c — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()
// IDA 0x2e712c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e712c() {
}

// 0x2e8528 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EEC2EMS2_FvS5_EPKcSB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::BoundFuncDesc(void (RBX::LuaDragger::*)(G3D::Vector3::Axis),char const*,char const*,G3D::Vector3::Axis,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x2e8528: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e8528() {
}

// 0x2e86d4 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::declareSignature(char const*,RBX::Reflection::Variant)
// IDA 0x2e86d4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e86d4() {
}

// 0x2e8704 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::~BoundFuncDesc()
// IDA 0x2e8704: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e8704() {
}

// 0x2e87d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN3G3D7Vector34AxisEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(G3D::Vector3::Axis),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0x2e87d8: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e87d8() {
}

// 0x2e880c — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector34AxisELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: G3D::Vector3::Axis RBX::Reflection::ArgHelper::getArg<G3D::Vector3::Axis,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3::Axis> const&,boost::disable_if<boost::is_same<G3D::Vector3::Axis,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// IDA 0x2e880c: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e880c() {
}

// 0x2e899c — __ZN3RBX10Reflection9ArgHelper8try_enumILi1EN3G3D7Vector34AxisEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSB_7is_enumIS9_EEvE4typeE
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)")]
// was: bool RBX::Reflection::ArgHelper::try_enum<1,G3D::Vector3::Axis>(RBX::Reflection::FunctionDescriptor::Arguments &,G3D::Vector3::Axis &,boost::enable_if<boost::is_enum<G3D::Vector3::Axis>,void>::type *)
// IDA 0x2e899c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e899c() {
}

// 0x2e8ee8 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EEC2EMS2_FvS6_S8_SD_EPKcSJ_SJ_SJ_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::BoundFuncDesc(void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x2e8ee8: 240 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e8ee8() {
}

// 0x2e9144 — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE16declareSignatureEPKcNS0_7VariantESH_SI_SH_SI_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// IDA 0x2e9144: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e9144() {
}

// 0x2e91ac — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()
// IDA 0x2e91ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e91ac() {
}

// 0x2e924c — __ZNK3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0x2e924c: 117 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e924c() {
}

// 0x2e9388 — __ZN3RBX10Reflection11Call3HelperINS_10LuaDraggerEMS2_FvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEES6_S8_SD_vE4callEPS2_SF_RNS0_7VariantERKS6_RKS8_RKSD_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,G3D::Vector3 const&,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&)")]
// was: RBX::Reflection::Call3Helper<RBX::LuaDragger,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>,void>::call(RBX::LuaDragger*,void (RBX::LuaDragger::*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,G3D::Vector3 const&,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&)
// IDA 0x2e9388: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e9388() {
}

// 0x2e94dc — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector3ELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: G3D::Vector3 RBX::Reflection::ArgHelper::getArg<G3D::Vector3,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector3> const&,boost::disable_if<boost::is_same<G3D::Vector3,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// IDA 0x2e94dc: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e94dc() {
}

// 0x2e998c — __ZN3RBX10Reflection13BoundFuncDescINS_10LuaDraggerEFvN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3ENS4_IKSt6vectorIS6_SaIS6_EEEEELi3EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>),3>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::LuaDragger,void ()(boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>),3>::~BoundFuncDesc()
// IDA 0x2e998c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2e998c() {
}

// 0x2e9f80 — __ZN3RBX11LuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2e9f80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2e9f80() {
}

// 0x2e9f84 — __ZN3RBX11LuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::LuaDragTool::LuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2e9f84: 223 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e9f84() {
}

// 0x2eb604 — __ZN3RBX11MegaDragger13safeMoveYDropERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaDragger::safeMoveYDrop(G3D::Vector3 const&)")]
// was: RBX::MegaDragger::safeMoveYDrop(G3D::Vector3 const&)
// IDA 0x2eb604: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb604() {
}

// 0x2eb680 — __ZN3RBX11MegaDragger15getPartsForDragERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::MegaDragger::getPartsForDrag(G3D::Array<RBX::Primitive *,10,32ul> &)")]
// was: RBX::MegaDragger::getPartsForDrag(G3D::Array<RBX::Primitive *,10,32ul> &)
// IDA 0x2eb680: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb680() {
}

// 0x2eb734 — __ZN3RBX11MegaDragger14safeMoveNoDropERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaDragger::safeMoveNoDrop(G3D::Vector3 const&)")]
// was: RBX::MegaDragger::safeMoveNoDrop(G3D::Vector3 const&)
// IDA 0x2eb734: 115 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb734() {
}

// 0x2eb87c — __ZN3RBX11MegaDragger17safeMoveAlongLineERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaDragger::safeMoveAlongLine(G3D::Vector3 const&)")]
// was: RBX::MegaDragger::safeMoveAlongLine(G3D::Vector3 const&)
// IDA 0x2eb87c: 151 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eb87c() {
}

// 0x2eba30 — __ZN3RBX11MegaDragger22moveSafePlaceAlongLineERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaDragger::moveSafePlaceAlongLine(G3D::Vector3 const&)")]
// was: RBX::MegaDragger::moveSafePlaceAlongLine(G3D::Vector3 const&)
// IDA 0x2eba30: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2eba30() {
}

// 0x2ebc38 — __ZN3RBX11MegaDragger13moveAlongLineERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaDragger::moveAlongLine(G3D::Vector3 const&)")]
// was: RBX::MegaDragger::moveAlongLine(G3D::Vector3 const&)
// IDA 0x2ebc38: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ebc38() {
}

// 0x2ebc44 — __ZN3RBX11MegaDragger19safeRotateAlongLineERKN3G3D7Vector3E
#[doc(alias = "RBX::MegaDragger::safeRotateAlongLine(G3D::Vector3 const&)")]
// was: RBX::MegaDragger::safeRotateAlongLine(G3D::Vector3 const&)
// IDA 0x2ebc44: 109 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ebc44() {
}

// 0x2ebd7c — __ZN3RBX11MegaDragger10safeRotateERKN3G3D7Matrix3E
#[doc(alias = "RBX::MegaDragger::safeRotate(G3D::Matrix3 const&)")]
// was: RBX::MegaDragger::safeRotate(G3D::Matrix3 const&)
// IDA 0x2ebd7c: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ebd7c() {
}

// 0x2ebf24 — __ZN3RBX11MegaDragger15rotateDragPartsERKN3G3D7Matrix3Eb
#[doc(alias = "RBX::MegaDragger::rotateDragParts(G3D::Matrix3 const&,bool)")]
// was: RBX::MegaDragger::rotateDragParts(G3D::Matrix3 const&,bool)
// IDA 0x2ebf24: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ebf24() {
}

// 0x2ef364 — __ZN3RBX11NewNullTool16getIndicatedPartERKNS_7UIEventERKbPPNS_12PartInstanceEPbPN3G3D7Vector3E
#[doc(alias = "RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)")]
// was: RBX::NewNullTool::getIndicatedPart(RBX::UIEvent const&,bool const&,RBX::PartInstance **,bool *,G3D::Vector3 *)
// IDA 0x2ef364: 109 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ef364() {
}

// 0x2f0948 — __ZN3RBX12PartDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2f0948: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2f0948() {
}

// 0x2f094c — __ZN3RBX12PartDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3EPNS_9WorkspaceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartDragTool::PartDragTool(RBX::PartInstance *,G3D::Vector3 const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)
// IDA 0x2f094c: 233 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f094c() {
}

// 0x2f2bf0 — __ZN3RBX10RunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
#[doc(alias = "RBX::RunDragger::initLocal(RBX::Workspace *,rbx_core::WeakPtr<RBX::PartInstance>,G3D::Vector3 const&)")]
// was: RBX::RunDragger::initLocal(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)
// IDA 0x2f2bf0: 298 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f2bf0() {
}

// 0x2f2ff8 — __ZN3RBX10RunDragger4initEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3E
#[doc(alias = "RBX::RunDragger::init(RBX::Workspace *,rbx_core::WeakPtr<RBX::PartInstance>,G3D::Vector3 const&)")]
// was: RBX::RunDragger::init(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&)
// IDA 0x2f2ff8: 334 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f2ff8() {
}

// 0x2f33e0 — __ZN3RBX10RunDragger17createSnapSurfaceEPNS_9PrimitiveEPN3G3D5ArrayImLi10ELm32EEE
#[doc(alias = "RBX::RunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")]
// was: RBX::RunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)
// IDA 0x2f33e0: 333 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f33e0() {
}

// 0x2f4630 — __ZN3RBX10RunDragger8notTriedEPNS_9PrimitiveERKN3G3D5ArrayIS2_Li10ELm32EEE
#[doc(alias = "RBX::RunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::RunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x2f4630: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4630() {
}

// 0x2f4700 — __ZN3RBX10RunDragger11rayHitsPartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEb
#[doc(alias = "RBX::RunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)")]
// was: RBX::RunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)
// IDA 0x2f4700: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4700() {
}

// 0x2f495c — __ZN3RBX10RunDragger17bestProximatePartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEMNS_7ContactEFbfE
#[doc(alias = "RBX::RunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))")]
// was: RBX::RunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))
// IDA 0x2f495c: 146 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f495c() {
}

// 0x2f4eac — __ZN3RBX10RunDragger8findSnapERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::RunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::RunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x2f4eac: 116 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f4eac() {
}

// 0x2f5018 — __ZN3RBX10RunDragger18findNoSnapPositionERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::RunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")]
// was: RBX::RunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)
// IDA 0x2f5018: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f5018() {
}

// 0x2f53f4 — __ZN3RBX10RunDragger32rotatePart90DegAboutSnapFaceAxisEN3G3D7Vector34AxisE
#[doc(alias = "RBX::RunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)")]
// was: RBX::RunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)
// IDA 0x2f53f4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f53f4() {
}

// 0x2f5410 — __ZN3RBX10RunDragger27rotatePartAboutSnapFaceAxisEN3G3D7Vector34AxisERKf
#[doc(alias = "RBX::RunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)")]
// was: RBX::RunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)
// IDA 0x2f5410: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f5410() {
}

// 0x2f587c — __ZN3G3D5ArrayImLi10ELm32EE6appendERKm
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)")]
// was: G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)
// IDA 0x2f587c: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f587c() {
}

// 0x2f58d8 — __ZN3G3D5ArrayImLi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::resize(int,bool)")]
// was: G3D::Array<unsigned long,10,32ul>::resize(int,bool)
// IDA 0x2f58d8: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f58d8() {
}

// 0x2f5990 — __ZN3G3D5ArrayImLi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::realloc(int)")]
// was: G3D::Array<unsigned long,10,32ul>::realloc(int)
// IDA 0x2f5990: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f5990() {
}

// 0x2f5b78 — __ZN3G3D5ArrayImLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::~Array()")]
// was: G3D::Array<unsigned long,10,32ul>::~Array()
// IDA 0x2f5b78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2f5b78() {
}

// 0x2f5c4c — __ZN3G3D5ArrayImLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::Array(void)")]
// was: G3D::Array<unsigned long,10,32ul>::Array(void)
// IDA 0x2f5c4c: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f5c4c() {
}

// 0x2f75e8 — __ZN3RBX16BoxSelectCommand17getMouseInstancesERSt3setIN5boost10shared_ptrINS_8InstanceEEESt4lessIS5_ESaIS5_EERKNS_7UIEventERN3G3D6Rect2DE
#[doc(alias = "RBX::BoxSelectCommand::getMouseInstances(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)")]
// was: RBX::BoxSelectCommand::getMouseInstances(std::set<boost::shared_ptr<RBX::Instance>,std::less<boost::shared_ptr<RBX::Instance>>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)
// IDA 0x2f75e8: 194 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2f75e8() {
}

// 0x302ed8 — __ZN3RBX4Axes14axisToNormalIdEN3G3D7Vector34AxisE
#[doc(alias = "RBX::Axes::axisToNormalId(G3D::Vector3::Axis)")]
// was: RBX::Axes::axisToNormalId(G3D::Vector3::Axis)
// IDA 0x302ed8: 4 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_302ed8() {
}

// 0x302ee0 — __ZN3RBX4Axes10axisToMaskEN3G3D7Vector34AxisE
#[doc(alias = "RBX::Axes::axisToMask(G3D::Vector3::Axis)")]
// was: RBX::Axes::axisToMask(G3D::Vector3::Axis)
// IDA 0x302ee0: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_302ee0() {
}

// 0x302f1c — __ZNK3RBX4Axes7getAxisEN3G3D7Vector34AxisE
#[doc(alias = "RBX::Axes::getAxis(G3D::Vector3::Axis)const")]
// was: RBX::Axes::getAxis(G3D::Vector3::Axis)const
// IDA 0x302f1c: 10 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_302f1c() {
}

// 0x303124 — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)
// IDA 0x303124: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_303124() {
}

// 0x303128 — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)
// IDA 0x303128: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303128() {
}

// 0x303304 — __ZN3RBX15StringConverterIN3G3D7Vector34AxisEE14convertToValueERKSsRS3_
#[doc(alias = "RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&,G3D::Vector3::Axis&)")]
// was: RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&,G3D::Vector3::Axis&)
// IDA 0x303304: 97 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303304() {
}

// 0x30367c — __ZN3RBX10Reflection8EnumDescIN3G3D7Vector34AxisEE7addPairES4_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::addPair(G3D::Vector3::Axis,char const*)")]
// was: RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::addPair(G3D::Vector3::Axis,char const*)
// IDA 0x30367c: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30367c() {
}

// 0x3039dc — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector34AxisEEERT_v
#[doc(alias = "G3D::Vector3::Axis & RBX::Reflection::Variant::genericConvert<G3D::Vector3::Axis>(void)")]
// was: G3D::Vector3::Axis & RBX::Reflection::Variant::genericConvert<G3D::Vector3::Axis>(void)
// IDA 0x3039dc: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3039dc() {
}

// 0x303bc8 — __ZN3rbx8any_castIN3G3D7Vector34AxisEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3::Axis * rbx::any_cast<G3D::Vector3::Axis,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: G3D::Vector3::Axis * rbx::any_cast<G3D::Vector3::Axis,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// IDA 0x303bc8: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303bc8() {
}

// 0x303c20 — __ZN3rbx8any_castIRN3G3D7Vector34AxisEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3::Axis & rbx::any_cast<G3D::Vector3::Axis &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector3::Axis & rbx::any_cast<G3D::Vector3::Axis &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x303c20: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303c20() {
}

// 0x303d10 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::resize(unsigned long,G3D::Vector3::Axis)")]
// was: std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::resize(unsigned long,G3D::Vector3::Axis)
// IDA 0x303d10: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303d10() {
}

// 0x303d44 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::push_back(G3D::Vector3::Axis const&)")]
// was: std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::push_back(G3D::Vector3::Axis const&)
// IDA 0x303d44: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_303d44() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x303d6c — __ZNSt3mapIPKN3RBX4NameEN3G3D7Vector34AxisESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
#[doc(alias = "std::map<RBX::Name const*,G3D::Vector3::Axis,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,G3D::Vector3::Axis,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::operator[](RBX::Name const* const&)
// IDA 0x303d6c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303d6c() {
}

// 0x303dc4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)
// IDA 0x303dc4: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303dc4() {
}

// 0x303e78 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)
// IDA 0x303e78: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303e78() {
}

// 0x303ed0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)
// IDA 0x303ed0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_303ed0() {
}

// 0x303f38 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,G3D::Vector3::Axis const&)")]
// was: std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,G3D::Vector3::Axis const&)
// IDA 0x303f38: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_303f38() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x30401c — __ZNSt12_Vector_baseIN3G3D7Vector34AxisESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_allocate(unsigned long)
// IDA 0x30401c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_30401c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x304034 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector34AxisES6_EET0_T_S8_S7_
#[doc(alias = "G3D::Vector3::Axis * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3::Axis *,G3D::Vector3::Axis *>(G3D::Vector3::Axis *,G3D::Vector3::Axis *,G3D::Vector3::Axis *)")]
// was: G3D::Vector3::Axis * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3::Axis *,G3D::Vector3::Axis *>(G3D::Vector3::Axis *,G3D::Vector3::Axis *,G3D::Vector3::Axis *)
// IDA 0x304034: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_304034() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x304070 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,unsigned long,G3D::Vector3::Axis const&)")]
// was: std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,unsigned long,G3D::Vector3::Axis const&)
// IDA 0x304070: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_304070() {
}

// 0x3044a0 — __ZN3RBX10BrickColor7closestEN3G3D6Color3E
#[doc(alias = "RBX::BrickColor::closest(G3D::Color3)")]
// was: RBX::BrickColor::closest(G3D::Color3)
// IDA 0x3044a0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3044a0() {
}

// 0x3044c4 — __ZN3RBX10BrickColor7closestEN3G3D6Color4E
#[doc(alias = "RBX::BrickColor::closest(G3D::Color4)")]
// was: RBX::BrickColor::closest(G3D::Color4)
// IDA 0x3044c4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3044c4() {
}

// 0x30dd94 — __ZN3RBX13CameraSubject11doOcclusionERN3G3D7Vector3ERNS1_15CoordinateFrameEf
#[doc(alias = "RBX::CameraSubject::doOcclusion(G3D::Vector3 &,G3D::CoordinateFrame &,float)")]
// was: RBX::CameraSubject::doOcclusion(G3D::Vector3 &,G3D::CoordinateFrame &,float)
// IDA 0x30dd94: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30dd94() {
}

// 0x30de2c — __ZN3RBX13CameraSubject13testOcclusionERKN3G3D7Vector3ERKNS1_15CoordinateFrameERf
#[doc(alias = "RBX::CameraSubject::testOcclusion(G3D::Vector3 const&,G3D::CoordinateFrame const&,float &)")]
// was: RBX::CameraSubject::testOcclusion(G3D::Vector3 const&,G3D::CoordinateFrame const&,float &)
// IDA 0x30de2c: 260 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30de2c() {
}

// 0x30e130 — __ZN3RBX13CameraSubject23cameraPointFromDistanceERKN3G3D7Vector3ES4_f
#[doc(alias = "RBX::CameraSubject::cameraPointFromDistance(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// was: RBX::CameraSubject::cameraPointFromDistance(G3D::Vector3 const&,G3D::Vector3 const&,float)
// IDA 0x30e130: 32 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_30e130() {
}
