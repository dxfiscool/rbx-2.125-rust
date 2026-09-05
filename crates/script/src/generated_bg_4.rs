// Auto-generated skeletons for rbx-script — script-bg filler EA-sorted asc next 120 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x555598..0x55b41c | script 21631->21751 total (gap filler 0x555598 asc, not-in-any-crate)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::registerBodyMovers(void)")]
pub fn stub_0x555598() -> crate::slot::PortedFn {
// IDA 0x555598: RBX::registerBodyMovers().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x555598, "RBX::registerBodyMovers()")
}

#[doc(alias = "RBX::BodyMover::BodyMover(char const*)")]
pub fn stub_0x5555d8() -> crate::slot::InstanceHandle {
// RBX::BodyMover ctor.
crate::slot::InstanceHandle::new("RBX::BodyMover")
}

#[doc(alias = "RBX::BodyMover::~BodyMover()")]
pub fn stub_0x555878(handle: crate::slot::InstanceHandle) {
// RBX::BodyMover dtor.
drop(handle);
}

#[doc(alias = "RBX::BodyMover::~BodyMover() [0x555918]")]
pub fn stub_0x555918(handle: crate::slot::InstanceHandle) {
// RBX::BodyMover dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover()")]
pub fn stub_0x55591c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x555924]")]
pub fn stub_0x555924(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x55592c]")]
pub fn stub_0x55592c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x555934]")]
pub fn stub_0x555934(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x55593c]")]
pub fn stub_0x55593c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "RBX::BodyMover::~BodyMover() [0x555944]")]
pub fn stub_0x555944(handle: crate::slot::InstanceHandle) {
// RBX::BodyMover dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x555b68]")]
pub fn stub_0x555b68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x555b70]")]
pub fn stub_0x555b70(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x555b78]")]
pub fn stub_0x555b78(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x555b80]")]
pub fn stub_0x555b80(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::~BodyMover() [0x555b88]")]
pub fn stub_0x555b88(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "RBX::BodyMover::computeForce(bool)")]
pub fn stub_0x555b90(handle: &crate::slot::InstanceHandle) {
// RBX::BodyMover::computeForce(bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyMover::computeForce(bool,RBX::Body *&,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x555e18(handle: &crate::slot::InstanceHandle) {
// RBX::BodyMover::computeForce(bool, RBX::Body*&, G3D::Vector3&, G3D::Vector3&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::computeForce(bool)")]
pub fn stub_0x556034(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "RBX::BodyMover::stepWorld(void)")]
pub fn stub_0x556140(handle: &crate::slot::InstanceHandle) {
// RBX::BodyMover::stepWorld() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::stepWorld(void)")]
pub fn stub_0x55627c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::BodyMover::getEngineBody(void)")]
pub fn stub_0x556284(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BodyMover getter.
cell.get()
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::getEngineBody(void)")]
pub fn stub_0x556318(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::BodyMover::duplicateBodyMoverExists(RBX::Primitive *,RBX::Primitive *)")]
pub fn stub_0x556320(handle: &crate::slot::InstanceHandle) {
// RBX::BodyMover::duplicateBodyMoverExists(RBX::Primitive*, RBX::Primitive*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyMover::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x556368(handle: &crate::slot::InstanceHandle) {
// RBX::BodyMover::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Rocket::fire(void)")]
pub fn stub_0x5568b0(handle: &crate::slot::InstanceHandle) {
// RBX::Rocket::fire() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Rocket::abort(void)")]
pub fn stub_0x5568dc(handle: &crate::slot::InstanceHandle) {
// RBX::Rocket::abort() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Rocket::Rocket(void)")]
pub fn stub_0x55690c() -> crate::slot::InstanceHandle {
// RBX::Rocket ctor.
crate::slot::InstanceHandle::new("RBX::Rocket")
}

#[doc(alias = "RBX::Rocket::~Rocket()")]
pub fn stub_0x556bb0(handle: crate::slot::InstanceHandle) {
// RBX::Rocket dtor.
drop(handle);
}

#[doc(alias = "RBX::Rocket::~Rocket() [0x556c50]")]
pub fn stub_0x556c50(handle: crate::slot::InstanceHandle) {
// RBX::Rocket dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket()")]
pub fn stub_0x556c54(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556c5c]")]
pub fn stub_0x556c5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556c64]")]
pub fn stub_0x556c64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556c6c]")]
pub fn stub_0x556c6c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556c74]")]
pub fn stub_0x556c74(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556c7c]")]
pub fn stub_0x556c7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 304, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 304);
}

#[doc(alias = "RBX::Rocket::~Rocket() [0x556c84]")]
pub fn stub_0x556c84(handle: crate::slot::InstanceHandle) {
// RBX::Rocket dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556e1c]")]
pub fn stub_0x556e1c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556e24]")]
pub fn stub_0x556e24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556e2c]")]
pub fn stub_0x556e2c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556e34]")]
pub fn stub_0x556e34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556e3c]")]
pub fn stub_0x556e3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::~Rocket() [0x556e44]")]
pub fn stub_0x556e44(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 304, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 304);
}

