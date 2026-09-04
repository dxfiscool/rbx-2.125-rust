//! rendering shard 471 — 100 stubs 0x555598..0x55ad3c EA-sorted asc global gap filler not yet in any crate (Ogre/G3D complete)
//! Source: ida/export.json (85545 funcs) EA asc global gap filler 51504 distinct existing -> 51604 after, next 100 gaps ascending
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x555598 — __ZN3RBX18registerBodyMoversEv
// type: _DWORD __fastcall(RBX *__hidden this)
#[doc(alias = "RBX::registerBodyMovers(void)")]
#[doc(alias = "__ZN3RBX18registerBodyMoversEv")]
// IDA 0x555598: 17 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_555598() {
}

// 0x5555d8 — __ZN3RBX9BodyMoverC2EPKc
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, const char *)
#[doc(alias = "RBX::BodyMover::BodyMover(char const*)")]
#[doc(alias = "__ZN3RBX9BodyMoverC2EPKc")]
// IDA 0x5555d8: 228 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5555d8() {
}

// 0x555878 — __ZN3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZN3RBX9BodyMoverD0Ev")]
// IDA 0x555878: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555878() {
}

// 0x555918 — __ZN3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZN3RBX9BodyMoverD1Ev")]
// IDA 0x555918: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_555918() {
}

// 0x55591c — __ZThn32_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn32_N3RBX9BodyMoverD0Ev")]
// IDA 0x55591c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55591c() {
}

// 0x555924 — __ZThn36_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn36_N3RBX9BodyMoverD0Ev")]
// IDA 0x555924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555924() {
}

// 0x55592c — __ZThn92_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn92_N3RBX9BodyMoverD0Ev")]
// IDA 0x55592c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55592c() {
}

// 0x555934 — __ZThn124_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn124_N3RBX9BodyMoverD0Ev")]
// IDA 0x555934: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555934() {
}

// 0x55593c — __ZThn244_N3RBX9BodyMoverD0Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn244_N3RBX9BodyMoverD0Ev")]
// IDA 0x55593c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55593c() {
}

// 0x555944 — __ZN3RBX9BodyMoverD2Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZN3RBX9BodyMoverD2Ev")]
// IDA 0x555944: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555944() {
}

// 0x555b68 — __ZThn32_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn32_N3RBX9BodyMoverD1Ev")]
// IDA 0x555b68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b68() {
}

// 0x555b70 — __ZThn36_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn36_N3RBX9BodyMoverD1Ev")]
// IDA 0x555b70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b70() {
}

// 0x555b78 — __ZThn92_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn92_N3RBX9BodyMoverD1Ev")]
// IDA 0x555b78: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b78() {
}

// 0x555b80 — __ZThn124_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn124_N3RBX9BodyMoverD1Ev")]
// IDA 0x555b80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b80() {
}

// 0x555b88 — __ZThn244_N3RBX9BodyMoverD1Ev
// type: void __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::~BodyMover()")]
#[doc(alias = "__ZThn244_N3RBX9BodyMoverD1Ev")]
// IDA 0x555b88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_555b88() {
}

// 0x555b90 — __ZN3RBX9BodyMover12computeForceEb
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, bool)
#[doc(alias = "RBX::BodyMover::computeForce(bool)")]
#[doc(alias = "__ZN3RBX9BodyMover12computeForceEb")]
// IDA 0x555b90: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_555b90() {
}

// 0x555e18 — __ZN3RBX9BodyMover12computeForceEbRPNS_4BodyERN3G3D7Vector3ES6_
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, bool, RBX::Body **, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "RBX::BodyMover::computeForce(bool,RBX::Body *&,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX9BodyMover12computeForceEbRPNS_4BodyERN3G3D7Vector3ES6_")]
// IDA 0x555e18: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_555e18() {
}

