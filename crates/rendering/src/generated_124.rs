//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0x27832c..0xf66254 (100 stubs, EA-sorted asc, 12109->12209 covered, 1124 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x27832c — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(G3D::Color3 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(G3D::Color3 const&,lua_State *)
// IDA 0x27832c: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_27832c() {
}

// 0x2d98e4 — __ZN3RBX13AdvRunDragger27rotatePartAboutSnapFaceAxisEN3G3D7Vector34AxisERKf
#[doc(alias = "RBX::AdvRunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)")]
// was: RBX::AdvRunDragger::rotatePartAboutSnapFaceAxis(G3D::Vector3::Axis,float const&)
// IDA 0x2d98e4: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d98e4() {
}

// 0x2e304c — __ZNK3RBX8GameTool13draggablePartEPKNS_12PartInstanceERKN3G3D7Vector3E
#[doc(alias = "RBX::GameTool::draggablePart(RBX::PartInstance const*,G3D::Vector3 const&)const")]
// was: RBX::GameTool::draggablePart(RBX::PartInstance const*,G3D::Vector3 const&)const
// IDA 0x2e304c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2e304c() {
}

// 0x303300 — __ZN3RBX10Reflection7Variant7convertIN3G3D7Vector34AxisEEERT_v
#[doc(alias = "G3D::Vector3::Axis & RBX::Reflection::Variant::convert<G3D::Vector3::Axis>(void)")]
// was: G3D::Vector3::Axis & RBX::Reflection::Variant::convert<G3D::Vector3::Axis>(void)
// IDA 0x303300: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_303300() {
}

// 0x31fdc4 — __ZNK3RBX4Face6minMaxERKN3G3D7Vector3ES4_RfS5_
#[doc(alias = "RBX::Face::minMax(G3D::Vector3 const&,G3D::Vector3 const&,float &,float &)const")]
// was: RBX::Face::minMax(G3D::Vector3 const&,G3D::Vector3 const&,float &,float &)const
// IDA 0x31fdc4: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_31fdc4() {
}

// 0x3596f8 — __ZN3RBX4Math18intersectLinePlaneERKN3G3D4LineERKNS1_5PlaneERNS1_7Vector3E
#[doc(alias = "RBX::Math::intersectLinePlane(G3D::Line const&,G3D::Plane const&,G3D::Vector3 &)")]
// was: RBX::Math::intersectLinePlane(G3D::Line const&,G3D::Plane const&,G3D::Vector3 &)
// IDA 0x3596f8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3596f8() {
}

// 0x3a952c — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)
// IDA 0x3a952c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3a952c() {
}

// 0x3a9ea0 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")]
// was: rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)
// IDA 0x3a9ea0: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3a9ea0() {
}

// 0x3d1900 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEaSERKS2_
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::operator=(G3D::Array<G3D::Plane,10,32ul> const&)")]
// was: G3D::Array<G3D::Plane,10,32ul>::operator=(G3D::Array<G3D::Plane,10,32ul> const&)
// IDA 0x3d1900: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d1900() {
}

// 0x46f740 — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()
// IDA 0x46f740: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_46f740() {
}

// 0x46fb34 — __ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::~PropDescriptor()
// IDA 0x46fb34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_46fb34() {
}

// 0x60bb08 — __ZN3RBX12RootInstance8moveSafeERNS_11MegaDraggerEN3G3D7Vector3ENS_4DRAG8MoveTypeE
#[doc(alias = "RBX::RootInstance::moveSafe(RBX::MegaDragger &,G3D::Vector3,RBX::DRAG::MoveType)")]
// was: RBX::RootInstance::moveSafe(RBX::MegaDragger &,G3D::Vector3,RBX::DRAG::MoveType)
// IDA 0x60bb08: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60bb08() {
}

// 0x679278 — __ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Vector2>::~PropDescriptor()
// IDA 0x679278: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_679278() {
}

// 0x679ea8 — __ZN3RBX10Reflection14PropDescriptorINS_9TextLabelEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::TextLabel,G3D::Vector2>::~PropDescriptor()
// IDA 0x679ea8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_679ea8() {
}

// 0x6b0b90 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D6Color3EEEclES3_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Color3)>::operator()(G3D::Color3)")]
// was: rbx::signals::signal_with_args<1,void ()(G3D::Color3)>::operator()(G3D::Color3)
// IDA 0x6b0b90: 87 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b0b90() {
}

