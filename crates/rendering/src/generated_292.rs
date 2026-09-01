//! rendering shard 292 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 31740->31840 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31740 before -> 31840 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x406610

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x406620 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_406620() -> ! {
    todo!("0x406620 boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x406638 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_406638() -> ! {
    todo!("0x406638 boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x40663c — __ZN3RBX4Name7declareILZNS_9sGameToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sGameToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sGameToolEEEERKS0_v
pub fn stub_40663c() -> ! {
    todo!("0x40663c __ZN3RBX4Name7declareILZNS_9sGameToolEEEERKS0_v")
}

// 0x406680 — __ZN3RBX4Name13callDoDeclareILZNS_9sGameToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sGameToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sGameToolEEEEvv
pub fn stub_406680() -> ! {
    todo!("0x406680 __ZN3RBX4Name13callDoDeclareILZNS_9sGameToolEEEEvv")
}

// 0x406684 — __ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v
pub fn stub_406684() -> ! {
    todo!("0x406684 __ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v")
}

// 0x406768 — __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_406768() -> ! {
    todo!("0x406768 RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x4068ec — __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEED0Ev
pub fn stub_4068ec() -> ! {
    todo!("0x4068ec RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40698c — __ZNK3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_40698c() -> ! {
    todo!("0x40698c RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x4069c4 — __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_4069c4() -> ! {
    todo!("0x4069c4 RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x406ad8 — __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_406ad8() -> ! {
    todo!("0x406ad8 RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x406ba4 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8NullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "boost::shared_ptr<RBX::NullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NullTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8NullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_406ba4() -> ! {
    todo!("0x406ba4 boost::shared_ptr<RBX::NullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NullTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x406c58 — __ZN5boost10shared_ptrIN3RBX8NullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::NullTool>::shared_ptr<RBX::NullTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8NullToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_406c58() -> ! {
    todo!("0x406c58 boost::shared_ptr<RBX::NullTool>::shared_ptr<RBX::NullTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x406d20 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8NullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NullTool,RBX::NullTool>(boost::shared_ptr<RBX::NullTool> const*,RBX::NullTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8NullToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_406d20() -> ! {
    todo!("0x406d20 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::NullTool,RBX::NullTool>(boost::shared_ptr<RBX::NullTool> const*,RBX::NullTool *)const")
}

// 0x406e04 — __ZN5boost6detail12shared_countC2IPN3RBX8NullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8NullToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_406e04() -> ! {
    todo!("0x406e04 boost::detail::shared_count::shared_count<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x406efc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_406efc() -> ! {
    todo!("0x406efc boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x406f00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_406f00() -> ! {
    todo!("0x406f00 boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x406f04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_406f04() -> ! {
    todo!("0x406f04 boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x406f14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_406f14() -> ! {
    todo!("0x406f14 boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x406f2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8NullToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_406f2c() -> ! {
    todo!("0x406f2c boost::detail::sp_counted_impl_pd<RBX::NullTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x406f30 — __ZN3RBX4Name7declareILZNS_9sNullToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sNullToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sNullToolEEEERKS0_v
pub fn stub_406f30() -> ! {
    todo!("0x406f30 __ZN3RBX4Name7declareILZNS_9sNullToolEEEERKS0_v")
}

// 0x406f74 — __ZN3RBX4Name13callDoDeclareILZNS_9sNullToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sNullToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sNullToolEEEEvv
pub fn stub_406f74() -> ! {
    todo!("0x406f74 __ZN3RBX4Name13callDoDeclareILZNS_9sNullToolEEEEvv")
}

// 0x406f78 — __ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v
pub fn stub_406f78() -> ! {
    todo!("0x406f78 __ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v")
}

// 0x40705c — __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_40705c() -> ! {
    todo!("0x40705c RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x4071e0 — __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEED0Ev
pub fn stub_4071e0() -> ! {
    todo!("0x4071e0 RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x407280 — __ZNK3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_407280() -> ! {
    todo!("0x407280 RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x4072b8 — __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_4072b8() -> ! {
    todo!("0x4072b8 RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x4073cc — __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_4073cc() -> ! {
    todo!("0x4073cc RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x407498 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11DropperToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "boost::shared_ptr<RBX::DropperTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DropperTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11DropperToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_407498() -> ! {
    todo!("0x407498 boost::shared_ptr<RBX::DropperTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DropperTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x407570 — __ZNK3RBX5NamedINS_8PartToolELZNS_12sDropperToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_12sDropperToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_8PartToolELZNS_12sDropperToolEEE7getNameEv
pub fn stub_407570() -> ! {
    todo!("0x407570 __ZNK3RBX5NamedINS_8PartToolELZNS_12sDropperToolEEE7getNameEv")
}

// 0x407574 — __ZNK3RBX11DropperTool13getCursorNameEv
// type: int __fastcall(RBX::DropperTool *this)
#[doc(alias = "RBX::DropperTool::getCursorName(void)const")]
// was: __ZNK3RBX11DropperTool13getCursorNameEv
pub fn stub_407574() -> ! {
    todo!("0x407574 RBX::DropperTool::getCursorName(void)const")
}

// 0x407590 — __ZN5boost10shared_ptrIN3RBX11DropperToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::DropperTool>::shared_ptr<RBX::DropperTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX11DropperToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_407590() -> ! {
    todo!("0x407590 boost::shared_ptr<RBX::DropperTool>::shared_ptr<RBX::DropperTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x407658 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11DropperToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DropperTool,RBX::DropperTool>(boost::shared_ptr<RBX::DropperTool> const*,RBX::DropperTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11DropperToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_407658() -> ! {
    todo!("0x407658 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::DropperTool,RBX::DropperTool>(boost::shared_ptr<RBX::DropperTool> const*,RBX::DropperTool *)const")
}

// 0x40773c — __ZN5boost6detail12shared_countC2IPN3RBX11DropperToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11DropperToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_40773c() -> ! {
    todo!("0x40773c boost::detail::shared_count::shared_count<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x407834 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_407834() -> ! {
    todo!("0x407834 boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x407838 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_407838() -> ! {
    todo!("0x407838 boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40783c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_40783c() -> ! {
    todo!("0x40783c boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x40784c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_40784c() -> ! {
    todo!("0x40784c boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x407864 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11DropperToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_407864() -> ! {
    todo!("0x407864 boost::detail::sp_counted_impl_pd<RBX::DropperTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x407868 — __ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v
pub fn stub_407868() -> ! {
    todo!("0x407868 __ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v")
}

// 0x4078ac — __ZN3RBX4Name13callDoDeclareILZNS_12sDropperToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sDropperToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sDropperToolEEEEvv
pub fn stub_4078ac() -> ! {
    todo!("0x4078ac __ZN3RBX4Name13callDoDeclareILZNS_12sDropperToolEEEEvv")
}

// 0x4078b0 — __ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v
pub fn stub_4078b0() -> ! {
    todo!("0x4078b0 __ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v")
}

// 0x407994 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_407994() -> ! {
    todo!("0x407994 RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x407b18 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED0Ev
pub fn stub_407b18() -> ! {
    todo!("0x407b18 RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x407bb8 — __ZNK3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_407bb8() -> ! {
    todo!("0x407bb8 RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x407bf0 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_407bf0() -> ! {
    todo!("0x407bf0 RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x407d04 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_407d04() -> ! {
    todo!("0x407d04 RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x407dd0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12MaterialToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "boost::shared_ptr<RBX::MaterialTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MaterialTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12MaterialToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_407dd0() -> ! {
    todo!("0x407dd0 boost::shared_ptr<RBX::MaterialTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MaterialTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x407ea8 — __ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv
pub fn stub_407ea8() -> ! {
    todo!("0x407ea8 __ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv")
}

// 0x407eac — __ZNK3RBX12MaterialTool8isStickyEv
// type: void __fastcall(RBX::MaterialTool *this, int)
#[doc(alias = "RBX::MaterialTool::isSticky(void)const")]
// was: __ZNK3RBX12MaterialTool8isStickyEv
pub fn stub_407eac() -> ! {
    todo!("0x407eac RBX::MaterialTool::isSticky(void)const")
}

// 0x407f74 — __ZNK3RBX12MaterialTool13getCursorNameEv
// type: int __fastcall(RBX::MaterialTool *this)
#[doc(alias = "RBX::MaterialTool::getCursorName(void)const")]
// was: __ZNK3RBX12MaterialTool13getCursorNameEv
pub fn stub_407f74() -> ! {
    todo!("0x407f74 RBX::MaterialTool::getCursorName(void)const")
}

// 0x407f90 — __ZN5boost10shared_ptrIN3RBX12MaterialToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::MaterialTool>::shared_ptr<RBX::MaterialTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12MaterialToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_407f90() -> ! {
    todo!("0x407f90 boost::shared_ptr<RBX::MaterialTool>::shared_ptr<RBX::MaterialTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x408058 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12MaterialToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MaterialTool,RBX::MaterialTool>(boost::shared_ptr<RBX::MaterialTool> const*,RBX::MaterialTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_12MaterialToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_408058() -> ! {
    todo!("0x408058 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::MaterialTool,RBX::MaterialTool>(boost::shared_ptr<RBX::MaterialTool> const*,RBX::MaterialTool *)const")
}

// 0x40813c — __ZN5boost6detail12shared_countC2IPN3RBX12MaterialToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX12MaterialToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_40813c() -> ! {
    todo!("0x40813c boost::detail::shared_count::shared_count<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x408234 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_408234() -> ! {
    todo!("0x408234 boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x408238 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_408238() -> ! {
    todo!("0x408238 boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40823c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_40823c() -> ! {
    todo!("0x40823c boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x40824c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_40824c() -> ! {
    todo!("0x40824c boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x408264 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12MaterialToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_408264() -> ! {
    todo!("0x408264 boost::detail::sp_counted_impl_pd<RBX::MaterialTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x408268 — __ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v
pub fn stub_408268() -> ! {
    todo!("0x408268 __ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v")
}

// 0x4082ac — __ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv
pub fn stub_4082ac() -> ! {
    todo!("0x4082ac __ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv")
}

// 0x4082b0 — __ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v
pub fn stub_4082b0() -> ! {
    todo!("0x4082b0 __ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v")
}

// 0x408394 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_408394() -> ! {
    todo!("0x408394 RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x408518 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEED0Ev
pub fn stub_408518() -> ! {
    todo!("0x408518 RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x4085b8 — __ZNK3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_4085b8() -> ! {
    todo!("0x4085b8 RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x4085f0 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_4085f0() -> ! {
    todo!("0x4085f0 RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x408704 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_408704() -> ! {
    todo!("0x408704 RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x4087d0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8FillToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "boost::shared_ptr<RBX::FillTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FillTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8FillToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_4087d0() -> ! {
    todo!("0x4087d0 boost::shared_ptr<RBX::FillTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FillTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x4088a8 — __ZNK3RBX5NamedINS_8PartToolELZNS_9sFillToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_9sFillToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_8PartToolELZNS_9sFillToolEEE7getNameEv
pub fn stub_4088a8() -> ! {
    todo!("0x4088a8 __ZNK3RBX5NamedINS_8PartToolELZNS_9sFillToolEEE7getNameEv")
}

// 0x4088ac — __ZNK3RBX8FillTool8isStickyEv
// type: void __fastcall(RBX::FillTool *this, int)
#[doc(alias = "RBX::FillTool::isSticky(void)const")]
// was: __ZNK3RBX8FillTool8isStickyEv
pub fn stub_4088ac() -> ! {
    todo!("0x4088ac RBX::FillTool::isSticky(void)const")
}

// 0x408974 — __ZNK3RBX8FillTool13getCursorNameEv
// type: int __fastcall(RBX::FillTool *this)
#[doc(alias = "RBX::FillTool::getCursorName(void)const")]
// was: __ZNK3RBX8FillTool13getCursorNameEv
pub fn stub_408974() -> ! {
    todo!("0x408974 RBX::FillTool::getCursorName(void)const")
}

// 0x408990 — __ZN5boost10shared_ptrIN3RBX8FillToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::FillTool>::shared_ptr<RBX::FillTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8FillToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_408990() -> ! {
    todo!("0x408990 boost::shared_ptr<RBX::FillTool>::shared_ptr<RBX::FillTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x408a58 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8FillToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::FillTool,RBX::FillTool>(boost::shared_ptr<RBX::FillTool> const*,RBX::FillTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8FillToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_408a58() -> ! {
    todo!("0x408a58 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::FillTool,RBX::FillTool>(boost::shared_ptr<RBX::FillTool> const*,RBX::FillTool *)const")
}

// 0x408b3c — __ZN5boost6detail12shared_countC2IPN3RBX8FillToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8FillToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_408b3c() -> ! {
    todo!("0x408b3c boost::detail::shared_count::shared_count<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x408c34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_408c34() -> ! {
    todo!("0x408c34 boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x408c38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_408c38() -> ! {
    todo!("0x408c38 boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x408c3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_408c3c() -> ! {
    todo!("0x408c3c boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x408c4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_408c4c() -> ! {
    todo!("0x408c4c boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x408c64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FillToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_408c64() -> ! {
    todo!("0x408c64 boost::detail::sp_counted_impl_pd<RBX::FillTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x408c68 — __ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v
pub fn stub_408c68() -> ! {
    todo!("0x408c68 __ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v")
}

// 0x408cac — __ZN3RBX4Name13callDoDeclareILZNS_9sFillToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFillToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sFillToolEEEEvv
pub fn stub_408cac() -> ! {
    todo!("0x408cac __ZN3RBX4Name13callDoDeclareILZNS_9sFillToolEEEEvv")
}

// 0x408cb0 — __ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v
pub fn stub_408cb0() -> ! {
    todo!("0x408cb0 __ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v")
}

// 0x408d94 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_408d94() -> ! {
    todo!("0x408d94 RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x408f18 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEED0Ev
pub fn stub_408f18() -> ! {
    todo!("0x408f18 RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x408fb8 — __ZNK3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_408fb8() -> ! {
    todo!("0x408fb8 RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x408ff0 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_408ff0() -> ! {
    todo!("0x408ff0 RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x409104 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_409104() -> ! {
    todo!("0x409104 RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x4091d0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8LockToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "boost::shared_ptr<RBX::LockTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LockTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8LockToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_4091d0() -> ! {
    todo!("0x4091d0 boost::shared_ptr<RBX::LockTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LockTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x4092a8 — __ZNK3RBX5NamedINS_9ModelToolELZNS_9sLockToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_9ModelToolELZNS_9sLockToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_9ModelToolELZNS_9sLockToolEEE7getNameEv
pub fn stub_4092a8() -> ! {
    todo!("0x4092a8 __ZNK3RBX5NamedINS_9ModelToolELZNS_9sLockToolEEE7getNameEv")
}

// 0x4092ac — __ZN3RBX8LockTool9onMouseUpERKNS_7UIEventE
// type: void __fastcall(RBX::LockTool *this, const RBX::UIEvent *)
#[doc(alias = "RBX::LockTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX8LockTool9onMouseUpERKNS_7UIEventE
pub fn stub_4092ac() -> ! {
    todo!("0x4092ac RBX::LockTool::onMouseUp(RBX::UIEvent const&)")
}

// 0x409374 — __ZN3RBX11shared_fromINS_8LockToolEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::LockTool> RBX::shared_from<RBX::LockTool>(RBX::LockTool*)")]
// was: __ZN3RBX11shared_fromINS_8LockToolEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_409374() -> ! {
    todo!("0x409374 boost::shared_ptr<RBX::LockTool> RBX::shared_from<RBX::LockTool>(RBX::LockTool*)")
}

// 0x4094dc — __ZN5boost10shared_ptrIN3RBX8LockToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::LockTool>::shared_ptr<RBX::LockTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8LockToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_4094dc() -> ! {
    todo!("0x4094dc boost::shared_ptr<RBX::LockTool>::shared_ptr<RBX::LockTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x4095a4 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8LockToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LockTool,RBX::LockTool>(boost::shared_ptr<RBX::LockTool> const*,RBX::LockTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8LockToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4095a4() -> ! {
    todo!("0x4095a4 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LockTool,RBX::LockTool>(boost::shared_ptr<RBX::LockTool> const*,RBX::LockTool *)const")
}

// 0x409688 — __ZN5boost6detail12shared_countC2IPN3RBX8LockToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8LockToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_409688() -> ! {
    todo!("0x409688 boost::detail::shared_count::shared_count<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x409780 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_409780() -> ! {
    todo!("0x409780 boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x409784 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_409784() -> ! {
    todo!("0x409784 boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x409788 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_409788() -> ! {
    todo!("0x409788 boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x409798 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_409798() -> ! {
    todo!("0x409798 boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4097b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8LockToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4097b0() -> ! {
    todo!("0x4097b0 boost::detail::sp_counted_impl_pd<RBX::LockTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x4097b4 — __ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v
pub fn stub_4097b4() -> ! {
    todo!("0x4097b4 __ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v")
}

// 0x4097f8 — __ZN3RBX4Name13callDoDeclareILZNS_9sLockToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sLockToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sLockToolEEEEvv
pub fn stub_4097f8() -> ! {
    todo!("0x4097f8 __ZN3RBX4Name13callDoDeclareILZNS_9sLockToolEEEEvv")
}
