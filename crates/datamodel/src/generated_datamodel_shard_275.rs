// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x2f23f4..0x2ff674 | datamodel distinct 33459->33579 global uncovered 52887->52767, lowest gap EA-sorted asc next 120 after shard 274 (0x2a44d8..0x2abbc4)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed
// Shard: 275 EA-sorted ascending next uncovered gap after shard 274 (distinct check via export.json sorted EA, no overlap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2f23f4 — __ZN3RBX10RunDraggerC2Ev
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::RunDragger(void)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f23f4 as stub_2f23f4;

// 0x2f25ac — __ZN3RBX10RunDraggerD1Ev
// type: void __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::~RunDragger()")]
pub use rbx_reflection::generated_refl_wd2::stub_0x2f25ac as stub_2f25ac;

// 0x2f25b0 — __ZN3RBX10RunDraggerD2Ev
// type: void __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::~RunDragger()")]
pub use rbx_reflection::generated_refl_wd2::stub_0x2f25b0 as stub_2f25b0;

// 0x2f33e0 — __ZN3RBX10RunDragger17createSnapSurfaceEPNS_9PrimitiveEPN3G3D5ArrayImLi10ELm32EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::RunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f33e0 as stub_2f33e0;

// 0x2f41c8 — __ZN3RBX10RunDragger19getSnapSurfaceCoordEv
// type: void __fastcall(RBX::RunDragger *this, int)
#[doc(alias = "RBX::RunDragger::getSnapSurfaceCoord(void)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f41c8 as stub_2f41c8;

// 0x2f4630 — __ZN3RBX10RunDragger8notTriedEPNS_9PrimitiveERKN3G3D5ArrayIS2_Li10ELm32EEE
#[doc(alias = "RBX::RunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f4630 as stub_2f4630;

// 0x2f46c0 — __ZN3RBX10RunDragger8adjacentEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RunDragger::adjacent(RBX::Primitive *,RBX::Primitive *)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f46c0 as stub_2f46c0;

// 0x2f4ae0 — __ZN3RBX10RunDragger11fallOffEdgeEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::fallOffEdge(void)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f4ae0 as stub_2f4ae0;

// 0x2f4dd8 — __ZN3RBX10RunDragger16tooCloseToCameraEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::tooCloseToCamera(void)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f4dd8 as stub_2f4dd8;

// 0x2f4eac — __ZN3RBX10RunDragger8findSnapERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::RunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f4eac as stub_2f4eac;

// 0x2f5018 — __ZN3RBX10RunDragger18findNoSnapPositionERKN3G3D15CoordinateFrameE
// type: void __fastcall(RBX::RunDragger *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::RunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f5018 as stub_2f5018;

// 0x2f5168 — __ZN3RBX10RunDragger9findSafeYEv
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this)
#[doc(alias = "RBX::RunDragger::findSafeY(void)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f5168 as stub_2f5168;

// 0x2f5610 — __ZN3RBX10RunDragger4snapERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::RunDragger *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::RunDragger::snap(RBX::RbxRay const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f5610 as stub_2f5610;

// 0x2f587c — __ZN3G3D5ArrayImLi10ELm32EE6appendERKm
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::append(unsigned long const&)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f587c as stub_2f587c;

// 0x2f58d8 — __ZN3G3D5ArrayImLi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::resize(int,bool)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f58d8 as stub_2f58d8;

// 0x2f5990 — __ZN3G3D5ArrayImLi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::realloc(int)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f5990 as stub_2f5990;

// 0x2f5b78 — __ZN3G3D5ArrayImLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::~Array()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f5b78 as stub_2f5b78;

// 0x2f5c4c — __ZN3G3D5ArrayImLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<unsigned long,10,32ul>::Array(void)")]
pub use rbx_core::generated_core_shard_le::stub_0x2f5c4c as stub_2f5c4c;

// 0x2f5d3c — __GLOBAL__I_a_102
#[doc(alias = "global constructor keyed to_a_102")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f5d3c as stub_2f5d3c;

// 0x2f614c — __ZN3RBX13ArrowToolBase12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, const UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onMouseHover(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f614c as stub_2f614c;

