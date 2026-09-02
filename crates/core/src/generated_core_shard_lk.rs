//! core shard lk — 100 core stubs EA-sorted, next uncovered fallback after shard lj (0x3fc098..0x41f378, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 41432, 9332->9232 uncovered, 37899->37999 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::SetGridToOneFifth::SetGridToOneFifth(RBX::DataModel *)")]
// 0x3fc098 — __ZN3RBX17SetGridToOneFifthC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetGridToOneFifth *this, RBX::DataModel *)
pub fn stub_0x3fc098() -> ! {
    todo!("0x3fc098 __ZN3RBX17SetGridToOneFifthC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetGridToOneFifth::SetGridToOneFifth(RBX::DataModel *)")]
// 0x3fc09c — __ZN3RBX17SetGridToOneFifthC2EPNS_9DataModelE
// type: RBX::SetGridToOneFifth *__fastcall(RBX::SetGridToOneFifth *this, RBX::DataModel *)
pub fn stub_0x3fc09c() -> ! {
    todo!("0x3fc09c __ZN3RBX17SetGridToOneFifthC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetGridToOff::SetGridToOff(RBX::DataModel *)")]
// 0x3fc1f4 — __ZN3RBX12SetGridToOffC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetGridToOff *this, RBX::DataModel *)
pub fn stub_0x3fc1f4() -> ! {
    todo!("0x3fc1f4 __ZN3RBX12SetGridToOffC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetGridToOff::SetGridToOff(RBX::DataModel *)")]
// 0x3fc1f8 — __ZN3RBX12SetGridToOffC2EPNS_9DataModelE
// type: RBX::SetGridToOff *__fastcall(RBX::SetGridToOff *this, RBX::DataModel *)
pub fn stub_0x3fc1f8() -> ! {
    todo!("0x3fc1f8 __ZN3RBX12SetGridToOffC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetManualJointToWeak::SetManualJointToWeak(RBX::DataModel *)")]
// 0x3fc350 — __ZN3RBX20SetManualJointToWeakC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetManualJointToWeak *this, RBX::DataModel *)
pub fn stub_0x3fc350() -> ! {
    todo!("0x3fc350 __ZN3RBX20SetManualJointToWeakC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetManualJointToWeak::SetManualJointToWeak(RBX::DataModel *)")]
// 0x3fc354 — __ZN3RBX20SetManualJointToWeakC2EPNS_9DataModelE
// type: RBX::SetManualJointToWeak *__fastcall(RBX::SetManualJointToWeak *this, RBX::DataModel *)
pub fn stub_0x3fc354() -> ! {
    todo!("0x3fc354 __ZN3RBX20SetManualJointToWeakC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetManualJointToStrong::SetManualJointToStrong(RBX::DataModel *)")]
// 0x3fc4ac — __ZN3RBX22SetManualJointToStrongC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetManualJointToStrong *this, RBX::DataModel *)
pub fn stub_0x3fc4ac() -> ! {
    todo!("0x3fc4ac __ZN3RBX22SetManualJointToStrongC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetManualJointToStrong::SetManualJointToStrong(RBX::DataModel *)")]
// 0x3fc4b0 — __ZN3RBX22SetManualJointToStrongC2EPNS_9DataModelE
// type: RBX::SetManualJointToStrong *__fastcall(RBX::SetManualJointToStrong *this, RBX::DataModel *)
pub fn stub_0x3fc4b0() -> ! {
    todo!("0x3fc4b0 __ZN3RBX22SetManualJointToStrongC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetManualJointToInfinite::SetManualJointToInfinite(RBX::DataModel *)")]
// 0x3fc608 — __ZN3RBX24SetManualJointToInfiniteC1EPNS_9DataModelE
// type: int __fastcall(RBX::SetManualJointToInfinite *this, RBX::DataModel *)
pub fn stub_0x3fc608() -> ! {
    todo!("0x3fc608 __ZN3RBX24SetManualJointToInfiniteC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::SetManualJointToInfinite::SetManualJointToInfinite(RBX::DataModel *)")]
