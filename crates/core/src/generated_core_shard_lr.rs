//! core shard lr — 150 core stubs EA-sorted, next uncovered fallback after shard lq (0x61f5cc..0x7120f4, lowest EA first).
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback, EA-sorted, next 150 uncovered, lowest EA first, rbx_core::SharedPtr not boost) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::SelectionPointLasso::setPoint(G3D::Vector3)")]
#[doc(alias = "__ZN3RBX19SelectionPointLasso8setPointEN3G3D7Vector3E")]
// 0x61f5cc — __ZN3RBX19SelectionPointLasso8setPointEN3G3D7Vector3E
pub fn stub_0x61f5cc() -> ! {
    todo!("0x61f5cc __ZN3RBX19SelectionPointLasso8setPointEN3G3D7Vector3E")
}

#[doc(alias = "RBX::SelectionPointLasso::getPosition(G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX19SelectionPointLasso11getPositionERN3G3D7Vector3E")]
// 0x6206f0 — __ZNK3RBX19SelectionPointLasso11getPositionERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::SelectionPointLasso *__hidden this, Vector3 *)
pub fn stub_0x6206f0() -> ! {
    todo!("0x6206f0 __ZNK3RBX19SelectionPointLasso11getPositionERN3G3D7Vector3E")
}

#[doc(alias = "RBX::SkateboardPlatform::applySpecificImpulse(G3D::Vector3)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3E")]
// 0x6271fc — __ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, Vector3)
pub fn stub_0x6271fc() -> ! {
    todo!("0x6271fc __ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3E")
}

#[doc(alias = "RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x6291c4 — __ZN3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
pub fn stub_0x6291c4() -> ! {
    todo!("0x6291c4 __ZN3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")
}

#[doc(alias = "non-virtual thunk toRBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZThn132_N3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x629334 — __ZThn132_N3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
// was: `non-virtual thunk to'RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
pub fn stub_0x629334() -> ! {
    todo!("0x629334 __ZThn132_N3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")
}

#[doc(alias = "RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x629340 — __ZN3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
pub fn stub_0x629340() -> ! {
    todo!("0x629340 __ZN3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")
}

#[doc(alias = "non-virtual thunk toRBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZThn132_N3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x6295a4 — __ZThn132_N3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
// was: `non-virtual thunk to'RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
pub fn stub_0x6295a4() -> ! {
    todo!("0x6295a4 __ZThn132_N3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")
}

