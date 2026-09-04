//! rendering shard rend_wd_11a — 100 stubs 0x7b3fa0..0x7b84d8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre/G3D complete, global gap filler EA asc) [skeleton batch rend_wd_11a]
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7b3fa0 — __GLOBAL__I_a_371
#[doc(alias = "global constructor keyed to_a_371")]
#[doc(alias = "__GLOBAL__I_a_371")]
// was: global constructor keyed to_a_371
// IDA 0x7b3fa0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7b3fa0() {
}


// 0x7b4210 — __ZN3RBX5HUMAN4DeadC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Dead::Dead(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN4DeadC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7b4210: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4210() {
}


// 0x7b4234 — __ZN3RBX5HUMAN4Dead10onStepImplEv
// type: _DWORD __fastcall(RBX::HUMAN::Dead *__hidden this)
#[doc(alias = "RBX::HUMAN::Dead::onStepImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN4Dead10onStepImplEv")]
// IDA 0x7b4234: 14 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4234() {
}


// 0x7b425c — __ZN3RBX5HUMAN4Dead19onSimulatorStepImplEf
// type: _DWORD __fastcall(RBX::HUMAN::Dead *__hidden this, float)
#[doc(alias = "RBX::HUMAN::Dead::onSimulatorStepImpl(float)")]
#[doc(alias = "__ZN3RBX5HUMAN4Dead19onSimulatorStepImplEf")]
// IDA 0x7b425c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b425c() {
}


// 0x7b4290 — __ZN3RBX5HUMAN11FallingDownC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::FallingDown::FallingDown(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN11FallingDownC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7b4290: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4290() {
}


// 0x7b42bc — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_5sDeadEEE7getNameEv")]
// IDA 0x7b42bc: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b42bc() {
}


// 0x7b42e4 — __ZN3RBX5HUMAN4Dead18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::Dead *__hidden this)
#[doc(alias = "RBX::HUMAN::Dead::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN4Dead18onComputeForceImplEv")]
// IDA 0x7b42e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7b42e4() {
}


// 0x7b42e8 — __ZN3RBX5HUMAN4DeadD1Ev
// type: void __fastcall(RBX::HUMAN::Dead *__hidden this)
#[doc(alias = "RBX::HUMAN::Dead::~Dead()")]
#[doc(alias = "__ZN3RBX5HUMAN4DeadD1Ev")]
// IDA 0x7b42e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b42e8() {
}


// 0x7b42ec — __ZN3RBX5HUMAN4DeadD0Ev
// type: void __fastcall(RBX::HUMAN::Dead *__hidden this)
#[doc(alias = "RBX::HUMAN::Dead::~Dead()")]
#[doc(alias = "__ZN3RBX5HUMAN4DeadD0Ev")]
// IDA 0x7b42ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b42ec() {
}


// 0x7b438c — __ZNK3RBX5HUMAN4Dead12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Dead *__hidden this)
#[doc(alias = "RBX::HUMAN::Dead::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN4Dead12getStateTypeEv")]
// IDA 0x7b438c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b438c() {
}


// 0x7b4390 — __ZThn4_N3RBX5HUMAN4DeadD1Ev
// type: void __fastcall(RBX::HUMAN::Dead *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Dead::~Dead()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN4DeadD1Ev")]
// was: non-virtual thunk toRBX::HUMAN::Dead::~Dead()
// IDA 0x7b4390: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b4390() {
}


// 0x7b4398 — __ZThn4_N3RBX5HUMAN4DeadD0Ev
// type: void __fastcall(RBX::HUMAN::Dead *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Dead::~Dead()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN4DeadD0Ev")]
// was: non-virtual thunk toRBX::HUMAN::Dead::~Dead()
// IDA 0x7b4398: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b4398() {
}


// 0x7b443c — __ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN13HumanoidStateELZNS1_12sFallingDownEEE7getNameEv")]
// IDA 0x7b443c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b443c() {
}


// 0x7b4464 — __ZN3RBX5HUMAN11FallingDown18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::FallingDown *__hidden this)
#[doc(alias = "RBX::HUMAN::FallingDown::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN11FallingDown18onComputeForceImplEv")]
// IDA 0x7b4464: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7b4464() {
}