// 0x556034 — __ZThn244_N3RBX9BodyMover12computeForceEb
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, bool)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::computeForce(bool)")]
#[doc(alias = "__ZThn244_N3RBX9BodyMover12computeForceEb")]
// IDA 0x556034: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556034() {
}

// 0x556140 — __ZN3RBX9BodyMover9stepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "RBX::BodyMover::stepWorld(void)")]
#[doc(alias = "__ZN3RBX9BodyMover9stepWorldEv")]
// IDA 0x556140: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556140() {
}

// 0x55627c — __ZThn92_N3RBX9BodyMover9stepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::stepWorld(void)")]
#[doc(alias = "__ZThn92_N3RBX9BodyMover9stepWorldEv")]
// IDA 0x55627c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55627c() {
}

// 0x556284 — __ZN3RBX9BodyMover13getEngineBodyEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "RBX::BodyMover::getEngineBody(void)")]
#[doc(alias = "__ZN3RBX9BodyMover13getEngineBodyEv")]
// IDA 0x556284: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556284() {
}

// 0x556318 — __ZThn92_N3RBX9BodyMover13getEngineBodyEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::getEngineBody(void)")]
#[doc(alias = "__ZThn92_N3RBX9BodyMover13getEngineBodyEv")]
// IDA 0x556318: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556318() {
}

// 0x556320 — __ZN3RBX9BodyMover24duplicateBodyMoverExistsEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::BodyMover::duplicateBodyMoverExists(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9BodyMover24duplicateBodyMoverExistsEPNS_9PrimitiveES2_")]
// IDA 0x556320: 30 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556320() {
}

// 0x556368 — __ZN3RBX9BodyMover17onAncestorChangedERKNS_15AncestorChangedE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::BodyMover::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX9BodyMover17onAncestorChangedERKNS_15AncestorChangedE")]
// IDA 0x556368: 372 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556368() {
}

// 0x5568b0 — __ZN3RBX6Rocket4fireEv
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "RBX::Rocket::fire(void)")]
#[doc(alias = "__ZN3RBX6Rocket4fireEv")]
// IDA 0x5568b0: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5568b0() {
}

// 0x5568dc — __ZN3RBX6Rocket5abortEv
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "RBX::Rocket::abort(void)")]
#[doc(alias = "__ZN3RBX6Rocket5abortEv")]
// IDA 0x5568dc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5568dc() {
}

// 0x55690c — __ZN3RBX6RocketC2Ev
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "RBX::Rocket::Rocket(void)")]
#[doc(alias = "__ZN3RBX6RocketC2Ev")]
// IDA 0x55690c: 225 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55690c() {
}

// 0x556bb0 — __ZN3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZN3RBX6RocketD0Ev")]
// IDA 0x556bb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556bb0() {
}

// 0x556c50 — __ZN3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZN3RBX6RocketD1Ev")]
// IDA 0x556c50: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_556c50() {
}

// 0x556c54 — __ZThn32_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn32_N3RBX6RocketD0Ev")]
// IDA 0x556c54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c54() {
}

// 0x556c5c — __ZThn36_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn36_N3RBX6RocketD0Ev")]
// IDA 0x556c5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c5c() {
}

// 0x556c64 — __ZThn92_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn92_N3RBX6RocketD0Ev")]
// IDA 0x556c64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c64() {
}

// 0x556c6c — __ZThn124_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn124_N3RBX6RocketD0Ev")]
// IDA 0x556c6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c6c() {
}

// 0x556c74 — __ZThn244_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn244_N3RBX6RocketD0Ev")]
// IDA 0x556c74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c74() {
}

// 0x556c7c — __ZThn304_N3RBX6RocketD0Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn304_N3RBX6RocketD0Ev")]
// IDA 0x556c7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c7c() {
}

// 0x556c84 — __ZN3RBX6RocketD2Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZN3RBX6RocketD2Ev")]
// IDA 0x556c84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556c84() {
}

