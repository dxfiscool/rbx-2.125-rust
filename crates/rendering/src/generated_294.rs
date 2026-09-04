//! rendering shard 294 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 31940->32040 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31940 before -> 32040 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x40caa4

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x40cc28 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEED0Ev
// IDA 0x40cc28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40cc28() {
}

// 0x40ccc8 — __ZNK3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40ccc8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ccc8() {
}

// 0x40cd00 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40cd00: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40cd00() {
}

// 0x40ce14 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40ce14: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ce14() {
}

// 0x40cee0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9HingeToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::HingeTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HingeTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9HingeToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40cee0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40cee0() {
}

// 0x40cfb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sHingeToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sHingeToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sHingeToolEEE7getNameEv
// IDA 0x40cfb8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40cfb8() {
}

// 0x40cfbc — __ZNK3RBX9HingeTool8isStickyEv
// type: void __fastcall(RBX::HingeTool *this, int)
#[doc(alias = "RBX::HingeTool::isSticky(void)const")]
// was: __ZNK3RBX9HingeTool8isStickyEv
// IDA 0x40cfbc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40cfbc() {
}

// 0x40d084 — __ZNK3RBX9HingeTool13getCursorNameEv
// type: int __fastcall(RBX::HingeTool *this)
#[doc(alias = "RBX::HingeTool::getCursorName(void)const")]
// was: __ZNK3RBX9HingeTool13getCursorNameEv
// IDA 0x40d084: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d084() {
}

// 0x40d0a0 — __ZN5boost10shared_ptrIN3RBX9HingeToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::HingeTool>::shared_ptr<RBX::HingeTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9HingeToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40d0a0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d0a0() {
}

// 0x40d168 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9HingeToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::HingeTool,RBX::HingeTool>(rbx_core::SharedPtr<RBX::HingeTool> const*,RBX::HingeTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9HingeToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40d168: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d168() {
}

// 0x40d24c — __ZN5boost6detail12shared_countC2IPN3RBX9HingeToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9HingeToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40d24c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d24c() {
}

// 0x40d344 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40d344: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40d344() {
}

// 0x40d348 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40d348: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40d348() {
}

// 0x40d34c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40d34c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d34c() {
}

// 0x40d35c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40d35c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d35c() {
}

// 0x40d374 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HingeTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HingeToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40d374: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d374() {
}

// 0x40d378 — __ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v
// IDA 0x40d378: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d378() {
}

// 0x40d3bc — __ZN3RBX4Name13callDoDeclareILZNS_10sHingeToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sHingeToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sHingeToolEEEEvv
// IDA 0x40d3bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40d3bc() {
}

// 0x40d3c0 — __ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v
// IDA 0x40d3c0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d3c0() {
}

// 0x40d4a4 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40d4a4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d4a4() {
}

// 0x40d628 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEED0Ev
// IDA 0x40d628: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40d628() {
}

// 0x40d6c8 — __ZNK3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40d6c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d6c8() {
}

// 0x40d700 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40d700: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d700() {
}

// 0x40d814 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40d814: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d814() {
}

// 0x40d8e0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13UniversalToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::UniversalTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::UniversalTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13UniversalToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40d8e0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d8e0() {
}

// 0x40d9b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sUniversalToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sUniversalToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sUniversalToolEEE7getNameEv
// IDA 0x40d9b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40d9b8() {
}

// 0x40d9bc — __ZNK3RBX13UniversalTool8isStickyEv
// type: void __fastcall(RBX::UniversalTool *this, int)
#[doc(alias = "RBX::UniversalTool::isSticky(void)const")]
// was: __ZNK3RBX13UniversalTool8isStickyEv
// IDA 0x40d9bc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40d9bc() {
}

// 0x40da84 — __ZNK3RBX13UniversalTool13getCursorNameEv
// type: int __fastcall(RBX::UniversalTool *this)
#[doc(alias = "RBX::UniversalTool::getCursorName(void)const")]
// was: __ZNK3RBX13UniversalTool13getCursorNameEv
// IDA 0x40da84: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40da84() {
}

// 0x40daa0 — __ZN5boost10shared_ptrIN3RBX13UniversalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::UniversalTool>::shared_ptr<RBX::UniversalTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13UniversalToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40daa0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40daa0() {
}

// 0x40db68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13UniversalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::UniversalTool,RBX::UniversalTool>(rbx_core::SharedPtr<RBX::UniversalTool> const*,RBX::UniversalTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_13UniversalToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40db68: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40db68() {
}