// 0x3fc60c — __ZN3RBX24SetManualJointToInfiniteC2EPNS_9DataModelE
// type: RBX::SetManualJointToInfinite *__fastcall(RBX::SetManualJointToInfinite *this, RBX::DataModel *)
pub fn stub_0x3fc60c() -> ! {
    todo!("0x3fc60c __ZN3RBX24SetManualJointToInfiniteC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::RotateAxisCommand::RotateAxisCommand(std::string,RBX::DataModel *)")]
// 0x3fcca8 — __ZN3RBX17RotateAxisCommandC2ESsPNS_9DataModelE
// type: _DWORD *__fastcall(_DWORD *, const std::string *, int)
pub fn stub_0x3fcca8() -> ! {
    todo!("0x3fcca8 __ZN3RBX17RotateAxisCommandC2ESsPNS_9DataModelE")
}

#[doc(alias = "RBX::CommonVerbs::CommonVerbs(RBX::DataModel *)")]
// 0x403820 — __ZN3RBX11CommonVerbsC1EPNS_9DataModelE
// type: int __fastcall(RBX::CommonVerbs *this, RBX::DataModel *)
pub fn stub_0x403820() -> ! {
    todo!("0x403820 __ZN3RBX11CommonVerbsC1EPNS_9DataModelE")
}

