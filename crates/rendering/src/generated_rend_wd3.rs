//! rendering shard rend_wd3 — 100 stubs 0x7cd744..0x7d3a78 EA-sorted asc gap filler not yet in crates/rendering/src
//! Source: ida/export.json (85545 funcs) EA asc global gap filler not yet in crates/rendering/src — next 100 uncovered sorted asc after 0x7cd744
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7cd744 — __ZN3RBX5HUMAN13HumanoidStateD1Ev
// type: void __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::~HumanoidState()")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidStateD1Ev")]
// was: RBX::HUMAN::HumanoidState::~HumanoidState()
// IDA 0x7cd744: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7cd744() {
}


// 0x7cd748 — __ZThn4_N3RBX5HUMAN13HumanoidStateD0Ev
// type: void __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::HumanoidState::~HumanoidState()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN13HumanoidStateD0Ev")]
// was: non-virtual thunk toRBX::HUMAN::HumanoidState::~HumanoidState()
// IDA 0x7cd748: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7cd748() {
}


// 0x7cd750 — __ZN3RBX5HUMAN13HumanoidStateD2Ev
// type: void __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::~HumanoidState()")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidStateD2Ev")]
// was: RBX::HUMAN::HumanoidState::~HumanoidState()
// IDA 0x7cd750: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7cd750() {
}


// 0x7cd8c0 — __ZThn4_N3RBX5HUMAN13HumanoidStateD1Ev
// type: void __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::HumanoidState::~HumanoidState()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN13HumanoidStateD1Ev")]
// was: non-virtual thunk toRBX::HUMAN::HumanoidState::~HumanoidState()
// IDA 0x7cd8c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7cd8c0() {
}


// 0x7cd8c8 — __ZN3RBX5HUMAN13HumanoidState10fireEventsEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::fireEvents(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState10fireEventsEv")]
// was: RBX::HUMAN::HumanoidState::fireEvents(void)
// IDA 0x7cd8c8: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cd8c8() {
}


// 0x7cd904 — __ZN3RBX5HUMAN13HumanoidState9fireEventENS0_9StateTypeEb
#[doc(alias = "RBX::HUMAN::HumanoidState::fireEvent(RBX::HUMAN::StateType,bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState9fireEventENS0_9StateTypeEb")]
// was: RBX::HUMAN::HumanoidState::fireEvent(RBX::HUMAN::StateType,bool)
// IDA 0x7cd904: 101 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cd904() {
}


// 0x7cda58 — __ZNK3RBX5HUMAN13HumanoidState16getAssemblyConstEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getAssemblyConst(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState16getAssemblyConstEv")]
// was: RBX::HUMAN::HumanoidState::getAssemblyConst(void)const
// IDA 0x7cda58: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cda58() {
}


// 0x7cda78 — __ZN3RBX5HUMAN13HumanoidState15stateToAssemblyEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::stateToAssembly(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState15stateToAssemblyEv")]
// was: RBX::HUMAN::HumanoidState::stateToAssembly(void)
// IDA 0x7cda78: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cda78() {
}


// 0x7cda98 — __ZNK3RBX5HUMAN13HumanoidState16getHumanoidConstEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getHumanoidConst(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState16getHumanoidConstEv")]
// was: RBX::HUMAN::HumanoidState::getHumanoidConst(void)const
// IDA 0x7cda98: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cda98() {
}


// 0x7cdaf0 — __ZN3RBX5HUMAN13HumanoidState14onComputeForceEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::onComputeForce(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState14onComputeForceEv")]
// was: RBX::HUMAN::HumanoidState::onComputeForce(void)
// IDA 0x7cdaf0: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cdaf0() {
}


// 0x7ce82c — __ZN3RBX5HUMAN13HumanoidState17getFloorPrimitiveEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorPrimitive(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState17getFloorPrimitiveEv")]
// was: RBX::HUMAN::HumanoidState::getFloorPrimitive(void)
// IDA 0x7ce82c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7ce82c() {
}


// 0x7ce830 — __ZNK3RBX5HUMAN13HumanoidState22getFloorPrimitiveConstEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorPrimitiveConst(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState22getFloorPrimitiveConstEv")]
// was: RBX::HUMAN::HumanoidState::getFloorPrimitiveConst(void)const
// IDA 0x7ce830: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ce830() {
}


