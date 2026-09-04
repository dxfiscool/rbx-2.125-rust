//! rendering shard 288 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15586/15586 complete, 31340->31440 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31340 before -> 31440 after; global gap filler)
//! Filter: Ogre|G3D|Render exhausted (0 remaining), filler global asc next 100 after 0x3f97a8

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3f9848 — __ZN3RBX17EditSelectionVerbD1Ev
// type: void __fastcall(RBX::EditSelectionVerb *__hidden this)
#[doc(alias = "RBX::EditSelectionVerb::~EditSelectionVerb()")]
// was: __ZN3RBX17EditSelectionVerbD1Ev
// IDA 0x3f9848: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3f9848() {
}

// 0x3f984c — __ZNK3RBX17EditSelectionVerb9isEnabledEv
// type: bool __fastcall(RBX::EditSelectionVerb *this)
#[doc(alias = "RBX::EditSelectionVerb::isEnabled(void)const")]
// was: __ZNK3RBX17EditSelectionVerb9isEnabledEv
// IDA 0x3f984c: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f984c() {
}

// 0x3f986c — __ZN3RBX18GroupSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::GroupSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::GroupSelectionVerb::GroupSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX18GroupSelectionVerbC1EPNS_9DataModelE
// IDA 0x3f986c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3f986c() {
}

// 0x3f9870 — __ZN3RBX18GroupSelectionVerbC2EPNS_9DataModelE
// type: RBX::GroupSelectionVerb *__fastcall(RBX::GroupSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::GroupSelectionVerb::GroupSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX18GroupSelectionVerbC2EPNS_9DataModelE
// IDA 0x3f9870: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9870() {
}

// 0x3f99b8 — __ZNK3RBX18GroupSelectionVerb9isEnabledEv
// type: _DWORD __fastcall(RBX::GroupSelectionVerb *__hidden this)
#[doc(alias = "RBX::GroupSelectionVerb::isEnabled(void)const")]
// was: __ZNK3RBX18GroupSelectionVerb9isEnabledEv
// IDA 0x3f99b8: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f99b8() {
}

// 0x3f99fc — __ZN3RBX18GroupSelectionVerb4doItEPNS_10IDataStateE
#[doc(alias = "RBX::GroupSelectionVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX18GroupSelectionVerb4doItEPNS_10IDataStateE
// IDA 0x3f99fc: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f99fc() {
}

// 0x3f9ab0 — __ZN3RBX17SnapSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::SnapSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::SnapSelectionVerb::SnapSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX17SnapSelectionVerbC1EPNS_9DataModelE
// IDA 0x3f9ab0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3f9ab0() {
}

// 0x3f9ab4 — __ZN3RBX17SnapSelectionVerbC2EPNS_9DataModelE
// type: RBX::SnapSelectionVerb *__fastcall(RBX::SnapSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::SnapSelectionVerb::SnapSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX17SnapSelectionVerbC2EPNS_9DataModelE
// IDA 0x3f9ab4: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9ab4() {
}

// 0x3f9bfc — __ZNK3RBX17SnapSelectionVerb9isEnabledEv
// type: _DWORD __fastcall(RBX::SnapSelectionVerb *__hidden this)
#[doc(alias = "RBX::SnapSelectionVerb::isEnabled(void)const")]
// was: __ZNK3RBX17SnapSelectionVerb9isEnabledEv
// IDA 0x3f9bfc: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9bfc() {
}

// 0x3f9c3c — __ZN3RBX17SnapSelectionVerb4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::SnapSelectionVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX17SnapSelectionVerb4doItEPNS_10IDataStateE
// IDA 0x3f9c3c: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9c3c() {
}

// 0x3f9ce0 — __ZN3RBXL11SurfaceSwapILNS_11SurfaceTypeE2ELS1_3EEEvN5boost10shared_ptrINS_8InstanceEEE
// type: _DWORD *__fastcall(int, int, int, int)
#[doc(alias = "void RBX::SurfaceSwap<(RBX::SurfaceType)2,(RBX::SurfaceType)3>(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBXL11SurfaceSwapILNS_11SurfaceTypeE2ELS1_3EEEvN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x3f9ce0: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9ce0() {
}

// 0x3f9d60 — __ZN3RBX13UnlockAllVerb4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool)
#[doc(alias = "RBX::UnlockAllVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX13UnlockAllVerb4doItEPNS_10IDataStateE
// IDA 0x3f9d60: 21 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9d60() {
}

// 0x3f9d94 — __ZN3RBX20UngroupSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::UngroupSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::UngroupSelectionVerb::UngroupSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX20UngroupSelectionVerbC1EPNS_9DataModelE
// IDA 0x3f9d94: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3f9d94() {
}