// 0x2f6154 — __ZN3RBX13ArrowToolBase11onMouseIdleERKNS_7UIEventE
// type: int __fastcall(RBX::ArrowToolBase *this, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onMouseIdle(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6154 as stub_2f6154;

// 0x2f6190 — __ZNK3RBX13ArrowToolBase13getCursorNameEv
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this)
#[doc(alias = "RBX::ArrowToolBase::getCursorName(void)const")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6190 as stub_2f6190;

// 0x2f6254 — __ZN3RBX13ArrowToolBase11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6254 as stub_2f6254;

// 0x2f6610 — __ZN3RBX13ArrowToolBase13onPeekKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ArrowToolBase::onPeekKeyDown(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6610 as stub_2f6610;

// 0x2f6850 — __ZN3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
pub use rbx_core::generated_core_shard_ap::stub_0x2f6850 as stub_2f6850;

// 0x2f6858 — __ZN3RBX13ArrowToolBase15renderHoverOverEPNS_5AdornEb
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Adorn *, bool)
#[doc(alias = "RBX::ArrowToolBase::renderHoverOver(RBX::Adorn *,bool)")]
pub use rbx_core::generated_core_shard_ap::stub_0x2f6858 as stub_2f6858;

// 0x2f68c8 — __ZThn4_N3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f68c8 as stub_2f68c8;

// 0x2f68d0 — __ZNK3RBX16AdvArrowToolBase13getCursorNameEv
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this)
#[doc(alias = "RBX::AdvArrowToolBase::getCursorName(void)const")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f68d0 as stub_2f68d0;

// 0x2f6900 — __ZN3RBX16AdvArrowToolBase9onKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onKeyDown(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6900 as stub_2f6900;

// 0x2f6954 — __ZN3RBX16AdvArrowToolBase11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onMouseDown(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6954 as stub_2f6954;

// 0x2f6d04 — __ZN3RBX16AdvArrowToolBase11onMouseMoveERKNS_7UIEventE
// type: int __fastcall(RBX::AdvArrowToolBase *this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onMouseMove(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6d04 as stub_2f6d04;

// 0x2f6d18 — __ZN3RBX16AdvArrowToolBase30determineManualJointConditionsEv
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this)
#[doc(alias = "RBX::AdvArrowToolBase::determineManualJointConditions(void)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6d18 as stub_2f6d18;

// 0x2f6fb8 — __ZN3RBX16AdvArrowToolBase9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::AdvArrowToolBase::onMouseUp(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f6fb8 as stub_2f6fb8;

// 0x2f7134 — __ZN3RBX16BoxSelectCommandD0Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f7134 as stub_2f7134;

// 0x2f71d4 — __ZN3RBX16BoxSelectCommandD1Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f71d4 as stub_2f71d4;

// 0x2f71d8 — __ZThn36_N3RBX16BoxSelectCommandD0Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::~BoxSelectCommand()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f71d8 as stub_2f71d8;

// 0x2f71e0 — __ZN3RBX16BoxSelectCommandD2Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f71e0 as stub_2f71e0;

// 0x2f7324 — __ZThn36_N3RBX16BoxSelectCommandD1Ev
// type: void __fastcall(RBX::BoxSelectCommand *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::~BoxSelectCommand()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f7324 as stub_2f7324;

// 0x2f73fc — __ZN3RBX16BoxSelectCommand11onMouseDownERKNS_7UIEventE
// type: int __fastcall(RBX::BoxSelectCommand *this, const RBX::UIEvent *, int)
#[doc(alias = "RBX::BoxSelectCommand::onMouseDown(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f73fc as stub_2f73fc;

// 0x2f7468 — __ZN3RBX16BoxSelectCommand11onMouseMoveERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::BoxSelectCommand::onMouseMove(RBX::UIEvent const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f7468 as stub_2f7468;

// 0x2f7818 — __ZN3RBX16BoxSelectCommand8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
pub use rbx_core::generated_core_shard_ap::stub_0x2f7818 as stub_2f7818;

// 0x2f78d0 — __ZThn4_N3RBX16BoxSelectCommand8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f78d0 as stub_2f78d0;

// 0x2f7ec0 — __ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f7ec0 as stub_2f7ec0;

// 0x2f7ee8 — __ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f7ee8 as stub_2f7ee8;

