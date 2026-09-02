//! core shard li — 100 core stubs EA-sorted, next uncovered fallback after shard lh (0x38dcb0..0x3fbf40, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 41432, 9432 uncovered before batch, 9332 after), EA-sorted asc, next 100 uncovered not yet in core.
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Accoutrement::setAttachmentForward(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement20setAttachmentForwardERKN3G3D7Vector3E")]
// 0x38dcb0 — __ZN3RBX12Accoutrement20setAttachmentForwardERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
pub fn stub_0x38dcb0() -> ! {
    todo!("0x38dcb0 __ZN3RBX12Accoutrement20setAttachmentForwardERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Accoutrement::setAttachmentUp(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement15setAttachmentUpERKN3G3D7Vector3E")]
// 0x38de0c — __ZN3RBX12Accoutrement15setAttachmentUpERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
pub fn stub_0x38de0c() -> ! {
    todo!("0x38de0c __ZN3RBX12Accoutrement15setAttachmentUpERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Accoutrement::setAttachmentRight(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12Accoutrement18setAttachmentRightERKN3G3D7Vector3E")]
// 0x38df40 — __ZN3RBX12Accoutrement18setAttachmentRightERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
pub fn stub_0x38df40() -> ! {
    todo!("0x38df40 __ZN3RBX12Accoutrement18setAttachmentRightERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb")]
// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: void __fastcall(int, int)
pub fn stub_0x3a7f68() -> ! {
    todo!("0x3a7f68 __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb")]
// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: void __fastcall(int, int)
pub fn stub_0x3a80c8() -> ! {
    todo!("0x3a80c8 __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv")]
// 0x3a9944 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
// type: int __fastcall(int)
pub fn stub_0x3a9944() -> ! {
    todo!("0x3a9944 __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv")]
// 0x3a9c2c — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv
// type: int __fastcall(int)
pub fn stub_0x3a9c2c() -> ! {
    todo!("0x3a9c2c __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv")
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev")]
// 0x3a9ea0 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
pub fn stub_0x3a9ea0() -> ! {
    todo!("0x3a9ea0 __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEEC2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev")]
// 0x3aa174 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
pub fn stub_0x3aa174() -> ! {
    todo!("0x3aa174 __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEEC2Ev")
}

#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")]
// 0x3aa448 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
pub fn stub_0x3aa448() -> ! {
    todo!("0x3aa448 __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")
}

#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff")]
// 0x3aa5a4 — __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff
pub fn stub_0x3aa5a4() -> ! {
    todo!("0x3aa5a4 __ZN3RBX19EventReplicatorImplILi3ENS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE25signalProducedIncrementedES4_ff")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")]
// 0x3ab0a0 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv
// type: void()
pub fn stub_0x3ab0a0() -> ! {
    todo!("0x3ab0a0 __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE21connectSignalListenerEv")
}

#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")]
// 0x3ab0a4 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void __fastcall(_DWORD *)
pub fn stub_0x3ab0a4() -> ! {
    todo!("0x3ab0a4 __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")
}

#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_")]
// 0x3ab200 — __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_
// type: int __fastcall(int, int)
pub fn stub_0x3ab200() -> ! {
    todo!("0x3ab200 __ZN3RBX19EventReplicatorImplILi1ENS_10ArcHandlesEFvN3G3D7Vector34AxisEEE25signalProducedIncrementedES4_")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")]
// 0x3abc44 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv
// type: void()
pub fn stub_0x3abc44() -> ! {
    todo!("0x3abc44 __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE21connectSignalListenerEv")
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev")]
// 0x3b0324 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
pub fn stub_0x3b0324() -> ! {
    todo!("0x3b0324 __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEffEED2Ev")
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev")]
// 0x3b0470 — __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev
// type: int *__fastcall(int, int, int, int, char, int)
pub fn stub_0x3b0470() -> ! {
    todo!("0x3b0470 __ZN3rbx13remote_signalIFvN3G3D7Vector34AxisEEED2Ev")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev")]
// 0x3b05bc — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3b05bc() -> ! {
    todo!("0x3b05bc __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev")
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev")]
// 0x3b06ec — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3b06ec() -> ! {
    todo!("0x3b06ec __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev")
}

#[doc(alias = "RBX::Backpack::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZN3RBX8Backpack15scriptShouldRunEPNS_10BaseScriptE")]
// 0x3b1014 — __ZN3RBX8Backpack15scriptShouldRunEPNS_10BaseScriptE
// type: int __fastcall(RBX::Backpack *this, RBX::Reflection::ClassDescriptor **, int, int (*)(const char *, ...))
pub fn stub_0x3b1014() -> ! {
    todo!("0x3b1014 __ZN3RBX8Backpack15scriptShouldRunEPNS_10BaseScriptE")
}

#[doc(alias = "non-virtual thunk toRBX::Backpack::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZThn148_N3RBX8Backpack15scriptShouldRunEPNS_10BaseScriptE")]
// 0x3b1218 — __ZThn148_N3RBX8Backpack15scriptShouldRunEPNS_10BaseScriptE
// type: int __fastcall(RBX::Backpack *this, RBX::Reflection::ClassDescriptor **, int, int (*)(const char *, ...))
pub fn stub_0x3b1218() -> ! {
    todo!("0x3b1218 __ZThn148_N3RBX8Backpack15scriptShouldRunEPNS_10BaseScriptE")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")]
// 0x3bf254 — __ZNK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv
pub fn stub_0x3bf254() -> ! {
    todo!("0x3bf254 __ZNK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")]
// 0x3bf328 — __ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv
pub fn stub_0x3bf328() -> ! {
    todo!("0x3bf328 __ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")
}

#[doc(alias = "RBX::BillboardGui::setStudsOffset(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E")]
// 0x3bfd80 — __ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
pub fn stub_0x3bfd80() -> ! {
    todo!("0x3bfd80 __ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::BillboardGui::setExtentsOffset(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E")]
// 0x3bfdf8 — __ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
pub fn stub_0x3bfdf8() -> ! {
    todo!("0x3bfdf8 __ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::BillboardGui::setSizeOffset(G3D::Vector2 const&)")]
#[doc(alias = "__ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E")]
// 0x3bfe70 — __ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector2 *)
pub fn stub_0x3bfe70() -> ! {
    todo!("0x3bfe70 __ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E")
}

#[doc(alias = "RBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")]
// 0x3c12b8 — __ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
// type: int()
pub fn stub_0x3c12b8() -> ! {
    todo!("0x3c12b8 __ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")
}

#[doc(alias = "non-virtual thunk toRBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
#[doc(alias = "__ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")]
// 0x3c1424 — __ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
// type: int()
pub fn stub_0x3c1424() -> ! {
    todo!("0x3c1424 __ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::Camera::setCameraCoordinateFrame(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX6Camera24setCameraCoordinateFrameERKN3G3D15CoordinateFrameE")]
// 0x3c35d4 — __ZN3RBX6Camera24setCameraCoordinateFrameERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
pub fn stub_0x3c35d4() -> ! {
    todo!("0x3c35d4 __ZN3RBX6Camera24setCameraCoordinateFrameERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Camera::setCameraFocus(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX6Camera14setCameraFocusERKN3G3D15CoordinateFrameE")]
// 0x3c3738 — __ZN3RBX6Camera14setCameraFocusERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
pub fn stub_0x3c3738() -> ! {
    todo!("0x3c3738 __ZN3RBX6Camera14setCameraFocusERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Camera::beginCameraInterpolation(G3D::CoordinateFrame,G3D::CoordinateFrame,float)")]
#[doc(alias = "__ZN3RBX6Camera24beginCameraInterpolationEN3G3D15CoordinateFrameES2_f")]
// 0x3c3fa4 — __ZN3RBX6Camera24beginCameraInterpolationEN3G3D15CoordinateFrameES2_f
// type: void __fastcall(int, __int64 *, __int64 *, float)
pub fn stub_0x3c3fa4() -> ! {
    todo!("0x3c3fa4 __ZN3RBX6Camera24beginCameraInterpolationEN3G3D15CoordinateFrameES2_f")
}

#[doc(alias = "RBX::Camera::frustum(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX6Camera7frustumERKN3G3D6Rect2DE")]
// 0x3c51c0 — __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DE
// type: void __fastcall(int, int, int)
pub fn stub_0x3c51c0() -> ! {
    todo!("0x3c51c0 __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::Camera::getNearViewportCorners(G3D::Rect2D const&,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_")]
// 0x3c5284 — __ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_
// type: int __fastcall(int result, __int32 *, __int32 *, __int32 *, __int32 *, __int32 *)
pub fn stub_0x3c5284() -> ! {
    todo!("0x3c5284 __ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_")
}

#[doc(alias = "RBX::Camera::setCameraFocusWithoutPropertyChange(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX6Camera35setCameraFocusWithoutPropertyChangeERKN3G3D15CoordinateFrameE")]
// 0x3c6144 — __ZN3RBX6Camera35setCameraFocusWithoutPropertyChangeERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
pub fn stub_0x3c6144() -> ! {
    todo!("0x3c6144 __ZN3RBX6Camera35setCameraFocusWithoutPropertyChangeERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Camera::setCameraFocusOnly(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX6Camera18setCameraFocusOnlyERKN3G3D15CoordinateFrameE")]
// 0x3c6210 — __ZN3RBX6Camera18setCameraFocusOnlyERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
pub fn stub_0x3c6210() -> ! {
    todo!("0x3c6210 __ZN3RBX6Camera18setCameraFocusOnlyERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Camera::setCameraFocusOnlyWithoutPropertyChange(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX6Camera39setCameraFocusOnlyWithoutPropertyChangeERKN3G3D15CoordinateFrameE")]
// 0x3c690c — __ZN3RBX6Camera39setCameraFocusOnlyWithoutPropertyChangeERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
pub fn stub_0x3c690c() -> ! {
    todo!("0x3c690c __ZN3RBX6Camera39setCameraFocusOnlyWithoutPropertyChangeERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Camera::setDistanceFromTarget(float,G3D::CoordinateFrame &,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX6Camera21setDistanceFromTargetEfRN3G3D15CoordinateFrameERKS2_")]
// 0x3c6d3c — __ZN3RBX6Camera21setDistanceFromTargetEfRN3G3D15CoordinateFrameERKS2_
// type: int __fastcall(RBX::Camera *this, float, G3D::CoordinateFrame *, const G3D::CoordinateFrame *)
pub fn stub_0x3c6d3c() -> ! {
    todo!("0x3c6d3c __ZN3RBX6Camera21setDistanceFromTargetEfRN3G3D15CoordinateFrameERKS2_")
}

#[doc(alias = "RBX::Camera::lerpToExtents(RBX::Extents const&,G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX6Camera13lerpToExtentsERKNS_7ExtentsERKN3G3D6Rect2DE")]
// 0x3c6e7c — __ZN3RBX6Camera13lerpToExtentsERKNS_7ExtentsERKN3G3D6Rect2DE
// type: void __fastcall(int, RBX::Extents *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x3c6e7c() -> ! {
    todo!("0x3c6e7c __ZN3RBX6Camera13lerpToExtentsERKNS_7ExtentsERKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::Camera::zoomExtents(G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX6Camera11zoomExtentsERKN3G3D6Rect2DE")]
// 0x3c79f4 — __ZN3RBX6Camera11zoomExtentsERKN3G3D6Rect2DE
// type: int __fastcall(RBX::Camera *)
pub fn stub_0x3c79f4() -> ! {
    todo!("0x3c79f4 __ZN3RBX6Camera11zoomExtentsERKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::Camera::setCameraFocusAndMaintainFocus(G3D::CoordinateFrame const&,bool)")]
#[doc(alias = "__ZN3RBX6Camera30setCameraFocusAndMaintainFocusERKN3G3D15CoordinateFrameEb")]
// 0x3c7b34 — __ZN3RBX6Camera30setCameraFocusAndMaintainFocusERKN3G3D15CoordinateFrameEb
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *, bool)
pub fn stub_0x3c7b34() -> ! {
    todo!("0x3c7b34 __ZN3RBX6Camera30setCameraFocusAndMaintainFocusERKN3G3D15CoordinateFrameEb")
}

#[doc(alias = "RBX::Camera::legalCameraCoord(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX6Camera16legalCameraCoordERKN3G3D15CoordinateFrameE")]
// 0x3c7b48 — __ZN3RBX6Camera16legalCameraCoordERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Math *, const G3D::CoordinateFrame *)
pub fn stub_0x3c7b48() -> ! {
    todo!("0x3c7b48 __ZN3RBX6Camera16legalCameraCoordERKN3G3D15CoordinateFrameE")
}

#[doc(alias = "RBX::Camera::getImagePlaneDepth(G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX6Camera18getImagePlaneDepthERKN3G3D6Rect2DE")]
// 0x3c8734 — __ZNK3RBX6Camera18getImagePlaneDepthERKN3G3D6Rect2DE
// type: unsigned __int32 __fastcall(int, int)
pub fn stub_0x3c8734() -> ! {
    todo!("0x3c8734 __ZNK3RBX6Camera18getImagePlaneDepthERKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::Camera::project(G3D::Vector3 const&,G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX6Camera7projectERKN3G3D7Vector3ERKNS1_6Rect2DE")]
// 0x3c8750 — __ZNK3RBX6Camera7projectERKN3G3D7Vector3ERKNS1_6Rect2DE
// type: int *__fastcall(int *result, _DWORD *, __int32 *, __int32 *)
pub fn stub_0x3c8750() -> ! {
    todo!("0x3c8750 __ZNK3RBX6Camera7projectERKN3G3D7Vector3ERKNS1_6Rect2DE")
}

#[doc(alias = "RBX::Camera::worldRay(float,float,G3D::Rect2D const&)const")]
#[doc(alias = "__ZNK3RBX6Camera8worldRayEffRKN3G3D6Rect2DE")]
// 0x3c8888 — __ZNK3RBX6Camera8worldRayEffRKN3G3D6Rect2DE
// type: int __fastcall(__int64, __int32, __int32, __int32 *)
pub fn stub_0x3c8888() -> ! {
    todo!("0x3c8888 __ZNK3RBX6Camera8worldRayEffRKN3G3D6Rect2DE")
}

#[doc(alias = "RBX::Camera::dot(G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX6Camera3dotERKN3G3D7Vector3E")]
// 0x3c89dc — __ZNK3RBX6Camera3dotERKN3G3D7Vector3E
// type: unsigned __int32 __fastcall(RBX::Camera *this, const G3D::Vector3 *)
pub fn stub_0x3c89dc() -> ! {
    todo!("0x3c89dc __ZNK3RBX6Camera3dotERKN3G3D7Vector3E")
}

#[doc(alias = "RBX::Camera::frustum(G3D::Rect2D const&,float,RBX::Frustum &)const")]
#[doc(alias = "__ZNK3RBX6Camera7frustumERKN3G3D6Rect2DEfRNS_7FrustumE")]
// 0x3c8a58 — __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DEfRNS_7FrustumE
// type: void __fastcall(float *, __int32 *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int)
pub fn stub_0x3c8a58() -> ! {
    todo!("0x3c8a58 __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DEfRNS_7FrustumE")
}

#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::at(unsigned long)")]
#[doc(alias = "__ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE2atEm")]
// 0x3c9acc — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE2atEm
// type: int __fastcall(int *, unsigned int)
pub fn stub_0x3c9acc() -> ! {
    todo!("0x3c9acc __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE2atEm")
}

#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::insert(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// 0x3c9b00 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int)
pub fn stub_0x3c9b00() -> ! {
    todo!("0x3c9b00 __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::push_back(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE9push_backERKS3_")]
// 0x3c9b48 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE9push_backERKS3_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
pub fn stub_0x3c9b48() -> ! {
    todo!("0x3c9b48 __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "RBX::Extents::contains(G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX7Extents8containsERKN3G3D7Vector3E")]
// 0x3c9b80 — __ZNK3RBX7Extents8containsERKN3G3D7Vector3E
// type: bool __fastcall(RBX::Extents *this, const Vector3 *)
pub fn stub_0x3c9b80() -> ! {
    todo!("0x3c9b80 __ZNK3RBX7Extents8containsERKN3G3D7Vector3E")
}

#[doc(alias = "std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy<false,std::random_access_iterator_tag>::copy<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_")]
// 0x3cba0c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_
// type: int __fastcall(__int64 *, int, int)
pub fn stub_0x3cba0c() -> ! {
    todo!("0x3cba0c __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>::construct(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorISt4pairIN3G3D15CoordinateFrameES3_EE9constructEPS4_RKS4_")]
// 0x3cbab4 — __ZN9__gnu_cxx13new_allocatorISt4pairIN3G3D15CoordinateFrameES3_EE9constructEPS4_RKS4_
// type: int __fastcall(int, int, int)
pub fn stub_0x3cbab4() -> ! {
    todo!("0x3cbab4 __ZN9__gnu_cxx13new_allocatorISt4pairIN3G3D15CoordinateFrameES3_EE9constructEPS4_RKS4_")
}

#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
#[doc(alias = "__ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// 0x3cbaf0 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void __fastcall(int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
pub fn stub_0x3cbaf0() -> ! {
    todo!("0x3cbaf0 __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::_Vector_base<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE11_M_allocateEm")]
// 0x3cbe40 — __ZNSt12_Vector_baseISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x3cbe40() -> ! {
    todo!("0x3cbe40 __ZNSt12_Vector_baseISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_")]
// 0x3cbe64 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_
// type: int __fastcall(int, int, int)
pub fn stub_0x3cbe64() -> ! {
    todo!("0x3cbe64 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_")
}

#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::operator=(G3D::Array<G3D::Plane,10,32ul> const&)")]
#[doc(alias = "__ZN3G3D5ArrayINS_5PlaneELi10ELm32EEaSERKS2_")]
// 0x3d1900 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEaSERKS2_
// type: int *__fastcall(int *, int *)
pub fn stub_0x3d1900() -> ! {
    todo!("0x3d1900 __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEaSERKS2_")
}

#[doc(alias = "RBX::ChangeHistoryService::setCell(G3D::Vector3int16 const&,G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)")]
#[doc(alias = "__ZN3RBX20ChangeHistoryService7setCellERKN3G3D12Vector3int16ES4_NS_5Voxel4CellENS5_12CellMaterialE")]
// 0x3d362c — __ZN3RBX20ChangeHistoryService7setCellERKN3G3D12Vector3int16ES4_NS_5Voxel4CellENS5_12CellMaterialE
// type: int __fastcall(int)
pub fn stub_0x3d362c() -> ! {
    todo!("0x3d362c __ZN3RBX20ChangeHistoryService7setCellERKN3G3D12Vector3int16ES4_NS_5Voxel4CellENS5_12CellMaterialE")
}

#[doc(alias = "RBX::BoolPropertyVerb::BoolPropertyVerb(std::string const&,RBX::DataModel *,char const*)")]
#[doc(alias = "__ZN3RBX16BoolPropertyVerbC2ERKSsPNS_9DataModelEPKc")]
// 0x3f5208 — __ZN3RBX16BoolPropertyVerbC2ERKSsPNS_9DataModelEPKc
// type: char **__fastcall(char **this, const std::string *, RBX::DataModel *, const char *)
pub fn stub_0x3f5208() -> ! {
    todo!("0x3f5208 __ZN3RBX16BoolPropertyVerbC2ERKSsPNS_9DataModelEPKc")
}

#[doc(alias = "RBX::EditSelectionVerb::EditSelectionVerb(std::string,RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX17EditSelectionVerbC2ESsPNS_9DataModelE")]
// 0x3f5368 — __ZN3RBX17EditSelectionVerbC2ESsPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::Verb *, const std::string *, _DWORD *)
pub fn stub_0x3f5368() -> ! {
    todo!("0x3f5368 __ZN3RBX17EditSelectionVerbC2ESsPNS_9DataModelE")
}

#[doc(alias = "RBX::requireEdit(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBXL11requireEditEPNS_9DataModelE")]
// 0x3f5710 — __ZN3RBXL11requireEditEPNS_9DataModelE
// type: void __fastcall(RBX *this, RBX::DataModel *, bool)
pub fn stub_0x3f5710() -> ! {
    todo!("0x3f5710 __ZN3RBXL11requireEditEPNS_9DataModelE")
}

#[doc(alias = "RBX::FirstPersonCommand::FirstPersonCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18FirstPersonCommandC1EPNS_9DataModelE")]
// 0x3f6054 — __ZN3RBX18FirstPersonCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::FirstPersonCommand *this, RBX::DataModel *)
pub fn stub_0x3f6054() -> ! {
    todo!("0x3f6054 __ZN3RBX18FirstPersonCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::FirstPersonCommand::FirstPersonCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18FirstPersonCommandC2EPNS_9DataModelE")]
// 0x3f6058 — __ZN3RBX18FirstPersonCommandC2EPNS_9DataModelE
// type: RBX::FirstPersonCommand *__fastcall(RBX::FirstPersonCommand *this, RBX::DataModel *)
pub fn stub_0x3f6058() -> ! {
    todo!("0x3f6058 __ZN3RBX18FirstPersonCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::ToggleViewMode::ToggleViewMode(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX14ToggleViewModeC1EPNS_9DataModelE")]
// 0x3f61cc — __ZN3RBX14ToggleViewModeC1EPNS_9DataModelE
// type: int __fastcall(RBX::ToggleViewMode *this, RBX::DataModel *)
pub fn stub_0x3f61cc() -> ! {
    todo!("0x3f61cc __ZN3RBX14ToggleViewModeC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::ToggleViewMode::ToggleViewMode(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX14ToggleViewModeC2EPNS_9DataModelE")]
// 0x3f61d0 — __ZN3RBX14ToggleViewModeC2EPNS_9DataModelE
// type: RBX::ToggleViewMode *__fastcall(RBX::ToggleViewMode *this, RBX::DataModel *)
pub fn stub_0x3f61d0() -> ! {
    todo!("0x3f61d0 __ZN3RBX14ToggleViewModeC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::StatsCommand::StatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX12StatsCommandC1EPNS_9DataModelE")]
// 0x3f637c — __ZN3RBX12StatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::StatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f637c() -> ! {
    todo!("0x3f637c __ZN3RBX12StatsCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::StatsCommand::StatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX12StatsCommandC2EPNS_9DataModelE")]
// 0x3f6380 — __ZN3RBX12StatsCommandC2EPNS_9DataModelE
// type: RBX::StatsCommand *__fastcall(RBX::StatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f6380() -> ! {
    todo!("0x3f6380 __ZN3RBX12StatsCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::RenderStatsCommand::RenderStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18RenderStatsCommandC1EPNS_9DataModelE")]
// 0x3f6a8c — __ZN3RBX18RenderStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::RenderStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f6a8c() -> ! {
    todo!("0x3f6a8c __ZN3RBX18RenderStatsCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::RenderStatsCommand::RenderStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18RenderStatsCommandC2EPNS_9DataModelE")]
// 0x3f6a90 — __ZN3RBX18RenderStatsCommandC2EPNS_9DataModelE
// type: RBX::RenderStatsCommand *__fastcall(RBX::RenderStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f6a90() -> ! {
    todo!("0x3f6a90 __ZN3RBX18RenderStatsCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SummaryStatsCommand::SummaryStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19SummaryStatsCommandC1EPNS_9DataModelE")]
// 0x3f71b8 — __ZN3RBX19SummaryStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::SummaryStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f71b8() -> ! {
    todo!("0x3f71b8 __ZN3RBX19SummaryStatsCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SummaryStatsCommand::SummaryStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19SummaryStatsCommandC2EPNS_9DataModelE")]
// 0x3f71bc — __ZN3RBX19SummaryStatsCommandC2EPNS_9DataModelE
// type: RBX::SummaryStatsCommand *__fastcall(RBX::SummaryStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f71bc() -> ! {
    todo!("0x3f71bc __ZN3RBX19SummaryStatsCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::CustomStatsCommand::CustomStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18CustomStatsCommandC1EPNS_9DataModelE")]
// 0x3f77d4 — __ZN3RBX18CustomStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::CustomStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f77d4() -> ! {
    todo!("0x3f77d4 __ZN3RBX18CustomStatsCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::CustomStatsCommand::CustomStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18CustomStatsCommandC2EPNS_9DataModelE")]