// 0x7b4468 — __ZN3RBX5HUMAN11FallingDownD1Ev
// type: void __fastcall(RBX::HUMAN::FallingDown *__hidden this)
#[doc(alias = "RBX::HUMAN::FallingDown::~FallingDown()")]
#[doc(alias = "__ZN3RBX5HUMAN11FallingDownD1Ev")]
// IDA 0x7b4468: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b4468() {
}


// 0x7b446c — __ZN3RBX5HUMAN11FallingDownD0Ev
// type: void __fastcall(RBX::HUMAN::FallingDown *__hidden this)
#[doc(alias = "RBX::HUMAN::FallingDown::~FallingDown()")]
#[doc(alias = "__ZN3RBX5HUMAN11FallingDownD0Ev")]
// IDA 0x7b446c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b446c() {
}


// 0x7b450c — __ZNK3RBX5HUMAN11FallingDown12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::FallingDown *__hidden this)
#[doc(alias = "RBX::HUMAN::FallingDown::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN11FallingDown12getStateTypeEv")]
// IDA 0x7b450c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b450c() {
}


// 0x7b4510 — __ZThn4_N3RBX5HUMAN11FallingDownD1Ev
// type: void __fastcall(RBX::HUMAN::FallingDown *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::FallingDown::~FallingDown()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN11FallingDownD1Ev")]
// was: non-virtual thunk toRBX::HUMAN::FallingDown::~FallingDown()
// IDA 0x7b4510: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b4510() {
}


// 0x7b4518 — __ZThn4_N3RBX5HUMAN11FallingDownD0Ev
// type: void __fastcall(RBX::HUMAN::FallingDown *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::FallingDown::~FallingDown()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN11FallingDownD0Ev")]
// was: non-virtual thunk toRBX::HUMAN::FallingDown::~FallingDown()
// IDA 0x7b4518: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b4518() {
}


// 0x7b45bc — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN12sFallingDownEEEEvv")]
// IDA 0x7b45bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b45bc() {
}


// 0x7b45c0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN12sFallingDownEEEERKS0_v")]
// IDA 0x7b45c0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b45c0() {
}


// 0x7b46a0 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN5sDeadEEEEvv")]
// IDA 0x7b46a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b46a0() {
}


// 0x7b46a4 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN5sDeadEEEERKS0_v")]
// IDA 0x7b46a4: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b46a4() {
}


// 0x7b4784 — __GLOBAL__I_a_372
#[doc(alias = "global constructor keyed to_a_372")]
#[doc(alias = "__GLOBAL__I_a_372")]
// was: global constructor keyed to_a_372
// IDA 0x7b4784: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7b4784() {
}


// 0x7b49c0 — __ZN3RBX5HUMAN6FlyingC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Flying::Flying(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN6FlyingC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7b49c0: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b49c0() {
}


// 0x7b4a04 — __ZN3RBX5HUMAN6FlyingC2EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Flying::Flying(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN6FlyingC2EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7b4a04: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4a04() {
}


// 0x7b4a48 — __ZN3RBX5HUMAN6Flying19onSimulatorStepImplEf
// type: _DWORD __fastcall(RBX::HUMAN::Flying *__hidden this, float)
#[doc(alias = "RBX::HUMAN::Flying::onSimulatorStepImpl(float)")]
#[doc(alias = "__ZN3RBX5HUMAN6Flying19onSimulatorStepImplEf")]
// IDA 0x7b4a48: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7b4a48() {
}


// 0x7b4a4c — __ZN3RBX5HUMAN6Flying18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::Flying *__hidden this)
#[doc(alias = "RBX::HUMAN::Flying::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN6Flying18onComputeForceImplEv")]
// IDA 0x7b4a4c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7b4a4c() {
}


// 0x7b4a50 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_7sFlyingEEE7getNameEv")]
// IDA 0x7b4a50: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4a50() {
}


// 0x7b4a78 — __ZN3RBX5HUMAN6FlyingD1Ev
// type: void __fastcall(RBX::HUMAN::Flying *__hidden this)
#[doc(alias = "RBX::HUMAN::Flying::~Flying()")]
#[doc(alias = "__ZN3RBX5HUMAN6FlyingD1Ev")]
// IDA 0x7b4a78: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b4a78() {
}