// 0x2f7eec — __ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f7eec as stub_2f7eec;

// 0x2f8624 — __ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: boost::shared_ptr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub use rbx_core::generated_core_shard_ap::stub_0x2f8624 as stub_2f8624;

// 0x2f86ec — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const")]
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(boost::shared_ptr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f86ec as stub_2f86ec;

// 0x2f87d0 — __ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub use rbx_core::generated_core_shard_ap::stub_0x2f87d0 as stub_2f87d0;

// 0x2f88c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f88c8 as stub_2f88c8;

// 0x2f88cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2f88cc as stub_2f88cc;

// 0x2f88d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub use rbx_core::generated_core_shard_ap::stub_0x2f88d0 as stub_2f88d0;

// 0x2f88e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub use rbx_core::generated_core_shard_ap::stub_0x2f88e0 as stub_2f88e0;

// 0x2f88f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub use rbx_core::generated_core_shard_ap::stub_0x2f88f8 as stub_2f88f8;

// 0x2f8c2c — __GLOBAL__I_a_103
#[doc(alias = "global constructor keyed to_a_103")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f8c2c as stub_2f8c2c;

// 0x2f8f04 — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")]
pub use rbx_reflection::generated::stub_0x2f8f04 as stub_2f8f04;

// 0x2f8f08 — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")]
pub use rbx_reflection::generated::stub_0x2f8f08 as stub_2f8f08;

// 0x2f910c — __ZN3RBX10Reflection8EnumDescINS_6Action10ActionTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType,char const*)")]
pub use rbx_reflection::generated::stub_0x2f910c as stub_2f910c;

// 0x2f946c — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f946c as stub_2f946c;

// 0x2f94a0 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f94a0 as stub_2f94a0;

// 0x2f94c8 — __ZNSt3mapIPKN3RBX4NameENS0_6Action10ActionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f94c8 as stub_2f94c8;

// 0x2f9520 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f9520 as stub_2f9520;

// 0x2f95d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f95d4 as stub_2f95d4;

// 0x2f962c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f962c as stub_2f962c;

// 0x2f9694 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f9694 as stub_2f9694;

// 0x2f9778 — __ZNSt12_Vector_baseIN3RBX6Action10ActionTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f9778 as stub_2f9778;

// 0x2f9790 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Action10ActionTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f9790 as stub_2f9790;

// 0x2f97cc — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f97cc as stub_2f97cc;

// 0x2f995c — __GLOBAL__I_a_104
#[doc(alias = "global constructor keyed to_a_104")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f995c as stub_2f995c;

// 0x2f9a24 — __ZN3RBX15StringConverterINS_11AnimationIdEE14convertToValueERKSsRS1_
// type: int __fastcall(std::string *)
#[doc(alias = "RBX::StringConverter<RBX::AnimationId>::convertToValue(std::string const&,RBX::AnimationId&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2f9a24 as stub_2f9a24;

// 0x2f9b48 — __ZN3RBX10Reflection4Type12getSingletonINS_11AnimationIdEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::AnimationId>(void)")]
pub use rbx_core::generated_core_watchdog_l::stub_0x2f9b48 as stub_2f9b48;

// 0x2f9b4c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub use rbx_core::generated_core_watchdog_m::stub_2f9b4c as stub_2f9b4c;

// 0x2f9d34 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub use rbx_core::generated_core_watchdog_m::stub_2f9d34 as stub_2f9d34;

// 0x2f9edc — __ZN3RBX10Reflection7Variant7convertINS_11AnimationIdEEERT_v
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::AnimationId & RBX::Reflection::Variant::convert<RBX::AnimationId>(void)")]
pub use rbx_core::generated_core_watchdog_m::stub_2f9edc as stub_2f9edc;

// 0x2fa0c8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
pub use rbx_core::generated_core_watchdog_m::stub_2fa0c8 as stub_2fa0c8;

// 0x2fa124 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::hasStringValue(void)const")]
pub use rbx_reflection::generated::stub_0x2fa124 as stub_2fa124;

// 0x2fa128 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub use rbx_core::generated_core_watchdog_m::stub_2fa128 as stub_2fa128;

// 0x2fa244 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub use rbx_core::generated_core_watchdog_m::stub_2fa244 as stub_2fa244;

// 0x2fa39c — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11AnimationIdEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2fa39c as stub_2fa39c;

// 0x2fa3fc — __ZN3RBX10Reflection7Variant14genericConvertINS_11AnimationIdEEERT_v
#[doc(alias = "RBX::AnimationId & RBX::Reflection::Variant::genericConvert<RBX::AnimationId>(void)")]
pub use rbx_core::generated_core_watchdog_m::stub_2fa3fc as stub_2fa3fc;

// 0x2fa6a8 — __ZN3rbx8any_castIN3RBX11AnimationIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2fa6a8 as stub_2fa6a8;

// 0x2fa700 — __ZN3rbx8any_castIRN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2fa700 as stub_2fa700;

// 0x2fa7f0 — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2fa7f0 as stub_2fa7f0;

// 0x2fa85c — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::construct_func(char const*,char *)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2fa85c as stub_2fa85c;

// 0x2fa878 — __ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE13destruct_funcEPc
// type: int()
#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::destruct_func(char *)")]
pub use rbx_core::generated_core_shard_ag::stub_0x2fa878 as stub_2fa878;

// 0x2fa87c — __GLOBAL__I_a_105
#[doc(alias = "global constructor keyed to_a_105")]
pub use rbx_core::generated_core_shard_ag::stub_0x2fa87c as stub_2fa87c;

// 0x2fad24 — __ZN3RBX14AsyncHttpQueue13setThreadPoolEi
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this, int)
#[doc(alias = "RBX::AsyncHttpQueue::setThreadPool(int)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fad24 as stub_2fad24;