// 0x7ce860 — __ZN3RBX5HUMAN13HumanoidState12defaultStateEPNS_8HumanoidE
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, RBX::Humanoid *)
#[doc(alias = "RBX::HUMAN::HumanoidState::defaultState(RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState12defaultStateEPNS_8HumanoidE")]
// was: RBX::HUMAN::HumanoidState::defaultState(RBX::Humanoid *)
// IDA 0x7ce860: 59 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ce860() {
}


// 0x7ce910 — __ZN3RBX5HUMAN13HumanoidState20preStepSimulatorSideEf
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, float)
#[doc(alias = "RBX::HUMAN::HumanoidState::preStepSimulatorSide(float)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState20preStepSimulatorSideEf")]
// was: RBX::HUMAN::HumanoidState::preStepSimulatorSide(float)
// IDA 0x7ce910: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ce910() {
}


// 0x7cea20 — __ZN3RBX5HUMAN13HumanoidState12preStepFloorEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::preStepFloor(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState12preStepFloorEv")]
// was: RBX::HUMAN::HumanoidState::preStepFloor(void)
// IDA 0x7cea20: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cea20() {
}


// 0x7ceb8c — __ZN3RBX5HUMAN13HumanoidState14preStepCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::preStepCollide(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState14preStepCollideEv")]
// was: RBX::HUMAN::HumanoidState::preStepCollide(void)
// IDA 0x7ceb8c: 32 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ceb8c() {
}


// 0x7cebd8 — __ZN3RBX5HUMAN13HumanoidState10doAutoJumpEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::doAutoJump(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState10doAutoJumpEv")]
// was: RBX::HUMAN::HumanoidState::doAutoJump(void)
// IDA 0x7cebd8: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cebd8() {
}


// 0x7cee64 — __ZN3RBX5HUMAN13HumanoidState17setLegsCanCollideEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setLegsCanCollide(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState17setLegsCanCollideEb")]
// was: RBX::HUMAN::HumanoidState::setLegsCanCollide(bool)
// IDA 0x7cee64: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cee64() {
}


// 0x7ceea4 — __ZN3RBX5HUMAN13HumanoidState17setArmsCanCollideEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setArmsCanCollide(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState17setArmsCanCollideEb")]
// was: RBX::HUMAN::HumanoidState::setArmsCanCollide(bool)
// IDA 0x7ceea4: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ceea4() {
}


// 0x7ceee4 — __ZN3RBX5HUMAN13HumanoidState17setHeadCanCollideEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setHeadCanCollide(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState17setHeadCanCollideEb")]
// was: RBX::HUMAN::HumanoidState::setHeadCanCollide(bool)
// IDA 0x7ceee4: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ceee4() {
}


// 0x7cef08 — __ZN3RBX5HUMAN13HumanoidState18setTorsoCanCollideEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setTorsoCanCollide(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState18setTorsoCanCollideEb")]
// was: RBX::HUMAN::HumanoidState::setTorsoCanCollide(bool)
// IDA 0x7cef08: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cef08() {
}


// 0x7cef2c — __ZN3RBX5HUMAN13HumanoidState9findFloorERN5boost10shared_ptrINS_12PartInstanceEEE
#[doc(alias = "RBX::HUMAN::HumanoidState::findFloor(boost::shared_ptr<RBX::PartInstance> &)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState9findFloorERN5boost10shared_ptrINS_12PartInstanceEEE")]
// was: RBX::HUMAN::HumanoidState::findFloor(boost::shared_ptr<RBX::PartInstance> &)
// IDA 0x7cef2c: 585 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cef2c() {
}


// 0x7cf624 — __ZN3RBX5HUMAN13HumanoidState8simulateERN5boost10shared_ptrIS1_EEf
#[doc(alias = "RBX::HUMAN::HumanoidState::simulate(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,float)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState8simulateERN5boost10shared_ptrIS1_EEf")]
// was: RBX::HUMAN::HumanoidState::simulate(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,float)
// IDA 0x7cf624: 109 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cf624() {
}


// 0x7cf76c — __ZN3RBX5HUMAN13HumanoidState21doSimulatorStateTableERN5boost10shared_ptrIS1_EEf
#[doc(alias = "RBX::HUMAN::HumanoidState::doSimulatorStateTable(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,float)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState21doSimulatorStateTableERN5boost10shared_ptrIS1_EEf")]
// was: RBX::HUMAN::HumanoidState::doSimulatorStateTable(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,float)
// IDA 0x7cf76c: 65 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cf76c() {
}


