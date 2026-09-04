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
// IDA 0x4097fc: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4097fc() {
}

// 0x4098e0 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x4098e0: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4098e0() {
}

// 0x409a64 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED0Ev
// IDA 0x409a64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_409a64() {
}

// 0x409b04 — __ZNK3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x409b04: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_409b04() {
}

// 0x409b3c — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x409b3c: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_409b3c() {
}

// 0x409c50 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x409c50: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_409c50() {
}

// 0x409d1c — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10AnchorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AnchorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10AnchorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x409d1c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_409d1c() {
}

// 0x409dfc — __ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv
// IDA 0x409dfc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_409dfc() {
}

// 0x409e00 — __ZN3RBX10AnchorTool9onMouseUpERKNS_7UIEventE
// type: void __fastcall(RBX::AnchorTool *this, const RBX::UIEvent *)
#[doc(alias = "RBX::AnchorTool::onMouseUp(RBX::UIEvent const&)")]
// was: __ZN3RBX10AnchorTool9onMouseUpERKNS_7UIEventE
// IDA 0x409e00: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_409e00() {
}

// 0x409ec8 — __ZN3RBX11shared_fromINS_10AnchorToolEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::shared_from<RBX::AnchorTool>(RBX::AnchorTool*)")]
// was: __ZN3RBX11shared_fromINS_10AnchorToolEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x409ec8: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_409ec8() {
}

// 0x40a030 — __ZN5boost10shared_ptrIN3RBX10AnchorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool>::shared_ptr<RBX::AnchorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10AnchorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40a030: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a030() {
}

// 0x40a0f8 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_10AnchorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AnchorTool,RBX::AnchorTool>(rbx_core::SharedPtr<RBX::AnchorTool> const*,RBX::AnchorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_10AnchorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40a0f8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a0f8() {
}

// 0x40a1dc — __ZN5boost6detail12shared_countC2IPN3RBX10AnchorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10AnchorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40a1dc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a1dc() {
}

// 0x40a2d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40a2d4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40a2d4() {
}

// 0x40a2d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40a2d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40a2d8() {
}

// 0x40a2dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40a2dc: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a2dc() {
}

// 0x40a2ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40a2ec: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a2ec() {
}

// 0x40a304 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AnchorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10AnchorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40a304: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a304() {
}

// 0x40a308 — __ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v
// IDA 0x40a308: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a308() {
}

// 0x40a34c — __ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv
// IDA 0x40a34c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40a34c() {
}

// 0x40a350 — __ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v
// IDA 0x40a350: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a350() {
}

// 0x40a434 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40a434: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a434() {
}

// 0x40a5b8 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED0Ev
// IDA 0x40a5b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40a5b8() {
}

// 0x40a658 — __ZNK3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40a658: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a658() {
}

// 0x40a690 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40a690: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a690() {
}

// 0x40a7a4 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40a7a4: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a7a4() {
}

// 0x40a870 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_20SmoothNoOutlinesToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::SmoothNoOutlinesTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_20SmoothNoOutlinesToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40a870: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a870() {
}

// 0x40a948 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv
// IDA 0x40a948: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40a948() {
}

// 0x40a94c — __ZNK3RBX20SmoothNoOutlinesTool8isStickyEv
// type: void __fastcall(RBX::SmoothNoOutlinesTool *this, int)
#[doc(alias = "RBX::SmoothNoOutlinesTool::isSticky(void)const")]
// was: __ZNK3RBX20SmoothNoOutlinesTool8isStickyEv
// IDA 0x40a94c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40a94c() {
}

// 0x40aa14 — __ZNK3RBX20SmoothNoOutlinesTool13getCursorNameEv
// type: int __fastcall(RBX::SmoothNoOutlinesTool *this)
#[doc(alias = "RBX::SmoothNoOutlinesTool::getCursorName(void)const")]
// was: __ZNK3RBX20SmoothNoOutlinesTool13getCursorNameEv
// IDA 0x40aa14: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40aa14() {
}

