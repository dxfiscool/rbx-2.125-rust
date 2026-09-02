//! core shard IN — 150 core stubs EA-sorted, 0x6c0b60..0x6dd2d4 (strict RBX|boost excluding Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|ViewController|UIApplication|Platform|iOS, EA-sorted ascending, next 150 uncovered after 0x6bfddc (global max stub_0x 0xf6fb4c, filtered 20103 total, 16077 remaining).
//! Source: ida/export.json filtered where demangled NOT containing Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|lua|ViewController|UIApplication|Platform|iOS but containing RBX:: or boost::, EA-sorted ascending, next 150 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Velocity::zero(void)")]
// 0x6c0b60 — __ZN3RBX8Velocity4zeroEv
// type: _DWORD __fastcall(RBX::Velocity *__hidden this)
// was: RBX::Velocity::zero(void)
pub fn stub_0x6c0b60() -> ! {
    todo!("0x6c0b60 __ZN3RBX8Velocity4zeroEv")
}

#[doc(alias = "RBX::IPipelined::inStage(RBX::IStage::StageType)const")]
// 0x6c2b88 — __ZNK3RBX10IPipelined7inStageENS_6IStage9StageTypeE
// type: int __fastcall(_DWORD, _DWORD)
// was: RBX::IPipelined::inStage(RBX::IStage::StageType)const
pub fn stub_0x6c2b88() -> ! {
    todo!("0x6c2b88 __ZNK3RBX10IPipelined7inStageENS_6IStage9StageTypeE")
}

#[doc(alias = "RBX::KernelJoint::~KernelJoint()")]
// 0x6c2bf4 — __ZN3RBX11KernelJointD1Ev
// type: void __fastcall(RBX::KernelJoint *__hidden this)
// was: RBX::KernelJoint::~KernelJoint()
pub fn stub_0x6c2bf4() -> ! {
    todo!("0x6c2bf4 __ZN3RBX11KernelJointD1Ev")
}

#[doc(alias = "RBX::KernelJoint::~KernelJoint()")]
// 0x6c2bf8 — __ZN3RBX11KernelJointD0Ev
// type: void __fastcall(RBX::KernelJoint *__hidden this)
// was: RBX::KernelJoint::~KernelJoint()
pub fn stub_0x6c2bf8() -> ! {
    todo!("0x6c2bf8 __ZN3RBX11KernelJointD0Ev")
}

#[doc(alias = "RBX::Joint::canStepUi(void)const")]
// 0x6c2c98 — __ZNK3RBX5Joint9canStepUiEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
// was: RBX::Joint::canStepUi(void)const
pub fn stub_0x6c2c98() -> ! {
    todo!("0x6c2c98 __ZNK3RBX5Joint9canStepUiEv")
}

#[doc(alias = "RBX::Joint::stepUi(double)")]
// 0x6c2c9c — __ZN3RBX5Joint6stepUiEd
// type: _DWORD __fastcall(RBX::Joint *__hidden this, double)
// was: RBX::Joint::stepUi(double)
pub fn stub_0x6c2c9c() -> ! {
    todo!("0x6c2c9c __ZN3RBX5Joint6stepUiEd")
}

#[doc(alias = "non-virtual thunk toRBX::KernelJoint::~KernelJoint()")]
// 0x6c2ca0 — __ZThn32_N3RBX11KernelJointD1Ev
// type: void __fastcall(RBX::KernelJoint *__hidden this)
// was: non-virtual thunk toRBX::KernelJoint::~KernelJoint()
pub fn stub_0x6c2ca0() -> ! {
    todo!("0x6c2ca0 __ZThn32_N3RBX11KernelJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::KernelJoint::~KernelJoint()")]
// 0x6c2ca8 — __ZThn32_N3RBX11KernelJointD0Ev
// type: void __fastcall(RBX::KernelJoint *__hidden this)
// was: non-virtual thunk toRBX::KernelJoint::~KernelJoint()
pub fn stub_0x6c2ca8() -> ! {
    todo!("0x6c2ca8 __ZThn32_N3RBX11KernelJointD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::KernelJoint::~KernelJoint()")]
// 0x6c2cb0 — __ZThn152_N3RBX11KernelJointD1Ev
// type: void __fastcall(RBX::KernelJoint *__hidden this)
// was: non-virtual thunk toRBX::KernelJoint::~KernelJoint()
pub fn stub_0x6c2cb0() -> ! {
    todo!("0x6c2cb0 __ZThn152_N3RBX11KernelJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::KernelJoint::~KernelJoint()")]
// 0x6c2cb8 — __ZThn152_N3RBX11KernelJointD0Ev
// type: void __fastcall(RBX::KernelJoint *__hidden this)
// was: non-virtual thunk toRBX::KernelJoint::~KernelJoint()
pub fn stub_0x6c2cb8() -> ! {
    todo!("0x6c2cb8 __ZThn152_N3RBX11KernelJointD0Ev")
}

#[doc(alias = "RBX::VirtualUser::startRecording(void)")]
// 0x6c3660 — __ZN3RBX11VirtualUser14startRecordingEv
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this)
// was: RBX::VirtualUser::startRecording(void)
pub fn stub_0x6c3660() -> ! {
    todo!("0x6c3660 __ZN3RBX11VirtualUser14startRecordingEv")
}

#[doc(alias = "RBX::VirtualUser::stopRecording(void)")]
// 0x6c389c — __ZN3RBX11VirtualUser13stopRecordingEv
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this)
// was: RBX::VirtualUser::stopRecording(void)
pub fn stub_0x6c389c() -> ! {
    todo!("0x6c389c __ZN3RBX11VirtualUser13stopRecordingEv")
}

#[doc(alias = "RBX::VirtualUser::captureInputDevice(void)")]
// 0x6c3a14 — __ZN3RBX11VirtualUser18captureInputDeviceEv
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this)
// was: RBX::VirtualUser::captureInputDevice(void)
pub fn stub_0x6c3a14() -> ! {
    todo!("0x6c3a14 __ZN3RBX11VirtualUser18captureInputDeviceEv")
}

#[doc(alias = "RBX::VirtualUser::pressKey(std::string)")]
// 0x6c3bc4 — __ZN3RBX11VirtualUser8pressKeyESs
// was: RBX::VirtualUser::pressKey(std::string)
pub fn stub_0x6c3bc4() -> ! {
    todo!("0x6c3bc4 __ZN3RBX11VirtualUser8pressKeyESs")
}

#[doc(alias = "RBX::VirtualUser::setKeyDown(std::string)")]
// 0x6c3d70 — __ZN3RBX11VirtualUser10setKeyDownESs
// was: RBX::VirtualUser::setKeyDown(std::string)
pub fn stub_0x6c3d70() -> ! {
    todo!("0x6c3d70 __ZN3RBX11VirtualUser10setKeyDownESs")
}

#[doc(alias = "RBX::VirtualUser::setKeyUp(std::string)")]
// 0x6c3dc0 — __ZN3RBX11VirtualUser8setKeyUpESs
// was: RBX::VirtualUser::setKeyUp(std::string)
pub fn stub_0x6c3dc0() -> ! {
    todo!("0x6c3dc0 __ZN3RBX11VirtualUser8setKeyUpESs")
}