// 0x3f77d8 — __ZN3RBX18CustomStatsCommandC2EPNS_9DataModelE
// type: RBX::CustomStatsCommand *__fastcall(RBX::CustomStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f77d8() -> ! {
    todo!("0x3f77d8 __ZN3RBX18CustomStatsCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE")]
// 0x3f7df0 — __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f7df0() -> ! {
    todo!("0x3f7df0 __ZN3RBX19NetworkStatsCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::NetworkStatsCommand::NetworkStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE")]
// 0x3f7df4 — __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::NetworkStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f7df4() -> ! {
    todo!("0x3f7df4 __ZN3RBX19NetworkStatsCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::PhysicsStatsCommand::PhysicsStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19PhysicsStatsCommandC1EPNS_9DataModelE")]
// 0x3f8570 — __ZN3RBX19PhysicsStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::PhysicsStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f8570() -> ! {
    todo!("0x3f8570 __ZN3RBX19PhysicsStatsCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::PhysicsStatsCommand::PhysicsStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19PhysicsStatsCommandC2EPNS_9DataModelE")]
// 0x3f8574 — __ZN3RBX19PhysicsStatsCommandC2EPNS_9DataModelE
// type: RBX::PhysicsStatsCommand *__fastcall(RBX::PhysicsStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f8574() -> ! {
    todo!("0x3f8574 __ZN3RBX19PhysicsStatsCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::EngineStatsCommand::EngineStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18EngineStatsCommandC1EPNS_9DataModelE")]
