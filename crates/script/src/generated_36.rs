// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 120)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x3bfd80..0x3d429c | existing 8581 -> 8701 total (filler 0x3bfd80 ascending, global remaining 52734 -> 52614)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x3bfd80 — __ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
// was: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
#[doc(alias = "RBX::BillboardGui::setStudsOffset(G3D::Vector3 const&)")]
pub fn stub_0x3bfd80(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3bfdf8 — __ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
// was: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
#[doc(alias = "RBX::BillboardGui::setExtentsOffset(G3D::Vector3 const&)")]
pub fn stub_0x3bfdf8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3bfe70 — __ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector2 *)
// was: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector2 *)
#[doc(alias = "RBX::BillboardGui::setSizeOffset(G3D::Vector2 const&)")]
pub fn stub_0x3bfe70(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3c042c — __ZN3RBX12BillboardGui17setRenderFunctionEN5boost8functionIFvPS0_PNS_5AdornEEEE
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::BillboardGui::setRenderFunction(boost::function<void ()(RBX::BillboardGui*,RBX::Adorn *)>)")]
pub fn stub_0x3c042c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::BillboardGui setter.
cell.set(value)
}

// 0x3c04a8 — __ZNK3RBX12BillboardGui25shouldRender3dSortedAdornEv
// type: bool __fastcall(RBX::BillboardGui *this)
// was: bool __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::shouldRender3dSortedAdorn(void)const")]
pub fn stub_0x3c04a8(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::shouldRender3dSortedAdorn(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c066c — __ZThn96_NK3RBX12BillboardGui25shouldRender3dSortedAdornEv
// type: bool __fastcall(RBX::BillboardGui *this)
// was: bool __fastcall(RBX::BillboardGui *this)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::shouldRender3dSortedAdorn(void)const")]
pub fn stub_0x3c066c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x3c0678 — __ZNK3RBX12BillboardGui13getModelAdornEv
// type: void __fastcall(RBX::BillboardGui *this, int)
// was: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "RBX::BillboardGui::getModelAdorn(void)const")]
pub fn stub_0x3c0678(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c0764 — __ZNK3RBX12BillboardGui12getPartAdornEv
// type: void __fastcall(RBX::BillboardGui *this, int)
// was: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "RBX::BillboardGui::getPartAdorn(void)const")]
pub fn stub_0x3c0764(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c0a34 — __ZN3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
// type: void __fastcall(RBX::BillboardGui *this, RBX::Adorn *)
// was: void __fastcall(RBX::BillboardGui *this, RBX::Adorn *)
#[doc(alias = "RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
pub fn stub_0x3c0a34(handle: &crate::slot::InstanceHandle) {
// RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c0e90 — __ZThn96_N3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
// type: void __fastcall(RBX::BillboardGui *this, RBX::Adorn *)
// was: void __fastcall(RBX::BillboardGui *this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
pub fn stub_0x3c0e90(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x3c0f58 — __ZNK3RBX12BillboardGui19getAdorneeDangerousEv
// type: int __fastcall(RBX::BillboardGui *this)
// was: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "RBX::BillboardGui::getAdorneeDangerous(void)const")]
pub fn stub_0x3c0f58(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c106c — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)> const&)")]
pub fn stub_0x3c106c() -> crate::slot::PortedFn {
// IDA 0x3c106c: boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::BillboardGui *,RBX::A~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3c106c, "boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::B~")
}

// 0x3c1130 — __ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_
// type: void __fastcall(_DWORD *, int, int)
// was: void __fastcall(_DWORD *, int, int)
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::operator()(RBX::BillboardGui *,RBX::Adorn *)const")]
pub fn stub_0x3c1130(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

// 0x3c12a4 — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3c12a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

// 0x3c12b8 — __ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
// type: int()
// was: int()
#[doc(alias = "RBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
pub fn stub_0x3c12b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BillboardGui getter.
cell.get()
}

// 0x3c1368 — __ZThn32_NK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv
// type: int()
// was: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3c1368() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

// 0x3c1424 — __ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
// type: int()
// was: int()
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
pub fn stub_0x3c1424(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x3c14d4 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD1Ev
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3c14d4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

// 0x3c14d8 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x3c14d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x3c1574() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

// 0x3c15fc — __ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x3c15fc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAdornmentGuiEEEEvv")]
pub fn stub_0x3c1740() -> crate::slot::PortedFn {
// IDA 0x3c1740: void RBX::Name::callDoDeclare<RBX::sAdornmentGui>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3c1740, "void RBX::Name::callDoDeclare<RBX::sAdornmentGui>()")
}

// 0x3c1744 — __ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v
// type: int()
// was: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v")]
pub fn stub_0x3c1744(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sAdornmentGui>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c1824 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
// was: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x3c1824() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

// 0x3c1a68 — __ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE17static_getCreatorEv
// type: void *()
// was: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BillboardGuiENS_17GuiLayerCollectorELZNS_13sAdornmentGuiEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3c1a68() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BillboardGui"
}

// 0x3c1adc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE5dummy7nonnullEv
// type: void()
// was: void()
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::dummy::nonnull(void)")]
pub fn stub_0x3c1adc() -> crate::slot::PortedFn {
// IDA 0x3c1adc: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::dummy::nonnull(void).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3c1adc, "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::dummy::nonnull(void)")
}

// 0x3c1ae0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_
// type: void __fastcall(int, int, int, int)
// was: void __fastcall(int, int, int, int)
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
pub fn stub_0x3c1ae0() -> crate::slot::PortedFn {
// IDA 0x3c1ae0: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3c1ae0, "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGu~")
}

// 0x3c1bbc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_
// type: void __fastcall(int, int *, int, int, void *, int)
// was: void __fastcall(int, int *, int, int, void *, int)
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
pub fn stub_0x3c1bbc() -> crate::slot::PortedFn {
// IDA 0x3c1bbc: boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::BillboardGui *,RBX::Ador~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3c1bbc, "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::Bill~")
}

// 0x3c1cc0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_
// type: int __fastcall(int result, int *)
// was: int __fastcall(int result, int *)
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to_own(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *> const&)")]
pub fn stub_0x3c1cc0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

// 0x3c35d4 — __ZN3RBX6Camera24setCameraCoordinateFrameERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
// was: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Camera::setCameraCoordinateFrame(G3D::CoordinateFrame const&)")]
pub fn stub_0x3c35d4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c3738 — __ZN3RBX6Camera14setCameraFocusERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
// was: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Camera::setCameraFocus(G3D::CoordinateFrame const&)")]
pub fn stub_0x3c3738(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c3fa4 — __ZN3RBX6Camera24beginCameraInterpolationEN3G3D15CoordinateFrameES2_f
// type: void __fastcall(int, __int64 *, __int64 *, float)
// was: void __fastcall(int, __int64 *, __int64 *, float)
#[doc(alias = "RBX::Camera::beginCameraInterpolation(G3D::CoordinateFrame,G3D::CoordinateFrame,float)")]
pub fn stub_0x3c3fa4(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::beginCameraInterpolation(G3D::CoordinateFrame,G3D::CoordinateFrame,float) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c51c0 — __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DE
// type: void __fastcall(int, int, int)
// was: void __fastcall(int, int, int)
#[doc(alias = "RBX::Camera::frustum(G3D::Rect2D const&)const")]
pub fn stub_0x3c51c0(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::frustum(G3D::Rect2D const&)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c5284 — __ZNK3RBX6Camera22getNearViewportCornersERKN3G3D6Rect2DERNS1_7Vector3ES6_S6_S6_
// type: int __fastcall(int result, __int32 *, __int32 *, __int32 *, __int32 *, __int32 *)
// was: int __fastcall(int result, __int32 *, __int32 *, __int32 *, __int32 *, __int32 *)
#[doc(alias = "RBX::Camera::getNearViewportCorners(G3D::Rect2D const&,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
pub fn stub_0x3c5284(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c6144 — __ZN3RBX6Camera35setCameraFocusWithoutPropertyChangeERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
// was: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Camera::setCameraFocusWithoutPropertyChange(G3D::CoordinateFrame const&)")]
pub fn stub_0x3c6144(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c6210 — __ZN3RBX6Camera18setCameraFocusOnlyERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
// was: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Camera::setCameraFocusOnly(G3D::CoordinateFrame const&)")]
pub fn stub_0x3c6210(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c690c — __ZN3RBX6Camera39setCameraFocusOnlyWithoutPropertyChangeERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
// was: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Camera::setCameraFocusOnlyWithoutPropertyChange(G3D::CoordinateFrame const&)")]
pub fn stub_0x3c690c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c6d3c — __ZN3RBX6Camera21setDistanceFromTargetEfRN3G3D15CoordinateFrameERKS2_
// type: int __fastcall(RBX::Camera *this, float, G3D::CoordinateFrame *, const G3D::CoordinateFrame *)
// was: int __fastcall(RBX::Camera *this, float, G3D::CoordinateFrame *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Camera::setDistanceFromTarget(float,G3D::CoordinateFrame &,G3D::CoordinateFrame const&)")]
pub fn stub_0x3c6d3c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c6e7c — __ZN3RBX6Camera13lerpToExtentsERKNS_7ExtentsERKN3G3D6Rect2DE
// type: void __fastcall(int, RBX::Extents *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
// was: void __fastcall(int, RBX::Extents *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Camera::lerpToExtents(RBX::Extents const&,G3D::Rect2D const&)")]
pub fn stub_0x3c6e7c(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::lerpToExtents(RBX::Extents const&,G3D::Rect2D const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c79f4 — __ZN3RBX6Camera11zoomExtentsERKN3G3D6Rect2DE
// type: int __fastcall(RBX::Camera *)
// was: int __fastcall(RBX::Camera *)
#[doc(alias = "RBX::Camera::zoomExtents(G3D::Rect2D const&)")]
pub fn stub_0x3c79f4(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::zoomExtents(G3D::Rect2D const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c7b34 — __ZN3RBX6Camera30setCameraFocusAndMaintainFocusERKN3G3D15CoordinateFrameEb
// type: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *, bool)
// was: int __fastcall(RBX::Camera *this, const G3D::CoordinateFrame *, bool)
#[doc(alias = "RBX::Camera::setCameraFocusAndMaintainFocus(G3D::CoordinateFrame const&,bool)")]
pub fn stub_0x3c7b34(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Camera setter.
cell.set(value)
}

// 0x3c7b48 — __ZN3RBX6Camera16legalCameraCoordERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Math *, const G3D::CoordinateFrame *)
// was: int __fastcall(RBX::Math *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Camera::legalCameraCoord(G3D::CoordinateFrame const&)")]
pub fn stub_0x3c7b48(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::legalCameraCoord(G3D::CoordinateFrame const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c8734 — __ZNK3RBX6Camera18getImagePlaneDepthERKN3G3D6Rect2DE
// type: unsigned __int32 __fastcall(int, int)
// was: unsigned __int32 __fastcall(int, int)
#[doc(alias = "RBX::Camera::getImagePlaneDepth(G3D::Rect2D const&)const")]
pub fn stub_0x3c8734(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Camera getter.
cell.get()
}

// 0x3c8750 — __ZNK3RBX6Camera7projectERKN3G3D7Vector3ERKNS1_6Rect2DE
// type: int *__fastcall(int *result, _DWORD *, __int32 *, __int32 *)
// was: int *__fastcall(int *result, _DWORD *, __int32 *, __int32 *)
#[doc(alias = "RBX::Camera::project(G3D::Vector3 const&,G3D::Rect2D const&)const")]
pub fn stub_0x3c8750(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::project(G3D::Vector3 const&,G3D::Rect2D const&)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c8888 — __ZNK3RBX6Camera8worldRayEffRKN3G3D6Rect2DE
// type: int __fastcall(__int64, __int32, __int32, __int32 *)
// was: int __fastcall(__int64, __int32, __int32, __int32 *)
#[doc(alias = "RBX::Camera::worldRay(float,float,G3D::Rect2D const&)const")]
pub fn stub_0x3c8888(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::worldRay(float,float,G3D::Rect2D const&)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c89dc — __ZNK3RBX6Camera3dotERKN3G3D7Vector3E
// type: unsigned __int32 __fastcall(RBX::Camera *this, const G3D::Vector3 *)
// was: unsigned __int32 __fastcall(RBX::Camera *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Camera::dot(G3D::Vector3 const&)const")]
pub fn stub_0x3c89dc(a: &crate::lua::LuaVector3, b: &crate::lua::LuaVector3) -> f32 {
// G3D::Vector3::dot.
a.x * b.x + a.y * b.y + a.z * b.z
}

// 0x3c8a58 — __ZNK3RBX6Camera7frustumERKN3G3D6Rect2DEfRNS_7FrustumE
// type: void __fastcall(float *, __int32 *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int)
// was: void __fastcall(float *, __int32 *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Camera::frustum(G3D::Rect2D const&,float,RBX::Frustum &)const")]
pub fn stub_0x3c8a58(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::frustum(G3D::Rect2D const&,float,RBX::Frustum &)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3c9acc — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE2atEm
// type: int __fastcall(int *, unsigned int)
// was: int __fastcall(int *, unsigned int)
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::at(unsigned long)")]
pub fn stub_0x3c9acc(vec: &crate::slot::VecModel, index: usize) -> Option<usize> {
// bounds-checked element access shape.
if index < vec.len() { Some(index) } else { None }
}

// 0x3c9b00 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE6insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::insert(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
pub fn stub_0x3c9b00() -> crate::slot::PortedFn {
// IDA 0x3c9b00: std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::Coor~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3c9b00, "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::Coord~")
}

// 0x3c9b48 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE9push_backERKS3_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
// was: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::push_back(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
pub fn stub_0x3c9b48(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x3c9b80 — __ZNK3RBX7Extents8containsERKN3G3D7Vector3E
// type: bool __fastcall(RBX::Extents *this, const Vector3 *)
// was: bool __fastcall(RBX::Extents *this, const Vector3 *)
#[doc(alias = "RBX::Extents::contains(G3D::Vector3 const&)const")]
pub fn stub_0x3c9b80(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::contains(G3D::Vector3 const&)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cba0c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_
// type: int __fastcall(__int64 *, int, int)
// was: int __fastcall(__int64 *, int, int)
#[doc(alias = "std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy<false,std::random_access_iterator_tag>::copy<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)")]
pub fn stub_0x3cba0c(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

// 0x3cbab4 — __ZN9__gnu_cxx13new_allocatorISt4pairIN3G3D15CoordinateFrameES3_EE9constructEPS4_RKS4_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>::construct(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
pub fn stub_0x3cbab4() -> crate::slot::PortedFn {
// IDA 0x3cbab4: __gnu_cxx::new_allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>::construct(std::pair<G3D::CoordinateFrame~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3cbab4, "__gnu_cxx::new_allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>::construct(std::pair<~")
}

// 0x3cbaf0 — __ZNSt6vectorISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void __fastcall(int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
// was: void __fastcall(int *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int)
#[doc(alias = "std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>*,std::vector<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>>,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> const&)")]
pub fn stub_0x3cbaf0(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x3cbe40 — __ZNSt12_Vector_baseISt4pairIN3G3D15CoordinateFrameES2_ESaIS3_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>,std::allocator<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame>>>::_M_allocate(unsigned long)")]
pub fn stub_0x3cbe40() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x3cbe64 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIN3G3D15CoordinateFrameES5_ES7_EET0_T_S9_S8_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *>(std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *,std::pair<G3D::CoordinateFrame,G3D::CoordinateFrame> *)")]
pub fn stub_0x3cbe64(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

// 0x3cc1a4 — __ZN3rbx8any_castIRN3RBX6Camera13CameraPanModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: char ****__fastcall(char ****)
#[doc(alias = "RBX::Camera::CameraPanMode & rbx::any_cast<RBX::Camera::CameraPanMode &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x3cc1a4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x3cc294 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
// was: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::resize(unsigned long,RBX::Camera::CameraPanMode)")]
pub fn stub_0x3cc294(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

// 0x3cc2c8 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
// was: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::push_back(RBX::Camera::CameraPanMode const&)")]
pub fn stub_0x3cc2c8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x3cc2f0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera13CameraPanModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
// was: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraPanMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x3cc2f0(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x3cc348 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
// was: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
pub fn stub_0x3cc348(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cc3fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
// was: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
pub fn stub_0x3cc3fc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cc454 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
// was: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode> const&)")]
pub fn stub_0x3cc454(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cc4bc — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
// was: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,RBX::Camera::CameraPanMode const&)")]
pub fn stub_0x3cc4bc(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x3cc5a0 — __ZNSt12_Vector_baseIN3RBX6Camera13CameraPanModeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_allocate(unsigned long)")]
pub fn stub_0x3cc5a0() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x3cc5b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera13CameraPanModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *>(RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *,RBX::Camera::CameraPanMode *)")]
pub fn stub_0x3cc5b8(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::CameraPanMode * std::__copy_backward<false,std::random_access_iterator_tag>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cc5f4 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
// was: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraPanMode*,std::vector<RBX::Camera::CameraPanMode,std::allocator<RBX::Camera::CameraPanMode>>>,unsigned long,RBX::Camera::CameraPanMode const&)")]
pub fn stub_0x3cc5f4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

// 0x3cc784 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
// was: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::resize(unsigned long,RBX::Camera::CameraMode)")]
pub fn stub_0x3cc784(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

// 0x3cc7b8 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
// was: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::push_back(RBX::Camera::CameraMode const&)")]
pub fn stub_0x3cc7b8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x3cc7e0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
// was: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x3cc7e0(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x3cc838 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
// was: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
pub fn stub_0x3cc838(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cc8ec — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
// was: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
pub fn stub_0x3cc8ec(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cc944 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
// was: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraMode> const&)")]
pub fn stub_0x3cc944(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cc9ac — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
// was: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,RBX::Camera::CameraMode const&)")]
pub fn stub_0x3cc9ac(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x3cca90 — __ZNSt12_Vector_baseIN3RBX6Camera10CameraModeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_allocate(unsigned long)")]
pub fn stub_0x3cca90() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x3ccaa8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraModeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Camera::CameraMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraMode *,RBX::Camera::CameraMode *>(RBX::Camera::CameraMode *,RBX::Camera::CameraMode *,RBX::Camera::CameraMode *)")]
pub fn stub_0x3ccaa8(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::CameraMode * std::__copy_backward<false,std::random_access_iterator_tag>::__c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3ccae4 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
// was: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraMode*,std::vector<RBX::Camera::CameraMode,std::allocator<RBX::Camera::CameraMode>>>,unsigned long,RBX::Camera::CameraMode const&)")]
pub fn stub_0x3ccae4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

// 0x3ccc74 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
// was: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::resize(unsigned long,RBX::Camera::CameraType)")]
pub fn stub_0x3ccc74(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

// 0x3ccca8 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
// was: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::push_back(RBX::Camera::CameraType const&)")]
pub fn stub_0x3ccca8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

// 0x3cccd0 — __ZNSt3mapIPKN3RBX4NameENS0_6Camera10CameraTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
// was: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::Camera::CameraType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x3cccd0(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

// 0x3ccd28 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
// was: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
pub fn stub_0x3ccd28(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3ccddc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
// was: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
pub fn stub_0x3ccddc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cce34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
// was: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Camera::CameraType> const&)")]
pub fn stub_0x3cce34(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0x3cce9c — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
// was: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,RBX::Camera::CameraType const&)")]
pub fn stub_0x3cce9c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

// 0x3ccf80 — __ZNSt12_Vector_baseIN3RBX6Camera10CameraTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
// was: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_allocate(unsigned long)")]
pub fn stub_0x3ccf80() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

// 0x3ccf98 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Camera10CameraTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
// was: int __fastcall(int, int, int)
#[doc(alias = "RBX::Camera::CameraType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Camera::CameraType *,RBX::Camera::CameraType *>(RBX::Camera::CameraType *,RBX::Camera::CameraType *,RBX::Camera::CameraType *)")]
pub fn stub_0x3ccf98(handle: &crate::slot::InstanceHandle) {
// RBX::Camera::CameraType * std::__copy_backward<false,std::random_access_iterator_tag>::__c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3ccfd4 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
// was: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Camera::CameraType*,std::vector<RBX::Camera::CameraType,std::allocator<RBX::Camera::CameraType>>>,unsigned long,RBX::Camera::CameraType const&)")]
pub fn stub_0x3ccfd4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

// 0x3cdb0c — __ZN3rbx7signals6signalIFvbEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
// was: void __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::function<void ()(bool)>>(boost::function<void ()(bool)> const&)")]
pub fn stub_0x3cdb0c() -> crate::slot::SlotConnection {
// IDA 0x3cdb0c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x3cdc00 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::function<void ()(bool)>>::~callable_slot()")]
pub fn stub_0x3cdc00(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x3cdd10 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::function<void ()(bool)>>::~callable_slot() [0x3cdd10]")]
pub fn stub_0x3cdd10(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x3cde40 — __ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv
// type: void *()
// was: void *()
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x3cde40(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void ()(bool)>::slot::safe_static_do_get_mutex(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3cdf30 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::~callable()")]
pub fn stub_0x3cdf30(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3cdf30: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x3ce040 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_ED0Ev
// type: void __fastcall(_DWORD *)
// was: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::~callable() [0x3ce040]")]
pub fn stub_0x3ce040(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ce040: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x3ce170 — __ZN3rbx7signals6signalIFvbEE4slotD1Ev
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::~slot()")]
pub fn stub_0x3ce170(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x3d16d8 — __ZN3RBX6CameraD2Ev
// type: void __fastcall(RBX::Camera *this, int, int, int)
// was: void __fastcall(RBX::Camera *this, int, int, int)
#[doc(alias = "RBX::Camera::~Camera() [0x3d16d8]")]
pub fn stub_0x3d16d8(handle: crate::slot::InstanceHandle) {
// RBX::Camera dtor.
drop(handle);
}

// 0x3d1900 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEaSERKS2_
// type: int *__fastcall(int *, int *)
// was: int *__fastcall(int *, int *)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::operator=(G3D::Array<G3D::Plane,10,32ul> const&)")]
pub fn stub_0x3d1900() -> crate::slot::PortedFn {
// IDA 0x3d1900: G3D::Array<G3D::Plane,10,32ul>::operator=(G3D::Array<G3D::Plane,10,32ul> const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3d1900, "G3D::Array<G3D::Plane,10,32ul>::operator=(G3D::Array<G3D::Plane,10,32ul> const&)")
}

// 0x3d194c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera13CameraPanModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
// was: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraPanMode>> *)")]
pub fn stub_0x3d194c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x3d1974 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
// was: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraMode>> *)")]
pub fn stub_0x3d1974(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x3d199c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Camera10CameraTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
// was: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Camera::CameraType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Camera::CameraType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Camera::CameraType>> *)")]
pub fn stub_0x3d199c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "global constructor keyed to_a_166")]
pub fn stub_0x3d19c4() -> crate::slot::PortedFn {
// IDA 0x3d19c4: __GLOBAL__I_a_166.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3d19c4, "__GLOBAL__I_a_166")
}

// 0x3d249c — __ZN3RBX20ChangeHistoryService10setEnabledEb
// type: RBX::ChangeHistoryService *__fastcall(RBX::ChangeHistoryService *this, int)
// was: RBX::ChangeHistoryService *__fastcall(RBX::ChangeHistoryService *this, int)
#[doc(alias = "RBX::ChangeHistoryService::setEnabled(bool)")]
pub fn stub_0x3d249c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ChangeHistoryService setter.
cell.set(value)
}

// 0x3d24b8 — __ZN3RBX20ChangeHistoryService17resetBaseWaypointEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
// was: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::resetBaseWaypoint(void)")]
pub fn stub_0x3d24b8(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::resetBaseWaypoint(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d250c — __ZN3RBX20ChangeHistoryService4playEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
// was: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::play(void)")]
pub fn stub_0x3d250c(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::play(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d28cc — __ZN3RBX20ChangeHistoryService6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
// was: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::unplay(void)")]
pub fn stub_0x3d28cc(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::unplay(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d2c28 — __ZN3RBX20ChangeHistoryService10canUnplay2Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, RBX::ChangeHistoryService *)
// was: void __fastcall(RBX::ChangeHistoryService *this, RBX::ChangeHistoryService *)
#[doc(alias = "RBX::ChangeHistoryService::canUnplay2(void)")]
pub fn stub_0x3d2c28(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::canUnplay2(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d2ea0 — __ZN3RBX20ChangeHistoryService8canPlay2Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int)
// was: void __fastcall(RBX::ChangeHistoryService *this, int)
#[doc(alias = "RBX::ChangeHistoryService::canPlay2(void)")]
pub fn stub_0x3d2ea0(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::canPlay2(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d3120 — __ZN3RBX20ChangeHistoryService4Item12unplayDeleteEv
// type: void __fastcall(RBX::Instance **this, int, int, int)
// was: void __fastcall(RBX::Instance **this, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplayDelete(void)")]
pub fn stub_0x3d3120(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::unplayDelete(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d3518 — __ZN3RBX20ChangeHistoryService4Item17unplayClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// type: _DWORD *__fastcall(RBX::Instance **, unsigned int *)
// was: _DWORD *__fastcall(RBX::Instance **, unsigned int *)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplayClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
pub fn stub_0x3d3518(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::unplayClusterData(std::pair<unsigned int,std::vector<unsi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d362c — __ZN3RBX20ChangeHistoryService7setCellERKN3G3D12Vector3int16ES4_NS_5Voxel4CellENS5_12CellMaterialE
// type: int __fastcall(int)
// was: int __fastcall(int)
#[doc(alias = "RBX::ChangeHistoryService::setCell(G3D::Vector3int16 const&,G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)")]
pub fn stub_0x3d362c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::ChangeHistoryService setter.
cell.set(value)
}

// 0x3d367c — __ZN3RBX20ChangeHistoryServiceC1Ev
// type: int __fastcall(RBX::ChangeHistoryService *this)
// was: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::ChangeHistoryService(void)")]
pub fn stub_0x3d367c() -> crate::slot::InstanceHandle {
// RBX::ChangeHistoryService ctor.
crate::slot::InstanceHandle::new("RBX::ChangeHistoryService")
}

// 0x3d3680 — __ZN3RBX20ChangeHistoryServiceC2Ev
// type: __guard *__fastcall(RBX::ChangeHistoryService *this)
// was: __guard *__fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::ChangeHistoryService(void) [0x3d3680]")]
pub fn stub_0x3d3680() -> crate::slot::InstanceHandle {
// RBX::ChangeHistoryService ctor.
crate::slot::InstanceHandle::new("RBX::ChangeHistoryService")
}

// 0x3d39cc — __ZN3RBX20ChangeHistoryServiceD0Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
// was: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService()")]
pub fn stub_0x3d39cc(handle: crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService dtor.
drop(handle);
}

// 0x3d3a6c — __ZN3RBX20ChangeHistoryServiceD1Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
// was: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService() [0x3d3a6c]")]
pub fn stub_0x3d3a6c(handle: crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService dtor.
drop(handle);
}

// 0x3d3a70 — __ZThn32_N3RBX20ChangeHistoryServiceD0Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
// was: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService()")]
pub fn stub_0x3d3a70(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3d3a78 — __ZThn36_N3RBX20ChangeHistoryServiceD0Ev
// type: void __fastcall(RBX::ChangeHistoryService *__hidden this)
// was: void __fastcall(RBX::ChangeHistoryService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService() [0x3d3a78]")]
pub fn stub_0x3d3a78(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3d3a80 — __ZN3RBX20ChangeHistoryServiceD2Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
// was: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::~ChangeHistoryService() [0x3d3a80]")]
pub fn stub_0x3d3a80(handle: crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService dtor.
drop(handle);
}

// 0x3d3f08 — __ZThn32_N3RBX20ChangeHistoryServiceD1Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
// was: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService() [0x3d3f08]")]
pub fn stub_0x3d3f08(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3d3f10 — __ZThn36_N3RBX20ChangeHistoryServiceD1Ev
// type: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
// was: void __fastcall(RBX::ChangeHistoryService *this, int, int, int)
#[doc(alias = "non-virtual thunk toRBX::ChangeHistoryService::~ChangeHistoryService() [0x3d3f10]")]
pub fn stub_0x3d3f10(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

// 0x3d3f18 — __ZN3RBX20ChangeHistoryService6attachEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
// was: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::attach(void)")]
pub fn stub_0x3d3f18(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::attach(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x3d429c — __ZN3RBX20ChangeHistoryService7dettachEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
// was: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::dettach(void)")]
pub fn stub_0x3d429c(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::dettach(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}
