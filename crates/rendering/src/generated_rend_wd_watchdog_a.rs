//! rendering shard rend_wd_watchdog_a — 120 stubs 0x7a19f4..0x880740 EA-sorted asc Ogre|Gfx|Render|G3D|Adorn filtered not yet in crates (global dedup 0x-prefixed) — next 120
//! Source: ida/export.json (85545 funcs) EA asc Ogre/Gfx/Render/G3D/Adorn-filtered then global gap filler — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7a19f4 — __ZN3RBX10ChatOutput8render2dEPNS_5AdornE
// type: int __fastcall(RBX::ChatOutput *this, RBX::Adorn *)
#[doc(alias = "RBX::ChatOutput::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX10ChatOutput8render2dEPNS_5AdornE")]
// IDA 0x7a19f4: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a19f4() {
}

// 0x7a1a38 — __ZN3RBX10ChatOutput20render2d_bubbleStyleEPNS_5AdornEb
// type: void __fastcall(_Rb_tree_node_base **this, RBX::Adorn *, int)
#[doc(alias = "RBX::ChatOutput::render2d_bubbleStyle(RBX::Adorn *,bool)")]
#[doc(alias = "__ZN3RBX10ChatOutput20render2d_bubbleStyleEPNS_5AdornEb")]
// IDA 0x7a1a38: 896 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a1a38() {
}

// 0x7a2400 — __ZN3RBX10ChatOutput21render2d_classicStyleEPNS_5AdornEb
// type: _DWORD __fastcall(RBX::ChatOutput *__hidden this, RBX::Adorn *, bool)
#[doc(alias = "RBX::ChatOutput::render2d_classicStyle(RBX::Adorn *,bool)")]
#[doc(alias = "__ZN3RBX10ChatOutput21render2d_classicStyleEPNS_5AdornEb")]
// IDA 0x7a2400: 107 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a2400() {
}

// 0x7a3f74 — __ZN3RBX16AdornBillboarder10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE
// type: int __fastcall(int, RBX::AdornBillboarder *this, int, int, float, int, int, char, int, int, int, int)
#[doc(alias = "RBX::AdornBillboarder::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX16AdornBillboarder10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE")]
// IDA 0x7a3f74: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a3f74() {
}

// 0x7a9b58 — __ZN3RBX5AdornD0Ev
// type: void __fastcall(RBX::Adorn *__hidden this)
#[doc(alias = "RBX::Adorn::~Adorn()")]
#[doc(alias = "__ZN3RBX5AdornD0Ev")]
// IDA 0x7a9b58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7a9b58() {
}

// 0x7a9bf8 — __ZN3RBX5Adorn17prepareRenderPassEv
// type: _DWORD __fastcall(RBX::Adorn *__hidden this)
#[doc(alias = "RBX::Adorn::prepareRenderPass(void)")]
#[doc(alias = "__ZN3RBX5Adorn17prepareRenderPassEv")]
// IDA 0x7a9bf8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7a9bf8() {
}

// 0x7a9bfc — __ZN3RBX5Adorn13preSubmitPassEv
// type: _DWORD __fastcall(RBX::Adorn *__hidden this)
#[doc(alias = "RBX::Adorn::preSubmitPass(void)")]
#[doc(alias = "__ZN3RBX5Adorn13preSubmitPassEv")]
// IDA 0x7a9bfc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7a9bfc() {
}

// 0x7a9c00 — __ZN3RBX5Adorn10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE
// type: int __fastcall(int, int, int, int, float, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Adorn::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX5Adorn10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE")]
// IDA 0x7a9c00: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a9c00() {
}

// 0x7aa7a8 — __ZN3RBX18UnifiedImageWidget10render2dMeEPNS_5AdornE
// type: _DWORD __fastcall(RBX::UnifiedImageWidget *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::UnifiedImageWidget::render2dMe(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX18UnifiedImageWidget10render2dMeEPNS_5AdornE")]
// IDA 0x7aa7a8: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7aa7a8() {
}

// 0x7abe70 — __ZN3RBX15EquationDisplay8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::EquationDisplay *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::EquationDisplay::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX15EquationDisplay8render2dEPNS_5AdornE")]
// IDA 0x7abe70: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7abe70() {
}

// 0x7ad2b0 — __ZNK3RBX7GuiItem7label2dEPNS_5AdornERKSsRKN3G3D6Color4ES8_NS_4Text6XAlignE
#[doc(alias = "RBX::GuiItem::label2d(RBX::Adorn *,std::string const&,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign)const")]
#[doc(alias = "__ZNK3RBX7GuiItem7label2dEPNS_5AdornERKSsRKN3G3D6Color4ES8_NS_4Text6XAlignE")]
// IDA 0x7ad2b0: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ad2b0() {
}