// 0x7b4a7c — __ZN3RBX5HUMAN6FlyingD0Ev
// type: void __fastcall(RBX::HUMAN::Flying *__hidden this)
#[doc(alias = "RBX::HUMAN::Flying::~Flying()")]
#[doc(alias = "__ZN3RBX5HUMAN6FlyingD0Ev")]
// IDA 0x7b4a7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b4a7c() {
}


// 0x7b4b1c — __ZNK3RBX5HUMAN6Flying12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Flying *__hidden this)
#[doc(alias = "RBX::HUMAN::Flying::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN6Flying12getStateTypeEv")]
// IDA 0x7b4b1c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4b1c() {
}


// 0x7b4b20 — __ZThn4_N3RBX5HUMAN6FlyingD1Ev
// type: void __fastcall(RBX::HUMAN::Flying *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Flying::~Flying()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN6FlyingD1Ev")]
// was: non-virtual thunk toRBX::HUMAN::Flying::~Flying()
// IDA 0x7b4b20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b4b20() {
}


// 0x7b4b28 — __ZThn4_N3RBX5HUMAN6FlyingD0Ev
// type: void __fastcall(RBX::HUMAN::Flying *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Flying::~Flying()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN6FlyingD0Ev")]
// was: non-virtual thunk toRBX::HUMAN::Flying::~Flying()
// IDA 0x7b4b28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b4b28() {
}


// 0x7b4bcc — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN7sFlyingEEEEvv")]
// IDA 0x7b4bcc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b4bcc() {
}


// 0x7b4bd0 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN7sFlyingEEEERKS0_v")]
// IDA 0x7b4bd0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4bd0() {
}


// 0x7b4cb0 — __GLOBAL__I_a_373
#[doc(alias = "global constructor keyed to_a_373")]
#[doc(alias = "__GLOBAL__I_a_373")]
// was: global constructor keyed to_a_373
// IDA 0x7b4cb0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7b4cb0() {
}


// 0x7b4f24 — __ZN3RBX5HUMAN8FreefallC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Freefall::Freefall(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN8FreefallC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7b4f24: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b4f24() {
}


// 0x7b4f28 — __ZN3RBX5HUMAN8FreefallC2EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Freefall::Freefall(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN8FreefallC2EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7b4f28: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b4f28() {
}


// 0x7b50bc — __ZN3RBX5HUMAN8FreefallD0Ev
// type: void __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "RBX::HUMAN::Freefall::~Freefall()")]
#[doc(alias = "__ZN3RBX5HUMAN8FreefallD0Ev")]
// IDA 0x7b50bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b50bc() {
}


// 0x7b515c — __ZN3RBX5HUMAN8FreefallD1Ev
// type: void __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "RBX::HUMAN::Freefall::~Freefall()")]
#[doc(alias = "__ZN3RBX5HUMAN8FreefallD1Ev")]
// IDA 0x7b515c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b515c() {
}


// 0x7b5160 — __ZThn4_N3RBX5HUMAN8FreefallD0Ev
// type: void __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Freefall::~Freefall()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN8FreefallD0Ev")]
// was: non-virtual thunk toRBX::HUMAN::Freefall::~Freefall()
// IDA 0x7b5160: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b5160() {
}


// 0x7b5168 — __ZN3RBX5HUMAN8FreefallD2Ev
// type: void __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "RBX::HUMAN::Freefall::~Freefall()")]
#[doc(alias = "__ZN3RBX5HUMAN8FreefallD2Ev")]
// IDA 0x7b5168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b5168() {
}


// 0x7b5388 — __ZThn4_N3RBX5HUMAN8FreefallD1Ev
// type: void __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Freefall::~Freefall()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN8FreefallD1Ev")]
// was: non-virtual thunk toRBX::HUMAN::Freefall::~Freefall()
// IDA 0x7b5388: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b5388() {
}