#[doc(alias = "RBX::SkateboardPlatform::applySpecificImpulse(G3D::Vector3,G3D::Vector3)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3ES2_")]
// 0x6295bc — __ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3ES2_
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, Vector3, Vector3)
pub fn stub_0x6295bc() -> ! {
    todo!("0x6295bc __ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3ES2_")
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::append(RBX::SkateboardPlatform::Wheel const&)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6appendERKS3_")]
// 0x62a284 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6appendERKS3_
// type: int(void)
pub fn stub_0x62a284() -> ! {
    todo!("0x62a284 __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6appendERKS3_")
}

#[doc(alias = "RBX::Body::accumulateForceAtBranchCofm(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E")]
// 0x62a2f0 — __ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x62a2f0() -> ! {
    todo!("0x62a2f0 __ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E")
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6resizeEib")]
// 0x62beec — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6resizeEib
// type: int(void)
pub fn stub_0x62beec() -> ! {
    todo!("0x62beec __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE7reallocEi")]
// 0x62bfb0 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE7reallocEi
// type: int(void)
pub fn stub_0x62bfb0() -> ! {
    todo!("0x62bfb0 __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE7reallocEi")
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EED2Ev")]
// 0x62e69c — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EED2Ev
pub fn stub_0x62e69c() -> ! {
    todo!("0x62e69c __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EEC2Ev")]
// 0x62e770 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EEC2Ev
pub fn stub_0x62e770() -> ! {
    todo!("0x62e770 __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EEC2Ev")
}

#[doc(alias = "RBX::Velocity::rotateBy(G3D::Matrix3 const&)const")]
#[doc(alias = "__ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E")]
// 0x633e44 — __ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E
// type: int __fastcall(int result, __int32 *, int)
pub fn stub_0x633e44() -> ! {
    todo!("0x633e44 __ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E")
}

#[doc(alias = "RBX::Smoke::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX5Smoke8setColorEN3G3D6Color3E")]
// 0x637264 — __ZN3RBX5Smoke8setColorEN3G3D6Color3E
pub fn stub_0x637264() -> ! {
    todo!("0x637264 __ZN3RBX5Smoke8setColorEN3G3D6Color3E")
}

#[doc(alias = "RBX::Sparkles::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX8Sparkles8setColorEN3G3D6Color3E")]
// 0x63c1a4 — __ZN3RBX8Sparkles8setColorEN3G3D6Color3E
pub fn stub_0x63c1a4() -> ! {
    todo!("0x63c1a4 __ZN3RBX8Sparkles8setColorEN3G3D6Color3E")
}

#[doc(alias = "RBX::Sparkles::setLegacyColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX8Sparkles14setLegacyColorEN3G3D6Color3E")]
// 0x63c248 — __ZN3RBX8Sparkles14setLegacyColorEN3G3D6Color3E
pub fn stub_0x63c248() -> ! {
    todo!("0x63c248 __ZN3RBX8Sparkles14setLegacyColorEN3G3D6Color3E")
}

#[doc(alias = "RBX::TextBox::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX7TextBox13setTextColor3EN3G3D6Color3E")]
// 0x665fec — __ZN3RBX7TextBox13setTextColor3EN3G3D6Color3E
pub fn stub_0x665fec() -> ! {
    todo!("0x665fec __ZN3RBX7TextBox13setTextColor3EN3G3D6Color3E")
}

#[doc(alias = "RBX::TextBox::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX7TextBox19setTextStrokeColor3EN3G3D6Color3E")]
// 0x6664e4 — __ZN3RBX7TextBox19setTextStrokeColor3EN3G3D6Color3E
pub fn stub_0x6664e4() -> ! {
    todo!("0x6664e4 __ZN3RBX7TextBox19setTextStrokeColor3EN3G3D6Color3E")
}

#[doc(alias = "RBX::TextBox::getPosInString(G3D::Vector2)const")]
#[doc(alias = "__ZNK3RBX7TextBox14getPosInStringEN3G3D7Vector2E")]
// 0x6665ec — __ZNK3RBX7TextBox14getPosInStringEN3G3D7Vector2E
pub fn stub_0x6665ec() -> ! {
    todo!("0x6665ec __ZNK3RBX7TextBox14getPosInStringEN3G3D7Vector2E")
}

#[doc(alias = "RBX::TextBox::getCursorPos(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX7TextBox12getCursorPosEN3G3D7Vector2E")]
// 0x667500 — __ZN3RBX7TextBox12getCursorPosEN3G3D7Vector2E
pub fn stub_0x667500() -> ! {
    todo!("0x667500 __ZN3RBX7TextBox12getCursorPosEN3G3D7Vector2E")
}

#[doc(alias = "RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")]
// 0x668d2c — __ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE
// type: int(void)
pub fn stub_0x668d2c() -> ! {
    todo!("0x668d2c __ZNK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZThn96_NK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")]
// 0x668efc — __ZThn96_NK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE
// was: `non-virtual thunk to'RBX::GuiBase2d::isVisible(G3D::Rect2D const&)const
pub fn stub_0x668efc() -> ! {
    todo!("0x668efc __ZThn96_NK3RBX9GuiBase2d9isVisibleERKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::GuiTextButton::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX13GuiTextButton13setTextColor3EN3G3D6Color3E")]
// 0x673288 — __ZN3RBX13GuiTextButton13setTextColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x673288() -> ! {
    todo!("0x673288 __ZN3RBX13GuiTextButton13setTextColor3EN3G3D6Color3E")
}

#[doc(alias = "RBX::GuiTextButton::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX13GuiTextButton19setTextStrokeColor3EN3G3D6Color3E")]
// 0x673780 — __ZN3RBX13GuiTextButton19setTextStrokeColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x673780() -> ! {
    todo!("0x673780 __ZN3RBX13GuiTextButton19setTextStrokeColor3EN3G3D6Color3E")
}

#[doc(alias = "RBX::TextLabel::setTextColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX9TextLabel13setTextColor3EN3G3D6Color3E")]
// 0x678814 — __ZN3RBX9TextLabel13setTextColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x678814() -> ! {
    todo!("0x678814 __ZN3RBX9TextLabel13setTextColor3EN3G3D6Color3E")
}

#[doc(alias = "RBX::TextLabel::setTextStrokeColor3(G3D::Color3)")]
#[doc(alias = "__ZN3RBX9TextLabel19setTextStrokeColor3EN3G3D6Color3E")]
// 0x678d0c — __ZN3RBX9TextLabel19setTextStrokeColor3EN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x678d0c() -> ! {
    todo!("0x678d0c __ZN3RBX9TextLabel19setTextStrokeColor3EN3G3D6Color3E")
}

#[doc(alias = "RBX::Tool::setGrip(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Tool7setGripERKN3G3D15CoordinateFrameE")]
// 0x67e7a0 — __ZN3RBX4Tool7setGripERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::CoordinateFrame *)
pub fn stub_0x67e7a0() -> ! {
    todo!("0x67e7a0 __ZN3RBX4Tool7setGripERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Tool::setGripPos(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool10setGripPosERKN3G3D7Vector3E")]
// 0x67e8d0 — __ZN3RBX4Tool10setGripPosERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67e8d0() -> ! {
    todo!("0x67e8d0 __ZN3RBX4Tool10setGripPosERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Tool::setGripForward(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool14setGripForwardERKN3G3D7Vector3E")]
// 0x67e940 — __ZN3RBX4Tool14setGripForwardERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67e940() -> ! {
    todo!("0x67e940 __ZN3RBX4Tool14setGripForwardERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Tool::setGripUp(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool9setGripUpERKN3G3D7Vector3E")]
// 0x67ea9c — __ZN3RBX4Tool9setGripUpERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67ea9c() -> ! {
    todo!("0x67ea9c __ZN3RBX4Tool9setGripUpERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Tool::setGripRight(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool12setGripRightERKN3G3D7Vector3E")]
// 0x67ebd0 — __ZN3RBX4Tool12setGripRightERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
pub fn stub_0x67ebd0() -> ! {
    todo!("0x67ebd0 __ZN3RBX4Tool12setGripRightERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::cleanUpZeroColumn(G3D::Matrix3 &)")]
