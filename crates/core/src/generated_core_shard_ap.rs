//! core shard AP — 120 core stubs EA-sorted, next uncovered after AO 0x2e3d54..0x2e4624 (strict RBX|boost|std earliest gap).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 120 uncovered after 0x2e3d54.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::HammerTool::~HammerTool()")]
// 0x2e4624 — __ZN3RBX10HammerToolD0Ev
pub fn stub_0x2e4624() -> ! {
    todo!("0x2e4624 __ZN3RBX10HammerToolD0Ev")
}

#[doc(alias = "RBX::HammerTool::~HammerTool()")]
// 0x2e46c4 — __ZN3RBX10HammerToolD1Ev
pub fn stub_0x2e46c4() -> ! {
    todo!("0x2e46c4 __ZN3RBX10HammerToolD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::HammerTool::~HammerTool()")]
// 0x2e46c8 — __ZThn36_N3RBX10HammerToolD0Ev
pub fn stub_0x2e46c8() -> ! {
    todo!("0x2e46c8 __ZThn36_N3RBX10HammerToolD0Ev")
}

#[doc(alias = "RBX::HammerTool::~HammerTool()")]
// 0x2e46d0 — __ZN3RBX10HammerToolD2Ev
pub fn stub_0x2e46d0() -> ! {
    todo!("0x2e46d0 __ZN3RBX10HammerToolD2Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::HammerTool::~HammerTool()")]
// 0x2e47ec — __ZThn36_N3RBX10HammerToolD1Ev
pub fn stub_0x2e47ec() -> ! {
    todo!("0x2e47ec __ZThn36_N3RBX10HammerToolD1Ev")
}

#[doc(alias = "RBX::HammerTool::onMouseIdle(RBX::UIEvent const&)")]
// 0x2e47f4 — __ZN3RBX10HammerTool11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2e47f4() -> ! {
    todo!("0x2e47f4 __ZN3RBX10HammerTool11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::HammerTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2e48cc — __ZN3RBX10HammerTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2e48cc() -> ! {
    todo!("0x2e48cc __ZN3RBX10HammerTool11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::HammerTool::getCursorName(void)const")]
// 0x2e4a2c — __ZNK3RBX10HammerTool13getCursorNameEv
pub fn stub_0x2e4a2c() -> ! {
    todo!("0x2e4a2c __ZNK3RBX10HammerTool13getCursorNameEv")
}

#[doc(alias = "RBX::HammerTool::render3dAdorn(RBX::Adorn *)")]
// 0x2e4a5c — __ZN3RBX10HammerTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2e4a5c() -> ! {
    todo!("0x2e4a5c __ZN3RBX10HammerTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "`non-virtual thunk toRBX::HammerTool::render3dAdorn(RBX::Adorn *)")]
// 0x2e4a70 — __ZThn4_N3RBX10HammerTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2e4a70() -> ! {
    todo!("0x2e4a70 __ZThn4_N3RBX10HammerTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::HammerTool::isSticky(void)const")]
// 0x2e4b5c — __ZNK3RBX10HammerTool8isStickyEv
pub fn stub_0x2e4b5c() -> ! {
    todo!("0x2e4b5c __ZNK3RBX10HammerTool8isStickyEv")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Joint>,std::allocator<rbx_core::SharedPtr<RBX::Joint>>>::~vector()")]
// 0x2e8260 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev
pub fn stub_0x2e8260() -> ! {
    todo!("0x2e8260 __ZNSt6vectorIN5boost10shared_ptrIN3RBX5JointEEESaIS4_EED2Ev")
}

#[doc(alias = "RBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)")]
// 0x2ec558 — __ZN3RBX18MoveResizeJoinTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2ec558() -> ! {
    todo!("0x2ec558 __ZN3RBX18MoveResizeJoinTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "`non-virtual thunk toRBX::MoveResizeJoinTool::render3dAdorn(RBX::Adorn *)")]
// 0x2ec7e4 — __ZThn4_N3RBX18MoveResizeJoinTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2ec7e4() -> ! {
    todo!("0x2ec7e4 __ZThn4_N3RBX18MoveResizeJoinTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::MoveResizeJoinTool::render2d(RBX::Adorn *)")]
