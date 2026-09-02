//! platform — generated_plat_ios_gap_1788371189 — 120 stubs EA-sorted asc gap filler distinct (iOS filter)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 filtered iOS|Controller|View|RobloxView not yet in crates/platform/src
//! Filter: iOS|Controller|View|RobloxView total 2826, platform covered 41540 -> 41660 filtered covered 1879->1999 | range 0x3c3820..0x65ae24 | rbx_core::SharedPtr not boost
//! Batch: 120 stubs | EA-sorted asc gap filler distinct filtered iOS not yet in platform | rbx_core::SharedPtr not boost | // 0xADDR — mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x3c3820 — __ZN3RBX6Camera21setFieldOfViewDegreesEf — RBX::Camera::setFieldOfViewDegrees(float)
// type: void __fastcall(RBX::Camera *this, float32_t)
#[doc(alias = "__ZN3RBX6Camera21setFieldOfViewDegreesEf")]
#[doc(alias = "RBX::Camera::setFieldOfViewDegrees(float)")]
pub fn stub_3c3820() -> ! {
    todo!("0x3c3820 __ZN3RBX6Camera21setFieldOfViewDegreesEf")
}

// 0x3c5284 — __ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_ — RBX::Camera::getNearViewportCorners(G3D::Rect2D const&,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const
// type: int __fastcall(int result, __int32 *, __int32 *, __int32 *, __int32 *, __int32 *)
#[doc(alias = "__ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_")]
#[doc(alias = "RBX::Camera::getNearViewportCorners(G3D::Rect2D const&,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
pub fn stub_3c5284() -> ! {
    todo!("0x3c5284 __ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_")
}

// 0x3c8c3c — __ZNK3RBX6Camera21getFieldOfViewDegreesEv — RBX::Camera::getFieldOfViewDegrees(void)const
// type: float __fastcall(RBX::Camera *this)
#[doc(alias = "__ZNK3RBX6Camera21getFieldOfViewDegreesEv")]
#[doc(alias = "RBX::Camera::getFieldOfViewDegrees(void)const")]
pub fn stub_3c8c3c() -> ! {
    todo!("0x3c8c3c __ZNK3RBX6Camera21getFieldOfViewDegreesEv")
}

// 0x3c9c64 — __ZN3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_PKNS_8InstanceE — RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(RBX::Instance const*)
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_PKNS_8InstanceE")]
#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::create<RBX::ControllerService>(RBX::Instance const*)")]
pub fn stub_3c9c64() -> ! {
    todo!("0x3c9c64 __ZN3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_PKNS_8InstanceE")
}

// 0x5d1aa4 — __ZNK3RBX5Mouse12getViewSizeXEv — RBX::Mouse::getViewSizeX(void)const
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "__ZNK3RBX5Mouse12getViewSizeXEv")]
#[doc(alias = "RBX::Mouse::getViewSizeX(void)const")]
pub fn stub_5d1aa4() -> ! {
    todo!("0x5d1aa4 __ZNK3RBX5Mouse12getViewSizeXEv")
}

// 0x5d1abc — __ZNK3RBX5Mouse12getViewSizeYEv — RBX::Mouse::getViewSizeY(void)const
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "__ZNK3RBX5Mouse12getViewSizeYEv")]
#[doc(alias = "RBX::Mouse::getViewSizeY(void)const")]
pub fn stub_5d1abc() -> ! {
    todo!("0x5d1abc __ZNK3RBX5Mouse12getViewSizeYEv")
}

// 0x5df6c8 — __ZN3RBX12PartInstance15setSurfaceInputENS_8NormalIdENS_16LegacyController9InputTypeE — RBX::PartInstance::setSurfaceInput(RBX::NormalId,RBX::LegacyController::InputType)
// type: int __fastcall(int, int)
#[doc(alias = "__ZN3RBX12PartInstance15setSurfaceInputENS_8NormalIdENS_16LegacyController9InputTypeE")]
#[doc(alias = "RBX::PartInstance::setSurfaceInput(RBX::NormalId,RBX::LegacyController::InputType)")]
pub fn stub_5df6c8() -> ! {
    todo!("0x5df6c8 __ZN3RBX12PartInstance15setSurfaceInputENS_8NormalIdENS_16LegacyController9InputTypeE")
}