#[doc(alias = "RBX::CommonVerbs::CommonVerbs(RBX::DataModel *)")]
// 0x403824 — __ZN3RBX11CommonVerbsC2EPNS_9DataModelE
// type: RBX::CommonVerbs *__fastcall(RBX::CommonVerbs *this, RBX::DataModel *)
pub fn stub_0x403824() -> ! {
    todo!("0x403824 __ZN3RBX11CommonVerbsC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x404394 — __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x404394() -> ! {
    todo!("0x404394 __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x404c8c — __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x404c8c() -> ! {
    todo!("0x404c8c __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x405580 — __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x405580() -> ! {
    todo!("0x405580 __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x405e74 — __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x405e74() -> ! {
    todo!("0x405e74 __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x406768 — __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x406768() -> ! {
    todo!("0x406768 __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40705c — __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40705c() -> ! {
    todo!("0x40705c __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x407994 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x407994() -> ! {
    todo!("0x407994 __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x408394 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x408394() -> ! {
    todo!("0x408394 __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x408d94 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x408d94() -> ! {
    todo!("0x408d94 __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x4098e0 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x4098e0() -> ! {
    todo!("0x4098e0 __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40a434 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40a434() -> ! {
    todo!("0x40a434 __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40ae34 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40ae34() -> ! {
    todo!("0x40ae34 __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40b76c — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40b76c() -> ! {
    todo!("0x40b76c __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40c0a4 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40c0a4() -> ! {
    todo!("0x40c0a4 __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40caa4 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40caa4() -> ! {
    todo!("0x40caa4 __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40d4a4 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40d4a4() -> ! {
    todo!("0x40d4a4 __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40dea4 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40dea4() -> ! {
    todo!("0x40dea4 __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40e8a4 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40e8a4() -> ! {
    todo!("0x40e8a4 __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40f2a4 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40f2a4() -> ! {
    todo!("0x40f2a4 __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x40fca4 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x40fca4() -> ! {
    todo!("0x40fca4 __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x4106a4 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x4106a4() -> ! {
    todo!("0x4106a4 __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x4110a4 — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x4110a4() -> ! {
    todo!("0x4110a4 __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x4114dc — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x4114dc() -> ! {
    todo!("0x4114dc __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x4121b8 — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x4121b8() -> ! {
    todo!("0x4121b8 __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x412c84 — __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x412c84() -> ! {
    todo!("0x412c84 __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// 0x413798 — __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
pub fn stub_0x413798() -> ! {
    todo!("0x413798 __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEEC2EPNS_9DataModelEb")
}

#[doc(alias = "RBX::ResetCommand::ResetCommand(RBX::DataModel *)")]
// 0x414914 — __ZN3RBX12ResetCommandC2EPNS_9DataModelE
// type: RBX::ResetCommand *__fastcall(RBX::ResetCommand *this, RBX::DataModel *)
pub fn stub_0x414914() -> ! {
    todo!("0x414914 __ZN3RBX12ResetCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::StopCommand::StopCommand(RBX::DataModel *)")]
// 0x414a5c — __ZN3RBX11StopCommandC2EPNS_9DataModelE
// type: RBX::StopCommand *__fastcall(RBX::StopCommand *this, RBX::DataModel *)
pub fn stub_0x414a5c() -> ! {
    todo!("0x414a5c __ZN3RBX11StopCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::RunCommand::RunCommand(RBX::DataModel *)")]
// 0x414ba4 — __ZN3RBX10RunCommandC2EPNS_9DataModelE
// type: RBX::RunCommand *__fastcall(RBX::RunCommand *this, RBX::DataModel *)
pub fn stub_0x414ba4() -> ! {
    todo!("0x414ba4 __ZN3RBX10RunCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::MoveUpBrickVerb::MoveUpBrickVerb(RBX::DataModel *)")]
// 0x414cec — __ZN3RBX15MoveUpBrickVerbC2EPNS_9DataModelE
// type: RBX::MoveUpBrickVerb *__fastcall(RBX::MoveUpBrickVerb *this, RBX::DataModel *)
pub fn stub_0x414cec() -> ! {
    todo!("0x414cec __ZN3RBX15MoveUpBrickVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::MoveUpSelectionVerb::MoveUpSelectionVerb(RBX::DataModel *,std::string const&,float)")]
// 0x414e3c — __ZN3RBX19MoveUpSelectionVerbC2EPNS_9DataModelERKSsf
// type: RBX::MoveUpSelectionVerb *__fastcall(RBX::MoveUpSelectionVerb *this, RBX::DataModel *, const std::string *, float)
pub fn stub_0x414e3c() -> ! {
    todo!("0x414e3c __ZN3RBX19MoveUpSelectionVerbC2EPNS_9DataModelERKSsf")
}

#[doc(alias = "RBX::MoveUpPlateVerb::MoveUpPlateVerb(RBX::DataModel *)")]
// 0x415020 — __ZN3RBX15MoveUpPlateVerbC2EPNS_9DataModelE
// type: RBX::MoveUpPlateVerb *__fastcall(RBX::MoveUpPlateVerb *this, RBX::DataModel *)
pub fn stub_0x415020() -> ! {
    todo!("0x415020 __ZN3RBX15MoveUpPlateVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::AllCanSelectCommand::AllCanSelectCommand(RBX::DataModel *)")]
// 0x415210 — __ZN3RBX19AllCanSelectCommandC2EPNS_9DataModelE
// type: RBX::AllCanSelectCommand *__fastcall(RBX::AllCanSelectCommand *this, RBX::DataModel *)
pub fn stub_0x415210() -> ! {
    todo!("0x415210 __ZN3RBX19AllCanSelectCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::CanNotSelectCommand::CanNotSelectCommand(RBX::DataModel *)")]
// 0x415358 — __ZN3RBX19CanNotSelectCommandC2EPNS_9DataModelE
// type: RBX::CanNotSelectCommand *__fastcall(RBX::CanNotSelectCommand *this, RBX::DataModel *)
pub fn stub_0x415358() -> ! {
    todo!("0x415358 __ZN3RBX19CanNotSelectCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::CanCollideVerb::CanCollideVerb(RBX::DataModel *)")]
// 0x4154a0 — __ZN3RBX14CanCollideVerbC2EPNS_9DataModelE
// type: char **__fastcall(char **this, RBX::DataModel *)
pub fn stub_0x4154a0() -> ! {
    todo!("0x4154a0 __ZN3RBX14CanCollideVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::TranslucentVerb::TranslucentVerb(RBX::DataModel *)")]
// 0x415694 — __ZN3RBX15TranslucentVerbC2EPNS_9DataModelE
// type: char **__fastcall(char **this, RBX::DataModel *)
pub fn stub_0x415694() -> ! {
    todo!("0x415694 __ZN3RBX15TranslucentVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::AnchorVerb::AnchorVerb(RBX::DataModel *)")]
// 0x415888 — __ZN3RBX10AnchorVerbC2EPNS_9DataModelE
// type: char **__fastcall(char **this, RBX::DataModel *)
pub fn stub_0x415888() -> ! {
    todo!("0x415888 __ZN3RBX10AnchorVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::UnlockAllVerb::UnlockAllVerb(RBX::DataModel *)")]
// 0x415a7c — __ZN3RBX13UnlockAllVerbC2EPNS_9DataModelE
// type: RBX::UnlockAllVerb *__fastcall(RBX::UnlockAllVerb *this, RBX::DataModel *)
pub fn stub_0x415a7c() -> ! {
    todo!("0x415a7c __ZN3RBX13UnlockAllVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::SelectAllCommand::SelectAllCommand(RBX::DataModel *)")]
// 0x415bc4 — __ZN3RBX16SelectAllCommandC2EPNS_9DataModelE
// type: RBX::SelectAllCommand *__fastcall(RBX::SelectAllCommand *this, RBX::DataModel *)
pub fn stub_0x415bc4() -> ! {
    todo!("0x415bc4 __ZN3RBX16SelectAllCommandC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::DeleteSelectionVerb::DeleteSelectionVerb(RBX::DataModel *)")]
// 0x415d0c — __ZN3RBX19DeleteSelectionVerbC2EPNS_9DataModelE
// type: RBX::DeleteSelectionVerb *__fastcall(RBX::DeleteSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x415d0c() -> ! {
    todo!("0x415d0c __ZN3RBX19DeleteSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::PlayDeleteSelectionVerb::PlayDeleteSelectionVerb(RBX::DataModel *)")]
// 0x415ef8 — __ZN3RBX23PlayDeleteSelectionVerbC2EPNS_9DataModelE
// type: RBX::PlayDeleteSelectionVerb *__fastcall(RBX::PlayDeleteSelectionVerb *this, RBX::DataModel *)
pub fn stub_0x415ef8() -> ! {
    todo!("0x415ef8 __ZN3RBX23PlayDeleteSelectionVerbC2EPNS_9DataModelE")
}

#[doc(alias = "RBX::dummyLoader(RBX::DataModel *)")]
// 0x419518 — __ZN3RBXL11dummyLoaderEPNS_9DataModelE
// type: void __fastcall(RBX *this, RBX::DataModel *)
pub fn stub_0x419518() -> ! {
    todo!("0x419518 __ZN3RBXL11dummyLoaderEPNS_9DataModelE")
}

#[doc(alias = "RBX::DataModel::get(RBX::ContentId)")]
// 0x41951c — __ZN3RBX9DataModel3getENS_9ContentIdE
// type: void __fastcall(boost::detail::sp_counted_base *, int, const std::string *, int)
pub fn stub_0x41951c() -> ! {
    todo!("0x41951c __ZN3RBX9DataModel3getENS_9ContentIdE")
}

#[doc(alias = "RBX::DataModel::loadWorld(int)")]
// 0x419894 — __ZN3RBX9DataModel9loadWorldEi
// type: void __fastcall(RBX::DataModel *this, int, bool)
pub fn stub_0x419894() -> ! {
    todo!("0x419894 __ZN3RBX9DataModel9loadWorldEi")
}

#[doc(alias = "RBX::DataModel::loadGame(int)")]
// 0x419a60 — __ZN3RBX9DataModel8loadGameEi
// type: void __fastcall(RBX::DataModel *this, int, bool)
pub fn stub_0x419a60() -> ! {
    todo!("0x419a60 __ZN3RBX9DataModel8loadGameEi")
}

#[doc(alias = "RBX::DataModel::loadContent(RBX::ContentId)")]
// 0x419be0 — __ZN3RBX9DataModel11loadContentENS_9ContentIdE
// type: void __fastcall(int, const std::string *)
pub fn stub_0x419be0() -> ! {
    todo!("0x419be0 __ZN3RBX9DataModel11loadContentENS_9ContentIdE")
}

#[doc(alias = "RBX::DataModel::save(RBX::ContentId)")]
// 0x419e98 — __ZN3RBX9DataModel4saveENS_9ContentIdE
// type: void __fastcall(RBX::DataModel *, const std::string *)
pub fn stub_0x419e98() -> ! {
    todo!("0x419e98 __ZN3RBX9DataModel4saveENS_9ContentIdE")
}

#[doc(alias = "RBX::DataModel::setRemoteBuildMode(bool)")]
// 0x419fd8 — __ZN3RBX9DataModel18setRemoteBuildModeEb
// type: int __fastcall(int this, bool)
pub fn stub_0x419fd8() -> ! {
    todo!("0x419fd8 __ZN3RBX9DataModel18setRemoteBuildModeEb")
}

#[doc(alias = "RBX::DataModel::getRemoteBuildMode(void)")]
// 0x419fe0 — __ZN3RBX9DataModel18getRemoteBuildModeEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x419fe0() -> ! {
    todo!("0x419fe0 __ZN3RBX9DataModel18getRemoteBuildModeEv")
}

#[doc(alias = "RBX::DataModel::setServerSaveUrl(std::string)")]
// 0x419fe8 — __ZN3RBX9DataModel16setServerSaveUrlESs
// type: int __fastcall(int)
pub fn stub_0x419fe8() -> ! {
    todo!("0x419fe8 __ZN3RBX9DataModel16setServerSaveUrlESs")
}

#[doc(alias = "RBX::DataModel::httpGet(std::string,bool)")]
// 0x41a430 — __ZN3RBX9DataModel7httpGetESsb
// type: void __fastcall(RBX::DataModel *, int, const std::string *, int)
pub fn stub_0x41a430() -> ! {
    todo!("0x41a430 __ZN3RBX9DataModel7httpGetESsb")
}

#[doc(alias = "RBX::DataModel::httpPost(std::string,std::string,bool)")]
// 0x41a7a0 — __ZN3RBX9DataModel8httpPostESsSsb
// type: void __fastcall(RBX::DataModel *, int, const std::string *, const std::string *, int)
pub fn stub_0x41a7a0() -> ! {
    todo!("0x41a7a0 __ZN3RBX9DataModel8httpPostESsSsb")
}

#[doc(alias = "RBX::DataModel::getJobsInfo(void)")]
// 0x41abf8 — __ZN3RBX9DataModel11getJobsInfoEv
// type: int __fastcall(RBX::DataModel *this, int)
pub fn stub_0x41abf8() -> ! {
    todo!("0x41abf8 __ZN3RBX9DataModel11getJobsInfoEv")
}

#[doc(alias = "RBX::DataModel::reportMeasurement(std::string,std::string,std::string,std::string,std::string)")]
// 0x41b47c — __ZN3RBX9DataModel17reportMeasurementESsSsSsSsSs
// type: int __fastcall(int, const std::string *, const std::string *, const std::string *, const std::string *, const std::string *)
pub fn stub_0x41b47c() -> ! {
    todo!("0x41b47c __ZN3RBX9DataModel17reportMeasurementESsSsSsSsSs")
}

#[doc(alias = "RBX::DataModel::clearContents(bool)")]
// 0x41b498 — __ZN3RBX9DataModel13clearContentsEb
// type: void __fastcall(RBX::DataModel *this, int, int, int)
pub fn stub_0x41b498() -> ! {
    todo!("0x41b498 __ZN3RBX9DataModel13clearContentsEb")
}

#[doc(alias = "RBX::DataModel::close(void)")]
// 0x41bac0 — __ZN3RBX9DataModel5closeEv
// type: void __fastcall(RBX::DataModel *this)
pub fn stub_0x41bac0() -> ! {
    todo!("0x41bac0 __ZN3RBX9DataModel5closeEv")
}

#[doc(alias = "RBX::DataModel::toggleToolsOff(void)")]
// 0x41bcbc — __ZN3RBX9DataModel14toggleToolsOffEv
// type: void __fastcall(RBX::DataModel *this)
pub fn stub_0x41bcbc() -> ! {
    todo!("0x41bcbc __ZN3RBX9DataModel14toggleToolsOffEv")
}

#[doc(alias = "RBX::DataModel::canSaveLocal(void)const")]
// 0x41be10 — __ZNK3RBX9DataModel12canSaveLocalEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x41be10() -> ! {
    todo!("0x41be10 __ZNK3RBX9DataModel12canSaveLocalEv")
}

#[doc(alias = "RBX::DataModel::completeShutdown(bool)")]
// 0x41c07c — __ZN3RBX9DataModel16completeShutdownEb
// type: void __fastcall(RBX::DataModel *this, int)
pub fn stub_0x41c07c() -> ! {
    todo!("0x41c07c __ZN3RBX9DataModel16completeShutdownEb")
}

#[doc(alias = "RBX::DataModel::setUiMessage(std::string)")]
// 0x41c284 — __ZN3RBX9DataModel12setUiMessageESs
// type: int __fastcall(int)
pub fn stub_0x41c284() -> ! {
    todo!("0x41c284 __ZN3RBX9DataModel12setUiMessageESs")
}

#[doc(alias = "RBX::DataModel::clearUiMessage(void)")]
// 0x41c28c — __ZN3RBX9DataModel14clearUiMessageEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x41c28c() -> ! {
    todo!("0x41c28c __ZN3RBX9DataModel14clearUiMessageEv")
}

#[doc(alias = "RBX::DataModel::getJobsExtendedStats(void)")]
// 0x41c2a0 — __ZN3RBX9DataModel20getJobsExtendedStatsEv
// type: int __fastcall(RBX::DataModel *this, int)
pub fn stub_0x41c2a0() -> ! {
    todo!("0x41c2a0 __ZN3RBX9DataModel20getJobsExtendedStatsEv")
}

#[doc(alias = "RBX::DataModel::getJobTimePeakFraction(std::string,double)")]
// 0x41cd40 — __ZN3RBX9DataModel22getJobTimePeakFractionESsd
// type: __int64 __fastcall(RBX::TaskScheduler *, const std::string *, __int64)
pub fn stub_0x41cd40() -> ! {
    todo!("0x41cd40 __ZN3RBX9DataModel22getJobTimePeakFractionESsd")
}

#[doc(alias = "RBX::DataModel::getJobIntervalPeakFraction(std::string,double)")]
// 0x41cf9c — __ZN3RBX9DataModel26getJobIntervalPeakFractionESsd
// type: __int64 __fastcall(RBX::TaskScheduler *, const std::string *, __int64)
pub fn stub_0x41cf9c() -> ! {
    todo!("0x41cf9c __ZN3RBX9DataModel26getJobIntervalPeakFractionESsd")
}

#[doc(alias = "RBX::DataModel::setJobsExtendedStatsWindow(double)")]
// 0x41d1f8 — __ZN3RBX9DataModel26setJobsExtendedStatsWindowEd
// type: int __fastcall(RBX::DataModel *this, double)
pub fn stub_0x41d1f8() -> ! {
    todo!("0x41d1f8 __ZN3RBX9DataModel26setJobsExtendedStatsWindowEd")
}

#[doc(alias = "RBX::DataModel::setPlaceVersion(int)")]
// 0x41d210 — __ZN3RBX9DataModel15setPlaceVersionEi
// type: char *__fastcall(RBX::DataModel *this, char *, int, const void *)
pub fn stub_0x41d210() -> ! {
    todo!("0x41d210 __ZN3RBX9DataModel15setPlaceVersionEi")
}

#[doc(alias = "RBX::DataModel::setPlaceID(int,bool)")]
// 0x41d260 — __ZN3RBX9DataModel10setPlaceIDEib
// type: _DWORD __fastcall(RBX::DataModel *__hidden this, char *, bool)
pub fn stub_0x41d260() -> ! {
    todo!("0x41d260 __ZN3RBX9DataModel10setPlaceIDEib")
}

#[doc(alias = "RBX::DataModel::activateExperimentalFeatures(void)")]
// 0x41d2c8 — __ZN3RBX9DataModel28activateExperimentalFeaturesEv
// type: int __fastcall(int this)
pub fn stub_0x41d2c8() -> ! {
    todo!("0x41d2c8 __ZN3RBX9DataModel28activateExperimentalFeaturesEv")
}

#[doc(alias = "RBX::DataModel::setCreatorID(int,RBX::DataModel::CreatorType)")]
// 0x41d2d0 — __ZN3RBX9DataModel12setCreatorIDEiNS0_11CreatorTypeE
// type: int __fastcall(RBX::Instance *, int, int)
pub fn stub_0x41d2d0() -> ! {
    todo!("0x41d2d0 __ZN3RBX9DataModel12setCreatorIDEiNS0_11CreatorTypeE")
}

#[doc(alias = "RBX::DataModel::setGenre(RBX::DataModel::Genre)")]
// 0x41d320 — __ZN3RBX9DataModel8setGenreENS0_5GenreE
// type: RBX::Instance *__fastcall(RBX::Instance *result, int)
pub fn stub_0x41d320() -> ! {
    todo!("0x41d320 __ZN3RBX9DataModel8setGenreENS0_5GenreE")
}

#[doc(alias = "RBX::DataModel::setGear(RBX::DataModel::GearGenreSetting,int)")]
// 0x41d340 — __ZN3RBX9DataModel7setGearENS0_16GearGenreSettingEi
// type: int __fastcall(RBX::Instance *, int, int)
pub fn stub_0x41d340() -> ! {
    todo!("0x41d340 __ZN3RBX9DataModel7setGearENS0_16GearGenreSettingEi")
}

#[doc(alias = "RBX::DataModel::getLightingDeprecated(void)const")]
// 0x41d384 — __ZNK3RBX9DataModel21getLightingDeprecatedEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x41d384() -> ! {
    todo!("0x41d384 __ZNK3RBX9DataModel21getLightingDeprecatedEv")
}

#[doc(alias = "RBX::DataModel::isGearTypeAllowed(RBX::DataModel::GearType)")]
// 0x41d390 — __ZN3RBX9DataModel17isGearTypeAllowedENS0_8GearTypeE
// type: bool __fastcall(int, char)
pub fn stub_0x41d390() -> ! {
    todo!("0x41d390 __ZN3RBX9DataModel17isGearTypeAllowedENS0_8GearTypeE")
}

#[doc(alias = "RBX::DataModel::setScreenshotSEOInfo(std::string)")]
// 0x41d3a4 — __ZN3RBX9DataModel20setScreenshotSEOInfoESs
// type: int __fastcall(int)
pub fn stub_0x41d3a4() -> ! {
    todo!("0x41d3a4 __ZN3RBX9DataModel20setScreenshotSEOInfoESs")
}

#[doc(alias = "RBX::DataModel::setVideoSEOInfo(std::string)")]
// 0x41d3ac — __ZN3RBX9DataModel15setVideoSEOInfoESs
// type: int __fastcall(int)
pub fn stub_0x41d3ac() -> ! {
    todo!("0x41d3ac __ZN3RBX9DataModel15setVideoSEOInfoESs")
}

#[doc(alias = "RBX::DataModel::addCustomStat(std::string,std::string)")]
// 0x41d3b4 — __ZN3RBX9DataModel13addCustomStatESsSs
// type: int __fastcall(int, const std::string *, const std::string *)
pub fn stub_0x41d3b4() -> ! {
    todo!("0x41d3b4 __ZN3RBX9DataModel13addCustomStatESsSs")
}

#[doc(alias = "RBX::DataModel::removeCustomStat(std::string)")]
// 0x41d3bc — __ZN3RBX9DataModel16removeCustomStatESs
// type: int __fastcall(int, const std::string *)
pub fn stub_0x41d3bc() -> ! {
    todo!("0x41d3bc __ZN3RBX9DataModel16removeCustomStatESs")
}

#[doc(alias = "RBX::DataModel::writeStatsSettings(void)")]
// 0x41d3c4 — __ZN3RBX9DataModel18writeStatsSettingsEv
// type: int __fastcall(RBX::DataModel *this)
pub fn stub_0x41d3c4() -> ! {
    todo!("0x41d3c4 __ZN3RBX9DataModel18writeStatsSettingsEv")
}

#[doc(alias = "RBX::StringConverter<RBX::DataModel::CreatorType>::convertToValue(std::string const&,RBX::DataModel::CreatorType&)")]
// 0x41de60 — __ZN3RBX15StringConverterINS_9DataModel11CreatorTypeEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x41de60() -> ! {
    todo!("0x41de60 __ZN3RBX15StringConverterINS_9DataModel11CreatorTypeEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::StringConverter<RBX::DataModel::Genre>::convertToValue(std::string const&,RBX::DataModel::Genre&)")]
// 0x41deac — __ZN3RBX15StringConverterINS_9DataModel5GenreEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x41deac() -> ! {
    todo!("0x41deac __ZN3RBX15StringConverterINS_9DataModel5GenreEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::StringConverter<RBX::DataModel::GearGenreSetting>::convertToValue(std::string const&,RBX::DataModel::GearGenreSetting&)")]
// 0x41def8 — __ZN3RBX15StringConverterINS_9DataModel16GearGenreSettingEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x41def8() -> ! {
    todo!("0x41def8 __ZN3RBX15StringConverterINS_9DataModel16GearGenreSettingEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::StringConverter<RBX::DataModel::GearType>::convertToValue(std::string const&,RBX::DataModel::GearType&)")]
// 0x41df44 — __ZN3RBX15StringConverterINS_9DataModel8GearTypeEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x41df44() -> ! {
    todo!("0x41df44 __ZN3RBX15StringConverterINS_9DataModel8GearTypeEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::DataModel::getSyncronizationArbiter(void)")]
// 0x41e84c — __ZN3RBX9DataModel24getSyncronizationArbiterEv
// type: RBX::DataModel *__fastcall(RBX::DataModel *this)
pub fn stub_0x41e84c() -> ! {
    todo!("0x41e84c __ZN3RBX9DataModel24getSyncronizationArbiterEv")
}

#[doc(alias = "non-virtual thunk toRBX::DataModel::getSyncronizationArbiter(void)")]
// 0x41e860 — __ZThn184_N3RBX9DataModel24getSyncronizationArbiterEv
// type: char *__fastcall(RBX::DataModel *this)
pub fn stub_0x41e860() -> ! {
    todo!("0x41e860 __ZThn184_N3RBX9DataModel24getSyncronizationArbiterEv")
}

#[doc(alias = "RBX::DataModel::initializeContents(bool)")]
// 0x41ede4 — __ZN3RBX9DataModel18initializeContentsEb
// type: void __fastcall(RBX::DataModel *this, boost::detail::sp_counted_base *)
pub fn stub_0x41ede4() -> ! {
    todo!("0x41ede4 __ZN3RBX9DataModel18initializeContentsEb")
}

#[doc(alias = "RBX::DataModel::LegacyLock::~LegacyLock()")]
// 0x41f220 — __ZN3RBX9DataModel10LegacyLockD1Ev
// type: void __fastcall(RBX::DataModel::LegacyLock *__hidden this)
pub fn stub_0x41f220() -> ! {
    todo!("0x41f220 __ZN3RBX9DataModel10LegacyLockD1Ev")
}

#[doc(alias = "RBX::DataModel::createDataModel(bool,RBX::Verb *,RBX::DataModel*)")]
// 0x41f230 — __ZN3RBX9DataModel15createDataModelEbPNS_4VerbEPS0_
// type: void __fastcall(RBX::DataModel *this, const char *, RBX::Verb *, RBX::DataModel *)
pub fn stub_0x41f230() -> ! {
    todo!("0x41f230 __ZN3RBX9DataModel15createDataModelEbPNS_4VerbEPS0_")
}