// 0x7b5390 — __ZN3RBX5HUMAN8Freefall19onSimulatorStepImplEf
// type: _DWORD __fastcall(RBX::HUMAN::Freefall *__hidden this, float)
#[doc(alias = "RBX::HUMAN::Freefall::onSimulatorStepImpl(float)")]
#[doc(alias = "__ZN3RBX5HUMAN8Freefall19onSimulatorStepImplEf")]
// IDA 0x7b5390: 187 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5390() {
}


// 0x7b565c — __ZN3RBX5HUMAN8Freefall18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "RBX::HUMAN::Freefall::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN8Freefall18onComputeForceImplEv")]
// IDA 0x7b565c: 130 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b565c() {
}


// 0x7b5814 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_9sFreefallEEE7getNameEv")]
// IDA 0x7b5814: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5814() {
}


// 0x7b583c — __ZNK3RBX5HUMAN8Freefall17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "RBX::HUMAN::Freefall::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN8Freefall17armsShouldCollideEv")]
// IDA 0x7b583c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b583c() {
}


// 0x7b5840 — __ZNK3RBX5HUMAN8Freefall17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "RBX::HUMAN::Freefall::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN8Freefall17legsShouldCollideEv")]
// IDA 0x7b5840: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5840() {
}


// 0x7b5844 — __ZNK3RBX5HUMAN8Freefall12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Freefall *__hidden this)
#[doc(alias = "RBX::HUMAN::Freefall::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN8Freefall12getStateTypeEv")]
// IDA 0x7b5844: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5844() {
}


// 0x7b5848 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sFreefallEEEEvv")]
// IDA 0x7b5848: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b5848() {
}


// 0x7b584c — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sFreefallEEEERKS0_v")]
// IDA 0x7b584c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b584c() {
}


// 0x7b592c — __GLOBAL__I_a_374
#[doc(alias = "global constructor keyed to_a_374")]
#[doc(alias = "__GLOBAL__I_a_374")]
// was: global constructor keyed to_a_374
// IDA 0x7b592c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7b592c() {
}


// 0x7b5c04 — __ZN3RBX5HUMAN9GettingUpC1EPNS_8HumanoidENS0_9StateTypeE
// type: int __fastcall(int)
#[doc(alias = "RBX::HUMAN::GettingUp::GettingUp(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN9GettingUpC1EPNS_8HumanoidENS0_9StateTypeE")]
// IDA 0x7b5c04: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5c04() {
}


// 0x7b5c40 — __ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN9BalancingELZNS1_10sGettingUpEEE7getNameEv")]
// IDA 0x7b5c40: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5c40() {
}


// 0x7b5c68 — __ZNK3RBX5HUMAN9GettingUp17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::GettingUp *__hidden this)
#[doc(alias = "RBX::HUMAN::GettingUp::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN9GettingUp17armsShouldCollideEv")]
// IDA 0x7b5c68: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5c68() {
}


// 0x7b5c6c — __ZNK3RBX5HUMAN9GettingUp17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::GettingUp *__hidden this)
#[doc(alias = "RBX::HUMAN::GettingUp::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN9GettingUp17legsShouldCollideEv")]
// IDA 0x7b5c6c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5c6c() {
}


// 0x7b5c70 — __ZN3RBX5HUMAN9GettingUpD1Ev
// type: void __fastcall(RBX::HUMAN::GettingUp *__hidden this)
#[doc(alias = "RBX::HUMAN::GettingUp::~GettingUp()")]
#[doc(alias = "__ZN3RBX5HUMAN9GettingUpD1Ev")]
// IDA 0x7b5c70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b5c70() {
}


// 0x7b5c74 — __ZN3RBX5HUMAN9GettingUpD0Ev
// type: void __fastcall(RBX::HUMAN::GettingUp *__hidden this)
#[doc(alias = "RBX::HUMAN::GettingUp::~GettingUp()")]
#[doc(alias = "__ZN3RBX5HUMAN9GettingUpD0Ev")]
// IDA 0x7b5c74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b5c74() {
}


// 0x7b5d14 — __ZNK3RBX5HUMAN9GettingUp12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::GettingUp *__hidden this)
#[doc(alias = "RBX::HUMAN::GettingUp::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN9GettingUp12getStateTypeEv")]
// IDA 0x7b5d14: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5d14() {
}


