//! core shard BC — 100 core stubs EA-sorted, next uncovered after BB 0x411cc8 (strict RBX|boost|std earliest gap, after BB 0x408518..0x411cc8).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x411cc8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvRotateTool>::shared_ptr<RBX::AdvRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x411db4 — __ZN5boost10shared_ptrIN3RBX13AdvRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::AdvRotateTool>::shared_ptr<RBX::AdvRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x411db4() -> ! {
    todo!("0x411db4 __ZN5boost10shared_ptrIN3RBX13AdvRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvRotateTool,RBX::AdvRotateTool>(rbx_core::SharedPtr<RBX::AdvRotateTool> const*,RBX::AdvRotateTool *)const")]
// 0x411e7c — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13AdvRotateToolES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvRotateTool,RBX::AdvRotateTool>(rbx_core::SharedPtr<RBX::AdvRotateTool> const*,RBX::AdvRotateTool *)const
pub fn stub_0x411e7c() -> ! {
    todo!("0x411e7c __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13AdvRotateToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x411f60 — __ZN5boost6detail12shared_countC2IPN3RBX13AdvRotateToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x411f60() -> ! {
    todo!("0x411f60 __ZN5boost6detail12shared_countC2IPN3RBX13AdvRotateToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x412058 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x412058() -> ! {
    todo!("0x412058 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x41205c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x41205c() -> ! {
    todo!("0x41205c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x412060 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x412060() -> ! {
    todo!("0x412060 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x412070 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x412070() -> ! {
    todo!("0x412070 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x412088 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x412088() -> ! {
    todo!("0x412088 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x41233c — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x41233c() -> ! {
    todo!("0x41233c __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x4123dc — __ZNK3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x4123dc() -> ! {
    todo!("0x4123dc __ZNK3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x412414 — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x412414() -> ! {
    todo!("0x412414 __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x412528 — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x412528() -> ! {
    todo!("0x412528 __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::AdvMoveTool::~AdvMoveTool()")]
// 0x4126e8 — __ZN3RBX11AdvMoveToolD1Ev — RBX::AdvMoveTool::~AdvMoveTool()
pub fn stub_0x4126e8() -> ! {
    todo!("0x4126e8 __ZN3RBX11AdvMoveToolD1Ev")
}

#[doc(alias = "RBX::AdvMoveTool::~AdvMoveTool()")]
// 0x4126ec — __ZN3RBX11AdvMoveToolD0Ev — RBX::AdvMoveTool::~AdvMoveTool()
pub fn stub_0x4126ec() -> ! {
    todo!("0x4126ec __ZN3RBX11AdvMoveToolD0Ev")
}

#[doc(alias = "RBX::AdvMoveTool::isSticky(void)const")]
// 0x41278c — __ZNK3RBX11AdvMoveTool8isStickyEv — RBX::AdvMoveTool::isSticky(void)const
pub fn stub_0x41278c() -> ! {
    todo!("0x41278c __ZNK3RBX11AdvMoveTool8isStickyEv")
}

#[doc(alias = "RBX::AdvMoveTool::getHandleColor(void)const")]
// 0x412854 — __ZNK3RBX11AdvMoveTool14getHandleColorEv — RBX::AdvMoveTool::getHandleColor(void)const
pub fn stub_0x412854() -> ! {
    todo!("0x412854 __ZNK3RBX11AdvMoveTool14getHandleColorEv")
}

#[doc(alias = "RBX::AdvMoveTool::getDragType(void)const")]
// 0x41286c — __ZNK3RBX11AdvMoveTool11getDragTypeEv — RBX::AdvMoveTool::getDragType(void)const
pub fn stub_0x41286c() -> ! {
    todo!("0x41286c __ZNK3RBX11AdvMoveTool11getDragTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool()")]
// 0x412870 — __ZThn36_N3RBX11AdvMoveToolD1Ev — non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool()
pub fn stub_0x412870() -> ! {
    todo!("0x412870 __ZThn36_N3RBX11AdvMoveToolD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool()")]
// 0x412878 — __ZThn36_N3RBX11AdvMoveToolD0Ev — non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool()
pub fn stub_0x412878() -> ! {
    todo!("0x412878 __ZThn36_N3RBX11AdvMoveToolD0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveTool>::shared_ptr<RBX::AdvMoveTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x412880 — __ZN5boost10shared_ptrIN3RBX11AdvMoveToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::AdvMoveTool>::shared_ptr<RBX::AdvMoveTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x412880() -> ! {
    todo!("0x412880 __ZN5boost10shared_ptrIN3RBX11AdvMoveToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvMoveTool,RBX::AdvMoveTool>(rbx_core::SharedPtr<RBX::AdvMoveTool> const*,RBX::AdvMoveTool *)const")]
// 0x412948 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11AdvMoveToolES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvMoveTool,RBX::AdvMoveTool>(rbx_core::SharedPtr<RBX::AdvMoveTool> const*,RBX::AdvMoveTool *)const
pub fn stub_0x412948() -> ! {
    todo!("0x412948 __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11AdvMoveToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x412a2c — __ZN5boost6detail12shared_countC2IPN3RBX11AdvMoveToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x412a2c() -> ! {
    todo!("0x412a2c __ZN5boost6detail12shared_countC2IPN3RBX11AdvMoveToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x412b24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x412b24() -> ! {
    todo!("0x412b24 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x412b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x412b28() -> ! {
    todo!("0x412b28 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x412b2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x412b2c() -> ! {
    todo!("0x412b2c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x412b3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x412b3c() -> ! {
    todo!("0x412b3c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x412b54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x412b54() -> ! {
    todo!("0x412b54 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x412e08 — __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x412e08() -> ! {
    todo!("0x412e08 __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x412ea8 — __ZNK3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x412ea8() -> ! {
    todo!("0x412ea8 __ZNK3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x412ee0 — __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x412ee0() -> ! {
    todo!("0x412ee0 __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x412ff4 — __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x412ff4() -> ! {
    todo!("0x412ff4 __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::MoveResizeJoinTool::isSticky(void)const")]
// 0x4132b4 — __ZNK3RBX18MoveResizeJoinTool8isStickyEv — RBX::MoveResizeJoinTool::isSticky(void)const
pub fn stub_0x4132b4() -> ! {
    todo!("0x4132b4 __ZNK3RBX18MoveResizeJoinTool8isStickyEv")
}

#[doc(alias = "RBX::MoveResizeJoinTool::drawConnectors(void)const")]
// 0x41337c — __ZNK3RBX18MoveResizeJoinTool14drawConnectorsEv — RBX::MoveResizeJoinTool::drawConnectors(void)const
pub fn stub_0x41337c() -> ! {
    todo!("0x41337c __ZNK3RBX18MoveResizeJoinTool14drawConnectorsEv")
}

#[doc(alias = "RBX::MoveResizeJoinTool::getCursorName(void)const")]
// 0x413380 — __ZNK3RBX18MoveResizeJoinTool13getCursorNameEv — RBX::MoveResizeJoinTool::getCursorName(void)const
pub fn stub_0x413380() -> ! {
    todo!("0x413380 __ZNK3RBX18MoveResizeJoinTool13getCursorNameEv")
}

#[doc(alias = "RBX::MoveResizeJoinTool::setCursor(std::string)")]
// 0x41338c — __ZN3RBX18MoveResizeJoinTool9setCursorESs — RBX::MoveResizeJoinTool::setCursor(std::string)
pub fn stub_0x41338c() -> ! {
    todo!("0x41338c __ZN3RBX18MoveResizeJoinTool9setCursorESs")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool>::shared_ptr<RBX::MoveResizeJoinTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x413394 — __ZN5boost10shared_ptrIN3RBX18MoveResizeJoinToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::MoveResizeJoinTool>::shared_ptr<RBX::MoveResizeJoinTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x413394() -> ! {
    todo!("0x413394 __ZN5boost10shared_ptrIN3RBX18MoveResizeJoinToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MoveResizeJoinTool,RBX::MoveResizeJoinTool>(rbx_core::SharedPtr<RBX::MoveResizeJoinTool> const*,RBX::MoveResizeJoinTool *)const")]
// 0x41345c — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18MoveResizeJoinToolES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MoveResizeJoinTool,RBX::MoveResizeJoinTool>(rbx_core::SharedPtr<RBX::MoveResizeJoinTool> const*,RBX::MoveResizeJoinTool *)const
pub fn stub_0x41345c() -> ! {
    todo!("0x41345c __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18MoveResizeJoinToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x413540 — __ZN5boost6detail12shared_countC2IPN3RBX18MoveResizeJoinToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x413540() -> ! {
    todo!("0x413540 __ZN5boost6detail12shared_countC2IPN3RBX18MoveResizeJoinToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x413638 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x413638() -> ! {
    todo!("0x413638 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x41363c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x41363c() -> ! {
    todo!("0x41363c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x413640 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x413640() -> ! {
    todo!("0x413640 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x413650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x413650() -> ! {
    todo!("0x413650 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x413668 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::MoveResizeJoinTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x413668() -> ! {
    todo!("0x413668 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MoveResizeJoinToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x41391c — __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x41391c() -> ! {
    todo!("0x41391c __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x4139bc — __ZNK3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x4139bc() -> ! {
    todo!("0x4139bc __ZNK3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x4139f0 — __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x4139f0() -> ! {
    todo!("0x4139f0 __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x413b04 — __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x413b04() -> ! {
    todo!("0x413b04 __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::AxisRotateTool::~AxisRotateTool()")]
// 0x413cac — __ZN3RBX14AxisRotateToolD1Ev — RBX::AxisRotateTool::~AxisRotateTool()
pub fn stub_0x413cac() -> ! {
    todo!("0x413cac __ZN3RBX14AxisRotateToolD1Ev")
}

#[doc(alias = "RBX::AxisRotateTool::~AxisRotateTool()")]
// 0x413d90 — __ZN3RBX14AxisRotateToolD0Ev — RBX::AxisRotateTool::~AxisRotateTool()
pub fn stub_0x413d90() -> ! {
    todo!("0x413d90 __ZN3RBX14AxisRotateToolD0Ev")
}

#[doc(alias = "RBX::AxisRotateTool::isSticky(void)const")]
// 0x413e88 — __ZNK3RBX14AxisRotateTool8isStickyEv — RBX::AxisRotateTool::isSticky(void)const
pub fn stub_0x413e88() -> ! {
    todo!("0x413e88 __ZNK3RBX14AxisRotateTool8isStickyEv")
}

#[doc(alias = "RBX::AxisToolBase::drawConnectors(void)const")]
// 0x413f50 — __ZNK3RBX12AxisToolBase14drawConnectorsEv — RBX::AxisToolBase::drawConnectors(void)const
pub fn stub_0x413f50() -> ! {
    todo!("0x413f50 __ZNK3RBX12AxisToolBase14drawConnectorsEv")
}

#[doc(alias = "RBX::AxisToolBase::getCursorName(void)const")]
// 0x413f54 — __ZNK3RBX12AxisToolBase13getCursorNameEv — RBX::AxisToolBase::getCursorName(void)const
pub fn stub_0x413f54() -> ! {
    todo!("0x413f54 __ZNK3RBX12AxisToolBase13getCursorNameEv")
}

#[doc(alias = "RBX::AxisRotateTool::getHandleColor(void)const")]
// 0x413f60 — __ZNK3RBX14AxisRotateTool14getHandleColorEv — RBX::AxisRotateTool::getHandleColor(void)const
pub fn stub_0x413f60() -> ! {
    todo!("0x413f60 __ZNK3RBX14AxisRotateTool14getHandleColorEv")
}

#[doc(alias = "RBX::AxisRotateTool::getDragType(void)const")]
// 0x413f78 — __ZNK3RBX14AxisRotateTool11getDragTypeEv — RBX::AxisRotateTool::getDragType(void)const
pub fn stub_0x413f78() -> ! {
    todo!("0x413f78 __ZNK3RBX14AxisRotateTool11getDragTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::AxisRotateTool::~AxisRotateTool()")]
// 0x413f7c — __ZThn36_N3RBX14AxisRotateToolD1Ev — non-virtual thunk toRBX::AxisRotateTool::~AxisRotateTool()
pub fn stub_0x413f7c() -> ! {
    todo!("0x413f7c __ZThn36_N3RBX14AxisRotateToolD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AxisRotateTool::~AxisRotateTool()")]
// 0x414060 — __ZThn36_N3RBX14AxisRotateToolD0Ev — non-virtual thunk toRBX::AxisRotateTool::~AxisRotateTool()
pub fn stub_0x414060() -> ! {
    todo!("0x414060 __ZThn36_N3RBX14AxisRotateToolD0Ev")
}

#[doc(alias = "RBX::AxisToolBase::~AxisToolBase()")]
// 0x414158 — __ZN3RBX12AxisToolBaseD1Ev — RBX::AxisToolBase::~AxisToolBase()
pub fn stub_0x414158() -> ! {
    todo!("0x414158 __ZN3RBX12AxisToolBaseD1Ev")
}

#[doc(alias = "RBX::AxisToolBase::~AxisToolBase()")]
// 0x41423c — __ZN3RBX12AxisToolBaseD0Ev — RBX::AxisToolBase::~AxisToolBase()
pub fn stub_0x41423c() -> ! {
    todo!("0x41423c __ZN3RBX12AxisToolBaseD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AxisToolBase::~AxisToolBase()")]
// 0x414334 — __ZThn36_N3RBX12AxisToolBaseD1Ev — non-virtual thunk toRBX::AxisToolBase::~AxisToolBase()
pub fn stub_0x414334() -> ! {
    todo!("0x414334 __ZThn36_N3RBX12AxisToolBaseD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AxisToolBase::~AxisToolBase()")]
// 0x414418 — __ZThn36_N3RBX12AxisToolBaseD0Ev — non-virtual thunk toRBX::AxisToolBase::~AxisToolBase()
pub fn stub_0x414418() -> ! {
    todo!("0x414418 __ZThn36_N3RBX12AxisToolBaseD0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AxisRotateTool>::shared_ptr<RBX::AxisRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x414510 — __ZN5boost10shared_ptrIN3RBX14AxisRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_ — rbx_core::SharedPtr<RBX::AxisRotateTool>::shared_ptr<RBX::AxisRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x414510() -> ! {
    todo!("0x414510 __ZN5boost10shared_ptrIN3RBX14AxisRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AxisRotateTool,RBX::AxisRotateTool>(rbx_core::SharedPtr<RBX::AxisRotateTool> const*,RBX::AxisRotateTool *)const")]
// 0x4145d8 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14AxisRotateToolES5_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AxisRotateTool,RBX::AxisRotateTool>(rbx_core::SharedPtr<RBX::AxisRotateTool> const*,RBX::AxisRotateTool *)const
pub fn stub_0x4145d8() -> ! {
    todo!("0x4145d8 __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14AxisRotateToolES5_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// 0x4146bc — __ZN5boost6detail12shared_countC2IPN3RBX14AxisRotateToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)
pub fn stub_0x4146bc() -> ! {
    todo!("0x4146bc __ZN5boost6detail12shared_countC2IPN3RBX14AxisRotateToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x4147b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x4147b4() -> ! {
    todo!("0x4147b4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// 0x4147b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
pub fn stub_0x4147b8() -> ! {
    todo!("0x4147b8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// 0x4147bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
pub fn stub_0x4147bc() -> ! {
    todo!("0x4147bc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// 0x4147cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_0x4147cc() -> ! {
    todo!("0x4147cc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// 0x4147e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::AxisRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
pub fn stub_0x4147e4() -> ! {
    todo!("0x4147e4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AxisRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::MoveUpBrickVerb::~MoveUpBrickVerb()")]
// 0x414f80 — __ZN3RBX15MoveUpBrickVerbD0Ev — RBX::MoveUpBrickVerb::~MoveUpBrickVerb()
pub fn stub_0x414f80() -> ! {
    todo!("0x414f80 __ZN3RBX15MoveUpBrickVerbD0Ev")
}

#[doc(alias = "RBX::MoveUpPlateVerb::~MoveUpPlateVerb()")]
// 0x415170 — __ZN3RBX15MoveUpPlateVerbD0Ev — RBX::MoveUpPlateVerb::~MoveUpPlateVerb()
pub fn stub_0x415170() -> ! {
    todo!("0x415170 __ZN3RBX15MoveUpPlateVerbD0Ev")
}

#[doc(alias = "RBX::CanCollideVerb::~CanCollideVerb()")]
// 0x4155f4 — __ZN3RBX14CanCollideVerbD0Ev — RBX::CanCollideVerb::~CanCollideVerb()
pub fn stub_0x4155f4() -> ! {
    todo!("0x4155f4 __ZN3RBX14CanCollideVerbD0Ev")
}

#[doc(alias = "RBX::TranslucentVerb::~TranslucentVerb()")]
// 0x4157e8 — __ZN3RBX15TranslucentVerbD0Ev — RBX::TranslucentVerb::~TranslucentVerb()
pub fn stub_0x4157e8() -> ! {
    todo!("0x4157e8 __ZN3RBX15TranslucentVerbD0Ev")
}

#[doc(alias = "RBX::AnchorVerb::~AnchorVerb()")]
// 0x4159dc — __ZN3RBX10AnchorVerbD0Ev — RBX::AnchorVerb::~AnchorVerb()
pub fn stub_0x4159dc() -> ! {
    todo!("0x4159dc __ZN3RBX10AnchorVerbD0Ev")
}

#[doc(alias = "RBX::DeleteSelectionVerb::~DeleteSelectionVerb()")]
// 0x415e58 — __ZN3RBX19DeleteSelectionVerbD0Ev — RBX::DeleteSelectionVerb::~DeleteSelectionVerb()
pub fn stub_0x415e58() -> ! {
    todo!("0x415e58 __ZN3RBX19DeleteSelectionVerbD0Ev")
}

#[doc(alias = "RBX::PlayDeleteSelectionVerb::~PlayDeleteSelectionVerb()")]
// 0x416044 — __ZN3RBX23PlayDeleteSelectionVerbD0Ev — RBX::PlayDeleteSelectionVerb::~PlayDeleteSelectionVerb()
pub fn stub_0x416044() -> ! {
    todo!("0x416044 __ZN3RBX23PlayDeleteSelectionVerbD0Ev")
}

#[doc(alias = "RBX::Configuration::Configuration(void)")]
// 0x416388 — __ZN3RBX13ConfigurationC2Ev — RBX::Configuration::Configuration(void)
pub fn stub_0x416388() -> ! {
    todo!("0x416388 __ZN3RBX13ConfigurationC2Ev")
}

#[doc(alias = "RBX::Configuration::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x416674 — __ZN3RBX13Configuration17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::Configuration::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_0x416674() -> ! {
    todo!("0x416674 __ZN3RBX13Configuration17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Configuration> RBX::shared_from<RBX::Configuration>(RBX::Configuration*)")]
// 0x416808 — __ZN3RBX11shared_fromINS_13ConfigurationEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::Configuration> RBX::shared_from<RBX::Configuration>(RBX::Configuration*)
pub fn stub_0x416808() -> ! {
    todo!("0x416808 __ZN3RBX11shared_fromINS_13ConfigurationEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::Configuration::~Configuration()")]
// 0x416978 — __ZN3RBX13ConfigurationD1Ev — RBX::Configuration::~Configuration()
pub fn stub_0x416978() -> ! {
    todo!("0x416978 __ZN3RBX13ConfigurationD1Ev")
}

#[doc(alias = "RBX::Configuration::~Configuration()")]
// 0x41697c — __ZN3RBX13ConfigurationD0Ev — RBX::Configuration::~Configuration()
pub fn stub_0x41697c() -> ! {
    todo!("0x41697c __ZN3RBX13ConfigurationD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration()")]
// 0x416a2c — __ZThn32_N3RBX13ConfigurationD1Ev — non-virtual thunk toRBX::Configuration::~Configuration()
pub fn stub_0x416a2c() -> ! {
    todo!("0x416a2c __ZThn32_N3RBX13ConfigurationD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration()")]
// 0x416a34 — __ZThn32_N3RBX13ConfigurationD0Ev — non-virtual thunk toRBX::Configuration::~Configuration()
pub fn stub_0x416a34() -> ! {
    todo!("0x416a34 __ZThn32_N3RBX13ConfigurationD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration()")]
// 0x416ae8 — __ZThn36_N3RBX13ConfigurationD1Ev — non-virtual thunk toRBX::Configuration::~Configuration()
pub fn stub_0x416ae8() -> ! {
    todo!("0x416ae8 __ZThn36_N3RBX13ConfigurationD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Configuration::~Configuration()")]
// 0x416af0 — __ZThn36_N3RBX13ConfigurationD0Ev — non-virtual thunk toRBX::Configuration::~Configuration()
pub fn stub_0x416af0() -> ! {
    todo!("0x416af0 __ZThn36_N3RBX13ConfigurationD0Ev")
}

#[doc(alias = "RBX::registerNewImageAPI(void)")]
// 0x420c38 — __ZN3RBXL19registerNewImageAPIEv — RBX::registerNewImageAPI(void)
pub fn stub_0x420c38() -> ! {
    todo!("0x420c38 __ZN3RBXL19registerNewImageAPIEv")
}

#[doc(alias = "RBX::GreaterProjectedPosition(RBX::IAdornable const*,RBX::IAdornable const*)")]
// 0x4259b8 — __ZN3RBXL24GreaterProjectedPositionEPKNS_10IAdornableES2_ — RBX::GreaterProjectedPosition(RBX::IAdornable const*,RBX::IAdornable const*)
pub fn stub_0x4259b8() -> ! {
    todo!("0x4259b8 __ZN3RBXL24GreaterProjectedPositionEPKNS_10IAdornableES2_")
}

#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// 0x4336d8 — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEE — RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)
pub fn stub_0x4336d8() -> ! {
    todo!("0x4336d8 __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "RBX::ContentFilter * RBX::ServiceProvider::create<RBX::ContentFilter>(void)const")]
// 0x433940 — __ZNK3RBX15ServiceProvider6createINS_13ContentFilterEEEPT_v — RBX::ContentFilter * RBX::ServiceProvider::create<RBX::ContentFilter>(void)const
pub fn stub_0x433940() -> ! {
    todo!("0x433940 __ZNK3RBX15ServiceProvider6createINS_13ContentFilterEEEPT_v")
}

#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(void)const")]
// 0x433b08 — __ZNK3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_v — RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(void)const
pub fn stub_0x433b08() -> ! {
    todo!("0x433b08 __ZNK3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_v")
}

#[doc(alias = "RBX::GuiService * RBX::ServiceProvider::create<RBX::GuiService>(void)const")]
// 0x433cd0 — __ZNK3RBX15ServiceProvider6createINS_10GuiServiceEEEPT_v — RBX::GuiService * RBX::ServiceProvider::create<RBX::GuiService>(void)const
pub fn stub_0x433cd0() -> ! {
    todo!("0x433cd0 __ZNK3RBX15ServiceProvider6createINS_10GuiServiceEEEPT_v")
}

#[doc(alias = "RBX::ChatService * RBX::ServiceProvider::create<RBX::ChatService>(void)const")]
// 0x433e98 — __ZNK3RBX15ServiceProvider6createINS_11ChatServiceEEEPT_v — RBX::ChatService * RBX::ServiceProvider::create<RBX::ChatService>(void)const
pub fn stub_0x433e98() -> ! {
    todo!("0x433e98 __ZNK3RBX15ServiceProvider6createINS_11ChatServiceEEEPT_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack>::operator=(rbx_core::SharedPtr<RBX::LocalBackpack> const&)")]
// 0x434078 — __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEaSERKS3_ — rbx_core::SharedPtr<RBX::LocalBackpack>::operator=(rbx_core::SharedPtr<RBX::LocalBackpack> const&)
pub fn stub_0x434078() -> ! {
    todo!("0x434078 __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack> RBX::shared_from<RBX::LocalBackpack>(RBX::LocalBackpack*)")]
// 0x4340b0 — __ZN3RBX11shared_fromINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::LocalBackpack> RBX::shared_from<RBX::LocalBackpack>(RBX::LocalBackpack*)
pub fn stub_0x4340b0() -> ! {
    todo!("0x4340b0 __ZN3RBX11shared_fromINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::LocalBackpack * RBX::ServiceProvider::create<RBX::LocalBackpack>(void)const")]
// 0x434198 — __ZNK3RBX15ServiceProvider6createINS_13LocalBackpackEEEPT_v — RBX::LocalBackpack * RBX::ServiceProvider::create<RBX::LocalBackpack>(void)const
pub fn stub_0x434198() -> ! {
    todo!("0x434198 __ZNK3RBX15ServiceProvider6createINS_13LocalBackpackEEEPT_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerHUD>::operator=(rbx_core::SharedPtr<RBX::PlayerHUD> const&)")]
// 0x434360 — __ZN5boost10shared_ptrIN3RBX9PlayerHUDEEaSERKS3_ — rbx_core::SharedPtr<RBX::PlayerHUD>::operator=(rbx_core::SharedPtr<RBX::PlayerHUD> const&)
pub fn stub_0x434360() -> ! {
    todo!("0x434360 __ZN5boost10shared_ptrIN3RBX9PlayerHUDEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterPackService>::operator=(rbx_core::SharedPtr<RBX::StarterPackService> const&)")]
// 0x434448 — __ZN5boost10shared_ptrIN3RBX18StarterPackServiceEEaSERKS3_ — rbx_core::SharedPtr<RBX::StarterPackService>::operator=(rbx_core::SharedPtr<RBX::StarterPackService> const&)
pub fn stub_0x434448() -> ! {
    todo!("0x434448 __ZN5boost10shared_ptrIN3RBX18StarterPackServiceEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterPackService> RBX::shared_from<RBX::StarterPackService>(RBX::StarterPackService*)")]
// 0x434480 — __ZN3RBX11shared_fromINS_18StarterPackServiceEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::StarterPackService> RBX::shared_from<RBX::StarterPackService>(RBX::StarterPackService*)
pub fn stub_0x434480() -> ! {
    todo!("0x434480 __ZN3RBX11shared_fromINS_18StarterPackServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::StarterPackService * RBX::ServiceProvider::create<RBX::StarterPackService>(void)const")]
// 0x434568 — __ZNK3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_v — RBX::StarterPackService * RBX::ServiceProvider::create<RBX::StarterPackService>(void)const
pub fn stub_0x434568() -> ! {
    todo!("0x434568 __ZNK3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService>::operator=(rbx_core::SharedPtr<RBX::StarterGuiService> const&)")]
// 0x434730 — __ZN5boost10shared_ptrIN3RBX17StarterGuiServiceEEaSERKS3_ — rbx_core::SharedPtr<RBX::StarterGuiService>::operator=(rbx_core::SharedPtr<RBX::StarterGuiService> const&)
pub fn stub_0x434730() -> ! {
    todo!("0x434730 __ZN5boost10shared_ptrIN3RBX17StarterGuiServiceEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService> RBX::shared_from<RBX::StarterGuiService>(RBX::StarterGuiService*)")]
// 0x434768 — __ZN3RBX11shared_fromINS_17StarterGuiServiceEEEN5boost10shared_ptrIT_EEPS4_ — rbx_core::SharedPtr<RBX::StarterGuiService> RBX::shared_from<RBX::StarterGuiService>(RBX::StarterGuiService*)
pub fn stub_0x434768() -> ! {
    todo!("0x434768 __ZN3RBX11shared_fromINS_17StarterGuiServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