// 0x2ec7ec — __ZN3RBX18MoveResizeJoinTool8render2dEPNS_5AdornE
pub fn stub_0x2ec7ec() -> ! {
    todo!("0x2ec7ec __ZN3RBX18MoveResizeJoinTool8render2dEPNS_5AdornE")
}

#[doc(alias = "`non-virtual thunk toRBX::MoveResizeJoinTool::render2d(RBX::Adorn *)")]
// 0x2ed9d4 — __ZThn4_N3RBX18MoveResizeJoinTool8render2dEPNS_5AdornE
pub fn stub_0x2ed9d4() -> ! {
    todo!("0x2ed9d4 __ZThn4_N3RBX18MoveResizeJoinTool8render2dEPNS_5AdornE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::shared_from<RBX::MoveResizeJoinTool>(RBX::MoveResizeJoinTool*)")]
// 0x2ee6b0 — __ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_0x2ee6b0() -> ! {
    todo!("0x2ee6b0 __ZN3RBX11shared_fromINS_18MoveResizeJoinToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::NewNullTool::render3dAdorn(RBX::Adorn *)")]
// 0x2efef0 — __ZN3RBX11NewNullTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2efef0() -> ! {
    todo!("0x2efef0 __ZN3RBX11NewNullTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "`non-virtual thunk toRBX::NewNullTool::render3dAdorn(RBX::Adorn *)")]
// 0x2efef4 — __ZThn4_N3RBX11NewNullTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2efef4() -> ! {
    todo!("0x2efef4 __ZThn4_N3RBX11NewNullTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::shared_from<RBX::NewNullTool>(RBX::NewNullTool*)")]
// 0x2efef8 — __ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_0x2efef8() -> ! {
    todo!("0x2efef8 __ZN3RBX11shared_fromINS_11NewNullToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::NewNullTool::shouldRender3dAdorn(void)const")]
// 0x2f0410 — __ZNK3RBX11NewNullTool19shouldRender3dAdornEv
pub fn stub_0x2f0410() -> ! {
    todo!("0x2f0410 __ZNK3RBX11NewNullTool19shouldRender3dAdornEv")
}

#[doc(alias = "`non-virtual thunk toRBX::NewNullTool::shouldRender3dAdorn(void)const")]
// 0x2f0414 — __ZThn4_NK3RBX11NewNullTool19shouldRender3dAdornEv
pub fn stub_0x2f0414() -> ! {
    todo!("0x2f0414 __ZThn4_NK3RBX11NewNullTool19shouldRender3dAdornEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::shared_from<RBX::NullTool>(RBX::NullTool*)")]
// 0x2f04fc — __ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_0x2f04fc() -> ! {
    todo!("0x2f04fc __ZN3RBX11shared_fromINS_8NullToolEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::PartDragTool::onMouseDown(RBX::UIEvent const&)")]
// 0x2f0bb8 — __ZN3RBX12PartDragTool11onMouseDownERKNS_7UIEventE
pub fn stub_0x2f0bb8() -> ! {
    todo!("0x2f0bb8 __ZN3RBX12PartDragTool11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::PartDragTool::onMouseMove(RBX::UIEvent const&)")]
// 0x2f0cb0 — __ZN3RBX12PartDragTool11onMouseMoveERKNS_7UIEventE
pub fn stub_0x2f0cb0() -> ! {
    todo!("0x2f0cb0 __ZN3RBX12PartDragTool11onMouseMoveERKNS_7UIEventE")
}