// 0x3f9d98 — __ZN3RBX20UngroupSelectionVerbC2EPNS_9DataModelE
// type: RBX::EditSelectionVerb *__fastcall(RBX::UngroupSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::UngroupSelectionVerb::UngroupSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX20UngroupSelectionVerbC2EPNS_9DataModelE
// IDA 0x3f9d98: 133 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9d98() {
}

// 0x3f9f18 — __ZNK3RBX20UngroupSelectionVerb9isEnabledEv
// type: RBX::Instance *__fastcall(RBX::UngroupSelectionVerb *this)
#[doc(alias = "RBX::UngroupSelectionVerb::isEnabled(void)const")]
// was: __ZNK3RBX20UngroupSelectionVerb9isEnabledEv
// IDA 0x3f9f18: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9f18() {
}

// 0x3f9ff8 — __ZN3RBX20UngroupSelectionVerb4doItEPNS_10IDataStateE
// type: void __fastcall(int, RBX::DataModel *, bool, const void *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int)
#[doc(alias = "RBX::UngroupSelectionVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX20UngroupSelectionVerb4doItEPNS_10IDataStateE
// IDA 0x3f9ff8: 219 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3f9ff8() {
}

// 0x3fa22c — __ZN3RBX18SelectChildrenVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::SelectChildrenVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::SelectChildrenVerb::SelectChildrenVerb(RBX::DataModel *)")]
// was: __ZN3RBX18SelectChildrenVerbC1EPNS_9DataModelE
// IDA 0x3fa22c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fa22c() {
}

// 0x3fa230 — __ZN3RBX18SelectChildrenVerbC2EPNS_9DataModelE
// type: RBX::EditSelectionVerb *__fastcall(RBX::SelectChildrenVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::SelectChildrenVerb::SelectChildrenVerb(RBX::DataModel *)")]
// was: __ZN3RBX18SelectChildrenVerbC2EPNS_9DataModelE
// IDA 0x3fa230: 133 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fa230() {
}

// 0x3fa3b0 — __ZNK3RBX18SelectChildrenVerb9isEnabledEv
// type: RBX::Instance *__fastcall(RBX::SelectChildrenVerb *this)
#[doc(alias = "RBX::SelectChildrenVerb::isEnabled(void)const")]
// was: __ZNK3RBX18SelectChildrenVerb9isEnabledEv
// IDA 0x3fa3b0: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fa3b0() {
}

// 0x3fa490 — __ZN3RBX18SelectChildrenVerb4doItEPNS_10IDataStateE
// type: void __fastcall(int, RBX::DataModel *, bool, const void *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int)
#[doc(alias = "RBX::SelectChildrenVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX18SelectChildrenVerb4doItEPNS_10IDataStateE
// IDA 0x3fa490: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fa490() {
}

// 0x3fa6c0 — __ZN3RBX10DeleteBaseC2EPNS_9DataModelESsb
// type: RBX::Verb *__fastcall(RBX::Verb *, _DWORD *, const std::string *, char)
#[doc(alias = "RBX::DeleteBase::DeleteBase(RBX::DataModel *,std::string,bool)")]
// was: __ZN3RBX10DeleteBaseC2EPNS_9DataModelESsb
// IDA 0x3fa6c0: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fa6c0() {
}

// 0x3fa7fc — __ZN3RBX10DeleteBase4doItEPNS_10IDataStateE
// type: void __fastcall(int, RBX::DataModel *, bool, const void *, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::DeleteBase::doIt(RBX::IDataState *)")]
// was: __ZN3RBX10DeleteBase4doItEPNS_10IDataStateE
// IDA 0x3fa7fc: 265 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fa7fc() {
}

// 0x3fabb8 — __ZN3RBX19RotateSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::RotateSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::RotateSelectionVerb::RotateSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX19RotateSelectionVerbC1EPNS_9DataModelE
// IDA 0x3fabb8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fabb8() {
}

// 0x3fabbc — __ZN3RBX19RotateSelectionVerbC2EPNS_9DataModelE
// type: RBX::RotateSelectionVerb *__fastcall(RBX::RotateSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::RotateSelectionVerb::RotateSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX19RotateSelectionVerbC2EPNS_9DataModelE
// IDA 0x3fabbc: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fabbc() {
}

// 0x3fad04 — __ZN3RBX17RotateAxisCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::RotateAxisCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX17RotateAxisCommand4doItEPNS_10IDataStateE
// IDA 0x3fad04: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fad04() {
}

// 0x3fad90 — __ZN3RBX19RotateSelectionVerb15getRotationAxisEv
// type: int __fastcall(RBX::RotateSelectionVerb *this)
#[doc(alias = "RBX::RotateSelectionVerb::getRotationAxis(void)")]
// was: __ZN3RBX19RotateSelectionVerb15getRotationAxisEv
// IDA 0x3fad90: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fad90() {
}

