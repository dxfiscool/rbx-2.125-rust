//! rendering shard 291 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 31640->31740 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31640 before -> 31740 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x403278

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x40336c — __ZN5boost3_bi5list2INS0_5valueIPKcEENS_3argILi1EEEEclIbPFbS4_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRKSD_EEEET_NS0_4typeISK_EERT0_RT1_l
// type: int __fastcall(int *, int (__fastcall **)(int, sp_counted_base **), const shared_count **)
#[doc(alias = "bool boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>::operator()<bool,bool (*)(char const*,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<bool>,bool (*)(char const*,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,long)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPKcEENS_3argILi1EEEEclIbPFbS4_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRKSD_EEEET_NS0_4typeISK_EERT0_RT1_l
pub fn stub_40336c() -> ! {
    todo!("0x40336c bool boost::_bi::list2<boost::_bi::value<char const*>,boost::arg<1>>::operator()<bool,bool (*)(char const*,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<bool>,bool (*)(char const*,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,long)")
}

// 0x403444 — __GLOBAL__I_a_173
#[doc(alias = "global constructor keyed to _a_173")]
// was: __GLOBAL__I_a_173
pub fn stub_403444() -> ! {
    todo!("0x403444 global constructor keyed to _a_173")
}

// 0x403820 — __ZN3RBX11CommonVerbsC1EPNS_9DataModelE
// type: int __fastcall(RBX::CommonVerbs *this, RBX::DataModel *)
#[doc(alias = "RBX::CommonVerbs::CommonVerbs(RBX::DataModel *)")]
// was: __ZN3RBX11CommonVerbsC1EPNS_9DataModelE
pub fn stub_403820() -> ! {
    todo!("0x403820 RBX::CommonVerbs::CommonVerbs(RBX::DataModel *)")
}

// 0x403824 — __ZN3RBX11CommonVerbsC2EPNS_9DataModelE
// type: RBX::CommonVerbs *__fastcall(RBX::CommonVerbs *this, RBX::DataModel *)
#[doc(alias = "RBX::CommonVerbs::CommonVerbs(RBX::DataModel *)")]
// was: __ZN3RBX11CommonVerbsC2EPNS_9DataModelE
pub fn stub_403824() -> ! {
    todo!("0x403824 RBX::CommonVerbs::CommonVerbs(RBX::DataModel *)")
}