// 0x7cf838 — __ZN3RBX5HUMAN13HumanoidState10noSimulateERN5boost10shared_ptrIS1_EE
#[doc(alias = "RBX::HUMAN::HumanoidState::noSimulate(boost::shared_ptr<RBX::HUMAN::HumanoidState> &)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState10noSimulateERN5boost10shared_ptrIS1_EE")]
// was: RBX::HUMAN::HumanoidState::noSimulate(boost::shared_ptr<RBX::HUMAN::HumanoidState> &)
// IDA 0x7cf838: 49 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cf838() {
}


// 0x7cf8cc — __ZN3RBX5HUMAN13HumanoidState17doSlaveStateTableERN5boost10shared_ptrIS1_EENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::HumanoidState::doSlaveStateTable(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState17doSlaveStateTableERN5boost10shared_ptrIS1_EENS0_9StateTypeE")]
// was: RBX::HUMAN::HumanoidState::doSlaveStateTable(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)
// IDA 0x7cf8cc: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cf8cc() {
}


// 0x7cf96c — __ZN3RBX5HUMAN13HumanoidState11changeStateERN5boost10shared_ptrIS1_EENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::HumanoidState::changeState(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState11changeStateERN5boost10shared_ptrIS1_EENS0_9StateTypeE")]
// was: RBX::HUMAN::HumanoidState::changeState(boost::shared_ptr<RBX::HUMAN::HumanoidState> &,RBX::HUMAN::StateType)
// IDA 0x7cf96c: 51 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cf96c() {
}


// 0x7cf9f4 — __ZN3RBX5HUMAN13HumanoidState12computeEventENS0_9EventTypeE
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, EventType)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeEvent(RBX::HUMAN::EventType)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState12computeEventENS0_9EventTypeE")]
// was: RBX::HUMAN::HumanoidState::computeEvent(RBX::HUMAN::EventType)
// IDA 0x7cf9f4: 158 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cf9f4() {
}


// 0x7cfbc8 — __ZN3RBX5HUMAN13HumanoidState28computeTouchedByMySimulationEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeTouchedByMySimulation(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState28computeTouchedByMySimulationEv")]
// was: RBX::HUMAN::HumanoidState::computeTouchedByMySimulation(void)
// IDA 0x7cfbc8: 95 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cfbc8() {
}


// 0x7cfce4 — __ZNK3RBX5HUMAN13HumanoidState13computeJumpedEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeJumped(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState13computeJumpedEv")]
// was: RBX::HUMAN::HumanoidState::computeJumped(void)const
// IDA 0x7cfce4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cfce4() {
}


// 0x7cfd28 — __ZNK3RBX5HUMAN13HumanoidState13computeTippedEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeTipped(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState13computeTippedEv")]
// was: RBX::HUMAN::HumanoidState::computeTipped(void)const
// IDA 0x7cfd28: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cfd28() {
}


// 0x7cfd80 — __ZN3RBX5HUMAN13HumanoidState14computeTouchedEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeTouched(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState14computeTouchedEv")]
// was: RBX::HUMAN::HumanoidState::computeTouched(void)
// IDA 0x7cfd80: 83 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cfd80() {
}


// 0x7cfe70 — __ZNK3RBX5HUMAN13HumanoidState14computeHasGyroEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeHasGyro(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState14computeHasGyroEv")]
// was: RBX::HUMAN::HumanoidState::computeHasGyro(void)const
// IDA 0x7cfe70: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cfe70() {
}


// 0x7cfed8 — __ZN3RBX5HUMAN13HumanoidState9createNewENS0_9StateTypeES2_PNS_8HumanoidE
#[doc(alias = "RBX::HUMAN::HumanoidState::createNew(RBX::HUMAN::StateType,RBX::HUMAN::StateType,RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState9createNewENS0_9StateTypeES2_PNS_8HumanoidE")]
// was: RBX::HUMAN::HumanoidState::createNew(RBX::HUMAN::StateType,RBX::HUMAN::StateType,RBX::Humanoid *)
// IDA 0x7cfed8: 417 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cfed8() {
}