// 0x6b218c — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Color3)>::slot::safe_static_do_get_mutex(void)
// IDA 0x6b218c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b218c() {
}

// 0x6b3138 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::on_error(std::exception &)
// IDA 0x6b3138: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b3138() {
}

// 0x6b4254 — __ZNK3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::connected(void)const
// IDA 0x6b4254: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b4254() {
}

// 0x6f6970 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIN3G3D6Color3EE14hasStringValueEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Color3>::hasStringValue(void)const")]
// was: RBX::Reflection::TypedPropertyDescriptor<G3D::Color3>::hasStringValue(void)const
// IDA 0x6f6970: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6f6970() {
}

// 0x6fc448 — __ZN3RBX10Reflection4TypeC2IN3G3D12Vector2int16EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<G3D::Vector2int16>(char const*,G3D::Vector2int16 *)")]
// was: RBX::Reflection::Type::Type<G3D::Vector2int16>(char const*,G3D::Vector2int16 *)
// IDA 0x6fc448: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fc448() {
}

// 0x6fccb4 — __ZN3RBX10Reflection4TypeC2IN3G3D12Vector3int16EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<G3D::Vector3int16>(char const*,G3D::Vector3int16 *)")]
// was: RBX::Reflection::Type::Type<G3D::Vector3int16>(char const*,G3D::Vector3int16 *)
// IDA 0x6fccb4: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6fccb4() {
}

// 0x868a0c — __ZN3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::~PropDescriptor()
// IDA 0x868a0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_868a0c() {
}

// 0x86a0dc — __ZN3RBX10Reflection14PropDescriptorINS_9FloorWireEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::FloorWire,G3D::Vector2>::~PropDescriptor()
// IDA 0x86a0dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_86a0dc() {
}

// 0x86e440 — __ZNK3RBX19MegaClusterInstance23getCellMaterialInternalERKN3G3D12Vector3int16E
#[doc(alias = "RBX::MegaClusterInstance::getCellMaterialInternal(G3D::Vector3int16 const&)const")]
// was: RBX::MegaClusterInstance::getCellMaterialInternal(G3D::Vector3int16 const&)const
// IDA 0x86e440: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86e440() {
}

// 0x8c441c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_init_mutex(void)
// IDA 0x8c441c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8c441c() {
}

// 0x8e17d8 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::~PropDescriptor()
// IDA 0x8e17d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8e17d8() {
}

// 0x8e2538 — __ZN3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::~PropDescriptor()
// IDA 0x8e2538: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8e2538() {
}

// 0x94a548 — __ZNK3RBX7Frustum12containsAABBERKNS_7ExtentsERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Frustum::containsAABB(RBX::Extents const&,G3D::CoordinateFrame const&)const")]
// was: RBX::Frustum::containsAABB(RBX::Extents const&,G3D::CoordinateFrame const&)const
// IDA 0x94a548: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94a548() {
}

// 0x94acac — __ZN3RBX4Draw12selectionBoxERKNS_4PartEPNS_5AdornERKN3G3D6Color4Ef
#[doc(alias = "RBX::Draw::selectionBox(RBX::Part const&,RBX::Adorn *,G3D::Color4 const&,float)")]
// was: RBX::Draw::selectionBox(RBX::Part const&,RBX::Adorn *,G3D::Color4 const&,float)
// IDA 0x94acac: 281 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_94acac() {
}

// 0x9bf340 — __ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_f
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// was: RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)
// IDA 0x9bf340: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_9bf340() {
}

// 0xa29bdc — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::disconnect(void)
// IDA 0xa29bdc: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_a29bdc() {
}

// 0xb61e00 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_init_mutex(void)
// IDA 0xb61e00: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b61e00() {
}

// 0xb93a08 — __ZN3RBX11AdornRbxGfx11lineSegmentERKN3G3D11LineSegmentERKNS1_6Color4Ef
#[doc(alias = "RBX::AdornRbxGfx::lineSegment(G3D::LineSegment const&,G3D::Color4 const&,float)")]
// was: RBX::AdornRbxGfx::lineSegment(G3D::LineSegment const&,G3D::Color4 const&,float)
// IDA 0xb93a08: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_b93a08() {
}