#[doc(alias = "__ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E")]
// 0x6827bc — __ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX *__hidden this, G3D::Matrix3 *)
pub fn stub_0x6827bc() -> ! {
    todo!("0x6827bc __ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev")]
// 0x6a407c — __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x6a407c() -> ! {
    todo!("0x6a407c __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")]
// 0x6a4334 — __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a4334() -> ! {
    todo!("0x6a4334 __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")]
// 0x6a4448 — __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a4448() -> ! {
    todo!("0x6a4448 __ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")]
// 0x6a4584 — __ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev
pub fn stub_0x6a4584() -> ! {
    todo!("0x6a4584 __ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")]
// 0x6a4694 — __ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev
pub fn stub_0x6a4694() -> ! {
    todo!("0x6a4694 __ZThn32_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")]
// 0x6a47cc — __ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev
pub fn stub_0x6a47cc() -> ! {
    todo!("0x6a47cc __ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")]
// 0x6a48dc — __ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev
pub fn stub_0x6a48dc() -> ! {
    todo!("0x6a48dc __ZThn36_N3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEED0Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev")]
// 0x6a5798 — __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev
// type: int __fastcall(int)
pub fn stub_0x6a5798() -> ! {
    todo!("0x6a5798 __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")]
// 0x6a5a60 — __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a5a60() -> ! {
    todo!("0x6a5a60 __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")]
// 0x6a5b74 — __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a5b74() -> ! {
    todo!("0x6a5b74 __ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")]
// 0x6a5cb0 — __ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev
// type: void __fastcall(_QWORD *, int, int, int)
pub fn stub_0x6a5cb0() -> ! {
    todo!("0x6a5cb0 __ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")]
// 0x6a5dc4 — __ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev
pub fn stub_0x6a5dc4() -> ! {
    todo!("0x6a5dc4 __ZThn32_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")]
// 0x6a5f00 — __ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev
pub fn stub_0x6a5f00() -> ! {
    todo!("0x6a5f00 __ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")]
// 0x6a6014 — __ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev
// type: void __fastcall(int, int, int, int)
pub fn stub_0x6a6014() -> ! {
    todo!("0x6a6014 __ZThn36_N3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEED0Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev")]
// 0x6a6ed4 — __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0x6a6ed4() -> ! {
    todo!("0x6a6ed4 __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")]
// 0x6a7178 — __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a7178() -> ! {
    todo!("0x6a7178 __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")
}

#[doc(alias = "__ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")]
// 0x6a728c — __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x6a728c() -> ! {
    todo!("0x6a728c __ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")]
// 0x6a73c8 — __ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev
pub fn stub_0x6a73c8() -> ! {
    todo!("0x6a73c8 __ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")]
// 0x6a74d8 — __ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev
pub fn stub_0x6a74d8() -> ! {
    todo!("0x6a74d8 __ZThn32_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")]
// 0x6a7610 — __ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev
pub fn stub_0x6a7610() -> ! {
    todo!("0x6a7610 __ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")]
// 0x6a7720 — __ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev
pub fn stub_0x6a7720() -> ! {
    todo!("0x6a7720 __ZThn36_N3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE8on_errorERSt9exception")]
// 0x6b7cdc — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE8on_errorERSt9exception
pub fn stub_0x6b7cdc() -> ! {
    todo!("0x6b7cdc __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot10disconnectEv")]
// 0x6b8ac4 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot10disconnectEv
pub fn stub_0x6b8ac4() -> ! {
    todo!("0x6b8ac4 __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::remove(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6removeEPNS5_4slotE")]
// 0x6b8cb0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x6b8cb0() -> ! {
    todo!("0x6b8cb0 __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6removeEPNS5_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot24safe_static_do_get_mutexEv")]
// 0x6b8da0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x6b8da0() -> ! {
    todo!("0x6b8da0 __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD1Ev")]
// 0x6b90d0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD1Ev
pub fn stub_0x6b90d0() -> ! {
    todo!("0x6b90d0 __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD1Ev")
}

#[doc(alias = "RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZN3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x6be0cc — __ZN3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
pub fn stub_0x6be0cc() -> ! {
    todo!("0x6be0cc __ZN3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_")
}

#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_")]
// 0x6be23c — __ZThn132_N3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
// was: `non-virtual thunk to'RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
pub fn stub_0x6be23c() -> ! {
    todo!("0x6be23c __ZThn132_N3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_")
}

#[doc(alias = "RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZN3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x6be248 — __ZN3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
pub fn stub_0x6be248() -> ! {
    todo!("0x6be248 __ZN3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")
}

#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
// 0x6be4ac — __ZThn132_N3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
// was: `non-virtual thunk to'RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
pub fn stub_0x6be4ac() -> ! {
    todo!("0x6be4ac __ZThn132_N3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::append(RBX::RotateJoint * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_")]
// 0x6be770 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_
pub fn stub_0x6be770() -> ! {
    todo!("0x6be770 __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_")
}

#[doc(alias = "G3D::Array<bool,10,32ul>::append(bool const&)")]
#[doc(alias = "__ZN3G3D5ArrayIbLi10ELm32EE6appendERKb")]
// 0x6be7cc — __ZN3G3D5ArrayIbLi10ELm32EE6appendERKb
pub fn stub_0x6be7cc() -> ! {
    todo!("0x6be7cc __ZN3G3D5ArrayIbLi10ELm32EE6appendERKb")
}