// 0x60c7dc — __ZN3RBX12RootInstance25insertRemoteCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EEPKN3G3D7Vector3E — RBX::RootInstance::insertRemoteCharacterView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,G3D::Vector3 const*)
// type: int __fastcall(RBX::Instance *this, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX12RootInstance25insertRemoteCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EEPKN3G3D7Vector3E")]
#[doc(alias = "RBX::RootInstance::insertRemoteCharacterView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,G3D::Vector3 const*)")]
pub fn stub_60c7dc() -> ! {
    todo!("0x60c7dc __ZN3RBX12RootInstance25insertRemoteCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EEPKN3G3D7Vector3E")
}

// 0x60c86c — __ZN3RBX12RootInstance19insertCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EE — RBX::RootInstance::insertCharacterView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)
// type: int __fastcall(RBX::Instance *this, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX12RootInstance19insertCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EE")]
#[doc(alias = "RBX::RootInstance::insertCharacterView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>)")]
pub fn stub_60c86c() -> ! {
    todo!("0x60c86c __ZN3RBX12RootInstance19insertCharacterViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EE")
}

// 0x60c94c — __ZN3RBX12RootInstance13insertIdeViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EENS_10PromptModeEb — RBX::RootInstance::insertIdeView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,RBX::PromptMode,bool)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX12RootInstance13insertIdeViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EENS_10PromptModeEb")]
#[doc(alias = "RBX::RootInstance::insertIdeView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,std::vector&<boost::weak_ptr<RBX::PartInstance>,std::allocator<RBX::PartInstance>>,RBX::PromptMode,bool)")]
pub fn stub_60c94c() -> ! {
    todo!("0x60c94c __ZN3RBX12RootInstance13insertIdeViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EERS1_INS2_8weak_ptrINS_12PartInstanceEEESaISC_EENS_10PromptModeEb")
}

// 0x60ca5c — __ZN3RBX12RootInstance12insert3dViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_10PromptModeEbPKN3G3D7Vector3E — RBX::RootInstance::insert3dView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::PromptMode,bool,G3D::Vector3 const*)
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, void *, int, int, char, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX12RootInstance12insert3dViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_10PromptModeEbPKN3G3D7Vector3E")]
#[doc(alias = "RBX::RootInstance::insert3dView(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const&,RBX::PromptMode,bool,G3D::Vector3 const*)")]
pub fn stub_60ca5c() -> ! {
    todo!("0x60ca5c __ZN3RBX12RootInstance12insert3dViewERKSt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EENS_10PromptModeEbPKN3G3D7Vector3E")
}

// 0x6245b8 — __ZN3RBX20SkateboardControllerC1Ev — RBX::SkateboardController::SkateboardController(void)
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZN3RBX20SkateboardControllerC1Ev")]
#[doc(alias = "RBX::SkateboardController::SkateboardController(void)")]
pub fn stub_6245b8() -> ! {
    todo!("0x6245b8 __ZN3RBX20SkateboardControllerC1Ev")
}

// 0x6245bc — __ZN3RBX20SkateboardControllerC2Ev — RBX::SkateboardController::SkateboardController(void)
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZN3RBX20SkateboardControllerC2Ev")]
#[doc(alias = "RBX::SkateboardController::SkateboardController(void)")]
pub fn stub_6245bc() -> ! {
    todo!("0x6245bc __ZN3RBX20SkateboardControllerC2Ev")
}

// 0x624878 — __ZN3RBX20SkateboardController19onSteppedTouchInputEv — RBX::SkateboardController::onSteppedTouchInput(void)
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZN3RBX20SkateboardController19onSteppedTouchInputEv")]
#[doc(alias = "RBX::SkateboardController::onSteppedTouchInput(void)")]
pub fn stub_624878() -> ! {
    todo!("0x624878 __ZN3RBX20SkateboardController19onSteppedTouchInputEv")
}