// 0xb94328 — __ZN3RBX11AdornRbxGfx13convexPolygonEPKN3G3D7Vector3EiRKNS1_6Color4Ei
#[doc(alias = "RBX::AdornRbxGfx::convexPolygon(G3D::Vector3 const*,int,G3D::Color4 const&,int)")]
// was: RBX::AdornRbxGfx::convexPolygon(G3D::Vector3 const*,int,G3D::Color4 const&,int)
// IDA 0xb94328: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_b94328() {
}

// 0xbf5b4c — __ZN5boost6detail17sp_counted_impl_pIN4Ogre13RbxTypesetterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<Ogre::RbxTypesetter>::get_untyped_deleter(void)
// IDA 0xbf5b4c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bf5b4c() {
}

// 0xbfad1c — __ZN4Ogre25RbxSpatialHashedSceneNodeC1EPNS_12SceneManagerE
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::RbxSpatialHashedSceneNode(Ogre::SceneManager *)")]
// was: Ogre::RbxSpatialHashedSceneNode::RbxSpatialHashedSceneNode(Ogre::SceneManager *)
// IDA 0xbfad1c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_bfad1c() {
}

// 0xbfad20 — __ZN4Ogre25RbxSpatialHashedSceneNodeC2EPNS_12SceneManagerE
#[doc(alias = "Ogre::RbxSpatialHashedSceneNode::RbxSpatialHashedSceneNode(Ogre::SceneManager *)")]
// was: Ogre::RbxSpatialHashedSceneNode::RbxSpatialHashedSceneNode(Ogre::SceneManager *)
// IDA 0xbfad20: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_bfad20() {
}

// 0xc50e64 — __ZN4Ogre9Animation27setDefaultInterpolationModeENS0_17InterpolationModeE
#[doc(alias = "Ogre::Animation::setDefaultInterpolationMode(Ogre::Animation::InterpolationMode)")]
// was: Ogre::Animation::setDefaultInterpolationMode(Ogre::Animation::InterpolationMode)
// IDA 0xc50e64: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c50e64() {
}

// 0xc5738c — __ZN4Ogre19AutoParamDataSource16setWorldMatricesEPKNS_7Matrix4Em
#[doc(alias = "Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)")]
// was: Ogre::AutoParamDataSource::setWorldMatrices(Ogre::Matrix4 const*,unsigned long)
// IDA 0xc5738c: 8 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c5738c() {
}

// 0xc6a064 — __ZN4Ogre25BorderPanelOverlayElement13CmdBorderSize5doSetEPvRKSs
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderSize::doSet(void *,std::string const&)")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderSize::doSet(void *,std::string const&)
// IDA 0xc6a064: 245 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6a064() {
}

// 0xc6a554 — __ZNK4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUV5doGetEPKv
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::doGet(void const*)const")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::doGet(void const*)const
// IDA 0xc6a554: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6a554() {
}

// 0xc6a7e8 — __ZNK4Ogre25BorderPanelOverlayElement22CmdBorderBottomRightUV5doGetEPKv
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomRightUV::doGet(void const*)const")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomRightUV::doGet(void const*)const
// IDA 0xc6a7e8: 6 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c6a7e8() {
}

// 0xc6ba1c — __ZN4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUVD1Ev
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()
// IDA 0xc6ba1c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c6ba1c() {
}

// 0xc6ba7c — __ZN4Ogre25BorderPanelOverlayElement21CmdBorderBottomLeftUVD0Ev
#[doc(alias = "Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()")]
// was: Ogre::BorderPanelOverlayElement::CmdBorderBottomLeftUV::~CmdBorderBottomLeftUV()
// IDA 0xc6ba7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c6ba7c() {
}

// 0xc8a3e0 — __ZN4Ogre6Entity22EntityShadowRenderableD2Ev
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()")]
// was: Ogre::Entity::EntityShadowRenderable::~EntityShadowRenderable()
// IDA 0xc8a3e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8a3e0() {
}

// 0xc8a580 — __ZNK4Ogre6Entity22EntityShadowRenderable18getWorldTransformsEPNS_7Matrix4E
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const")]
// was: Ogre::Entity::EntityShadowRenderable::getWorldTransforms(Ogre::Matrix4 *)const
// IDA 0xc8a580: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a580() {
}

// 0xc8a5c8 — __ZNK4Ogre6Entity22EntityShadowRenderable9isVisibleEv
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::isVisible(void)const")]
// was: Ogre::Entity::EntityShadowRenderable::isVisible(void)const
// IDA 0xc8a5c8: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a5c8() {
}