#[doc(alias = "RBX::PartDragTool::onMouseDelta(RBX::UIEvent const&)")]
// 0x2f0d60 — __ZN3RBX12PartDragTool12onMouseDeltaERKNS_7UIEventE
pub fn stub_0x2f0d60() -> ! {
    todo!("0x2f0d60 __ZN3RBX12PartDragTool12onMouseDeltaERKNS_7UIEventE")
}

#[doc(alias = "RBX::PartDragTool::onMouseIdle(RBX::UIEvent const&)")]
// 0x2f0ecc — __ZN3RBX12PartDragTool11onMouseIdleERKNS_7UIEventE
pub fn stub_0x2f0ecc() -> ! {
    todo!("0x2f0ecc __ZN3RBX12PartDragTool11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::PartDragTool::onMouseUp(RBX::UIEvent const&)")]
// 0x2f0f68 — __ZN3RBX12PartDragTool9onMouseUpERKNS_7UIEventE
pub fn stub_0x2f0f68() -> ! {
    todo!("0x2f0f68 __ZN3RBX12PartDragTool9onMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::PartDragTool::onKeyDown(RBX::UIEvent const&)")]
// 0x2f1134 — __ZN3RBX12PartDragTool9onKeyDownERKNS_7UIEventE
pub fn stub_0x2f1134() -> ! {
    todo!("0x2f1134 __ZN3RBX12PartDragTool9onKeyDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
// 0x2f12c0 — __ZN3RBX12PartDragTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2f12c0() -> ! {
    todo!("0x2f12c0 __ZN3RBX12PartDragTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "`non-virtual thunk toRBX::PartDragTool::render3dAdorn(RBX::Adorn *)")]
// 0x2f13d0 — __ZThn4_N3RBX12PartDragTool13render3dAdornEPNS_5AdornE
pub fn stub_0x2f13d0() -> ! {
    todo!("0x2f13d0 __ZThn4_N3RBX12PartDragTool13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// 0x2f13d8 — __ZN3RBX12PartDragToolD0Ev
pub fn stub_0x2f13d8() -> ! {
    todo!("0x2f13d8 __ZN3RBX12PartDragToolD0Ev")
}

#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// 0x2f1478 — __ZN3RBX12PartDragToolD1Ev
pub fn stub_0x2f1478() -> ! {
    todo!("0x2f1478 __ZN3RBX12PartDragToolD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::PartDragTool::~PartDragTool()")]
// 0x2f147c — __ZThn36_N3RBX12PartDragToolD0Ev
pub fn stub_0x2f147c() -> ! {
    todo!("0x2f147c __ZThn36_N3RBX12PartDragToolD0Ev")
}

#[doc(alias = "RBX::PartDragTool::~PartDragTool()")]
// 0x2f1484 — __ZN3RBX12PartDragToolD2Ev
pub fn stub_0x2f1484() -> ! {
    todo!("0x2f1484 __ZN3RBX12PartDragToolD2Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::PartDragTool::~PartDragTool()")]
// 0x2f15e4 — __ZThn36_N3RBX12PartDragToolD1Ev
pub fn stub_0x2f15e4() -> ! {
    todo!("0x2f15e4 __ZThn36_N3RBX12PartDragToolD1Ev")
}

#[doc(alias = "RBX::PartDragTool::drawConnectors(void)const")]
// 0x2f1830 — __ZNK3RBX12PartDragTool14drawConnectorsEv
pub fn stub_0x2f1830() -> ! {
    todo!("0x2f1830 __ZNK3RBX12PartDragTool14drawConnectorsEv")
}

#[doc(alias = "RBX::PartDragTool::getCursorName(void)const")]
// 0x2f1834 — __ZNK3RBX12PartDragTool13getCursorNameEv
pub fn stub_0x2f1834() -> ! {
    todo!("0x2f1834 __ZNK3RBX12PartDragTool13getCursorNameEv")
}

#[doc(alias = "RBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
// 0x2f6850 — __ZN3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE
pub fn stub_0x2f6850() -> ! {
    todo!("0x2f6850 __ZN3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::ArrowToolBase::renderHoverOver(RBX::Adorn *,bool)")]
// 0x2f6858 — __ZN3RBX13ArrowToolBase15renderHoverOverEPNS_5AdornEb
pub fn stub_0x2f6858() -> ! {
    todo!("0x2f6858 __ZN3RBX13ArrowToolBase15renderHoverOverEPNS_5AdornEb")
}

#[doc(alias = "`non-virtual thunk toRBX::ArrowToolBase::render3dAdorn(RBX::Adorn *)")]
// 0x2f68c8 — __ZThn4_N3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE
pub fn stub_0x2f68c8() -> ! {
    todo!("0x2f68c8 __ZThn4_N3RBX13ArrowToolBase13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
// 0x2f7818 — __ZN3RBX16BoxSelectCommand8render2dEPNS_5AdornE
pub fn stub_0x2f7818() -> ! {
    todo!("0x2f7818 __ZN3RBX16BoxSelectCommand8render2dEPNS_5AdornE")
}

#[doc(alias = "`non-virtual thunk toRBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
// 0x2f78d0 — __ZThn4_N3RBX16BoxSelectCommand8render2dEPNS_5AdornE
pub fn stub_0x2f78d0() -> ! {
    todo!("0x2f78d0 __ZThn4_N3RBX16BoxSelectCommand8render2dEPNS_5AdornE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x2f8624 — __ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_0x2f8624() -> ! {
    todo!("0x2f8624 __ZN5boost10shared_ptrIN3RBX16BoxSelectCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const")]
// 0x2f86ec — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0x2f86ec() -> ! {
    todo!("0x2f86ec __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16BoxSelectCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x2f87d0 — __ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_0x2f87d0() -> ! {
    todo!("0x2f87d0 __ZN5boost6detail12shared_countC2IPN3RBX16BoxSelectCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x2f88c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_0x2f88c8() -> ! {
    todo!("0x2f88c8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x2f88cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_0x2f88cc() -> ! {
    todo!("0x2f88cc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x2f88d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_0x2f88d0() -> ! {
    todo!("0x2f88d0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x2f88e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_0x2f88e0() -> ! {
    todo!("0x2f88e0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x2f88f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_0x2f88f8() -> ! {
    todo!("0x2f88f8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16BoxSelectCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::AsyncHttpQueue::processRequests(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>)")]
// 0x2fb548 — __ZN3RBX14AsyncHttpQueue15processRequestsEN5boost8weak_ptrIS0_EESt14_List_iteratorINS0_7RequestEENS1_10shared_ptrINS_5mutexEEE
pub fn stub_0x2fb548() -> ! {
    todo!("0x2fb548 __ZN3RBX14AsyncHttpQueue15processRequestsEN5boost8weak_ptrIS0_EESt14_List_iteratorINS0_7RequestEENS1_10shared_ptrINS_5mutexEEE")
}

#[doc(alias = "RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
// 0x2fc874 — __ZN3RBXL19InvokeAsyncCallbackEN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS0_10shared_ptrIKSsEEEEES3_S7_
pub fn stub_0x2fc874() -> ! {
    todo!("0x2fc874 __ZN3RBXL19InvokeAsyncCallbackEN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS0_10shared_ptrIKSsEEEEES3_S7_")
}

#[doc(alias = "RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)")]
// 0x2fd37c — __ZN3RBX14AsyncHttpQueue12asyncRequestERKSsfPN5boost8functionIFvNS0_13RequestResultEPSiNS3_10shared_ptrIS1_EEEEENS0_9ResultJobEb
pub fn stub_0x2fd37c() -> ! {
    todo!("0x2fd37c __ZN3RBX14AsyncHttpQueue12asyncRequestERKSsfPN5boost8functionIFvNS0_13RequestResultEPSiNS3_10shared_ptrIS1_EEEEENS0_9ResultJobEb")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::operator=(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const&)")]
// 0x2fded0 — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEaSERKS3_
pub fn stub_0x2fded0() -> ! {
    todo!("0x2fded0 __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEaSERKS3_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)")]
// 0x2fdfbc — __ZN5boost4bindIvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS3_7RequestEENS_10shared_ptrINS2_5mutexEEES4_S7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
pub fn stub_0x2fdfbc() -> ! {
    todo!("0x2fdfbc __ZN5boost4bindIvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS3_7RequestEENS_10shared_ptrINS2_5mutexEEES4_S7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
// 0x2fe168 — __ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_
pub fn stub_0x2fe168() -> ! {
    todo!("0x2fe168 __ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
// 0x2fe358 — __ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
pub fn stub_0x2fe358() -> ! {
    todo!("0x2fe358 __ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_")
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const")]
// 0x2fe524 — __ZNK5boost9function2IbRKSsPSsEclES2_S3_
pub fn stub_0x2fe524() -> ! {
    todo!("0x2fe524 __ZNK5boost9function2IbRKSsPSsEclES2_S3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::operator=(rbx_core::SharedPtr<RBX::Http> const&)")]
// 0x2fe5f0 — __ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_
pub fn stub_0x2fe5f0() -> ! {
    todo!("0x2fe5f0 __ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_")
}

#[doc(alias = "void rbx_core::SharedPtr<std::string>::reset<std::string>(std::string *)")]
// 0x2fe628 — __ZN5boost10shared_ptrISsE5resetISsEEvPT_
pub fn stub_0x2fe628() -> ! {
    todo!("0x2fe628 __ZN5boost10shared_ptrISsE5resetISsEEvPT_")
}

#[doc(alias = "RBX::AsyncHttpQueue::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// 0x2feaa8 — __ZN3RBX14AsyncHttpQueue15registerContentERKSsN5boost10shared_ptrIS1_EES5_
pub fn stub_0x2feaa8() -> ! {
    todo!("0x2feaa8 __ZN3RBX14AsyncHttpQueue15registerContentERKSsN5boost10shared_ptrIS1_EES5_")
}

#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> const&)")]
// 0x2fee80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_
pub fn stub_0x2fee80() -> ! {
    todo!("0x2fee80 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
// 0x2fef44 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_
pub fn stub_0x2fef44() -> ! {
    todo!("0x2fef44 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
// 0x2ff020 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_
pub fn stub_0x2ff020() -> ! {
    todo!("0x2ff020 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")]
// 0x2ffb24 — __ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_
pub fn stub_0x2ffb24() -> ! {
    todo!("0x2ffb24 __ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *)")]
// 0x2ffbfc — __ZN5boost6detail12shared_countC2IN3RBX4HttpEEEPT_
pub fn stub_0x2ffbfc() -> ! {
    todo!("0x2ffbfc __ZN5boost6detail12shared_countC2IN3RBX4HttpEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()")]
// 0x2ffd34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED1Ev
pub fn stub_0x2ffd34() -> ! {
    todo!("0x2ffd34 __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()")]
// 0x2ffd38 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED0Ev
pub fn stub_0x2ffd38() -> ! {
    todo!("0x2ffd38 __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::dispose(void)")]
// 0x2ffd3c — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE7disposeEv
pub fn stub_0x2ffd3c() -> ! {
    todo!("0x2ffd3c __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_deleter(std::type_info const&)")]
// 0x2ffe10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE11get_deleterERKSt9type_info
pub fn stub_0x2ffe10() -> ! {
    todo!("0x2ffe10 __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_untyped_deleter(void)")]
// 0x2ffe14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE19get_untyped_deleterEv
pub fn stub_0x2ffe14() -> ! {
    todo!("0x2ffe14 __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(rbx_core::WeakPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")]
// 0x2ffe1c — __ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_0x2ffe1c() -> ! {
    todo!("0x2ffe1c __ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::SharedPtr<std::string>::shared_ptr<std::string>(std::string *)")]
// 0x2ffe98 — __ZN5boost10shared_ptrISsEC2ISsEEPT_
pub fn stub_0x2ffe98() -> ! {
    todo!("0x2ffe98 __ZN5boost10shared_ptrISsEC2ISsEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::~sp_counted_impl_p()")]
// 0x2fff70 — __ZN5boost6detail17sp_counted_impl_pISsED0Ev
pub fn stub_0x2fff70() -> ! {
    todo!("0x2fff70 __ZN5boost6detail17sp_counted_impl_pISsED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::get_deleter(std::type_info const&)")]
// 0x2fff78 — __ZN5boost6detail17sp_counted_impl_pISsE11get_deleterERKSt9type_info
pub fn stub_0x2fff78() -> ! {
    todo!("0x2fff78 __ZN5boost6detail17sp_counted_impl_pISsE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x30039c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
pub fn stub_0x30039c() -> ! {
    todo!("0x30039c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x3008a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0x3008a4() -> ! {
    todo!("0x3008a4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
// 0x300a60 — __ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
pub fn stub_0x300a60() -> ! {
    todo!("0x300a60 __ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
// 0x300b6c — __ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
pub fn stub_0x300b6c() -> ! {
    todo!("0x300b6c __ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)")]
// 0x300c6c — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_
pub fn stub_0x300c6c() -> ! {
    todo!("0x300c6c __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")]
// 0x300f98 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_
pub fn stub_0x300f98() -> ! {
    todo!("0x300f98 __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x3010d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
pub fn stub_0x3010d8() -> ! {
    todo!("0x3010d8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
// 0x3010f4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
pub fn stub_0x3010f4() -> ! {
    todo!("0x3010f4 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0x30110c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0x30110c() -> ! {
    todo!("0x30110c __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x301238 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0x301238() -> ! {
    todo!("0x301238 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x301360 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0x301360() -> ! {
    todo!("0x301360 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&> &,int)")]
// 0x301478 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x301478() -> ! {
    todo!("0x301478 __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x3015d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0x3015d8() -> ! {
    todo!("0x3015d8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
// 0x301770 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
pub fn stub_0x301770() -> ! {
    todo!("0x301770 __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
// 0x30188c — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
pub fn stub_0x30188c() -> ! {
    todo!("0x30188c __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
// 0x3019a8 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_
pub fn stub_0x3019a8() -> ! {
    todo!("0x3019a8 __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(rbx_core::SharedPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")]
// 0x301afc — __ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
pub fn stub_0x301afc() -> ! {
    todo!("0x301afc __ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")]
// 0x3022c8 — __ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_
pub fn stub_0x3022c8() -> ! {
    todo!("0x3022c8 __ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::clear(void)")]
// 0x3022f8 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv
pub fn stub_0x3022f8() -> ! {
    todo!("0x3022f8 __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv")
}

#[doc(alias = "RBX::BrickColor::BrickMap::setRenderingSupportedPaletteSize(unsigned long)")]
// 0x3047f0 — __ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm
pub fn stub_0x3047f0() -> ! {
    todo!("0x3047f0 __ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm")
}

#[doc(alias = "RBX::staticDoFilterRequest(rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
// 0x30ef44 — __ZN3RBXL21staticDoFilterRequestEN5boost8weak_ptrINS_13ContentFilterEEESs
pub fn stub_0x30ef44() -> ! {
    todo!("0x30ef44 __ZN3RBXL21staticDoFilterRequestEN5boost8weak_ptrINS_13ContentFilterEEESs")
}

#[doc(alias = "RBX::staticDoFilterResult(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
// 0x30f598 — __ZN3RBXL20staticDoFilterResultEPSsPSt9exceptionN5boost8weak_ptrINS_13ContentFilterEEESs
pub fn stub_0x30f598() -> ! {
    todo!("0x30f598 __ZN3RBXL20staticDoFilterResultEPSsPSt9exceptionN5boost8weak_ptrINS_13ContentFilterEEESs")
}

#[doc(alias = "RBX::staticSaveFilterResult(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool)")]
// 0x30f8e8 — __ZN3RBXL22staticSaveFilterResultEN5boost8weak_ptrINS_13ContentFilterEEESsb
pub fn stub_0x30f8e8() -> ! {
    todo!("0x30f8e8 __ZN3RBXL22staticSaveFilterResultEN5boost8weak_ptrINS_13ContentFilterEEESsb")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::ContentFilter>,std::string>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>(void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
// 0x30faec — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13ContentFilterEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
pub fn stub_0x30faec() -> ! {
    todo!("0x30faec __ZN5boost4bindIvNS_8weak_ptrIN3RBX13ContentFilterEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ContentFilter> RBX::weak_from<RBX::ContentFilter>(RBX::ContentFilter*)")]
// 0x30fdbc — __ZN3RBX9weak_fromINS_13ContentFilterEEEN5boost8weak_ptrIT_EEPS4_
pub fn stub_0x30fdbc() -> ! {
    todo!("0x30fdbc __ZN3RBX9weak_fromINS_13ContentFilterEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>::type> boost::bind<void,std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>(void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
// 0x30ffb4 — __ZN5boost4bindIvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsNS_3argILi1EEENS8_ILi2EEES7_SsEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_
pub fn stub_0x30ffb4() -> ! {
    todo!("0x30ffb4 __ZN5boost4bindIvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsNS_3argILi1EEENS8_ILi2EEES7_SsEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)")]
// 0x310b80 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEEvT_
pub fn stub_0x310b80() -> ! {
    todo!("0x310b80 __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x310d54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
pub fn stub_0x310d54() -> ! {
    todo!("0x310d54 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x310d70 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
pub fn stub_0x310d70() -> ! {
    todo!("0x310d70 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0x310d90 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0x310d90() -> ! {
    todo!("0x310d90 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x310f54 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0x310f54() -> ! {
    todo!("0x310f54 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x311114 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0x311114() -> ! {
    todo!("0x311114 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x311258 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEclIPFvPSsPSt9exceptionS9_SsENS0_5list2IRSE_RSG_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x311258() -> ! {
    todo!("0x311258 __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEclIPFvPSsPSt9exceptionS9_SsENS0_5list2IRSE_RSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x31140c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0x31140c() -> ! {
    todo!("0x31140c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
// 0x3115e8 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_
pub fn stub_0x3115e8() -> ! {
    todo!("0x3115e8 __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_")
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
// 0x311794 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_
pub fn stub_0x311794() -> ! {
    todo!("0x311794 __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x311e60 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
pub fn stub_0x311e60() -> ! {
    todo!("0x311e60 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x312508 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0x312508() -> ! {
    todo!("0x312508 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
// 0x3126e4 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEC2ES7_S8_
pub fn stub_0x3126e4() -> ! {
    todo!("0x3126e4 __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEC2ES7_S8_")
}

#[doc(alias = "RBX::Http::getRobloxResponceLock(void)")]
// 0x316590 — __ZN3RBX4Http21getRobloxResponceLockEv
pub fn stub_0x316590() -> ! {
    todo!("0x316590 __ZN3RBX4Http21getRobloxResponceLockEv")
}

#[doc(alias = "RBX::Http::isRobloxSite(char const*)")]
// 0x3180dc — __ZN3RBX4Http12isRobloxSiteEPKc
pub fn stub_0x3180dc() -> ! {
    todo!("0x3180dc __ZN3RBX4Http12isRobloxSiteEPKc")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
// 0x326a5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_0x326a5c() -> ! {
    todo!("0x326a5c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,RBX::KeywordFilterType const&)")]
// 0x326ac4 — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x326ac4() -> ! {
    todo!("0x326ac4 __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_allocate(unsigned long)")]
// 0x326ba8 — __ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm
pub fn stub_0x326ba8() -> ! {
    todo!("0x326ba8 __ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm")
}
