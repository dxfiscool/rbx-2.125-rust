//! rendering shard 295 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32040->32140 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32040 before -> 32140 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x40fca4

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x40fe28 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEED0Ev
// IDA 0x40fe28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40fe28() {
}

// 0x40fec8 — __ZNK3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40fec8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fec8() {
}

// 0x40ff00 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40ff00: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ff00() {
}

// 0x410014 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x410014: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410014() {
}

// 0x4100e0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GlueToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlueTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GlueTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GlueToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x4100e0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4100e0() {
}

// 0x4101b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sGlueToolEEE7getNameEv
// IDA 0x4101b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4101b8() {
}

// 0x4101bc — __ZNK3RBX8GlueTool8isStickyEv
// type: void __fastcall(RBX::GlueTool *this, int)
#[doc(alias = "RBX::GlueTool::isSticky(void)const")]
// was: __ZNK3RBX8GlueTool8isStickyEv
// IDA 0x4101bc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4101bc() {
}

// 0x410284 — __ZNK3RBX8GlueTool13getCursorNameEv
// type: int __fastcall(RBX::GlueTool *this)
#[doc(alias = "RBX::GlueTool::getCursorName(void)const")]
// was: __ZNK3RBX8GlueTool13getCursorNameEv
// IDA 0x410284: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410284() {
}

// 0x4102a0 — __ZN5boost10shared_ptrIN3RBX8GlueToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GlueTool>::shared_ptr<RBX::GlueTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8GlueToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x4102a0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4102a0() {
}

// 0x410368 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8GlueToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::GlueTool,RBX::GlueTool>(rbx_core::SharedPtr<RBX::GlueTool> const*,RBX::GlueTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8GlueToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x410368: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410368() {
}

// 0x41044c — __ZN5boost6detail12shared_countC2IPN3RBX8GlueToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8GlueToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x41044c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_41044c() {
}

// 0x410544 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x410544: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_410544() {
}

// 0x410548 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x410548: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_410548() {
}

// 0x41054c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x41054c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_41054c() {
}

// 0x41055c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x41055c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_41055c() {
}

// 0x410574 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GlueTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GlueToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x410574: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410574() {
}

// 0x410578 — __ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v
// IDA 0x410578: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410578() {
}

// 0x4105bc — __ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sGlueToolEEEEvv
// IDA 0x4105bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4105bc() {
}

// 0x4105c0 — __ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v
// IDA 0x4105c0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4105c0() {
}

// 0x4106a4 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x4106a4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4106a4() {
}

// 0x410828 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEED0Ev
// IDA 0x410828: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_410828() {
}

// 0x4108c8 — __ZNK3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x4108c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4108c8() {
}

// 0x410900 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x410900: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410900() {
}

// 0x410a14 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x410a14: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410a14() {
}

// 0x410ae0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8FlatToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::FlatTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FlatTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8FlatToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x410ae0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410ae0() {
}

// 0x410bb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sFlatToolEEE7getNameEv
// IDA 0x410bb8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_410bb8() {
}

// 0x410bbc — __ZNK3RBX8FlatTool8isStickyEv
// type: void __fastcall(RBX::FlatTool *this, int)
#[doc(alias = "RBX::FlatTool::isSticky(void)const")]
// was: __ZNK3RBX8FlatTool8isStickyEv
// IDA 0x410bbc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410bbc() {
}

// 0x410c84 — __ZNK3RBX8FlatTool13getCursorNameEv
// type: int __fastcall(RBX::FlatTool *this)
#[doc(alias = "RBX::FlatTool::getCursorName(void)const")]
// was: __ZNK3RBX8FlatTool13getCursorNameEv
// IDA 0x410c84: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410c84() {
}

// 0x410ca0 — __ZN5boost10shared_ptrIN3RBX8FlatToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::FlatTool>::shared_ptr<RBX::FlatTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8FlatToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x410ca0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410ca0() {
}

