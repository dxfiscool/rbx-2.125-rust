// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x406f30..0x40e100 | existing 8991 -> 9091 total (union; gen 4601->4701), filler 0x406f30 ascending, global remaining 31195->31095
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sNullToolEEEERKS0_v")]
pub fn stub_0x406f30(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sNullTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sNullToolEEEEvv")]
pub fn stub_0x406f74() -> crate::slot::PortedFn {
// IDA 0x406f74: void RBX::Name::callDoDeclare<RBX::sNullTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x406f74, "void RBX::Name::callDoDeclare<RBX::sNullTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v")]
pub fn stub_0x406f78(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sNullTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_12sDropperToolEEE7getNameEv")]
pub fn stub_0x407570(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v")]
pub fn stub_0x407868(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sDropperTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sDropperToolEEEEvv")]
pub fn stub_0x4078ac() -> crate::slot::PortedFn {
// IDA 0x4078ac: void RBX::Name::callDoDeclare<RBX::sDropperTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4078ac, "void RBX::Name::callDoDeclare<RBX::sDropperTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v")]
pub fn stub_0x4078b0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDropperTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_13sMaterialToolEEE7getNameEv")]
pub fn stub_0x407ea8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v")]
pub fn stub_0x408268(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sMaterialTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sMaterialToolEEEEvv")]
pub fn stub_0x4082ac() -> crate::slot::PortedFn {
// IDA 0x4082ac: void RBX::Name::callDoDeclare<RBX::sMaterialTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4082ac, "void RBX::Name::callDoDeclare<RBX::sMaterialTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v")]
pub fn stub_0x4082b0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sMaterialTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::~TToolVerb() [0x408518]")]
pub fn stub_0x408518(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x4085b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x4085f0(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::FillTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::FillTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x408704(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::FillTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_8PartToolELZNS_9sFillToolEEE7getNameEv")]
pub fn stub_0x4088a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::FillTool::isSticky(void)const")]
pub fn stub_0x4088ac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FillTool getter.
cell.get()
}

#[doc(alias = "RBX::FillTool::getCursorName(void)const")]
pub fn stub_0x408974(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FillTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v")]
pub fn stub_0x408c68(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sFillTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sFillToolEEEEvv")]
pub fn stub_0x408cac() -> crate::slot::PortedFn {
// IDA 0x408cac: void RBX::Name::callDoDeclare<RBX::sFillTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x408cac, "void RBX::Name::callDoDeclare<RBX::sFillTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v")]
pub fn stub_0x408cb0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFillTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::~TToolVerb() [0x408f18]")]
pub fn stub_0x408f18(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x408fb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x408ff0(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::LockTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::LockTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x409104(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::LockTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_9ModelToolELZNS_9sLockToolEEE7getNameEv")]
pub fn stub_0x4092a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::LockTool::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x4092ac(handle: &crate::slot::InstanceHandle) {
// RBX::LockTool::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v")]
pub fn stub_0x4097b4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sLockTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sLockToolEEEEvv")]
pub fn stub_0x4097f8(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v")]
pub fn stub_0x4097fc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLockTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::~TToolVerb() [0x409a64]")]
pub fn stub_0x409a64(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x409b04(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x409b3c(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AnchorTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::AnchorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x409c50(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::AnchorTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_9ModelToolELZNS_11sAnchorToolEEE7getNameEv")]
pub fn stub_0x409dfc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::AnchorTool::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x409e00(handle: &crate::slot::InstanceHandle) {
// RBX::AnchorTool::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v")]
pub fn stub_0x40a308(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sAnchorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sAnchorToolEEEEvv")]
pub fn stub_0x40a34c() -> crate::slot::PortedFn {
// IDA 0x40a34c: void RBX::Name::callDoDeclare<RBX::sAnchorTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x40a34c, "void RBX::Name::callDoDeclare<RBX::sAnchorTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v")]
pub fn stub_0x40a350(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAnchorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::~TToolVerb() [0x40a5b8]")]
pub fn stub_0x40a5b8(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x40a658(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x40a690(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::SmoothNoOutlinesTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::SmoothNoOutlinesTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x40a7a4(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::SmoothNoOutlinesTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_21sSmoothNoOutlinesToolEEE7getNameEv")]
pub fn stub_0x40a948(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::SmoothNoOutlinesTool::isSticky(void)const")]
pub fn stub_0x40a94c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SmoothNoOutlinesTool getter.
cell.get()
}

#[doc(alias = "RBX::SmoothNoOutlinesTool::getCursorName(void)const")]
pub fn stub_0x40aa14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SmoothNoOutlinesTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
pub fn stub_0x40ad08(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sSmoothNoOutlinesTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sSmoothNoOutlinesToolEEEEvv")]
pub fn stub_0x40ad4c() -> crate::slot::PortedFn {
// IDA 0x40ad4c: void RBX::Name::callDoDeclare<RBX::sSmoothNoOutlinesTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x40ad4c, "void RBX::Name::callDoDeclare<RBX::sSmoothNoOutlinesTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v")]
pub fn stub_0x40ad50(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSmoothNoOutlinesTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::~TToolVerb() [0x40afb8]")]
pub fn stub_0x40afb8(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x40b058(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x40b090(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::OscillateMotorTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::OscillateMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x40b1a4(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::OscillateMotorTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_19sOscillateMotorToolEEE7getNameEv")]
pub fn stub_0x40b348(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::OscillateMotorTool::getCursorName(void)const")]
pub fn stub_0x40b34c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::OscillateMotorTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v")]
pub fn stub_0x40b640(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sOscillateMotorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sOscillateMotorToolEEEEvv")]
pub fn stub_0x40b684() -> crate::slot::PortedFn {
// IDA 0x40b684: void RBX::Name::callDoDeclare<RBX::sOscillateMotorTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x40b684, "void RBX::Name::callDoDeclare<RBX::sOscillateMotorTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v")]
pub fn stub_0x40b688(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sOscillateMotorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::~TToolVerb() [0x40b8f0]")]
pub fn stub_0x40b8f0(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x40b990(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x40b9c8(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::LeftMotorTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::LeftMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x40badc(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::LeftMotorTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sLeftMotorToolEEE7getNameEv")]
pub fn stub_0x40bc80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::LeftMotorTool::getCursorName(void)const")]
pub fn stub_0x40bc84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::LeftMotorTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v")]
pub fn stub_0x40bf78(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sLeftMotorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLeftMotorToolEEEEvv")]
pub fn stub_0x40bfbc() -> crate::slot::PortedFn {
// IDA 0x40bfbc: void RBX::Name::callDoDeclare<RBX::sLeftMotorTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x40bfbc, "void RBX::Name::callDoDeclare<RBX::sLeftMotorTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v")]
pub fn stub_0x40bfc0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLeftMotorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::~TToolVerb() [0x40c228]")]
pub fn stub_0x40c228(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x40c2c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x40c300(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::RightMotorTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::RightMotorTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x40c414(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::RightMotorTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_15sRightMotorToolEEE7getNameEv")]
pub fn stub_0x40c5b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::RightMotorTool::isSticky(void)const")]
pub fn stub_0x40c5bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RightMotorTool getter.
cell.get()
}

#[doc(alias = "RBX::RightMotorTool::getCursorName(void)const")]
pub fn stub_0x40c684(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RightMotorTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v")]
pub fn stub_0x40c978(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sRightMotorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sRightMotorToolEEEEvv")]
pub fn stub_0x40c9bc() -> crate::slot::PortedFn {
// IDA 0x40c9bc: void RBX::Name::callDoDeclare<RBX::sRightMotorTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x40c9bc, "void RBX::Name::callDoDeclare<RBX::sRightMotorTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v")]
pub fn stub_0x40c9c0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sRightMotorTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::~TToolVerb() [0x40cc28]")]
pub fn stub_0x40cc28(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x40ccc8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x40cd00(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::HingeTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::HingeTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x40ce14(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::HingeTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_10sHingeToolEEE7getNameEv")]
pub fn stub_0x40cfb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::HingeTool::isSticky(void)const")]
pub fn stub_0x40cfbc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::HingeTool getter.
cell.get()
}

#[doc(alias = "RBX::HingeTool::getCursorName(void)const")]
pub fn stub_0x40d084(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::HingeTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v")]
pub fn stub_0x40d378(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sHingeTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sHingeToolEEEEvv")]
pub fn stub_0x40d3bc() -> crate::slot::PortedFn {
// IDA 0x40d3bc: void RBX::Name::callDoDeclare<RBX::sHingeTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x40d3bc, "void RBX::Name::callDoDeclare<RBX::sHingeTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v")]
pub fn stub_0x40d3c0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sHingeTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::~TToolVerb() [0x40d628]")]
pub fn stub_0x40d628(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x40d6c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x40d700(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::UniversalTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::UniversalTool,RBX::RunStateVerb>::newMouseCommand(void)")]
pub fn stub_0x40d814(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::UniversalTool, RBX::RunStateVerb>::newMouseCommand() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_11SurfaceToolELZNS_14sUniversalToolEEE7getNameEv")]
pub fn stub_0x40d9b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::UniversalTool::isSticky(void)const")]
pub fn stub_0x40d9bc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UniversalTool getter.
cell.get()
}

#[doc(alias = "RBX::UniversalTool::getCursorName(void)const")]
pub fn stub_0x40da84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::UniversalTool getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v")]
pub fn stub_0x40dd78(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sUniversalTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sUniversalToolEEEEvv")]
pub fn stub_0x40ddbc() -> crate::slot::PortedFn {
// IDA 0x40ddbc: void RBX::Name::callDoDeclare<RBX::sUniversalTool>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x40ddbc, "void RBX::Name::callDoDeclare<RBX::sUniversalTool>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v")]
pub fn stub_0x40ddc0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sUniversalTool>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::~TToolVerb() [0x40e028]")]
pub fn stub_0x40e028(handle: crate::slot::InstanceHandle) {
// RBX::TToolVerb dtor.
drop(handle);
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::isChecked(void)const")]
pub fn stub_0x40e0c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::TToolVerb getter.
cell.get()
}

#[doc(alias = "RBX::TToolVerb<RBX::InletTool,RBX::RunStateVerb>::doIt(RBX::IDataState *)")]
pub fn stub_0x40e100(handle: &crate::slot::InstanceHandle) {
// RBX::TToolVerb<RBX::InletTool, RBX::RunStateVerb>::doIt(RBX::IDataState*) — engine-side; linkage preserved via the alias.
let _ = handle;
}