// 0x624a50 — __ZN3RBX20SkateboardController11setThrottleEf — RBX::SkateboardController::setThrottle(float)
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this, float)
#[doc(alias = "__ZN3RBX20SkateboardController11setThrottleEf")]
#[doc(alias = "RBX::SkateboardController::setThrottle(float)")]
pub fn stub_624a50() -> ! {
    todo!("0x624a50 __ZN3RBX20SkateboardController11setThrottleEf")
}

// 0x624b9c — __ZN3RBX20SkateboardController8setSteerEf — RBX::SkateboardController::setSteer(float)
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this, float)
#[doc(alias = "__ZN3RBX20SkateboardController8setSteerEf")]
#[doc(alias = "RBX::SkateboardController::setSteer(float)")]
pub fn stub_624b9c() -> ! {
    todo!("0x624b9c __ZN3RBX20SkateboardController8setSteerEf")
}

// 0x624ce8 — __ZN3RBX20SkateboardController22onSteppedKeyboardInputEv — RBX::SkateboardController::onSteppedKeyboardInput(void)
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZN3RBX20SkateboardController22onSteppedKeyboardInputEv")]
#[doc(alias = "RBX::SkateboardController::onSteppedKeyboardInput(void)")]
pub fn stub_624ce8() -> ! {
    todo!("0x624ce8 __ZN3RBX20SkateboardController22onSteppedKeyboardInputEv")
}

// 0x624f08 — __ZN3RBX20SkateboardController9onSteppedERKNS_7SteppedE — RBX::SkateboardController::onStepped(RBX::Stepped const&)
#[doc(alias = "__ZN3RBX20SkateboardController9onSteppedERKNS_7SteppedE")]
#[doc(alias = "RBX::SkateboardController::onStepped(RBX::Stepped const&)")]
pub fn stub_624f08() -> ! {
    todo!("0x624f08 __ZN3RBX20SkateboardController9onSteppedERKNS_7SteppedE")
}

// 0x624f38 — __ZThn92_N3RBX20SkateboardController9onSteppedERKNS_7SteppedE — `non-virtual thunk to'RBX::SkateboardController::onStepped(RBX::Stepped const&)
#[doc(alias = "__ZThn92_N3RBX20SkateboardController9onSteppedERKNS_7SteppedE")]
#[doc(alias = "non-virtual thunk toRBX::SkateboardController::onStepped(RBX::Stepped const&)")]
pub fn stub_624f38() -> ! {
    todo!("0x624f38 __ZThn92_N3RBX20SkateboardController9onSteppedERKNS_7SteppedE")
}

// 0x625078 — __ZNK3RBX20SkateboardController11getThrottleEv — RBX::SkateboardController::getThrottle(void)const
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZNK3RBX20SkateboardController11getThrottleEv")]
#[doc(alias = "RBX::SkateboardController::getThrottle(void)const")]
pub fn stub_625078() -> ! {
    todo!("0x625078 __ZNK3RBX20SkateboardController11getThrottleEv")
}

// 0x625080 — __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED1Ev — RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::~PropDescriptor()
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::~PropDescriptor()")]
pub fn stub_625080() -> ! {
    todo!("0x625080 __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED1Ev")
}

// 0x6250a4 — __ZNK3RBX20SkateboardController8getSteerEv — RBX::SkateboardController::getSteer(void)const
// type: _DWORD __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZNK3RBX20SkateboardController8getSteerEv")]
#[doc(alias = "RBX::SkateboardController::getSteer(void)const")]
pub fn stub_6250a4() -> ! {
    todo!("0x6250a4 __ZNK3RBX20SkateboardController8getSteerEv")
}

// 0x6250ac — __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev — RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::~EventDesc()
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::~EventDesc()")]
pub fn stub_6250ac() -> ! {
    todo!("0x6250ac __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev")
}

// 0x625240 — __ZN3RBX20SkateboardControllerD1Ev — RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZN3RBX20SkateboardControllerD1Ev")]
#[doc(alias = "RBX::SkateboardController::~SkateboardController()")]
pub fn stub_625240() -> ! {
    todo!("0x625240 __ZN3RBX20SkateboardControllerD1Ev")
}

