//! core shard BB — 100 core stubs EA-sorted, next uncovered after BA 0x40824c (strict RBX|boost|std earliest gap, after BA 0x404360..0x40824c).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x40824c.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x408518 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x408518() -> ! {
    todo!("0x408518 __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x4085b8 — __ZNK3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x4085b8() -> ! {
    todo!("0x4085b8 __ZNK3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x4085f0 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x4085f0() -> ! {
    todo!("0x4085f0 __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x408704 — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x408704() -> ! {
    todo!("0x408704 __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::FillTool::isSticky(void)const")]
// 0x4088ac — __ZNK3RBX8FillTool8isStickyEv — RBX::FillTool::isSticky(void)const
pub fn stub_0x4088ac() -> ! {
    todo!("0x4088ac __ZNK3RBX8FillTool8isStickyEv")
}

#[doc(alias = "RBX::FillTool::getCursorName(void)const")]
// 0x408974 — __ZNK3RBX8FillTool13getCursorNameEv — RBX::FillTool::getCursorName(void)const
pub fn stub_0x408974() -> ! {
    todo!("0x408974 __ZNK3RBX8FillTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x408f18 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x408f18() -> ! {
    todo!("0x408f18 __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x408fb8 — __ZNK3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x408fb8() -> ! {
    todo!("0x408fb8 __ZNK3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x408ff0 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x408ff0() -> ! {
    todo!("0x408ff0 __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x409104 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x409104() -> ! {
    todo!("0x409104 __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::LockTool::onMouseUp(RBX::UIEvent const&)")]
// 0x4092ac — __ZN3RBX8LockTool9onMouseUpERKNS_7UIEventE — RBX::LockTool::onMouseUp(RBX::UIEvent const&)
pub fn stub_0x4092ac() -> ! {
    todo!("0x4092ac __ZN3RBX8LockTool9onMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x409a64 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x409a64() -> ! {
    todo!("0x409a64 __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x409b04 — __ZNK3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x409b04() -> ! {
    todo!("0x409b04 __ZNK3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x409b3c — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x409b3c() -> ! {
    todo!("0x409b3c __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x409c50 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x409c50() -> ! {
    todo!("0x409c50 __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::AnchorTool::onMouseUp(RBX::UIEvent const&)")]
// 0x409e00 — __ZN3RBX10AnchorTool9onMouseUpERKNS_7UIEventE — RBX::AnchorTool::onMouseUp(RBX::UIEvent const&)
pub fn stub_0x409e00() -> ! {
    todo!("0x409e00 __ZN3RBX10AnchorTool9onMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40a5b8 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40a5b8() -> ! {
    todo!("0x40a5b8 __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40a658 — __ZNK3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40a658() -> ! {
    todo!("0x40a658 __ZNK3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40a690 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40a690() -> ! {
    todo!("0x40a690 __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40a7a4 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40a7a4() -> ! {
    todo!("0x40a7a4 __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::SmoothNoOutlinesTool::isSticky(void)const")]
// 0x40a94c — __ZNK3RBX20SmoothNoOutlinesTool8isStickyEv — RBX::SmoothNoOutlinesTool::isSticky(void)const
pub fn stub_0x40a94c() -> ! {
    todo!("0x40a94c __ZNK3RBX20SmoothNoOutlinesTool8isStickyEv")
}

#[doc(alias = "RBX::SmoothNoOutlinesTool::getCursorName(void)const")]
// 0x40aa14 — __ZNK3RBX20SmoothNoOutlinesTool13getCursorNameEv — RBX::SmoothNoOutlinesTool::getCursorName(void)const
pub fn stub_0x40aa14() -> ! {
    todo!("0x40aa14 __ZNK3RBX20SmoothNoOutlinesTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40afb8 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40afb8() -> ! {
    todo!("0x40afb8 __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40b058 — __ZNK3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40b058() -> ! {
    todo!("0x40b058 __ZNK3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40b090 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40b090() -> ! {
    todo!("0x40b090 __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40b1a4 — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40b1a4() -> ! {
    todo!("0x40b1a4 __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::OscillateMotorTool::getCursorName(void)const")]
// 0x40b34c — __ZNK3RBX18OscillateMotorTool13getCursorNameEv — RBX::OscillateMotorTool::getCursorName(void)const
pub fn stub_0x40b34c() -> ! {
    todo!("0x40b34c __ZNK3RBX18OscillateMotorTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40b8f0 — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40b8f0() -> ! {
    todo!("0x40b8f0 __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40b990 — __ZNK3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40b990() -> ! {
    todo!("0x40b990 __ZNK3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40b9c8 — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40b9c8() -> ! {
    todo!("0x40b9c8 __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40badc — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40badc() -> ! {
    todo!("0x40badc __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::LeftMotorTool::getCursorName(void)const")]
// 0x40bc84 — __ZNK3RBX13LeftMotorTool13getCursorNameEv — RBX::LeftMotorTool::getCursorName(void)const
pub fn stub_0x40bc84() -> ! {
    todo!("0x40bc84 __ZNK3RBX13LeftMotorTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40c228 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40c228() -> ! {
    todo!("0x40c228 __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40c2c8 — __ZNK3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40c2c8() -> ! {
    todo!("0x40c2c8 __ZNK3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40c300 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40c300() -> ! {
    todo!("0x40c300 __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40c414 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40c414() -> ! {
    todo!("0x40c414 __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::RightMotorTool::isSticky(void)const")]
// 0x40c5bc — __ZNK3RBX14RightMotorTool8isStickyEv — RBX::RightMotorTool::isSticky(void)const
pub fn stub_0x40c5bc() -> ! {
    todo!("0x40c5bc __ZNK3RBX14RightMotorTool8isStickyEv")
}

#[doc(alias = "RBX::RightMotorTool::getCursorName(void)const")]
// 0x40c684 — __ZNK3RBX14RightMotorTool13getCursorNameEv — RBX::RightMotorTool::getCursorName(void)const
pub fn stub_0x40c684() -> ! {
    todo!("0x40c684 __ZNK3RBX14RightMotorTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40cc28 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40cc28() -> ! {
    todo!("0x40cc28 __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40ccc8 — __ZNK3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40ccc8() -> ! {
    todo!("0x40ccc8 __ZNK3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40cd00 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40cd00() -> ! {
    todo!("0x40cd00 __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40ce14 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40ce14() -> ! {
    todo!("0x40ce14 __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::HingeTool::isSticky(void)const")]
// 0x40cfbc — __ZNK3RBX9HingeTool8isStickyEv — RBX::HingeTool::isSticky(void)const
pub fn stub_0x40cfbc() -> ! {
    todo!("0x40cfbc __ZNK3RBX9HingeTool8isStickyEv")
}

#[doc(alias = "RBX::HingeTool::getCursorName(void)const")]
// 0x40d084 — __ZNK3RBX9HingeTool13getCursorNameEv — RBX::HingeTool::getCursorName(void)const
pub fn stub_0x40d084() -> ! {
    todo!("0x40d084 __ZNK3RBX9HingeTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40d628 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40d628() -> ! {
    todo!("0x40d628 __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40d6c8 — __ZNK3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40d6c8() -> ! {
    todo!("0x40d6c8 __ZNK3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40d700 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40d700() -> ! {
    todo!("0x40d700 __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40d814 — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40d814() -> ! {
    todo!("0x40d814 __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::UniversalTool::isSticky(void)const")]
// 0x40d9bc — __ZNK3RBX13UniversalTool8isStickyEv — RBX::UniversalTool::isSticky(void)const
pub fn stub_0x40d9bc() -> ! {
    todo!("0x40d9bc __ZNK3RBX13UniversalTool8isStickyEv")
}

#[doc(alias = "RBX::UniversalTool::getCursorName(void)const")]
// 0x40da84 — __ZNK3RBX13UniversalTool13getCursorNameEv — RBX::UniversalTool::getCursorName(void)const
pub fn stub_0x40da84() -> ! {
    todo!("0x40da84 __ZNK3RBX13UniversalTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40e028 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40e028() -> ! {
    todo!("0x40e028 __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40e0c8 — __ZNK3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40e0c8() -> ! {
    todo!("0x40e0c8 __ZNK3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40e100 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40e100() -> ! {
    todo!("0x40e100 __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40e214 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40e214() -> ! {
    todo!("0x40e214 __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::InletTool::isSticky(void)const")]
// 0x40e3bc — __ZNK3RBX9InletTool8isStickyEv — RBX::InletTool::isSticky(void)const
pub fn stub_0x40e3bc() -> ! {
    todo!("0x40e3bc __ZNK3RBX9InletTool8isStickyEv")
}

#[doc(alias = "RBX::InletTool::getCursorName(void)const")]
// 0x40e484 — __ZNK3RBX9InletTool13getCursorNameEv — RBX::InletTool::getCursorName(void)const
pub fn stub_0x40e484() -> ! {
    todo!("0x40e484 __ZNK3RBX9InletTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40ea28 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40ea28() -> ! {
    todo!("0x40ea28 __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40eac8 — __ZNK3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40eac8() -> ! {
    todo!("0x40eac8 __ZNK3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40eb00 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40eb00() -> ! {
    todo!("0x40eb00 __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40ec14 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40ec14() -> ! {
    todo!("0x40ec14 __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::StudsTool::isSticky(void)const")]
// 0x40edbc — __ZNK3RBX9StudsTool8isStickyEv — RBX::StudsTool::isSticky(void)const
pub fn stub_0x40edbc() -> ! {
    todo!("0x40edbc __ZNK3RBX9StudsTool8isStickyEv")
}

#[doc(alias = "RBX::StudsTool::getCursorName(void)const")]
// 0x40ee84 — __ZNK3RBX9StudsTool13getCursorNameEv — RBX::StudsTool::getCursorName(void)const
pub fn stub_0x40ee84() -> ! {
    todo!("0x40ee84 __ZNK3RBX9StudsTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40f428 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40f428() -> ! {
    todo!("0x40f428 __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40f4c8 — __ZNK3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40f4c8() -> ! {
    todo!("0x40f4c8 __ZNK3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40f500 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40f500() -> ! {
    todo!("0x40f500 __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x40f614 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x40f614() -> ! {
    todo!("0x40f614 __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::WeldTool::isSticky(void)const")]
// 0x40f7bc — __ZNK3RBX8WeldTool8isStickyEv — RBX::WeldTool::isSticky(void)const
pub fn stub_0x40f7bc() -> ! {
    todo!("0x40f7bc __ZNK3RBX8WeldTool8isStickyEv")
}

#[doc(alias = "RBX::WeldTool::getCursorName(void)const")]
// 0x40f884 — __ZNK3RBX8WeldTool13getCursorNameEv — RBX::WeldTool::getCursorName(void)const
pub fn stub_0x40f884() -> ! {
    todo!("0x40f884 __ZNK3RBX8WeldTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x40fe28 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x40fe28() -> ! {
    todo!("0x40fe28 __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x40fec8 — __ZNK3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x40fec8() -> ! {
    todo!("0x40fec8 __ZNK3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x40ff00 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x40ff00() -> ! {
    todo!("0x40ff00 __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x410014 — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x410014() -> ! {
    todo!("0x410014 __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::GlueTool::isSticky(void)const")]
// 0x4101bc — __ZNK3RBX8GlueTool8isStickyEv — RBX::GlueTool::isSticky(void)const
pub fn stub_0x4101bc() -> ! {
    todo!("0x4101bc __ZNK3RBX8GlueTool8isStickyEv")
}

#[doc(alias = "RBX::GlueTool::getCursorName(void)const")]
// 0x410284 — __ZNK3RBX8GlueTool13getCursorNameEv — RBX::GlueTool::getCursorName(void)const
pub fn stub_0x410284() -> ! {
    todo!("0x410284 __ZNK3RBX8GlueTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x410828 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x410828() -> ! {
    todo!("0x410828 __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x4108c8 — __ZNK3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x4108c8() -> ! {
    todo!("0x4108c8 __ZNK3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x410900 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x410900() -> ! {
    todo!("0x410900 __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x410a14 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x410a14() -> ! {
    todo!("0x410a14 __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::FlatTool::isSticky(void)const")]
// 0x410bbc — __ZNK3RBX8FlatTool8isStickyEv — RBX::FlatTool::isSticky(void)const
pub fn stub_0x410bbc() -> ! {
    todo!("0x410bbc __ZNK3RBX8FlatTool8isStickyEv")
}

#[doc(alias = "RBX::FlatTool::getCursorName(void)const")]
// 0x410c84 — __ZNK3RBX8FlatTool13getCursorNameEv — RBX::FlatTool::getCursorName(void)const
pub fn stub_0x410c84() -> ! {
    todo!("0x410c84 __ZNK3RBX8FlatTool13getCursorNameEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x411228 — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x411228() -> ! {
    todo!("0x411228 __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x4112c8 — __ZNK3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x4112c8() -> ! {
    todo!("0x4112c8 __ZNK3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x4112fc — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x4112fc() -> ! {
    todo!("0x4112fc __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x411410 — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x411410() -> ! {
    todo!("0x411410 __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::~TToolVerb()")]
// 0x411660 — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEED0Ev — RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::~TToolVerb()
pub fn stub_0x411660() -> ! {
    todo!("0x411660 __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEED0Ev")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::isChecked(void)const")]
// 0x411700 — __ZNK3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE9isCheckedEv — RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::isChecked(void)const
pub fn stub_0x411700() -> ! {
    todo!("0x411700 __ZNK3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE9isCheckedEv")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// 0x411738 — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE — RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)
pub fn stub_0x411738() -> ! {
    todo!("0x411738 __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// 0x41184c — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE15newMouseCommandEv — RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::newMouseCommand(void)
pub fn stub_0x41184c() -> ! {
    todo!("0x41184c __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEE15newMouseCommandEv")
}

#[doc(alias = "RBX::AdvRotateTool::isSticky(void)const")]
// 0x411a00 — __ZNK3RBX13AdvRotateTool8isStickyEv — RBX::AdvRotateTool::isSticky(void)const
pub fn stub_0x411a00() -> ! {
    todo!("0x411a00 __ZNK3RBX13AdvRotateTool8isStickyEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::drawConnectors(void)const")]
// 0x411ac8 — __ZNK3RBX15AdvMoveToolBase14drawConnectorsEv — RBX::AdvMoveToolBase::drawConnectors(void)const
pub fn stub_0x411ac8() -> ! {
    todo!("0x411ac8 __ZNK3RBX15AdvMoveToolBase14drawConnectorsEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::getCursorName(void)const")]
// 0x411acc — __ZNK3RBX15AdvMoveToolBase13getCursorNameEv — RBX::AdvMoveToolBase::getCursorName(void)const
pub fn stub_0x411acc() -> ! {
    todo!("0x411acc __ZNK3RBX15AdvMoveToolBase13getCursorNameEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::setCursor(std::string)")]
// 0x411ad8 — __ZN3RBX15AdvMoveToolBase9setCursorESs — RBX::AdvMoveToolBase::setCursor(std::string)
pub fn stub_0x411ad8() -> ! {
    todo!("0x411ad8 __ZN3RBX15AdvMoveToolBase9setCursorESs")
}

#[doc(alias = "RBX::AdvRotateTool::getHandleColor(void)const")]
// 0x411ae0 — __ZNK3RBX13AdvRotateTool14getHandleColorEv — RBX::AdvRotateTool::getHandleColor(void)const
pub fn stub_0x411ae0() -> ! {
    todo!("0x411ae0 __ZNK3RBX13AdvRotateTool14getHandleColorEv")
}

#[doc(alias = "RBX::AdvRotateTool::getDragType(void)const")]
// 0x411af8 — __ZNK3RBX13AdvRotateTool11getDragTypeEv — RBX::AdvRotateTool::getDragType(void)const
pub fn stub_0x411af8() -> ! {
    todo!("0x411af8 __ZNK3RBX13AdvRotateTool11getDragTypeEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// 0x411afc — __ZN3RBX15AdvMoveToolBaseD2Ev — RBX::AdvMoveToolBase::~AdvMoveToolBase()
pub fn stub_0x411afc() -> ! {
    todo!("0x411afc __ZN3RBX15AdvMoveToolBaseD2Ev")
}

#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// 0x411c14 — __ZN3RBX15AdvMoveToolBaseD1Ev — RBX::AdvMoveToolBase::~AdvMoveToolBase()
pub fn stub_0x411c14() -> ! {
    todo!("0x411c14 __ZN3RBX15AdvMoveToolBaseD1Ev")
}

#[doc(alias = "RBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// 0x411c18 — __ZN3RBX15AdvMoveToolBaseD0Ev — RBX::AdvMoveToolBase::~AdvMoveToolBase()
pub fn stub_0x411c18() -> ! {
    todo!("0x411c18 __ZN3RBX15AdvMoveToolBaseD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// 0x411cb8 — __ZThn36_N3RBX15AdvMoveToolBaseD1Ev — non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()
// was: non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()
pub fn stub_0x411cb8() -> ! {
    todo!("0x411cb8 __ZThn36_N3RBX15AdvMoveToolBaseD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()")]
// 0x411cc0 — __ZThn36_N3RBX15AdvMoveToolBaseD0Ev — non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()
// was: non-virtual thunk toRBX::AdvMoveToolBase::~AdvMoveToolBase()
pub fn stub_0x411cc0() -> ! {
    todo!("0x411cc0 __ZThn36_N3RBX15AdvMoveToolBaseD0Ev")
}

#[doc(alias = "std::auto_ptr<RBX::MegaDragger>::~auto_ptr()")]
// 0x411cc8 — __ZNSt8auto_ptrIN3RBX11MegaDraggerEED2Ev — std::auto_ptr<RBX::MegaDragger>::~auto_ptr()
pub fn stub_0x411cc8() -> ! {
    todo!("0x411cc8 __ZNSt8auto_ptrIN3RBX11MegaDraggerEED2Ev")
}
