// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 120)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x3bf284..0x3cc14c | existing 8241 -> 8361 total (filler 0x3bf284 ascending, global remaining 31417 -> 31297)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x3bf284 — __ZThn32_N3RBX9BevelMeshD0Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
// was: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BevelMesh::~BevelMesh() [0x3bf284]")]
pub fn stub_0x3bf284(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13DataModelMeshELZNS_10sBevelMeshEEE12getClassNameEv")]
pub fn stub_0x3bf328() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"DataModelMesh"
}

// 0x3bf350 — __ZThn36_N3RBX9BevelMeshD1Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
// was: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BevelMesh::~BevelMesh() [0x3bf350]")]
pub fn stub_0x3bf350(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3bf358 — __ZThn36_N3RBX9BevelMeshD0Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
// was: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BevelMesh::~BevelMesh() [0x3bf358]")]
pub fn stub_0x3bf358(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv")]
pub fn stub_0x3bf3fc() -> crate::slot::PortedFn {
// IDA 0x3bf3fc: void RBX::Name::callDoDeclare<RBX::sBevelMesh>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3bf3fc, "void RBX::Name::callDoDeclare<RBX::sBevelMesh>()")
}

// 0x3bf400 — __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v")]
pub fn stub_0x3bf400(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBevelMesh>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_163")]
pub fn stub_0x3bf868() -> crate::slot::PortedFn {
// IDA 0x3bf868: __GLOBAL__I_a_163.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3bf868, "__GLOBAL__I_a_163")
}

// 0x3bfd78 — __ZNK3RBX12BillboardGui14getStudsOffsetEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getStudsOffset(void)const")]
pub fn stub_0x3bfd78(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3bfdf0 — __ZNK3RBX12BillboardGui16getExtentsOffsetEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getExtentsOffset(void)const")]
pub fn stub_0x3bfdf0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3bfe68 — __ZNK3RBX12BillboardGui13getSizeOffsetEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getSizeOffset(void)const")]
pub fn stub_0x3bfe68(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3bfeb8 — __ZNK3RBX12BillboardGui7getSizeEv
// type: _QWORD *__fastcall(_QWORD *this, int)
// was: _QWORD *__fastcall(_QWORD *this, int)
#[doc(alias = "RBX::BillboardGui::getSize(void)const")]
pub fn stub_0x3bfeb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3bfec8 — __ZN3RBX12BillboardGui7setSizeENS_5UDim2E
// type: RBX::Instance *__fastcall(RBX::Instance *result, float, unsigned __int16, float, unsigned __int16)
// was: RBX::Instance *__fastcall(RBX::Instance *result, float, unsigned __int16, float, unsigned __int16)
#[doc(alias = "RBX::BillboardGui::setSize(RBX::UDim2)")]
pub fn stub_0x3bfec8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3bff3c — __ZN3RBX12BillboardGui10setEnabledEb
// type: int __fastcall(RBX::BillboardGui *this, int)
// was: int __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "RBX::BillboardGui::setEnabled(bool)")]
pub fn stub_0x3bff3c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3bff70 — __ZN3RBX12BillboardGui9setActiveEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
// was: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::BillboardGui::setActive(bool)")]
pub fn stub_0x3bff70(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3bff90 — __ZNK3RBX12BillboardGui14getAlwaysOnTopEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getAlwaysOnTop(void)const")]
pub fn stub_0x3bff90(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3bff9c — __ZN3RBX12BillboardGui14setAlwaysOnTopEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
// was: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "RBX::BillboardGui::setAlwaysOnTop(bool)")]
pub fn stub_0x3bff9c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3c01c4 — __ZN3RBX12BillboardGuiC1Ev
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::BillboardGui(void)")]
pub fn stub_0x3c01c4() -> crate::slot::InstanceHandle {
// RBX::BillboardGui ctor.
crate::slot::InstanceHandle::new("RBX::BillboardGui")
}

// 0x3c01c8 — __ZN3RBX12BillboardGuiC2Ev
// type: RBX::GuiLayerCollector *__fastcall(RBX::BillboardGui *this)
// was: RBX::GuiLayerCollector *__fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::BillboardGui(void) [0x3c01c8]")]
pub fn stub_0x3c01c8() -> crate::slot::InstanceHandle {
// RBX::BillboardGui ctor.
crate::slot::InstanceHandle::new("RBX::BillboardGui")
}

// 0x3c0474 — __ZN3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE
// type: _BYTE *__fastcall(_BYTE *result)
// was: _BYTE *__fastcall(_BYTE *result)
#[doc(alias = "RBX::BillboardGui::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x3c0474(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::onHeartbeat(RBX::Heartbeat const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c048c — __ZThn168_N3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int result)
// was: int __fastcall(int result)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x3c048c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x3c057c — __ZNK3RBX12BillboardGui7getPartEv
// type: void __fastcall(RBX::BillboardGui *this, int)
// was: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "RBX::BillboardGui::getPart(void)const")]
pub fn stub_0x3c057c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c0850 — __ZNK3RBX12BillboardGui22render3dSortedPositionEv
// type: void __fastcall(RBX::BillboardGui *this, int)
// was: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "RBX::BillboardGui::render3dSortedPosition(void)const")]
pub fn stub_0x3c0850(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::render3dSortedPosition(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c0a28 — __ZThn96_NK3RBX12BillboardGui22render3dSortedPositionEv
// type: void __fastcall(RBX::BillboardGui *this, int)
// was: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::render3dSortedPosition(void)const")]
pub fn stub_0x3c0a28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x3c0e98 — __ZN3RBX12BillboardGui7processERKNS_8GuiEventE
// type: unsigned int __fastcall(_QWORD *, int, _DWORD *, int)
// was: unsigned int __fastcall(_QWORD *, int, _DWORD *, int)
#[doc(alias = "RBX::BillboardGui::process(RBX::GuiEvent const&)")]
pub fn stub_0x3c0e98(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::process(RBX::GuiEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c0f34 — __ZThn92_N3RBX12BillboardGui7processERKNS_8GuiEventE
// type: unsigned int __fastcall(_QWORD *, int, _DWORD *, int)
// was: unsigned int __fastcall(_QWORD *, int, _DWORD *, int)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::process(RBX::GuiEvent const&)")]
pub fn stub_0x3c0f34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "RBX::BillboardGui::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x3c0f40(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c1014 — __ZNK3RBX12BillboardGui10getEnabledEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getEnabled(void)const")]
pub fn stub_0x3c1014(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c1040 — __ZNK3RBX12BillboardGui9getActiveEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getActive(void)const")]
pub fn stub_0x3c1040(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c1048 — __ZNK3RBX12BillboardGui19getPlayerToHideFromEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getPlayerToHideFrom(void)const")]
pub fn stub_0x3c1048(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c11f8 — __ZN3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
pub fn stub_0x3c11f8(handle: crate::slot::InstanceHandle) {
// RBX::BillboardGui dtor.
drop(handle);
}

// 0x3c11fc — __ZN3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "RBX::BillboardGui::~BillboardGui() [0x3c11fc]")]
pub fn stub_0x3c11fc(handle: crate::slot::InstanceHandle) {
// RBX::BillboardGui dtor.
drop(handle);
}

// 0x3c129c — __ZN3RBX12BillboardGui17onServiceProviderEPNS_15ServiceProviderES2_
// type: int __fastcall(RBX::BillboardGui *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
// was: int __fastcall(RBX::BillboardGui *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::BillboardGui::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x3c129c(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c12b4 — __ZNK3RBX12BillboardGui26canProcessMeAndDescendantsEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::canProcessMeAndDescendants(void)const")]
pub fn stub_0x3c12b4(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::canProcessMeAndDescendants(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c12bc — __ZThn32_N3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui()")]
pub fn stub_0x3c12bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c12c4 — __ZThn32_N3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui() [0x3c12c4]")]
pub fn stub_0x3c12c4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c1378 — __ZThn36_N3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui() [0x3c1378]")]
pub fn stub_0x3c1378(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c1380 — __ZThn36_N3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui() [0x3c1380]")]
pub fn stub_0x3c1380(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c1428 — __ZThn168_N3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui() [0x3c1428]")]
pub fn stub_0x3c1428(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c1430 — __ZThn168_N3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui() [0x3c1430]")]
pub fn stub_0x3c1430(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c2b5c — __ZN3RBX12BillboardGuiD2Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
// was: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "RBX::BillboardGui::~BillboardGui() [0x3c2b5c]")]
pub fn stub_0x3c2b5c(handle: crate::slot::InstanceHandle) {
// RBX::BillboardGui dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_164")]
pub fn stub_0x3c2d40() -> crate::slot::PortedFn {
// IDA 0x3c2d40: __GLOBAL__I_a_164.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3c2d40, "__GLOBAL__I_a_164")
}

#[doc(alias = "global constructor keyed to_a_165")]
pub fn stub_0x3c333c() -> crate::slot::PortedFn {
// IDA 0x3c333c: __GLOBAL__I_a_165.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3c333c, "__GLOBAL__I_a_165")
}

// 0x3c3510 — __ZN3RBX6Camera13setCameraTypeENS0_10CameraTypeE
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::Camera::setCameraType(RBX::Camera::CameraType)")]
pub fn stub_0x3c3510(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c3820 — __ZN3RBX6Camera21setFieldOfViewDegreesEf
// type: void __fastcall(RBX::Camera *this, float32_t)
// was: void __fastcall(RBX::Camera *this, float32_t)
#[doc(alias = "RBX::Camera::setFieldOfViewDegrees(float)")]
pub fn stub_0x3c3820(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c3cb4 — __ZN3RBX6Camera7setRollEf
// type: void __fastcall(RBX::Camera *this, float)
// was: void __fastcall(RBX::Camera *this, float)
#[doc(alias = "RBX::Camera::setRoll(float)")]
pub fn stub_0x3c3cb4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c3da4 — __ZN3RBX6Camera11getRollSlowEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getRollSlow(void)")]
pub fn stub_0x3c3da4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c3dac — __ZN3RBX6Camera16setCameraPanModeENS0_13CameraPanModeE
// type: int __fastcall(int result, int)
// was: int __fastcall(int result, int)
#[doc(alias = "RBX::Camera::setCameraPanMode(RBX::Camera::CameraPanMode)")]
pub fn stub_0x3c3dac(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c3db4 — __ZN3RBX6Camera4zoomEf
// type: int __fastcall(RBX::Camera *this, const RBX::Instance *)
// was: int __fastcall(RBX::Camera *this, const RBX::Instance *)
#[doc(alias = "RBX::Camera::zoom(float)")]
pub fn stub_0x3c3db4(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::zoom(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c3e64 — __ZN3RBX6Camera8panUnitsEi
// type: int __fastcall(RBX::Camera *this, int)
// was: int __fastcall(RBX::Camera *this, int)
#[doc(alias = "RBX::Camera::panUnits(int)")]
pub fn stub_0x3c3e64(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::panUnits(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c3f04 — __ZN3RBX6Camera9tiltUnitsEi
// type: int __fastcall(RBX::Camera *this, int)
// was: int __fastcall(RBX::Camera *this, int)
#[doc(alias = "RBX::Camera::tiltUnits(int)")]
pub fn stub_0x3c3f04(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::tiltUnits(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c493c — __ZN3RBX15StringConverterINS_6Camera13CameraPanModeEE14convertToValueERKSsRS2_
// type: int __fastcall(std::string *, _DWORD *)
// was: int __fastcall(std::string *, _DWORD *)
#[doc(alias = "RBX::StringConverter<RBX::Camera::CameraPanMode>::convertToValue(std::string const&,RBX::Camera::CameraPanMode&)")]
pub fn stub_0x3c493c(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Camera::CameraPanMode>::convertToValue(std::string const&,RBX::C~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c4988 — __ZN3RBX6CameraC1Ev
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::Camera(void)")]
pub fn stub_0x3c4988() -> crate::slot::InstanceHandle {
// RBX::Camera ctor.
crate::slot::InstanceHandle::new("RBX::Camera")
}

// 0x3c498c — __ZN3RBX6CameraC2Ev
// type: RBX::Instance *__fastcall(RBX::Camera *this)
// was: RBX::Instance *__fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::Camera(void) [0x3c498c]")]
pub fn stub_0x3c498c() -> crate::slot::InstanceHandle {
// RBX::Camera ctor.
crate::slot::InstanceHandle::new("RBX::Camera")
}

// 0x3c4ecc — __ZN3RBX6Camera18getNewZoomDistanceEff
// type: __int32 __fastcall(RBX::Camera *this, float32_t, float)
// was: __int32 __fastcall(RBX::Camera *this, float32_t, float)
#[doc(alias = "RBX::Camera::getNewZoomDistance(float,float)")]
pub fn stub_0x3c4ecc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c4f24 — __ZNK3RBX6Camera17isCharacterCameraEv
// type: unsigned int __fastcall(RBX::Camera *this)
// was: unsigned int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::isCharacterCamera(void)const")]
pub fn stub_0x3c4f24(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c4f48 — __ZNK3RBX6Camera19isFirstPersonCameraEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::isFirstPersonCamera(void)const")]
pub fn stub_0x3c4f48(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c5544 — __ZNK3RBX6Camera21isLockedToFirstPersonEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::isLockedToFirstPerson(void)const")]
pub fn stub_0x3c5544(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c5588 — __ZN3RBX6Camera11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x3c5588(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::onHeartbeat(RBX::Heartbeat const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c5688 — __ZN3RBX6Camera16getCameraSubjectEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getCameraSubject(void)")]
pub fn stub_0x3c5688(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c5690 — __ZN3RBX6Camera27fixedSpeedInterpolateCameraEd
// type: int __fastcall(RBX::Camera *this, double)
// was: int __fastcall(RBX::Camera *this, double)
#[doc(alias = "RBX::Camera::fixedSpeedInterpolateCamera(double)")]
pub fn stub_0x3c5690(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::fixedSpeedInterpolateCamera(double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c58a8 — __ZThn92_N3RBX6Camera11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "non-virtual thunk toRBX::Camera::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x3c58a8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x3c58b0 — __ZN3RBX6Camera14getCameraOwnerEv
// type: void *__fastcall(RBX::Camera *this)
// was: void *__fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getCameraOwner(void)")]
pub fn stub_0x3c58b0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c58e8 — __ZN3RBX6Camera22pushCameraHistoryStackEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::pushCameraHistoryStack(void)")]
pub fn stub_0x3c58e8(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::pushCameraHistoryStack(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c5a7c — __ZN3RBX6Camera21popCameraHistoryStackEb
// type: void __fastcall(RBX::Camera *this, int *, int)
// was: void __fastcall(RBX::Camera *this, int *, int)
#[doc(alias = "RBX::Camera::popCameraHistoryStack(bool)")]
pub fn stub_0x3c5a7c(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::popCameraHistoryStack(bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c5cf4 — __ZN3RBX6Camera24stepCameraHistoryForwardEv
// type: void __fastcall(RBX::Camera *this)
// was: void __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::stepCameraHistoryForward(void)")]
pub fn stub_0x3c5cf4(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::stepCameraHistoryForward(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c5eac — __ZN3RBX6Camera25stepCameraHistoryBackwardEv
// type: void __fastcall(RBX::Camera *this)
// was: void __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::stepCameraHistoryBackward(void)")]
pub fn stub_0x3c5eac(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::stepCameraHistoryBackward(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c6064 — __ZN3RBX6Camera11updateFocusEv
// type: void __fastcall(RBX::Camera *this)
// was: void __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::updateFocus(void)")]
pub fn stub_0x3c6064(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::updateFocus(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c62bc — __ZN3RBX6Camera4stepEd
// type: void __fastcall(RBX::Camera *this, double)
// was: void __fastcall(RBX::Camera *this, double)
#[doc(alias = "RBX::Camera::step(double)")]
pub fn stub_0x3c62bc(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::step(double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c69a0 — __ZN3RBX6Camera10panRadiansEf
// type: int __fastcall(RBX::Camera *this, float32_t, int, float *)
// was: int __fastcall(RBX::Camera *this, float32_t, int, float *)
#[doc(alias = "RBX::Camera::panRadians(float)")]
pub fn stub_0x3c69a0(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::panRadians(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c6be8 — __ZN3RBX6Camera11tiltRadiansEf
// type: int __fastcall(RBX::Camera *this, float32_t, int, float *)
// was: int __fastcall(RBX::Camera *this, float32_t, int, float *)
#[doc(alias = "RBX::Camera::tiltRadians(float)")]
pub fn stub_0x3c6be8(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::tiltRadians(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c7374 — __ZN3RBX6Camera14tryZoomExtentsERKNS_7ExtentsE
// type: void __fastcall(RBX::Camera *this, const RBX::Extents *)
// was: void __fastcall(RBX::Camera *this, const RBX::Extents *)
#[doc(alias = "RBX::Camera::tryZoomExtents(RBX::Extents const&)")]
pub fn stub_0x3c7374(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::tryZoomExtents(RBX::Extents const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c75e4 — __ZN3RBX6Camera11zoomExtentsERKNS_7ExtentsENS0_8ZoomTypeE
// type: void __fastcall(int, const RBX::Extents *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
// was: void __fastcall(int, const RBX::Extents *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Camera::zoomExtents(RBX::Extents const&,RBX::Camera::ZoomType)")]
pub fn stub_0x3c75e4(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::zoomExtents(RBX::Extents const&,RBX::Camera::ZoomType) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c7a18 — __ZNK3RBX6Camera7canZoomEb
// type: bool __fastcall(RBX::Camera *this, int)
// was: bool __fastcall(RBX::Camera *this, int)
#[doc(alias = "RBX::Camera::canZoom(bool)const")]
pub fn stub_0x3c7a18(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::canZoom(bool)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c7a9c — __ZN3RBX6Camera21setDistanceFromTargetEf
// type: int __fastcall(RBX::Camera *this, float)
// was: int __fastcall(RBX::Camera *this, float)
#[doc(alias = "RBX::Camera::setDistanceFromTarget(float)")]
pub fn stub_0x3c7a9c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c7ab0 — __ZNK3RBX6Camera21getConstCameraSubjectEv
// type: void *__fastcall(RBX::Camera *this)
// was: void *__fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getConstCameraSubject(void)const")]
pub fn stub_0x3c7ab0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c7bd8 — __ZN3RBX6Camera16nonCharacterZoomEf
// type: int __fastcall(RBX::Camera *this, float32_t)
// was: int __fastcall(RBX::Camera *this, float32_t)
#[doc(alias = "RBX::Camera::nonCharacterZoom(float)")]
pub fn stub_0x3c7bd8(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::nonCharacterZoom(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c8060 — __ZN3RBX6Camera13characterZoomEf
// type: int __fastcall(RBX::Camera *this, float32_t)
// was: int __fastcall(RBX::Camera *this, float32_t)
#[doc(alias = "RBX::Camera::characterZoom(float)")]
pub fn stub_0x3c8060(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::characterZoom(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c8200 — __ZNK3RBX6Camera7canTiltEi
// type: bool __fastcall(RBX::Camera *this, int)
// was: bool __fastcall(RBX::Camera *this, int)
#[doc(alias = "RBX::Camera::canTilt(int)const")]
pub fn stub_0x3c8200(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::canTilt(int)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c82a4 — __ZN3RBX6Camera27setHeadingElevationDistanceEfff
// type: int __fastcall(RBX::Camera *this, G3D::CoordinateFrame *, float, float)
// was: int __fastcall(RBX::Camera *this, G3D::CoordinateFrame *, float, float)
#[doc(alias = "RBX::Camera::setHeadingElevationDistance(float,float,float)")]
pub fn stub_0x3c82a4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c8358 — __ZN3RBX6Camera16tiltSpeedRadiansEf
// type: int __fastcall(int this, float)
// was: int __fastcall(int this, float)
#[doc(alias = "RBX::Camera::tiltSpeedRadians(float)")]
pub fn stub_0x3c8358(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::tiltSpeedRadians(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c8360 — __ZN3RBX6Camera15panSpeedRadiansEf
// type: int __fastcall(int this, float)
// was: int __fastcall(int this, float)
#[doc(alias = "RBX::Camera::panSpeedRadians(float)")]
pub fn stub_0x3c8360(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::panSpeedRadians(float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c836c — __ZN3RBX6Camera5doFlyERKNS_7NavKeysEib
// type: void __fastcall(RBX::Camera *this, const RBX::NavKeys *, int, int)
// was: void __fastcall(RBX::Camera *this, const RBX::NavKeys *, int, int)
#[doc(alias = "RBX::Camera::doFly(RBX::NavKeys const&,int,bool)")]
pub fn stub_0x3c836c(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::doFly(RBX::NavKeys const&,int,bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c872c — __ZNK3RBX6Camera10nearPlaneZEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::nearPlaneZ(void)const")]
pub fn stub_0x3c872c(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::nearPlaneZ(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c89d8 — __ZNK3RBX6Camera15coordinateFrameEv
// type: char *__fastcall(RBX::Camera *this)
// was: char *__fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::coordinateFrame(void)const")]
pub fn stub_0x3c89d8(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::coordinateFrame(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c8be4 — __ZNK3RBX6Camera13getCameraTypeEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getCameraType(void)const")]
pub fn stub_0x3c8be4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c8c10 — __ZNK3RBX6Camera24getCameraCoordinateFrameEv
// type: char *__fastcall(RBX::Camera *this)
// was: char *__fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getCameraCoordinateFrame(void)const")]
pub fn stub_0x3c8c10(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c8c38 — __ZNK3RBX6Camera14getCameraFocusEv
// type: char *__fastcall(RBX::Camera *this)
// was: char *__fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getCameraFocus(void)const")]
pub fn stub_0x3c8c38(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c8c3c — __ZNK3RBX6Camera21getFieldOfViewDegreesEv
// type: float __fastcall(RBX::Camera *this)
// was: float __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getFieldOfViewDegrees(void)const")]
pub fn stub_0x3c8c3c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c8d14 — __ZN3RBX6Camera12getTiltSpeedEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getTiltSpeed(void)")]
pub fn stub_0x3c8d14(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c8d1c — __ZN3RBX6Camera11getPanSpeedEv
// type: int __fastcall(RBX::Camera *this)
// was: int __fastcall(RBX::Camera *this)
#[doc(alias = "RBX::Camera::getPanSpeed(void)")]
pub fn stub_0x3c8d1c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c9bf0 — __ZN3RBX9Tolerance10maxExtentsEv
// type: int *__fastcall(RBX::Tolerance *this)
// was: int *__fastcall(RBX::Tolerance *this)
#[doc(alias = "RBX::Tolerance::maxExtents(void)")]
pub fn stub_0x3c9bf0(handle: &crate::slot::InstanceHandle) {
// RBX::Tolerance::maxExtents(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c9c7c — __ZNK3RBX7NavKeys10navKeyDownEv
// type: int __fastcall(RBX::NavKeys *this)
// was: int __fastcall(RBX::NavKeys *this)
#[doc(alias = "RBX::NavKeys::navKeyDown(void)const")]
pub fn stub_0x3c9c7c(handle: &crate::slot::InstanceHandle) {
// RBX::NavKeys::navKeyDown(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c9cd0 — __ZN3RBX6CameraD1Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "RBX::Camera::~Camera()")]
pub fn stub_0x3c9cd0(handle: crate::slot::InstanceHandle) {
// RBX::Camera dtor.
drop(handle);
}

// 0x3c9cd4 — __ZN3RBX6CameraD0Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "RBX::Camera::~Camera() [0x3c9cd4]")]
pub fn stub_0x3c9cd4(handle: crate::slot::InstanceHandle) {
// RBX::Camera dtor.
drop(handle);
}

// 0x3c9d74 — __ZN3RBX6Camera17onServiceProviderEPNS_15ServiceProviderES2_
// type: int __fastcall(RBX::Camera *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
// was: int __fastcall(RBX::Camera *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Camera::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x3c9d74(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c9d7c — __ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")]
pub fn stub_0x3c9d7c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Camera"
}

// 0x3c9d8c — __ZThn32_N3RBX6CameraD1Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Camera::~Camera()")]
pub fn stub_0x3c9d8c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c9d94 — __ZThn32_N3RBX6CameraD0Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Camera::~Camera() [0x3c9d94]")]
pub fn stub_0x3c9d94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c9d9c — __ZThn32_NK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E12getClassNameEv")]
pub fn stub_0x3c9d9c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Camera"
}

// 0x3c9dac — __ZThn36_N3RBX6CameraD1Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Camera::~Camera() [0x3c9dac]")]
pub fn stub_0x3c9dac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c9db4 — __ZThn36_N3RBX6CameraD0Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Camera::~Camera() [0x3c9db4]")]
pub fn stub_0x3c9db4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c9dbc — __ZThn92_N3RBX6CameraD1Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Camera::~Camera() [0x3c9dbc]")]
pub fn stub_0x3c9dbc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3c9dc4 — __ZThn92_N3RBX6CameraD0Ev
// type: void __fastcall(RBX::Camera *__hidden this)
// was: void __fastcall(RBX::Camera *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Camera::~Camera() [0x3c9dc4]")]
pub fn stub_0x3c9dc4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3ca6b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera13CameraPanModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
// was: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraPanMode>(RBX::Camera::CameraPanMode const&)")]
pub fn stub_0x3ca6b0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x3ca700 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE9singletonEv
// type: _DWORD *()
// was: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::singleton(void)")]
pub fn stub_0x3ca700(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::singleton(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3ca76c — __ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
// was: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::construct_func(char const*,char *)")]
pub fn stub_0x3ca76c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::construct_func(char const*,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3ca778 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera13CameraPanModeEE13destruct_funcEPc
// type: void()
// was: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::destruct_func(char *)")]
pub fn stub_0x3ca778(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraPanMode>::destruct_func(char *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3ca848 — __ZN3rbx8any_castIRKN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraPanMode const& rbx::any_cast<RBX::Camera::CameraPanMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x3ca848(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x3cad28 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraModeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
// was: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraMode>(RBX::Camera::CameraMode const&)")]
pub fn stub_0x3cad28() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x3cad78 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE9singletonEv
// type: _DWORD *()
// was: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::singleton(void)")]
pub fn stub_0x3cad78(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraMode>::singleton(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cade4 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
// was: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::construct_func(char const*,char *)")]
pub fn stub_0x3cade4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraMode>::construct_func(char const*,cha~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cadf0 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraModeEE13destruct_funcEPc
// type: void()
// was: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraMode>::destruct_func(char *)")]
pub fn stub_0x3cadf0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraMode>::destruct_func(char *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3caec0 — __ZN3rbx8any_castIRKN3RBX6Camera10CameraModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraMode const& rbx::any_cast<RBX::Camera::CameraMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x3caec0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x3cb3a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Camera10CameraTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
// was: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Camera::CameraType>(RBX::Camera::CameraType const&)")]
pub fn stub_0x3cb3a0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x3cb3f0 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE9singletonEv
// type: _DWORD *()
// was: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void)")]
pub fn stub_0x3cb3f0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraType>::singleton(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cb45c — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
// was: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::construct_func(char const*,char *)")]
pub fn stub_0x3cb45c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraType>::construct_func(char const*,cha~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cb468 — __ZN3rbx14implementation12typed_holderIN3RBX6Camera10CameraTypeEE13destruct_funcEPc
// type: void()
// was: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Camera::CameraType>::destruct_func(char *)")]
pub fn stub_0x3cb468(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Camera::CameraType>::destruct_func(char *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cb538 — __ZN3rbx8any_castIRKN3RBX6Camera10CameraTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraType const& rbx::any_cast<RBX::Camera::CameraType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x3cb538(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x3cb878 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv
// type: void *()
// was: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E17static_getCreatorEv")]
pub fn stub_0x3cb878() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Camera"
}

// 0x3cc14c — __ZN3rbx8any_castIN3RBX6Camera13CameraPanModeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
// was: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Camera::CameraPanMode * rbx::any_cast<RBX::Camera::CameraPanMode,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x3cc14c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}