// 0x625384 — __ZN3RBX20SkateboardControllerD0Ev — RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZN3RBX20SkateboardControllerD0Ev")]
#[doc(alias = "RBX::SkateboardController::~SkateboardController()")]
pub fn stub_625384() -> ! {
    todo!("0x625384 __ZN3RBX20SkateboardControllerD0Ev")
}

// 0x625424 — __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv — __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv")]
pub fn stub_625424() -> ! {
    todo!("0x625424 __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv")
}

// 0x625434 — __ZThn32_N3RBX20SkateboardControllerD1Ev — `non-virtual thunk to'RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZThn32_N3RBX20SkateboardControllerD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SkateboardController::~SkateboardController()")]
pub fn stub_625434() -> ! {
    todo!("0x625434 __ZThn32_N3RBX20SkateboardControllerD1Ev")
}

// 0x625574 — __ZThn32_N3RBX20SkateboardControllerD0Ev — `non-virtual thunk to'RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZThn32_N3RBX20SkateboardControllerD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SkateboardController::~SkateboardController()")]
pub fn stub_625574() -> ! {
    todo!("0x625574 __ZThn32_N3RBX20SkateboardControllerD0Ev")
}

// 0x6256cc — __ZThn32_NK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv — __ZThn32_NK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv")]
pub fn stub_6256cc() -> ! {
    todo!("0x6256cc __ZThn32_NK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE12getClassNameEv")
}

// 0x6256dc — __ZThn36_N3RBX20SkateboardControllerD1Ev — `non-virtual thunk to'RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZThn36_N3RBX20SkateboardControllerD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SkateboardController::~SkateboardController()")]
pub fn stub_6256dc() -> ! {
    todo!("0x6256dc __ZThn36_N3RBX20SkateboardControllerD1Ev")
}

// 0x62581c — __ZThn36_N3RBX20SkateboardControllerD0Ev — `non-virtual thunk to'RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZThn36_N3RBX20SkateboardControllerD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SkateboardController::~SkateboardController()")]
pub fn stub_62581c() -> ! {
    todo!("0x62581c __ZThn36_N3RBX20SkateboardControllerD0Ev")
}

// 0x625974 — __ZThn92_N3RBX20SkateboardControllerD1Ev — `non-virtual thunk to'RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZThn92_N3RBX20SkateboardControllerD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SkateboardController::~SkateboardController()")]
pub fn stub_625974() -> ! {
    todo!("0x625974 __ZThn92_N3RBX20SkateboardControllerD1Ev")
}

// 0x625ab4 — __ZThn92_N3RBX20SkateboardControllerD0Ev — `non-virtual thunk to'RBX::SkateboardController::~SkateboardController()
// type: void __fastcall(RBX::SkateboardController *__hidden this)
#[doc(alias = "__ZThn92_N3RBX20SkateboardControllerD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SkateboardController::~SkateboardController()")]
pub fn stub_625ab4() -> ! {
    todo!("0x625ab4 __ZThn92_N3RBX20SkateboardControllerD0Ev")
}

// 0x625c0c — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD1Ev — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_625c0c() -> ! {
    todo!("0x625c0c __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD1Ev")
}

// 0x625c10 — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD2Ev — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_625c10() -> ! {
    todo!("0x625c10 __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorD2Ev")
}

// 0x625cac — __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator12getClassNameEv — __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_625cac() -> ! {
    todo!("0x625cac __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x625d34 — __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator6createEv — __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator6createEv")]
pub fn stub_625d34() -> ! {
    todo!("0x625d34 __ZNK3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7Creator6createEv")
}

// 0x625e78 — __ZN3RBX4Name13callDoDeclareILZNS_21sSkateboardControllerEEEEvv — __ZN3RBX4Name13callDoDeclareILZNS_21sSkateboardControllerEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sSkateboardControllerEEEEvv")]
pub fn stub_625e78() -> ! {
    todo!("0x625e78 __ZN3RBX4Name13callDoDeclareILZNS_21sSkateboardControllerEEEEvv")
}

// 0x625e7c — __ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v — __ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v")]
pub fn stub_625e7c() -> ! {
    todo!("0x625e7c __ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v")
}