// 0xc8a5e0 — __ZN4Ogre6Entity22EntityShadowRenderable17rebindIndexBufferERKNS_28HardwareIndexBufferSharedPtrE
#[doc(alias = "Ogre::Entity::EntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)")]
// was: Ogre::Entity::EntityShadowRenderable::rebindIndexBuffer(Ogre::HardwareIndexBufferSharedPtr const&)
// IDA 0xc8a5e0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a5e0() {
}

// 0xc8a600 — __ZN4Ogre6Entity19setRenderQueueGroupEh
#[doc(alias = "Ogre::Entity::setRenderQueueGroup(unsigned char)")]
// was: Ogre::Entity::setRenderQueueGroup(unsigned char)
// IDA 0xc8a600: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a600() {
}

// 0xc8a638 — __ZN4Ogre6Entity30setRenderQueueGroupAndPriorityEht
#[doc(alias = "Ogre::Entity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)")]
// was: Ogre::Entity::setRenderQueueGroupAndPriority(unsigned char,unsigned short)
// IDA 0xc8a638: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a638() {
}

// 0xc8a67c — __ZNK4Ogre6Entity12getTypeFlagsEv
#[doc(alias = "Ogre::Entity::getTypeFlags(void)const")]
// was: Ogre::Entity::getTypeFlags(void)const
// IDA 0xc8a67c: 5 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a67c() {
}

// 0xc8a68c — __ZN4Ogre6Entity23getVertexDataForBindingEv
#[doc(alias = "Ogre::Entity::getVertexDataForBinding(void)")]
// was: Ogre::Entity::getVertexDataForBinding(void)
// IDA 0xc8a68c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a68c() {
}

// 0xc8a6f8 — __ZN4Ogre6Entity26chooseVertexDataForBindingEb
#[doc(alias = "Ogre::Entity::chooseVertexDataForBinding(bool)")]
// was: Ogre::Entity::chooseVertexDataForBinding(bool)
// IDA 0xc8a6f8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a6f8() {
}

// 0xc8a738 — __ZN4Ogre6Entity16visitRenderablesEPNS_10Renderable7VisitorEb
#[doc(alias = "Ogre::Entity::visitRenderables(Ogre::Renderable::Visitor *,bool)")]
// was: Ogre::Entity::visitRenderables(Ogre::Renderable::Visitor *,bool)
// IDA 0xc8a738: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a738() {
}

// 0xc8a7d4 — __ZNK4Ogre13EntityFactory7getTypeEv
#[doc(alias = "Ogre::EntityFactory::getType(void)const")]
// was: Ogre::EntityFactory::getType(void)const
// IDA 0xc8a7d4: 3 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a7d4() {
}

// 0xc8a7e0 — __ZN4Ogre13EntityFactory18createInstanceImplERKSsPKSt3mapISsSsSt4lessISsENS_12STLAllocatorISt4pairIS1_SsENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
#[doc(alias = "Ogre::EntityFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
// was: Ogre::EntityFactory::createInstanceImpl(std::string const&,std::map<std::string,std::string,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)
// IDA 0xc8a7e0: 533 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8a7e0() {
}

// 0xc8ad84 — __ZN4Ogre13EntityFactory15destroyInstanceEPNS_13MovableObjectE
#[doc(alias = "Ogre::EntityFactory::destroyInstance(Ogre::MovableObject *)")]
// was: Ogre::EntityFactory::destroyInstance(Ogre::MovableObject *)
// IDA 0xc8ad84: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ad84() {
}

// 0xc8ad98 — __ZN4Ogre14AxisAlignedBox15transformAffineERKNS_7Matrix4E
#[doc(alias = "Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)")]
// was: Ogre::AxisAlignedBox::transformAffine(Ogre::Matrix4 const&)
// IDA 0xc8ad98: 104 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ad98() {
}

// 0xc8af24 — __ZN4Ogre25RuntimeAssertionExceptionD1Ev
#[doc(alias = "Ogre::RuntimeAssertionException::~RuntimeAssertionException()")]
// was: Ogre::RuntimeAssertionException::~RuntimeAssertionException()
// IDA 0xc8af24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8af24() {
}

