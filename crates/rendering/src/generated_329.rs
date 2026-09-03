//! rendering shard 329 — 120 stubs 0x5a465c..0x5a865c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 35840->35960 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 35840 before -> 35960 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 120 after 0x5a464c (lowest remaining 0x5a465c..0x5a865c, next lowest 0x5a8660 if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5a465c — __ZThn36_N3RBX6RotateD1Ev
// type: void __fastcall(RBX::Rotate *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Rotate::~Rotate()")]
// was: __ZThn36_N3RBX6RotateD1Ev
// IDA 0x5a465c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a465c() {
}

// 0x5a4664 — __ZThn36_N3RBX6RotateD0Ev
// type: void __fastcall(RBX::Rotate *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Rotate::~Rotate()")]
// was: __ZThn36_N3RBX6RotateD0Ev
// IDA 0x5a4664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4664() {
}

// 0x5a4708 — __ZN3RBX13DynamicRotateD1Ev
// type: void __fastcall(RBX::DynamicRotate *__hidden this)
#[doc(alias = "RBX::DynamicRotate::~DynamicRotate()")]
// was: __ZN3RBX13DynamicRotateD1Ev
// IDA 0x5a4708: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a4708() {
}

// 0x5a470c — __ZN3RBX13DynamicRotateD0Ev
// type: void __fastcall(RBX::DynamicRotate *__hidden this)
#[doc(alias = "RBX::DynamicRotate::~DynamicRotate()")]
// was: __ZN3RBX13DynamicRotateD0Ev
// IDA 0x5a470c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a470c() {
}

// 0x5a47ac — __ZNK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv
// IDA 0x5a47ac: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a47ac() {
}

// 0x5a47d4 — __ZThn32_N3RBX13DynamicRotateD1Ev
// type: void __fastcall(RBX::DynamicRotate *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotate::~DynamicRotate()")]
// was: __ZThn32_N3RBX13DynamicRotateD1Ev
// IDA 0x5a47d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a47d4() {
}

// 0x5a47dc — __ZThn32_N3RBX13DynamicRotateD0Ev
// type: void __fastcall(RBX::DynamicRotate *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotate::~DynamicRotate()")]
// was: __ZThn32_N3RBX13DynamicRotateD0Ev
// IDA 0x5a47dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a47dc() {
}

// 0x5a4880 — __ZThn32_NK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_13JointInstanceELZNS_14sDynamicRotateEEE12getClassNameEv
// IDA 0x5a4880: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4880() {
}

// 0x5a48a8 — __ZThn36_N3RBX13DynamicRotateD1Ev
// type: void __fastcall(RBX::DynamicRotate *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotate::~DynamicRotate()")]
// was: __ZThn36_N3RBX13DynamicRotateD1Ev
// IDA 0x5a48a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a48a8() {
}

// 0x5a48b0 — __ZThn36_N3RBX13DynamicRotateD0Ev
// type: void __fastcall(RBX::DynamicRotate *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotate::~DynamicRotate()")]
// was: __ZThn36_N3RBX13DynamicRotateD0Ev
// IDA 0x5a48b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a48b0() {
}

// 0x5a4954 — __ZN3RBX7RotatePD1Ev
// type: void __fastcall(RBX::RotateP *__hidden this)
#[doc(alias = "RBX::RotateP::~RotateP()")]
// was: __ZN3RBX7RotatePD1Ev
// IDA 0x5a4954: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a4954() {
}

// 0x5a4958 — __ZN3RBX7RotatePD0Ev
// type: void __fastcall(RBX::RotateP *__hidden this)
#[doc(alias = "RBX::RotateP::~RotateP()")]
// was: __ZN3RBX7RotatePD0Ev
// IDA 0x5a4958: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4958() {
}

// 0x5a49f8 — __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv
// IDA 0x5a49f8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a49f8() {
}

// 0x5a4a08 — __ZThn32_N3RBX7RotatePD1Ev
// type: void __fastcall(RBX::RotateP *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateP::~RotateP()")]
// was: __ZThn32_N3RBX7RotatePD1Ev
// IDA 0x5a4a08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4a08() {
}