// 0x556e1c — __ZThn32_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn32_N3RBX6RocketD1Ev")]
// IDA 0x556e1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e1c() {
}

// 0x556e24 — __ZThn36_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn36_N3RBX6RocketD1Ev")]
// IDA 0x556e24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e24() {
}

// 0x556e2c — __ZThn92_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn92_N3RBX6RocketD1Ev")]
// IDA 0x556e2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e2c() {
}

// 0x556e34 — __ZThn124_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn124_N3RBX6RocketD1Ev")]
// IDA 0x556e34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e34() {
}

// 0x556e3c — __ZThn244_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn244_N3RBX6RocketD1Ev")]
// IDA 0x556e3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e3c() {
}

// 0x556e44 — __ZThn304_N3RBX6RocketD1Ev
// type: void __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Rocket::~Rocket()")]
#[doc(alias = "__ZThn304_N3RBX6RocketD1Ev")]
// IDA 0x556e44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_556e44() {
}

// 0x556e4c — __ZN3RBX6Rocket9onSteppedERKNS_7SteppedE
#[doc(alias = "RBX::Rocket::onStepped(RBX::Stepped const&)")]
#[doc(alias = "__ZN3RBX6Rocket9onSteppedERKNS_7SteppedE")]
// IDA 0x556e4c: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_556e4c() {
}

// 0x55705c — __ZThn304_N3RBX6Rocket9onSteppedERKNS_7SteppedE
#[doc(alias = "non-virtual thunk to RBX::Rocket::onStepped(RBX::Stepped const&)")]
#[doc(alias = "__ZThn304_N3RBX6Rocket9onSteppedERKNS_7SteppedE")]
// IDA 0x55705c: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55705c() {
}

// 0x557064 — __ZN3RBX6Rocket16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::Rocket *__hidden this, bool, RBX::Body *, RBX::Body *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "RBX::Rocket::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX6Rocket16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_")]
// IDA 0x557064: 365 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_557064() {
}

// 0x5575a4 — __ZN3RBX6Rocket13computeTorqueEPNS_4BodyES2_RKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Rocket *__hidden this, RBX::Body *, RBX::Body *, const G3D::Vector3 *)
#[doc(alias = "RBX::Rocket::computeTorque(RBX::Body *,RBX::Body *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX6Rocket13computeTorqueEPNS_4BodyES2_RKN3G3D7Vector3E")]
// IDA 0x5575a4: 229 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5575a4() {
}

// 0x5578a0 — __ZN3RBX8BodyGyroC2Ev
// type: _DWORD __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "RBX::BodyGyro::BodyGyro(void)")]
#[doc(alias = "__ZN3RBX8BodyGyroC2Ev")]
// IDA 0x5578a0: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5578a0() {
}

// 0x557a64 — __ZN3RBX8BodyGyro16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::BodyGyro *__hidden this, bool, RBX::Body *, RBX::Body *, G3D::Vector3 *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::BodyGyro::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX8BodyGyro16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_")]
// IDA 0x557a64: 164 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_557a64() {
}

// 0x557c50 — __ZN3RBX8BodyGyro20computeBalanceTorqueEPNS_4BodyES2_
// type: _DWORD __fastcall(RBX::BodyGyro *__hidden this, RBX::Body *, RBX::Body *)
#[doc(alias = "RBX::BodyGyro::computeBalanceTorque(RBX::Body *,RBX::Body *)")]
#[doc(alias = "__ZN3RBX8BodyGyro20computeBalanceTorqueEPNS_4BodyES2_")]
// IDA 0x557c50: 274 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_557c50() {
}

// 0x557ff8 — __ZN3RBX8BodyGyro24computeOrientationTorqueEPNS_4BodyES2_
// type: _DWORD __fastcall(RBX::BodyGyro *__hidden this, RBX::Body *, RBX::Body *)
#[doc(alias = "RBX::BodyGyro::computeOrientationTorque(RBX::Body *,RBX::Body *)")]
#[doc(alias = "__ZN3RBX8BodyGyro24computeOrientationTorqueEPNS_4BodyES2_")]
// IDA 0x557ff8: 206 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_557ff8() {
}