// 0xc8af30 — __ZN4Ogre8Resource8Listener27backgroundPreparingCompleteEPS0_
#[doc(alias = "Ogre::Resource::Listener::backgroundPreparingComplete(Ogre::Resource*)")]
// was: Ogre::Resource::Listener::backgroundPreparingComplete(Ogre::Resource*)
// IDA 0xc8af30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8af30() {
}

// 0xc8af34 — __ZN4Ogre8Resource8Listener15loadingCompleteEPS0_
#[doc(alias = "Ogre::Resource::Listener::loadingComplete(Ogre::Resource*)")]
// was: Ogre::Resource::Listener::loadingComplete(Ogre::Resource*)
// IDA 0xc8af34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8af34() {
}

// 0xc8af38 — __ZN4Ogre8Resource8Listener17preparingCompleteEPS0_
#[doc(alias = "Ogre::Resource::Listener::preparingComplete(Ogre::Resource*)")]
// was: Ogre::Resource::Listener::preparingComplete(Ogre::Resource*)
// IDA 0xc8af38: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8af38() {
}

// 0xc8af3c — __ZN4Ogre8Resource8Listener17unloadingCompleteEPS0_
#[doc(alias = "Ogre::Resource::Listener::unloadingComplete(Ogre::Resource*)")]
// was: Ogre::Resource::Listener::unloadingComplete(Ogre::Resource*)
// IDA 0xc8af3c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8af3c() {
}

// 0xc8af40 — __ZNK4Ogre16ShadowRenderable11getMaterialEv
#[doc(alias = "Ogre::ShadowRenderable::getMaterial(void)const")]
// was: Ogre::ShadowRenderable::getMaterial(void)const
// IDA 0xc8af40: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8af40() {
}

// 0xc8af44 — __ZN4Ogre16ShadowRenderable18getRenderOperationERNS_15RenderOperationE
#[doc(alias = "Ogre::ShadowRenderable::getRenderOperation(Ogre::RenderOperation &)")]
// was: Ogre::ShadowRenderable::getRenderOperation(Ogre::RenderOperation &)
// IDA 0xc8af44: 8 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8af44() {
}

// 0xc8af60 — __ZNSt6vectorIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
#[doc(alias = "std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::ShadowRenderable **,std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::ShadowRenderable * const&)")]
// was: std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::ShadowRenderable **,std::vector<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::ShadowRenderable * const&)
// IDA 0xc8af60: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8af60() {
}

// 0xc8b108 — __ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,bool>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,bool>> *)
// IDA 0xc8b108: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8b108() {
}

// 0xc8b130 — __ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,bool> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,bool> const&)
// IDA 0xc8b130: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8b130() {
}

// 0xc8b228 — __ZNSt6vectorIPN4Ogre9SubEntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubEntity **,std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubEntity * const&)")]
// was: std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SubEntity **,std::vector<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SubEntity * const&)
// IDA 0xc8b228: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_c8b228() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xc8b320 — __ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
#[doc(alias = "std::_Rb_tree<Ogre::Entity *,Ogre::Entity *,std::_Identity<Ogre::Entity *>,std::less<Ogre::Entity *>,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Entity *>,std::_Rb_tree_iterator<Ogre::Entity *>)")]
// was: std::_Rb_tree<Ogre::Entity *,Ogre::Entity *,std::_Identity<Ogre::Entity *>,std::less<Ogre::Entity *>,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Entity *>,std::_Rb_tree_iterator<Ogre::Entity *>)
// IDA 0xc8b320: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8b320() {
}

// 0xc8b384 — __ZNSt8_Rb_treeIPN4Ogre6EntityES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<Ogre::Entity *,Ogre::Entity *,std::_Identity<Ogre::Entity *>,std::less<Ogre::Entity *>,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Entity *> *)")]
// was: std::_Rb_tree<Ogre::Entity *,Ogre::Entity *,std::_Identity<Ogre::Entity *>,std::less<Ogre::Entity *>,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Entity *> *)
// IDA 0xc8b384: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8b384() {
}

// 0xc8b3ac — __ZNSt6vectorIPN4Ogre6EntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
#[doc(alias = "std::vector<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Entity **,std::vector<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Entity * const&)")]
// was: std::vector<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Entity **,std::vector<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Entity * const&)
// IDA 0xc8b3ac: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_c8b3ac() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xc8b4a4 — __ZNSt12_Vector_baseIPN4Ogre16ShadowRenderableENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::ShadowRenderable *,Ogre::STLAllocator<Ogre::ShadowRenderable *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc8b4a4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8b4a4() {
}

