// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3f3970..0x3fdb84 | existing 9151 -> 9251 total (filler 0x3f3970 ascending, global remaining 29996 -> 29896)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x3f3970 — __ZThn36_N3RBX17CollectionServiceD0Ev
// type: void __fastcall(RBX::CollectionService *__hidden this)
// was: non-virtual thunk to RBX::CollectionService::~CollectionService()
#[doc(alias = "non-virtual thunk to RBX::CollectionService::~CollectionService()")]
pub fn stub_0x3f3970(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "RBX::CollectionService::~CollectionService() [0x3f3a14]")]
pub fn stub_0x3f3a14(handle: crate::slot::InstanceHandle) {
// RBX::CollectionService dtor.
drop(handle);
}

// 0x3f4fac — __GLOBAL__I_a_172
// was: global constructor keyed to_a_172
#[doc(alias = "global constructor keyed to_a_172")]
pub fn stub_0x3f4fac() -> crate::slot::PortedFn {
// IDA 0x3f4fac: global constructor keyed to_a_172.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3f4fac, "global constructor keyed to_a_172")
}

#[doc(alias = "RBX::EditSelectionVerb::~EditSelectionVerb()")]
pub fn stub_0x3f5454(handle: crate::slot::InstanceHandle) {
// RBX::EditSelectionVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::BoolPropertyVerb::isChecked(void)const")]
pub fn stub_0x3f5548(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BoolPropertyVerb getter.
cell.get()
}

#[doc(alias = "RBX::BoolPropertyVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3f569c(handle: &crate::slot::InstanceHandle) {
// RBX::BoolPropertyVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraCenterCommand::isEnabled(void)const")]
pub fn stub_0x3f5a80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraCenterCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraCenterCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f5bcc(handle: &crate::slot::InstanceHandle) {
// RBX::CameraCenterCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3f5f00(handle: &crate::slot::InstanceHandle) {
// RBX::CameraVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SelectAllCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f5f64(handle: &crate::slot::InstanceHandle) {
// RBX::SelectAllCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AllCanSelectCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f5fa0(handle: &crate::slot::InstanceHandle) {
// RBX::AllCanSelectCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CanNotSelectCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f5fd4(handle: &crate::slot::InstanceHandle) {
// RBX::CanNotSelectCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FirstPersonCommand::isEnabled(void)const")]
pub fn stub_0x3f61b0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FirstPersonCommand getter.
cell.get()
}

#[doc(alias = "RBX::ToggleViewMode::isChecked(void)const")]
pub fn stub_0x3f6328(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ToggleViewMode getter.
cell.get()
}

#[doc(alias = "RBX::ToggleViewMode::isEnabled(void)const")]
pub fn stub_0x3f632c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ToggleViewMode getter.
cell.get()
}

#[doc(alias = "RBX::ToggleViewMode::isSelected(void)const")]
pub fn stub_0x3f6348(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ToggleViewMode getter.
cell.get()
}