// 0x7ad5d4 — __ZNK3RBX6Canvas11toPixelSizeERKN3G3D7Vector2E
// type: _DWORD __fastcall(RBX::Canvas *__hidden this, const Vector2 *)
#[doc(alias = "RBX::Canvas::toPixelSize(G3D::Vector2 const&)const")]
#[doc(alias = "__ZNK3RBX6Canvas11toPixelSizeERKN3G3D7Vector2E")]
// IDA 0x7ad5d4: 23 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ad5d4() {
}

// 0x7ad6e8 — __ZN3RBX7GuiRoot8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::GuiRoot *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::GuiRoot::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7GuiRoot8render2dEPNS_5AdornE")]
// IDA 0x7ad6e8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ad6e8() {
}

// 0x7ad720 — __ZN3RBX7GuiRoot12render2dItemEPNS_5AdornEPNS_7GuiItemE
// type: _DWORD __fastcall(RBX::GuiRoot *__hidden this, RBX::Adorn *, RBX::GuiItem *)
#[doc(alias = "RBX::GuiRoot::render2dItem(RBX::Adorn *,RBX::GuiItem *)")]
#[doc(alias = "__ZN3RBX7GuiRoot12render2dItemEPNS_5AdornEPNS_7GuiItemE")]
// IDA 0x7ad720: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ad720() {
}

// 0x7adda8 — __ZN3RBX10TopMenuBar8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::TopMenuBar *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::TopMenuBar::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX10TopMenuBar8render2dEPNS_5AdornE")]
// IDA 0x7adda8: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7adda8() {
}

// 0x7adea4 — __ZN3RBX13UnifiedWidget10render2dMeEPNS_5AdornE
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::UnifiedWidget::render2dMe(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget10render2dMeEPNS_5AdornE")]
// IDA 0x7adea4: 107 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7adea4() {
}

// 0x7adfcc — __ZN3RBX13UnifiedWidget16render2dChildrenEPNS_5AdornE
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::UnifiedWidget::render2dChildren(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget16render2dChildrenEPNS_5AdornE")]
// IDA 0x7adfcc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7adfcc() {
}

// 0x7ae00c — __ZN3RBX13UnifiedWidget8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::UnifiedWidget::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget8render2dEPNS_5AdornE")]
// IDA 0x7ae00c: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ae00c() {
}

// 0x7ae9b8 — __ZN3RBX11TextDisplay8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::TextDisplay *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::TextDisplay::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX11TextDisplay8render2dEPNS_5AdornE")]
// IDA 0x7ae9b8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ae9b8() {
}

// 0x7aecd8 — __ZN3RBX7GuiItem8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::GuiItem::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7GuiItem8render2dEPNS_5AdornE")]
// IDA 0x7aecd8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7aecd8() {
}

// 0x7afdbc — __ZN3RBX12GuiDrawImage8setImageEPNS_5AdornERKNS_9TextureIdEjPN3G3D7Vector2E
// type: _DWORD __fastcall(RBX::GuiDrawImage *__hidden this, RBX::Adorn *, const RBX::TextureId *, unsigned int, G3D::Vector2 *)
#[doc(alias = "RBX::GuiDrawImage::setImage(RBX::Adorn *,RBX::TextureId const&,unsigned int,G3D::Vector2 *)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8setImageEPNS_5AdornERKNS_9TextureIdEjPN3G3D7Vector2E")]
// IDA 0x7afdbc: 1505 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7afdbc() {
}

// 0x7b0fbc — __ZN3RBX12GuiDrawImage16setImageFromNameEPNS_5AdornERKSsj
// type: _DWORD __fastcall(RBX::GuiDrawImage *__hidden this, RBX::Adorn *, const std::string *, unsigned int)
#[doc(alias = "RBX::GuiDrawImage::setImageFromName(RBX::Adorn *,std::string const&,unsigned int)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage16setImageFromNameEPNS_5AdornERKSsj")]
// IDA 0x7b0fbc: 277 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b0fbc() {
}

// 0x7b15fc — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectENS_3Gui11WidgetStateEb
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,RBX::Gui::WidgetState,bool)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectENS_3Gui11WidgetStateEb")]
// IDA 0x7b15fc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b15fc() {
}

// 0x7b163c — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectERKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectERKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb")]
// IDA 0x7b163c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b163c() {
}

// 0x7b1658 — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectES5_RKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectES5_RKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb")]
// IDA 0x7b1658: 437 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b1658() {
}

// 0x7b1a9c — __ZN3RBX12GuiDrawImage9computeUVERN3G3D7Vector2ES3_RKS2_S5_S5_
// type: _DWORD __fastcall(RBX::GuiDrawImage *__hidden this, Vector2 *, Vector2 *, const Vector2 *, const Vector2 *, const Vector2 *)
#[doc(alias = "RBX::GuiDrawImage::computeUV(G3D::Vector2 &,G3D::Vector2 &,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Vector2 const&)")]
#[doc(alias = "__ZN3RBX12GuiDrawImage9computeUVERN3G3D7Vector2ES3_RKS2_S5_S5_")]
// IDA 0x7b1a9c: 82 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b1a9c() {
}