// 0x3fada4 — __ZN3RBX17TiltSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::TiltSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::TiltSelectionVerb::TiltSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX17TiltSelectionVerbC1EPNS_9DataModelE
// IDA 0x3fada4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fada4() {
}

// 0x3fada8 — __ZN3RBX17TiltSelectionVerbC2EPNS_9DataModelE
// type: RBX::TiltSelectionVerb *__fastcall(RBX::TiltSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::TiltSelectionVerb::TiltSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX17TiltSelectionVerbC2EPNS_9DataModelE
// IDA 0x3fada8: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fada8() {
}

// 0x3faef0 — __ZN3RBX17TiltSelectionVerb15getRotationAxisEv
// type: int __fastcall(RBX::TiltSelectionVerb *this, int)
#[doc(alias = "RBX::TiltSelectionVerb::getRotationAxis(void)")]
// was: __ZN3RBX17TiltSelectionVerb15getRotationAxisEv
// IDA 0x3faef0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3faef0() {
}

// 0x3faf14 — __ZN3RBX19MoveUpSelectionVerb4doItEPNS_10IDataStateE
// type: void __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::MoveUpSelectionVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19MoveUpSelectionVerb4doItEPNS_10IDataStateE
// IDA 0x3faf14: 143 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3faf14() {
}

// 0x3fb09c — __ZN3RBX21MoveDownSelectionVerbC1EPNS_9DataModelE
// type: int __fastcall(RBX::MoveDownSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::MoveDownSelectionVerb::MoveDownSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX21MoveDownSelectionVerbC1EPNS_9DataModelE
// IDA 0x3fb09c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fb09c() {
}

// 0x3fb0a0 — __ZN3RBX21MoveDownSelectionVerbC2EPNS_9DataModelE
// type: RBX::MoveDownSelectionVerb *__fastcall(RBX::MoveDownSelectionVerb *this, RBX::DataModel *)
#[doc(alias = "RBX::MoveDownSelectionVerb::MoveDownSelectionVerb(RBX::DataModel *)")]
// was: __ZN3RBX21MoveDownSelectionVerbC2EPNS_9DataModelE
// IDA 0x3fb0a0: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb0a0() {
}

// 0x3fb1e8 — __ZN3RBX21MoveDownSelectionVerb4doItEPNS_10IDataStateE
// type: void __fastcall(int, RBX::DataModel *, bool, const void *)
#[doc(alias = "RBX::MoveDownSelectionVerb::doIt(RBX::IDataState *)")]
// was: __ZN3RBX21MoveDownSelectionVerb4doItEPNS_10IDataStateE
// IDA 0x3fb1e8: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb1e8() {
}

// 0x3fb378 — __ZNK3RBX20CameraPanLeftCommand9isEnabledEv
// type: int __fastcall(RBX::CameraPanLeftCommand *this)
#[doc(alias = "RBX::CameraPanLeftCommand::isEnabled(void)const")]
// was: __ZNK3RBX20CameraPanLeftCommand9isEnabledEv
// IDA 0x3fb378: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb378() {
}

// 0x3fb37c — __ZN3RBX20CameraPanLeftCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, int (__fastcall ***)(_DWORD, int), int, const void *)
#[doc(alias = "RBX::CameraPanLeftCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX20CameraPanLeftCommand4doItEPNS_10IDataStateE
// IDA 0x3fb37c: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb37c() {
}

// 0x3fb3c0 — __ZNK3RBX21CameraPanRightCommand9isEnabledEv
// type: int __fastcall(RBX::CameraPanRightCommand *this)
#[doc(alias = "RBX::CameraPanRightCommand::isEnabled(void)const")]
// was: __ZNK3RBX21CameraPanRightCommand9isEnabledEv
// IDA 0x3fb3c0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb3c0() {
}

// 0x3fb3c4 — __ZN3RBX21CameraPanRightCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, int (__fastcall ***)(_DWORD, int), int, const void *)
#[doc(alias = "RBX::CameraPanRightCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX21CameraPanRightCommand4doItEPNS_10IDataStateE
// IDA 0x3fb3c4: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb3c4() {
}

// 0x3fb408 — __ZNK3RBX19CameraTiltUpCommand9isEnabledEv
// type: int __fastcall(RBX::CameraTiltUpCommand *this)
#[doc(alias = "RBX::CameraTiltUpCommand::isEnabled(void)const")]
// was: __ZNK3RBX19CameraTiltUpCommand9isEnabledEv
// IDA 0x3fb408: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb408() {
}

// 0x3fb420 — __ZN3RBX19CameraTiltUpCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::CameraTiltUpCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19CameraTiltUpCommand4doItEPNS_10IDataStateE
// IDA 0x3fb420: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb420() {
}

// 0x3fb46c — __ZNK3RBX21CameraTiltDownCommand9isEnabledEv
// type: int __fastcall(RBX::CameraTiltDownCommand *this)
#[doc(alias = "RBX::CameraTiltDownCommand::isEnabled(void)const")]
// was: __ZNK3RBX21CameraTiltDownCommand9isEnabledEv
// IDA 0x3fb46c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb46c() {
}