#[doc(alias = "RBX::VirtualUser::VirtualUser(void)")]
// 0x6c3f58 — __ZN3RBX11VirtualUserC2Ev
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this)
// was: RBX::VirtualUser::VirtualUser(void)
pub fn stub_0x6c3f58() -> ! {
    todo!("0x6c3f58 __ZN3RBX11VirtualUserC2Ev")
}

#[doc(alias = "RBX::VirtualUser::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x6c40e0 — __ZN3RBX11VirtualUser17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this, pthread_mutex_t *, RBX::ServiceProvider *)
// was: RBX::VirtualUser::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_0x6c40e0() -> ! {
    todo!("0x6c40e0 __ZN3RBX11VirtualUser17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::VirtualUser::convert(std::string const&)")]
// 0x6c424c — __ZN3RBX11VirtualUser7convertERKSs
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this, const std::string *)
// was: RBX::VirtualUser::convert(std::string const&)
pub fn stub_0x6c424c() -> ! {
    todo!("0x6c424c __ZN3RBX11VirtualUser7convertERKSs")
}

#[doc(alias = "RBX::VirtualUser::onGuiEvent(RBX::UIEvent const&)")]
// 0x6c43e8 — __ZN3RBX11VirtualUser10onGuiEventERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this, const RBX::UIEvent *)
// was: RBX::VirtualUser::onGuiEvent(RBX::UIEvent const&)
pub fn stub_0x6c43e8() -> ! {
    todo!("0x6c43e8 __ZN3RBX11VirtualUser10onGuiEventERKNS_7UIEventE")
}

#[doc(alias = "RBX::VirtualUser::writeWait(void)")]
// 0x6c443c — __ZN3RBX11VirtualUser9writeWaitEv
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this)
// was: RBX::VirtualUser::writeWait(void)
pub fn stub_0x6c443c() -> ! {
    todo!("0x6c443c __ZN3RBX11VirtualUser9writeWaitEv")
}

#[doc(alias = "RBX::VirtualUser::writeKey(char const*,RBX::UIEvent const&)")]
// 0x6c4498 — __ZN3RBX11VirtualUser8writeKeyEPKcRKNS_7UIEventE
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this, const char *, const RBX::UIEvent *)
// was: RBX::VirtualUser::writeKey(char const*,RBX::UIEvent const&)
pub fn stub_0x6c4498() -> ! {
    todo!("0x6c4498 __ZN3RBX11VirtualUser8writeKeyEPKcRKNS_7UIEventE")
}

#[doc(alias = "RBX::VirtualUser::writeMouse(char const*,RBX::UIEvent const&)")]
// 0x6c468c — __ZN3RBX11VirtualUser10writeMouseEPKcRKNS_7UIEventE
// type: _DWORD __fastcall(RBX::VirtualUser *__hidden this, const char *, const RBX::UIEvent *)
// was: RBX::VirtualUser::writeMouse(char const*,RBX::UIEvent const&)
pub fn stub_0x6c468c() -> ! {
    todo!("0x6c468c __ZN3RBX11VirtualUser10writeMouseEPKcRKNS_7UIEventE")
}

#[doc(alias = "RBX::VirtualUser::~VirtualUser()")]
// 0x6c4b28 — __ZN3RBX11VirtualUserD1Ev
// type: void __fastcall(RBX::VirtualUser *__hidden this)
// was: RBX::VirtualUser::~VirtualUser()
pub fn stub_0x6c4b28() -> ! {
    todo!("0x6c4b28 __ZN3RBX11VirtualUserD1Ev")
}

#[doc(alias = "RBX::VirtualUser::~VirtualUser()")]
// 0x6c4c80 — __ZN3RBX11VirtualUserD0Ev
// type: void __fastcall(RBX::VirtualUser *__hidden this)
// was: RBX::VirtualUser::~VirtualUser()
pub fn stub_0x6c4c80() -> ! {
    todo!("0x6c4c80 __ZN3RBX11VirtualUserD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::VirtualUser::~VirtualUser()")]
// 0x6c4d30 — __ZThn32_N3RBX11VirtualUserD1Ev
// type: void __fastcall(RBX::VirtualUser *__hidden this)
// was: non-virtual thunk toRBX::VirtualUser::~VirtualUser()
pub fn stub_0x6c4d30() -> ! {
    todo!("0x6c4d30 __ZThn32_N3RBX11VirtualUserD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::VirtualUser::~VirtualUser()")]
// 0x6c4e84 — __ZThn32_N3RBX11VirtualUserD0Ev
// type: void __fastcall(RBX::VirtualUser *__hidden this)
// was: non-virtual thunk toRBX::VirtualUser::~VirtualUser()
pub fn stub_0x6c4e84() -> ! {
    todo!("0x6c4e84 __ZThn32_N3RBX11VirtualUserD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::VirtualUser::~VirtualUser()")]
// 0x6c5000 — __ZThn36_N3RBX11VirtualUserD1Ev
// type: void __fastcall(RBX::VirtualUser *__hidden this)
// was: non-virtual thunk toRBX::VirtualUser::~VirtualUser()
pub fn stub_0x6c5000() -> ! {
    todo!("0x6c5000 __ZThn36_N3RBX11VirtualUserD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::VirtualUser::~VirtualUser()")]
// 0x6c5154 — __ZThn36_N3RBX11VirtualUserD0Ev
// type: void __fastcall(RBX::VirtualUser *__hidden this)
// was: non-virtual thunk toRBX::VirtualUser::~VirtualUser()
pub fn stub_0x6c5154() -> ! {
    todo!("0x6c5154 __ZThn36_N3RBX11VirtualUserD0Ev")
}

#[doc(alias = "RBX::UserInputBase::~UserInputBase()")]
// 0x6c6578 — __ZN3RBX13UserInputBaseD2Ev
// type: void __fastcall(RBX::UserInputBase *__hidden this)
// was: RBX::UserInputBase::~UserInputBase()
pub fn stub_0x6c6578() -> ! {
    todo!("0x6c6578 __ZN3RBX13UserInputBaseD2Ev")
}

#[doc(alias = "RBX::VirtualHardwareDevice::getCursorPosition(void)")]
// 0x6c787c — __ZN3RBX21VirtualHardwareDevice17getCursorPositionEv
// type: _DWORD __fastcall(RBX::VirtualHardwareDevice *__hidden this)
// was: RBX::VirtualHardwareDevice::getCursorPosition(void)
pub fn stub_0x6c787c() -> ! {
    todo!("0x6c787c __ZN3RBX21VirtualHardwareDevice17getCursorPositionEv")
}

#[doc(alias = "RBX::VirtualHardwareDevice::centerCursor(void)")]
// 0x6c788c — __ZN3RBX21VirtualHardwareDevice12centerCursorEv
// type: _DWORD __fastcall(RBX::VirtualHardwareDevice *__hidden this)
// was: RBX::VirtualHardwareDevice::centerCursor(void)
pub fn stub_0x6c788c() -> ! {
    todo!("0x6c788c __ZN3RBX21VirtualHardwareDevice12centerCursorEv")
}