// 0x5a4a10 — __ZThn32_N3RBX7RotatePD0Ev
// type: void __fastcall(RBX::RotateP *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateP::~RotateP()")]
// was: __ZThn32_N3RBX7RotatePD0Ev
// IDA 0x5a4a10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4a10() {
}

// 0x5a4ab4 — __ZThn32_NK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE12getClassNameEv
// IDA 0x5a4ab4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4ab4() {
}

// 0x5a4ac4 — __ZThn36_N3RBX7RotatePD1Ev
// type: void __fastcall(RBX::RotateP *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateP::~RotateP()")]
// was: __ZThn36_N3RBX7RotatePD1Ev
// IDA 0x5a4ac4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4ac4() {
}

// 0x5a4acc — __ZThn36_N3RBX7RotatePD0Ev
// type: void __fastcall(RBX::RotateP *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateP::~RotateP()")]
// was: __ZThn36_N3RBX7RotatePD0Ev
// IDA 0x5a4acc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4acc() {
}

// 0x5a4b70 — __ZN3RBX7RotateVD1Ev
// type: void __fastcall(RBX::RotateV *__hidden this)
#[doc(alias = "RBX::RotateV::~RotateV()")]
// was: __ZN3RBX7RotateVD1Ev
// IDA 0x5a4b70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a4b70() {
}

// 0x5a4b74 — __ZN3RBX7RotateVD0Ev
// type: void __fastcall(RBX::RotateV *__hidden this)
#[doc(alias = "RBX::RotateV::~RotateV()")]
// was: __ZN3RBX7RotateVD0Ev
// IDA 0x5a4b74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4b74() {
}

// 0x5a4c14 — __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv
// IDA 0x5a4c14: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4c14() {
}

// 0x5a4c24 — __ZThn32_N3RBX7RotateVD1Ev
// type: void __fastcall(RBX::RotateV *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateV::~RotateV()")]
// was: __ZThn32_N3RBX7RotateVD1Ev
// IDA 0x5a4c24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4c24() {
}

// 0x5a4c2c — __ZThn32_N3RBX7RotateVD0Ev
// type: void __fastcall(RBX::RotateV *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateV::~RotateV()")]
// was: __ZThn32_N3RBX7RotateVD0Ev
// IDA 0x5a4c2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4c2c() {
}

// 0x5a4cd0 — __ZThn32_NK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE12getClassNameEv
// IDA 0x5a4cd0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4cd0() {
}

// 0x5a4ce0 — __ZThn36_N3RBX7RotateVD1Ev
// type: void __fastcall(RBX::RotateV *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateV::~RotateV()")]
// was: __ZThn36_N3RBX7RotateVD1Ev
// IDA 0x5a4ce0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4ce0() {
}

// 0x5a4ce8 — __ZThn36_N3RBX7RotateVD0Ev
// type: void __fastcall(RBX::RotateV *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateV::~RotateV()")]
// was: __ZThn36_N3RBX7RotateVD0Ev
// IDA 0x5a4ce8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a4ce8() {
}

// 0x5a4d8c — __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_7RotateVENS_13DynamicRotateELZNS_8sRotateVEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a4d8c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4d8c() {
}

// 0x5a4e00 — __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_7RotatePENS_13DynamicRotateELZNS_8sRotatePEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a4e00: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4e00() {
}

// 0x5a4e74 — __ZN3RBX4Name13callDoDeclareILZNS_14sDynamicRotateEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDynamicRotateEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sDynamicRotateEEEEvv
// IDA 0x5a4e74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a4e74() {
}

// 0x5a4e78 — __ZN3RBX4Name9doDeclareILZNS_14sDynamicRotateEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDynamicRotateEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sDynamicRotateEEEERKS0_v
// IDA 0x5a4e78: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4e78() {
}

// 0x5a4f58 — __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_6RotateENS_13JointInstanceELZNS_7sRotateEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a4f58: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4f58() {
}

