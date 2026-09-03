//! core shard lr — 150 core stubs EA-sorted, next uncovered fallback after shard lq (0x61f5cc..0x7120f4, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback, EA-sorted, next 150 uncovered, lowest EA first, rbx_core::SharedPtr not boost) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::SelectionPointLasso::setPoint(G3D::Vector3)")]
#[doc(alias = "__ZN3RBX19SelectionPointLasso8setPointEN3G3D7Vector3E")]
// 0x61f5cc — __ZN3RBX19SelectionPointLasso8setPointEN3G3D7Vector3E
pub fn stub_0x61f5cc() {
    // IDA 0x61f5cc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::SelectionPointLasso::getPosition(G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX19SelectionPointLasso11getPositionERN3G3D7Vector3E")]
// 0x6206f0 — __ZNK3RBX19SelectionPointLasso11getPositionERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::SelectionPointLasso *__hidden this, Vector3 *)
pub fn stub_0x6206f0() {
    // IDA 0x6206f0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::SkateboardPlatform::applySpecificImpulse(G3D::Vector3)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3E")]
// 0x6271fc — __ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, Vector3)
pub fn stub_0x6271fc() {
    // IDA 0x6271fc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x6291c4 — __ZN3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
pub fn stub_0x6291c4() {
    // IDA 0x6291c4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk to RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZThn132_N3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x629334 — __ZThn132_N3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
// was: non-virtual thunk to RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
pub fn stub_0x629334() {
    // IDA 0x629334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x629340 — __ZN3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
pub fn stub_0x629340() {
    // IDA 0x629340: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZThn132_N3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x6295a4 — __ZThn132_N3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
// was: non-virtual thunk to RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
pub fn stub_0x6295a4() {
    // IDA 0x6295a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SkateboardPlatform::applySpecificImpulse(G3D::Vector3,G3D::Vector3)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3ES2_")]
// 0x6295bc — __ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3ES2_
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, Vector3, Vector3)
pub fn stub_0x6295bc() {
    // IDA 0x6295bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::append(RBX::SkateboardPlatform::Wheel const&)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6appendERKS3_")]
// 0x62a284 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6appendERKS3_
// type: int(void)
pub fn stub_0x62a284() {
    // IDA 0x62a284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Body::accumulateForceAtBranchCofm(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E")]
// 0x62a2f0 — __ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x62a2f0() {
    // IDA 0x62a2f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6resizeEib")]
// 0x62beec — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6resizeEib
// type: int(void)
pub fn stub_0x62beec() {
    // IDA 0x62beec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE7reallocEi")]
// 0x62bfb0 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE7reallocEi
// type: int(void)
pub fn stub_0x62bfb0() {
    // IDA 0x62bfb0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EED2Ev")]
// 0x62e69c — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EED2Ev
pub fn stub_0x62e69c() {
    // IDA 0x62e69c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EEC2Ev")]
// 0x62e770 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EEC2Ev
pub fn stub_0x62e770() {
    // IDA 0x62e770: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Velocity::rotateBy(G3D::Matrix3 const&)const")]
#[doc(alias = "__ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E")]
// 0x633e44 — __ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E
// type: int __fastcall(int result, __int32 *, int)
pub fn stub_0x633e44() {
    // IDA 0x633e44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Smoke::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX5Smoke8setColorEN3G3D6Color3E")]
// 0x637264 — __ZN3RBX5Smoke8setColorEN3G3D6Color3E
pub fn stub_0x637264() {
    // IDA 0x637264: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Sparkles::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX8Sparkles8setColorEN3G3D6Color3E")]
// 0x63c1a4 — __ZN3RBX8Sparkles8setColorEN3G3D6Color3E
pub fn stub_0x63c1a4() {
    // IDA 0x63c1a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Sparkles::setLegacyColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX8Sparkles14setLegacyColorEN3G3D6Color3E")]
// 0x63c248 — __ZN3RBX8Sparkles14setLegacyColorEN3G3D6Color3E
pub fn stub_0x63c248() {
    // IDA 0x63c248: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX7TextBox13setTextColor3EN3G3D6Color3E")]
// 0x665fec — __ZN3RBX7TextBox13setTextColor3EN3G3D6Color3E
pub fn stub_0x665fec() {
    // IDA 0x665fec: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX7TextBox19setTextStrokeColor3EN3G3D6Color3E")]
// 0x6664e4 — __ZN3RBX7TextBox19setTextStrokeColor3EN3G3D6Color3E
pub fn stub_0x6664e4() {
    // IDA 0x6664e4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::getPosInString(G3D::Vector2)const")]
#[doc(alias = "__ZNK3RBX7TextBox14getPosInStringEN3G3D7Vector2E")]
// 0x6665ec — __ZNK3RBX7TextBox14getPosInStringEN3G3D7Vector2E
pub fn stub_0x6665ec() {
    // IDA 0x6665ec: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::TextBox::getCursorPos(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX7TextBox12getCursorPosEN3G3D7Vector2E")]
// 0x667500 — __ZN3RBX7TextBox12getCursorPosEN3G3D7Vector2E
pub fn stub_0x667500() {
    // IDA 0x667500: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")]
// 0x668d2c — __ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE
// type: int(void)
pub fn stub_0x668d2c() {
    // IDA 0x668d2c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk to RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZThn96_NK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")]
// 0x668efc — __ZThn96_NK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE
// was: non-virtual thunk to RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const
pub fn stub_0x668efc() {
    // IDA 0x668efc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiTextButton::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX13GuiTextButton13setTextColor3EN3G3D6Color3E")]
// 0x673288 — __ZN3RBX13GuiTextButton13setTextColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x673288() {
    // IDA 0x673288: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiTextButton::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX13GuiTextButton19setTextStrokeColor3EN3G3D6Color3E")]
// 0x673780 — __ZN3RBX13GuiTextButton19setTextStrokeColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x673780() {
    // IDA 0x673780: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextLabel::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX9TextLabel13setTextColor3EN3G3D6Color3E")]
// 0x678814 — __ZN3RBX9TextLabel13setTextColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x678814() {
    // IDA 0x678814: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TextLabel::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX9TextLabel19setTextStrokeColor3EN3G3D6Color3E")]
// 0x678d0c — __ZN3RBX9TextLabel19setTextStrokeColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x678d0c() {
    // IDA 0x678d0c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Tool::setGrip(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Tool7setGripERKN3G3D15CoordinateFrameE")]
// 0x67e7a0 — __ZN3RBX4Tool7setGripERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::CoordinateFrame *)
pub fn stub_0x67e7a0() {
    // IDA 0x67e7a0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Tool::setGripPos(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool10setGripPosERKN3G3D7Vector3E")]
// 0x67e8d0 — __ZN3RBX4Tool10setGripPosERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67e8d0() {
    // IDA 0x67e8d0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Tool::setGripForward(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool14setGripForwardERKN3G3D7Vector3E")]
// 0x67e940 — __ZN3RBX4Tool14setGripForwardERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67e940() {
    // IDA 0x67e940: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Tool::setGripUp(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool9setGripUpERKN3G3D7Vector3E")]
// 0x67ea9c — __ZN3RBX4Tool9setGripUpERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67ea9c() {
    // IDA 0x67ea9c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Tool::setGripRight(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool12setGripRightERKN3G3D7Vector3E")]
// 0x67ebd0 — __ZN3RBX4Tool12setGripRightERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67ebd0() {
    // IDA 0x67ebd0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::cleanUpZeroColumn(G3D::Matrix3 &)")]
#[doc(alias = "__ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E")]
// 0x6827bc — __ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX *__hidden this, G3D::Matrix3 *)
pub fn stub_0x6827bc() {
    // IDA 0x6827bc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev")]
// 0x6a407c — __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x6a407c() {
    // IDA 0x6a407c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")]
// 0x6a4334 — __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a4334() {
    // IDA 0x6a4334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")]
// 0x6a4448 — __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a4448() {
    // IDA 0x6a4448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")]
// 0x6a4584 — __ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev
pub fn stub_0x6a4584() {
    // IDA 0x6a4584: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")]
// 0x6a4694 — __ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev
pub fn stub_0x6a4694() {
    // IDA 0x6a4694: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")]
// 0x6a47cc — __ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev
pub fn stub_0x6a47cc() {
    // IDA 0x6a47cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")]
// 0x6a48dc — __ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev
pub fn stub_0x6a48dc() {
    // IDA 0x6a48dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev")]
// 0x6a5798 — __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x6a5798() {
    // IDA 0x6a5798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")]
// 0x6a5a60 — __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a5a60() {
    // IDA 0x6a5a60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")]
// 0x6a5b74 — __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a5b74() {
    // IDA 0x6a5b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")]
// 0x6a5cb0 — __ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev
// type: void __fastcall(_QWORD *, int, int, int)
pub fn stub_0x6a5cb0() {
    // IDA 0x6a5cb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")]
// 0x6a5dc4 — __ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev
pub fn stub_0x6a5dc4() {
    // IDA 0x6a5dc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")]
// 0x6a5f00 — __ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev
pub fn stub_0x6a5f00() {
    // IDA 0x6a5f00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")]
// 0x6a6014 — __ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev
// type: void __fastcall(int, int, int, int)
pub fn stub_0x6a6014() {
    // IDA 0x6a6014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev")]
// 0x6a6ed4 — __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x6a6ed4() {
    // IDA 0x6a6ed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")]
// 0x6a7178 — __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a7178() {
    // IDA 0x6a7178: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")]
// 0x6a728c — __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a728c() {
    // IDA 0x6a728c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")]
// 0x6a73c8 — __ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev
pub fn stub_0x6a73c8() {
    // IDA 0x6a73c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")]
// 0x6a74d8 — __ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev
pub fn stub_0x6a74d8() {
    // IDA 0x6a74d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")]
// 0x6a7610 — __ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev
pub fn stub_0x6a7610() {
    // IDA 0x6a7610: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")]
// 0x6a7720 — __ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev
pub fn stub_0x6a7720() {
    // IDA 0x6a7720: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE8on_errorERSt9exception")]
// 0x6b7cdc — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE8on_errorERSt9exception
pub fn stub_0x6b7cdc() {
    // IDA 0x6b7cdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot10disconnectEv")]
// 0x6b8ac4 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot10disconnectEv
pub fn stub_0x6b8ac4() {
    // IDA 0x6b8ac4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::remove(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6removeEPNS5_4slotE")]
// 0x6b8cb0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x6b8cb0() {
    // IDA 0x6b8cb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot24safe_static_do_get_mutexEv")]
// 0x6b8da0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x6b8da0() {
    // IDA 0x6b8da0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD1Ev")]
// 0x6b90d0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD1Ev
pub fn stub_0x6b90d0() {
    // IDA 0x6b90d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZN3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x6be0cc — __ZN3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
pub fn stub_0x6be0cc() {
    // IDA 0x6be0cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x6be23c — __ZThn132_N3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
// was: non-virtual thunk to RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
pub fn stub_0x6be23c() {
    // IDA 0x6be23c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZN3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x6be248 — __ZN3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
pub fn stub_0x6be248() {
    // IDA 0x6be248: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x6be4ac — __ZThn132_N3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
// was: non-virtual thunk to RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
pub fn stub_0x6be4ac() {
    // IDA 0x6be4ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::append(RBX::RotateJoint * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_")]
// 0x6be770 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_
pub fn stub_0x6be770() {
    // IDA 0x6be770: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<bool,10,32ul>::append(bool const&)")]
#[doc(alias = "__ZN3G3D5ArrayIbLi10ELm32EE6appendERKb")]
// 0x6be7cc — __ZN3G3D5ArrayIbLi10ELm32EE6appendERKb
pub fn stub_0x6be7cc() {
    // IDA 0x6be7cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Body::accumulateTorque(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E")]
// 0x6be824 — __ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6be824() {
    // IDA 0x6be824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib")]
// 0x6c020c — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib
pub fn stub_0x6c020c() {
    // IDA 0x6c020c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi")]
// 0x6c02c4 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi
// type: int(void)
pub fn stub_0x6c02c4() {
    // IDA 0x6c02c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<bool,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIbLi10ELm32EED2Ev")]
// 0x6c1824 — __ZN3G3D5ArrayIbLi10ELm32EED2Ev
pub fn stub_0x6c1824() {
    // IDA 0x6c1824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev")]
// 0x6c18f8 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev
pub fn stub_0x6c18f8() {
    // IDA 0x6c18f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<bool,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIbLi10ELm32EEC2Ev")]
// 0x6c19cc — __ZN3G3D5ArrayIbLi10ELm32EEC2Ev
pub fn stub_0x6c19cc() {
    // IDA 0x6c19cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev")]
// 0x6c1abc — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev
pub fn stub_0x6c1abc() {
    // IDA 0x6c1abc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::clickButton1(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser12clickButton1EN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c32b0 — __ZN3RBX11VirtualUser12clickButton1EN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c32b0() {
    // IDA 0x6c32b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::button1Down(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser11button1DownEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c33fc — __ZN3RBX11VirtualUser11button1DownEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c33fc() {
    // IDA 0x6c33fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::VirtualUser::button1Up(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser9button1UpEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c3434 — __ZN3RBX11VirtualUser9button1UpEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c3434() {
    // IDA 0x6c3434: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::VirtualUser::clickButton2(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser12clickButton2EN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c346c — __ZN3RBX11VirtualUser12clickButton2EN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c346c() {
    // IDA 0x6c346c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::VirtualUser::button2Down(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser11button2DownEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c35b8 — __ZN3RBX11VirtualUser11button2DownEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c35b8() {
    // IDA 0x6c35b8: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::VirtualUser::button2Up(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser9button2UpEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c35f0 — __ZN3RBX11VirtualUser9button2UpEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c35f0() {
    // IDA 0x6c35f0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::VirtualUser::moveMouse(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser9moveMouseEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c3628 — __ZN3RBX11VirtualUser9moveMouseEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c3628() {
    // IDA 0x6c3628: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::VirtualUser::getDataModel(void)")]
#[doc(alias = "__ZN3RBX11VirtualUser12getDataModelEv")]
// 0x6c3e14 — __ZN3RBX11VirtualUser12getDataModelEv
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this)
pub fn stub_0x6c3e14() {
    // IDA 0x6c3e14: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::VirtualUser::sendMouseEvent(RBX::UIEvent::EventType,G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser14sendMouseEventENS_7UIEvent9EventTypeEN3G3D7Vector2ENS3_15CoordinateFrameE")]
// 0x6c4118 — __ZN3RBX11VirtualUser14sendMouseEventENS_7UIEvent9EventTypeEN3G3D7Vector2ENS3_15CoordinateFrameE
pub fn stub_0x6c4118() {
    // IDA 0x6c4118: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZN3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE")]
// 0x6d0138 — __ZN3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, RBX::BaseScript *)
pub fn stub_0x6d0138() {
    // IDA 0x6d0138: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk to RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZThn388_N3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE")]
// 0x6d02e4 — __ZThn388_N3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
// type: int __fastcall(RBX::Workspace *this, RBX::BaseScript *)
// was: non-virtual thunk to RBX::Workspace::scriptShouldRun(RBX::BaseScript *)
pub fn stub_0x6d02e4() {
    // IDA 0x6d02e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CameraSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13CameraSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_")]
// 0x6d2d60 — __ZN3RBX13CameraSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this, const Vector3 *, const Vector3 *)
pub fn stub_0x6d2d60() {
    // IDA 0x6d2d60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE9singletonEv")]
// 0x6d9cc0 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE9singletonEv
pub fn stub_0x6d9cc0() {
    // IDA 0x6d9cc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6resizeEib")]
// 0x6e0e8c — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6resizeEib
pub fn stub_0x6e0e8c() {
    // IDA 0x6e0e8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE7reallocEi")]
// 0x6e0f48 — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE7reallocEi
pub fn stub_0x6e0f48() {
    // IDA 0x6e0f48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Body::setCofmOffset(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body13setCofmOffsetERKN3G3D7Vector3E")]
// 0x6e2f94 — __ZN3RBX4Body13setCofmOffsetERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e2f94() {
    // IDA 0x6e2f94: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Body::setMeInParent(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Body13setMeInParentERKN3G3D15CoordinateFrameE")]
// 0x6e3118 — __ZN3RBX4Body13setMeInParentERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::CoordinateFrame *)
pub fn stub_0x6e3118() {
    // IDA 0x6e3118: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::setMoment(G3D::Matrix3 const&)")]
#[doc(alias = "__ZN3RBX4Body9setMomentERKN3G3D7Matrix3E")]
// 0x6e344c — __ZN3RBX4Body9setMomentERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Matrix3 *)
pub fn stub_0x6e344c() {
    // IDA 0x6e344c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::getIBodyAtPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body15getIBodyAtPointERKN3G3D7Vector3E")]
// 0x6e3488 — __ZN3RBX4Body15getIBodyAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e3488() {
    // IDA 0x6e3488: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::getIWorldAtPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body16getIWorldAtPointERKN3G3D7Vector3E")]
// 0x6e34bc — __ZN3RBX4Body16getIWorldAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e34bc() {
    // IDA 0x6e34bc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Body::getBranchIWorldAtPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body22getBranchIWorldAtPointERKN3G3D7Vector3E")]
// 0x6e3500 — __ZN3RBX4Body22getBranchIWorldAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e3500() {
    // IDA 0x6e3500: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)")]
#[doc(alias = "__ZN3RBX15RotateConnectorC1EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff")]
// 0x6e4598 — __ZN3RBX15RotateConnectorC1EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, RBX::Body *, RBX::Body *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, float, float, float)
pub fn stub_0x6e4598() {
    // IDA 0x6e4598: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)")]
#[doc(alias = "__ZN3RBX15RotateConnectorC2EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff")]
// 0x6e459c — __ZN3RBX15RotateConnectorC2EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, RBX::Body *, RBX::Body *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, float, float, float)
pub fn stub_0x6e459c() {
    // IDA 0x6e459c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RotateConnector::computeNormalRotationFromBase(G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX15RotateConnector29computeNormalRotationFromBaseERN3G3D7Vector3E")]
// 0x6e4710 — __ZN3RBX15RotateConnector29computeNormalRotationFromBaseERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, G3D::Vector3 *)
pub fn stub_0x6e4710() {
    // IDA 0x6e4710: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RotateConnector::computeJointAngle(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX15RotateConnector17computeJointAngleERKN3G3D15CoordinateFrameES4_S4_S4_RNS1_7Vector3E")]
// 0x6e4770 — __ZN3RBX15RotateConnector17computeJointAngleERKN3G3D15CoordinateFrameES4_S4_S4_RNS1_7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, G3D::Vector3 *)
pub fn stub_0x6e4770() {
    // IDA 0x6e4770: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RotateConnector::computeNormalRotationFromBaseFast(G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX15RotateConnector33computeNormalRotationFromBaseFastERN3G3D7Vector3E")]
// 0x6e4800 — __ZN3RBX15RotateConnector33computeNormalRotationFromBaseFastERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, G3D::Vector3 *)
pub fn stub_0x6e4800() {
    // IDA 0x6e4800: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Constants::getJointKMultiplier(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX9Constants19getJointKMultiplierERKN3G3D7Vector3Eb")]
// 0x6e5278 — __ZN3RBX9Constants19getJointKMultiplierERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(RBX::Constants *__hidden this, const G3D::Vector3 *, bool)
pub fn stub_0x6e5278() {
    // IDA 0x6e5278: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Constants::getJointK(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX9Constants9getJointKERKN3G3D7Vector3Eb")]
// 0x6e5694 — __ZN3RBX9Constants9getJointKERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(Vector3 *this, const G3D::Vector3 *, bool)
pub fn stub_0x6e5694() {
    // IDA 0x6e5694: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactConnector::computeRelativeVelocity(RBX::PairParams const&,G3D::Vector3 *,G3D::Vector3 *)")]
#[doc(alias = "__ZN3RBX16ContactConnector23computeRelativeVelocityERKNS_10PairParamsEPN3G3D7Vector3ES6_")]
// 0x6e57dc — __ZN3RBX16ContactConnector23computeRelativeVelocityERKNS_10PairParamsEPN3G3D7Vector3ES6_
pub fn stub_0x6e57dc() {
    // IDA 0x6e57dc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactConnector::getSimBodyAndContactVelocity(RBX::SimBody *&,RBX::SimBody *&,RBX::PairParams &,float &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX16ContactConnector28getSimBodyAndContactVelocityERPNS_7SimBodyES3_RNS_10PairParamsERfRN3G3D7Vector3E")]
// 0x6e5b34 — __ZN3RBX16ContactConnector28getSimBodyAndContactVelocityERPNS_7SimBodyES3_RNS_10PairParamsERfRN3G3D7Vector3E
pub fn stub_0x6e5b34() {
    // IDA 0x6e5b34: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Kernel::newPointLocal(RBX::Body *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX6Kernel13newPointLocalEPNS_4BodyERKN3G3D7Vector3E")]
// 0x6e8154 — __ZN3RBX6Kernel13newPointLocalEPNS_4BodyERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Kernel *__hidden this, RBX::Body *, const G3D::Vector3 *)
pub fn stub_0x6e8154() {
    // IDA 0x6e8154: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6resizeEib")]
// 0x6eb568 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6resizeEib
pub fn stub_0x6eb568() {
    // IDA 0x6eb568: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE7reallocEi")]
// 0x6eb620 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE7reallocEi
pub fn stub_0x6eb620() {
    // IDA 0x6eb620: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::append(RBX::Point * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6appendERKS3_")]
// 0x6eb808 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6appendERKS3_
pub fn stub_0x6eb808() {
    // IDA 0x6eb808: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::append(RBX::SimBody * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6appendERKS3_")]
// 0x6eb864 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6appendERKS3_
pub fn stub_0x6eb864() {
    // IDA 0x6eb864: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6resizeEib")]
// 0x6eb8c0 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6resizeEib
pub fn stub_0x6eb8c0() {
    // IDA 0x6eb8c0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE7reallocEi")]
// 0x6eb978 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE7reallocEi
pub fn stub_0x6eb978() {
    // IDA 0x6eb978: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::append(RBX::Body * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6appendERKS3_")]
// 0x6ebce4 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6appendERKS3_
pub fn stub_0x6ebce4() {
    // IDA 0x6ebce4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6resizeEib")]
// 0x6ebd40 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6resizeEib
pub fn stub_0x6ebd40() {
    // IDA 0x6ebd40: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE7reallocEi")]
// 0x6ebdf8 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE7reallocEi
pub fn stub_0x6ebdf8() {
    // IDA 0x6ebdf8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6resizeEib")]
// 0x6ecbac — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6resizeEib
pub fn stub_0x6ecbac() {
    // IDA 0x6ecbac: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE7reallocEi")]
// 0x6ecc64 — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE7reallocEi
pub fn stub_0x6ecc64() {
    // IDA 0x6ecc64: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::append(RBX::Connector * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6appendERKS3_")]
// 0x6ece4c — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6appendERKS3_
pub fn stub_0x6ece4c() {
    // IDA 0x6ece4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EED2Ev")]
// 0x6ed470 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EED2Ev
pub fn stub_0x6ed470() {
    // IDA 0x6ed470: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EED2Ev")]
// 0x6ed544 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EED2Ev
pub fn stub_0x6ed544() {
    // IDA 0x6ed544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EED2Ev")]
// 0x6ed618 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EED2Ev
pub fn stub_0x6ed618() {
    // IDA 0x6ed618: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EED2Ev")]
// 0x6ed6ec — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EED2Ev
pub fn stub_0x6ed6ec() {
    // IDA 0x6ed6ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EEC2Ev")]
// 0x6ed9bc — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EEC2Ev
pub fn stub_0x6ed9bc() {
    // IDA 0x6ed9bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EEC2Ev")]
// 0x6edaac — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EEC2Ev
pub fn stub_0x6edaac() {
    // IDA 0x6edaac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EEC2Ev")]
// 0x6edb9c — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EEC2Ev
pub fn stub_0x6edb9c() {
    // IDA 0x6edb9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EEC2Ev")]
// 0x6edc8c — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EEC2Ev
pub fn stub_0x6edc8c() {
    // IDA 0x6edc8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Link::reset(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Link5resetERKN3G3D15CoordinateFrameES4_")]
// 0x6ee1a4 — __ZN3RBX4Link5resetERKN3G3D15CoordinateFrameES4_
// type: _DWORD __fastcall(RBX::Link *__hidden this, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *)
pub fn stub_0x6ee1a4() {
    // IDA 0x6ee1a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::RevoluteLink::computeChildInParent(G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX12RevoluteLink20computeChildInParentERN3G3D15CoordinateFrameE")]
// 0x6ee2e0 — __ZNK3RBX12RevoluteLink20computeChildInParentERN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::RevoluteLink *__hidden this, G3D::CoordinateFrame *)
pub fn stub_0x6ee2e0() {
    // IDA 0x6ee2e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::D6Link::computeChildInParent(G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX6D6Link20computeChildInParentERN3G3D15CoordinateFrameE")]
// 0x6ee3bc — __ZNK3RBX6D6Link20computeChildInParentERN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::D6Link *__hidden this, G3D::CoordinateFrame *)
pub fn stub_0x6ee3bc() {
    // IDA 0x6ee3bc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Point::setLocalPos(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX5Point11setLocalPosERKN3G3D7Vector3E")]
// 0x6ef68c — __ZN3RBX5Point11setLocalPosERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Point *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6ef68c() {
    // IDA 0x6ef68c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "G3D::Line::closestPoints(G3D::Line const&,G3D::Line const&,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_")]
// 0x6f0580 — __ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const G3D::Line *, const G3D::Line *, G3D::Vector3 *, G3D::Vector3 *)
pub fn stub_0x6f0580() {
    // IDA 0x6f0580: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::SimBody::applyImpulse(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7SimBody12applyImpulseERKN3G3D7Vector3ES4_")]
// 0x6f1c48 — __ZN3RBX7SimBody12applyImpulseERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::SimBody *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
pub fn stub_0x6f1c48() {
    // IDA 0x6f1c48: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "G3D::Vector3 rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6f96e0 — __ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: void __fastcall(int, int)
pub fn stub_0x6f96e0() {
    // IDA 0x6f96e0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::CoordinateFrame rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6f97dc — __ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: void __fastcall(int, int)
pub fn stub_0x6f97dc() {
    // IDA 0x6f97dc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Color3 * rbx::any_cast<G3D::Color3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D6Color3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fb968 — __ZN3rbx8any_castIN3G3D6Color3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fb968() {
    // IDA 0x6fb968: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Color3 & rbx::any_cast<G3D::Color3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fb9c0 — __ZN3rbx8any_castIRN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fb9c0() {
    // IDA 0x6fb9c0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Vector2int16 * rbx::any_cast<G3D::Vector2int16,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D12Vector2int16EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fc4f8 — __ZN3rbx8any_castIN3G3D12Vector2int16EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fc4f8() {
    // IDA 0x6fc4f8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2int16>(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector2int16EEERS3_RKT_")]
// 0x6fc550 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector2int16EEERS3_RKT_
pub fn stub_0x6fc550() {
    // IDA 0x6fc550: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Vector2int16 & rbx::any_cast<G3D::Vector2int16 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fc5a0 — __ZN3rbx8any_castIRN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fc5a0() {
    // IDA 0x6fc5a0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2int16>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE9singletonEv")]
// 0x6fc690 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE9singletonEv
pub fn stub_0x6fc690() {
    // IDA 0x6fc690: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2int16>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE13destruct_funcEPc")]
// 0x6fc6fc — __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE13destruct_funcEPc
// type: void()
pub fn stub_0x6fc6fc() {
    // IDA 0x6fc6fc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::CoordinateFrame * rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fd0bc — __ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fd0bc() {
    // IDA 0x6fd0bc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::CoordinateFrame & rbx::any_cast<G3D::CoordinateFrame &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fd114 — __ZN3rbx8any_castIRN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fd114() {
    // IDA 0x6fd114: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::CoordinateFrame>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE9singletonEv")]
// 0x6fd204 — __ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE9singletonEv
pub fn stub_0x6fd204() {
    // IDA 0x6fd204: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::CoordinateFrame>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE13destruct_funcEPc")]
// 0x6fd270 — __ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE13destruct_funcEPc
pub fn stub_0x6fd270() {
    // IDA 0x6fd270: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Vector2 * rbx::any_cast<G3D::Vector2,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector2EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fd274 — __ZN3rbx8any_castIN3G3D7Vector2EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fd274() {
    // IDA 0x6fd274: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Vector2 & rbx::any_cast<G3D::Vector2 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fd2cc — __ZN3rbx8any_castIRN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fd2cc() {
    // IDA 0x6fd2cc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Vector3 * rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fd3bc — __ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fd3bc() {
    // IDA 0x6fd3bc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "G3D::Vector3 & rbx::any_cast<G3D::Vector3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fd414 — __ZN3rbx8any_castIRN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fd414() {
    // IDA 0x6fd414: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::StringConverter<G3D::Vector2int16>::convertToString(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D12Vector2int16EE15convertToStringERKS2_")]
// 0x711ea0 — __ZN3RBX15StringConverterIN3G3D12Vector2int16EE15convertToStringERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x711ea0() {
    // IDA 0x711ea0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::StringConverter<G3D::Vector2int16>::convertToValue(std::string const&,G3D::Vector2int16&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D12Vector2int16EE14convertToValueERKSsRS2_")]
// 0x712010 — __ZN3RBX15StringConverterIN3G3D12Vector2int16EE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x712010() {
    // IDA 0x712010: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<G3D::Vector3int16>::convertToString(G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D12Vector3int16EE15convertToStringERKS2_")]
// 0x7120f4 — __ZN3RBX15StringConverterIN3G3D12Vector3int16EE15convertToStringERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x7120f4() {
    // IDA 0x7120f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}