#[doc(alias = "RBX::Rocket::onStepped(RBX::Stepped const&)")]
pub fn stub_0x556e4c(handle: &crate::slot::InstanceHandle) {
// RBX::Rocket::onStepped(RBX::Stepped const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Rocket::onStepped(RBX::Stepped const&)")]
pub fn stub_0x55705c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 304, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 304);
}

#[doc(alias = "RBX::Rocket::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x557064(handle: &crate::slot::InstanceHandle) {
// RBX::Rocket::computeForceImpl(bool, RBX::Body*, RBX::Body*, G3D::Vector3&, G3D::Vector3&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Rocket::computeTorque(RBX::Body *,RBX::Body *,G3D::Vector3 const&)")]
pub fn stub_0x5575a4(handle: &crate::slot::InstanceHandle) {
// RBX::Rocket::computeTorque(RBX::Body*, RBX::Body*, G3D::Vector3 const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyGyro::BodyGyro(void)")]
pub fn stub_0x5578a0() -> crate::slot::InstanceHandle {
// RBX::BodyGyro ctor.
crate::slot::InstanceHandle::new("RBX::BodyGyro")
}

#[doc(alias = "RBX::BodyGyro::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x557a64(handle: &crate::slot::InstanceHandle) {
// RBX::BodyGyro::computeForceImpl(bool, RBX::Body*, RBX::Body*, G3D::Vector3&, G3D::Vector3&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyGyro::computeBalanceTorque(RBX::Body *,RBX::Body *)")]
pub fn stub_0x557c50(handle: &crate::slot::InstanceHandle) {
// RBX::BodyGyro::computeBalanceTorque(RBX::Body*, RBX::Body*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyGyro::computeOrientationTorque(RBX::Body *,RBX::Body *)")]
pub fn stub_0x557ff8(handle: &crate::slot::InstanceHandle) {
// RBX::BodyGyro::computeOrientationTorque(RBX::Body*, RBX::Body*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyPosition::BodyPosition(void)")]
pub fn stub_0x5582bc() -> crate::slot::InstanceHandle {
// RBX::BodyPosition ctor.
crate::slot::InstanceHandle::new("RBX::BodyPosition")
}

#[doc(alias = "RBX::BodyPosition::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x5584cc(handle: &crate::slot::InstanceHandle) {
// RBX::BodyPosition::computeForceImpl(bool, RBX::Body*, RBX::Body*, G3D::Vector3&, G3D::Vect~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyPosition::onStepped(RBX::Stepped const&)")]
pub fn stub_0x558780(handle: &crate::slot::InstanceHandle) {
// RBX::BodyPosition::onStepped(RBX::Stepped const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::onStepped(RBX::Stepped const&)")]
pub fn stub_0x5588ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 304, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 304);
}

#[doc(alias = "RBX::BodyVelocity::BodyVelocity(void)")]
pub fn stub_0x5588f8() -> crate::slot::InstanceHandle {
// RBX::BodyVelocity ctor.
crate::slot::InstanceHandle::new("RBX::BodyVelocity")
}

#[doc(alias = "RBX::BodyVelocity::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x558aac(handle: &crate::slot::InstanceHandle) {
// RBX::BodyVelocity::computeForceImpl(bool, RBX::Body*, RBX::Body*, G3D::Vector3&, G3D::Vect~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyAngularVelocity::BodyAngularVelocity(void)")]
pub fn stub_0x558c34() -> crate::slot::InstanceHandle {
// RBX::BodyAngularVelocity ctor.
crate::slot::InstanceHandle::new("RBX::BodyAngularVelocity")
}

#[doc(alias = "RBX::BodyAngularVelocity::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x558de8(handle: &crate::slot::InstanceHandle) {
// RBX::BodyAngularVelocity::computeForceImpl(bool, RBX::Body*, RBX::Body*, G3D::Vector3&, G3~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyForce::BodyForce(void)")]
pub fn stub_0x558f70() -> crate::slot::InstanceHandle {
// RBX::BodyForce ctor.
crate::slot::InstanceHandle::new("RBX::BodyForce")
}

#[doc(alias = "RBX::BodyForce::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x559108(handle: &crate::slot::InstanceHandle) {
// RBX::BodyForce::computeForceImpl(bool, RBX::Body*, RBX::Body*, G3D::Vector3&, G3D::Vector3~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BodyThrust::BodyThrust(void)")]
pub fn stub_0x559124() -> crate::slot::InstanceHandle {
// RBX::BodyThrust ctor.
crate::slot::InstanceHandle::new("RBX::BodyThrust")
}

#[doc(alias = "RBX::BodyThrust::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
pub fn stub_0x5592d8(handle: &crate::slot::InstanceHandle) {
// RBX::BodyThrust::computeForceImpl(bool, RBX::Body*, RBX::Body*, G3D::Vector3&, G3D::Vector~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Rocket::getTargetDangerous(void)const")]
pub fn stub_0x559440(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Rocket getter.
cell.get()
}

#[doc(alias = "RBX::Body::getBranchForce(void)const")]
pub fn stub_0x5594c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Body getter.
cell.get()
}

#[doc(alias = "RBX::Body::getBranchTorque(void)const")]
pub fn stub_0x559534(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Body getter.
cell.get()
}

#[doc(alias = "RBX::BodyPosition::getLastForce(void)")]
pub fn stub_0x5595ac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BodyPosition getter.
cell.get()
}

#[doc(alias = "RBX::BodyVelocity::getLastForce(void)")]
pub fn stub_0x559604(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BodyVelocity getter.
cell.get()
}

#[doc(alias = "RBX::Body::getBranchVelocity(void)")]
pub fn stub_0x559638(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Body getter.
cell.get()
}

#[doc(alias = "RBX::BodyPosition::~BodyPosition()")]
pub fn stub_0x5596b0(handle: crate::slot::InstanceHandle) {
// RBX::BodyPosition dtor.
drop(handle);
}

#[doc(alias = "RBX::BodyPosition::~BodyPosition() [0x5597e0]")]
pub fn stub_0x5597e0(handle: crate::slot::InstanceHandle) {
// RBX::BodyPosition dtor.
drop(handle);
}

#[doc(alias = "RBX::BodyMover::canStepWorld(void)const")]
pub fn stub_0x559938(handle: &crate::slot::InstanceHandle) {
// RBX::BodyMover::canStepWorld() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition()")]
pub fn stub_0x55993c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x559a68]")]
pub fn stub_0x559a68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x559bb8]")]
pub fn stub_0x559bb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x559ce4]")]
pub fn stub_0x559ce4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x559e24]")]
pub fn stub_0x559e24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x559f50]")]
pub fn stub_0x559f50(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyMover::canStepWorld(void)const")]
pub fn stub_0x55a090(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x55a094]")]
pub fn stub_0x55a094(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x55a1c0]")]
pub fn stub_0x55a1c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x55a300]")]
pub fn stub_0x55a300(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x55a430]")]
pub fn stub_0x55a430(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x55a574]")]
pub fn stub_0x55a574(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 304, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 304);
}