#[doc(alias = "RBX::VirtualHardwareDevice::keyDown(RBX::KeyCode)const")]
// 0x6c78a0 — __ZNK3RBX21VirtualHardwareDevice7keyDownENS_7KeyCodeE
// was: RBX::VirtualHardwareDevice::keyDown(RBX::KeyCode)const
pub fn stub_0x6c78a0() -> ! {
    todo!("0x6c78a0 __ZNK3RBX21VirtualHardwareDevice7keyDownENS_7KeyCodeE")
}

#[doc(alias = "RBX::VirtualHardwareDevice::setKeyState(RBX::KeyCode,bool)")]
// 0x6c78a8 — __ZN3RBX21VirtualHardwareDevice11setKeyStateENS_7KeyCodeEb
// was: RBX::VirtualHardwareDevice::setKeyState(RBX::KeyCode,bool)
pub fn stub_0x6c78a8() -> ! {
    todo!("0x6c78a8 __ZN3RBX21VirtualHardwareDevice11setKeyStateENS_7KeyCodeEb")
}

#[doc(alias = "RBX::Visit::setUploadUrl(std::string)")]
// 0x6c7f30 — __ZN3RBX5Visit12setUploadUrlESs
// was: RBX::Visit::setUploadUrl(std::string)
pub fn stub_0x6c7f30() -> ! {
    todo!("0x6c7f30 __ZN3RBX5Visit12setUploadUrlESs")
}

#[doc(alias = "RBX::Visit::setPing(std::string,int)")]
// 0x6c7f38 — __ZN3RBX5Visit7setPingESsi
// was: RBX::Visit::setPing(std::string,int)
pub fn stub_0x6c7f38() -> ! {
    todo!("0x6c7f38 __ZN3RBX5Visit7setPingESsi")
}

#[doc(alias = "RBX::Visit::Visit(void)")]
// 0x6c81c4 — __ZN3RBX5VisitC1Ev
// type: _DWORD __fastcall(RBX::Visit *__hidden this)
// was: RBX::Visit::Visit(void)
pub fn stub_0x6c81c4() -> ! {
    todo!("0x6c81c4 __ZN3RBX5VisitC1Ev")
}

#[doc(alias = "RBX::Visit::Visit(void)")]
// 0x6c81c8 — __ZN3RBX5VisitC2Ev
// type: _DWORD __fastcall(RBX::Visit *__hidden this)
// was: RBX::Visit::Visit(void)
pub fn stub_0x6c81c8() -> ! {
    todo!("0x6c81c8 __ZN3RBX5VisitC2Ev")
}

#[doc(alias = "RBX::Visit::~Visit()")]
// 0x6c8410 — __ZN3RBX5VisitD0Ev
// type: void __fastcall(RBX::Visit *__hidden this)
// was: RBX::Visit::~Visit()
pub fn stub_0x6c8410() -> ! {
    todo!("0x6c8410 __ZN3RBX5VisitD0Ev")
}

#[doc(alias = "RBX::Visit::~Visit()")]
// 0x6c84b0 — __ZN3RBX5VisitD1Ev
// type: void __fastcall(RBX::Visit *__hidden this)
// was: RBX::Visit::~Visit()
pub fn stub_0x6c84b0() -> ! {
    todo!("0x6c84b0 __ZN3RBX5VisitD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Visit::~Visit()")]
// 0x6c84b4 — __ZThn32_N3RBX5VisitD0Ev
// type: void __fastcall(RBX::Visit *__hidden this)
// was: non-virtual thunk toRBX::Visit::~Visit()
pub fn stub_0x6c84b4() -> ! {
    todo!("0x6c84b4 __ZThn32_N3RBX5VisitD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Visit::~Visit()")]
// 0x6c84bc — __ZThn36_N3RBX5VisitD0Ev
// type: void __fastcall(RBX::Visit *__hidden this)
// was: non-virtual thunk toRBX::Visit::~Visit()
pub fn stub_0x6c84bc() -> ! {
    todo!("0x6c84bc __ZThn36_N3RBX5VisitD0Ev")
}

#[doc(alias = "RBX::Visit::~Visit()")]
// 0x6c84c4 — __ZN3RBX5VisitD2Ev
// type: void __fastcall(RBX::Visit *__hidden this)
// was: RBX::Visit::~Visit()
pub fn stub_0x6c84c4() -> ! {
    todo!("0x6c84c4 __ZN3RBX5VisitD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Visit::~Visit()")]
// 0x6c85b8 — __ZThn32_N3RBX5VisitD1Ev
// type: void __fastcall(RBX::Visit *__hidden this)
// was: non-virtual thunk toRBX::Visit::~Visit()
pub fn stub_0x6c85b8() -> ! {
    todo!("0x6c85b8 __ZThn32_N3RBX5VisitD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Visit::~Visit()")]
// 0x6c85c0 — __ZThn36_N3RBX5VisitD1Ev
// type: void __fastcall(RBX::Visit *__hidden this)
// was: non-virtual thunk toRBX::Visit::~Visit()
pub fn stub_0x6c85c0() -> ! {
    todo!("0x6c85c0 __ZThn36_N3RBX5VisitD1Ev")
}

#[doc(alias = "RBX::Visit::ping(std::string,int)")]
// 0x6c85c8 — __ZN3RBX5Visit4pingESsi
// was: RBX::Visit::ping(std::string,int)
pub fn stub_0x6c85c8() -> ! {
    todo!("0x6c85c8 __ZN3RBX5Visit4pingESsi")
}

#[doc(alias = "RBX::Visit::getUploadUrl(void)")]
// 0x6c8ab4 — __ZN3RBX5Visit12getUploadUrlEv
// type: _DWORD __fastcall(RBX::Visit *__hidden this)
// was: RBX::Visit::getUploadUrl(void)
pub fn stub_0x6c8ab4() -> ! {
    todo!("0x6c8ab4 __ZN3RBX5Visit12getUploadUrlEv")
}

#[doc(alias = "RBX::Workspace::setDistributedGameTime(double)")]
// 0x6ca9b8 — __ZN3RBX9Workspace22setDistributedGameTimeEd
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, double)
// was: RBX::Workspace::setDistributedGameTime(double)
pub fn stub_0x6ca9b8() -> ! {
    todo!("0x6ca9b8 __ZN3RBX9Workspace22setDistributedGameTimeEd")
}

#[doc(alias = "RBX::Workspace::insertContent(RBX::ContentId)")]
// 0x6cae14 — __ZN3RBX9Workspace13insertContentENS_9ContentIdE
// was: RBX::Workspace::insertContent(RBX::ContentId)
pub fn stub_0x6cae14() -> ! {
    todo!("0x6cae14 __ZN3RBX9Workspace13insertContentENS_9ContentIdE")
}

#[doc(alias = "RBX::Workspace::getTerrain(void)const")]
// 0x6cb718 — __ZNK3RBX9Workspace10getTerrainEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getTerrain(void)const
pub fn stub_0x6cb718() -> ! {
    todo!("0x6cb718 __ZNK3RBX9Workspace10getTerrainEv")
}