// 0x5582bc — __ZN3RBX12BodyPositionC2Ev
// type: _DWORD __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "RBX::BodyPosition::BodyPosition(void)")]
#[doc(alias = "__ZN3RBX12BodyPositionC2Ev")]
// IDA 0x5582bc: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5582bc() {
}

// 0x5584cc — __ZN3RBX12BodyPosition16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::BodyPosition *__hidden this, bool, RBX::Body *, RBX::Body *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "RBX::BodyPosition::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX12BodyPosition16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_")]
// IDA 0x5584cc: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5584cc() {
}

// 0x558780 — __ZN3RBX12BodyPosition9onSteppedERKNS_7SteppedE
#[doc(alias = "RBX::BodyPosition::onStepped(RBX::Stepped const&)")]
#[doc(alias = "__ZN3RBX12BodyPosition9onSteppedERKNS_7SteppedE")]
// IDA 0x558780: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558780() {
}

// 0x5588ec — __ZThn304_N3RBX12BodyPosition9onSteppedERKNS_7SteppedE
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::onStepped(RBX::Stepped const&)")]
#[doc(alias = "__ZThn304_N3RBX12BodyPosition9onSteppedERKNS_7SteppedE")]
// IDA 0x5588ec: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5588ec() {
}

// 0x5588f8 — __ZN3RBX12BodyVelocityC2Ev
// type: _DWORD __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "RBX::BodyVelocity::BodyVelocity(void)")]
#[doc(alias = "__ZN3RBX12BodyVelocityC2Ev")]
// IDA 0x5588f8: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5588f8() {
}

// 0x558aac — __ZN3RBX12BodyVelocity16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::BodyVelocity *__hidden this, bool, RBX::Body *, RBX::Body *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "RBX::BodyVelocity::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX12BodyVelocity16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_")]
// IDA 0x558aac: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558aac() {
}

// 0x558c34 — __ZN3RBX19BodyAngularVelocityC2Ev
// type: _DWORD __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "RBX::BodyAngularVelocity::BodyAngularVelocity(void)")]
#[doc(alias = "__ZN3RBX19BodyAngularVelocityC2Ev")]
// IDA 0x558c34: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558c34() {
}

// 0x558de8 — __ZN3RBX19BodyAngularVelocity16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::BodyAngularVelocity *__hidden this, bool, RBX::Body *, RBX::Body *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "RBX::BodyAngularVelocity::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX19BodyAngularVelocity16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_")]
// IDA 0x558de8: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558de8() {
}

// 0x558f70 — __ZN3RBX9BodyForceC2Ev
// type: _DWORD __fastcall(RBX::BodyForce *__hidden this)
#[doc(alias = "RBX::BodyForce::BodyForce(void)")]
#[doc(alias = "__ZN3RBX9BodyForceC2Ev")]
// IDA 0x558f70: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_558f70() {
}

// 0x559108 — __ZN3RBX9BodyForce16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_
#[doc(alias = "RBX::BodyForce::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX9BodyForce16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_")]
// IDA 0x559108: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559108() {
}

// 0x559124 — __ZN3RBX10BodyThrustC2Ev
// type: _DWORD __fastcall(RBX::BodyThrust *__hidden this)
#[doc(alias = "RBX::BodyThrust::BodyThrust(void)")]
#[doc(alias = "__ZN3RBX10BodyThrustC2Ev")]
// IDA 0x559124: 143 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559124() {
}

// 0x5592d8 — __ZN3RBX10BodyThrust16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::BodyThrust *__hidden this, bool, RBX::Body *, RBX::Body *, G3D::Vector3 *, G3D::Vector3 *)
#[doc(alias = "RBX::BodyThrust::computeForceImpl(bool,RBX::Body *,RBX::Body *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX10BodyThrust16computeForceImplEbPNS_4BodyES2_RN3G3D7Vector3ES5_")]
// IDA 0x5592d8: 100 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5592d8() {
}