// 0x5a4fcc — __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_4GlueENS_13JointInstanceELZNS_5sGlueEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a4fcc: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a4fcc() {
}

// 0x5a5040 — __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a5040: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5040() {
}

// 0x5a50b4 — __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5a50b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a50b4() {
}

// 0x5a5150 — __ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5a5150: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5150() {
}

// 0x5a51d8 — __ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7Creator6createEv
// IDA 0x5a51d8: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a51d8() {
}

// 0x5a531c — __ZN3RBX4Name13callDoDeclareILZNS_8sMotor6DEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_8sMotor6DEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_8sMotor6DEEEEvv
// IDA 0x5a531c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a531c() {
}

// 0x5a5320 — __ZN3RBX4Name9doDeclareILZNS_8sMotor6DEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sMotor6DEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_8sMotor6DEEEERKS0_v
// IDA 0x5a5320: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5320() {
}

// 0x5a5400 — __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5a5400: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5400() {
}

// 0x5a5644 — __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_7Motor6DENS_5MotorELZNS_8sMotor6DEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a5644: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5644() {
}

// 0x5a56b8 — __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5a56b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a56b8() {
}

// 0x5a5754 — __ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5a5754: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5754() {
}

// 0x5a57dc — __ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7Creator6createEv
// IDA 0x5a57dc: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a57dc() {
}

// 0x5a5920 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ManualGlueEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualGlue> RBX::Creatable<RBX::Instance>::create<RBX::ManualGlue>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ManualGlueEEEN5boost10shared_ptrIT_EEv
// IDA 0x5a5920: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5920() {
}

// 0x5a59d0 — __ZN5boost10shared_ptrIN3RBX10ManualGlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualGlue>::shared_ptr<RBX::ManualGlue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10ManualGlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5a59d0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a59d0() {
}

// 0x5a5a98 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualGlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualGlue,RBX::ManualGlue>(rbx_core::SharedPtr<RBX::ManualGlue> const*,RBX::ManualGlue *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualGlueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5a5a98: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5a98() {
}

// 0x5a5b80 — __ZN5boost6detail12shared_countC2IPN3RBX10ManualGlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10ManualGlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5a5b80: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5b80() {
}

// 0x5a5c88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5a5c88: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5a5c88() {
}

// 0x5a5c8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5a5c8c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a5c8c() {
}

// 0x5a5c90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5a5c90: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5c90() {
}

// 0x5a5cb0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5a5cb0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5cb0() {
}

// 0x5a5cc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualGlueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5a5cc8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5cc8() {
}

// 0x5a5ccc — __ZN3RBX4Name13callDoDeclareILZNS_11sManualGlueEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sManualGlueEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sManualGlueEEEEvv
// IDA 0x5a5ccc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a5ccc() {
}

// 0x5a5cd0 — __ZN3RBX4Name9doDeclareILZNS_11sManualGlueEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sManualGlueEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sManualGlueEEEERKS0_v
// IDA 0x5a5cd0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5cd0() {
}

// 0x5a5db0 — __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5a5db0: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5db0() {
}

// 0x5a5ff4 — __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10ManualGlueENS_26ManualSurfaceJointInstanceELZNS_11sManualGlueEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a5ff4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a5ff4() {
}

// 0x5a6068 — __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5a6068: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a6068() {
}

// 0x5a6104 — __ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5a6104: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6104() {
}

// 0x5a618c — __ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7Creator6createEv
// IDA 0x5a618c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a618c() {
}

// 0x5a62d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ManualWeldEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualWeld> RBX::Creatable<RBX::Instance>::create<RBX::ManualWeld>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ManualWeldEEEN5boost10shared_ptrIT_EEv
// IDA 0x5a62d0: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a62d0() {
}

// 0x5a6380 — __ZN5boost10shared_ptrIN3RBX10ManualWeldEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualWeld>::shared_ptr<RBX::ManualWeld,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10ManualWeldEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5a6380: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6380() {
}