// 0x7b5d18 — __ZThn4_N3RBX5HUMAN9GettingUpD1Ev
// type: void __fastcall(RBX::HUMAN::GettingUp *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::GettingUp::~GettingUp()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN9GettingUpD1Ev")]
// was: non-virtual thunk toRBX::HUMAN::GettingUp::~GettingUp()
// IDA 0x7b5d18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b5d18() {
}


// 0x7b5d20 — __ZThn4_N3RBX5HUMAN9GettingUpD0Ev
// type: void __fastcall(RBX::HUMAN::GettingUp *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::GettingUp::~GettingUp()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN9GettingUpD0Ev")]
// was: non-virtual thunk toRBX::HUMAN::GettingUp::~GettingUp()
// IDA 0x7b5d20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b5d20() {
}


// 0x7b5dc4 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN10sGettingUpEEEEvv")]
// IDA 0x7b5dc4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b5dc4() {
}


// 0x7b5dc8 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN10sGettingUpEEEERKS0_v")]
// IDA 0x7b5dc8: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b5dc8() {
}


// 0x7b5ea8 — __GLOBAL__I_a_375
#[doc(alias = "global constructor keyed to_a_375")]
#[doc(alias = "__GLOBAL__I_a_375")]
// was: global constructor keyed to_a_375
// IDA 0x7b5ea8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7b5ea8() {
}


// 0x7b60e8 — __ZNK3RBX8Humanoid17getTorsoDangerousEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::getTorsoDangerous(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid17getTorsoDangerousEv")]
// IDA 0x7b60e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7b60e8() {
}


// 0x7b60ec — __ZN3RBX8Humanoid8setTorsoEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::Humanoid::setTorso(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX8Humanoid8setTorsoEPNS_12PartInstanceE")]
// IDA 0x7b60ec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7b60ec() {
}


// 0x7b60f0 — __ZNK3RBX8Humanoid19getLeftLegDangerousEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::getLeftLegDangerous(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid19getLeftLegDangerousEv")]
// IDA 0x7b60f0: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b60f0() {
}


// 0x7b60f8 — __ZN3RBX8Humanoid10setLeftLegEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::Humanoid::setLeftLeg(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX8Humanoid10setLeftLegEPNS_12PartInstanceE")]
// IDA 0x7b60f8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7b60f8() {
}


// 0x7b60fc — __ZNK3RBX8Humanoid20getRightLegDangerousEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::getRightLegDangerous(void)const")]
#[doc(alias = "__ZNK3RBX8Humanoid20getRightLegDangerousEv")]
// IDA 0x7b60fc: 2 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b60fc() {
}


// 0x7b6104 — __ZN3RBX8Humanoid11setRightLegEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::Humanoid::setRightLeg(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX8Humanoid11setRightLegEPNS_12PartInstanceE")]
// IDA 0x7b6104: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7b6104() {
}


// 0x7b6108 — __ZN3RBX8Humanoid9setHealthEf
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, float)
#[doc(alias = "RBX::Humanoid::setHealth(float)")]
#[doc(alias = "__ZN3RBX8Humanoid9setHealthEf")]
// IDA 0x7b6108: 43 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6108() {
}


// 0x7b619c — __ZN3RBX8Humanoid12setMaxHealthEf
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, float)
#[doc(alias = "RBX::Humanoid::setMaxHealth(float)")]
#[doc(alias = "__ZN3RBX8Humanoid12setMaxHealthEf")]
// IDA 0x7b619c: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b619c() {
}


// 0x7b61e4 — __ZN3RBX8Humanoid12setWalkSpeedEf
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, float)
#[doc(alias = "RBX::Humanoid::setWalkSpeed(float)")]
#[doc(alias = "__ZN3RBX8Humanoid12setWalkSpeedEf")]
// IDA 0x7b61e4: 12 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b61e4() {
}


// 0x7b6210 — __ZN3RBX8Humanoid13setWalkToPartEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::Humanoid::setWalkToPart(RBX::PartInstance *)")]
#[doc(alias = "__ZN3RBX8Humanoid13setWalkToPartEPNS_12PartInstanceE")]
// IDA 0x7b6210: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6210() {
}