// 0x559440 — __ZNK3RBX6Rocket18getTargetDangerousEv
// type: _DWORD __fastcall(RBX::Rocket *__hidden this)
#[doc(alias = "RBX::Rocket::getTargetDangerous(void)const")]
#[doc(alias = "__ZNK3RBX6Rocket18getTargetDangerousEv")]
// IDA 0x559440: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559440() {
}

// 0x5594c4 — __ZNK3RBX4Body14getBranchForceEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getBranchForce(void)const")]
#[doc(alias = "__ZNK3RBX4Body14getBranchForceEv")]
// IDA 0x5594c4: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5594c4() {
}

// 0x559534 — __ZNK3RBX4Body15getBranchTorqueEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getBranchTorque(void)const")]
#[doc(alias = "__ZNK3RBX4Body15getBranchTorqueEv")]
// IDA 0x559534: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559534() {
}

// 0x5595ac — __ZN3RBX12BodyPosition12getLastForceEv
// type: _DWORD __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "RBX::BodyPosition::getLastForce(void)")]
#[doc(alias = "__ZN3RBX12BodyPosition12getLastForceEv")]
// IDA 0x5595ac: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5595ac() {
}

// 0x559604 — __ZN3RBX12BodyVelocity12getLastForceEv
// type: _DWORD __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "RBX::BodyVelocity::getLastForce(void)")]
#[doc(alias = "__ZN3RBX12BodyVelocity12getLastForceEv")]
// IDA 0x559604: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559604() {
}

// 0x559638 — __ZN3RBX4Body17getBranchVelocityEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getBranchVelocity(void)")]
#[doc(alias = "__ZN3RBX4Body17getBranchVelocityEv")]
// IDA 0x559638: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559638() {
}

// 0x5596b0 — __ZN3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZN3RBX12BodyPositionD1Ev")]
// IDA 0x5596b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5596b0() {
}

// 0x5597e0 — __ZN3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZN3RBX12BodyPositionD0Ev")]
// IDA 0x5597e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5597e0() {
}

// 0x559938 — __ZNK3RBX9BodyMover12canStepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "RBX::BodyMover::canStepWorld(void)const")]
#[doc(alias = "__ZNK3RBX9BodyMover12canStepWorldEv")]
// IDA 0x559938: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_559938() {
}

// 0x55993c — __ZThn32_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn32_N3RBX12BodyPositionD1Ev")]
// IDA 0x55993c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55993c() {
}

// 0x559a68 — __ZThn32_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn32_N3RBX12BodyPositionD0Ev")]
// IDA 0x559a68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559a68() {
}

// 0x559bb8 — __ZThn36_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn36_N3RBX12BodyPositionD1Ev")]
// IDA 0x559bb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559bb8() {
}

// 0x559ce4 — __ZThn36_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn36_N3RBX12BodyPositionD0Ev")]
// IDA 0x559ce4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559ce4() {
}

// 0x559e24 — __ZThn92_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn92_N3RBX12BodyPositionD1Ev")]
// IDA 0x559e24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559e24() {
}

// 0x559f50 — __ZThn92_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn92_N3RBX12BodyPositionD0Ev")]
// IDA 0x559f50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_559f50() {
}

// 0x55a090 — __ZThn92_NK3RBX9BodyMover12canStepWorldEv
// type: _DWORD __fastcall(RBX::BodyMover *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyMover::canStepWorld(void)const")]
#[doc(alias = "__ZThn92_NK3RBX9BodyMover12canStepWorldEv")]
// IDA 0x55a090: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a090() {
}

// 0x55a094 — __ZThn124_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn124_N3RBX12BodyPositionD1Ev")]
// IDA 0x55a094: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a094() {
}