// 0x5a6448 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualWeldES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualWeld,RBX::ManualWeld>(rbx_core::SharedPtr<RBX::ManualWeld> const*,RBX::ManualWeld *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ManualWeldES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5a6448: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6448() {
}

// 0x5a6530 — __ZN5boost6detail12shared_countC2IPN3RBX10ManualWeldENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10ManualWeldENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5a6530: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6530() {
}

// 0x5a6638 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5a6638: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5a6638() {
}

// 0x5a663c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5a663c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a663c() {
}

// 0x5a6640 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5a6640: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6640() {
}

// 0x5a6660 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5a6660: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6660() {
}

// 0x5a6678 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ManualWeldENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5a6678: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6678() {
}

// 0x5a667c — __ZN3RBX4Name13callDoDeclareILZNS_11sManualWeldEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sManualWeldEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_11sManualWeldEEEEvv
// IDA 0x5a667c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a667c() {
}

// 0x5a6680 — __ZN3RBX4Name9doDeclareILZNS_11sManualWeldEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sManualWeldEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_11sManualWeldEEEERKS0_v
// IDA 0x5a6680: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6680() {
}

// 0x5a6760 — __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5a6760: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6760() {
}

// 0x5a69a4 — __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10ManualWeldENS_26ManualSurfaceJointInstanceELZNS_11sManualWeldEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a69a4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a69a4() {
}

// 0x5a6a18 — __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5a6a18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a6a18() {
}

// 0x5a6ab4 — __ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5a6ab4: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6ab4() {
}

// 0x5a6b3c — __ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7Creator6createEv
// IDA 0x5a6b3c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6b3c() {
}

// 0x5a6c80 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_26ManualSurfaceJointInstanceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance> RBX::Creatable<RBX::Instance>::create<RBX::ManualSurfaceJointInstance>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_26ManualSurfaceJointInstanceEEEN5boost10shared_ptrIT_EEv
// IDA 0x5a6c80: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6c80() {
}

// 0x5a6d30 — __ZN5boost10shared_ptrIN3RBX26ManualSurfaceJointInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance>::shared_ptr<RBX::ManualSurfaceJointInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX26ManualSurfaceJointInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5a6d30: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6d30() {
}

// 0x5a6df8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_26ManualSurfaceJointInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ManualSurfaceJointInstance,RBX::ManualSurfaceJointInstance>(rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance> const*,RBX::ManualSurfaceJointInstance *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_26ManualSurfaceJointInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5a6df8: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6df8() {
}

// 0x5a6ee0 — __ZN5boost6detail12shared_countC2IPN3RBX26ManualSurfaceJointInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX26ManualSurfaceJointInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5a6ee0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6ee0() {
}

// 0x5a6fe8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5a6fe8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5a6fe8() {
}

// 0x5a6fec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5a6fec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a6fec() {
}

// 0x5a6ff0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5a6ff0: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a6ff0() {
}

// 0x5a7010 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5a7010: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7010() {
}

// 0x5a7028 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX26ManualSurfaceJointInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5a7028: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7028() {
}

// 0x5a702c — __ZN3RBX4Name13callDoDeclareILZNS_27sManualSurfaceJointInstanceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_27sManualSurfaceJointInstanceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_27sManualSurfaceJointInstanceEEEEvv
// IDA 0x5a702c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a702c() {
}

// 0x5a7030 — __ZN3RBX4Name9doDeclareILZNS_27sManualSurfaceJointInstanceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_27sManualSurfaceJointInstanceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_27sManualSurfaceJointInstanceEEEERKS0_v
// IDA 0x5a7030: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7030() {
}

// 0x5a7110 — __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5a7110: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7110() {
}

// 0x5a7354 — __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_26ManualSurfaceJointInstanceENS_13JointInstanceELZNS_27sManualSurfaceJointInstanceEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a7354: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7354() {
}

// 0x5a73c8 — __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5a73c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a73c8() {
}

// 0x5a7464 — __ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5a7464: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7464() {
}

// 0x5a74ec — __ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7Creator6createEv
// IDA 0x5a74ec: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a74ec() {
}