// 0x3f8e6c — __ZN3RBX18EngineStatsCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::EngineStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f8e6c() -> ! {
    todo!("0x3f8e6c __ZN3RBX18EngineStatsCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::EngineStatsCommand::EngineStatsCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18EngineStatsCommandC2EPNS_9DataModelE")]
// 0x3f8e70 — __ZN3RBX18EngineStatsCommandC2EPNS_9DataModelE
// type: RBX::EngineStatsCommand *__fastcall(RBX::EngineStatsCommand *this, RBX::DataModel *)
pub fn stub_0x3f8e70() -> ! {
    todo!("0x3f8e70 __ZN3RBX18EngineStatsCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::JoinCommand::JoinCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX11JoinCommandC1EPNS_9DataModelE")]
// 0x3f9004 — __ZN3RBX11JoinCommandC1EPNS_9DataModelE
// type: int __fastcall(RBX::JoinCommand *this, RBX::DataModel *)
pub fn stub_0x3f9004() -> ! {
    todo!("0x3f9004 __ZN3RBX11JoinCommandC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::JoinCommand::JoinCommand(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX11JoinCommandC2EPNS_9DataModelE")]
// 0x3f9008 — __ZN3RBX11JoinCommandC2EPNS_9DataModelE
// type: RBX::JoinCommand *__fastcall(RBX::JoinCommand *this, RBX::DataModel *)
pub fn stub_0x3f9008() -> ! {
    todo!("0x3f9008 __ZN3RBX11JoinCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::RunStateVerb::RunStateVerb(std::string,RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX12RunStateVerbC2ESsPNS_9DataModelE")]