// 0x7b32c4 — __ZN3RBX6Widget8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Widget *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::Widget::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX6Widget8render2dEPNS_5AdornE")]
// IDA 0x7b32c4: 228 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b32c4() {
}

// 0x7b6384 — __ZN3RBX8Humanoid14setWalkToPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Humanoid::setWalkToPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid14setWalkToPointERKN3G3D7Vector3E")]
// IDA 0x7b6384: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6384() {
}

// 0x7b6464 — __ZN3RBX8Humanoid19setTargetPointLocalERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Humanoid::setTargetPointLocal(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid19setTargetPointLocalERKN3G3D7Vector3E")]
// IDA 0x7b6464: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6464() {
}

// 0x7b65ec — __ZN3RBX8Humanoid16setWalkDirectionERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Humanoid::setWalkDirection(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid16setWalkDirectionERKN3G3D7Vector3E")]
// IDA 0x7b65ec: 121 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b65ec() {
}

// 0x7bacac — __ZNK3RBX8Humanoid14hasWalkToPointERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, G3D::Vector3 *)
#[doc(alias = "RBX::Humanoid::hasWalkToPoint(G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX8Humanoid14hasWalkToPointERN3G3D7Vector3E")]
// IDA 0x7bacac: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bacac() {
}

// 0x7bb51c — __ZN3RBX8Humanoid14setTargetPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Humanoid::setTargetPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid14setTargetPointERKN3G3D7Vector3E")]
// IDA 0x7bb51c: 71 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bb51c() {
}

// 0x7bc6bc — __ZN3RBX8Humanoid14renderWaypointEPNS_5AdornERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::Adorn *, const G3D::Vector3 *)
#[doc(alias = "RBX::Humanoid::renderWaypoint(RBX::Adorn *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX8Humanoid14renderWaypointEPNS_5AdornERKN3G3D7Vector3E")]
// IDA 0x7bc6bc: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bc6bc() {
}

// 0x7bc720 — __ZN3RBX8Humanoid13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::Humanoid::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX8Humanoid13render3dAdornEPNS_5AdornE")]
// IDA 0x7bc720: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bc720() {
}

// 0x7bc79c — __ZThn268_N3RBX8Humanoid13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn268_N3RBX8Humanoid13render3dAdornEPNS_5AdornE")]
// IDA 0x7bc79c: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bc79c() {
}

// 0x7bc7a4 — __ZN3RBX8Humanoid17renderMultiplayerEPNS_5AdornERKNS_6CameraE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::Adorn *, const RBX::Camera *)
#[doc(alias = "RBX::Humanoid::renderMultiplayer(RBX::Adorn *,RBX::Camera const&)")]
#[doc(alias = "__ZN3RBX8Humanoid17renderMultiplayerEPNS_5AdornERKNS_6CameraE")]
// IDA 0x7bc7a4: 387 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bc7a4() {
}

// 0x7bccac — __ZN3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::Humanoid::render3dSortedAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE")]
// IDA 0x7bccac: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bccac() {
}

// 0x7bcce0 — __ZThn268_N3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::render3dSortedAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn268_N3RBX8Humanoid19render3dSortedAdornEPNS_5AdornE")]
// IDA 0x7bcce0: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bcce0() {
}

// 0x7bce4c — __ZN3RBX8Humanoid17getRenderLocationEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::getRenderLocation(void)")]
#[doc(alias = "__ZN3RBX8Humanoid17getRenderLocationEv")]
// IDA 0x7bce4c: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bce4c() {
}

// 0x7bcee4 — __ZThn292_N3RBX8Humanoid17getRenderLocationEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::getRenderLocation(void)")]
#[doc(alias = "__ZThn292_N3RBX8Humanoid17getRenderLocationEv")]
// IDA 0x7bcee4: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bcee4() {
}

// 0x7bcef4 — __ZN3RBX8Humanoid13getRenderSizeEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::getRenderSize(void)")]
#[doc(alias = "__ZN3RBX8Humanoid13getRenderSizeEv")]
// IDA 0x7bcef4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bcef4() {
}

// 0x7bcf1c — __ZThn292_N3RBX8Humanoid13getRenderSizeEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::getRenderSize(void)")]
#[doc(alias = "__ZThn292_N3RBX8Humanoid13getRenderSizeEv")]
// IDA 0x7bcf1c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bcf1c() {
}

// 0x7bd338 — __ZN3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, const G3D::Vector3 *, bool)
#[doc(alias = "RBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb")]
// IDA 0x7bd338: 164 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bd338() {
}