#[doc(alias = "RBX::Body::accumulateTorque(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E")]
// 0x6be824 — __ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6be824() -> ! {
    todo!("0x6be824 __ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E")
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib")]
// 0x6c020c — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib
pub fn stub_0x6c020c() -> ! {
    todo!("0x6c020c __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi")]
// 0x6c02c4 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi
// type: int(void)
pub fn stub_0x6c02c4() -> ! {
    todo!("0x6c02c4 __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi")
}

#[doc(alias = "G3D::Array<bool,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIbLi10ELm32EED2Ev")]
// 0x6c1824 — __ZN3G3D5ArrayIbLi10ELm32EED2Ev
pub fn stub_0x6c1824() -> ! {
    todo!("0x6c1824 __ZN3G3D5ArrayIbLi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev")]
// 0x6c18f8 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev
pub fn stub_0x6c18f8() -> ! {
    todo!("0x6c18f8 __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<bool,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIbLi10ELm32EEC2Ev")]
// 0x6c19cc — __ZN3G3D5ArrayIbLi10ELm32EEC2Ev
pub fn stub_0x6c19cc() -> ! {
    todo!("0x6c19cc __ZN3G3D5ArrayIbLi10ELm32EEC2Ev")
}

#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev")]
// 0x6c1abc — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev
pub fn stub_0x6c1abc() -> ! {
    todo!("0x6c1abc __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev")
}

#[doc(alias = "RBX::VirtualUser::clickButton1(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser12clickButton1EN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c32b0 — __ZN3RBX11VirtualUser12clickButton1EN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c32b0() -> ! {
    todo!("0x6c32b0 __ZN3RBX11VirtualUser12clickButton1EN3G3D7Vector2ENS1_15CoordinateFrameE")
}

#[doc(alias = "RBX::VirtualUser::button1Down(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser11button1DownEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c33fc — __ZN3RBX11VirtualUser11button1DownEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c33fc() -> ! {
    todo!("0x6c33fc __ZN3RBX11VirtualUser11button1DownEN3G3D7Vector2ENS1_15CoordinateFrameE")
}

#[doc(alias = "RBX::VirtualUser::button1Up(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser9button1UpEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c3434 — __ZN3RBX11VirtualUser9button1UpEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c3434() -> ! {
    todo!("0x6c3434 __ZN3RBX11VirtualUser9button1UpEN3G3D7Vector2ENS1_15CoordinateFrameE")
}

#[doc(alias = "RBX::VirtualUser::clickButton2(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser12clickButton2EN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c346c — __ZN3RBX11VirtualUser12clickButton2EN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c346c() -> ! {
    todo!("0x6c346c __ZN3RBX11VirtualUser12clickButton2EN3G3D7Vector2ENS1_15CoordinateFrameE")
}

#[doc(alias = "RBX::VirtualUser::button2Down(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser11button2DownEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c35b8 — __ZN3RBX11VirtualUser11button2DownEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c35b8() -> ! {
    todo!("0x6c35b8 __ZN3RBX11VirtualUser11button2DownEN3G3D7Vector2ENS1_15CoordinateFrameE")
}

#[doc(alias = "RBX::VirtualUser::button2Up(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser9button2UpEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c35f0 — __ZN3RBX11VirtualUser9button2UpEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c35f0() -> ! {
    todo!("0x6c35f0 __ZN3RBX11VirtualUser9button2UpEN3G3D7Vector2ENS1_15CoordinateFrameE")
}

#[doc(alias = "RBX::VirtualUser::moveMouse(G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser9moveMouseEN3G3D7Vector2ENS1_15CoordinateFrameE")]
// 0x6c3628 — __ZN3RBX11VirtualUser9moveMouseEN3G3D7Vector2ENS1_15CoordinateFrameE
pub fn stub_0x6c3628() -> ! {
    todo!("0x6c3628 __ZN3RBX11VirtualUser9moveMouseEN3G3D7Vector2ENS1_15CoordinateFrameE")
}

#[doc(alias = "RBX::VirtualUser::getDataModel(void)")]
#[doc(alias = "__ZN3RBX11VirtualUser12getDataModelEv")]
// 0x6c3e14 — __ZN3RBX11VirtualUser12getDataModelEv
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this)
pub fn stub_0x6c3e14() -> ! {
    todo!("0x6c3e14 __ZN3RBX11VirtualUser12getDataModelEv")
}

#[doc(alias = "RBX::VirtualUser::sendMouseEvent(RBX::UIEvent::EventType,G3D::Vector2,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX11VirtualUser14sendMouseEventENS_7UIEvent9EventTypeEN3G3D7Vector2ENS3_15CoordinateFrameE")]
// 0x6c4118 — __ZN3RBX11VirtualUser14sendMouseEventENS_7UIEvent9EventTypeEN3G3D7Vector2ENS3_15CoordinateFrameE
pub fn stub_0x6c4118() -> ! {
    todo!("0x6c4118 __ZN3RBX11VirtualUser14sendMouseEventENS_7UIEvent9EventTypeEN3G3D7Vector2ENS3_15CoordinateFrameE")
}