// 0x2faf2c — __ZNK3RBX14AsyncHttpQueue19getRequestQueueSizeEv
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::getRequestQueueSize(void)const")]
pub use rbx_core::generated_core_shard_ah::stub_0x2faf2c as stub_2faf2c;

// 0x2faf68 — __ZN3RBX14AsyncHttpQueueD0Ev
// type: void __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2faf68 as stub_2faf68;

// 0x2fb008 — __ZN3RBX14AsyncHttpQueueD1Ev
// type: void __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2fb008 as stub_2fb008;

// 0x2fb00c — __ZN3RBX14AsyncHttpQueueD2Ev
// type: void __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub use rbx_reflection::generated_refl_wd_watchdog::stub_0x2fb00c as stub_2fb00c;

// 0x2fb2ac — __ZN3RBX14AsyncHttpQueue11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, pthread_mutex_t *, char, pthread_mutex_t *, char, void *, int, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fb2ac as stub_2fb2ac;

// 0x2fb548 — __ZN3RBX14AsyncHttpQueue15processRequestsEN5boost8weak_ptrIS0_EESt14_List_iteratorINS0_7RequestEENS1_10shared_ptrINS_5mutexEEE
#[doc(alias = "RBX::AsyncHttpQueue::processRequests(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>)")]
// was: RBX::AsyncHttpQueue::processRequests(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>)
pub use rbx_core::generated_core_shard_ap::stub_0x2fb548 as stub_2fb548;

// 0x2fc874 — __ZN3RBXL19InvokeAsyncCallbackEN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS0_10shared_ptrIKSsEEEEES3_S7_
// type: void __fastcall(int, int, const shared_count *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
// was: RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)
pub use rbx_core::generated_core_shard_ap::stub_0x2fc874 as stub_2fc874;

// 0x2fca04 — __ZN3RBX14AsyncHttpQueue19isRequestQueueEmptyEv
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this)
#[doc(alias = "RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fca04 as stub_2fca04;

// 0x2fca3c — __ZN3RBXL15checkContentUrlESs
#[doc(alias = "RBX::checkContentUrl(std::string)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fca3c as stub_2fca3c;

// 0x2fd150 — __ZN3RBX14AsyncHttpQueue9FailedUrlC2EPKc
// type: _DWORD __fastcall(RBX::AsyncHttpQueue::FailedUrl *__hidden this, const char *)
#[doc(alias = "RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fd150 as stub_2fd150;

// 0x2fd220 — __ZN3RBX14AsyncHttpQueue8isUrlBadERKSs
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this, const std::string *)
#[doc(alias = "RBX::AsyncHttpQueue::isUrlBad(std::string const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fd220 as stub_2fd220;