// 0x7bd574 — __ZThn292_N3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb
// type: int __fastcall(RBX::Humanoid *this, const G3D::Vector3 *, bool)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZThn292_N3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb")]
// IDA 0x7bd574: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bd574() {
}

// 0x7bf638 — __ZNK3RBX8Humanoid19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid19shouldRender3dAdornEv")]
// IDA 0x7bf638: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bf638() {
}

// 0x7bf63c — __ZNK3RBX8Humanoid25shouldRender3dSortedAdornEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::shouldRender3dSortedAdorn(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid25shouldRender3dSortedAdornEv")]
// IDA 0x7bf63c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bf63c() {
}

// 0x7bf69c — __ZThn268_NK3RBX8Humanoid19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZThn268_NK3RBX8Humanoid19shouldRender3dAdornEv")]
// IDA 0x7bf69c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bf69c() {
}

// 0x7bf6a0 — __ZThn268_NK3RBX8Humanoid25shouldRender3dSortedAdornEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::shouldRender3dSortedAdorn(void)const")]
#[doc(alias = "__ZThn268_NK3RBX8Humanoid25shouldRender3dSortedAdornEv")]
// IDA 0x7bf6a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bf6a0() {
}

// 0x7cdba4 — __ZN3RBX5HUMAN13HumanoidState13render3dAdornEPNS_5AdornE
// type: int __fastcall(RBX::HUMAN::HumanoidState *this, RBX::Adorn *)
#[doc(alias = "RBX::HUMAN::HumanoidState::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState13render3dAdornEPNS_5AdornE")]
// IDA 0x7cdba4: 701 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cdba4() {
}

// 0x7ce378 — __ZN3RBX5HUMAN13HumanoidState10findLadderEPNS_5AdornE
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::HUMAN::HumanoidState::findLadder(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState10findLadderEPNS_5AdornE")]
// IDA 0x7ce378: 400 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ce378() {
}

// 0x7d09a8 — __ZN3RBX5HUMAN13HumanoidState8tryFloorERKNS_6RbxRayERN3G3D7Vector3EfPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, const RBX::RbxRay *, G3D::Vector3 *, float, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::HUMAN::HumanoidState::tryFloor(RBX::RbxRay const&,G3D::Vector3 &,float,RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState8tryFloorERKNS_6RbxRayERN3G3D7Vector3EfPNS_8AssemblyE")]
// IDA 0x7d09a8: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d09a8() {
}

// 0x7d1004 — __ZN3RBX5HUMAN13HumanoidState25findPrimitiveInLadderZoneEPNS_5AdornE
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::HUMAN::HumanoidState::findPrimitiveInLadderZone(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState25findPrimitiveInLadderZoneEPNS_5AdornE")]
// IDA 0x7d1004: 186 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1004() {
}

// 0x7d1230 — __ZN3RBX5HUMAN13HumanoidState15doLadderRaycastEPNS_15GeometryServiceERKNS_6RbxRayEPNS_8HumanoidEPPNS_9PrimitiveEPN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, RBX::GeometryService *, const RBX::RbxRay *, RBX::Humanoid *, RBX::Primitive **, G3D::Vector3 *)
#[doc(alias = "RBX::HUMAN::HumanoidState::doLadderRaycast(RBX::GeometryService *,RBX::RbxRay const&,RBX::Humanoid *,RBX::Primitive **,G3D::Vector3 *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState15doLadderRaycastEPNS_15GeometryServiceERKNS_6RbxRayEPNS_8HumanoidEPPNS_9PrimitiveEPN3G3D7Vector3E")]
// IDA 0x7d1230: 202 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1230() {
}

// 0x7f964c — __ZN3RBX10Soundscape14CollisionSound4PlayEPN4FMOD6SystemEPNS2_12ChannelGroupERKN3G3D7Vector3ESA_f
// type: _DWORD __fastcall(RBX::Soundscape::CollisionSound *__hidden this, FMOD::System *, FMOD::ChannelGroup *, const G3D::Vector3 *, const G3D::Vector3 *, boost::detail::sp_counted_base *)
#[doc(alias = "RBX::Soundscape::CollisionSound::Play(FMOD::System *,FMOD::ChannelGroup *,G3D::Vector3 const&,G3D::Vector3 const&,float)")]
#[doc(alias = "__ZN3RBX10Soundscape14CollisionSound4PlayEPN4FMOD6SystemEPNS2_12ChannelGroupERKN3G3D7Vector3ESA_f")]
// IDA 0x7f964c: 203 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7f964c() {
}

// 0x816a34 — __ZN3RBX7Region216getRelativeErrorERKN3G3D7Vector2ERKNS0_13WeightedPointE
// type: float __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Region2::getRelativeError(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&)")]
#[doc(alias = "__ZN3RBX7Region216getRelativeErrorERKN3G3D7Vector2ERKNS0_13WeightedPointE")]
// IDA 0x816a34: 14 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816a34() {
}