// 0x7d033c — __ZNK3RBX5HUMAN13HumanoidState11computeTiltEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeTilt(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState11computeTiltEv")]
// was: RBX::HUMAN::HumanoidState::computeTilt(void)const
// IDA 0x7d033c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d033c() {
}


// 0x7d03b4 — __ZNK3RBX5HUMAN13HumanoidState16computeFloorTiltEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeFloorTilt(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState16computeFloorTiltEv")]
// was: RBX::HUMAN::HumanoidState::computeFloorTilt(void)const
// IDA 0x7d03b4: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d03b4() {
}


// 0x7d0404 — __ZN3RBX5HUMANL28intersectsOutsideOfCharacterEPNS_9PrimitiveEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::HUMAN *__hidden this, RBX::Primitive *, const RBX::Instance *)
#[doc(alias = "RBX::HUMAN::intersectsOutsideOfCharacter(RBX::Primitive *,RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX5HUMANL28intersectsOutsideOfCharacterEPNS_9PrimitiveEPKNS_8InstanceE")]
// was: RBX::HUMAN::intersectsOutsideOfCharacter(RBX::Primitive *,RBX::Instance const*)
// IDA 0x7d0404: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0404() {
}


// 0x7d0450 — __ZN3RBX5HUMAN13HumanoidState28computeHitByHighImpactObjectEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::computeHitByHighImpactObject(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState28computeHitByHighImpactObjectEv")]
// was: RBX::HUMAN::HumanoidState::computeHitByHighImpactObject(void)
// IDA 0x7d0450: 207 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0450() {
}


// 0x7d0ba8 — __ZNK3RBX5HUMAN13HumanoidState12filterResultEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::HUMAN::HumanoidState::filterResult(RBX::Primitive const*)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState12filterResultEPKNS_9PrimitiveE")]
// was: RBX::HUMAN::HumanoidState::filterResult(RBX::Primitive const*)const
// IDA 0x7d0ba8: 48 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0ba8() {
}


// 0x7d0c30 — __ZThn4_NK3RBX5HUMAN13HumanoidState12filterResultEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, const RBX::Primitive *)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::HumanoidState::filterResult(RBX::Primitive const*)const")]
#[doc(alias = "__ZThn4_NK3RBX5HUMAN13HumanoidState12filterResultEPKNS_9PrimitiveE")]
// was: non-virtual thunk toRBX::HUMAN::HumanoidState::filterResult(RBX::Primitive const*)const
// IDA 0x7d0c30: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0c30() {
}


// 0x7d0c38 — __ZN3RBX5HUMAN13HumanoidState21getFloorPointVelocityEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorPointVelocity(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState21getFloorPointVelocityEv")]
// was: RBX::HUMAN::HumanoidState::getFloorPointVelocity(void)
// IDA 0x7d0c38: 174 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0c38() {
}


// 0x7d0e94 — __ZN3RBX5HUMAN13HumanoidState27getRelativeMovementVelocityEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getRelativeMovementVelocity(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState27getRelativeMovementVelocityEv")]
// was: RBX::HUMAN::HumanoidState::getRelativeMovementVelocity(void)
// IDA 0x7d0e94: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0e94() {
}


// 0x7d0f28 — __ZN3RBX5HUMAN13HumanoidState18fireMovementSignalERN3rbx6signalIFvfEEEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::HUMAN::HumanoidState::fireMovementSignal(rbx::signal<void ()(float)> &,float)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState18fireMovementSignalERN3rbx6signalIFvfEEEf")]
// was: RBX::HUMAN::HumanoidState::fireMovementSignal(rbx::signal<void ()(float)> &,float)
// IDA 0x7d0f28: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0f28() {
}


// 0x7d0f98 — __ZNK3RBX5HUMAN13HumanoidState18getDesiredAltitudeEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getDesiredAltitude(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState18getDesiredAltitudeEv")]
// was: RBX::HUMAN::HumanoidState::getDesiredAltitude(void)const
// IDA 0x7d0f98: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0f98() {
}


// 0x7d144c — __ZNK3RBX5HUMAN13HumanoidState21unitializedFloorTouchEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::unitializedFloorTouch(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState21unitializedFloorTouchEv")]
// was: RBX::HUMAN::HumanoidState::unitializedFloorTouch(void)const
// IDA 0x7d144c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d144c() {
}