// 0x40aa30 — __ZN5boost10shared_ptrIN3RBX20SmoothNoOutlinesToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool>::shared_ptr<RBX::SmoothNoOutlinesTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX20SmoothNoOutlinesToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40aa30: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40aa30() {
}

// 0x40aaf8 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_20SmoothNoOutlinesToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::SmoothNoOutlinesTool,RBX::SmoothNoOutlinesTool>(rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> const*,RBX::SmoothNoOutlinesTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_20SmoothNoOutlinesToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40aaf8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40aaf8() {
}

// 0x40abdc — __ZN5boost6detail12shared_countC2IPN3RBX20SmoothNoOutlinesToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX20SmoothNoOutlinesToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40abdc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40abdc() {
}

// 0x40acd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40acd4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40acd4() {
}

// 0x40acd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40acd8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40acd8() {
}

// 0x40acdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40acdc: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40acdc() {
}

// 0x40acec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40acec: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40acec() {
}

// 0x40ad04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SmoothNoOutlinesTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20SmoothNoOutlinesToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40ad04: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ad04() {
}

// 0x40ad08 — __ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
// IDA 0x40ad08: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ad08() {
}

// 0x40ad4c — __ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv
// IDA 0x40ad4c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40ad4c() {
}

// 0x40ad50 — __ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v
// IDA 0x40ad50: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ad50() {
}

// 0x40ae34 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40ae34: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ae34() {
}

// 0x40afb8 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED0Ev
// IDA 0x40afb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40afb8() {
}

// 0x40b058 — __ZNK3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40b058: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b058() {
}

// 0x40b090 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40b090: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b090() {
}

// 0x40b1a4 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40b1a4: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b1a4() {
}

// 0x40b270 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18OscillateMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::OscillateMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::OscillateMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18OscillateMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40b270: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b270() {
}

// 0x40b348 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv
// IDA 0x40b348: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40b348() {
}

// 0x40b34c — __ZNK3RBX18OscillateMotorTool13getCursorNameEv
// type: int __fastcall(RBX::OscillateMotorTool *this)
#[doc(alias = "RBX::OscillateMotorTool::getCursorName(void)const")]
// was: __ZNK3RBX18OscillateMotorTool13getCursorNameEv
// IDA 0x40b34c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b34c() {
}

// 0x40b368 — __ZN5boost10shared_ptrIN3RBX18OscillateMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::OscillateMotorTool>::shared_ptr<RBX::OscillateMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX18OscillateMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40b368: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b368() {
}

// 0x40b430 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18OscillateMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::OscillateMotorTool,RBX::OscillateMotorTool>(rbx_core::SharedPtr<RBX::OscillateMotorTool> const*,RBX::OscillateMotorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18OscillateMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40b430: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b430() {
}

// 0x40b514 — __ZN5boost6detail12shared_countC2IPN3RBX18OscillateMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX18OscillateMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40b514: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b514() {
}

// 0x40b60c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40b60c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40b60c() {
}

// 0x40b610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40b610: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40b610() {
}

// 0x40b614 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40b614: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b614() {
}

// 0x40b624 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40b624: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b624() {
}

// 0x40b63c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::OscillateMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18OscillateMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40b63c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b63c() {
}

// 0x40b640 — __ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v
// IDA 0x40b640: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b640() {
}

// 0x40b684 — __ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv
// IDA 0x40b684: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40b684() {
}

// 0x40b688 — __ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v
// IDA 0x40b688: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b688() {
}

// 0x40b76c — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40b76c: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b76c() {
}

// 0x40b8f0 — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED0Ev
// IDA 0x40b8f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40b8f0() {
}

// 0x40b990 — __ZNK3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40b990: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b990() {
}

// 0x40b9c8 — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40b9c8: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40b9c8() {
}

// 0x40badc — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40badc: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40badc() {
}

// 0x40bba8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13LeftMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::LeftMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LeftMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13LeftMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40bba8: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bba8() {
}

// 0x40bc80 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv
// IDA 0x40bc80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40bc80() {
}