// 0x816a6c — __ZN3RBX7Region212pointInRangeERKN3G3D7Vector2ERKNS0_13WeightedPointEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Region2::pointInRange(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,float)")]
#[doc(alias = "__ZN3RBX7Region212pointInRangeERKN3G3D7Vector2ERKNS0_13WeightedPointEf")]
// IDA 0x816a6c: 19 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816a6c() {
}

// 0x816ab0 — __ZNK3RBX7Region28containsERKN3G3D7Vector2Ef
// type: _DWORD __fastcall(RBX::Region2 *__hidden this, const G3D::Vector2 *, float)
#[doc(alias = "RBX::Region2::contains(G3D::Vector2 const&,float)const")]
#[doc(alias = "__ZNK3RBX7Region28containsERKN3G3D7Vector2Ef")]
// IDA 0x816ab0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816ab0() {
}

// 0x816b04 — __ZNK3RBX7Region215findCloserOtherERKN3G3D7Vector2Ef
// type: _DWORD __fastcall(RBX::Region2 *__hidden this, const G3D::Vector2 *, float)
#[doc(alias = "RBX::Region2::findCloserOther(G3D::Vector2 const&,float)const")]
#[doc(alias = "__ZNK3RBX7Region215findCloserOtherERKN3G3D7Vector2Ef")]
// IDA 0x816b04: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816b04() {
}

// 0x816b54 — __ZN3RBX7Region218closerToOtherPointERKN3G3D7Vector2ERKNS0_13WeightedPointES7_f
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Region2::closerToOtherPoint(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,RBX::Region2::WeightedPoint const&,float)")]
#[doc(alias = "__ZN3RBX7Region218closerToOtherPointERKN3G3D7Vector2ERKNS0_13WeightedPointES7_f")]
// IDA 0x816b54: 43 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816b54() {
}

// 0x816d20 — __ZN3RBX7Region3C1ERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Region3 *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Region3::Region3(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7Region3C1ERKN3G3D7Vector3ES4_")]
// IDA 0x816d20: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816d20() {
}

// 0x83548c — __ZN3RBX15NotificationBox8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::NotificationBox::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX15NotificationBox8render2dEPNS_5AdornE")]
// IDA 0x83548c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_83548c() {
}

// 0x835490 — __ZThn96_N3RBX15NotificationBox8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX15NotificationBox8render2dEPNS_5AdornE")]
// IDA 0x835490: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835490() {
}

// 0x83753c — __ZN3RBX18NotificationObject8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::NotificationObject *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::NotificationObject::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX18NotificationObject8render2dEPNS_5AdornE")]
// IDA 0x83753c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_83753c() {
}

// 0x837540 — __ZThn96_N3RBX18NotificationObject8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::NotificationObject *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX18NotificationObject8render2dEPNS_5AdornE")]
// IDA 0x837540: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837540() {
}

// 0x84fa9c — __ZN3RBX18RenderHooksService14captureMetricsEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::captureMetrics(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService14captureMetricsEv")]
// IDA 0x84fa9c: 9 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84fa9c() {
}

// 0x84fab0 — __ZN3RBX18RenderHooksService12resizeWindowEii
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this, int, int)
#[doc(alias = "RBX::RenderHooksService::resizeWindow(int,int)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12resizeWindowEii")]
// IDA 0x84fab0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84fab0() {
}

// 0x84fac0 — __ZN3RBX18RenderHooksService12enableAdornsEb
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this, bool)
#[doc(alias = "RBX::RenderHooksService::enableAdorns(bool)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12enableAdornsEb")]
// IDA 0x84fac0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84fac0() {
}

// 0x84fad0 — __ZN3RBX18RenderHooksService10printSceneEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::printScene(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService10printSceneEv")]
// IDA 0x84fad0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84fad0() {
}

// 0x84fae0 — __ZN3RBX18RenderHooksServiceC1Ev
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::RenderHooksService(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceC1Ev")]
// IDA 0x84fae0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_84fae0() {
}

// 0x84fae4 — __ZN3RBX18RenderHooksServiceC2Ev
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::RenderHooksService(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceC2Ev")]
// IDA 0x84fae4: 350 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84fae4() {
}

// 0x84fea4 — __ZN3RBX18RenderHooksService13reloadShadersEv
// type: int __fastcall(RBX::RenderHooksService *this)
#[doc(alias = "RBX::RenderHooksService::reloadShaders(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService13reloadShadersEv")]
// IDA 0x84fea4: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84fea4() {
}