// 0x3f93e4 — __ZN3RBX12RunStateVerbC2ESsPNS_9DataModelE
// type: RBX::Verb *__fastcall(RBX::Verb *, const std::string *, int)
pub fn stub_0x3f93e4() -> ! {
    todo!("0x3f93e4 __ZN3RBX12RunStateVerbC2ESsPNS_9DataModelE")
}

#[doc(alias = "RBX::GroupSelectionVerb::GroupSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18GroupSelectionVerbC1EPNS_9DataModelE")]
// 0x3f986c — __ZN3RBX18GroupSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::GroupSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3f986c() -> ! {
    todo!("0x3f986c __ZN3RBX18GroupSelectionVerbC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::GroupSelectionVerb::GroupSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18GroupSelectionVerbC2EPNS_9DataModelE")]
// 0x3f9870 — __ZN3RBX18GroupSelectionVerbC2EPNS_9DataModelE
// type: RBX::GroupSelectionVerb *__fastcall(RBX::GroupSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3f9870() -> ! {
    todo!("0x3f9870 __ZN3RBX18GroupSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SnapSelectionVerb::SnapSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX17SnapSelectionVerbC1EPNS_9DataModelE")]
// 0x3f9ab0 — __ZN3RBX17SnapSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::SnapSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3f9ab0() -> ! {
    todo!("0x3f9ab0 __ZN3RBX17SnapSelectionVerbC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SnapSelectionVerb::SnapSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX17SnapSelectionVerbC2EPNS_9DataModelE")]