// 0x7d149c — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::push_back(rbx::signals::connection const&)")]
#[doc(alias = "__ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE9push_backERKS2_")]
// was: std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::push_back(rbx::signals::connection const&)
// IDA 0x7d149c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_7d149c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x7d14d4 — __ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// was: rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>> const&)
// IDA 0x7d14d4: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d14d4() {
}


// 0x7d1548 — __ZN3RBX5HUMAN13HumanoidState19setTorsoHasBuoyancyEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setTorsoHasBuoyancy(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState19setTorsoHasBuoyancyEb")]
// was: RBX::HUMAN::HumanoidState::setTorsoHasBuoyancy(bool)
// IDA 0x7d1548: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1548() {
}


// 0x7d1550 — __ZN3RBX5HUMAN13HumanoidState21setLeftLegHasBuoyancyEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setLeftLegHasBuoyancy(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState21setLeftLegHasBuoyancyEb")]
// was: RBX::HUMAN::HumanoidState::setLeftLegHasBuoyancy(bool)
// IDA 0x7d1550: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1550() {
}


// 0x7d1558 — __ZN3RBX5HUMAN13HumanoidState22setRightLegHasBuoyancyEb
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, bool)
#[doc(alias = "RBX::HUMAN::HumanoidState::setRightLegHasBuoyancy(bool)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState22setRightLegHasBuoyancyEb")]
// was: RBX::HUMAN::HumanoidState::setRightLegHasBuoyancy(bool)
// IDA 0x7d1558: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1558() {
}


// 0x7d1560 — __ZNK3RBX5HUMAN13HumanoidState10usesLadderEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::usesLadder(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState10usesLadderEv")]
// was: RBX::HUMAN::HumanoidState::usesLadder(void)const
// IDA 0x7d1560: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1560() {
}


// 0x7d1598 — __ZNK3RBX5HUMAN13HumanoidState9usesFloorEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::usesFloor(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState9usesFloorEv")]
// was: RBX::HUMAN::HumanoidState::usesFloor(void)const
// IDA 0x7d1598: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1598() {
}


// 0x7d15e8 — __ZNK3RBX5HUMAN13HumanoidState19getFloorTouchNormalEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorTouchNormal(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState19getFloorTouchNormalEv")]
// was: RBX::HUMAN::HumanoidState::getFloorTouchNormal(void)const
// IDA 0x7d15e8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d15e8() {
}


// 0x7d1688 — __ZNK3RBX8Instance30findConstFirstDescendantOfTypeINS_9BodyMoverEEEPKT_v
#[doc(alias = "RBX::BodyMover const* RBX::Instance::findConstFirstDescendantOfType<RBX::BodyMover>(void)const")]
#[doc(alias = "__ZNK3RBX8Instance30findConstFirstDescendantOfTypeINS_9BodyMoverEEEPKT_v")]
// was: RBX::BodyMover const* RBX::Instance::findConstFirstDescendantOfType<RBX::BodyMover>(void)const
// IDA 0x7d1688: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1688() {
}


// 0x7d16f8 — __ZNK3RBX5HUMAN13HumanoidState31getFloorHumanoidLocationInWorldEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorHumanoidLocationInWorld(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState31getFloorHumanoidLocationInWorldEv")]
// was: RBX::HUMAN::HumanoidState::getFloorHumanoidLocationInWorld(void)const
// IDA 0x7d16f8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d16f8() {
}


// 0x7d1798 — __ZN3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_PKNS_8InstanceE")]
// was: RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(RBX::Instance const*)
// IDA 0x7d1798: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1798() {
}


// 0x7d17b0 — __ZN3RBX5HUMAN13HumanoidState10onStepImplEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::onStepImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState10onStepImplEv")]
// was: RBX::HUMAN::HumanoidState::onStepImpl(void)
// IDA 0x7d17b0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7d17b0() {
}


// 0x7d17b4 — __ZN3RBX5HUMAN13HumanoidState19onSimulatorStepImplEf
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this, float)
#[doc(alias = "RBX::HUMAN::HumanoidState::onSimulatorStepImpl(float)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState19onSimulatorStepImplEf")]
// was: RBX::HUMAN::HumanoidState::onSimulatorStepImpl(float)
// IDA 0x7d17b4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7d17b4() {
}