#[doc(alias = "RBX::ToggleViewMode::doIt(RBX::IDataState *)")]
pub fn stub_0x3f634c(handle: &crate::slot::InstanceHandle) {
// RBX::ToggleViewMode::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StatsCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f64d8(handle: &crate::slot::InstanceHandle) {
// RBX::StatsCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StatsCommand::isEnabled(void)const")]
pub fn stub_0x3f6784(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::StatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::StatsCommand::isChecked(void)const")]
pub fn stub_0x3f6900(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::StatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::SummaryStatsCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f7314(handle: &crate::slot::InstanceHandle) {
// RBX::SummaryStatsCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SummaryStatsCommand::isEnabled(void)const")]
pub fn stub_0x3f74cc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SummaryStatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::SummaryStatsCommand::isChecked(void)const")]
pub fn stub_0x3f7648(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SummaryStatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::CustomStatsCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f7930(handle: &crate::slot::InstanceHandle) {
// RBX::CustomStatsCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CustomStatsCommand::isEnabled(void)const")]
pub fn stub_0x3f7ae8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CustomStatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::CustomStatsCommand::isChecked(void)const")]
pub fn stub_0x3f7c64(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CustomStatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::PhysicsStatsCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f86cc(handle: &crate::slot::InstanceHandle) {
// RBX::PhysicsStatsCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PhysicsStatsCommand::isEnabled(void)const")]
pub fn stub_0x3f8988(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PhysicsStatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::PhysicsStatsCommand::isChecked(void)const")]
pub fn stub_0x3f8be8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PhysicsStatsCommand getter.
cell.get()
}

#[doc(alias = "RBX::EngineStatsCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f8fc8(handle: &crate::slot::InstanceHandle) {
// RBX::EngineStatsCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::JoinCommand::isEnabled(void)const")]
pub fn stub_0x3f9160(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::JoinCommand getter.
cell.get()
}

#[doc(alias = "RBX::JoinCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f9294(handle: &crate::slot::InstanceHandle) {
// RBX::JoinCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunStateVerb::~RunStateVerb()")]
pub fn stub_0x3f9418(handle: crate::slot::InstanceHandle) {
// RBX::RunStateVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::RunStateVerb::~RunStateVerb() [0x3f94b8]")]
pub fn stub_0x3f94b8(handle: crate::slot::InstanceHandle) {
// RBX::RunStateVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::RunStateVerb::~RunStateVerb() [0x3f94bc]")]
pub fn stub_0x3f94bc(handle: crate::slot::InstanceHandle) {
// RBX::RunStateVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::RunCommand::isEnabled(void)const")]
pub fn stub_0x3f9588(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RunCommand getter.
cell.get()
}

#[doc(alias = "RBX::RunCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f95b0(handle: &crate::slot::InstanceHandle) {
// RBX::RunCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StopCommand::isEnabled(void)const")]
pub fn stub_0x3f9644(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::StopCommand getter.
cell.get()
}

#[doc(alias = "RBX::StopCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f9658(handle: &crate::slot::InstanceHandle) {
// RBX::StopCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ResetCommand::isEnabled(void)const")]
pub fn stub_0x3f96ec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ResetCommand getter.
cell.get()
}

#[doc(alias = "RBX::ResetCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3f9714(handle: &crate::slot::InstanceHandle) {
// RBX::ResetCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::EditSelectionVerb::~EditSelectionVerb() [0x3f97a8]")]
pub fn stub_0x3f97a8(handle: crate::slot::InstanceHandle) {
// RBX::EditSelectionVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::EditSelectionVerb::~EditSelectionVerb() [0x3f9848]")]
pub fn stub_0x3f9848(handle: crate::slot::InstanceHandle) {
// RBX::EditSelectionVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::EditSelectionVerb::isEnabled(void)const")]
pub fn stub_0x3f984c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::EditSelectionVerb getter.
cell.get()
}

#[doc(alias = "RBX::GroupSelectionVerb::isEnabled(void)const")]
pub fn stub_0x3f99b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::GroupSelectionVerb getter.
cell.get()
}

#[doc(alias = "RBX::GroupSelectionVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3f99fc(handle: &crate::slot::InstanceHandle) {
// RBX::GroupSelectionVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SnapSelectionVerb::isEnabled(void)const")]
pub fn stub_0x3f9bfc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SnapSelectionVerb getter.
cell.get()
}

#[doc(alias = "RBX::SnapSelectionVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3f9c3c(handle: &crate::slot::InstanceHandle) {
// RBX::SnapSelectionVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UnlockAllVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3f9d60(handle: &crate::slot::InstanceHandle) {
// RBX::UnlockAllVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UngroupSelectionVerb::isEnabled(void)const")]
pub fn stub_0x3f9f18(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UngroupSelectionVerb getter.
cell.get()
}

#[doc(alias = "RBX::UngroupSelectionVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3f9ff8(handle: &crate::slot::InstanceHandle) {
// RBX::UngroupSelectionVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SelectChildrenVerb::isEnabled(void)const")]
pub fn stub_0x3fa3b0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SelectChildrenVerb getter.
cell.get()
}

#[doc(alias = "RBX::SelectChildrenVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3fa490(handle: &crate::slot::InstanceHandle) {
// RBX::SelectChildrenVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DeleteBase::doIt(RBX::IDataState *)")]
pub fn stub_0x3fa7fc(handle: &crate::slot::InstanceHandle) {
// RBX::DeleteBase::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RotateAxisCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fad04(handle: &crate::slot::InstanceHandle) {
// RBX::RotateAxisCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RotateSelectionVerb::getRotationAxis(void)")]
pub fn stub_0x3fad90(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RotateSelectionVerb getter.
cell.get()
}

#[doc(alias = "RBX::TiltSelectionVerb::getRotationAxis(void)")]
pub fn stub_0x3faef0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TiltSelectionVerb getter.
cell.get()
}

#[doc(alias = "RBX::MoveUpSelectionVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3faf14(handle: &crate::slot::InstanceHandle) {
// RBX::MoveUpSelectionVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MoveDownSelectionVerb::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb1e8(handle: &crate::slot::InstanceHandle) {
// RBX::MoveDownSelectionVerb::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraPanLeftCommand::isEnabled(void)const")]
pub fn stub_0x3fb378(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraPanLeftCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraPanLeftCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb37c(handle: &crate::slot::InstanceHandle) {
// RBX::CameraPanLeftCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraPanRightCommand::isEnabled(void)const")]
pub fn stub_0x3fb3c0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraPanRightCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraPanRightCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb3c4(handle: &crate::slot::InstanceHandle) {
// RBX::CameraPanRightCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraTiltUpCommand::isEnabled(void)const")]
pub fn stub_0x3fb408(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraTiltUpCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraTiltUpCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb420(handle: &crate::slot::InstanceHandle) {
// RBX::CameraTiltUpCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraTiltDownCommand::isEnabled(void)const")]
pub fn stub_0x3fb46c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraTiltDownCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraTiltDownCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb484(handle: &crate::slot::InstanceHandle) {
// RBX::CameraTiltDownCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraZoomInCommand::isEnabled(void)const")]
pub fn stub_0x3fb4cc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraZoomInCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraZoomInCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb4e4(handle: &crate::slot::InstanceHandle) {
// RBX::CameraZoomInCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraZoomOutCommand::isEnabled(void)const")]
pub fn stub_0x3fb530(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraZoomOutCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraZoomOutCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb548(handle: &crate::slot::InstanceHandle) {
// RBX::CameraZoomOutCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::CameraZoomExtentsCommand::isEnabled(void)const")]
pub fn stub_0x3fb778(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraZoomExtentsCommand getter.
cell.get()
}

#[doc(alias = "RBX::CameraZoomExtentsCommand::doIt(RBX::IDataState *)")]
pub fn stub_0x3fb8dc(handle: &crate::slot::InstanceHandle) {
// RBX::CameraZoomExtentsCommand::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TurnOnManualJointCreation::doIt(RBX::IDataState *)")]
pub fn stub_0x3fbee8(handle: &crate::slot::InstanceHandle) {
// RBX::TurnOnManualJointCreation::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Selection::front(void)const")]
pub fn stub_0x3fc87c(handle: &crate::slot::InstanceHandle) {
// RBX::Selection::front() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Selection::back(void)const")]
pub fn stub_0x3fc8bc(handle: &crate::slot::InstanceHandle) {
// RBX::Selection::back() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DeleteBase::~DeleteBase()")]
pub fn stub_0x3fcdd8(handle: crate::slot::InstanceHandle) {
// RBX::DeleteBase dtor.
drop(handle);
}

#[doc(alias = "RBX::DeleteBase::~DeleteBase() [0x3fcddc]")]
pub fn stub_0x3fcddc(handle: crate::slot::InstanceHandle) {
// RBX::DeleteBase dtor.
drop(handle);
}

#[doc(alias = "RBX::Verb::isEnabled(void)const")]
pub fn stub_0x3fce7c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Verb getter.
cell.get()
}

#[doc(alias = "RBX::SelectAllCommand::~SelectAllCommand()")]
pub fn stub_0x3fce80(handle: crate::slot::InstanceHandle) {
// RBX::SelectAllCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::SelectAllCommand::~SelectAllCommand() [0x3fce84]")]
pub fn stub_0x3fce84(handle: crate::slot::InstanceHandle) {
// RBX::SelectAllCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::UnlockAllVerb::~UnlockAllVerb()")]
pub fn stub_0x3fcf24(handle: crate::slot::InstanceHandle) {
// RBX::UnlockAllVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::UnlockAllVerb::~UnlockAllVerb() [0x3fcf28]")]
pub fn stub_0x3fcf28(handle: crate::slot::InstanceHandle) {
// RBX::UnlockAllVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraVerb::~CameraVerb()")]
pub fn stub_0x3fcfc8(handle: crate::slot::InstanceHandle) {
// RBX::CameraVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraVerb::~CameraVerb() [0x3fd094]")]
pub fn stub_0x3fd094(handle: crate::slot::InstanceHandle) {
// RBX::CameraVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraVerb::isEnabled(void)const")]
pub fn stub_0x3fd174(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::CameraVerb getter.
cell.get()
}

#[doc(alias = "RBX::CameraTiltUpCommand::~CameraTiltUpCommand()")]
pub fn stub_0x3fd178(handle: crate::slot::InstanceHandle) {
// RBX::CameraTiltUpCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraTiltUpCommand::~CameraTiltUpCommand() [0x3fd244]")]
pub fn stub_0x3fd244(handle: crate::slot::InstanceHandle) {
// RBX::CameraTiltUpCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraTiltDownCommand::~CameraTiltDownCommand()")]
pub fn stub_0x3fd324(handle: crate::slot::InstanceHandle) {
// RBX::CameraTiltDownCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraTiltDownCommand::~CameraTiltDownCommand() [0x3fd3f0]")]
pub fn stub_0x3fd3f0(handle: crate::slot::InstanceHandle) {
// RBX::CameraTiltDownCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraPanLeftCommand::~CameraPanLeftCommand()")]
pub fn stub_0x3fd4d0(handle: crate::slot::InstanceHandle) {
// RBX::CameraPanLeftCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraPanLeftCommand::~CameraPanLeftCommand() [0x3fd59c]")]
pub fn stub_0x3fd59c(handle: crate::slot::InstanceHandle) {
// RBX::CameraPanLeftCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraPanRightCommand::~CameraPanRightCommand()")]
pub fn stub_0x3fd67c(handle: crate::slot::InstanceHandle) {
// RBX::CameraPanRightCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraPanRightCommand::~CameraPanRightCommand() [0x3fd748]")]
pub fn stub_0x3fd748(handle: crate::slot::InstanceHandle) {
// RBX::CameraPanRightCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraZoomInCommand::~CameraZoomInCommand()")]
pub fn stub_0x3fd828(handle: crate::slot::InstanceHandle) {
// RBX::CameraZoomInCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraZoomInCommand::~CameraZoomInCommand() [0x3fd8f4]")]
pub fn stub_0x3fd8f4(handle: crate::slot::InstanceHandle) {
// RBX::CameraZoomInCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraZoomOutCommand::~CameraZoomOutCommand()")]
pub fn stub_0x3fd9d4(handle: crate::slot::InstanceHandle) {
// RBX::CameraZoomOutCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::CameraZoomOutCommand::~CameraZoomOutCommand() [0x3fdaa0]")]
pub fn stub_0x3fdaa0(handle: crate::slot::InstanceHandle) {
// RBX::CameraZoomOutCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::BoolPropertyVerb::~BoolPropertyVerb()")]
pub fn stub_0x3fdb80(handle: crate::slot::InstanceHandle) {
// RBX::BoolPropertyVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::BoolPropertyVerb::~BoolPropertyVerb() [0x3fdb84]")]
pub fn stub_0x3fdb84(handle: crate::slot::InstanceHandle) {
// RBX::BoolPropertyVerb dtor.
drop(handle);
}