// 0x3f9ab4 — __ZN3RBX17SnapSelectionVerbC2EPNS_9DataModelE
// type: RBX::SnapSelectionVerb *__fastcall(RBX::SnapSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3f9ab4() -> ! {
    todo!("0x3f9ab4 __ZN3RBX17SnapSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::UngroupSelectionVerb::UngroupSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX20UngroupSelectionVerbC1EPNS_9DataModelE")]
// 0x3f9d94 — __ZN3RBX20UngroupSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::UngroupSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3f9d94() -> ! {
    todo!("0x3f9d94 __ZN3RBX20UngroupSelectionVerbC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::UngroupSelectionVerb::UngroupSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX20UngroupSelectionVerbC2EPNS_9DataModelE")]
// 0x3f9d98 — __ZN3RBX20UngroupSelectionVerbC2EPNS_9DataModelE
// type: RBX::EditSelectionVerb *__fastcall(RBX::UngroupSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3f9d98() -> ! {
    todo!("0x3f9d98 __ZN3RBX20UngroupSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SelectChildrenVerb::SelectChildrenVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18SelectChildrenVerbC1EPNS_9DataModelE")]
// 0x3fa22c — __ZN3RBX18SelectChildrenVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::SelectChildrenVerb *this, RBX::DataModel *)
pub fn stub_0x3fa22c() -> ! {
    todo!("0x3fa22c __ZN3RBX18SelectChildrenVerbC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SelectChildrenVerb::SelectChildrenVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX18SelectChildrenVerbC2EPNS_9DataModelE")]