#[doc(alias = "RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZN3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE")]
// 0x6d0138 — __ZN3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, RBX::BaseScript *)
pub fn stub_0x6d0138() -> ! {
    todo!("0x6d0138 __ZN3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZThn388_N3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE")]
// 0x6d02e4 — __ZThn388_N3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
// type: int __fastcall(RBX::Workspace *this, RBX::BaseScript *)
// was: `non-virtual thunk to'RBX::Workspace::scriptShouldRun(RBX::BaseScript *)
pub fn stub_0x6d02e4() -> ! {
    todo!("0x6d02e4 __ZThn388_N3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE")
}

#[doc(alias = "RBX::CameraSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX13CameraSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_")]
// 0x6d2d60 — __ZN3RBX13CameraSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this, const Vector3 *, const Vector3 *)
pub fn stub_0x6d2d60() -> ! {
    todo!("0x6d2d60 __ZN3RBX13CameraSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_")
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE9singletonEv")]
// 0x6d9cc0 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE9singletonEv
pub fn stub_0x6d9cc0() -> ! {
    todo!("0x6d9cc0 __ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE9singletonEv")
}

#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6resizeEib")]
// 0x6e0e8c — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6resizeEib
pub fn stub_0x6e0e8c() -> ! {
    todo!("0x6e0e8c __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE7reallocEi")]
// 0x6e0f48 — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE7reallocEi
pub fn stub_0x6e0f48() -> ! {
    todo!("0x6e0f48 __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE7reallocEi")
}

#[doc(alias = "RBX::Body::setCofmOffset(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body13setCofmOffsetERKN3G3D7Vector3E")]
// 0x6e2f94 — __ZN3RBX4Body13setCofmOffsetERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e2f94() -> ! {
    todo!("0x6e2f94 __ZN3RBX4Body13setCofmOffsetERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Body::setMeInParent(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Body13setMeInParentERKN3G3D15CoordinateFrameE")]
// 0x6e3118 — __ZN3RBX4Body13setMeInParentERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::CoordinateFrame *)
pub fn stub_0x6e3118() -> ! {
    todo!("0x6e3118 __ZN3RBX4Body13setMeInParentERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Body::setMoment(G3D::Matrix3 const&)")]
#[doc(alias = "__ZN3RBX4Body9setMomentERKN3G3D7Matrix3E")]
// 0x6e344c — __ZN3RBX4Body9setMomentERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Matrix3 *)
pub fn stub_0x6e344c() -> ! {
    todo!("0x6e344c __ZN3RBX4Body9setMomentERKN3G3D7Matrix3E")
}

#[doc(alias = "RBX::Body::getIBodyAtPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body15getIBodyAtPointERKN3G3D7Vector3E")]
// 0x6e3488 — __ZN3RBX4Body15getIBodyAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e3488() -> ! {
    todo!("0x6e3488 __ZN3RBX4Body15getIBodyAtPointERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Body::getIWorldAtPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body16getIWorldAtPointERKN3G3D7Vector3E")]
// 0x6e34bc — __ZN3RBX4Body16getIWorldAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e34bc() -> ! {
    todo!("0x6e34bc __ZN3RBX4Body16getIWorldAtPointERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Body::getBranchIWorldAtPoint(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body22getBranchIWorldAtPointERKN3G3D7Vector3E")]
// 0x6e3500 — __ZN3RBX4Body22getBranchIWorldAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6e3500() -> ! {
    todo!("0x6e3500 __ZN3RBX4Body22getBranchIWorldAtPointERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)")]
#[doc(alias = "__ZN3RBX15RotateConnectorC1EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff")]
// 0x6e4598 — __ZN3RBX15RotateConnectorC1EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, RBX::Body *, RBX::Body *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, float, float, float)
pub fn stub_0x6e4598() -> ! {
    todo!("0x6e4598 __ZN3RBX15RotateConnectorC1EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff")
}

#[doc(alias = "RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)")]
#[doc(alias = "__ZN3RBX15RotateConnectorC2EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff")]
// 0x6e459c — __ZN3RBX15RotateConnectorC2EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, RBX::Body *, RBX::Body *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, float, float, float)
pub fn stub_0x6e459c() -> ! {
    todo!("0x6e459c __ZN3RBX15RotateConnectorC2EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff")
}

#[doc(alias = "RBX::RotateConnector::computeNormalRotationFromBase(G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX15RotateConnector29computeNormalRotationFromBaseERN3G3D7Vector3E")]
// 0x6e4710 — __ZN3RBX15RotateConnector29computeNormalRotationFromBaseERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, G3D::Vector3 *)
pub fn stub_0x6e4710() -> ! {
    todo!("0x6e4710 __ZN3RBX15RotateConnector29computeNormalRotationFromBaseERN3G3D7Vector3E")
}

#[doc(alias = "RBX::RotateConnector::computeJointAngle(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX15RotateConnector17computeJointAngleERKN3G3D15CoordinateFrameES4_S4_S4_RNS1_7Vector3E")]
// 0x6e4770 — __ZN3RBX15RotateConnector17computeJointAngleERKN3G3D15CoordinateFrameES4_S4_S4_RNS1_7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, G3D::Vector3 *)
pub fn stub_0x6e4770() -> ! {
    todo!("0x6e4770 __ZN3RBX15RotateConnector17computeJointAngleERKN3G3D15CoordinateFrameES4_S4_S4_RNS1_7Vector3E")
}