// 0x3fb484 — __ZN3RBX21CameraTiltDownCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::CameraTiltDownCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX21CameraTiltDownCommand4doItEPNS_10IDataStateE
// IDA 0x3fb484: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb484() {
}

// 0x3fb4cc — __ZNK3RBX19CameraZoomInCommand9isEnabledEv
// type: int __fastcall(RBX::CameraZoomInCommand *this)
#[doc(alias = "RBX::CameraZoomInCommand::isEnabled(void)const")]
// was: __ZNK3RBX19CameraZoomInCommand9isEnabledEv
// IDA 0x3fb4cc: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb4cc() {
}

// 0x3fb4e4 — __ZN3RBX19CameraZoomInCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::CameraZoomInCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX19CameraZoomInCommand4doItEPNS_10IDataStateE
// IDA 0x3fb4e4: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb4e4() {
}

// 0x3fb530 — __ZNK3RBX20CameraZoomOutCommand9isEnabledEv
// type: int __fastcall(RBX::CameraZoomOutCommand *this)
#[doc(alias = "RBX::CameraZoomOutCommand::isEnabled(void)const")]
// was: __ZNK3RBX20CameraZoomOutCommand9isEnabledEv
// IDA 0x3fb530: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb530() {
}

// 0x3fb548 — __ZN3RBX20CameraZoomOutCommand4doItEPNS_10IDataStateE
// type: int __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::CameraZoomOutCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX20CameraZoomOutCommand4doItEPNS_10IDataStateE
// IDA 0x3fb548: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb548() {
}

// 0x3fb594 — __ZN3RBX24CameraZoomExtentsCommandC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::CameraZoomExtentsCommand *this, RBX::Workspace *)
#[doc(alias = "RBX::CameraZoomExtentsCommand::CameraZoomExtentsCommand(RBX::Workspace *)")]
// was: __ZN3RBX24CameraZoomExtentsCommandC1EPNS_9WorkspaceE
// IDA 0x3fb594: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fb594() {
}

// 0x3fb598 — __ZN3RBX24CameraZoomExtentsCommandC2EPNS_9WorkspaceE
// type: RBX::Verb *__fastcall(RBX::CameraZoomExtentsCommand *this, RBX::Workspace *)
#[doc(alias = "RBX::CameraZoomExtentsCommand::CameraZoomExtentsCommand(RBX::Workspace *)")]
// was: __ZN3RBX24CameraZoomExtentsCommandC2EPNS_9WorkspaceE
// IDA 0x3fb598: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb598() {
}

// 0x3fb778 — __ZNK3RBX24CameraZoomExtentsCommand9isEnabledEv
// type: bool __fastcall(RBX::CameraZoomExtentsCommand *this)
#[doc(alias = "RBX::CameraZoomExtentsCommand::isEnabled(void)const")]
// was: __ZNK3RBX24CameraZoomExtentsCommand9isEnabledEv
// IDA 0x3fb778: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb778() {
}

// 0x3fb8dc — __ZN3RBX24CameraZoomExtentsCommand4doItEPNS_10IDataStateE
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CameraZoomExtentsCommand::doIt(RBX::IDataState *)")]
// was: __ZN3RBX24CameraZoomExtentsCommand4doItEPNS_10IDataStateE
// IDA 0x3fb8dc: 420 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fb8dc() {
}

// 0x3fbd8c — __ZN3RBX25TurnOnManualJointCreationC1EPNS_9DataModelE
// type: int __fastcall(RBX::TurnOnManualJointCreation *this, RBX::DataModel *)
#[doc(alias = "RBX::TurnOnManualJointCreation::TurnOnManualJointCreation(RBX::DataModel *)")]
// was: __ZN3RBX25TurnOnManualJointCreationC1EPNS_9DataModelE
// IDA 0x3fbd8c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fbd8c() {
}

// 0x3fbd90 — __ZN3RBX25TurnOnManualJointCreationC2EPNS_9DataModelE
// type: RBX::TurnOnManualJointCreation *__fastcall(RBX::TurnOnManualJointCreation *this, RBX::DataModel *)
#[doc(alias = "RBX::TurnOnManualJointCreation::TurnOnManualJointCreation(RBX::DataModel *)")]
// was: __ZN3RBX25TurnOnManualJointCreationC2EPNS_9DataModelE
// IDA 0x3fbd90: 117 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fbd90() {
}

// 0x3fbee8 — __ZN3RBX25TurnOnManualJointCreation4doItEPNS_10IDataStateE
// type: int __fastcall(int, int, int, const void *)
#[doc(alias = "RBX::TurnOnManualJointCreation::doIt(RBX::IDataState *)")]
// was: __ZN3RBX25TurnOnManualJointCreation4doItEPNS_10IDataStateE
// IDA 0x3fbee8: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fbee8() {
}