// 0x40dc4c — __ZN5boost6detail12shared_countC2IPN3RBX13UniversalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13UniversalToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40dc4c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40dc4c() {
}

// 0x40dd44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40dd44: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40dd44() {
}

// 0x40dd48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40dd48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40dd48() {
}

// 0x40dd4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40dd4c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40dd4c() {
}

// 0x40dd5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40dd5c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40dd5c() {
}

// 0x40dd74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::UniversalTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13UniversalToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40dd74: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40dd74() {
}

// 0x40dd78 — __ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v
// IDA 0x40dd78: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40dd78() {
}

// 0x40ddbc — __ZN3RBX4Name13callDoDeclareILZNS_14sUniversalToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sUniversalToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sUniversalToolEEEEvv
// IDA 0x40ddbc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40ddbc() {
}

// 0x40ddc0 — __ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v
// IDA 0x40ddc0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ddc0() {
}

// 0x40dea4 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40dea4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40dea4() {
}

// 0x40e028 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEED0Ev
// IDA 0x40e028: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40e028() {
}

// 0x40e0c8 — __ZNK3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40e0c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e0c8() {
}

// 0x40e100 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40e100: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e100() {
}

// 0x40e214 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40e214: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e214() {
}

// 0x40e2e0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9InletToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::InletTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::InletTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9InletToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40e2e0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e2e0() {
}

// 0x40e3b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sInletToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sInletToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sInletToolEEE7getNameEv
// IDA 0x40e3b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40e3b8() {
}

// 0x40e3bc — __ZNK3RBX9InletTool8isStickyEv
// type: void __fastcall(RBX::InletTool *this, int)
#[doc(alias = "RBX::InletTool::isSticky(void)const")]
// was: __ZNK3RBX9InletTool8isStickyEv
// IDA 0x40e3bc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e3bc() {
}

// 0x40e484 — __ZNK3RBX9InletTool13getCursorNameEv
// type: int __fastcall(RBX::InletTool *this)
#[doc(alias = "RBX::InletTool::getCursorName(void)const")]
// was: __ZNK3RBX9InletTool13getCursorNameEv
// IDA 0x40e484: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e484() {
}

// 0x40e4a0 — __ZN5boost10shared_ptrIN3RBX9InletToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::InletTool>::shared_ptr<RBX::InletTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9InletToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40e4a0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e4a0() {
}

// 0x40e568 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9InletToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::InletTool,RBX::InletTool>(rbx_core::SharedPtr<RBX::InletTool> const*,RBX::InletTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9InletToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40e568: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e568() {
}

// 0x40e64c — __ZN5boost6detail12shared_countC2IPN3RBX9InletToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9InletToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40e64c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e64c() {
}

// 0x40e744 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40e744: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40e744() {
}

// 0x40e748 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40e748: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40e748() {
}

// 0x40e74c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40e74c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e74c() {
}

// 0x40e75c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40e75c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e75c() {
}

// 0x40e774 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::InletTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9InletToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40e774: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e774() {
}

// 0x40e778 — __ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v
// IDA 0x40e778: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e778() {
}

// 0x40e7bc — __ZN3RBX4Name13callDoDeclareILZNS_10sInletToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sInletToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sInletToolEEEEvv
// IDA 0x40e7bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40e7bc() {
}

// 0x40e7c0 — __ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v
// IDA 0x40e7c0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e7c0() {
}

// 0x40e8a4 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40e8a4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40e8a4() {
}

// 0x40ea28 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEED0Ev
// IDA 0x40ea28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40ea28() {
}

// 0x40eac8 — __ZNK3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40eac8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40eac8() {
}

// 0x40eb00 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40eb00: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40eb00() {
}

// 0x40ec14 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40ec14: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ec14() {
}

// 0x40ece0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9StudsToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::StudsTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::StudsTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9StudsToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40ece0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ece0() {
}

// 0x40edb8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sStudsToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sStudsToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sStudsToolEEE7getNameEv
// IDA 0x40edb8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40edb8() {
}

// 0x40edbc — __ZNK3RBX9StudsTool8isStickyEv
// type: void __fastcall(RBX::StudsTool *this, int)
#[doc(alias = "RBX::StudsTool::isSticky(void)const")]
// was: __ZNK3RBX9StudsTool8isStickyEv
// IDA 0x40edbc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40edbc() {
}