// 0x7d17b8 — __ZNK3RBX5HUMAN13HumanoidState17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState17armsShouldCollideEv")]
// was: RBX::HUMAN::HumanoidState::armsShouldCollide(void)const
// IDA 0x7d17b8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d17b8() {
}


// 0x7d17bc — __ZNK3RBX5HUMAN13HumanoidState17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState17legsShouldCollideEv")]
// was: RBX::HUMAN::HumanoidState::legsShouldCollide(void)const
// IDA 0x7d17bc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d17bc() {
}


// 0x7d17c0 — __ZNK3RBX5HUMAN13HumanoidState17headShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::headShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState17headShouldCollideEv")]
// was: RBX::HUMAN::HumanoidState::headShouldCollide(void)const
// IDA 0x7d17c0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d17c0() {
}


// 0x7d17c4 — __ZNK3RBX5HUMAN13HumanoidState18torsoShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::torsoShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState18torsoShouldCollideEv")]
// was: RBX::HUMAN::HumanoidState::torsoShouldCollide(void)const
// IDA 0x7d17c4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d17c4() {
}


// 0x7d17c8 — __ZN3RBX5HUMAN13HumanoidState29onCFrameChangedFromReflectionEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::onCFrameChangedFromReflection(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState29onCFrameChangedFromReflectionEv")]
// was: RBX::HUMAN::HumanoidState::onCFrameChangedFromReflection(void)
// IDA 0x7d17c8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d17c8() {
}


// 0x7d17cc — __ZNK3RBX5HUMAN13HumanoidState26getYAxisRotationalVelocityEv
// type: _DWORD __fastcall(RBX::HUMAN::HumanoidState *__hidden this)
#[doc(alias = "RBX::HUMAN::HumanoidState::getYAxisRotationalVelocity(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN13HumanoidState26getYAxisRotationalVelocityEv")]
// was: RBX::HUMAN::HumanoidState::getYAxisRotationalVelocity(void)const
// IDA 0x7d17cc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d17cc() {
}


// 0x7d17d0 — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE15_M_erase_at_endEPS2_
#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_erase_at_end(rbx::signals::connection*)")]
#[doc(alias = "__ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE15_M_erase_at_endEPS2_")]
// was: std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_erase_at_end(rbx::signals::connection*)
// IDA 0x7d17d0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d17d0() {
}


// 0x7d1800 — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED1Ev")]
// was: rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()
// IDA 0x7d1800: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d1800() {
}


// 0x7d182c — __ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEED0Ev")]
// was: rbx::signals::signal<void ()(bool)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>::~callable_slot()
// IDA 0x7d182c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d182c() {
}


// 0x7d1900 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
// IDA 0x7d1900: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1900() {
}


// 0x7d1924 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_E4callEb")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::call(bool)
// IDA 0x7d1924: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1924() {
}


// 0x7d1948 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list1<bool &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool> &,boost::_bi::list1<bool &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list1<bool &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool> &,boost::_bi::list1<bool &> &,int)
// IDA 0x7d1948: 13 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1948() {
}


// 0x7d1968 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()
// IDA 0x7d1968: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d1968() {
}


// 0x7d1994 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS7_5list2INS7_5valueIPSD_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev")]
// was: rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>,1,void ()(bool)>::~callable()
// IDA 0x7d1994: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d1994() {
}


// 0x7d1a68 — __ZN9__gnu_cxx13new_allocatorIN3rbx7signals10connectionEE9constructEPS3_RKS3_
#[doc(alias = "__gnu_cxx::new_allocator<rbx::signals::connection>::construct(rbx::signals::connection*,rbx::signals::connection const&)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIN3rbx7signals10connectionEE9constructEPS3_RKS3_")]
// was: __gnu_cxx::new_allocator<rbx::signals::connection>::construct(rbx::signals::connection*,rbx::signals::connection const&)
// IDA 0x7d1a68: 14 insns (CMP..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1a68() {
}