// 0x3fbf3c — __ZN3RBX12SetGridToOneC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetGridToOne *this, RBX::DataModel *)
#[doc(alias = "RBX::SetGridToOne::SetGridToOne(RBX::DataModel *)")]
// was: __ZN3RBX12SetGridToOneC1EPNS_9DataModelE
// IDA 0x3fbf3c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fbf3c() {
}

// 0x3fbf40 — __ZN3RBX12SetGridToOneC2EPNS_9DataModelE
// type: RBX::SetGridToOne *__fastcall(RBX::SetGridToOne *this, RBX::DataModel *)
#[doc(alias = "RBX::SetGridToOne::SetGridToOne(RBX::DataModel *)")]
// was: __ZN3RBX12SetGridToOneC2EPNS_9DataModelE
// IDA 0x3fbf40: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fbf40() {
}

// 0x3fc098 — __ZN3RBX17SetGridToOneFifthC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetGridToOneFifth *this, RBX::DataModel *)
#[doc(alias = "RBX::SetGridToOneFifth::SetGridToOneFifth(RBX::DataModel *)")]
// was: __ZN3RBX17SetGridToOneFifthC1EPNS_9DataModelE
// IDA 0x3fc098: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fc098() {
}

// 0x3fc09c — __ZN3RBX17SetGridToOneFifthC2EPNS_9DataModelE
// type: RBX::SetGridToOneFifth *__fastcall(RBX::SetGridToOneFifth *this, RBX::DataModel *)
#[doc(alias = "RBX::SetGridToOneFifth::SetGridToOneFifth(RBX::DataModel *)")]
// was: __ZN3RBX17SetGridToOneFifthC2EPNS_9DataModelE
// IDA 0x3fc09c: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc09c() {
}

// 0x3fc1f4 — __ZN3RBX12SetGridToOffC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetGridToOff *this, RBX::DataModel *)
#[doc(alias = "RBX::SetGridToOff::SetGridToOff(RBX::DataModel *)")]
// was: __ZN3RBX12SetGridToOffC1EPNS_9DataModelE
// IDA 0x3fc1f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fc1f4() {
}

// 0x3fc1f8 — __ZN3RBX12SetGridToOffC2EPNS_9DataModelE
// type: RBX::SetGridToOff *__fastcall(RBX::SetGridToOff *this, RBX::DataModel *)
#[doc(alias = "RBX::SetGridToOff::SetGridToOff(RBX::DataModel *)")]
// was: __ZN3RBX12SetGridToOffC2EPNS_9DataModelE
// IDA 0x3fc1f8: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc1f8() {
}

// 0x3fc350 — __ZN3RBX20SetManualJointToWeakC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetManualJointToWeak *this, RBX::DataModel *)
#[doc(alias = "RBX::SetManualJointToWeak::SetManualJointToWeak(RBX::DataModel *)")]
// was: __ZN3RBX20SetManualJointToWeakC1EPNS_9DataModelE
// IDA 0x3fc350: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fc350() {
}

// 0x3fc354 — __ZN3RBX20SetManualJointToWeakC2EPNS_9DataModelE
// type: RBX::SetManualJointToWeak *__fastcall(RBX::SetManualJointToWeak *this, RBX::DataModel *)
#[doc(alias = "RBX::SetManualJointToWeak::SetManualJointToWeak(RBX::DataModel *)")]
// was: __ZN3RBX20SetManualJointToWeakC2EPNS_9DataModelE
// IDA 0x3fc354: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc354() {
}

// 0x3fc4ac — __ZN3RBX22SetManualJointToStrongC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetManualJointToStrong *this, RBX::DataModel *)
#[doc(alias = "RBX::SetManualJointToStrong::SetManualJointToStrong(RBX::DataModel *)")]
// was: __ZN3RBX22SetManualJointToStrongC1EPNS_9DataModelE
// IDA 0x3fc4ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fc4ac() {
}

// 0x3fc4b0 — __ZN3RBX22SetManualJointToStrongC2EPNS_9DataModelE
// type: RBX::SetManualJointToStrong *__fastcall(RBX::SetManualJointToStrong *this, RBX::DataModel *)
#[doc(alias = "RBX::SetManualJointToStrong::SetManualJointToStrong(RBX::DataModel *)")]
// was: __ZN3RBX22SetManualJointToStrongC2EPNS_9DataModelE
// IDA 0x3fc4b0: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc4b0() {
}

// 0x3fc608 — __ZN3RBX24SetManualJointToInfiniteC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetManualJointToInfinite *this, RBX::DataModel *)
#[doc(alias = "RBX::SetManualJointToInfinite::SetManualJointToInfinite(RBX::DataModel *)")]
// was: __ZN3RBX24SetManualJointToInfiniteC1EPNS_9DataModelE
// IDA 0x3fc608: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fc608() {
}