// 0x3fa230 — __ZN3RBX18SelectChildrenVerbC2EPNS_9DataModelE
// type: RBX::EditSelectionVerb *__fastcall(RBX::SelectChildrenVerb *this, RBX::DataModel *)
pub fn stub_0x3fa230() -> ! {
    todo!("0x3fa230 __ZN3RBX18SelectChildrenVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::DeleteBase::DeleteBase(RBX::DataModel *,std::string,bool)")]
#[doc(alias = "__ZN3RBX10DeleteBaseC2EPNS_9DataModelESsb")]
// 0x3fa6c0 — __ZN3RBX10DeleteBaseC2EPNS_9DataModelESsb
// type: RBX::Verb *__fastcall(RBX::Verb *, _DWORD *, const std::string *, char)
pub fn stub_0x3fa6c0() -> ! {
    todo!("0x3fa6c0 __ZN3RBX10DeleteBaseC2EPNS_9DataModelESsb")
}

#[doc(alias = "RBX::RotateSelectionVerb::RotateSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19RotateSelectionVerbC1EPNS_9DataModelE")]
// 0x3fabb8 — __ZN3RBX19RotateSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::RotateSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3fabb8() -> ! {
    todo!("0x3fabb8 __ZN3RBX19RotateSelectionVerbC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::RotateSelectionVerb::RotateSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX19RotateSelectionVerbC2EPNS_9DataModelE")]