// 0x404314 — __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEED1Ev
pub fn stub_404314() -> ! {
    todo!("0x404314 RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404318 — __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEED1Ev
pub fn stub_404318() -> ! {
    todo!("0x404318 RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40431c — __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEED1Ev
pub fn stub_40431c() -> ! {
    todo!("0x40431c RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404320 — __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8NullToolENS_12RunStateVerbEED1Ev
pub fn stub_404320() -> ! {
    todo!("0x404320 RBX::TToolVerb<RBX::NullTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404324 — __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_11DropperToolENS_12RunStateVerbEED1Ev
pub fn stub_404324() -> ! {
    todo!("0x404324 RBX::TToolVerb<RBX::DropperTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404328 — __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_12MaterialToolENS_12RunStateVerbEED1Ev
pub fn stub_404328() -> ! {
    todo!("0x404328 RBX::TToolVerb<RBX::MaterialTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40432c — __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8FillToolENS_12RunStateVerbEED1Ev
pub fn stub_40432c() -> ! {
    todo!("0x40432c RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404330 — __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8LockToolENS_12RunStateVerbEED1Ev
pub fn stub_404330() -> ! {
    todo!("0x404330 RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404334 — __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_10AnchorToolENS_12RunStateVerbEED1Ev
pub fn stub_404334() -> ! {
    todo!("0x404334 RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404338 — __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_20SmoothNoOutlinesToolENS_12RunStateVerbEED1Ev
pub fn stub_404338() -> ! {
    todo!("0x404338 RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40433c — __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_18OscillateMotorToolENS_12RunStateVerbEED1Ev
pub fn stub_40433c() -> ! {
    todo!("0x40433c RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404340 — __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_13LeftMotorToolENS_12RunStateVerbEED1Ev
pub fn stub_404340() -> ! {
    todo!("0x404340 RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404344 — __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_14RightMotorToolENS_12RunStateVerbEED1Ev
pub fn stub_404344() -> ! {
    todo!("0x404344 RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404348 — __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9HingeToolENS_12RunStateVerbEED1Ev
pub fn stub_404348() -> ! {
    todo!("0x404348 RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40434c — __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_13UniversalToolENS_12RunStateVerbEED1Ev
pub fn stub_40434c() -> ! {
    todo!("0x40434c RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404350 — __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9InletToolENS_12RunStateVerbEED1Ev
pub fn stub_404350() -> ! {
    todo!("0x404350 RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404354 — __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9StudsToolENS_12RunStateVerbEED1Ev
pub fn stub_404354() -> ! {
    todo!("0x404354 RBX::TToolVerb<RBX::StudsTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404358 — __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8WeldToolENS_12RunStateVerbEED1Ev
pub fn stub_404358() -> ! {
    todo!("0x404358 RBX::TToolVerb<RBX::WeldTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40435c — __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8GlueToolENS_12RunStateVerbEED1Ev
pub fn stub_40435c() -> ! {
    todo!("0x40435c RBX::TToolVerb<RBX::GlueTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404360 — __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8FlatToolENS_12RunStateVerbEED1Ev
pub fn stub_404360() -> ! {
    todo!("0x404360 RBX::TToolVerb<RBX::FlatTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404364 — __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_18MoveResizeJoinToolENS_12RunStateVerbEED1Ev
pub fn stub_404364() -> ! {
    todo!("0x404364 RBX::TToolVerb<RBX::MoveResizeJoinTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404368 — __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_12AdvArrowToolENS_12RunStateVerbEED1Ev
pub fn stub_404368() -> ! {
    todo!("0x404368 RBX::TToolVerb<RBX::AdvArrowTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40436c — __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_13AdvRotateToolENS_12RunStateVerbEED1Ev
pub fn stub_40436c() -> ! {
    todo!("0x40436c RBX::TToolVerb<RBX::AdvRotateTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404370 — __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_11AdvMoveToolENS_12RunStateVerbEED1Ev
pub fn stub_404370() -> ! {
    todo!("0x404370 RBX::TToolVerb<RBX::AdvMoveTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404374 — __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_14AxisRotateToolENS_12RunStateVerbEED1Ev
pub fn stub_404374() -> ! {
    todo!("0x404374 RBX::TToolVerb<RBX::AxisRotateTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404378 — __ZN3RBX15MoveUpBrickVerbD1Ev
// type: void __fastcall(RBX::MoveUpBrickVerb *__hidden this)
#[doc(alias = "RBX::MoveUpBrickVerb::~MoveUpBrickVerb()")]
// was: __ZN3RBX15MoveUpBrickVerbD1Ev
pub fn stub_404378() -> ! {
    todo!("0x404378 RBX::MoveUpBrickVerb::~MoveUpBrickVerb()")
}

// 0x40437c — __ZN3RBX15MoveUpPlateVerbD1Ev
// type: void __fastcall(RBX::MoveUpPlateVerb *__hidden this)
#[doc(alias = "RBX::MoveUpPlateVerb::~MoveUpPlateVerb()")]
// was: __ZN3RBX15MoveUpPlateVerbD1Ev
pub fn stub_40437c() -> ! {
    todo!("0x40437c RBX::MoveUpPlateVerb::~MoveUpPlateVerb()")
}

// 0x404380 — __ZN3RBX14CanCollideVerbD1Ev
// type: void __fastcall(RBX::CanCollideVerb *__hidden this)
#[doc(alias = "RBX::CanCollideVerb::~CanCollideVerb()")]
// was: __ZN3RBX14CanCollideVerbD1Ev
pub fn stub_404380() -> ! {
    todo!("0x404380 RBX::CanCollideVerb::~CanCollideVerb()")
}

// 0x404384 — __ZN3RBX15TranslucentVerbD1Ev
// type: void __fastcall(RBX::TranslucentVerb *__hidden this)
#[doc(alias = "RBX::TranslucentVerb::~TranslucentVerb()")]
// was: __ZN3RBX15TranslucentVerbD1Ev
pub fn stub_404384() -> ! {
    todo!("0x404384 RBX::TranslucentVerb::~TranslucentVerb()")
}

// 0x404388 — __ZN3RBX10AnchorVerbD1Ev
// type: void __fastcall(RBX::AnchorVerb *__hidden this)
#[doc(alias = "RBX::AnchorVerb::~AnchorVerb()")]
// was: __ZN3RBX10AnchorVerbD1Ev
pub fn stub_404388() -> ! {
    todo!("0x404388 RBX::AnchorVerb::~AnchorVerb()")
}

// 0x40438c — __ZN3RBX19DeleteSelectionVerbD1Ev
// type: void __fastcall(RBX::DeleteSelectionVerb *__hidden this)
#[doc(alias = "RBX::DeleteSelectionVerb::~DeleteSelectionVerb()")]
// was: __ZN3RBX19DeleteSelectionVerbD1Ev
pub fn stub_40438c() -> ! {
    todo!("0x40438c RBX::DeleteSelectionVerb::~DeleteSelectionVerb()")
}

// 0x404390 — __ZN3RBX23PlayDeleteSelectionVerbD1Ev
// type: void __fastcall(RBX::PlayDeleteSelectionVerb *__hidden this)
#[doc(alias = "RBX::PlayDeleteSelectionVerb::~PlayDeleteSelectionVerb()")]
// was: __ZN3RBX23PlayDeleteSelectionVerbD1Ev
pub fn stub_404390() -> ! {
    todo!("0x404390 RBX::PlayDeleteSelectionVerb::~PlayDeleteSelectionVerb()")
}

// 0x404394 — __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_404394() -> ! {
    todo!("0x404394 RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x404518 — __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEED1Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEED1Ev
pub fn stub_404518() -> ! {
    todo!("0x404518 RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x40451c — __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEED0Ev
pub fn stub_40451c() -> ! {
    todo!("0x40451c RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x4045bc — __ZNK3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_4045bc() -> ! {
    todo!("0x4045bc RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x4045f4 — __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_4045f4() -> ! {
    todo!("0x4045f4 RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x404708 — __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_10HammerToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_404708() -> ! {
    todo!("0x404708 RBX::TToolVerb<RBX::HammerTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x4047d4 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10HammerToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::HammerTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HammerTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10HammerToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_4047d4() -> ! {
    todo!("0x4047d4 boost::shared_ptr<RBX::HammerTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HammerTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x404888 — __ZN5boost10shared_ptrIN3RBX10HammerToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::HammerTool>::shared_ptr<RBX::HammerTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10HammerToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_404888() -> ! {
    todo!("0x404888 boost::shared_ptr<RBX::HammerTool>::shared_ptr<RBX::HammerTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x404950 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_10HammerToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::HammerTool,RBX::HammerTool>(rbx_core::SharedPtr<RBX::HammerTool> const*,RBX::HammerTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_10HammerToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_404950() -> ! {
    todo!("0x404950 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::HammerTool,RBX::HammerTool>(boost::shared_ptr<RBX::HammerTool> const*,RBX::HammerTool *)const")
}

// 0x404a34 — __ZN5boost6detail12shared_countC2IPN3RBX10HammerToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10HammerToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_404a34() -> ! {
    todo!("0x404a34 boost::detail::shared_count::shared_count<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x404b2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_404b2c() -> ! {
    todo!("0x404b2c boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x404b30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_404b30() -> ! {
    todo!("0x404b30 boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x404b34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_404b34() -> ! {
    todo!("0x404b34 boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x404b44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_404b44() -> ! {
    todo!("0x404b44 boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x404b5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10HammerToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_404b5c() -> ! {
    todo!("0x404b5c boost::detail::sp_counted_impl_pd<RBX::HammerTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x404b60 — __ZN3RBX4Name7declareILZNS_11sHammerToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sHammerToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_11sHammerToolEEEERKS0_v
pub fn stub_404b60() -> ! {
    todo!("0x404b60 __ZN3RBX4Name7declareILZNS_11sHammerToolEEEERKS0_v")
}

// 0x404ba4 — __ZN3RBX4Name13callDoDeclareILZNS_11sHammerToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sHammerToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sHammerToolEEEEvv
pub fn stub_404ba4() -> ! {
    todo!("0x404ba4 __ZN3RBX4Name13callDoDeclareILZNS_11sHammerToolEEEEvv")
}

// 0x404ba8 — __ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v
pub fn stub_404ba8() -> ! {
    todo!("0x404ba8 __ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v")
}

// 0x404c8c — __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_404c8c() -> ! {
    todo!("0x404c8c RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x404e10 — __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEED0Ev
pub fn stub_404e10() -> ! {
    todo!("0x404e10 RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x404eb0 — __ZNK3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_404eb0() -> ! {
    todo!("0x404eb0 RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x404ee8 — __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_404ee8() -> ! {
    todo!("0x404ee8 RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x404ffc — __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_9CloneToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_404ffc() -> ! {
    todo!("0x404ffc RBX::TToolVerb<RBX::CloneTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x4050c8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9CloneToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::CloneTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::CloneTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9CloneToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_4050c8() -> ! {
    todo!("0x4050c8 boost::shared_ptr<RBX::CloneTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::CloneTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x40517c — __ZN5boost10shared_ptrIN3RBX9CloneToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::CloneTool>::shared_ptr<RBX::CloneTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9CloneToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_40517c() -> ! {
    todo!("0x40517c boost::shared_ptr<RBX::CloneTool>::shared_ptr<RBX::CloneTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x405244 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9CloneToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::CloneTool,RBX::CloneTool>(rbx_core::SharedPtr<RBX::CloneTool> const*,RBX::CloneTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_9CloneToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_405244() -> ! {
    todo!("0x405244 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::CloneTool,RBX::CloneTool>(boost::shared_ptr<RBX::CloneTool> const*,RBX::CloneTool *)const")
}

// 0x405328 — __ZN5boost6detail12shared_countC2IPN3RBX9CloneToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9CloneToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_405328() -> ! {
    todo!("0x405328 boost::detail::shared_count::shared_count<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x405420 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_405420() -> ! {
    todo!("0x405420 boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x405424 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_405424() -> ! {
    todo!("0x405424 boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x405428 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_405428() -> ! {
    todo!("0x405428 boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x405438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_405438() -> ! {
    todo!("0x405438 boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x405450 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9CloneToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_405450() -> ! {
    todo!("0x405450 boost::detail::sp_counted_impl_pd<RBX::CloneTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x405454 — __ZN3RBX4Name7declareILZNS_10sCloneToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sCloneToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_10sCloneToolEEEERKS0_v
pub fn stub_405454() -> ! {
    todo!("0x405454 __ZN3RBX4Name7declareILZNS_10sCloneToolEEEERKS0_v")
}

// 0x405498 — __ZN3RBX4Name13callDoDeclareILZNS_10sCloneToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sCloneToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sCloneToolEEEEvv
pub fn stub_405498() -> ! {
    todo!("0x405498 __ZN3RBX4Name13callDoDeclareILZNS_10sCloneToolEEEEvv")
}

// 0x40549c — __ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v
pub fn stub_40549c() -> ! {
    todo!("0x40549c __ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v")
}

// 0x405580 — __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_405580() -> ! {
    todo!("0x405580 RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x405704 — __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEED0Ev
pub fn stub_405704() -> ! {
    todo!("0x405704 RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x4057a4 — __ZNK3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_4057a4() -> ! {
    todo!("0x4057a4 RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x4057dc — __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_4057dc() -> ! {
    todo!("0x4057dc RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x4058f0 — __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8GrabToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_4058f0() -> ! {
    todo!("0x4058f0 RBX::TToolVerb<RBX::GrabTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x4059bc — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GrabToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::GrabTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GrabTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GrabToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_4059bc() -> ! {
    todo!("0x4059bc boost::shared_ptr<RBX::GrabTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GrabTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x405a70 — __ZN5boost10shared_ptrIN3RBX8GrabToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GrabTool>::shared_ptr<RBX::GrabTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8GrabToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_405a70() -> ! {
    todo!("0x405a70 boost::shared_ptr<RBX::GrabTool>::shared_ptr<RBX::GrabTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x405b38 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8GrabToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::GrabTool,RBX::GrabTool>(rbx_core::SharedPtr<RBX::GrabTool> const*,RBX::GrabTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8GrabToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_405b38() -> ! {
    todo!("0x405b38 void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::GrabTool,RBX::GrabTool>(boost::shared_ptr<RBX::GrabTool> const*,RBX::GrabTool *)const")
}

// 0x405c1c — __ZN5boost6detail12shared_countC2IPN3RBX8GrabToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8GrabToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_405c1c() -> ! {
    todo!("0x405c1c boost::detail::shared_count::shared_count<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x405d14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_405d14() -> ! {
    todo!("0x405d14 boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x405d18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_405d18() -> ! {
    todo!("0x405d18 boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x405d1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_405d1c() -> ! {
    todo!("0x405d1c boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}

// 0x405d2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_405d2c() -> ! {
    todo!("0x405d2c boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x405d44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GrabToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
pub fn stub_405d44() -> ! {
    todo!("0x405d44 boost::detail::sp_counted_impl_pd<RBX::GrabTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")
}

// 0x405d48 — __ZN3RBX4Name7declareILZNS_9sGrabToolEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sGrabToolEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_9sGrabToolEEEERKS0_v
pub fn stub_405d48() -> ! {
    todo!("0x405d48 __ZN3RBX4Name7declareILZNS_9sGrabToolEEEERKS0_v")
}

// 0x405d8c — __ZN3RBX4Name13callDoDeclareILZNS_9sGrabToolEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sGrabToolEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_9sGrabToolEEEEvv
pub fn stub_405d8c() -> ! {
    todo!("0x405d8c __ZN3RBX4Name13callDoDeclareILZNS_9sGrabToolEEEEvv")
}

// 0x405d90 — __ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v
pub fn stub_405d90() -> ! {
    todo!("0x405d90 __ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v")
}

// 0x405e74 — __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
// type: int __fastcall(int, int, char)
#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")]
// was: __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEEC2EPNS_9DataModelEb
pub fn stub_405e74() -> ! {
    todo!("0x405e74 RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::TToolVerb(RBX::DataModel *,bool)")
}

// 0x405ff8 — __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEED0Ev
// type: void __fastcall(RBX::RunStateVerb *)
#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::~TToolVerb()")]
// was: __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEED0Ev
pub fn stub_405ff8() -> ! {
    todo!("0x405ff8 RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::~TToolVerb()")
}

// 0x406098 — __ZNK3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEE9isCheckedEv
// type: bool __fastcall(int)
#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::isChecked(void)const")]
// was: __ZNK3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEE9isCheckedEv
pub fn stub_406098() -> ! {
    todo!("0x406098 RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::isChecked(void)const")
}

// 0x4060d0 — __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
// type: void __fastcall(_BYTE *)
#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
// was: __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEE4doItEPNS_10IDataStateE
pub fn stub_4060d0() -> ! {
    todo!("0x4060d0 RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")
}

// 0x4061e4 — __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEE15newMouseCommandEv
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::newMouseCommand(void)")]
// was: __ZN3RBX9TToolVerbINS_8GameToolENS_12RunStateVerbEE15newMouseCommandEv
pub fn stub_4061e4() -> ! {
    todo!("0x4061e4 RBX::TToolVerb<RBX::GameTool,RBX::RunStateVerb>::newMouseCommand(void)")
}

// 0x4062b0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GameToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
#[doc(alias = "rbx_core::SharedPtr<RBX::GameTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GameTool,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GameToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_4062b0() -> ! {
    todo!("0x4062b0 boost::shared_ptr<RBX::GameTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GameTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x406364 — __ZN5boost10shared_ptrIN3RBX8GameToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::GameTool>::shared_ptr<RBX::GameTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX8GameToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
pub fn stub_406364() -> ! {
    todo!("0x406364 boost::shared_ptr<RBX::GameTool>::shared_ptr<RBX::GameTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x40642c — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8GameToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::GameTool,RBX::GameTool>(rbx_core::SharedPtr<RBX::GameTool> const*,RBX::GameTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_8GameToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_40642c() -> ! {
    todo!("0x40642c void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::GameTool,RBX::GameTool>(boost::shared_ptr<RBX::GameTool> const*,RBX::GameTool *)const")
}

// 0x406510 — __ZN5boost6detail12shared_countC2IPN3RBX8GameToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX8GameToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
pub fn stub_406510() -> ! {
    todo!("0x406510 boost::detail::shared_count::shared_count<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")
}

// 0x406608 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
pub fn stub_406608() -> ! {
    todo!("0x406608 boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x40660c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
pub fn stub_40660c() -> ! {
    todo!("0x40660c boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")
}

// 0x406610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8GameToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
pub fn stub_406610() -> ! {
    todo!("0x406610 boost::detail::sp_counted_impl_pd<RBX::GameTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")
}