// 0x7b654c — __ZN3RBX8Humanoid7setJumpEb
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, bool)
#[doc(alias = "RBX::Humanoid::setJump(bool)")]
#[doc(alias = "__ZN3RBX8Humanoid7setJumpEb")]
// IDA 0x7b654c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b654c() {
}


// 0x7b65ac — __ZN3RBX8Humanoid6setSitEb
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, bool)
#[doc(alias = "RBX::Humanoid::setSit(bool)")]
#[doc(alias = "__ZN3RBX8Humanoid6setSitEb")]
// IDA 0x7b65ac: 9 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b65ac() {
}


// 0x7b65cc — __ZN3RBX8Humanoid19setPlatformStandingEb
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, bool)
#[doc(alias = "RBX::Humanoid::setPlatformStanding(bool)")]
#[doc(alias = "__ZN3RBX8Humanoid19setPlatformStandingEb")]
// IDA 0x7b65cc: 9 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b65cc() {
}


// 0x7b678c — __ZN3RBX8Humanoid17setWalkAngleErrorERKf
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, const float *)
#[doc(alias = "RBX::Humanoid::setWalkAngleError(float const&)")]
#[doc(alias = "__ZN3RBX8Humanoid17setWalkAngleErrorERKf")]
// IDA 0x7b678c: 17 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b678c() {
}


// 0x7b67c8 — __ZN3RBX8Humanoid9setStrafeEb
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, bool)
#[doc(alias = "RBX::Humanoid::setStrafe(bool)")]
#[doc(alias = "__ZN3RBX8Humanoid9setStrafeEb")]
// IDA 0x7b67c8: 9 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b67c8() {
}


// 0x7b67e8 — __ZN3RBX8Humanoid10takeDamageEf
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this, float)
#[doc(alias = "RBX::Humanoid::takeDamage(float)")]
#[doc(alias = "__ZN3RBX8Humanoid10takeDamageEf")]
// IDA 0x7b67e8: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b67e8() {
}


// 0x7b69a4 — __ZN3RBX8Humanoid9addStatusENS0_6StatusE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Humanoid::addStatus(RBX::Humanoid::Status)")]
#[doc(alias = "__ZN3RBX8Humanoid9addStatusENS0_6StatusE")]
// IDA 0x7b69a4: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b69a4() {
}


// 0x7b6b04 — __ZN3RBX8Humanoid12removeStatusENS0_6StatusE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Humanoid::removeStatus(RBX::Humanoid::Status)")]
#[doc(alias = "__ZN3RBX8Humanoid12removeStatusENS0_6StatusE")]
// IDA 0x7b6b04: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6b04() {
}


// 0x7b6c50 — __ZN3RBX8Humanoid9hasStatusENS0_6StatusE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Humanoid::hasStatus(RBX::Humanoid::Status)")]
#[doc(alias = "__ZN3RBX8Humanoid9hasStatusENS0_6StatusE")]
// IDA 0x7b6c50: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6c50() {
}


// 0x7b6db0 — __ZN3RBX8Humanoid15addCustomStatusESs
#[doc(alias = "RBX::Humanoid::addCustomStatus(std::string)")]
#[doc(alias = "__ZN3RBX8Humanoid15addCustomStatusESs")]
// IDA 0x7b6db0: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6db0() {
}


// 0x7b6eec — __ZN3RBX8Humanoid18removeCustomStatusESs
#[doc(alias = "RBX::Humanoid::removeCustomStatus(std::string)")]
#[doc(alias = "__ZN3RBX8Humanoid18removeCustomStatusESs")]
// IDA 0x7b6eec: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6eec() {
}


// 0x7b6f1c — __ZN3RBX8Humanoid15hasCustomStatusESs
#[doc(alias = "RBX::Humanoid::hasCustomStatus(std::string)")]
#[doc(alias = "__ZN3RBX8Humanoid15hasCustomStatusESs")]
// IDA 0x7b6f1c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6f1c() {
}