#[doc(alias = "RBX::Workspace::zoomToExtents(void)")]
// 0x6cb720 — __ZN3RBX9Workspace13zoomToExtentsEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::zoomToExtents(void)
pub fn stub_0x6cb720() -> ! {
    todo!("0x6cb720 __ZN3RBX9Workspace13zoomToExtentsEv")
}

#[doc(alias = "RBX::Workspace::getCurrentCameraDangerous(void)const")]
// 0x6cb73c — __ZNK3RBX9Workspace25getCurrentCameraDangerousEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getCurrentCameraDangerous(void)const
pub fn stub_0x6cb73c() -> ! {
    todo!("0x6cb73c __ZNK3RBX9Workspace25getCurrentCameraDangerousEv")
}

#[doc(alias = "RBX::Workspace::setCurrentCamera(RBX::Camera *)")]
// 0x6cb744 — __ZN3RBX9Workspace16setCurrentCameraEPNS_6CameraE
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, RBX::Camera *)
// was: RBX::Workspace::setCurrentCamera(RBX::Camera *)
pub fn stub_0x6cb744() -> ! {
    todo!("0x6cb744 __ZN3RBX9Workspace16setCurrentCameraEPNS_6CameraE")
}

#[doc(alias = "RBX::Workspace::getRealPhysicsFPS(void)")]
// 0x6cb8c4 — __ZN3RBX9Workspace17getRealPhysicsFPSEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getRealPhysicsFPS(void)
pub fn stub_0x6cb8c4() -> ! {
    todo!("0x6cb8c4 __ZN3RBX9Workspace17getRealPhysicsFPSEv")
}

#[doc(alias = "RBX::Workspace::getPhysicsThrottling(void)")]
// 0x6cb8f8 — __ZN3RBX9Workspace20getPhysicsThrottlingEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getPhysicsThrottling(void)
pub fn stub_0x6cb8f8() -> ! {
    todo!("0x6cb8f8 __ZN3RBX9Workspace20getPhysicsThrottlingEv")
}

#[doc(alias = "RBX::Workspace::getNumAwakeParts(void)")]
// 0x6cb920 — __ZN3RBX9Workspace16getNumAwakePartsEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getNumAwakeParts(void)
pub fn stub_0x6cb920() -> ! {
    todo!("0x6cb920 __ZN3RBX9Workspace16getNumAwakePartsEv")
}

#[doc(alias = "RBX::Workspace::Workspace(RBX::IDataState *)")]
// 0x6cb974 — __ZN3RBX9WorkspaceC1EPNS_10IDataStateE
// type: int __fastcall(_DWORD, _DWORD)
// was: RBX::Workspace::Workspace(RBX::IDataState *)
pub fn stub_0x6cb974() -> ! {
    todo!("0x6cb974 __ZN3RBX9WorkspaceC1EPNS_10IDataStateE")
}

#[doc(alias = "RBX::Workspace::~Workspace()")]
// 0x6cc0b4 — __ZN3RBX9WorkspaceD0Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::~Workspace()
pub fn stub_0x6cc0b4() -> ! {
    todo!("0x6cc0b4 __ZN3RBX9WorkspaceD0Ev")
}