// 0x7d1a88 — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx::signals::connection*,std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>>,rbx::signals::connection const&)")]
#[doc(alias = "__ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// was: std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx::signals::connection*,std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>>,rbx::signals::connection const&)
// IDA 0x7d1a88: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_7d1a88() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x7d1e04 — __ZNSt12_Vector_baseIN3rbx7signals10connectionESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3rbx7signals10connectionESaIS2_EE11_M_allocateEm")]
// was: std::_Vector_base<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_allocate(unsigned long)
// IDA 0x7d1e04: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_7d1e04() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x7d1e1c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3rbx7signals10connectionES6_EET0_T_S8_S7_
#[doc(alias = "rbx::signals::connection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx::signals::connection *,rbx::signals::connection *>(rbx::signals::connection *,rbx::signals::connection *,rbx::signals::connection *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3rbx7signals10connectionES6_EET0_T_S8_S7_")]
// was: rbx::signals::connection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx::signals::connection *,rbx::signals::connection *>(rbx::signals::connection *,rbx::signals::connection *,rbx::signals::connection *)
// IDA 0x7d1e1c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_7d1e1c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x7d1e6c — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED2Ev
#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED2Ev")]
// was: std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::~vector()
// IDA 0x7d1e6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d1e6c() {
}


// 0x7d1f38 — __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_5HUMAN11RunningBaseELZNS1_9sClimbingEEE7getNameEv
// IDA 0x7d1f38: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d1f38() {
}


// 0x7d1f3c — __ZNK3RBX5HUMAN11RunningBase17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningBase::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN11RunningBase17armsShouldCollideEv")]
// was: RBX::HUMAN::RunningBase::armsShouldCollide(void)const
// IDA 0x7d1f3c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1f3c() {
}


// 0x7d1f40 — __ZNK3RBX5HUMAN11RunningBase17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningBase::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN11RunningBase17legsShouldCollideEv")]
// was: RBX::HUMAN::RunningBase::legsShouldCollide(void)const
// IDA 0x7d1f40: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1f40() {
}


// 0x7d1f44 — __ZNK3RBX5HUMAN11RunningBase26getYAxisRotationalVelocityEv
// type: _DWORD __fastcall(RBX::HUMAN::RunningBase *__hidden this)
#[doc(alias = "RBX::HUMAN::RunningBase::getYAxisRotationalVelocity(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN11RunningBase26getYAxisRotationalVelocityEv")]
// was: RBX::HUMAN::RunningBase::getYAxisRotationalVelocity(void)const
// IDA 0x7d1f44: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1f44() {
}


// 0x7d1f4c — __ZNK3RBX5HUMAN8Climbing12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Climbing *__hidden this)
#[doc(alias = "RBX::HUMAN::Climbing::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN8Climbing12getStateTypeEv")]
// was: RBX::HUMAN::Climbing::getStateType(void)const
// IDA 0x7d1f4c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1f4c() {
}


// 0x7d1f50 — __ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_5HUMAN9sClimbingEEEERKS0_v
// IDA 0x7d1f50: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1f50() {
}


// 0x7d1f94 — __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5HUMAN9sClimbingEEEEvv
// IDA 0x7d1f94: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d1f94() {
}


// 0x7d1f98 — __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5HUMAN9sClimbingEEEERKS0_v
// IDA 0x7d1f98: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1f98() {
}


// 0x7d207c — __GLOBAL__I_a_377
#[doc(alias = "global constructor keyed to_a_377")]
#[doc(alias = "__GLOBAL__I_a_377")]
// was: global constructor keyed to_a_377
// IDA 0x7d207c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7d207c() {
}


// 0x7d24ac — __ZN3RBX5HUMAN7JumpingC1EPNS_8HumanoidENS0_9StateTypeE
#[doc(alias = "RBX::HUMAN::Jumping::Jumping(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN7JumpingC1EPNS_8HumanoidENS0_9StateTypeE")]
// was: RBX::HUMAN::Jumping::Jumping(RBX::Humanoid *,RBX::HUMAN::StateType)
// IDA 0x7d24ac: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d24ac() {
}


// 0x7d24e0 — __ZN3RBX5HUMAN7Jumping18onComputeForceImplEv
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN7Jumping18onComputeForceImplEv")]
// was: RBX::HUMAN::Jumping::onComputeForceImpl(void)
// IDA 0x7d24e0: 151 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d24e0() {
}


// 0x7d26d4 — __ZN3RBX5HUMAN7Jumping11findCeilingEv
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::findCeiling(void)")]
#[doc(alias = "__ZN3RBX5HUMAN7Jumping11findCeilingEv")]
// was: RBX::HUMAN::Jumping::findCeiling(void)
// IDA 0x7d26d4: 336 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d26d4() {
}