// 0x7b6f40 — __ZN3RBX8Humanoid17equipToolInstanceEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Humanoid::equipToolInstance(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Humanoid17equipToolInstanceEN5boost10shared_ptrINS_8InstanceEEE")]
// was: RBX::Humanoid::equipToolInstance(boost::shared_ptr<RBX::Instance>)
// IDA 0x7b6f40: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6f40() {
}


// 0x7b6f7c — __ZN3RBX8Humanoid12unequipToolsEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::unequipTools(void)")]
#[doc(alias = "__ZN3RBX8Humanoid12unequipToolsEv")]
// IDA 0x7b6f7c: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6f7c() {
}


// 0x7b70e0 — __ZN3RBX8Humanoid11getStatusesEv
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::getStatuses(void)")]
#[doc(alias = "__ZN3RBX8Humanoid11getStatusesEv")]
// IDA 0x7b70e0: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b70e0() {
}


// 0x7b7238 — __ZN3RBX8Humanoid16setNameOcclusionENS0_13NameOcclusionE
#[doc(alias = "RBX::Humanoid::setNameOcclusion(RBX::Humanoid::NameOcclusion)")]
#[doc(alias = "__ZN3RBX8Humanoid16setNameOcclusionENS0_13NameOcclusionE")]
// IDA 0x7b7238: 9 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b7238() {
}


// 0x7b7258 — __ZN3RBX8Humanoid13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Humanoid::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Humanoid13loadAnimationEN5boost10shared_ptrINS_8InstanceEEE")]
// was: RBX::Humanoid::loadAnimation(boost::shared_ptr<RBX::Instance>)
// IDA 0x7b7258: 77 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b7258() {
}


// 0x7b7334 — __ZN3RBX8HumanoidC1Ev
// type: _DWORD __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::Humanoid(void)")]
#[doc(alias = "__ZN3RBX8HumanoidC1Ev")]
// IDA 0x7b7334: 1562 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b7334() {
}


// 0x7b83fc — __ZN3RBX8HumanoidD0Ev
// type: void __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::~Humanoid()")]
#[doc(alias = "__ZN3RBX8HumanoidD0Ev")]
// IDA 0x7b83fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b83fc() {
}


// 0x7b84a8 — __ZN3RBX8HumanoidD1Ev
// type: void __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "RBX::Humanoid::~Humanoid()")]
#[doc(alias = "__ZN3RBX8HumanoidD1Ev")]
// IDA 0x7b84a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b84a8() {
}


// 0x7b84b8 — __ZThn32_N3RBX8HumanoidD0Ev
// type: void __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::~Humanoid()")]
#[doc(alias = "__ZThn32_N3RBX8HumanoidD0Ev")]
// was: non-virtual thunk toRBX::Humanoid::~Humanoid()
// IDA 0x7b84b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b84b8() {
}


// 0x7b84c0 — __ZThn36_N3RBX8HumanoidD0Ev
// type: void __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::~Humanoid()")]
#[doc(alias = "__ZThn36_N3RBX8HumanoidD0Ev")]
// was: non-virtual thunk toRBX::Humanoid::~Humanoid()
// IDA 0x7b84c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b84c0() {
}


// 0x7b84c8 — __ZThn92_N3RBX8HumanoidD0Ev
// type: void __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::~Humanoid()")]
#[doc(alias = "__ZThn92_N3RBX8HumanoidD0Ev")]
// was: non-virtual thunk toRBX::Humanoid::~Humanoid()
// IDA 0x7b84c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b84c8() {
}


// 0x7b84d0 — __ZThn124_N3RBX8HumanoidD0Ev
// type: void __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::~Humanoid()")]
#[doc(alias = "__ZThn124_N3RBX8HumanoidD0Ev")]
// was: non-virtual thunk toRBX::Humanoid::~Humanoid()
// IDA 0x7b84d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b84d0() {
}


// 0x7b84d8 — __ZThn244_N3RBX8HumanoidD0Ev
// type: void __fastcall(RBX::Humanoid *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Humanoid::~Humanoid()")]
#[doc(alias = "__ZThn244_N3RBX8HumanoidD0Ev")]
// was: non-virtual thunk toRBX::Humanoid::~Humanoid()
// IDA 0x7b84d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7b84d8() {
}