// 0x625f5c — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorC2Ev — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_625f5c() -> ! {
    todo!("0x625f5c __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE7CreatorC2Ev")
}

// 0x6261a0 — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE17static_getCreatorEv — __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_6261a0() -> ! {
    todo!("0x6261a0 __ZN3RBX14FactoryProductINS_20SkateboardControllerENS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEE17static_getCreatorEv")
}

// 0x626290 — __ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev — __ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_626290() -> ! {
    todo!("0x626290 __ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x626294 — __ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev — __ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_626294() -> ! {
    todo!("0x626294 __ZN3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x626334 — __ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev — __ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_626334() -> ! {
    todo!("0x626334 __ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x62633c — __ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev — __ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_62633c() -> ! {
    todo!("0x62633c __ZThn32_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6263e0 — __ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev — __ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_6263e0() -> ! {
    todo!("0x6263e0 __ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6263e8 — __ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev — __ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_6263e8() -> ! {
    todo!("0x6263e8 __ZThn36_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x62648c — __ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev — __ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_62648c() -> ! {
    todo!("0x62648c __ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x626494 — __ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev — __ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_626494() -> ! {
    todo!("0x626494 __ZThn92_N3RBX10Reflection9DescribedINS_20SkateboardControllerELZNS_21sSkateboardControllerEENS_14FactoryProductIS2_NS_10ControllerELZNS_21sSkateboardControllerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x626538 — __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE — RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::SkateboardController::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::SkateboardController::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_626538() -> ! {
    todo!("0x626538 __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x6266bc — __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev — RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::~EventDesc()
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::~EventDesc()")]
pub fn stub_6266bc() -> ! {
    todo!("0x6266bc __ZN3RBX10Reflection9EventDescINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_ED0Ev")
}

// 0x626770 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE — RBX::Reflection::EventDescImpl<1,RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_626770() -> ! {
    todo!("0x626770 __ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

// 0x6268c4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE — RBX::Reflection::EventDescImpl<1,RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_6268c4() -> ! {
    todo!("0x6268c4 __ZNK3RBX10Reflection13EventDescImplILi1ENS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")
}

// 0x626a68 — __ZNK3RBX10Reflection13EventDescBaseINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE — RBX::Reflection::EventDescBase<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::disconnectAll(RBX::Reflection::EventSource *)const
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::SkateboardController,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::SkateboardController::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_626a68() -> ! {
    todo!("0x626a68 __ZNK3RBX10Reflection13EventDescBaseINS_20SkateboardControllerEFvSsEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")
}

// 0x626a7c — __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE — RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::PropDescriptor<float (RBX::SkateboardController::*)(void)const,int>(char const*,char const*,float (RBX::SkateboardController::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::PropDescriptor<float (RBX::SkateboardController::*)(void)const,int>(char const*,char const*,float (RBX::SkateboardController::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_626a7c() -> ! {
    todo!("0x626a7c __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x626b88 — __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED0Ev — RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::~PropDescriptor()
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::~PropDescriptor()")]
pub fn stub_626b88() -> ! {
    todo!("0x626b88 __ZN3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfED0Ev")
}

// 0x626bb4 — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE10isReadOnlyEv — RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::isReadOnly(void)const
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::isReadOnly(void)const")]
pub fn stub_626bb4() -> ! {
    todo!("0x626bb4 __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE10isReadOnlyEv")
}

// 0x626bb8 — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv — RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::isWriteOnly(void)const
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_626bb8() -> ! {
    todo!("0x626bb8 __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE11isWriteOnlyEv")
}

// 0x626bbc — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE — RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_626bbc() -> ! {
    todo!("0x626bbc __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x626bdc — __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf — RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,float const&)const
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardController,float>::GetImpl<float (RBX::SkateboardController::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_626bdc() -> ! {
    todo!("0x626bdc __ZNK3RBX10Reflection14PropDescriptorINS_20SkateboardControllerEfE7GetImplIMS2_KFfvEE8setValueEPNS0_13DescribedBaseERKf")
}

// 0x629af4 — __ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_ — boost::shared_ptr<RBX::SkateboardController>::operator=(boost::shared_ptr<RBX::SkateboardController> const&)
// type: int(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_")]
#[doc(alias = "boost::shared_ptr<RBX::SkateboardController>::operator=(boost::shared_ptr<RBX::SkateboardController> const&)")]
pub fn stub_629af4() -> ! {
    todo!("0x629af4 __ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_")
}

// 0x629b2c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<RBX::SkateboardController> RBX::Creatable<RBX::Instance>::create<RBX::SkateboardController>(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "boost::shared_ptr<RBX::SkateboardController> RBX::Creatable<RBX::Instance>::create<RBX::SkateboardController>(void)")]
pub fn stub_629b2c() -> ! {
    todo!("0x629b2c __ZN3RBX9CreatableINS_8InstanceEE6createINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEv")
}

// 0x62dff0 — __ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ — boost::shared_ptr<RBX::SkateboardController>::shared_ptr<RBX::SkateboardController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "boost::shared_ptr<RBX::SkateboardController>::shared_ptr<RBX::SkateboardController,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_62dff0() -> ! {
    todo!("0x62dff0 __ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x62e0b8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20SkateboardControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardController,RBX::SkateboardController>(boost::shared_ptr<RBX::SkateboardController> const*,RBX::SkateboardController *)const
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20SkateboardControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardController,RBX::SkateboardController>(boost::shared_ptr<RBX::SkateboardController> const*,RBX::SkateboardController *)const")]
pub fn stub_62e0b8() -> ! {
    todo!("0x62e0b8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20SkateboardControllerES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x62e1a0 — __ZN5boost6detail12shared_countC2IPN3RBX20SkateboardControllerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX20SkateboardControllerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_62e1a0() -> ! {
    todo!("0x62e1a0 __ZN5boost6detail12shared_countC2IPN3RBX20SkateboardControllerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

// 0x62e2a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_62e2a8() -> ! {
    todo!("0x62e2a8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

// 0x62e2ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_62e2ac() -> ! {
    todo!("0x62e2ac __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

// 0x62e2b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_62e2b0() -> ! {
    todo!("0x62e2b0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

// 0x62e2d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_62e2d0() -> ! {
    todo!("0x62e2d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

// 0x62e2e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardController *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_62e2e8() -> ! {
    todo!("0x62e2e8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SkateboardControllerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

// 0x630100 — __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEE9singletonEv — RBX::Reflection::RefType<RBX::SkateboardController *>::singleton(void)
#[doc(alias = "__ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEE9singletonEv")]
#[doc(alias = "RBX::Reflection::RefType<RBX::SkateboardController *>::singleton(void)")]
pub fn stub_630100() -> ! {
    todo!("0x630100 __ZN3RBX10Reflection7RefTypeIPNS_20SkateboardControllerEE9singletonEv")
}

// 0x630704 — __ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_ — boost::shared_ptr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)
// type: void __fastcall(_QWORD *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "__ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_")]
#[doc(alias = "boost::shared_ptr<RBX::SkateboardController> RBX::shared_from<RBX::SkateboardController>(RBX::SkateboardController*)")]
pub fn stub_630704() -> ! {
    todo!("0x630704 __ZN3RBX11shared_fromINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEPS4_")
}

// 0x658a0c — __ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE — RBX::Surface::setSurfaceInput(RBX::LegacyController::InputType)
#[doc(alias = "__ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE")]
#[doc(alias = "RBX::Surface::setSurfaceInput(RBX::LegacyController::InputType)")]
pub fn stub_658a0c() -> ! {
    todo!("0x658a0c __ZN3RBX7Surface15setSurfaceInputENS_16LegacyController9InputTypeE")
}

// 0x658f8c — __ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v — RBX::LegacyController::InputType & RBX::Reflection::Variant::genericConvert<RBX::LegacyController::InputType>(void)
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v")]
#[doc(alias = "RBX::LegacyController::InputType & RBX::Reflection::Variant::genericConvert<RBX::LegacyController::InputType>(void)")]
pub fn stub_658f8c() -> ! {
    todo!("0x658f8c __ZN3RBX10Reflection7Variant14genericConvertINS_16LegacyController9InputTypeEEERT_v")
}

// 0x659118 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)1,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_659118() -> ! {
    todo!("0x659118 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE1ENS_16LegacyController9InputTypeEED1Ev")
}

// 0x659184 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)4,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_659184() -> ! {
    todo!("0x659184 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE4ENS_16LegacyController9InputTypeEED1Ev")
}

// 0x6591f0 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED1Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED1Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)3,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_6591f0() -> ! {
    todo!("0x6591f0 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE3ENS_16LegacyController9InputTypeEED1Ev")
}

// 0x65925c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED1Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED1Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)0,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_65925c() -> ! {
    todo!("0x65925c __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE0ENS_16LegacyController9InputTypeEED1Ev")
}

// 0x6592c8 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED1Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED1Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_6592c8() -> ! {
    todo!("0x6592c8 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED1Ev")
}

// 0x659334 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED1Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED1Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_659334() -> ! {
    todo!("0x659334 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED1Ev")
}

// 0x659508 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_659508() -> ! {
    todo!("0x659508 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")
}

// 0x6595b4 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED0Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED0Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_6595b4() -> ! {
    todo!("0x6595b4 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEED0Ev")
}

// 0x6595e0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10isReadOnlyEv — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isReadOnly(void)const")]
pub fn stub_6595e0() -> ! {
    todo!("0x6595e0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

// 0x6595f0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11isWriteOnlyEv — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
pub fn stub_6595f0() -> ! {
    todo!("0x6595f0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

// 0x659600 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_ — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_659600() -> ! {
    todo!("0x659600 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")
}

// 0x659628 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_659628() -> ! {
    todo!("0x659628 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")
}

// 0x659650 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_659650() -> ! {
    todo!("0x659650 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")
}

// 0x6597a8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_ — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_6597a8() -> ! {
    todo!("0x6597a8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")
}

// 0x6597cc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14hasStringValueEv — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::hasStringValue(void)const")]
pub fn stub_6597cc() -> ! {
    todo!("0x6597cc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

// 0x6597d0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_6597d0() -> ! {
    todo!("0x6597d0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x659820 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_659820() -> ! {
    todo!("0x659820 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")
}

// 0x659884 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_659884() -> ! {
    todo!("0x659884 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")
}

// 0x6598a4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_6598a4() -> ! {
    todo!("0x6598a4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x659afc — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_659afc() -> ! {
    todo!("0x659afc __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x659b44 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_659b44() -> ! {
    todo!("0x659b44 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")
}

// 0x659ba0 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_659ba0() -> ! {
    todo!("0x659ba0 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x659ba8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_659ba8() -> ! {
    todo!("0x659ba8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE12setEnumValueEPNS_10Reflection13DescribedBaseEi")
}

// 0x659c1c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_659c1c() -> ! {
    todo!("0x659c1c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE11getEnumItemEPKNS_10Reflection13DescribedBaseE")
}

// 0x659c6c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)2,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_659c6c() -> ! {
    todo!("0x659c6c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE2ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKNS_4NameE")
}

// 0x659cc8 — __ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_ — RBX::Reflection::EnumDesc<RBX::LegacyController::InputType>::convertToIndex(RBX::LegacyController::InputType)const
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::LegacyController::InputType>::convertToIndex(RBX::LegacyController::InputType)const")]
pub fn stub_659cc8() -> ! {
    todo!("0x659cc8 __ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_")
}

// 0x659d38 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv — RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isReadOnly(void)const")]
pub fn stub_659d38() -> ! {
    todo!("0x659d38 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE10isReadOnlyEv")
}

// 0x659d3c — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv — RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::isWriteOnly(void)const")]
pub fn stub_659d3c() -> ! {
    todo!("0x659d3c __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE11isWriteOnlyEv")
}

// 0x659d40 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_659d40() -> ! {
    todo!("0x659d40 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8getValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x659d60 — __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_ — RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const
#[doc(alias = "__ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")]
#[doc(alias = "RBX::SurfaceGetSet<(RBX::NormalId)2,RBX::LegacyController::InputType,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>::setValue(RBX::Reflection::DescribedBase *,RBX::LegacyController::InputType const&)const")]
pub fn stub_659d60() -> ! {
    todo!("0x659d60 __ZNK3RBX13SurfaceGetSetILNS_8NormalIdE2ENS_16LegacyController9InputTypeEMNS_12PartInstanceEKFS3_S1_EMS4_FvS1_S3_EE8setValueEPNS_10Reflection13DescribedBaseERKS3_")
}

// 0x65a78c — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::SurfaceEnumPropDescriptor<RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType)>(char const*,char const*,RBX::LegacyController::InputType (RBX::PartInstance::*)(RBX::NormalId)const,void (RBX::PartInstance::*)(RBX::NormalId,RBX::LegacyController::InputType),RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_65a78c() -> ! {
    todo!("0x65a78c __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEEC2IMNS_12PartInstanceEKFS3_S1_EMS6_FvS1_S3_EEEPKcSC_T_T0_NS_10Reflection18PropertyDescriptor13FunctionalityE")
}

// 0x65a838 — __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED0Ev — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()
#[doc(alias = "__ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED0Ev")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::~SurfaceEnumPropDescriptor()")]
pub fn stub_65a838() -> ! {
    todo!("0x65a838 __ZN3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEED0Ev")
}

// 0x65a864 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10isReadOnlyEv — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isReadOnly(void)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10isReadOnlyEv")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isReadOnly(void)const")]
pub fn stub_65a864() -> ! {
    todo!("0x65a864 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10isReadOnlyEv")
}

// 0x65a874 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11isWriteOnlyEv — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isWriteOnly(void)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::isWriteOnly(void)const")]
pub fn stub_65a874() -> ! {
    todo!("0x65a874 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11isWriteOnlyEv")
}

// 0x65a884 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_ — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65a884() -> ! {
    todo!("0x65a884 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE11equalValuesEPKNS_10Reflection13DescribedBaseES8_")
}

// 0x65a8ac — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_65a8ac() -> ! {
    todo!("0x65a8ac __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10getVariantEPKNS_10Reflection13DescribedBaseERNS5_7VariantE")
}

// 0x65a8d4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_65a8d4() -> ! {
    todo!("0x65a8d4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10setVariantEPNS_10Reflection13DescribedBaseERKNS5_7VariantE")
}

// 0x65aa2c — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_ — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_65aa2c() -> ! {
    todo!("0x65aa2c __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9copyValueEPKNS_10Reflection13DescribedBaseEPS6_")
}

// 0x65aa50 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14hasStringValueEv — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::hasStringValue(void)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14hasStringValueEv")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::hasStringValue(void)const")]
pub fn stub_65aa50() -> ! {
    todo!("0x65aa50 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14hasStringValueEv")
}

// 0x65aa54 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65aa54() -> ! {
    todo!("0x65aa54 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14getStringValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x65aaa4 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_65aaa4() -> ! {
    todo!("0x65aaa4 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE14setStringValueEPNS_10Reflection13DescribedBaseERKSs")
}

// 0x65ab08 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_65ab08() -> ! {
    todo!("0x65ab08 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE10writeValueEPKNS_10Reflection13DescribedBaseEP10XmlElement")
}

// 0x65ab28 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_65ab28() -> ! {
    todo!("0x65ab28 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE9readValueEPNS_10Reflection13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")
}

// 0x65ad80 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65ad80() -> ! {
    todo!("0x65ad80 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13getIndexValueEPKNS_10Reflection13DescribedBaseE")
}

// 0x65adc8 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_65adc8() -> ! {
    todo!("0x65adc8 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE13setIndexValueEPNS_10Reflection13DescribedBaseEm")
}

// 0x65ae24 — __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE — RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const
#[doc(alias = "__ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")]
#[doc(alias = "RBX::SurfaceEnumPropDescriptor<(RBX::NormalId)5,RBX::LegacyController::InputType>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_65ae24() -> ! {
    todo!("0x65ae24 __ZNK3RBX25SurfaceEnumPropDescriptorILNS_8NormalIdE5ENS_16LegacyController9InputTypeEE12getEnumValueEPKNS_10Reflection13DescribedBaseE")
}