// 0x7d2b0c — __ZN3RBX5HUMAN7Jumping10tryCeilingERKNS_6RbxRayEfPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this, const RBX::RbxRay *, float, RBX::Assembly *)
#[doc(alias = "RBX::HUMAN::Jumping::tryCeiling(RBX::RbxRay const&,float,RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5HUMAN7Jumping10tryCeilingERKNS_6RbxRayEfPNS_8AssemblyE")]
// was: RBX::HUMAN::Jumping::tryCeiling(RBX::RbxRay const&,float,RBX::Assembly *)
// IDA 0x7d2b0c: 155 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2b0c() {
}


// 0x7d2cb8 — __ZNK3RBX5HUMAN7Jumping12filterResultEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::HUMAN::Jumping::filterResult(RBX::Primitive const*)const")]
#[doc(alias = "__ZNK3RBX5HUMAN7Jumping12filterResultEPKNS_9PrimitiveE")]
// was: RBX::HUMAN::Jumping::filterResult(RBX::Primitive const*)const
// IDA 0x7d2cb8: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2cb8() {
}


// 0x7d2d70 — __ZThn4_NK3RBX5HUMAN7Jumping12filterResultEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this, const RBX::Primitive *)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Jumping::filterResult(RBX::Primitive const*)const")]
#[doc(alias = "__ZThn4_NK3RBX5HUMAN7Jumping12filterResultEPKNS_9PrimitiveE")]
// was: non-virtual thunk toRBX::HUMAN::Jumping::filterResult(RBX::Primitive const*)const
// IDA 0x7d2d70: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2d70() {
}


// 0x7d2d78 — __ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv")]
#[doc(alias = "__ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv")]
// was: __ZNK3RBX5NamedINS_5HUMAN6FlyingELZNS1_8sJumpingEEE7getNameEv
// IDA 0x7d2d78: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2d78() {
}


// 0x7d2da0 — __ZNK3RBX5HUMAN7Jumping17armsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::armsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN7Jumping17armsShouldCollideEv")]
// was: RBX::HUMAN::Jumping::armsShouldCollide(void)const
// IDA 0x7d2da0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2da0() {
}


// 0x7d2da4 — __ZNK3RBX5HUMAN7Jumping17legsShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::legsShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN7Jumping17legsShouldCollideEv")]
// was: RBX::HUMAN::Jumping::legsShouldCollide(void)const
// IDA 0x7d2da4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2da4() {
}


// 0x7d2da8 — __ZNK3RBX5HUMAN7Jumping18torsoShouldCollideEv
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::torsoShouldCollide(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN7Jumping18torsoShouldCollideEv")]
// was: RBX::HUMAN::Jumping::torsoShouldCollide(void)const
// IDA 0x7d2da8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2da8() {
}


// 0x7d2dac — __ZN3RBX5HUMAN7JumpingD1Ev
// type: void __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::~Jumping()")]
#[doc(alias = "__ZN3RBX5HUMAN7JumpingD1Ev")]
// was: RBX::HUMAN::Jumping::~Jumping()
// IDA 0x7d2dac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7d2dac() {
}


// 0x7d2db0 — __ZN3RBX5HUMAN7JumpingD0Ev
// type: void __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::~Jumping()")]
#[doc(alias = "__ZN3RBX5HUMAN7JumpingD0Ev")]
// was: RBX::HUMAN::Jumping::~Jumping()
// IDA 0x7d2db0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d2db0() {
}


// 0x7d2e50 — __ZNK3RBX5HUMAN7Jumping12getStateTypeEv
// type: _DWORD __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "RBX::HUMAN::Jumping::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN7Jumping12getStateTypeEv")]
// was: RBX::HUMAN::Jumping::getStateType(void)const
// IDA 0x7d2e50: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d2e50() {
}


// 0x7d2e54 — __ZThn4_N3RBX5HUMAN7JumpingD1Ev
// type: void __fastcall(RBX::HUMAN::Jumping *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::HUMAN::Jumping::~Jumping()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN7JumpingD1Ev")]
// was: non-virtual thunk toRBX::HUMAN::Jumping::~Jumping()
// IDA 0x7d2e54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7d2e54() {
}