#[doc(alias = "RBX::RotateConnector::computeNormalRotationFromBaseFast(G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX15RotateConnector33computeNormalRotationFromBaseFastERN3G3D7Vector3E")]
// 0x6e4800 — __ZN3RBX15RotateConnector33computeNormalRotationFromBaseFastERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, G3D::Vector3 *)
pub fn stub_0x6e4800() -> ! {
    todo!("0x6e4800 __ZN3RBX15RotateConnector33computeNormalRotationFromBaseFastERN3G3D7Vector3E")
}

#[doc(alias = "RBX::Constants::getJointKMultiplier(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX9Constants19getJointKMultiplierERKN3G3D7Vector3Eb")]
// 0x6e5278 — __ZN3RBX9Constants19getJointKMultiplierERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(RBX::Constants *__hidden this, const G3D::Vector3 *, bool)
pub fn stub_0x6e5278() -> ! {
    todo!("0x6e5278 __ZN3RBX9Constants19getJointKMultiplierERKN3G3D7Vector3Eb")
}

#[doc(alias = "RBX::Constants::getJointK(G3D::Vector3 const&,bool)")]
#[doc(alias = "__ZN3RBX9Constants9getJointKERKN3G3D7Vector3Eb")]
// 0x6e5694 — __ZN3RBX9Constants9getJointKERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(Vector3 *this, const G3D::Vector3 *, bool)
pub fn stub_0x6e5694() -> ! {
    todo!("0x6e5694 __ZN3RBX9Constants9getJointKERKN3G3D7Vector3Eb")
}

#[doc(alias = "RBX::ContactConnector::computeRelativeVelocity(RBX::PairParams const&,G3D::Vector3 *,G3D::Vector3 *)")]
#[doc(alias = "__ZN3RBX16ContactConnector23computeRelativeVelocityERKNS_10PairParamsEPN3G3D7Vector3ES6_")]
// 0x6e57dc — __ZN3RBX16ContactConnector23computeRelativeVelocityERKNS_10PairParamsEPN3G3D7Vector3ES6_
pub fn stub_0x6e57dc() -> ! {
    todo!("0x6e57dc __ZN3RBX16ContactConnector23computeRelativeVelocityERKNS_10PairParamsEPN3G3D7Vector3ES6_")
}

#[doc(alias = "RBX::ContactConnector::getSimBodyAndContactVelocity(RBX::SimBody *&,RBX::SimBody *&,RBX::PairParams &,float &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX16ContactConnector28getSimBodyAndContactVelocityERPNS_7SimBodyES3_RNS_10PairParamsERfRN3G3D7Vector3E")]
// 0x6e5b34 — __ZN3RBX16ContactConnector28getSimBodyAndContactVelocityERPNS_7SimBodyES3_RNS_10PairParamsERfRN3G3D7Vector3E
pub fn stub_0x6e5b34() -> ! {
    todo!("0x6e5b34 __ZN3RBX16ContactConnector28getSimBodyAndContactVelocityERPNS_7SimBodyES3_RNS_10PairParamsERfRN3G3D7Vector3E")
}