// 0x5a7630 — __ZN3RBX4Name13callDoDeclareILZNS_5sWeldEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sWeldEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5sWeldEEEEvv
// IDA 0x5a7630: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a7630() {
}

// 0x5a7634 — __ZN3RBX4Name9doDeclareILZNS_5sWeldEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sWeldEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5sWeldEEEERKS0_v
// IDA 0x5a7634: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7634() {
}

// 0x5a7714 — __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5a7714: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7714() {
}

// 0x5a7958 — __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_4WeldENS_13JointInstanceELZNS_5sWeldEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a7958: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7958() {
}

// 0x5a79cc — __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5a79cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a79cc() {
}

// 0x5a7a68 — __ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5a7a68: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7a68() {
}

// 0x5a7af0 — __ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7Creator6createEv
// IDA 0x5a7af0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7af0() {
}

// 0x5a7c34 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5MotorEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Motor> RBX::Creatable<RBX::Instance>::create<RBX::Motor>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5MotorEEEN5boost10shared_ptrIT_EEv
// IDA 0x5a7c34: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7c34() {
}

// 0x5a7ce4 — __ZN5boost10shared_ptrIN3RBX5MotorEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Motor>::shared_ptr<RBX::Motor,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5MotorEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5a7ce4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7ce4() {
}

// 0x5a7dac — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MotorES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Motor,RBX::Motor>(rbx_core::SharedPtr<RBX::Motor> const*,RBX::Motor *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MotorES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5a7dac: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7dac() {
}

// 0x5a7e94 — __ZN5boost6detail12shared_countC2IPN3RBX5MotorENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5MotorENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5a7e94: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7e94() {
}

// 0x5a7f9c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5a7f9c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5a7f9c() {
}

// 0x5a7fa0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5a7fa0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a7fa0() {
}

// 0x5a7fa4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5a7fa4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7fa4() {
}

// 0x5a7fc4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5a7fc4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7fc4() {
}

// 0x5a7fdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MotorENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5a7fdc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7fdc() {
}

// 0x5a7fe0 — __ZN3RBX4Name13callDoDeclareILZNS_6sMotorEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sMotorEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_6sMotorEEEEvv
// IDA 0x5a7fe0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a7fe0() {
}

// 0x5a7fe4 — __ZN3RBX4Name9doDeclareILZNS_6sMotorEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sMotorEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_6sMotorEEEERKS0_v
// IDA 0x5a7fe4: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a7fe4() {
}

// 0x5a80c4 — __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5a80c4: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a80c4() {
}

// 0x5a8308 — __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_5MotorENS_13JointInstanceELZNS_6sMotorEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5a8308: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a8308() {
}

// 0x5a837c — __ZN3RBX4Name13callDoDeclareILZNS_14sJointInstanceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sJointInstanceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sJointInstanceEEEEvv
// IDA 0x5a837c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a837c() {
}

// 0x5a8380 — __ZN3RBX4Name9doDeclareILZNS_14sJointInstanceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sJointInstanceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sJointInstanceEEEERKS0_v
// IDA 0x5a8380: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5a8380() {
}

// 0x5a8460 — __ZN3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5a8460: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a8460() {
}

// 0x5a8464 — __ZN3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5a8464: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a8464() {
}

// 0x5a8504 — __ZThn32_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5a8504: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a8504() {
}

// 0x5a850c — __ZThn32_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5a850c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a850c() {
}

// 0x5a85b0 — __ZThn36_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5a85b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a85b0() {
}

// 0x5a85b8 — __ZThn36_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_7Motor6DELZNS_8sMotor6DEENS_14FactoryProductIS2_NS_5MotorELZNS_8sMotor6DEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5a85b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5a85b8() {
}

// 0x5a865c — __ZN3RBX10Reflection9DescribedINS_5MotorELZNS_6sMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_6sMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5MotorELZNS_6sMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_6sMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5MotorELZNS_6sMotorEENS_14FactoryProductIS2_NS_13JointInstanceELZNS_6sMotorEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5a865c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5a865c() {
}

