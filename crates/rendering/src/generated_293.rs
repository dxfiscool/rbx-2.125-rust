//! rendering shard 293 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 31840->31940 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31840 before -> 31940 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x4097f8

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4097fc — __ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v
pub fn stub_4097fc() -> ! {
    todo!("0x4097fc __ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v")
}

// 0x4098e0 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_4098e0() -> ! {
    todo!("0x4098e0 RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x409a64 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED0Ev
pub fn stub_409a64() -> ! {
    todo!("0x409a64 RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x409b04 — __ZNK3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_409b04() -> ! {
    todo!("0x409b04 RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x409b3c — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_409b3c() -> ! {
    todo!("0x409b3c RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x409c50 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_409c50() -> ! {
    todo!("0x409c50 RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x409d1c — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10AnchorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AnchorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10AnchorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_409d1c() -> ! {
    todo!("0x409d1c boost::shared_ptr<RBX::AnchorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AnchorTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x409dfc — __ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv
pub fn stub_409dfc() -> ! {
    todo!("0x409dfc __ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv")
}

// 0x409e00 — __ZN3RBX10AnchorTool9onMouseUpERKNS_7UIEventE
// type: void __fastcall(RBX::AnchorTool *this, const RBX::UIEvent *)
#[doc(alias = "RBX::AnchorTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX10AnchorTool9onMouseUpERKNS_7UIEventE
pub fn stub_409e00() -> ! {
    todo!("0x409e00 RBX::AnchorTool::onMouseUp(RBX::UIEvent const&)")
}

// 0x409ec8 — __ZN3RBX11shared_fromINS_10AnchorToolEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::shared_from<RBX::AnchorTool>(RBX::AnchorTool*)")]
// was: __ZN3RBX11shared_fromINS_10AnchorToolEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_409ec8() -> ! {
    todo!("0x409ec8 boost::shared_ptr<RBX::AnchorTool> RBX::shared_from<RBX::AnchorTool>(RBX::AnchorTool*)")
}

// 0x40a030 — __ZN5boost10shared_ptrIN3RBX10AnchorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool>::shared_ptr<RBX::AnchorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10AnchorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_40a030() -> ! {
    todo!("0x40a030 boost::shared_ptr<RBX::AnchorTool>::shared_ptr<RBX::AnchorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40a0f8 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_10AnchorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AnchorTool,RBX::AnchorTool>(rbx_core::SharedPtr<RBX::AnchorTool> const*,RBX::AnchorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_10AnchorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_40a0f8() -> ! {
    todo!("0x40a0f8 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AnchorTool,RBX::AnchorTool>(boost::shared_ptr<RBX::AnchorTool> const*,RBX::AnchorTool *)const")
}

// 0x40a1dc — __ZN5boost6detail12shared_countC2IPN3RBX10AnchorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10AnchorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_40a1dc() -> ! {
    todo!("0x40a1dc boost::detail::shared_count::shared_count<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40a2d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_40a2d4() -> ! {
    todo!("0x40a2d4 boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40a2d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_40a2d8() -> ! {
    todo!("0x40a2d8 boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40a2dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_40a2dc() -> ! {
    todo!("0x40a2dc boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x40a2ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_40a2ec() -> ! {
    todo!("0x40a2ec boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x40a304 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_40a304() -> ! {
    todo!("0x40a304 boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x40a308 — __ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v
pub fn stub_40a308() -> ! {
    todo!("0x40a308 __ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v")
}

// 0x40a34c — __ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv
pub fn stub_40a34c() -> ! {
    todo!("0x40a34c __ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv")
}

// 0x40a350 — __ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v
pub fn stub_40a350() -> ! {
    todo!("0x40a350 __ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v")
}

// 0x40a434 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_40a434() -> ! {
    todo!("0x40a434 RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x40a5b8 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED0Ev
pub fn stub_40a5b8() -> ! {
    todo!("0x40a5b8 RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40a658 — __ZNK3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_40a658() -> ! {
    todo!("0x40a658 RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x40a690 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_40a690() -> ! {
    todo!("0x40a690 RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x40a7a4 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_40a7a4() -> ! {
    todo!("0x40a7a4 RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x40a870 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_20SmoothNoOutlinesToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::SmoothNoOutlinesTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_20SmoothNoOutlinesToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_40a870() -> ! {
    todo!("0x40a870 boost::shared_ptr<RBX::SmoothNoOutlinesTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::SmoothNoOutlinesTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x40a948 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv
pub fn stub_40a948() -> ! {
    todo!("0x40a948 __ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv")
}

// 0x40a94c — __ZNK3RBX20SmoothNoOutlinesTool8isStickyEv
// type: void __fastcall(RBX::SmoothNoOutlinesTool *this, int)
#[doc(alias = "RBX::SmoothNoOutlinesTool::isSticky(void)const")]
// was: __ZNK3RBX20SmoothNoOutlinesTool8isStickyEv
pub fn stub_40a94c() -> ! {
    todo!("0x40a94c RBX::SmoothNoOutlinesTool::isSticky(void)const")
}

// 0x40aa14 — __ZNK3RBX20SmoothNoOutlinesTool13getCursorNameEv
// type: int __fastcall(RBX::SmoothNoOutlinesTool *this)
#[doc(alias = "RBX::SmoothNoOutlinesTool::getCursorName(void)const")]
// was: __ZNK3RBX20SmoothNoOutlinesTool13getCursorNameEv
pub fn stub_40aa14() -> ! {
    todo!("0x40aa14 RBX::SmoothNoOutlinesTool::getCursorName(void)const")
}

// 0x40aa30 — __ZN5boost10shared_ptrIN3RBX20SmoothNoOutlinesToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool>::shared_ptr<RBX::SmoothNoOutlinesTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX20SmoothNoOutlinesToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_40aa30() -> ! {
    todo!("0x40aa30 boost::shared_ptr<RBX::SmoothNoOutlinesTool>::shared_ptr<RBX::SmoothNoOutlinesTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40aaf8 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_20SmoothNoOutlinesToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::SmoothNoOutlinesTool,RBX::SmoothNoOutlinesTool>(rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> const*,RBX::SmoothNoOutlinesTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_20SmoothNoOutlinesToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_40aaf8() -> ! {
    todo!("0x40aaf8 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::SmoothNoOutlinesTool,RBX::SmoothNoOutlinesTool>(boost::shared_ptr<RBX::SmoothNoOutlinesTool> const*,RBX::SmoothNoOutlinesTool *)const")
}

// 0x40abdc — __ZN5boost6detail12shared_countC2IPN3RBX20SmoothNoOutlinesToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX20SmoothNoOutlinesToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_40abdc() -> ! {
    todo!("0x40abdc boost::detail::shared_count::shared_count<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40acd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_40acd4() -> ! {
    todo!("0x40acd4 boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40acd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_40acd8() -> ! {
    todo!("0x40acd8 boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40acdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_40acdc() -> ! {
    todo!("0x40acdc boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x40acec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_40acec() -> ! {
    todo!("0x40acec boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x40ad04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_40ad04() -> ! {
    todo!("0x40ad04 boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x40ad08 — __ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
pub fn stub_40ad08() -> ! {
    todo!("0x40ad08 __ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")
}

// 0x40ad4c — __ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv
pub fn stub_40ad4c() -> ! {
    todo!("0x40ad4c __ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv")
}

// 0x40ad50 — __ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
pub fn stub_40ad50() -> ! {
    todo!("0x40ad50 __ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")
}

// 0x40ae34 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_40ae34() -> ! {
    todo!("0x40ae34 RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x40afb8 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED0Ev
pub fn stub_40afb8() -> ! {
    todo!("0x40afb8 RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40b058 — __ZNK3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_40b058() -> ! {
    todo!("0x40b058 RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x40b090 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_40b090() -> ! {
    todo!("0x40b090 RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x40b1a4 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_40b1a4() -> ! {
    todo!("0x40b1a4 RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x40b270 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18OscillateMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::OscillateMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::OscillateMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18OscillateMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_40b270() -> ! {
    todo!("0x40b270 boost::shared_ptr<RBX::OscillateMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::OscillateMotorTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x40b348 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv
pub fn stub_40b348() -> ! {
    todo!("0x40b348 __ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv")
}

// 0x40b34c — __ZNK3RBX18OscillateMotorTool13getCursorNameEv
// type: int __fastcall(RBX::OscillateMotorTool *this)
#[doc(alias = "RBX::OscillateMotorTool::getCursorName(void)const")]
// was: __ZNK3RBX18OscillateMotorTool13getCursorNameEv
pub fn stub_40b34c() -> ! {
    todo!("0x40b34c RBX::OscillateMotorTool::getCursorName(void)const")
}

// 0x40b368 — __ZN5boost10shared_ptrIN3RBX18OscillateMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::OscillateMotorTool>::shared_ptr<RBX::OscillateMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX18OscillateMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_40b368() -> ! {
    todo!("0x40b368 boost::shared_ptr<RBX::OscillateMotorTool>::shared_ptr<RBX::OscillateMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40b430 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18OscillateMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::OscillateMotorTool,RBX::OscillateMotorTool>(rbx_core::SharedPtr<RBX::OscillateMotorTool> const*,RBX::OscillateMotorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18OscillateMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_40b430() -> ! {
    todo!("0x40b430 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::OscillateMotorTool,RBX::OscillateMotorTool>(boost::shared_ptr<RBX::OscillateMotorTool> const*,RBX::OscillateMotorTool *)const")
}

// 0x40b514 — __ZN5boost6detail12shared_countC2IPN3RBX18OscillateMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX18OscillateMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_40b514() -> ! {
    todo!("0x40b514 boost::detail::shared_count::shared_count<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40b60c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_40b60c() -> ! {
    todo!("0x40b60c boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40b610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_40b610() -> ! {
    todo!("0x40b610 boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40b614 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_40b614() -> ! {
    todo!("0x40b614 boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x40b624 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_40b624() -> ! {
    todo!("0x40b624 boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x40b63c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_40b63c() -> ! {
    todo!("0x40b63c boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x40b640 — __ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v
pub fn stub_40b640() -> ! {
    todo!("0x40b640 __ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v")
}

// 0x40b684 — __ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv
pub fn stub_40b684() -> ! {
    todo!("0x40b684 __ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv")
}

// 0x40b688 — __ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v
pub fn stub_40b688() -> ! {
    todo!("0x40b688 __ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v")
}

// 0x40b76c — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_40b76c() -> ! {
    todo!("0x40b76c RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x40b8f0 — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED0Ev
pub fn stub_40b8f0() -> ! {
    todo!("0x40b8f0 RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40b990 — __ZNK3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_40b990() -> ! {
    todo!("0x40b990 RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x40b9c8 — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_40b9c8() -> ! {
    todo!("0x40b9c8 RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x40badc — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_40badc() -> ! {
    todo!("0x40badc RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x40bba8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13LeftMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::LeftMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LeftMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13LeftMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_40bba8() -> ! {
    todo!("0x40bba8 boost::shared_ptr<RBX::LeftMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LeftMotorTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x40bc80 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv
pub fn stub_40bc80() -> ! {
    todo!("0x40bc80 __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv")
}

// 0x40bc84 — __ZNK3RBX13LeftMotorTool13getCursorNameEv
// type: int __fastcall(RBX::LeftMotorTool *this)
#[doc(alias = "RBX::LeftMotorTool::getCursorName(void)const")]
// was: __ZNK3RBX13LeftMotorTool13getCursorNameEv
pub fn stub_40bc84() -> ! {
    todo!("0x40bc84 RBX::LeftMotorTool::getCursorName(void)const")
}

// 0x40bca0 — __ZN5boost10shared_ptrIN3RBX13LeftMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LeftMotorTool>::shared_ptr<RBX::LeftMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13LeftMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_40bca0() -> ! {
    todo!("0x40bca0 boost::shared_ptr<RBX::LeftMotorTool>::shared_ptr<RBX::LeftMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40bd68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13LeftMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LeftMotorTool,RBX::LeftMotorTool>(rbx_core::SharedPtr<RBX::LeftMotorTool> const*,RBX::LeftMotorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13LeftMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_40bd68() -> ! {
    todo!("0x40bd68 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LeftMotorTool,RBX::LeftMotorTool>(boost::shared_ptr<RBX::LeftMotorTool> const*,RBX::LeftMotorTool *)const")
}

// 0x40be4c — __ZN5boost6detail12shared_countC2IPN3RBX13LeftMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13LeftMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_40be4c() -> ! {
    todo!("0x40be4c boost::detail::shared_count::shared_count<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40bf44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_40bf44() -> ! {
    todo!("0x40bf44 boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40bf48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_40bf48() -> ! {
    todo!("0x40bf48 boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40bf4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_40bf4c() -> ! {
    todo!("0x40bf4c boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x40bf5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_40bf5c() -> ! {
    todo!("0x40bf5c boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x40bf74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_40bf74() -> ! {
    todo!("0x40bf74 boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x40bf78 — __ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v
pub fn stub_40bf78() -> ! {
    todo!("0x40bf78 __ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v")
}

// 0x40bfbc — __ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv
pub fn stub_40bfbc() -> ! {
    todo!("0x40bfbc __ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv")
}

// 0x40bfc0 — __ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v
pub fn stub_40bfc0() -> ! {
    todo!("0x40bfc0 __ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v")
}

// 0x40c0a4 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_40c0a4() -> ! {
    todo!("0x40c0a4 RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x40c228 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED0Ev
pub fn stub_40c228() -> ! {
    todo!("0x40c228 RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40c2c8 — __ZNK3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_40c2c8() -> ! {
    todo!("0x40c2c8 RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x40c300 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_40c300() -> ! {
    todo!("0x40c300 RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x40c414 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_40c414() -> ! {
    todo!("0x40c414 RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x40c4e0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14RightMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::RightMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::RightMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14RightMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_40c4e0() -> ! {
    todo!("0x40c4e0 boost::shared_ptr<RBX::RightMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::RightMotorTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x40c5b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv
pub fn stub_40c5b8() -> ! {
    todo!("0x40c5b8 __ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv")
}

// 0x40c5bc — __ZNK3RBX14RightMotorTool8isStickyEv
// type: void __fastcall(RBX::RightMotorTool *this, int)
#[doc(alias = "RBX::RightMotorTool::isSticky(void)const")]
// was: __ZNK3RBX14RightMotorTool8isStickyEv
pub fn stub_40c5bc() -> ! {
    todo!("0x40c5bc RBX::RightMotorTool::isSticky(void)const")
}

// 0x40c684 — __ZNK3RBX14RightMotorTool13getCursorNameEv
// type: int __fastcall(RBX::RightMotorTool *this)
#[doc(alias = "RBX::RightMotorTool::getCursorName(void)const")]
// was: __ZNK3RBX14RightMotorTool13getCursorNameEv
pub fn stub_40c684() -> ! {
    todo!("0x40c684 RBX::RightMotorTool::getCursorName(void)const")
}

// 0x40c6a0 — __ZN5boost10shared_ptrIN3RBX14RightMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::RightMotorTool>::shared_ptr<RBX::RightMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX14RightMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_40c6a0() -> ! {
    todo!("0x40c6a0 boost::shared_ptr<RBX::RightMotorTool>::shared_ptr<RBX::RightMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40c768 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14RightMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::RightMotorTool,RBX::RightMotorTool>(rbx_core::SharedPtr<RBX::RightMotorTool> const*,RBX::RightMotorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14RightMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_40c768() -> ! {
    todo!("0x40c768 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::RightMotorTool,RBX::RightMotorTool>(boost::shared_ptr<RBX::RightMotorTool> const*,RBX::RightMotorTool *)const")
}

// 0x40c84c — __ZN5boost6detail12shared_countC2IPN3RBX14RightMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX14RightMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_40c84c() -> ! {
    todo!("0x40c84c boost::detail::shared_count::shared_count<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40c944 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_40c944() -> ! {
    todo!("0x40c944 boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40c948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_40c948() -> ! {
    todo!("0x40c948 boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40c94c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_40c94c() -> ! {
    todo!("0x40c94c boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x40c95c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_40c95c() -> ! {
    todo!("0x40c95c boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x40c974 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_40c974() -> ! {
    todo!("0x40c974 boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x40c978 — __ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v
pub fn stub_40c978() -> ! {
    todo!("0x40c978 __ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v")
}

// 0x40c9bc — __ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv
pub fn stub_40c9bc() -> ! {
    todo!("0x40c9bc __ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv")
}

// 0x40c9c0 — __ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v
pub fn stub_40c9c0() -> ! {
    todo!("0x40c9c0 __ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v")
}

// 0x40caa4 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_40caa4() -> ! {
    todo!("0x40caa4 RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}