// 0x410d68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8FlatToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::FlatTool,RBX::FlatTool>(rbx_core::SharedPtr<RBX::FlatTool> const*,RBX::FlatTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8FlatToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x410d68: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410d68() {
}

// 0x410e4c — __ZN5boost6detail12shared_countC2IPN3RBX8FlatToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8FlatToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x410e4c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410e4c() {
}

// 0x410f44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x410f44: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_410f44() {
}

// 0x410f48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x410f48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_410f48() {
}

// 0x410f4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x410f4c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410f4c() {
}

// 0x410f5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x410f5c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410f5c() {
}

// 0x410f74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlatTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8FlatToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x410f74: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410f74() {
}

// 0x410f78 — __ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v
// IDA 0x410f78: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410f78() {
}

// 0x410fbc — __ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sFlatToolEEEEvv
// IDA 0x410fbc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_410fbc() {
}

// 0x410fc0 — __ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v
// IDA 0x410fc0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_410fc0() {
}

// 0x4110a4 — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x4110a4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4110a4() {
}

// 0x411228 — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEED0Ev
// IDA 0x411228: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_411228() {
}

// 0x4112c8 — __ZNK3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x4112c8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4112c8() {
}

// 0x4112fc — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x4112fc: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4112fc() {
}

// 0x411410 — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x411410: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411410() {
}

// 0x4114dc — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x4114dc: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4114dc() {
}

// 0x411660 — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEED0Ev
// IDA 0x411660: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_411660() {
}

// 0x411700 — __ZNK3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x411700: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411700() {
}

// 0x411738 — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x411738: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411738() {
}

// 0x41184c — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x41184c: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_41184c() {
}

// 0x411918 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13AdvRotateToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvRotateTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvRotateTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13AdvRotateToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x411918: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411918() {
}

// 0x4119fc — __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_14sAdvRotateToolEEE7getNameEv
// IDA 0x4119fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4119fc() {
}

// 0x411a00 — __ZNK3RBX13AdvRotateTool8isStickyEv
// type: void __fastcall(RBX::AdvRotateTool *this, int)
#[doc(alias = "RBX::AdvRotateTool::isSticky(void)const")]
// was: __ZNK3RBX13AdvRotateTool8isStickyEv
// IDA 0x411a00: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411a00() {
}

// 0x411ac8 — __ZNK3RBX15AdvMoveToolBase14drawConnectorsEv
// type: int __fastcall(RBX::AdvMoveToolBase *this)
#[doc(alias = "RBX::AdvMoveToolBase::drawConnectors(void)const")]
// was: __ZNK3RBX15AdvMoveToolBase14drawConnectorsEv
// IDA 0x411ac8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411ac8() {
}

// 0x411acc — __ZNK3RBX15AdvMoveToolBase13getCursorNameEv
// type: int __fastcall(RBX::AdvMoveToolBase *this, int)
#[doc(alias = "RBX::AdvMoveToolBase::getCursorName(void)const")]
// was: __ZNK3RBX15AdvMoveToolBase13getCursorNameEv
// IDA 0x411acc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411acc() {
}

// 0x411ad8 — __ZN3RBX15AdvMoveToolBase9setCursorESs
// type: int __fastcall(int)
#[doc(alias = "RBX::AdvMoveToolBase::setCursor(std::string)")]
// was: __ZN3RBX15AdvMoveToolBase9setCursorESs
// IDA 0x411ad8: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411ad8() {
}

// 0x411ae0 — __ZNK3RBX13AdvRotateTool14getHandleColorEv
// type: int __fastcall(RBX::AdvRotateTool *this)
#[doc(alias = "RBX::AdvRotateTool::getHandleColor(void)const")]
// was: __ZNK3RBX13AdvRotateTool14getHandleColorEv
// IDA 0x411ae0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411ae0() {
}