// 0x55a1c0 — __ZThn124_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn124_N3RBX12BodyPositionD0Ev")]
// IDA 0x55a1c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a1c0() {
}

// 0x55a300 — __ZThn244_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn244_N3RBX12BodyPositionD1Ev")]
// IDA 0x55a300: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a300() {
}

// 0x55a430 — __ZThn244_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn244_N3RBX12BodyPositionD0Ev")]
// IDA 0x55a430: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a430() {
}

// 0x55a574 — __ZThn304_N3RBX12BodyPositionD1Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn304_N3RBX12BodyPositionD1Ev")]
// IDA 0x55a574: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a574() {
}

// 0x55a6a4 — __ZThn304_N3RBX12BodyPositionD0Ev
// type: void __fastcall(RBX::BodyPosition *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyPosition::~BodyPosition()")]
#[doc(alias = "__ZThn304_N3RBX12BodyPositionD0Ev")]
// IDA 0x55a6a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a6a4() {
}

// 0x55a860 — __ZN3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZN3RBX8BodyGyroD1Ev")]
// IDA 0x55a860: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_55a860() {
}

// 0x55a864 — __ZN3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZN3RBX8BodyGyroD0Ev")]
// IDA 0x55a864: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a864() {
}

// 0x55a914 — __ZThn32_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn32_N3RBX8BodyGyroD1Ev")]
// IDA 0x55a914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a914() {
}

// 0x55a91c — __ZThn32_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn32_N3RBX8BodyGyroD0Ev")]
// IDA 0x55a91c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a91c() {
}

// 0x55a9d0 — __ZThn36_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn36_N3RBX8BodyGyroD1Ev")]
// IDA 0x55a9d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a9d0() {
}

// 0x55a9d8 — __ZThn36_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn36_N3RBX8BodyGyroD0Ev")]
// IDA 0x55a9d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a9d8() {
}

// 0x55aa7c — __ZThn92_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn92_N3RBX8BodyGyroD1Ev")]
// IDA 0x55aa7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55aa7c() {
}

// 0x55aa84 — __ZThn92_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn92_N3RBX8BodyGyroD0Ev")]
// IDA 0x55aa84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55aa84() {
}

// 0x55ab28 — __ZThn124_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn124_N3RBX8BodyGyroD1Ev")]
// IDA 0x55ab28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ab28() {
}

// 0x55ab30 — __ZThn124_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn124_N3RBX8BodyGyroD0Ev")]
// IDA 0x55ab30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ab30() {
}

// 0x55abd4 — __ZThn244_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn244_N3RBX8BodyGyroD1Ev")]
// IDA 0x55abd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55abd4() {
}

// 0x55abdc — __ZThn244_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyGyro::~BodyGyro()")]
#[doc(alias = "__ZThn244_N3RBX8BodyGyroD0Ev")]
// IDA 0x55abdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55abdc() {
}

// 0x55ac80 — __ZN3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "RBX::BodyVelocity::~BodyVelocity()")]
#[doc(alias = "__ZN3RBX12BodyVelocityD1Ev")]
// IDA 0x55ac80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_55ac80() {
}

// 0x55ac84 — __ZN3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "RBX::BodyVelocity::~BodyVelocity()")]
#[doc(alias = "__ZN3RBX12BodyVelocityD0Ev")]
// IDA 0x55ac84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ac84() {
}

// 0x55ad34 — __ZThn32_N3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyVelocity::~BodyVelocity()")]
#[doc(alias = "__ZThn32_N3RBX12BodyVelocityD1Ev")]
// IDA 0x55ad34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ad34() {
}

// 0x55ad3c — __ZThn32_N3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::BodyVelocity::~BodyVelocity()")]
#[doc(alias = "__ZThn32_N3RBX12BodyVelocityD0Ev")]
// IDA 0x55ad3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ad3c() {
}