#[doc(alias = "RBX::Kernel::newPointLocal(RBX::Body *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX6Kernel13newPointLocalEPNS_4BodyERKN3G3D7Vector3E")]
// 0x6e8154 — __ZN3RBX6Kernel13newPointLocalEPNS_4BodyERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Kernel *__hidden this, RBX::Body *, const G3D::Vector3 *)
pub fn stub_0x6e8154() -> ! {
    todo!("0x6e8154 __ZN3RBX6Kernel13newPointLocalEPNS_4BodyERKN3G3D7Vector3E")
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6resizeEib")]
// 0x6eb568 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6resizeEib
pub fn stub_0x6eb568() -> ! {
    todo!("0x6eb568 __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE7reallocEi")]
// 0x6eb620 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE7reallocEi
pub fn stub_0x6eb620() -> ! {
    todo!("0x6eb620 __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE7reallocEi")
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::append(RBX::Point * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6appendERKS3_")]
// 0x6eb808 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6appendERKS3_
pub fn stub_0x6eb808() -> ! {
    todo!("0x6eb808 __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6appendERKS3_")
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::append(RBX::SimBody * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6appendERKS3_")]
// 0x6eb864 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6appendERKS3_
pub fn stub_0x6eb864() -> ! {
    todo!("0x6eb864 __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6appendERKS3_")
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6resizeEib")]
// 0x6eb8c0 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6resizeEib
pub fn stub_0x6eb8c0() -> ! {
    todo!("0x6eb8c0 __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE7reallocEi")]
// 0x6eb978 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE7reallocEi
pub fn stub_0x6eb978() -> ! {
    todo!("0x6eb978 __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE7reallocEi")
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::append(RBX::Body * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6appendERKS3_")]
// 0x6ebce4 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6appendERKS3_
pub fn stub_0x6ebce4() -> ! {
    todo!("0x6ebce4 __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6appendERKS3_")
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6resizeEib")]
// 0x6ebd40 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6resizeEib
pub fn stub_0x6ebd40() -> ! {
    todo!("0x6ebd40 __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE7reallocEi")]
// 0x6ebdf8 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE7reallocEi
pub fn stub_0x6ebdf8() -> ! {
    todo!("0x6ebdf8 __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EE7reallocEi")
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6resizeEib")]
// 0x6ecbac — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6resizeEib
pub fn stub_0x6ecbac() -> ! {
    todo!("0x6ecbac __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6resizeEib")
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE7reallocEi")]
// 0x6ecc64 — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE7reallocEi
pub fn stub_0x6ecc64() -> ! {
    todo!("0x6ecc64 __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE7reallocEi")
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::append(RBX::Connector * const&)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6appendERKS3_")]
// 0x6ece4c — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6appendERKS3_
pub fn stub_0x6ece4c() -> ! {
    todo!("0x6ece4c __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EE6appendERKS3_")
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EED2Ev")]
// 0x6ed470 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EED2Ev
pub fn stub_0x6ed470() -> ! {
    todo!("0x6ed470 __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EED2Ev")]
// 0x6ed544 — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EED2Ev
pub fn stub_0x6ed544() -> ! {
    todo!("0x6ed544 __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EED2Ev")]
// 0x6ed618 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EED2Ev
pub fn stub_0x6ed618() -> ! {
    todo!("0x6ed618 __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::~Array()")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EED2Ev")]
// 0x6ed6ec — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EED2Ev
pub fn stub_0x6ed6ec() -> ! {
    todo!("0x6ed6ec __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EED2Ev")
}

#[doc(alias = "G3D::Array<RBX::Connector *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EEC2Ev")]
// 0x6ed9bc — __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EEC2Ev
pub fn stub_0x6ed9bc() -> ! {
    todo!("0x6ed9bc __ZN3G3D5ArrayIPN3RBX9ConnectorELi10ELm32EEC2Ev")
}

#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EEC2Ev")]
// 0x6edaac — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EEC2Ev
pub fn stub_0x6edaac() -> ! {
    todo!("0x6edaac __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EEC2Ev")
}

#[doc(alias = "G3D::Array<RBX::Body *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EEC2Ev")]
// 0x6edb9c — __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EEC2Ev
pub fn stub_0x6edb9c() -> ! {
    todo!("0x6edb9c __ZN3G3D5ArrayIPN3RBX4BodyELi10ELm32EEC2Ev")
}

#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::Array(void)")]
#[doc(alias = "__ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EEC2Ev")]
// 0x6edc8c — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EEC2Ev
pub fn stub_0x6edc8c() -> ! {
    todo!("0x6edc8c __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EEC2Ev")
}

#[doc(alias = "RBX::Link::reset(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Link5resetERKN3G3D15CoordinateFrameES4_")]
// 0x6ee1a4 — __ZN3RBX4Link5resetERKN3G3D15CoordinateFrameES4_
// type: _DWORD __fastcall(RBX::Link *__hidden this, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *)
pub fn stub_0x6ee1a4() -> ! {
    todo!("0x6ee1a4 __ZN3RBX4Link5resetERKN3G3D15CoordinateFrameES4_")
}

#[doc(alias = "RBX::RevoluteLink::computeChildInParent(G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX12RevoluteLink20computeChildInParentERN3G3D15CoordinateFrameE")]
// 0x6ee2e0 — __ZNK3RBX12RevoluteLink20computeChildInParentERN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::RevoluteLink *__hidden this, G3D::CoordinateFrame *)
pub fn stub_0x6ee2e0() -> ! {
    todo!("0x6ee2e0 __ZNK3RBX12RevoluteLink20computeChildInParentERN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::D6Link::computeChildInParent(G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX6D6Link20computeChildInParentERN3G3D15CoordinateFrameE")]
// 0x6ee3bc — __ZNK3RBX6D6Link20computeChildInParentERN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::D6Link *__hidden this, G3D::CoordinateFrame *)
pub fn stub_0x6ee3bc() -> ! {
    todo!("0x6ee3bc __ZNK3RBX6D6Link20computeChildInParentERN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Point::setLocalPos(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX5Point11setLocalPosERKN3G3D7Vector3E")]
// 0x6ef68c — __ZN3RBX5Point11setLocalPosERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Point *__hidden this, const G3D::Vector3 *)
pub fn stub_0x6ef68c() -> ! {
    todo!("0x6ef68c __ZN3RBX5Point11setLocalPosERKN3G3D7Vector3E")
}

#[doc(alias = "G3D::Line::closestPoints(G3D::Line const&,G3D::Line const&,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_")]
// 0x6f0580 — __ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const G3D::Line *, const G3D::Line *, G3D::Vector3 *, G3D::Vector3 *)
pub fn stub_0x6f0580() -> ! {
    todo!("0x6f0580 __ZN3G3D4Line13closestPointsERKS0_S2_RNS_7Vector3ES4_")
}

#[doc(alias = "RBX::SimBody::applyImpulse(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX7SimBody12applyImpulseERKN3G3D7Vector3ES4_")]
// 0x6f1c48 — __ZN3RBX7SimBody12applyImpulseERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::SimBody *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
pub fn stub_0x6f1c48() -> ! {
    todo!("0x6f1c48 __ZN3RBX7SimBody12applyImpulseERKN3G3D7Vector3ES4_")
}

#[doc(alias = "G3D::Vector3 rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6f96e0 — __ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: void __fastcall(int, int)
pub fn stub_0x6f96e0() -> ! {
    todo!("0x6f96e0 __ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::CoordinateFrame rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6f97dc — __ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: void __fastcall(int, int)
pub fn stub_0x6f97dc() -> ! {
    todo!("0x6f97dc __ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::Color3 * rbx::any_cast<G3D::Color3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D6Color3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fb968 — __ZN3rbx8any_castIN3G3D6Color3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fb968() -> ! {
    todo!("0x6fb968 __ZN3rbx8any_castIN3G3D6Color3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::Color3 & rbx::any_cast<G3D::Color3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fb9c0 — __ZN3rbx8any_castIRN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fb9c0() -> ! {
    todo!("0x6fb9c0 __ZN3rbx8any_castIRN3G3D6Color3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::Vector2int16 * rbx::any_cast<G3D::Vector2int16,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D12Vector2int16EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fc4f8 — __ZN3rbx8any_castIN3G3D12Vector2int16EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fc4f8() -> ! {
    todo!("0x6fc4f8 __ZN3rbx8any_castIN3G3D12Vector2int16EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2int16>(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector2int16EEERS3_RKT_")]
// 0x6fc550 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector2int16EEERS3_RKT_
pub fn stub_0x6fc550() -> ! {
    todo!("0x6fc550 __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector2int16EEERS3_RKT_")
}

#[doc(alias = "G3D::Vector2int16 & rbx::any_cast<G3D::Vector2int16 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fc5a0 — __ZN3rbx8any_castIRN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fc5a0() -> ! {
    todo!("0x6fc5a0 __ZN3rbx8any_castIRN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2int16>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE9singletonEv")]
// 0x6fc690 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE9singletonEv
pub fn stub_0x6fc690() -> ! {
    todo!("0x6fc690 __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2int16>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE13destruct_funcEPc")]
// 0x6fc6fc — __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE13destruct_funcEPc
// type: void()
pub fn stub_0x6fc6fc() -> ! {
    todo!("0x6fc6fc __ZN3rbx14implementation12typed_holderIN3G3D12Vector2int16EE13destruct_funcEPc")
}

#[doc(alias = "G3D::CoordinateFrame * rbx::any_cast<G3D::CoordinateFrame,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fd0bc — __ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fd0bc() -> ! {
    todo!("0x6fd0bc __ZN3rbx8any_castIN3G3D15CoordinateFrameEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::CoordinateFrame & rbx::any_cast<G3D::CoordinateFrame &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fd114 — __ZN3rbx8any_castIRN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fd114() -> ! {
    todo!("0x6fd114 __ZN3rbx8any_castIRN3G3D15CoordinateFrameEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::CoordinateFrame>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE9singletonEv")]
// 0x6fd204 — __ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE9singletonEv
pub fn stub_0x6fd204() -> ! {
    todo!("0x6fd204 __ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<G3D::CoordinateFrame>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE13destruct_funcEPc")]
// 0x6fd270 — __ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE13destruct_funcEPc
pub fn stub_0x6fd270() -> ! {
    todo!("0x6fd270 __ZN3rbx14implementation12typed_holderIN3G3D15CoordinateFrameEE13destruct_funcEPc")
}

#[doc(alias = "G3D::Vector2 * rbx::any_cast<G3D::Vector2,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector2EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fd274 — __ZN3rbx8any_castIN3G3D7Vector2EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fd274() -> ! {
    todo!("0x6fd274 __ZN3rbx8any_castIN3G3D7Vector2EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::Vector2 & rbx::any_cast<G3D::Vector2 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fd2cc — __ZN3rbx8any_castIRN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fd2cc() -> ! {
    todo!("0x6fd2cc __ZN3rbx8any_castIRN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::Vector3 * rbx::any_cast<G3D::Vector3,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x6fd3bc — __ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0x6fd3bc() -> ! {
    todo!("0x6fd3bc __ZN3rbx8any_castIN3G3D7Vector3EN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "G3D::Vector3 & rbx::any_cast<G3D::Vector3 &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x6fd414 — __ZN3rbx8any_castIRN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x6fd414() -> ! {
    todo!("0x6fd414 __ZN3rbx8any_castIRN3G3D7Vector3EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::StringConverter<G3D::Vector2int16>::convertToString(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D12Vector2int16EE15convertToStringERKS2_")]
// 0x711ea0 — __ZN3RBX15StringConverterIN3G3D12Vector2int16EE15convertToStringERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x711ea0() -> ! {
    todo!("0x711ea0 __ZN3RBX15StringConverterIN3G3D12Vector2int16EE15convertToStringERKS2_")
}

#[doc(alias = "RBX::StringConverter<G3D::Vector2int16>::convertToValue(std::string const&,G3D::Vector2int16&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D12Vector2int16EE14convertToValueERKSsRS2_")]
// 0x712010 — __ZN3RBX15StringConverterIN3G3D12Vector2int16EE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x712010() -> ! {
    todo!("0x712010 __ZN3RBX15StringConverterIN3G3D12Vector2int16EE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::StringConverter<G3D::Vector3int16>::convertToString(G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D12Vector3int16EE15convertToStringERKS2_")]
// 0x7120f4 — __ZN3RBX15StringConverterIN3G3D12Vector3int16EE15convertToStringERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x7120f4() -> ! {
    todo!("0x7120f4 __ZN3RBX15StringConverterIN3G3D12Vector3int16EE15convertToStringERKS2_")
}