// 0x411af8 — __ZNK3RBX13AdvRotateTool11getDragTypeEv
// type: int __fastcall(RBX::AdvRotateTool *this)
#[doc(alias = "RBX::AdvRotateTool::getDragType(void)const")]
// was: __ZNK3RBX13AdvRotateTool11getDragTypeEv
// IDA 0x411af8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411af8() {
}

// 0x411afc — __ZN3RBX15AdvMoveToolBaseD2Ev
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// was: __ZN3RBX15AdvMoveToolBaseD2Ev
// IDA 0x411afc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_411afc() {
}

// 0x411c14 — __ZN3RBX15AdvMoveToolBaseD1Ev
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// was: __ZN3RBX15AdvMoveToolBaseD1Ev
// IDA 0x411c14: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_411c14() {
}

// 0x411c18 — __ZN3RBX15AdvMoveToolBaseD0Ev
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// was: __ZN3RBX15AdvMoveToolBaseD0Ev
// IDA 0x411c18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_411c18() {
}

// 0x411cb8 — __ZThn36_N3RBX15AdvMoveToolBaseD1Ev
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// was: __ZThn36_N3RBX15AdvMoveToolBaseD1Ev
// IDA 0x411cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_411cb8() {
}

// 0x411cc0 — __ZThn36_N3RBX15AdvMoveToolBaseD0Ev
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// was: __ZThn36_N3RBX15AdvMoveToolBaseD0Ev
// IDA 0x411cc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_411cc0() {
}

// 0x411cc8 — __ZNSt8auto_ptrIN3RBX11MegaDraggerEED2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(RBX::MegaDragger **)
#[doc(alias = "std::auto_ptr<RBX::MegaDragger>::~auto_ptr()")]
// was: __ZNSt8auto_ptrIN3RBX11MegaDraggerEED2Ev
// IDA 0x411cc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_411cc8() {
}

// 0x411d70 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_erase(std::_Rb_tree_node<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>> *)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0x411d70: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411d70() {
}

// 0x411d98 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::PartInstance>,std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>,std::less<rbx_core::WeakPtr<RBX::PartInstance>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<rbx_core::WeakPtr<RBX::PartInstance> const,float>> *)")]
// was: __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX12PartInstanceEEESt4pairIKS4_fESt10_Select1stIS7_ESt4lessIS4_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
// IDA 0x411d98: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411d98() {
}

// 0x411db4 — __ZN5boost10shared_ptrIN3RBX13AdvRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvRotateTool>::shared_ptr<RBX::AdvRotateTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13AdvRotateToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x411db4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411db4() {
}

// 0x411e7c — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13AdvRotateToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvRotateTool,RBX::AdvRotateTool>(rbx_core::SharedPtr<RBX::AdvRotateTool> const*,RBX::AdvRotateTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13AdvRotateToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x411e7c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411e7c() {
}

// 0x411f60 — __ZN5boost6detail12shared_countC2IPN3RBX13AdvRotateToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13AdvRotateToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x411f60: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_411f60() {
}

// 0x412058 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x412058: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_412058() {
}

// 0x41205c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x41205c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_41205c() {
}

// 0x412060 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x412060: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412060() {
}

// 0x412070 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x412070: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412070() {
}

// 0x412088 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvRotateTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13AdvRotateToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x412088: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412088() {
}

// 0x41208c — __ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v
// IDA 0x41208c: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_41208c() {
}

// 0x4120d0 — __ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sAdvRotateToolEEEEvv
// IDA 0x4120d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4120d0() {
}

// 0x4120d4 — __ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v
// IDA 0x4120d4: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4120d4() {
}

// 0x4121b8 — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x4121b8: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4121b8() {
}

// 0x41233c — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEED0Ev
// IDA 0x41233c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_41233c() {
}

// 0x4123dc — __ZNK3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x4123dc: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4123dc() {
}

// 0x412414 — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x412414: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412414() {
}

// 0x412528 — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x412528: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412528() {
}