#[doc(alias = "non-virtual thunk toRBX::BodyPosition::~BodyPosition() [0x55a6a4]")]
pub fn stub_0x55a6a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 304, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 304);
}

#[doc(alias = "RBX::BodyGyro::~BodyGyro()")]
pub fn stub_0x55a860(handle: crate::slot::InstanceHandle) {
// RBX::BodyGyro dtor.
drop(handle);
}

#[doc(alias = "RBX::BodyGyro::~BodyGyro() [0x55a864]")]
pub fn stub_0x55a864(handle: crate::slot::InstanceHandle) {
// RBX::BodyGyro dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
pub fn stub_0x55a914(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55a91c]")]
pub fn stub_0x55a91c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55a9d0]")]
pub fn stub_0x55a9d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55a9d8]")]
pub fn stub_0x55a9d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55aa7c]")]
pub fn stub_0x55aa7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55aa84]")]
pub fn stub_0x55aa84(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55ab28]")]
pub fn stub_0x55ab28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55ab30]")]
pub fn stub_0x55ab30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55abd4]")]
pub fn stub_0x55abd4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro() [0x55abdc]")]
pub fn stub_0x55abdc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "RBX::BodyVelocity::~BodyVelocity()")]
pub fn stub_0x55ac80(handle: crate::slot::InstanceHandle) {
// RBX::BodyVelocity dtor.
drop(handle);
}

#[doc(alias = "RBX::BodyVelocity::~BodyVelocity() [0x55ac84]")]
pub fn stub_0x55ac84(handle: crate::slot::InstanceHandle) {
// RBX::BodyVelocity dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
pub fn stub_0x55ad34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55ad3c]")]
pub fn stub_0x55ad3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55adf0]")]
pub fn stub_0x55adf0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55adf8]")]
pub fn stub_0x55adf8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55ae9c]")]
pub fn stub_0x55ae9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55aea4]")]
pub fn stub_0x55aea4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55af48]")]
pub fn stub_0x55af48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55af50]")]
pub fn stub_0x55af50(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55aff4]")]
pub fn stub_0x55aff4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity() [0x55affc]")]
pub fn stub_0x55affc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "RBX::BodyAngularVelocity::~BodyAngularVelocity()")]
pub fn stub_0x55b0a0(handle: crate::slot::InstanceHandle) {
// RBX::BodyAngularVelocity dtor.
drop(handle);
}

#[doc(alias = "RBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b0a4]")]
pub fn stub_0x55b0a4(handle: crate::slot::InstanceHandle) {
// RBX::BodyAngularVelocity dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
pub fn stub_0x55b154(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b15c]")]
pub fn stub_0x55b15c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b210]")]
pub fn stub_0x55b210(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b218]")]
pub fn stub_0x55b218(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b2bc]")]
pub fn stub_0x55b2bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b2c4]")]
pub fn stub_0x55b2c4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b368]")]
pub fn stub_0x55b368(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b370]")]
pub fn stub_0x55b370(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 124, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 124);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b414]")]
pub fn stub_0x55b414(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}

#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity() [0x55b41c]")]
pub fn stub_0x55b41c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 244, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 244);
}