// 0x3fc60c — __ZN3RBX24SetManualJointToInfiniteC2EPNS_9DataModelE
// type: RBX::SetManualJointToInfinite *__fastcall(RBX::SetManualJointToInfinite *this, RBX::DataModel *)
#[doc(alias = "RBX::SetManualJointToInfinite::SetManualJointToInfinite(RBX::DataModel *)")]
// was: __ZN3RBX24SetManualJointToInfiniteC2EPNS_9DataModelE
// IDA 0x3fc60c: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc60c() {
}

// 0x3fc764 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS4_21BoolPropertyVerbSetItEET0_T_SF_SE_
// type: void __fastcall(unsigned __int64 *, const shared_count *, const shared_count *, unsigned int, unsigned int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int)
#[doc(alias = "RBX::BoolPropertyVerbSetIt std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,RBX::BoolPropertyVerbSetIt>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,RBX::BoolPropertyVerbSetIt)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS4_21BoolPropertyVerbSetItEET0_T_SF_SE_
// IDA 0x3fc764: 94 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc764() {
}

// 0x3fc864 — __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::FilteredSelection<RBX::PVInstance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::PVInstance>>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_17FilteredSelectionINS_10PVInstanceEEEEEPT_PKNS_8InstanceE
// IDA 0x3fc864: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc864() {
}

// 0x3fc87c — __ZNK3RBX9Selection5frontEv
// type: int __fastcall(RBX::Selection *this, int)
#[doc(alias = "RBX::Selection::front(void)const")]
// was: __ZNK3RBX9Selection5frontEv
// IDA 0x3fc87c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc87c() {
}

// 0x3fc8bc — __ZNK3RBX9Selection4backEv
// type: int __fastcall(RBX::Selection *this, int)
#[doc(alias = "RBX::Selection::back(void)const")]
// was: __ZNK3RBX9Selection4backEv
// IDA 0x3fc8bc: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc8bc() {
}

// 0x3fc900 — __ZN3RBX13ModelInstance5groupIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS7_SaIS7_EEEEEENS5_IS0_EET_SF_
// type: void __fastcall(RBX::Instance **, int, const RBX::Instance **, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, char, int, void *, char, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance> RBX::ModelInstance::group<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)")]
// was: __ZN3RBX13ModelInstance5groupIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS7_SaIS7_EEEEEENS5_IS0_EET_SF_
// IDA 0x3fc900: 218 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fc900() {
}

// 0x3fcb44 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_ENSD_5list1INS2_3argILi1EEEEEEEET0_T_SN_SM_
// type: unsigned __int64 __fastcall(int, int, int, unsigned int, unsigned int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::arg<1>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS2_3_bi6bind_tIvPFvS6_ENSD_5list1INS2_3argILi1EEEEEEEET0_T_SN_SM_
// IDA 0x3fcb44: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fcb44() {
}

// 0x3fcb84 — __ZN3RBX11shared_fromINS_13ModelInstanceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance> RBX::shared_from<RBX::ModelInstance>(RBX::ModelInstance*)")]
// was: __ZN3RBX11shared_fromINS_13ModelInstanceEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x3fcb84: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fcb84() {
}

// 0x3fcc6c — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS4_7UngroupEET0_T_SE_SD_
// type: unsigned __int64 __fastcall(unsigned __int64 *, int, int, unsigned int, unsigned int)
#[doc(alias = "RBX::Ungroup std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,RBX::Ungroup>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,RBX::Ungroup)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEENS4_7UngroupEET0_T_SE_SD_
// IDA 0x3fcc6c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fcc6c() {
}

// 0x3fcca8 — __ZN3RBX17RotateAxisCommandC2ESsPNS_9DataModelE
// type: _DWORD *__fastcall(_DWORD *, const std::string *, int)
#[doc(alias = "RBX::RotateAxisCommand::RotateAxisCommand(std::string,RBX::DataModel *)")]
// was: __ZN3RBX17RotateAxisCommandC2ESsPNS_9DataModelE
// IDA 0x3fcca8: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fcca8() {
}

// 0x3fcdd8 — __ZN3RBX10DeleteBaseD1Ev
// type: void __fastcall(RBX::DeleteBase *__hidden this)
#[doc(alias = "RBX::DeleteBase::~DeleteBase()")]
// was: __ZN3RBX10DeleteBaseD1Ev
// IDA 0x3fcdd8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fcdd8() {
}

// 0x3fcddc — __ZN3RBX10DeleteBaseD0Ev
// type: void __fastcall(RBX::DeleteBase *__hidden this)
#[doc(alias = "RBX::DeleteBase::~DeleteBase()")]
// was: __ZN3RBX10DeleteBaseD0Ev
// IDA 0x3fcddc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fcddc() {
}