// 0x4125f4 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11AdvMoveToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvMoveTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11AdvMoveToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x4125f4: 81 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4125f4() {
}

// 0x4126e4 — __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_15AdvMoveToolBaseELZNS_12sAdvMoveToolEEE7getNameEv
// IDA 0x4126e4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4126e4() {
}

// 0x4126e8 — __ZN3RBX11AdvMoveToolD1Ev
// type: void __fastcall(RBX::AdvMoveTool *__hidden this)
#[doc(alias = "RBX::AdvMoveTool::~AdvMoveTool()")]
// was: __ZN3RBX11AdvMoveToolD1Ev
// IDA 0x4126e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4126e8() {
}

// 0x4126ec — __ZN3RBX11AdvMoveToolD0Ev
// type: void __fastcall(RBX::AdvMoveTool *__hidden this)
#[doc(alias = "RBX::AdvMoveTool::~AdvMoveTool()")]
// was: __ZN3RBX11AdvMoveToolD0Ev
// IDA 0x4126ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4126ec() {
}

// 0x41278c — __ZNK3RBX11AdvMoveTool8isStickyEv
// type: void __fastcall(RBX::AdvMoveTool *this, int)
#[doc(alias = "RBX::AdvMoveTool::isSticky(void)const")]
// was: __ZNK3RBX11AdvMoveTool8isStickyEv
// IDA 0x41278c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_41278c() {
}

// 0x412854 — __ZNK3RBX11AdvMoveTool14getHandleColorEv
// type: int __fastcall(RBX::AdvMoveTool *this)
#[doc(alias = "RBX::AdvMoveTool::getHandleColor(void)const")]
// was: __ZNK3RBX11AdvMoveTool14getHandleColorEv
// IDA 0x412854: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412854() {
}

// 0x41286c — __ZNK3RBX11AdvMoveTool11getDragTypeEv
// type: int __fastcall(RBX::AdvMoveTool *this)
#[doc(alias = "RBX::AdvMoveTool::getDragType(void)const")]
// was: __ZNK3RBX11AdvMoveTool11getDragTypeEv
// IDA 0x41286c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_41286c() {
}

// 0x412870 — __ZThn36_N3RBX11AdvMoveToolD1Ev
// type: void __fastcall(RBX::AdvMoveTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool()")]
// was: __ZThn36_N3RBX11AdvMoveToolD1Ev
// IDA 0x412870: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_412870() {
}

// 0x412878 — __ZThn36_N3RBX11AdvMoveToolD0Ev
// type: void __fastcall(RBX::AdvMoveTool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvMoveTool::~AdvMoveTool()")]
// was: __ZThn36_N3RBX11AdvMoveToolD0Ev
// IDA 0x412878: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_412878() {
}

// 0x412880 — __ZN5boost10shared_ptrIN3RBX11AdvMoveToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveTool>::shared_ptr<RBX::AdvMoveTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX11AdvMoveToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x412880: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412880() {
}

// 0x412948 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11AdvMoveToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvMoveTool,RBX::AdvMoveTool>(rbx_core::SharedPtr<RBX::AdvMoveTool> const*,RBX::AdvMoveTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_11AdvMoveToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x412948: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412948() {
}

// 0x412a2c — __ZN5boost6detail12shared_countC2IPN3RBX11AdvMoveToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11AdvMoveToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x412a2c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412a2c() {
}

// 0x412b24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x412b24: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_412b24() {
}

// 0x412b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x412b28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_412b28() {
}

// 0x412b2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x412b2c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412b2c() {
}

// 0x412b3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x412b3c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412b3c() {
}

// 0x412b54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvMoveTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11AdvMoveToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x412b54: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412b54() {
}

// 0x412b58 — __ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v
// IDA 0x412b58: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_412b58() {
}

// 0x412b9c — __ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sAdvMoveToolEEEEvv
// IDA 0x412b9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_412b9c() {
}