#[doc(alias = "RBX::Workspace::~Workspace()")]
// 0x6cc160 — __ZN3RBX9WorkspaceD1Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::~Workspace()
pub fn stub_0x6cc160() -> ! {
    todo!("0x6cc160 __ZN3RBX9WorkspaceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc170 — __ZThn32_N3RBX9WorkspaceD0Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc170() -> ! {
    todo!("0x6cc170 __ZThn32_N3RBX9WorkspaceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc178 — __ZThn36_N3RBX9WorkspaceD0Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc178() -> ! {
    todo!("0x6cc178 __ZThn36_N3RBX9WorkspaceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc180 — __ZThn120_N3RBX9WorkspaceD0Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc180() -> ! {
    todo!("0x6cc180 __ZThn120_N3RBX9WorkspaceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc188 — __ZThn280_N3RBX9WorkspaceD0Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc188() -> ! {
    todo!("0x6cc188 __ZThn280_N3RBX9WorkspaceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc190 — __ZThn324_N3RBX9WorkspaceD0Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc190() -> ! {
    todo!("0x6cc190 __ZThn324_N3RBX9WorkspaceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc198 — __ZThn356_N3RBX9WorkspaceD0Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc198() -> ! {
    todo!("0x6cc198 __ZThn356_N3RBX9WorkspaceD0Ev")
}

#[doc(alias = "RBX::Workspace::~Workspace()")]
// 0x6cc1a0 — __ZN3RBX9WorkspaceD2Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::~Workspace()
pub fn stub_0x6cc1a0() -> ! {
    todo!("0x6cc1a0 __ZN3RBX9WorkspaceD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc71c — __ZThn32_N3RBX9WorkspaceD1Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc71c() -> ! {
    todo!("0x6cc71c __ZThn32_N3RBX9WorkspaceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc72c — __ZThn36_N3RBX9WorkspaceD1Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc72c() -> ! {
    todo!("0x6cc72c __ZThn36_N3RBX9WorkspaceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc73c — __ZThn120_N3RBX9WorkspaceD1Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc73c() -> ! {
    todo!("0x6cc73c __ZThn120_N3RBX9WorkspaceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc74c — __ZThn280_N3RBX9WorkspaceD1Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc74c() -> ! {
    todo!("0x6cc74c __ZThn280_N3RBX9WorkspaceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc760 — __ZThn324_N3RBX9WorkspaceD1Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc760() -> ! {
    todo!("0x6cc760 __ZThn324_N3RBX9WorkspaceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::~Workspace()")]
// 0x6cc774 — __ZThn356_N3RBX9WorkspaceD1Ev
// type: void __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::~Workspace()
pub fn stub_0x6cc774() -> ! {
    todo!("0x6cc774 __ZThn356_N3RBX9WorkspaceD1Ev")
}

#[doc(alias = "RBX::Workspace::computeExtentsWorldFast(void)")]
// 0x6cc788 — __ZN3RBX9Workspace23computeExtentsWorldFastEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::computeExtentsWorldFast(void)
pub fn stub_0x6cc788() -> ! {
    todo!("0x6cc788 __ZN3RBX9Workspace23computeExtentsWorldFastEv")
}

#[doc(alias = "RBX::Workspace::onHeartbeat(RBX::Heartbeat const&)")]
// 0x6cc804 — __ZN3RBX9Workspace11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Workspace::onHeartbeat(RBX::Heartbeat const&)
pub fn stub_0x6cc804() -> ! {
    todo!("0x6cc804 __ZN3RBX9Workspace11onHeartbeatERKNS_9HeartbeatE")
}

#[doc(alias = "RBX::Workspace::replenishCamera(void)")]
// 0x6ccaa8 — __ZN3RBX9Workspace15replenishCameraEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::replenishCamera(void)
pub fn stub_0x6ccaa8() -> ! {
    todo!("0x6ccaa8 __ZN3RBX9Workspace15replenishCameraEv")
}

#[doc(alias = "RBX::Workspace::startDecalDrag(RBX::Decal *)")]
// 0x6ccda0 — __ZN3RBX9Workspace14startDecalDragEPNS_5DecalE
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, RBX::Decal *)
// was: RBX::Workspace::startDecalDrag(RBX::Decal *)
pub fn stub_0x6ccda0() -> ! {
    todo!("0x6ccda0 __ZN3RBX9Workspace14startDecalDragEPNS_5DecalE")
}

#[doc(alias = "RBX::Workspace::getCamera(void)")]
// 0x6cd45c — __ZN3RBX9Workspace9getCameraEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getCamera(void)
pub fn stub_0x6cd45c() -> ! {
    todo!("0x6cd45c __ZN3RBX9Workspace9getCameraEv")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::getCamera(void)")]
// 0x6cd464 — __ZThn280_N3RBX9Workspace9getCameraEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::getCamera(void)
pub fn stub_0x6cd464() -> ! {
    todo!("0x6cd464 __ZThn280_N3RBX9Workspace9getCameraEv")
}

#[doc(alias = "RBX::Workspace::getConstCamera(void)const")]
// 0x6cd478 — __ZNK3RBX9Workspace14getConstCameraEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getConstCamera(void)const
pub fn stub_0x6cd478() -> ! {
    todo!("0x6cd478 __ZNK3RBX9Workspace14getConstCameraEv")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::getConstCamera(void)const")]
// 0x6cd488 — __ZThn280_NK3RBX9Workspace14getConstCameraEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::getConstCamera(void)const
pub fn stub_0x6cd488() -> ! {
    todo!("0x6cd488 __ZThn280_NK3RBX9Workspace14getConstCameraEv")
}

#[doc(alias = "RBX::Workspace::createTerrain(void)")]
// 0x6cd688 — __ZN3RBX9Workspace13createTerrainEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::createTerrain(void)
pub fn stub_0x6cd688() -> ! {
    todo!("0x6cd688 __ZN3RBX9Workspace13createTerrainEv")
}

#[doc(alias = "RBX::Workspace::clearTerrain(void)")]
// 0x6cd86c — __ZN3RBX9Workspace12clearTerrainEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::clearTerrain(void)
pub fn stub_0x6cd86c() -> ! {
    todo!("0x6cd86c __ZN3RBX9Workspace12clearTerrainEv")
}

#[doc(alias = "RBX::Workspace::selectAllTopLevelRenderable(void)")]
// 0x6cd8e0 — __ZN3RBX9Workspace27selectAllTopLevelRenderableEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::selectAllTopLevelRenderable(void)
pub fn stub_0x6cd8e0() -> ! {
    todo!("0x6cd8e0 __ZN3RBX9Workspace27selectAllTopLevelRenderableEv")
}

#[doc(alias = "RBX::Workspace::joinAllHack(void)")]
// 0x6cdd98 — __ZN3RBX9Workspace11joinAllHackEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::joinAllHack(void)
pub fn stub_0x6cdd98() -> ! {
    todo!("0x6cdd98 __ZN3RBX9Workspace11joinAllHackEv")
}

#[doc(alias = "RBX::Workspace::start(void)")]
// 0x6cde50 — __ZN3RBX9Workspace5startEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::start(void)
pub fn stub_0x6cde50() -> ! {
    todo!("0x6cde50 __ZN3RBX9Workspace5startEv")
}

#[doc(alias = "RBX::Workspace::assemble(void)")]
// 0x6ce0b8 — __ZN3RBX9Workspace8assembleEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::assemble(void)
pub fn stub_0x6ce0b8() -> ! {
    todo!("0x6ce0b8 __ZN3RBX9Workspace8assembleEv")
}

#[doc(alias = "RBX::Workspace::stop(void)")]
// 0x6ce128 — __ZN3RBX9Workspace4stopEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::stop(void)
pub fn stub_0x6ce128() -> ! {
    todo!("0x6ce128 __ZN3RBX9Workspace4stopEv")
}

#[doc(alias = "RBX::Workspace::updateDistributedGameTime(void)")]
// 0x6ce398 — __ZN3RBX9Workspace25updateDistributedGameTimeEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::updateDistributedGameTime(void)
pub fn stub_0x6ce398() -> ! {
    todo!("0x6ce398 __ZN3RBX9Workspace25updateDistributedGameTimeEv")
}

#[doc(alias = "RBX::Workspace::reset(void)")]
// 0x6ce3e8 — __ZN3RBX9Workspace5resetEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::reset(void)
pub fn stub_0x6ce3e8() -> ! {
    todo!("0x6ce3e8 __ZN3RBX9Workspace5resetEv")
}

#[doc(alias = "RBX::Workspace::handleFallenParts(void)")]
// 0x6ce5b8 — __ZN3RBX9Workspace17handleFallenPartsEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::handleFallenParts(void)
pub fn stub_0x6ce5b8() -> ! {
    todo!("0x6ce5b8 __ZN3RBX9Workspace17handleFallenPartsEv")
}

#[doc(alias = "RBX::Workspace::physicsStep(bool,float,int)")]
// 0x6ce8dc — __ZN3RBX9Workspace11physicsStepEbfi
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, bool, float, int)
// was: RBX::Workspace::physicsStep(bool,float,int)
pub fn stub_0x6ce8dc() -> ! {
    todo!("0x6ce8dc __ZN3RBX9Workspace11physicsStepEbfi")
}

#[doc(alias = "RBX::Workspace::setDefaultMouseCommand(void)")]
// 0x6ceda4 — __ZN3RBX9Workspace22setDefaultMouseCommandEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::setDefaultMouseCommand(void)
pub fn stub_0x6ceda4() -> ! {
    todo!("0x6ceda4 __ZN3RBX9Workspace22setDefaultMouseCommandEv")
}

#[doc(alias = "RBX::Workspace::setNullMouseCommand(void)")]
// 0x6cee80 — __ZN3RBX9Workspace19setNullMouseCommandEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::setNullMouseCommand(void)
pub fn stub_0x6cee80() -> ! {
    todo!("0x6cee80 __ZN3RBX9Workspace19setNullMouseCommandEv")
}

#[doc(alias = "RBX::Workspace::getCursor(void)")]
// 0x6ceff0 — __ZN3RBX9Workspace9getCursorEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getCursor(void)
pub fn stub_0x6ceff0() -> ! {
    todo!("0x6ceff0 __ZN3RBX9Workspace9getCursorEv")
}

#[doc(alias = "RBX::Workspace::hasModalGuiObjects(void)")]
// 0x6cf47c — __ZN3RBX9Workspace18hasModalGuiObjectsEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::hasModalGuiObjects(void)
pub fn stub_0x6cf47c() -> ! {
    todo!("0x6cf47c __ZN3RBX9Workspace18hasModalGuiObjectsEv")
}

#[doc(alias = "RBX::Workspace::requestFirstPersonCamera(bool,bool,int)")]
// 0x6cf570 — __ZN3RBX9Workspace24requestFirstPersonCameraEbbi
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, bool, bool, int)
// was: RBX::Workspace::requestFirstPersonCamera(bool,bool,int)
pub fn stub_0x6cf570() -> ! {
    todo!("0x6cf570 __ZN3RBX9Workspace24requestFirstPersonCameraEbbi")
}

#[doc(alias = "RBX::Workspace::setRightMousePan(void)")]
// 0x6cf618 — __ZN3RBX9Workspace16setRightMousePanEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::setRightMousePan(void)
pub fn stub_0x6cf618() -> ! {
    todo!("0x6cf618 __ZN3RBX9Workspace16setRightMousePanEv")
}

#[doc(alias = "RBX::Workspace::cancelRightMousePan(void)")]
// 0x6cf648 — __ZN3RBX9Workspace19cancelRightMousePanEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::cancelRightMousePan(void)
pub fn stub_0x6cf648() -> ! {
    todo!("0x6cf648 __ZN3RBX9Workspace19cancelRightMousePanEv")
}

#[doc(alias = "RBX::Workspace::process(RBX::GuiEvent const&)")]
// 0x6cf66c — __ZN3RBX9Workspace7processERKNS_8GuiEventE
// type: int __fastcall(struct _Unwind_Exception *lpuexcpt, int, int, int, int, struct _Unwind_Exception *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Workspace::process(RBX::GuiEvent const&)
pub fn stub_0x6cf66c() -> ! {
    todo!("0x6cf66c __ZN3RBX9Workspace7processERKNS_8GuiEventE")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::process(RBX::GuiEvent const&)")]
// 0x6cfe54 — __ZThn320_N3RBX9Workspace7processERKNS_8GuiEventE
// was: non-virtual thunk toRBX::Workspace::process(RBX::GuiEvent const&)
pub fn stub_0x6cfe54() -> ! {
    todo!("0x6cfe54 __ZThn320_N3RBX9Workspace7processERKNS_8GuiEventE")
}

#[doc(alias = "RBX::Workspace::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x6cfe64 — __ZN3RBX9Workspace17onServiceProviderEPNS_15ServiceProviderES2_
// type: void __fastcall(RBX::Workspace *this, RBX::ServiceProvider *, RBX::ServiceProvider *, int)
// was: RBX::Workspace::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_0x6cfe64() -> ! {
    todo!("0x6cfe64 __ZN3RBX9Workspace17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::hash_value(RBX::TouchPair const&)")]
// 0x6d02f4 — __ZN3RBX10hash_valueERKNS_9TouchPairE
// type: int __fastcall(_DWORD)
// was: RBX::hash_value(RBX::TouchPair const&)
pub fn stub_0x6d02f4() -> ! {
    todo!("0x6d02f4 __ZN3RBX10hash_valueERKNS_9TouchPairE")
}

#[doc(alias = "RBX::Workspace::getDistributedGameTime(void)const")]
// 0x6d0328 — __ZNK3RBX9Workspace22getDistributedGameTimeEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getDistributedGameTime(void)const
pub fn stub_0x6d0328() -> ! {
    todo!("0x6d0328 __ZNK3RBX9Workspace22getDistributedGameTimeEv")
}

#[doc(alias = "RBX::Workspace::doNothing(bool)")]
// 0x6d127c — __ZN3RBX9Workspace9doNothingEb
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, bool)
// was: RBX::Workspace::doNothing(bool)
pub fn stub_0x6d127c() -> ! {
    todo!("0x6d127c __ZN3RBX9Workspace9doNothingEb")
}

#[doc(alias = "RBX::World::reset(void)")]
// 0x6d19d8 — __ZN3RBX5World5resetEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
// was: RBX::World::reset(void)
pub fn stub_0x6d19d8() -> ! {
    todo!("0x6d19d8 __ZN3RBX5World5resetEv")
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::resize(unsigned long,RBX::TouchPair)")]
// 0x6d1b68 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
// was: std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::resize(unsigned long,RBX::TouchPair)
pub fn stub_0x6d1b68() -> ! {
    todo!("0x6d1b68 __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "RBX::WorkspaceStatsItem::create(RBX::Workspace const*,RBX::World const*,RBX::RunService const*)")]
// 0x6d2338 — __ZN3RBX18WorkspaceStatsItem6createEPKNS_9WorkspaceEPKNS_5WorldEPKNS_10RunServiceE
// type: _DWORD __fastcall(RBX::WorkspaceStatsItem *__hidden this, const RBX::Workspace *, const RBX::World *, const RBX::RunService *)
// was: RBX::WorkspaceStatsItem::create(RBX::Workspace const*,RBX::World const*,RBX::RunService const*)
pub fn stub_0x6d2338() -> ! {
    todo!("0x6d2338 __ZN3RBX18WorkspaceStatsItem6createEPKNS_9WorkspaceEPKNS_5WorldEPKNS_10RunServiceE")
}

#[doc(alias = "RBX::Workspace::getCameraOwnerModel(void)const")]
// 0x6d2d18 — __ZNK3RBX9Workspace19getCameraOwnerModelEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getCameraOwnerModel(void)const
pub fn stub_0x6d2d18() -> ! {
    todo!("0x6d2d18 __ZNK3RBX9Workspace19getCameraOwnerModelEv")
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::getCameraOwnerModel(void)const")]
// 0x6d2d84 — __ZThn280_NK3RBX9Workspace19getCameraOwnerModelEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: non-virtual thunk toRBX::Workspace::getCameraOwnerModel(void)const
pub fn stub_0x6d2d84() -> ! {
    todo!("0x6d2d84 __ZThn280_NK3RBX9Workspace19getCameraOwnerModelEv")
}

#[doc(alias = "RBX::Stats::StatsService::StatsService(void)")]
// 0x6d3ed8 — __ZN3RBX5Stats12StatsServiceC2Ev
// type: _DWORD __fastcall(RBX::Stats::StatsService *__hidden this)
// was: RBX::Stats::StatsService::StatsService(void)
pub fn stub_0x6d3ed8() -> ! {
    todo!("0x6d3ed8 __ZN3RBX5Stats12StatsServiceC2Ev")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Selection>(void)")]
// 0x6d4658 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9SelectionEEEvv
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::Selection>(void)
pub fn stub_0x6d4658() -> ! {
    todo!("0x6d4658 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9SelectionEEEvv")
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TouchPair*,std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>>,unsigned long,RBX::TouchPair const&)")]
// 0x6d48e4 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int, int)
// was: std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TouchPair*,std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>>,unsigned long,RBX::TouchPair const&)
pub fn stub_0x6d48e4() -> ! {
    todo!("0x6d48e4 __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "void std::fill<RBX::TouchPair *,RBX::TouchPair>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair const&)")]
// 0x6d4cd8 — __ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_
// was: void std::fill<RBX::TouchPair *,RBX::TouchPair>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair const&)
pub fn stub_0x6d4cd8() -> ! {
    todo!("0x6d4cd8 __ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_")
}

#[doc(alias = "std::_Vector_base<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_allocate(unsigned long)")]
// 0x6d4d10 — __ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm
// was: std::_Vector_base<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_allocate(unsigned long)
pub fn stub_0x6d4d10() -> ! {
    todo!("0x6d4d10 __ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::TouchPair *,unsigned long,RBX::TouchPair>(RBX::TouchPair *,unsigned long,RBX::TouchPair const&,std::__false_type)")]
// 0x6d4d34 — __ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
// was: void std::__uninitialized_fill_n_aux<RBX::TouchPair *,unsigned long,RBX::TouchPair>(RBX::TouchPair *,unsigned long,RBX::TouchPair const&,std::__false_type)
pub fn stub_0x6d4d34() -> ! {
    todo!("0x6d4d34 __ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "RBX::TouchPair::operator=(RBX::TouchPair const&)")]
// 0x6d4eec — __ZN3RBX9TouchPairaSERKS0_
// was: RBX::TouchPair::operator=(RBX::TouchPair const&)
pub fn stub_0x6d4eec() -> ! {
    todo!("0x6d4eec __ZN3RBX9TouchPairaSERKS0_")
}

#[doc(alias = "RBX::TouchPair * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *)")]
// 0x6d4f0c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_
// was: RBX::TouchPair * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *)
pub fn stub_0x6d4f0c() -> ! {
    todo!("0x6d4f0c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_")
}

#[doc(alias = "RBX::TouchPair * std::__uninitialized_copy_aux<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *,std::__false_type)")]
// 0x6d4f68 — __ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
// was: RBX::TouchPair * std::__uninitialized_copy_aux<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *,std::__false_type)
pub fn stub_0x6d4f68() -> ! {
    todo!("0x6d4f68 __ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type")
}

#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::~vector()")]
// 0x6d5144 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev
// was: std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::~vector()
pub fn stub_0x6d5144() -> ! {
    todo!("0x6d5144 __ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::StarterPackService>(void)")]
// 0x6d5f20 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18StarterPackServiceEEEvv
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::StarterPackService>(void)
pub fn stub_0x6d5f20() -> ! {
    todo!("0x6d5f20 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18StarterPackServiceEEEvv")
}

#[doc(alias = "RBX::DecalTool::DecalTool(RBX::Workspace *,RBX::Decal *)")]
// 0x6d65c0 — __ZN3RBX9DecalToolC2EPNS_9WorkspaceEPNS_5DecalE
// type: _DWORD __fastcall(RBX::DecalTool *__hidden this, RBX::Workspace *, RBX::Decal *)
// was: RBX::DecalTool::DecalTool(RBX::Workspace *,RBX::Decal *)
pub fn stub_0x6d65c0() -> ! {
    todo!("0x6d65c0 __ZN3RBX9DecalToolC2EPNS_9WorkspaceEPNS_5DecalE")
}

#[doc(alias = "RBX::MouseCommand::onRightMouseDown(RBX::UIEvent const&)")]
// 0x6d6820 — __ZN3RBX12MouseCommand16onRightMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onRightMouseDown(RBX::UIEvent const&)
pub fn stub_0x6d6820() -> ! {
    todo!("0x6d6820 __ZN3RBX12MouseCommand16onRightMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onMouseIdle(RBX::UIEvent const&)")]
// 0x6d682c — __ZN3RBX12MouseCommand11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onMouseIdle(RBX::UIEvent const&)
pub fn stub_0x6d682c() -> ! {
    todo!("0x6d682c __ZN3RBX12MouseCommand11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onKeyUp(RBX::UIEvent const&)")]
// 0x6d6830 — __ZN3RBX12MouseCommand7onKeyUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onKeyUp(RBX::UIEvent const&)
pub fn stub_0x6d6830() -> ! {
    todo!("0x6d6830 __ZN3RBX12MouseCommand7onKeyUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onPeekKeyDown(RBX::UIEvent const&)")]
// 0x6d683c — __ZN3RBX12MouseCommand13onPeekKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onPeekKeyDown(RBX::UIEvent const&)
pub fn stub_0x6d683c() -> ! {
    todo!("0x6d683c __ZN3RBX12MouseCommand13onPeekKeyDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onPeekKeyUp(RBX::UIEvent const&)")]
// 0x6d6848 — __ZN3RBX12MouseCommand11onPeekKeyUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onPeekKeyUp(RBX::UIEvent const&)
pub fn stub_0x6d6848() -> ! {
    todo!("0x6d6848 __ZN3RBX12MouseCommand11onPeekKeyUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onMouseDelta(RBX::UIEvent const&)")]
// 0x6d6854 — __ZN3RBX12MouseCommand12onMouseDeltaERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onMouseDelta(RBX::UIEvent const&)
pub fn stub_0x6d6854() -> ! {
    todo!("0x6d6854 __ZN3RBX12MouseCommand12onMouseDeltaERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onRightMouseUp(RBX::UIEvent const&)")]
// 0x6d6858 — __ZN3RBX12MouseCommand14onRightMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onRightMouseUp(RBX::UIEvent const&)
pub fn stub_0x6d6858() -> ! {
    todo!("0x6d6858 __ZN3RBX12MouseCommand14onRightMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onMouseWheelForward(RBX::UIEvent const&)")]
// 0x6d6864 — __ZN3RBX12MouseCommand19onMouseWheelForwardERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onMouseWheelForward(RBX::UIEvent const&)
pub fn stub_0x6d6864() -> ! {
    todo!("0x6d6864 __ZN3RBX12MouseCommand19onMouseWheelForwardERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onMouseWheelBackward(RBX::UIEvent const&)")]
// 0x6d6870 — __ZN3RBX12MouseCommand20onMouseWheelBackwardERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onMouseWheelBackward(RBX::UIEvent const&)
pub fn stub_0x6d6870() -> ! {
    todo!("0x6d6870 __ZN3RBX12MouseCommand20onMouseWheelBackwardERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::releaseCapture(void)")]
// 0x6d687c — __ZN3RBX12MouseCommand14releaseCaptureEv
// type: int __fastcall(int this)
// was: RBX::MouseCommand::releaseCapture(void)
pub fn stub_0x6d687c() -> ! {
    todo!("0x6d687c __ZN3RBX12MouseCommand14releaseCaptureEv")
}

#[doc(alias = "RBX::MouseCommand::cancel(void)")]
// 0x6d6884 — __ZN3RBX12MouseCommand6cancelEv
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this)
// was: RBX::MouseCommand::cancel(void)
pub fn stub_0x6d6884() -> ! {
    todo!("0x6d6884 __ZN3RBX12MouseCommand6cancelEv")
}

#[doc(alias = "RBX::MouseCommand::isSticky(void)const")]
// 0x6d6894 — __ZNK3RBX12MouseCommand8isStickyEv
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this)
// was: RBX::MouseCommand::isSticky(void)const
pub fn stub_0x6d6894() -> ! {
    todo!("0x6d6894 __ZNK3RBX12MouseCommand8isStickyEv")
}

#[doc(alias = "RBX::MouseCommand::drawConnectors(void)const")]
// 0x6d68a0 — __ZNK3RBX12MouseCommand14drawConnectorsEv
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this)
// was: RBX::MouseCommand::drawConnectors(void)const
pub fn stub_0x6d68a0() -> ! {
    todo!("0x6d68a0 __ZNK3RBX12MouseCommand14drawConnectorsEv")
}

#[doc(alias = "RBX::DecalTool::getCursorName(void)const")]
// 0x6d68a4 — __ZNK3RBX9DecalTool13getCursorNameEv
// type: _DWORD __fastcall(RBX::DecalTool *__hidden this)
// was: RBX::DecalTool::getCursorName(void)const
pub fn stub_0x6d68a4() -> ! {
    todo!("0x6d68a4 __ZNK3RBX9DecalTool13getCursorNameEv")
}

#[doc(alias = "RBX::DecalTool::doAction(RBX::Surface *)")]
// 0x6d68c0 — __ZN3RBX9DecalTool8doActionEPNS_7SurfaceE
// type: _DWORD __fastcall(RBX::DecalTool *__hidden this, RBX::Surface *)
// was: RBX::DecalTool::doAction(RBX::Surface *)
pub fn stub_0x6d68c0() -> ! {
    todo!("0x6d68c0 __ZN3RBX9DecalTool8doActionEPNS_7SurfaceE")
}

#[doc(alias = "RBX::MouseCommand::onKeyDown(RBX::UIEvent const&)")]
// 0x6d6b5c — __ZN3RBX12MouseCommand9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onKeyDown(RBX::UIEvent const&)
pub fn stub_0x6d6b5c() -> ! {
    todo!("0x6d6b5c __ZN3RBX12MouseCommand9onKeyDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onMouseMove(RBX::UIEvent const&)")]
// 0x6d6b68 — __ZN3RBX12MouseCommand11onMouseMoveERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onMouseMove(RBX::UIEvent const&)
pub fn stub_0x6d6b68() -> ! {
    todo!("0x6d6b68 __ZN3RBX12MouseCommand11onMouseMoveERKNS_7UIEventE")
}

#[doc(alias = "RBX::MouseCommand::onMouseUp(RBX::UIEvent const&)")]
// 0x6d6b6c — __ZN3RBX12MouseCommand9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::MouseCommand *__hidden this, const RBX::UIEvent *)
// was: RBX::MouseCommand::onMouseUp(RBX::UIEvent const&)
pub fn stub_0x6d6b6c() -> ! {
    todo!("0x6d6b6c __ZN3RBX12MouseCommand9onMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dcee0 — __ZN3RBX12AdvArrowToolD1Ev
// type: void __fastcall(RBX::AdvArrowTool *__hidden this)
// was: RBX::AdvArrowTool::~AdvArrowTool()
pub fn stub_0x6dcee0() -> ! {
    todo!("0x6dcee0 __ZN3RBX12AdvArrowToolD1Ev")
}

#[doc(alias = "RBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dcee4 — __ZN3RBX12AdvArrowToolD0Ev
// type: void __fastcall(RBX::AdvArrowTool *__hidden this)
// was: RBX::AdvArrowTool::~AdvArrowTool()
pub fn stub_0x6dcee4() -> ! {
    todo!("0x6dcee4 __ZN3RBX12AdvArrowToolD0Ev")
}

#[doc(alias = "RBX::AdvArrowTool::isSticky(void)const")]
// 0x6dcf84 — __ZNK3RBX12AdvArrowTool8isStickyEv
// type: _DWORD __fastcall(RBX::AdvArrowTool *__hidden this)
// was: RBX::AdvArrowTool::isSticky(void)const
pub fn stub_0x6dcf84() -> ! {
    todo!("0x6dcf84 __ZNK3RBX12AdvArrowTool8isStickyEv")
}

#[doc(alias = "RBX::AdvArrowToolBase::getSelectedTargetPrimitives(std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)")]
// 0x6dd04c — __ZN3RBX16AdvArrowToolBase27getSelectedTargetPrimitivesERSt6vectorIPNS_9PrimitiveESaIS3_EE
// was: RBX::AdvArrowToolBase::getSelectedTargetPrimitives(std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>> &)
pub fn stub_0x6dd04c() -> ! {
    todo!("0x6dd04c __ZN3RBX16AdvArrowToolBase27getSelectedTargetPrimitivesERSt6vectorIPNS_9PrimitiveESaIS3_EE")
}

#[doc(alias = "RBX::AdvArrowTool::setCursor(std::string)")]
// 0x6dd050 — __ZN3RBX12AdvArrowTool9setCursorESs
// was: RBX::AdvArrowTool::setCursor(std::string)
pub fn stub_0x6dd050() -> ! {
    todo!("0x6dd050 __ZN3RBX12AdvArrowTool9setCursorESs")
}

#[doc(alias = "non-virtual thunk toRBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dd054 — __ZThn36_N3RBX12AdvArrowToolD1Ev
// type: void __fastcall(RBX::AdvArrowTool *__hidden this)
// was: non-virtual thunk toRBX::AdvArrowTool::~AdvArrowTool()
pub fn stub_0x6dd054() -> ! {
    todo!("0x6dd054 __ZThn36_N3RBX12AdvArrowToolD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AdvArrowTool::~AdvArrowTool()")]
// 0x6dd05c — __ZThn36_N3RBX12AdvArrowToolD0Ev
// type: void __fastcall(RBX::AdvArrowTool *__hidden this)
// was: non-virtual thunk toRBX::AdvArrowTool::~AdvArrowTool()
pub fn stub_0x6dd05c() -> ! {
    todo!("0x6dd05c __ZThn36_N3RBX12AdvArrowToolD0Ev")
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd100 — __ZN3RBX16AdvArrowToolBaseD2Ev
// type: void __fastcall(RBX::AdvArrowToolBase *__hidden this)
// was: RBX::AdvArrowToolBase::~AdvArrowToolBase()
pub fn stub_0x6dd100() -> ! {
    todo!("0x6dd100 __ZN3RBX16AdvArrowToolBaseD2Ev")
}

#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase()")]
// 0x6dd1dc — __ZN3RBX13ArrowToolBaseD2Ev
// type: void __fastcall(RBX::ArrowToolBase *__hidden this)
// was: RBX::ArrowToolBase::~ArrowToolBase()
pub fn stub_0x6dd1dc() -> ! {
    todo!("0x6dd1dc __ZN3RBX13ArrowToolBaseD2Ev")
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd2d0 — __ZN3RBX16AdvArrowToolBaseD1Ev
// type: void __fastcall(RBX::AdvArrowToolBase *__hidden this)
// was: RBX::AdvArrowToolBase::~AdvArrowToolBase()
pub fn stub_0x6dd2d0() -> ! {
    todo!("0x6dd2d0 __ZN3RBX16AdvArrowToolBaseD1Ev")
}

#[doc(alias = "RBX::AdvArrowToolBase::~AdvArrowToolBase()")]
// 0x6dd2d4 — __ZN3RBX16AdvArrowToolBaseD0Ev
// type: void __fastcall(RBX::AdvArrowToolBase *__hidden this)
// was: RBX::AdvArrowToolBase::~AdvArrowToolBase()
pub fn stub_0x6dd2d4() -> ! {
    todo!("0x6dd2d4 __ZN3RBX16AdvArrowToolBaseD0Ev")
}