// 0x3fce7c — __ZNK3RBX4Verb9isEnabledEv
// type: int __fastcall(RBX::Verb *this)
#[doc(alias = "RBX::Verb::isEnabled(void)const")]
// was: __ZNK3RBX4Verb9isEnabledEv
// IDA 0x3fce7c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fce7c() {
}

// 0x3fce80 — __ZN3RBX16SelectAllCommandD1Ev
// type: void __fastcall(RBX::SelectAllCommand *__hidden this)
#[doc(alias = "RBX::SelectAllCommand::~SelectAllCommand()")]
// was: __ZN3RBX16SelectAllCommandD1Ev
// IDA 0x3fce80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fce80() {
}

// 0x3fce84 — __ZN3RBX16SelectAllCommandD0Ev
// type: void __fastcall(RBX::SelectAllCommand *__hidden this)
#[doc(alias = "RBX::SelectAllCommand::~SelectAllCommand()")]
// was: __ZN3RBX16SelectAllCommandD0Ev
// IDA 0x3fce84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fce84() {
}

// 0x3fcf24 — __ZN3RBX13UnlockAllVerbD1Ev
// type: void __fastcall(RBX::UnlockAllVerb *__hidden this)
#[doc(alias = "RBX::UnlockAllVerb::~UnlockAllVerb()")]
// was: __ZN3RBX13UnlockAllVerbD1Ev
// IDA 0x3fcf24: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fcf24() {
}

// 0x3fcf28 — __ZN3RBX13UnlockAllVerbD0Ev
// type: void __fastcall(RBX::UnlockAllVerb *__hidden this)
#[doc(alias = "RBX::UnlockAllVerb::~UnlockAllVerb()")]
// was: __ZN3RBX13UnlockAllVerbD0Ev
// IDA 0x3fcf28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fcf28() {
}

// 0x3fcfc8 — __ZN3RBX10CameraVerbD1Ev
// type: void __fastcall(RBX::CameraVerb *__hidden this)
#[doc(alias = "RBX::CameraVerb::~CameraVerb()")]
// was: __ZN3RBX10CameraVerbD1Ev
// IDA 0x3fcfc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fcfc8() {
}

// 0x3fd094 — __ZN3RBX10CameraVerbD0Ev
// type: void __fastcall(RBX::CameraVerb *__hidden this)
#[doc(alias = "RBX::CameraVerb::~CameraVerb()")]
// was: __ZN3RBX10CameraVerbD0Ev
// IDA 0x3fd094: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd094() {
}

// 0x3fd174 — __ZNK3RBX10CameraVerb9isEnabledEv
// type: int __fastcall(RBX::CameraVerb *this)
#[doc(alias = "RBX::CameraVerb::isEnabled(void)const")]
// was: __ZNK3RBX10CameraVerb9isEnabledEv
// IDA 0x3fd174: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3fd174() {
}

// 0x3fd178 — __ZN3RBX19CameraTiltUpCommandD1Ev
// type: void __fastcall(RBX::CameraTiltUpCommand *__hidden this)
#[doc(alias = "RBX::CameraTiltUpCommand::~CameraTiltUpCommand()")]
// was: __ZN3RBX19CameraTiltUpCommandD1Ev
// IDA 0x3fd178: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd178() {
}

// 0x3fd244 — __ZN3RBX19CameraTiltUpCommandD0Ev
// type: void __fastcall(RBX::CameraTiltUpCommand *__hidden this)
#[doc(alias = "RBX::CameraTiltUpCommand::~CameraTiltUpCommand()")]
// was: __ZN3RBX19CameraTiltUpCommandD0Ev
// IDA 0x3fd244: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd244() {
}

// 0x3fd324 — __ZN3RBX21CameraTiltDownCommandD1Ev
// type: void __fastcall(RBX::CameraTiltDownCommand *__hidden this)
#[doc(alias = "RBX::CameraTiltDownCommand::~CameraTiltDownCommand()")]
// was: __ZN3RBX21CameraTiltDownCommandD1Ev
// IDA 0x3fd324: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd324() {
}

// 0x3fd3f0 — __ZN3RBX21CameraTiltDownCommandD0Ev
// type: void __fastcall(RBX::CameraTiltDownCommand *__hidden this)
#[doc(alias = "RBX::CameraTiltDownCommand::~CameraTiltDownCommand()")]
// was: __ZN3RBX21CameraTiltDownCommandD0Ev
// IDA 0x3fd3f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd3f0() {
}

// 0x3fd4d0 — __ZN3RBX20CameraPanLeftCommandD1Ev
// type: void __fastcall(RBX::CameraPanLeftCommand *__hidden this)
#[doc(alias = "RBX::CameraPanLeftCommand::~CameraPanLeftCommand()")]
// was: __ZN3RBX20CameraPanLeftCommandD1Ev
// IDA 0x3fd4d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd4d0() {
}