// 0x84fed0 — __ZN3RBX18RenderHooksService11enableQueueEi
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this, int)
#[doc(alias = "RBX::RenderHooksService::enableQueue(int)")]
#[doc(alias = "__ZN3RBX18RenderHooksService11enableQueueEi")]
// IDA 0x84fed0: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84fed0() {
}

// 0x84ff18 — __ZN3RBX18RenderHooksService12disableQueueEi
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this, int)
#[doc(alias = "RBX::RenderHooksService::disableQueue(int)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12disableQueueEi")]
// IDA 0x84ff18: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ff18() {
}

// 0x84ff68 — __ZN3RBX18RenderHooksService14getPresentTimeEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::getPresentTime(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService14getPresentTimeEv")]
// IDA 0x84ff68: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ff68() {
}

// 0x84ff98 — __ZN3RBX18RenderHooksService11getGPUDelayEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::getGPUDelay(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService11getGPUDelayEv")]
// IDA 0x84ff98: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ff98() {
}

// 0x84ffa4 — __ZN3RBX18RenderHooksService12getRenderAveEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::getRenderAve(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12getRenderAveEv")]
// IDA 0x84ffa4: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ffa4() {
}

// 0x84ffb0 — __ZN3RBX18RenderHooksService16getRenderConfMinEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::getRenderConfMin(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService16getRenderConfMinEv")]
// IDA 0x84ffb0: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ffb0() {
}

// 0x84ffbc — __ZN3RBX18RenderHooksService16getRenderConfMaxEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::getRenderConfMax(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService16getRenderConfMaxEv")]
// IDA 0x84ffbc: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ffbc() {
}

// 0x84ffc8 — __ZN3RBX18RenderHooksService12getRenderStdEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::getRenderStd(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService12getRenderStdEv")]
// IDA 0x84ffc8: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ffc8() {
}

// 0x84ffd4 — __ZN3RBX18RenderHooksService11getDeltaAveEv
// type: _DWORD __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::getDeltaAve(void)")]
#[doc(alias = "__ZN3RBX18RenderHooksService11getDeltaAveEv")]
// IDA 0x84ffd4: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_84ffd4() {
}

// 0x850020 — __ZN3RBX18RenderHooksServiceD1Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceD1Ev")]
// IDA 0x850020: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_850020() {
}

// 0x850024 — __ZN3RBX18RenderHooksServiceD0Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceD0Ev")]
// IDA 0x850024: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_850024() {
}

// 0x8500ec — __ZThn32_N3RBX18RenderHooksServiceD1Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn32_N3RBX18RenderHooksServiceD1Ev")]
// IDA 0x8500ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8500ec() {
}

// 0x8500f4 — __ZThn32_N3RBX18RenderHooksServiceD0Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn32_N3RBX18RenderHooksServiceD0Ev")]
// IDA 0x8500f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8500f4() {
}

// 0x8501c0 — __ZThn36_N3RBX18RenderHooksServiceD1Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn36_N3RBX18RenderHooksServiceD1Ev")]
// IDA 0x8501c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8501c0() {
}

// 0x8501c8 — __ZThn36_N3RBX18RenderHooksServiceD0Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZThn36_N3RBX18RenderHooksServiceD0Ev")]
// IDA 0x8501c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8501c8() {
}

// 0x85026c — __ZN3RBX18RenderHooksServiceD2Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
#[doc(alias = "__ZN3RBX18RenderHooksServiceD2Ev")]
// IDA 0x85026c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_85026c() {
}

// 0x855e20 — __ZN3RBX17ClientAppSettings30ReadValueAxisAdornmentGrabSizeEPKc
// type: _DWORD __fastcall(RBX::ClientAppSettings *__hidden this, const char *)
#[doc(alias = "RBX::ClientAppSettings::ReadValueAxisAdornmentGrabSize(char const*)")]
#[doc(alias = "__ZN3RBX17ClientAppSettings30ReadValueAxisAdornmentGrabSizeEPKc")]
// IDA 0x855e20: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_855e20() {
}

// 0x85df88 — __ZN3RBX12TextureTrail14setTextureSizeEN3G3D7Vector2E
// type: _DWORD __fastcall(RBX::TextureTrail *__hidden this, Vector2)
#[doc(alias = "RBX::TextureTrail::setTextureSize(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX12TextureTrail14setTextureSizeEN3G3D7Vector2E")]
// IDA 0x85df88: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85df88() {
}

// 0x85e2bc — __ZN3RBX12TextureTrail13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::TextureTrail *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::TextureTrail::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX12TextureTrail13render3dAdornEPNS_5AdornE")]
// IDA 0x85e2bc: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85e2bc() {
}

// 0x85e7f0 — __ZThn96_N3RBX12TextureTrail13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::TextureTrail *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::TextureTrail::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX12TextureTrail13render3dAdornEPNS_5AdornE")]
// IDA 0x85e7f0: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_85e7f0() {
}