// 0x40ee84 — __ZNK3RBX9StudsTool13getCursorNameEv
// type: int __fastcall(RBX::StudsTool *this)
#[doc(alias = "RBX::StudsTool::getCursorName(void)const")]
// was: __ZNK3RBX9StudsTool13getCursorNameEv
// IDA 0x40ee84: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ee84() {
}

// 0x40eea0 — __ZN5boost10shared_ptrIN3RBX9StudsToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::StudsTool>::shared_ptr<RBX::StudsTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9StudsToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40eea0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40eea0() {
}

// 0x40ef68 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9StudsToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::StudsTool,RBX::StudsTool>(rbx_core::SharedPtr<RBX::StudsTool> const*,RBX::StudsTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9StudsToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40ef68: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40ef68() {
}

// 0x40f04c — __ZN5boost6detail12shared_countC2IPN3RBX9StudsToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9StudsToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40f04c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f04c() {
}

// 0x40f144 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40f144: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40f144() {
}

// 0x40f148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40f148: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40f148() {
}

// 0x40f14c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40f14c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f14c() {
}

// 0x40f15c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40f15c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f15c() {
}

// 0x40f174 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StudsTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9StudsToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40f174: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f174() {
}

// 0x40f178 — __ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v
// IDA 0x40f178: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f178() {
}

// 0x40f1bc — __ZN3RBX4Name13callDoDeclareILZNS_10sStudsToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sStudsToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sStudsToolEEEEvv
// IDA 0x40f1bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40f1bc() {
}

// 0x40f1c0 — __ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v
// IDA 0x40f1c0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f1c0() {
}

// 0x40f2a4 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40f2a4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f2a4() {
}

// 0x40f428 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEED0Ev
// IDA 0x40f428: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_40f428() {
}

// 0x40f4c8 — __ZNK3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE9isCheckedEv
// IDA 0x40f4c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f4c8() {
}

// 0x40f500 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// IDA 0x40f500: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f500() {
}

// 0x40f614 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE15newMouseCommandEv
// IDA 0x40f614: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f614() {
}

// 0x40f6e0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8WeldToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::WeldTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::WeldTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8WeldToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x40f6e0: 74 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f6e0() {
}

// 0x40f7b8 — __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sWeldToolEEE7getNameEv
// type: int()
#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sWeldToolEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_11SurfaceToolELZNS_9sWeldToolEEE7getNameEv
// IDA 0x40f7b8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40f7b8() {
}

// 0x40f7bc — __ZNK3RBX8WeldTool8isStickyEv
// type: void __fastcall(RBX::WeldTool *this, int)
#[doc(alias = "RBX::WeldTool::isSticky(void)const")]
// was: __ZNK3RBX8WeldTool8isStickyEv
// IDA 0x40f7bc: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f7bc() {
}

// 0x40f884 — __ZNK3RBX8WeldTool13getCursorNameEv
// type: int __fastcall(RBX::WeldTool *this)
#[doc(alias = "RBX::WeldTool::getCursorName(void)const")]
// was: __ZNK3RBX8WeldTool13getCursorNameEv
// IDA 0x40f884: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f884() {
}

// 0x40f8a0 — __ZN5boost10shared_ptrIN3RBX8WeldToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::WeldTool>::shared_ptr<RBX::WeldTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8WeldToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x40f8a0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f8a0() {
}

// 0x40f968 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8WeldToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::WeldTool,RBX::WeldTool>(rbx_core::SharedPtr<RBX::WeldTool> const*,RBX::WeldTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8WeldToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x40f968: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40f968() {
}

// 0x40fa4c — __ZN5boost6detail12shared_countC2IPN3RBX8WeldToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8WeldToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x40fa4c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fa4c() {
}

// 0x40fb44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x40fb44: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_40fb44() {
}

// 0x40fb48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x40fb48: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40fb48() {
}

// 0x40fb4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x40fb4c: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fb4c() {
}

// 0x40fb5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x40fb5c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fb5c() {
}

// 0x40fb74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::WeldTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8WeldToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x40fb74: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fb74() {
}

// 0x40fb78 — __ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v
// IDA 0x40fb78: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fb78() {
}

// 0x40fbbc — __ZN3RBX4Name13callDoDeclareILZNS_9sWeldToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sWeldToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sWeldToolEEEEvv
// IDA 0x40fbbc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_40fbbc() {
}

// 0x40fbc0 — __ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v
// IDA 0x40fbc0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fbc0() {
}

// 0x40fca4 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// IDA 0x40fca4: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_40fca4() {
}