// 0x2fd37c — __ZN3RBX14AsyncHttpQueue12asyncRequestERKSsfPN5boost8functionIFvNS0_13RequestResultEPSiNS3_10shared_ptrIS1_EEEEENS0_9ResultJobEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)")]
// was: RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)
pub use rbx_core::generated_core_shard_ap::stub_0x2fd37c as stub_2fd37c;

// 0x2fd910 — __ZN3RBX14AsyncHttpQueue11syncRequestERKSs
// type: _DWORD __fastcall(RBX::AsyncHttpQueue *__hidden this, const std::string *)
#[doc(alias = "RBX::AsyncHttpQueue::syncRequest(std::string const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fd910 as stub_2fd910;

// 0x2fded0 — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::operator=(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const&)")]
// was: boost::shared_ptr<RBX::HttpQueueStatsItem>::operator=(boost::shared_ptr<RBX::HttpQueueStatsItem> const&)
pub use rbx_core::generated_core_shard_ap::stub_0x2fded0 as stub_2fded0;

// 0x2fdfbc — __ZN5boost4bindIvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS3_7RequestEENS_10shared_ptrINS2_5mutexEEES4_S7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_3<boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_3<boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>,boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)
pub use rbx_core::generated_core_shard_ap::stub_0x2fdfbc as stub_2fdfbc;

// 0x2fe168 — __ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "boost::weak_ptr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
// was: boost::weak_ptr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)
pub use rbx_core::generated_core_shard_ap::stub_0x2fe168 as stub_2fe168;

// 0x2fe358 — __ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)
pub use rbx_core::generated_core_shard_ap::stub_0x2fe358 as stub_2fe358;

// 0x2fe524 — __ZNK5boost9function2IbRKSsPSsEclES2_S3_
// type: int(void)
#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const")]
// was: boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const
pub use rbx_core::generated_core_shard_ap::stub_0x2fe524 as stub_2fe524;

// 0x2fe5f0 — __ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::operator=(rbx_core::SharedPtr<RBX::Http> const&)")]
// was: boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)
pub use rbx_core::generated_core_shard_ap::stub_0x2fe5f0 as stub_2fe5f0;

// 0x2fe628 — __ZN5boost10shared_ptrISsE5resetISsEEvPT_
#[doc(alias = "void rbx_core::SharedPtr<std::string>::reset<std::string>(std::string *)")]
// was: void boost::shared_ptr<std::string>::reset<std::string>(std::string *)
pub use rbx_core::generated_core_shard_ap::stub_0x2fe628 as stub_2fe628;

// 0x2fe654 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fe654 as stub_2fe654;

// 0x2fea20 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
// type: int __fastcall(int, std::_List_node_base *this)
#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fea20 as stub_2fea20;

// 0x2fea58 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fea58 as stub_2fea58;

// 0x2feaa8 — __ZN3RBX14AsyncHttpQueue15registerContentERKSsN5boost10shared_ptrIS1_EES5_
#[doc(alias = "RBX::AsyncHttpQueue::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: RBX::AsyncHttpQueue::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub use rbx_core::generated_core_shard_ap::stub_0x2feaa8 as stub_2feaa8;

// 0x2feab0 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2feab0 as stub_2feab0;

// 0x2fee5c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2fee5c as stub_2fee5c;

// 0x2fee80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_
#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> const&)")]
// was: boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> const&)
pub use rbx_core::generated_core_shard_ap::stub_0x2fee80 as stub_2fee80;

// 0x2fef44 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)
pub use rbx_core::generated_core_shard_ap::stub_0x2fef44 as stub_2fef44;

// 0x2ff020 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)
pub use rbx_core::generated_core_shard_ap::stub_0x2ff020 as stub_2ff020;

// 0x2ff128 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2ff128 as stub_2ff128;

// 0x2ff188 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2ff188 as stub_2ff188;

// 0x2ff2d4 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
// type: int(void)
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2ff2d4 as stub_2ff2d4;

// 0x2ff43c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2ff43c as stub_2ff43c;

// 0x2ff674 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
pub use rbx_core::generated_core_shard_ah::stub_0x2ff674 as stub_2ff674;