// 0x3fabbc — __ZN3RBX19RotateSelectionVerbC2EPNS_9DataModelE
// type: RBX::RotateSelectionVerb *__fastcall(RBX::RotateSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3fabbc() -> ! {
    todo!("0x3fabbc __ZN3RBX19RotateSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::TiltSelectionVerb::TiltSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX17TiltSelectionVerbC1EPNS_9DataModelE")]
// 0x3fada4 — __ZN3RBX17TiltSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::TiltSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3fada4() -> ! {
    todo!("0x3fada4 __ZN3RBX17TiltSelectionVerbC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::TiltSelectionVerb::TiltSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX17TiltSelectionVerbC2EPNS_9DataModelE")]
// 0x3fada8 — __ZN3RBX17TiltSelectionVerbC2EPNS_9DataModelE
// type: RBX::TiltSelectionVerb *__fastcall(RBX::TiltSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3fada8() -> ! {
    todo!("0x3fada8 __ZN3RBX17TiltSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::MoveDownSelectionVerb::MoveDownSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX21MoveDownSelectionVerbC1EPNS_9DataModelE")]
// 0x3fb09c — __ZN3RBX21MoveDownSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::MoveDownSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3fb09c() -> ! {
    todo!("0x3fb09c __ZN3RBX21MoveDownSelectionVerbC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::MoveDownSelectionVerb::MoveDownSelectionVerb(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX21MoveDownSelectionVerbC2EPNS_9DataModelE")]