// 0xc8b4a8 — __ZNSt12_Vector_baseIPN4Ogre6EntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc8b4a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8b4a8() {
}

// 0xc8b4ac — __ZNSt12_Vector_baseIPN4Ogre9SubEntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev
#[doc(alias = "std::_Vector_base<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc8b4ac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8b4ac() {
}

// 0xc8b4b0 — __ZNSt12_Vector_baseIPN4Ogre6EntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::Entity *,Ogre::STLAllocator<Ogre::Entity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc8b4b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8b4b0() {
}

// 0xc8b4bc — __ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xc8b4bc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8b4bc() {
}

// 0xc8b4c0 — __ZNSt8_Rb_treeItSt4pairIKtbESt10_Select1stIS2_ESt4lessItEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS6_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,bool>,std::_Select1st<std::pair<unsigned short const,bool>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,bool>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xc8b4c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8b4c0() {
}

// 0xc8b4cc — __ZNSt12_Vector_baseIPN4Ogre9SubEntityENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev
#[doc(alias = "std::_Vector_base<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
// was: std::_Vector_base<Ogre::SubEntity *,Ogre::STLAllocator<Ogre::SubEntity *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
// IDA 0xc8b4cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8b4cc() {
}

// 0xc8b4d8 — __ZN4Ogre25RuntimeAssertionExceptionD0Ev
#[doc(alias = "Ogre::RuntimeAssertionException::~RuntimeAssertionException()")]
// was: Ogre::RuntimeAssertionException::~RuntimeAssertionException()
// IDA 0xc8b4d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8b4d8() {
}

// 0xc8b4ec — __ZN4Ogre16ShadowRenderableD0Ev
#[doc(alias = "Ogre::ShadowRenderable::~ShadowRenderable()")]
// was: Ogre::ShadowRenderable::~ShadowRenderable()
// IDA 0xc8b4ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8b4ec() {
}

// 0xc8b5e8 — __ZN4Ogre9ExceptionC2EiRKSsS2_PKcS4_l
#[doc(alias = "Ogre::Exception::Exception(int,std::string const&,std::string const&,char const*,char const*,long)")]
// was: Ogre::Exception::Exception(int,std::string const&,std::string const&,char const*,char const*,long)
// IDA 0xc8b5e8: 229 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8b5e8() {
}

// 0xc8b870 — __ZN4Ogre9ExceptionC1ERKS0_
#[doc(alias = "Ogre::Exception::Exception(Ogre::Exception const&)")]
// was: Ogre::Exception::Exception(Ogre::Exception const&)
// IDA 0xc8b870: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8b870() {
}

// 0xc8b87c — __ZN4Ogre9ExceptionC2ERKS0_
#[doc(alias = "Ogre::Exception::Exception(Ogre::Exception const&)")]
// was: Ogre::Exception::Exception(Ogre::Exception const&)
// IDA 0xc8b87c: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8b87c() {
}

// 0xc8ba64 — __ZNK4Ogre9Exception18getFullDescriptionEv
#[doc(alias = "Ogre::Exception::getFullDescription(void)const")]
// was: Ogre::Exception::getFullDescription(void)const
// IDA 0xc8ba64: 324 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8ba64() {
}

// 0xc8be0c — __ZNK4Ogre9Exception9getNumberEv
#[doc(alias = "Ogre::Exception::getNumber(void)const")]
// was: Ogre::Exception::getNumber(void)const
// IDA 0xc8be0c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8be0c() {
}

// 0xc8be44 — __ZN4Ogre28ExternalTextureSourceManager12getSingletonEv
#[doc(alias = "Ogre::ExternalTextureSourceManager::getSingleton(void)")]
// was: Ogre::ExternalTextureSourceManager::getSingleton(void)
// IDA 0xc8be44: 4 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8be44() {
}

// 0xc8be54 — __ZN4Ogre28ExternalTextureSourceManagerC1Ev
#[doc(alias = "Ogre::ExternalTextureSourceManager::ExternalTextureSourceManager(void)")]
// was: Ogre::ExternalTextureSourceManager::ExternalTextureSourceManager(void)
// IDA 0xc8be54: 19 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8be54() {
}