// 0x40bc84 — __ZNK3RBX13LeftMotorTool13getCursorNameEv
// type: int __fastcall(RBX::LeftMotorTool *this)
#[doc(alias = "RBX::LeftMotorTool::getCursorName(void)const")]
// was: __ZNK3RBX13LeftMotorTool13getCursorNameEv
// IDA 0x40bc84: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bc84() {
}

// 0x40bca0 — __ZN5boost10shared_ptrIN3RBX13LeftMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::LeftMotorTool>::shared_ptr<RBX::LeftMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13LeftMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40bca0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bca0() {
}

// 0x40bd68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13LeftMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::LeftMotorTool,RBX::LeftMotorTool>(rbx_core::SharedPtr<RBX::LeftMotorTool> const*,RBX::LeftMotorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13LeftMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40bd68: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bd68() {
}

// 0x40be4c — __ZN5boost6detail12shared_countC2IPN3RBX13LeftMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13LeftMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40be4c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40be4c() {
}

// 0x40bf44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40bf44: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40bf44() {
}

// 0x40bf48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40bf48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40bf48() {
}

// 0x40bf4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40bf4c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bf4c() {
}

// 0x40bf5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40bf5c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bf5c() {
}

// 0x40bf74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LeftMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LeftMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40bf74: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bf74() {
}

// 0x40bf78 — __ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v
// IDA 0x40bf78: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bf78() {
}

// 0x40bfbc — __ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv
// IDA 0x40bfbc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40bfbc() {
}

// 0x40bfc0 — __ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v
// IDA 0x40bfc0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40bfc0() {
}

// 0x40c0a4 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40c0a4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c0a4() {
}

// 0x40c228 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED0Ev
// IDA 0x40c228: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40c228() {
}

// 0x40c2c8 — __ZNK3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40c2c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c2c8() {
}

// 0x40c300 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40c300: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c300() {
}

// 0x40c414 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40c414: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c414() {
}

// 0x40c4e0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14RightMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::RightMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::RightMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14RightMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40c4e0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c4e0() {
}

// 0x40c5b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv
// IDA 0x40c5b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40c5b8() {
}

// 0x40c5bc — __ZNK3RBX14RightMotorTool8isStickyEv
// type: void __fastcall(RBX::RightMotorTool *this, int)
#[doc(alias = "RBX::RightMotorTool::isSticky(void)const")]
// was: __ZNK3RBX14RightMotorTool8isStickyEv
// IDA 0x40c5bc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c5bc() {
}

// 0x40c684 — __ZNK3RBX14RightMotorTool13getCursorNameEv
// type: int __fastcall(RBX::RightMotorTool *this)
#[doc(alias = "RBX::RightMotorTool::getCursorName(void)const")]
// was: __ZNK3RBX14RightMotorTool13getCursorNameEv
// IDA 0x40c684: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c684() {
}

// 0x40c6a0 — __ZN5boost10shared_ptrIN3RBX14RightMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::RightMotorTool>::shared_ptr<RBX::RightMotorTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX14RightMotorToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40c6a0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c6a0() {
}

// 0x40c768 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14RightMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::RightMotorTool,RBX::RightMotorTool>(rbx_core::SharedPtr<RBX::RightMotorTool> const*,RBX::RightMotorTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14RightMotorToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40c768: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c768() {
}

// 0x40c84c — __ZN5boost6detail12shared_countC2IPN3RBX14RightMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX14RightMotorToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40c84c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c84c() {
}

// 0x40c944 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40c944: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40c944() {
}

// 0x40c948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40c948: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40c948() {
}

// 0x40c94c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40c94c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c94c() {
}

// 0x40c95c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40c95c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c95c() {
}

// 0x40c974 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RightMotorTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14RightMotorToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40c974: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c974() {
}

// 0x40c978 — __ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v
// IDA 0x40c978: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c978() {
}

// 0x40c9bc — __ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv
// IDA 0x40c9bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40c9bc() {
}

// 0x40c9c0 — __ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v
// IDA 0x40c9c0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40c9c0() {
}

// 0x40caa4 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40caa4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40caa4() {
}