// 0x3fb0a0 — __ZN3RBX21MoveDownSelectionVerbC2EPNS_9DataModelE
// type: RBX::MoveDownSelectionVerb *__fastcall(RBX::MoveDownSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x3fb0a0() -> ! {
    todo!("0x3fb0a0 __ZN3RBX21MoveDownSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::TurnOnManualJointCreation::TurnOnManualJointCreation(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX25TurnOnManualJointCreationC1EPNS_9DataModelE")]
// 0x3fbd8c — __ZN3RBX25TurnOnManualJointCreationC1EPNS_9DataModelE
// type: int __fastcall(RBX::TurnOnManualJointCreation *this, RBX::DataModel *)
pub fn stub_0x3fbd8c() -> ! {
    todo!("0x3fbd8c __ZN3RBX25TurnOnManualJointCreationC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::TurnOnManualJointCreation::TurnOnManualJointCreation(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX25TurnOnManualJointCreationC2EPNS_9DataModelE")]
// 0x3fbd90 — __ZN3RBX25TurnOnManualJointCreationC2EPNS_9DataModelE
// type: RBX::TurnOnManualJointCreation *__fastcall(RBX::TurnOnManualJointCreation *this, RBX::DataModel *)
pub fn stub_0x3fbd90() -> ! {
    todo!("0x3fbd90 __ZN3RBX25TurnOnManualJointCreationC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetGridToOne::SetGridToOne(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX12SetGridToOneC1EPNS_9DataModelE")]
// 0x3fbf3c — __ZN3RBX12SetGridToOneC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetGridToOne *this, RBX::DataModel *)
pub fn stub_0x3fbf3c() -> ! {
    todo!("0x3fbf3c __ZN3RBX12SetGridToOneC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetGridToOne::SetGridToOne(RBX::DataModel *)")]
#[doc(alias = "__ZN3RBX12SetGridToOneC2EPNS_9DataModelE")]
// 0x3fbf40 — __ZN3RBX12SetGridToOneC2EPNS_9DataModelE
// type: RBX::SetGridToOne *__fastcall(RBX::SetGridToOne *this, RBX::DataModel *)
pub fn stub_0x3fbf40() -> ! {
    todo!("0x3fbf40 __ZN3RBX12SetGridToOneC2EPNS_9DataModelE")
}