// 0xc8be8c — __ZN4Ogre28ExternalTextureSourceManagerD1Ev
#[doc(alias = "Ogre::ExternalTextureSourceManager::~ExternalTextureSourceManager()")]
// was: Ogre::ExternalTextureSourceManager::~ExternalTextureSourceManager()
// IDA 0xc8be8c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8be8c() {
}

// 0xc8bf78 — __ZN4Ogre28ExternalTextureSourceManager16setCurrentPlugInERKSs
#[doc(alias = "Ogre::ExternalTextureSourceManager::setCurrentPlugIn(std::string const&)")]
// was: Ogre::ExternalTextureSourceManager::setCurrentPlugIn(std::string const&)
// IDA 0xc8bf78: 152 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8bf78() {
}

// 0xc8c124 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21ExternalTextureSourceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ExternalTextureSource *>,std::_Select1st<std::pair<std::string const,Ogre::ExternalTextureSource *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ExternalTextureSource *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ExternalTextureSource *>> *)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ExternalTextureSource *>,std::_Select1st<std::pair<std::string const,Ogre::ExternalTextureSource *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ExternalTextureSource *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::ExternalTextureSource *>> *)
// IDA 0xc8c124: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8c124() {
}

// 0xc8c19c — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21ExternalTextureSourceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ExternalTextureSource *>,std::_Select1st<std::pair<std::string const,Ogre::ExternalTextureSource *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ExternalTextureSource *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ExternalTextureSource *>,std::_Select1st<std::pair<std::string const,Ogre::ExternalTextureSource *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ExternalTextureSource *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc8c19c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_c8c19c() {
}

// 0xc8c1a0 — __ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre21ExternalTextureSourceEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ExternalTextureSource *>,std::_Select1st<std::pair<std::string const,Ogre::ExternalTextureSource *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ExternalTextureSource *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,Ogre::ExternalTextureSource *>,std::_Select1st<std::pair<std::string const,Ogre::ExternalTextureSource *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::ExternalTextureSource *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<std::string>,false>::~_Rb_tree_impl()
// IDA 0xc8c1a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_c8c1a0() {
}

// 0xc8c1e0 — __ZN4Ogre17FileSystemArchiveC1ERKSsS2_
#[doc(alias = "Ogre::FileSystemArchive::FileSystemArchive(std::string const&,std::string const&)")]
// was: Ogre::FileSystemArchive::FileSystemArchive(std::string const&,std::string const&)
// IDA 0xc8c1e0: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c8c1e0() {
}

// 0xc938e8 — __ZN4Ogre9SharedPtrINS_7TextureEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::Texture>::operator=(Ogre::SharedPtr<Ogre::Texture> const&)")]
// was: Ogre::SharedPtr<Ogre::Texture>::operator=(Ogre::SharedPtr<Ogre::Texture> const&)
// IDA 0xc938e8: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_c938e8() {
}

// 0xcb6f8c — __ZN4Ogre25HardwareBufferManagerBase23_freeUnusedBufferCopiesEv
#[doc(alias = "Ogre::HardwareBufferManagerBase::_freeUnusedBufferCopies(void)")]
// was: Ogre::HardwareBufferManagerBase::_freeUnusedBufferCopies(void)
// IDA 0xcb6f8c: 285 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_cb6f8c() {
}

// 0xf66224 — j___ZN3RBX9LightGrid26lightingComputeShadowMaskZILb1ELb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<true,false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xf66224: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66224() {
}

// 0xf66234 — j___ZN3RBX9LightGrid26lightingComputeShadowMaskZILb1ELb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskZ<true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskZ<true,true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xf66234: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66234() {
}

// 0xf66244 — j___ZN3RBX9LightGrid27lightingComputeShadowMaskYZILb0EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskYZ<false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskYZ<false>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xf66244: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66244() {
}

// 0xf66254 — j___ZN3RBX9LightGrid27lightingComputeShadowMaskYZILb1EEEvRKNS_14LightGridChunkERKNS_12Vector3int32ES7_S7_RKN3G3D7Vector3E
#[doc(alias = "void RBX::LightGrid::lightingComputeShadowMaskYZ<true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)")]
// was: void RBX::LightGrid::lightingComputeShadowMaskYZ<true>(RBX::LightGridChunk const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,RBX::Vector3int32 const&,G3D::Vector3 const&)
// IDA 0xf66254: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f66254() {
}