// 0x867a90 — __ZN3RBX9FloorWire14setTextureSizeEN3G3D7Vector2E
// type: _DWORD __fastcall(RBX::FloorWire *__hidden this, Vector2)
#[doc(alias = "RBX::FloorWire::setTextureSize(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX9FloorWire14setTextureSizeEN3G3D7Vector2E")]
// IDA 0x867a90: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_867a90() {
}

// 0x867de4 — __ZN3RBX9FloorWire13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::FloorWire *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::FloorWire::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX9FloorWire13render3dAdornEPNS_5AdornE")]
// IDA 0x867de4: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_867de4() {
}

// 0x8685d8 — __ZThn96_N3RBX9FloorWire13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::FloorWire *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::FloorWire::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZThn96_N3RBX9FloorWire13render3dAdornEPNS_5AdornE")]
// IDA 0x8685d8: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8685d8() {
}

// 0x868ce0 — __ZNK3RBX9GuiBase3d19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::GuiBase3d *__hidden this)
#[doc(alias = "RBX::GuiBase3d::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX9GuiBase3d19shouldRender3dAdornEv")]
// IDA 0x868ce0: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_868ce0() {
}

// 0x8691c8 — __ZThn96_NK3RBX9GuiBase3d19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::GuiBase3d *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiBase3d::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZThn96_NK3RBX9GuiBase3d19shouldRender3dAdornEv")]
// IDA 0x8691c8: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8691c8() {
}

// 0x86f1ac — __ZN3RBX12MovePositionERN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE
#[doc(alias = "RBX::MovePosition(G3D::Vector3int16 &,RBX::Voxel::FaceDirection)")]
#[doc(alias = "__ZN3RBX12MovePositionERN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE")]
// IDA 0x86f1ac: 36 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_86f1ac() {
}

// 0x87b3bc — __ZN3RBX15MegaClusterPoly7hitTestERKNS_6RbxRayERN3G3D7Vector3ERbfRNS_6CellIDEbb
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, G3D::Vector3 *, bool *, float, RBX::CellID *, bool, bool)
#[doc(alias = "RBX::MegaClusterPoly::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &,float,RBX::CellID &,bool,bool)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly7hitTestERKNS_6RbxRayERN3G3D7Vector3ERbfRNS_6CellIDEbb")]
// IDA 0x87b3bc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b3bc() {
}

// 0x87b414 — __ZN3RBX15MegaClusterPoly9hitTestMCERKNS_6RbxRayERN3G3D7Vector3ERbRiRNS4_15CoordinateFrameEfRNS_6CellIDEbb
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, G3D::Vector3 *, bool *, int *, G3D::CoordinateFrame *, float, RBX::CellID *, bool, bool)
#[doc(alias = "RBX::MegaClusterPoly::hitTestMC(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly9hitTestMCERKNS_6RbxRayERN3G3D7Vector3ERbRiRNS4_15CoordinateFrameEfRNS_6CellIDEbb")]
// IDA 0x87b414: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b414() {
}

// 0x87b488 — __ZNK3RBX15MegaClusterPoly26findTouchingSurfacesConvexERKN3G3D15CoordinateFrameERmRKNS_8GeometryES4_S5_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::CoordinateFrame *, unsigned int *, const RBX::Geometry *, const G3D::CoordinateFrame *, unsigned int *)
#[doc(alias = "RBX::MegaClusterPoly::findTouchingSurfacesConvex(G3D::CoordinateFrame const&,unsigned long &,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly26findTouchingSurfacesConvexERKN3G3D15CoordinateFrameERmRKNS_8GeometryES4_S5_")]
// IDA 0x87b488: 113 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b488() {
}

// 0x87b5bc — __ZNK3RBX15MegaClusterPoly35findCellsTouchingGeometryWithBufferERKfRKN3G3D15CoordinateFrameERKNS_8GeometryES6_PSt3mapIiPNS3_12Vector3int16ESt4lessIiESaISt4pairIKiSC_EEE
// type: int __fastcall(int, int, int, int, G3D::CoordinateFrame *, int)
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometryWithBuffer(float const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly35findCellsTouchingGeometryWithBufferERKfRKN3G3D15CoordinateFrameERKNS_8GeometryES6_PSt3mapIiPNS3_12Vector3int16ESt4lessIiESaISt4pairIKiSC_EEE")]
// IDA 0x87b5bc: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b5bc() {
}

// 0x87b784 — __ZNK3RBX15MegaClusterPoly25findPlanarTouchesWithGeomERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::MegaClusterPoly::findPlanarTouchesWithGeom(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly25findPlanarTouchesWithGeomERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE")]
// IDA 0x87b784: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b784() {
}