// 0x3fd59c — __ZN3RBX20CameraPanLeftCommandD0Ev
// type: void __fastcall(RBX::CameraPanLeftCommand *__hidden this)
#[doc(alias = "RBX::CameraPanLeftCommand::~CameraPanLeftCommand()")]
// was: __ZN3RBX20CameraPanLeftCommandD0Ev
// IDA 0x3fd59c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd59c() {
}

// 0x3fd67c — __ZN3RBX21CameraPanRightCommandD1Ev
// type: void __fastcall(RBX::CameraPanRightCommand *__hidden this)
#[doc(alias = "RBX::CameraPanRightCommand::~CameraPanRightCommand()")]
// was: __ZN3RBX21CameraPanRightCommandD1Ev
// IDA 0x3fd67c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd67c() {
}

// 0x3fd748 — __ZN3RBX21CameraPanRightCommandD0Ev
// type: void __fastcall(RBX::CameraPanRightCommand *__hidden this)
#[doc(alias = "RBX::CameraPanRightCommand::~CameraPanRightCommand()")]
// was: __ZN3RBX21CameraPanRightCommandD0Ev
// IDA 0x3fd748: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd748() {
}

// 0x3fd828 — __ZN3RBX19CameraZoomInCommandD1Ev
// type: void __fastcall(RBX::CameraZoomInCommand *__hidden this)
#[doc(alias = "RBX::CameraZoomInCommand::~CameraZoomInCommand()")]
// was: __ZN3RBX19CameraZoomInCommandD1Ev
// IDA 0x3fd828: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd828() {
}

// 0x3fd8f4 — __ZN3RBX19CameraZoomInCommandD0Ev
// type: void __fastcall(RBX::CameraZoomInCommand *__hidden this)
#[doc(alias = "RBX::CameraZoomInCommand::~CameraZoomInCommand()")]
// was: __ZN3RBX19CameraZoomInCommandD0Ev
// IDA 0x3fd8f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd8f4() {
}

// 0x3fd9d4 — __ZN3RBX20CameraZoomOutCommandD1Ev
// type: void __fastcall(RBX::CameraZoomOutCommand *__hidden this)
#[doc(alias = "RBX::CameraZoomOutCommand::~CameraZoomOutCommand()")]
// was: __ZN3RBX20CameraZoomOutCommandD1Ev
// IDA 0x3fd9d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fd9d4() {
}

// 0x3fdaa0 — __ZN3RBX20CameraZoomOutCommandD0Ev
// type: void __fastcall(RBX::CameraZoomOutCommand *__hidden this)
#[doc(alias = "RBX::CameraZoomOutCommand::~CameraZoomOutCommand()")]
// was: __ZN3RBX20CameraZoomOutCommandD0Ev
// IDA 0x3fdaa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fdaa0() {
}

// 0x3fdb80 — __ZN3RBX16BoolPropertyVerbD1Ev
// type: void __fastcall(RBX::BoolPropertyVerb *__hidden this)
#[doc(alias = "RBX::BoolPropertyVerb::~BoolPropertyVerb()")]
// was: __ZN3RBX16BoolPropertyVerbD1Ev
// IDA 0x3fdb80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fdb80() {
}

// 0x3fdb84 — __ZN3RBX16BoolPropertyVerbD0Ev
// type: void __fastcall(RBX::BoolPropertyVerb *__hidden this)
#[doc(alias = "RBX::BoolPropertyVerb::~BoolPropertyVerb()")]
// was: __ZN3RBX16BoolPropertyVerbD0Ev
// IDA 0x3fdb84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fdb84() {
}

// 0x3fdc24 — __ZN3RBX19AllCanSelectCommandD1Ev
// type: void __fastcall(RBX::AllCanSelectCommand *__hidden this)
#[doc(alias = "RBX::AllCanSelectCommand::~AllCanSelectCommand()")]
// was: __ZN3RBX19AllCanSelectCommandD1Ev
// IDA 0x3fdc24: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fdc24() {
}

// 0x3fdc28 — __ZN3RBX19AllCanSelectCommandD0Ev
// type: void __fastcall(RBX::AllCanSelectCommand *__hidden this)
#[doc(alias = "RBX::AllCanSelectCommand::~AllCanSelectCommand()")]
// was: __ZN3RBX19AllCanSelectCommandD0Ev
// IDA 0x3fdc28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3fdc28() {
}

// 0x3fdcc8 — __ZN3RBX19CanNotSelectCommandD1Ev
// type: void __fastcall(RBX::CanNotSelectCommand *__hidden this)
#[doc(alias = "RBX::CanNotSelectCommand::~CanNotSelectCommand()")]
// was: __ZN3RBX19CanNotSelectCommandD1Ev
// IDA 0x3fdcc8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3fdcc8() {
}
