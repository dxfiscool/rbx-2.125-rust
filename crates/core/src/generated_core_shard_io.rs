//! core shard IO — 100 core stubs EA-sorted, 0x2b7698..0xf4bb74 (strict RBX|boost excluding Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|ViewController|UIApplication|Platform|iOS, EA-sorted ascending, next 100 uncovered after 0x2b7698 prior 108 remaining).
//! Source: ida/export.json filtered where demangled NOT containing Reflection|Instance|DataModel|Ogre|G3D|Rendering|Adorn|RakNet|Network|Replicat|Socket|Sound|Audio|FMOD|Script|Lua|lua|ViewController|UIApplication|Platform|iOS but containing RBX:: or boost::, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)")]
// 0x2b7698 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_9WorkspaceEEEmv
// was: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)
pub fn stub_0x2b7698() {
    // IDA 0x2b7698: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::AdvMoveToolBase::AdvMoveToolBase(RBX::Workspace *)")]
// 0x2d28a4 — __ZN3RBX15AdvMoveToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Workspace *)
// was: RBX::AdvMoveToolBase::AdvMoveToolBase(RBX::Workspace *)
pub fn stub_0x2d28a4() {
    // IDA 0x2d28a4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::AxisToolBase::AxisToolBase(RBX::Workspace *)")]
// 0x2da160 — __ZN3RBX12AxisToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::AxisToolBase *__hidden this, RBX::Workspace *)
// was: RBX::AxisToolBase::AxisToolBase(RBX::Workspace *)
pub fn stub_0x2da160() {
    // IDA 0x2da160: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CloneTool::CloneTool(RBX::Workspace *)")]
// 0x2db7a4 — __ZN3RBX9CloneToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this, RBX::Workspace *)
// was: RBX::CloneTool::CloneTool(RBX::Workspace *)
pub fn stub_0x2db7a4() {
    // IDA 0x2db7a4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CloneTool::CloneTool(RBX::Workspace *)")]
// 0x2db7a8 — __ZN3RBX9CloneToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CloneTool *__hidden this, RBX::Workspace *)
// was: RBX::CloneTool::CloneTool(RBX::Workspace *)
pub fn stub_0x2db7a8() {
    // IDA 0x2db7a8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
// 0x2e2f2c — __ZN3RBX8GameToolC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::GameTool *this, RBX::Workspace *)
// was: RBX::GameTool::GameTool(RBX::Workspace *)
pub fn stub_0x2e2f2c() {
    // IDA 0x2e2f2c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GameTool::GameTool(RBX::Workspace *)")]
// 0x2e2f30 — __ZN3RBX8GameToolC2EPNS_9WorkspaceE
// type: RBX::GameTool *__fastcall(RBX::GameTool *this, Workspace *)
// was: RBX::GameTool::GameTool(RBX::Workspace *)
pub fn stub_0x2e2f30() {
    // IDA 0x2e2f30: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
// 0x2e37c4 — __ZN3RBX8GrabToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this, RBX::Workspace *)
// was: RBX::GrabTool::GrabTool(RBX::Workspace *)
pub fn stub_0x2e37c4() {
    // IDA 0x2e37c4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GrabTool::GrabTool(RBX::Workspace *)")]
// 0x2e37c8 — __ZN3RBX8GrabToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::GrabTool *__hidden this, RBX::Workspace *)
// was: RBX::GrabTool::GrabTool(RBX::Workspace *)
pub fn stub_0x2e37c8() {
    // IDA 0x2e37c8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
// 0x2e4518 — __ZN3RBX10HammerToolC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::HammerTool *this, RBX::Workspace *)
// was: RBX::HammerTool::HammerTool(RBX::Workspace *)
pub fn stub_0x2e4518() {
    // IDA 0x2e4518: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HammerTool::HammerTool(RBX::Workspace *)")]
// 0x2e451c — __ZN3RBX10HammerToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::HammerTool *__hidden this, RBX::Workspace *)
// was: RBX::HammerTool::HammerTool(RBX::Workspace *)
pub fn stub_0x2e451c() {
    // IDA 0x2e451c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
// 0x2eee88 — __ZN3RBX8NullToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NullTool *__hidden this, RBX::Workspace *)
// was: RBX::NullTool::NullTool(RBX::Workspace *)
pub fn stub_0x2eee88() {
    // IDA 0x2eee88: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::NullTool::NullTool(RBX::Workspace *)")]
// 0x2eee8c — __ZN3RBX8NullToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NullTool *__hidden this, RBX::Workspace *)
// was: RBX::NullTool::NullTool(RBX::Workspace *)
pub fn stub_0x2eee8c() {
    // IDA 0x2eee8c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
// 0x2ef12c — __ZN3RBX11NewNullToolC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, RBX::Workspace *)
// was: RBX::NewNullTool::NewNullTool(RBX::Workspace *)
pub fn stub_0x2ef12c() {
    // IDA 0x2ef12c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::NewNullTool::NewNullTool(RBX::Workspace *)")]
// 0x2ef130 — __ZN3RBX11NewNullToolC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::NewNullTool *__hidden this, RBX::Workspace *)
// was: RBX::NewNullTool::NewNullTool(RBX::Workspace *)
pub fn stub_0x2ef130() {
    // IDA 0x2ef130: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)")]
// 0x2f6ff4 — __ZN3RBX16BoxSelectCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::BoxSelectCommand *__hidden this, RBX::Workspace *)
// was: RBX::BoxSelectCommand::BoxSelectCommand(RBX::Workspace *)
pub fn stub_0x2f6ff4() {
    // IDA 0x2f6ff4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Accoutrement::upTo_InWorkspace(void)")]
// 0x38fb1c — __ZN3RBX12Accoutrement16upTo_InWorkspaceEv
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
// was: RBX::Accoutrement::upTo_InWorkspace(void)
pub fn stub_0x38fb1c() {
    // IDA 0x38fb1c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)")]
// 0x3f5868 — __ZN3RBX19CameraCenterCommandC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::CameraCenterCommand *this, RBX::Workspace *)
// was: RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)
pub fn stub_0x3f5868() {
    // IDA 0x3f5868: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)")]
// 0x3f586c — __ZN3RBX19CameraCenterCommandC2EPNS_9WorkspaceE
// type: RBX::Verb *__fastcall(RBX::CameraCenterCommand *this, RBX::Workspace *)
// was: RBX::CameraCenterCommand::CameraCenterCommand(RBX::Workspace *)
pub fn stub_0x3f586c() {
    // IDA 0x3f586c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraVerb::CameraVerb(std::string,RBX::Workspace *)")]
// 0x3f5a4c — __ZN3RBX10CameraVerbC2ESsPNS_9WorkspaceE
// type: RBX::Verb *__fastcall(RBX::Verb *, const std::string *, int)
// was: RBX::CameraVerb::CameraVerb(std::string,RBX::Workspace *)
pub fn stub_0x3f5a4c() {
    // IDA 0x3f5a4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CameraZoomExtentsCommand::CameraZoomExtentsCommand(RBX::Workspace *)")]
// 0x3fb594 — __ZN3RBX24CameraZoomExtentsCommandC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::CameraZoomExtentsCommand *this, RBX::Workspace *)
// was: RBX::CameraZoomExtentsCommand::CameraZoomExtentsCommand(RBX::Workspace *)
pub fn stub_0x3fb594() {
    // IDA 0x3fb594: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::CameraZoomExtentsCommand::CameraZoomExtentsCommand(RBX::Workspace *)")]
// 0x3fb598 — __ZN3RBX24CameraZoomExtentsCommandC2EPNS_9WorkspaceE
// type: RBX::Verb *__fastcall(RBX::CameraZoomExtentsCommand *this, RBX::Workspace *)
// was: RBX::CameraZoomExtentsCommand::CameraZoomExtentsCommand(RBX::Workspace *)
pub fn stub_0x3fb598() {
    // IDA 0x3fb598: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::MoveResizeJoinTool::MoveResizeJoinTool(RBX::Workspace *)")]
// 0x413174 — __ZN3RBX18MoveResizeJoinToolC2EPNS_9WorkspaceE
// type: RBX::AdvArrowToolBase *__fastcall(RBX::MoveResizeJoinTool *this, RBX::Workspace *)
// was: RBX::MoveResizeJoinTool::MoveResizeJoinTool(RBX::Workspace *)
pub fn stub_0x413174() {
    // IDA 0x413174: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Workspace::getCurrentMouseCommand(void)")]
// 0x437650 — __ZN3RBX9Workspace22getCurrentMouseCommandEv
// type: int __fastcall(RBX::Workspace *this)
// was: RBX::Workspace::getCurrentMouseCommand(void)
pub fn stub_0x437650() {
    // IDA 0x437650: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Tool::upTo_InWorkspace(void)")]
// 0x681b88 — __ZN3RBX4Tool16upTo_InWorkspaceEv
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
// was: RBX::Tool::upTo_InWorkspace(void)
pub fn stub_0x681b88() {
    // IDA 0x681b88: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::AdvArrowToolBase::AdvArrowToolBase(RBX::Workspace *)")]
// 0x6dd568 — __ZN3RBX16AdvArrowToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, RBX::Workspace *)
// was: RBX::AdvArrowToolBase::AdvArrowToolBase(RBX::Workspace *)
pub fn stub_0x6dd568() {
    // IDA 0x6dd568: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ArrowToolBase::ArrowToolBase(RBX::Workspace *)")]
// 0x6dd644 — __ZN3RBX13ArrowToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Workspace *)
// was: RBX::ArrowToolBase::ArrowToolBase(RBX::Workspace *)
pub fn stub_0x6dd644() {
    // IDA 0x6dd644: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Workspace::getStatsFileTimeTotal(void)const")]
// 0x6de968 — __ZNK3RBX9Workspace21getStatsFileTimeTotalEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getStatsFileTimeTotal(void)const
pub fn stub_0x6de968() {
    // IDA 0x6de968: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Workspace::getStatsSyncHttpGetTime(void)const")]
// 0x6de984 — __ZNK3RBX9Workspace23getStatsSyncHttpGetTimeEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getStatsSyncHttpGetTime(void)const
pub fn stub_0x6de984() {
    // IDA 0x6de984: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Workspace::getStatsXMLLoadTime(void)const")]
// 0x6de990 — __ZNK3RBX9Workspace19getStatsXMLLoadTimeEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getStatsXMLLoadTime(void)const
pub fn stub_0x6de990() {
    // IDA 0x6de990: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Workspace::getStatsJoinAllTime(void)const")]
// 0x6de99c — __ZNK3RBX9Workspace19getStatsJoinAllTimeEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
// was: RBX::Workspace::getStatsJoinAllTime(void)const
pub fn stub_0x6de99c() {
    // IDA 0x6de99c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WorkspaceStatsItem::WorkspaceStatsItem(void)")]
// 0x6e06a0 — __ZN3RBX18WorkspaceStatsItemC2Ev
// type: _DWORD __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: RBX::WorkspaceStatsItem::WorkspaceStatsItem(void)
pub fn stub_0x6e06a0() {
    // IDA 0x6e06a0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WorkspaceStatsItem::~WorkspaceStatsItem()")]
// 0x6e0854 — __ZN3RBX18WorkspaceStatsItemD1Ev
// type: void __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: RBX::WorkspaceStatsItem::~WorkspaceStatsItem()
pub fn stub_0x6e0854() {
    // IDA 0x6e0854: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WorkspaceStatsItem::~WorkspaceStatsItem()")]
// 0x6e0890 — __ZN3RBX18WorkspaceStatsItemD0Ev
// type: void __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: RBX::WorkspaceStatsItem::~WorkspaceStatsItem()
pub fn stub_0x6e0890() {
    // IDA 0x6e0890: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()")]
// 0x6e0964 — __ZThn32_N3RBX18WorkspaceStatsItemD1Ev
// type: void __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()
pub fn stub_0x6e0964() {
    // IDA 0x6e0964: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()")]
// 0x6e09a4 — __ZThn32_N3RBX18WorkspaceStatsItemD0Ev
// type: void __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()
pub fn stub_0x6e09a4() {
    // IDA 0x6e09a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()")]
// 0x6e0a78 — __ZThn36_N3RBX18WorkspaceStatsItemD1Ev
// type: void __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()
pub fn stub_0x6e0a78() {
    // IDA 0x6e0a78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()")]
// 0x6e0ab8 — __ZThn36_N3RBX18WorkspaceStatsItemD0Ev
// type: void __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: non-virtual thunk to RBX::WorkspaceStatsItem::~WorkspaceStatsItem()
pub fn stub_0x6e0ab8() {
    // IDA 0x6e0ab8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Humanoid::onLocalHumanoidEnteringWorkspace(void)")]
// 0x7bc0b4 — __ZN3RBX8Humanoid32onLocalHumanoidEnteringWorkspaceEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
// was: RBX::Humanoid::onLocalHumanoidEnteringWorkspace(void)
pub fn stub_0x7bc0b4() {
    // IDA 0x7bc0b4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::ManualJointHelper(RBX::Workspace *)")]
// 0x8a7f94 — __ZN3RBX17ManualJointHelperC1EPNS_9WorkspaceE
// type: int __fastcall(RBX::ManualJointHelper *this, RBX::Workspace *)
// was: RBX::ManualJointHelper::ManualJointHelper(RBX::Workspace *)
pub fn stub_0x8a7f94() {
    // IDA 0x8a7f94: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::ManualJointHelper(RBX::Workspace *)")]
// 0x8a7f98 — __ZN3RBX17ManualJointHelperC2EPNS_9WorkspaceE
// type: RBX::ManualJointHelper *__fastcall(RBX::ManualJointHelper *this, RBX::Workspace *)
// was: RBX::ManualJointHelper::ManualJointHelper(RBX::Workspace *)
pub fn stub_0x8a7f98() {
    // IDA 0x8a7f98: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::setWorkspace(RBX::Workspace *)")]
// 0x8a80c0 — __ZN3RBX17ManualJointHelper12setWorkspaceEPNS_9WorkspaceE
// type: int __fastcall(RBX::ManualJointHelper *this, RBX::IAdornableCollector **)
// was: RBX::ManualJointHelper::setWorkspace(RBX::Workspace *)
pub fn stub_0x8a80c0() {
    // IDA 0x8a80c0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ViewRbxGfx::getWorkspace(void)")]
// 0xbe9620 — __ZN3RBX10ViewRbxGfx12getWorkspaceEv
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
// was: RBX::ViewRbxGfx::getWorkspace(void)
pub fn stub_0xbe9620() {
    // IDA 0xbe9620: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<YieldFunctionStateObject> RBX::shared_from<YieldFunctionStateObject>(YieldFunctionStateObject*)")]
// 0xf2a334 — j___ZN3RBX11shared_fromI24YieldFunctionStateObjectEEN5boost10shared_ptrIT_EEPS4_
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<YieldFunctionStateObject> RBX::shared_from<YieldFunctionStateObject>(YieldFunctionStateObject*)
pub fn stub_0xf2a334() {
    // IDA 0xf2a334: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<YieldFunctionStateObject>::shared_ptr<YieldFunctionStateObject>(YieldFunctionStateObject *)")]
// 0xf2a384 — j___ZN5boost10shared_ptrI24YieldFunctionStateObjectEC2IS1_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
// was: boost::shared_ptr<YieldFunctionStateObject>::shared_ptr<YieldFunctionStateObject>(YieldFunctionStateObject *)
pub fn stub_0xf2a384() {
    // IDA 0xf2a384: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>)")]
// 0xf2a3a4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEC2ES6_S8_
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>)
pub fn stub_0xf2a3a4() {
    // IDA 0xf2a3a4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string> &,boost::_bi::list1<std::string &> &,int)")]
// 0xf2a3c4 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string> &,boost::_bi::list1<std::string &> &,int)
pub fn stub_0xf2a3c4() {
    // IDA 0xf2a3c4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>)")]
// 0xf2a424 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEC2ES6_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>)
pub fn stub_0xf2a424() {
    // IDA 0xf2a424: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list_av_2<rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,std::string,rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>(void (YieldFunctionStateObject::*)(std::string),rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>)")]
// 0xf2a474 — j___ZN5boost4bindIv24YieldFunctionStateObjectSsNS_10shared_ptrIS1_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list_av_2<boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,std::string,boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>>(void (YieldFunctionStateObject::*)(std::string),boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>)
pub fn stub_0xf2a474() {
    // IDA 0xf2a474: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<YieldFunctionStateObject>(YieldFunctionStateObject *)")]
// 0xf2a494 — j___ZN5boost6detail12shared_countC2I24YieldFunctionStateObjectEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<YieldFunctionStateObject>(YieldFunctionStateObject *)
pub fn stub_0xf2a494() {
    // IDA 0xf2a494: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf2a4c4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0xf2a4c4() {
    // IDA 0xf2a4c4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>)")]
// 0xf2a584 — j___ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>)
pub fn stub_0xf2a584() {
    // IDA 0xf2a584: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<YieldFunctionStateObject>::_internal_accept_owner<YieldFunctionStateObject,YieldFunctionStateObject>(rbx_core::SharedPtr<YieldFunctionStateObject> const*,YieldFunctionStateObject *)const")]
// 0xf2a5f4 — j___ZNK5boost23enable_shared_from_thisI24YieldFunctionStateObjectE22_internal_accept_ownerIS1_S1_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: void boost::enable_shared_from_this<YieldFunctionStateObject>::_internal_accept_owner<YieldFunctionStateObject,YieldFunctionStateObject>(boost::shared_ptr<YieldFunctionStateObject> const*,YieldFunctionStateObject *)const
pub fn stub_0xf2a5f4() {
    // IDA 0xf2a5f4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "void boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>::call<rbx_core::SharedPtr<YieldFunctionStateObject>,std::string>(rbx_core::SharedPtr<YieldFunctionStateObject> &,void const*,std::string &)const")]
// 0xf2a614 — j___ZNK5boost4_mfi3mf1Iv24YieldFunctionStateObjectSsE4callINS_10shared_ptrIS2_EESsEEvRT_PKvRT0_
// type: int __fastcall(_DWORD, _DWORD)
// was: void boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>::call<boost::shared_ptr<YieldFunctionStateObject>,std::string>(boost::shared_ptr<YieldFunctionStateObject> &,void const*,std::string &)const
pub fn stub_0xf2a614() {
    // IDA 0xf2a614: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf2a6a4 — j___ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS5_5list2INS5_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int(void)
// was: void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0xf2a6a4() {
    // IDA 0xf2a6a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0xf2a6b4 — j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS5_5list2INS5_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_0xf2a6b4() {
    // IDA 0xf2a6b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf2a6c4 — j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS5_5list2INS5_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xf2a6c4() {
    // IDA 0xf2a6c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)")]
// 0xf2b124 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_9WorkspaceEEEmv
// type: int(void)
// was: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)
pub fn stub_0xf2b124() {
    // IDA 0xf2b124: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf2d654 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// was: boost::shared_ptr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf2d654() {
    // IDA 0xf2d654: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)")]
// 0xf33054 — j___ZN3RBX11shared_fromINS_9WorkspaceEEEN5boost10shared_ptrIT_EEPS4_
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)
pub fn stub_0xf33054() {
    // IDA 0xf33054: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::MoveResizeJoinTool::MoveResizeJoinTool(RBX::Workspace *)")]
// 0xf34964 — j___ZN3RBX18MoveResizeJoinToolC2EPNS_9WorkspaceE
// type: RBX::AdvArrowToolBase *__fastcall(RBX::MoveResizeJoinTool *this, RBX::Workspace *)
// was: RBX::MoveResizeJoinTool::MoveResizeJoinTool(RBX::Workspace *)
pub fn stub_0xf34964() {
    // IDA 0xf34964: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AnchorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AnchorTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34ce4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10AnchorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::AnchorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AnchorTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34ce4() {
    // IDA 0xf34ce4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HammerTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HammerTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34cf4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_10HammerToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::HammerTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HammerTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34cf4() {
    // IDA 0xf34cf4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvMoveTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d04 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11AdvMoveToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::AdvMoveTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvMoveTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d04() {
    // IDA 0xf34d04: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DropperTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DropperTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d14 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11DropperToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::DropperTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::DropperTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d14() {
    // IDA 0xf34d14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MaterialTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MaterialTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d24 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12MaterialToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::MaterialTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MaterialTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d24() {
    // IDA 0xf34d24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvRotateTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvRotateTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d34 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13AdvRotateToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::AdvRotateTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvRotateTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d34() {
    // IDA 0xf34d34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LeftMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LeftMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d44 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13LeftMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::LeftMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LeftMotorTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d44() {
    // IDA 0xf34d44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::UniversalTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::UniversalTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d54 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_13UniversalToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::UniversalTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::UniversalTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d54() {
    // IDA 0xf34d54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AxisRotateTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AxisRotateTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d64 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14AxisRotateToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::AxisRotateTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AxisRotateTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d64() {
    // IDA 0xf34d64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RightMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::RightMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d74 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14RightMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::RightMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::RightMotorTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d74() {
    // IDA 0xf34d74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MoveResizeJoinTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d84 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18MoveResizeJoinToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::MoveResizeJoinTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::MoveResizeJoinTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d84() {
    // IDA 0xf34d84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::OscillateMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::OscillateMotorTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34d94 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18OscillateMotorToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::OscillateMotorTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::OscillateMotorTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34d94() {
    // IDA 0xf34d94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::SmoothNoOutlinesTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::SmoothNoOutlinesTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34da4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_20SmoothNoOutlinesToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::SmoothNoOutlinesTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::SmoothNoOutlinesTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34da4() {
    // IDA 0xf34da4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FillTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FillTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34db4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8FillToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::FillTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FillTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34db4() {
    // IDA 0xf34db4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlatTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FlatTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34dc4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8FlatToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::FlatTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::FlatTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34dc4() {
    // IDA 0xf34dc4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GameTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GameTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34dd4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GameToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::GameTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GameTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34dd4() {
    // IDA 0xf34dd4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GlueTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GlueTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34de4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GlueToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::GlueTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GlueTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34de4() {
    // IDA 0xf34de4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GrabTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GrabTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34df4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8GrabToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::GrabTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::GrabTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34df4() {
    // IDA 0xf34df4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LockTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LockTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34e04 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8LockToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::LockTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::LockTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34e04() {
    // IDA 0xf34e04: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NullTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34e14 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8NullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::NullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NullTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34e14() {
    // IDA 0xf34e14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::WeldTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::WeldTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34e24 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_8WeldToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::WeldTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::WeldTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34e24() {
    // IDA 0xf34e24: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CloneTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::CloneTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34e34 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9CloneToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::CloneTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::CloneTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34e34() {
    // IDA 0xf34e34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HingeTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HingeTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34e44 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9HingeToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::HingeTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::HingeTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34e44() {
    // IDA 0xf34e44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InletTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::InletTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34e54 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9InletToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::InletTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::InletTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34e54() {
    // IDA 0xf34e54: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StudsTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::StudsTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf34e64 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_9StudsToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: void __fastcall(int, RBX::Workspace *)
// was: boost::shared_ptr<RBX::StudsTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::StudsTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf34e64() {
    // IDA 0xf34e64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Workspace::getCurrentMouseCommand(void)")]
// 0xf36b14 — j___ZN3RBX9Workspace22getCurrentMouseCommandEv
// type: int __fastcall(RBX::Workspace *this)
// was: RBX::Workspace::getCurrentMouseCommand(void)
pub fn stub_0xf36b14() {
    // IDA 0xf36b14: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraTiltUpCommand::CameraTiltUpCommand(RBX::Workspace *)")]
// 0xf3e984 — j___ZN3RBX19CameraTiltUpCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraTiltUpCommand *__hidden this, RBX::Workspace *)
// was: RBX::CameraTiltUpCommand::CameraTiltUpCommand(RBX::Workspace *)
pub fn stub_0xf3e984() {
    // IDA 0xf3e984: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraZoomInCommand::CameraZoomInCommand(RBX::Workspace *)")]
// 0xf3e994 — j___ZN3RBX19CameraZoomInCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraZoomInCommand *__hidden this, RBX::Workspace *)
// was: RBX::CameraZoomInCommand::CameraZoomInCommand(RBX::Workspace *)
pub fn stub_0xf3e994() {
    // IDA 0xf3e994: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraPanLeftCommand::CameraPanLeftCommand(RBX::Workspace *)")]
// 0xf3e9a4 — j___ZN3RBX20CameraPanLeftCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraPanLeftCommand *__hidden this, RBX::Workspace *)
// was: RBX::CameraPanLeftCommand::CameraPanLeftCommand(RBX::Workspace *)
pub fn stub_0xf3e9a4() {
    // IDA 0xf3e9a4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraZoomOutCommand::CameraZoomOutCommand(RBX::Workspace *)")]
// 0xf3e9b4 — j___ZN3RBX20CameraZoomOutCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraZoomOutCommand *__hidden this, RBX::Workspace *)
// was: RBX::CameraZoomOutCommand::CameraZoomOutCommand(RBX::Workspace *)
pub fn stub_0xf3e9b4() {
    // IDA 0xf3e9b4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraPanRightCommand::CameraPanRightCommand(RBX::Workspace *)")]
// 0xf3e9c4 — j___ZN3RBX21CameraPanRightCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraPanRightCommand *__hidden this, RBX::Workspace *)
// was: RBX::CameraPanRightCommand::CameraPanRightCommand(RBX::Workspace *)
pub fn stub_0xf3e9c4() {
    // IDA 0xf3e9c4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::CameraTiltDownCommand::CameraTiltDownCommand(RBX::Workspace *)")]
// 0xf3e9d4 — j___ZN3RBX21CameraTiltDownCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::CameraTiltDownCommand *__hidden this, RBX::Workspace *)
// was: RBX::CameraTiltDownCommand::CameraTiltDownCommand(RBX::Workspace *)
pub fn stub_0xf3e9d4() {
    // IDA 0xf3e9d4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ToolMouseCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::ToolMouseCommand,RBX::Workspace *,RBX::Tool *>(RBX::Workspace *,RBX::Tool *)")]
// 0xf492b4 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16ToolMouseCommandEPNS_9WorkspaceEPNS_4ToolEEEN5boost10shared_ptrIT_EET0_T1_
// was: boost::shared_ptr<RBX::ToolMouseCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::ToolMouseCommand,RBX::Workspace *,RBX::Tool *>(RBX::Workspace *,RBX::Tool *)
pub fn stub_0xf492b4() {
    // IDA 0xf492b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(void)const")]
// 0xf49ce4 — j___ZNK3RBX15ServiceProvider4findINS_9WorkspaceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(void)const
pub fn stub_0xf49ce4() {
    // IDA 0xf49ce4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ArrowToolBase::ArrowToolBase(RBX::Workspace *)")]
// 0xf4b924 — j___ZN3RBX13ArrowToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::ArrowToolBase *__hidden this, RBX::Workspace *)
// was: RBX::ArrowToolBase::ArrowToolBase(RBX::Workspace *)
pub fn stub_0xf4b924() {
    // IDA 0xf4b924: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::AdvArrowToolBase::AdvArrowToolBase(RBX::Workspace *)")]
// 0xf4ba14 — j___ZN3RBX16AdvArrowToolBaseC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::AdvArrowToolBase *__hidden this, RBX::Workspace *)
// was: RBX::AdvArrowToolBase::AdvArrowToolBase(RBX::Workspace *)
pub fn stub_0xf4ba14() {
    // IDA 0xf4ba14: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WorkspaceStatsItem::create(RBX::Workspace const*,RBX::World const*,RBX::RunService const*)")]
// 0xf4ba44 — j___ZN3RBX18WorkspaceStatsItem6createEPKNS_9WorkspaceEPKNS_5WorldEPKNS_10RunServiceE
// type: _DWORD __fastcall(RBX::WorkspaceStatsItem *__hidden this, const RBX::Workspace *, const RBX::World *, const RBX::RunService *)
// was: RBX::WorkspaceStatsItem::create(RBX::Workspace const*,RBX::World const*,RBX::RunService const*)
pub fn stub_0xf4ba44() {
    // IDA 0xf4ba44: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::WorkspaceStatsItem::WorkspaceStatsItem(void)")]
// 0xf4ba54 — j___ZN3RBX18WorkspaceStatsItemC2Ev
// type: _DWORD __fastcall(RBX::WorkspaceStatsItem *__hidden this)
// was: RBX::WorkspaceStatsItem::WorkspaceStatsItem(void)
pub fn stub_0xf4ba54() {
    // IDA 0xf4ba54: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NewNullTool,RBX::Workspace *>(RBX::Workspace *)")]
// 0xf4bb74 — j___ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11NewNullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::NewNullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NewNullTool,RBX::Workspace *>(RBX::Workspace *)
pub fn stub_0xf4bb74() {
    // IDA 0xf4bb74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