// 0x87b828 — __ZNK3RBX15MegaClusterPoly22hasPlanarTouchWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3int16 *, const G3D::CoordinateFrame *, const RBX::Geometry *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hasPlanarTouchWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly22hasPlanarTouchWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_")]
// IDA 0x87b828: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b828() {
}

// 0x87b874 — __ZNK3RBX15MegaClusterPoly28findCellIntersectionWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_Rm
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3int16 *, const G3D::CoordinateFrame *, const RBX::Geometry *, const G3D::CoordinateFrame *, unsigned int *)
#[doc(alias = "RBX::MegaClusterPoly::findCellIntersectionWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly28findCellIntersectionWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_Rm")]
// IDA 0x87b874: 500 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87b874() {
}

// 0x87be18 — __ZNK3RBX15MegaClusterPoly28hitLocationOnCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, const int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly28hitLocationOnCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87be18: 477 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87be18() {
}

// 0x87c450 — __ZNK3RBX15MegaClusterPoly32hitLocationOnHorizontalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: int __fastcall(RBX::MegaClusterPoly *this, const RBX::RbxRay *, const G3D::Vector3int16 *, int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnHorizontalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly32hitLocationOnHorizontalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87c450: 593 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87c450() {
}

// 0x87cc0c — __ZNK3RBX15MegaClusterPoly30hitLocationOnVerticalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: int __fastcall(RBX::MegaClusterPoly *this, const RBX::RbxRay *, const G3D::Vector3int16 *, int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnVerticalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly30hitLocationOnVerticalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87cc0c: 599 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87cc0c() {
}

// 0x87d3e0 — __ZNK3RBX15MegaClusterPoly35hitLocationOnInverseCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, const int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnInverseCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly35hitLocationOnInverseCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87d3e0: 783 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87d3e0() {
}

// 0x87de28 — __ZNK3RBX15MegaClusterPoly22hitLocationOnBlockCellERKNS_6RbxRayERKN3G3D12Vector3int16ERNS4_7Vector3ERiRNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, G3D::Vector3 *, int *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnBlockCell(RBX::RbxRay const&,G3D::Vector3int16 const&,G3D::Vector3 &,int &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly22hitLocationOnBlockCellERKNS_6RbxRayERKN3G3D12Vector3int16ERNS4_7Vector3ERiRNS4_15CoordinateFrameE")]
// IDA 0x87de28: 704 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87de28() {
}

// 0x87e738 — __ZNK3RBX15MegaClusterPoly25findCellsTouchingGeometryERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
// type: int __fastcall(int, int, int, G3D::CoordinateFrame *, int)
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometry(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly25findCellsTouchingGeometryERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE")]
// IDA 0x87e738: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87e738() {
}

// 0x87e758 — __ZN3RBX15MegaClusterPoly18cellsInBoundingBoxERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::MegaClusterPoly::cellsInBoundingBox(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly18cellsInBoundingBoxERKN3G3D7Vector3ES4_")]
// IDA 0x87e758: 620 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87e758() {
}

// 0x87edfc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE8getTokenERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE8getTokenERKS2_")]
// IDA 0x87edfc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87edfc() {
}

// 0x87ef60 — __ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_5Voxel4GridEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS7_15CoordinateFrameEfRNS_6CellIDEbb
// type: int __fastcall(int, int, int, int, int, G3D::CoordinateFrame *, float, int, int, int)
#[doc(alias = "bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::Voxel::Grid>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_5Voxel4GridEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS7_15CoordinateFrameEfRNS_6CellIDEbb")]
// IDA 0x87ef60: 500 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_87ef60() {
}

// 0x88004c — __ZN3RBX24getRegionForCellLocationINS_5Voxel4GridEEEKNT_6RegionEPKS3_RKN3G3D12Vector3int16EPS4_
// type: int __fastcall(int, int, G3D::Vector3int16 *this)
#[doc(alias = "RBX::Voxel::Grid::Region const RBX::getRegionForCellLocation<RBX::Voxel::Grid>(RBX::Voxel::Grid::Region const*,G3D::Vector3int16 const&,RBX::Voxel::Grid::Region const*)")]
#[doc(alias = "__ZN3RBX24getRegionForCellLocationINS_5Voxel4GridEEEKNT_6RegionEPKS3_RKN3G3D12Vector3int16EPS4_")]
// IDA 0x88004c: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88004c() {
}

// 0x8800ec — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_")]
// IDA 0x8800ec: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8800ec() {
}

// 0x880344 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE")]
// IDA 0x880344: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880344() {
}

// 0x880520 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountD2Ev")]
// IDA 0x880520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_880520() {
}

// 0x8806b8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_")]
// IDA 0x8806b8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8806b8() {
}

// 0x8806e0 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_")]
// IDA 0x8806e0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8806e0() {
}

// 0x880740 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
// IDA 0x880740: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_880740() {
}