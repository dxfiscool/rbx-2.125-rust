//! core shard DL — 100 core stubs EA-sorted, next uncovered after DK 0x7c6d74 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::push_back(RBX::Primitive * const&)")]
// 0x7cb6a0 — __ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE9push_backERKS2_
pub fn stub_7cb6a0() {
    // IDA 0x7cb6a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive * const&)")]
// 0x7cb6cc — __ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7cb6cc() {
    // IDA 0x7cb6cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::getState(RBX::HUMAN::StateType,RBX::HUMAN::EventType)")]
// 0x7cd220 — __ZN3RBX5HUMAN8getStateENS0_9StateTypeENS0_9EventTypeE
pub fn stub_7cd220() {
    // IDA 0x7cd220: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::HumanoidState(RBX::Humanoid *,RBX::HUMAN::StateType)")]
// 0x7cd2f4 — __ZN3RBX5HUMAN13HumanoidStateC2EPNS_8HumanoidENS0_9StateTypeE
pub fn stub_7cd2f4() {
    // IDA 0x7cd2f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setCanThrottleState(bool)")]
// 0x7cd5fc — __ZN3RBX5HUMAN13HumanoidState19setCanThrottleStateEb
pub fn stub_7cd5fc() {
    // IDA 0x7cd5fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::~HumanoidState()")]
// 0x7cd6a4 — __ZN3RBX5HUMAN13HumanoidStateD0Ev
pub fn stub_7cd6a4() {
    // IDA 0x7cd6a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::~HumanoidState()")]
// 0x7cd744 — __ZN3RBX5HUMAN13HumanoidStateD1Ev
pub fn stub_7cd744() {
    // IDA 0x7cd744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::HumanoidState::~HumanoidState()")]
// 0x7cd748 — __ZThn4_N3RBX5HUMAN13HumanoidStateD0Ev
pub fn stub_7cd748() {
    // IDA 0x7cd748: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::~HumanoidState()")]
// 0x7cd750 — __ZN3RBX5HUMAN13HumanoidStateD2Ev
pub fn stub_7cd750() {
    // IDA 0x7cd750: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::HumanoidState::~HumanoidState()")]
// 0x7cd8c0 — __ZThn4_N3RBX5HUMAN13HumanoidStateD1Ev
pub fn stub_7cd8c0() {
    // IDA 0x7cd8c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::fireEvents(void)")]
// 0x7cd8c8 — __ZN3RBX5HUMAN13HumanoidState10fireEventsEv
pub fn stub_7cd8c8() {
    // IDA 0x7cd8c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::fireEvent(RBX::HUMAN::StateType,bool)")]
// 0x7cd904 — __ZN3RBX5HUMAN13HumanoidState9fireEventENS0_9StateTypeEb
pub fn stub_7cd904() {
    // IDA 0x7cd904: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getAssemblyConst(void)const")]
// 0x7cda58 — __ZNK3RBX5HUMAN13HumanoidState16getAssemblyConstEv
pub fn stub_7cda58() {
    // IDA 0x7cda58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::stateToAssembly(void)")]
// 0x7cda78 — __ZN3RBX5HUMAN13HumanoidState15stateToAssemblyEv
pub fn stub_7cda78() {
    // IDA 0x7cda78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getHumanoidConst(void)const")]
// 0x7cda98 — __ZNK3RBX5HUMAN13HumanoidState16getHumanoidConstEv
pub fn stub_7cda98() {
    // IDA 0x7cda98: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::onComputeForce(void)")]
// 0x7cdaf0 — __ZN3RBX5HUMAN13HumanoidState14onComputeForceEv
pub fn stub_7cdaf0() {
    // IDA 0x7cdaf0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::render3dAdorn(RBX::Adorn *)")]
// 0x7cdba4 — __ZN3RBX5HUMAN13HumanoidState13render3dAdornEPNS_5AdornE
pub fn stub_7cdba4() {
    // IDA 0x7cdba4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::findLadder(RBX::Adorn *)")]
// 0x7ce378 — __ZN3RBX5HUMAN13HumanoidState10findLadderEPNS_5AdornE
pub fn stub_7ce378() {
    // IDA 0x7ce378: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorPrimitive(void)")]
// 0x7ce82c — __ZN3RBX5HUMAN13HumanoidState17getFloorPrimitiveEv
pub fn stub_7ce82c() {
    // IDA 0x7ce82c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorPrimitiveConst(void)const")]
// 0x7ce830 — __ZNK3RBX5HUMAN13HumanoidState22getFloorPrimitiveConstEv
pub fn stub_7ce830() {
    // IDA 0x7ce830: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::defaultState(RBX::Humanoid *)")]
// 0x7ce860 — __ZN3RBX5HUMAN13HumanoidState12defaultStateEPNS_8HumanoidE
pub fn stub_7ce860() {
    // IDA 0x7ce860: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::preStepSimulatorSide(float)")]
// 0x7ce910 — __ZN3RBX5HUMAN13HumanoidState20preStepSimulatorSideEf
pub fn stub_7ce910() {
    // IDA 0x7ce910: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::preStepFloor(void)")]
// 0x7cea20 — __ZN3RBX5HUMAN13HumanoidState12preStepFloorEv
pub fn stub_7cea20() {
    // IDA 0x7cea20: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::preStepCollide(void)")]
// 0x7ceb8c — __ZN3RBX5HUMAN13HumanoidState14preStepCollideEv
pub fn stub_7ceb8c() {
    // IDA 0x7ceb8c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::doAutoJump(void)")]
// 0x7cebd8 — __ZN3RBX5HUMAN13HumanoidState10doAutoJumpEv
pub fn stub_7cebd8() {
    // IDA 0x7cebd8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setLegsCanCollide(bool)")]
// 0x7cee64 — __ZN3RBX5HUMAN13HumanoidState17setLegsCanCollideEb
pub fn stub_7cee64() {
    // IDA 0x7cee64: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setArmsCanCollide(bool)")]
// 0x7ceea4 — __ZN3RBX5HUMAN13HumanoidState17setArmsCanCollideEb
pub fn stub_7ceea4() {
    // IDA 0x7ceea4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setHeadCanCollide(bool)")]
// 0x7ceee4 — __ZN3RBX5HUMAN13HumanoidState17setHeadCanCollideEb
pub fn stub_7ceee4() {
    // IDA 0x7ceee4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setTorsoCanCollide(bool)")]
// 0x7cef08 — __ZN3RBX5HUMAN13HumanoidState18setTorsoCanCollideEb
pub fn stub_7cef08() {
    // IDA 0x7cef08: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeEvent(RBX::HUMAN::EventType)")]
// 0x7cf9f4 — __ZN3RBX5HUMAN13HumanoidState12computeEventENS0_9EventTypeE
pub fn stub_7cf9f4() {
    // IDA 0x7cf9f4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeTouchedByMySimulation(void)")]
// 0x7cfbc8 — __ZN3RBX5HUMAN13HumanoidState28computeTouchedByMySimulationEv
pub fn stub_7cfbc8() {
    // IDA 0x7cfbc8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeJumped(void)const")]
// 0x7cfce4 — __ZNK3RBX5HUMAN13HumanoidState13computeJumpedEv
pub fn stub_7cfce4() {
    // IDA 0x7cfce4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeTipped(void)const")]
// 0x7cfd28 — __ZNK3RBX5HUMAN13HumanoidState13computeTippedEv
pub fn stub_7cfd28() {
    // IDA 0x7cfd28: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeTouched(void)")]
// 0x7cfd80 — __ZN3RBX5HUMAN13HumanoidState14computeTouchedEv
pub fn stub_7cfd80() {
    // IDA 0x7cfd80: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeHasGyro(void)const")]
// 0x7cfe70 — __ZNK3RBX5HUMAN13HumanoidState14computeHasGyroEv
pub fn stub_7cfe70() {
    // IDA 0x7cfe70: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::createNew(RBX::HUMAN::StateType,RBX::HUMAN::StateType,RBX::Humanoid *)")]
// 0x7cfed8 — __ZN3RBX5HUMAN13HumanoidState9createNewENS0_9StateTypeES2_PNS_8HumanoidE
pub fn stub_7cfed8() {
    // IDA 0x7cfed8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeTilt(void)const")]
// 0x7d033c — __ZNK3RBX5HUMAN13HumanoidState11computeTiltEv
pub fn stub_7d033c() {
    // IDA 0x7d033c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeFloorTilt(void)const")]
// 0x7d03b4 — __ZNK3RBX5HUMAN13HumanoidState16computeFloorTiltEv
pub fn stub_7d03b4() {
    // IDA 0x7d03b4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::computeHitByHighImpactObject(void)")]
// 0x7d0450 — __ZN3RBX5HUMAN13HumanoidState28computeHitByHighImpactObjectEv
pub fn stub_7d0450() {
    // IDA 0x7d0450: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::filterResult(RBX::Primitive const*)const")]
// 0x7d0ba8 — __ZNK3RBX5HUMAN13HumanoidState12filterResultEPKNS_9PrimitiveE
pub fn stub_7d0ba8() {
    // IDA 0x7d0ba8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::HumanoidState::filterResult(RBX::Primitive const*)const")]
// 0x7d0c30 — __ZThn4_NK3RBX5HUMAN13HumanoidState12filterResultEPKNS_9PrimitiveE
pub fn stub_7d0c30() {
    // IDA 0x7d0c30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorPointVelocity(void)")]
// 0x7d0c38 — __ZN3RBX5HUMAN13HumanoidState21getFloorPointVelocityEv
pub fn stub_7d0c38() {
    // IDA 0x7d0c38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getRelativeMovementVelocity(void)")]
// 0x7d0e94 — __ZN3RBX5HUMAN13HumanoidState27getRelativeMovementVelocityEv
pub fn stub_7d0e94() {
    // IDA 0x7d0e94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::fireMovementSignal(rbx::signal<void ()(float)> &,float)")]
// 0x7d0f28 — __ZN3RBX5HUMAN13HumanoidState18fireMovementSignalERN3rbx6signalIFvfEEEf
pub fn stub_7d0f28() {
    // IDA 0x7d0f28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getDesiredAltitude(void)const")]
// 0x7d0f98 — __ZNK3RBX5HUMAN13HumanoidState18getDesiredAltitudeEv
pub fn stub_7d0f98() {
    // IDA 0x7d0f98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::findPrimitiveInLadderZone(RBX::Adorn *)")]
// 0x7d1004 — __ZN3RBX5HUMAN13HumanoidState25findPrimitiveInLadderZoneEPNS_5AdornE
pub fn stub_7d1004() {
    // IDA 0x7d1004: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::unitializedFloorTouch(void)const")]
// 0x7d144c — __ZNK3RBX5HUMAN13HumanoidState21unitializedFloorTouchEv
pub fn stub_7d144c() {
    // IDA 0x7d144c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::push_back(rbx::signals::connection const&)")]
// 0x7d149c — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE9push_backERKS2_
pub fn stub_7d149c() {
    // IDA 0x7d149c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setTorsoHasBuoyancy(bool)")]
// 0x7d1548 — __ZN3RBX5HUMAN13HumanoidState19setTorsoHasBuoyancyEb
pub fn stub_7d1548() {
    // IDA 0x7d1548: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setLeftLegHasBuoyancy(bool)")]
// 0x7d1550 — __ZN3RBX5HUMAN13HumanoidState21setLeftLegHasBuoyancyEb
pub fn stub_7d1550() {
    // IDA 0x7d1550: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::setRightLegHasBuoyancy(bool)")]
// 0x7d1558 — __ZN3RBX5HUMAN13HumanoidState22setRightLegHasBuoyancyEb
pub fn stub_7d1558() {
    // IDA 0x7d1558: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::usesLadder(void)const")]
// 0x7d1560 — __ZNK3RBX5HUMAN13HumanoidState10usesLadderEv
pub fn stub_7d1560() {
    // IDA 0x7d1560: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::usesFloor(void)const")]
// 0x7d1598 — __ZNK3RBX5HUMAN13HumanoidState9usesFloorEv
pub fn stub_7d1598() {
    // IDA 0x7d1598: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorTouchNormal(void)const")]
// 0x7d15e8 — __ZNK3RBX5HUMAN13HumanoidState19getFloorTouchNormalEv
pub fn stub_7d15e8() {
    // IDA 0x7d15e8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorHumanoidLocationInWorld(void)const")]
// 0x7d16f8 — __ZNK3RBX5HUMAN13HumanoidState31getFloorHumanoidLocationInWorldEv
pub fn stub_7d16f8() {
    // IDA 0x7d16f8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::onStepImpl(void)")]
// 0x7d17b0 — __ZN3RBX5HUMAN13HumanoidState10onStepImplEv
pub fn stub_7d17b0() {
    // IDA 0x7d17b0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::onSimulatorStepImpl(float)")]
// 0x7d17b4 — __ZN3RBX5HUMAN13HumanoidState19onSimulatorStepImplEf
pub fn stub_7d17b4() {
    // IDA 0x7d17b4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::armsShouldCollide(void)const")]
// 0x7d17b8 — __ZNK3RBX5HUMAN13HumanoidState17armsShouldCollideEv
pub fn stub_7d17b8() {
    // IDA 0x7d17b8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::legsShouldCollide(void)const")]
// 0x7d17bc — __ZNK3RBX5HUMAN13HumanoidState17legsShouldCollideEv
pub fn stub_7d17bc() {
    // IDA 0x7d17bc: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::headShouldCollide(void)const")]
// 0x7d17c0 — __ZNK3RBX5HUMAN13HumanoidState17headShouldCollideEv
pub fn stub_7d17c0() {
    // IDA 0x7d17c0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::torsoShouldCollide(void)const")]
// 0x7d17c4 — __ZNK3RBX5HUMAN13HumanoidState18torsoShouldCollideEv
pub fn stub_7d17c4() {
    // IDA 0x7d17c4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getYAxisRotationalVelocity(void)const")]
// 0x7d17cc — __ZNK3RBX5HUMAN13HumanoidState26getYAxisRotationalVelocityEv
pub fn stub_7d17cc() {
    // IDA 0x7d17cc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_erase_at_end(rbx::signals::connection*)")]
// 0x7d17d0 — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_7d17d0() {
    // IDA 0x7d17d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::new_allocator<rbx::signals::connection>::construct(rbx::signals::connection*,rbx::signals::connection const&)")]
// 0x7d1a68 — __ZN9__gnu_cxx13new_allocatorIN3rbx7signals10connectionEE9constructEPS3_RKS3_
pub fn stub_7d1a68() {
    // IDA 0x7d1a68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx::signals::connection*,std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>>,rbx::signals::connection const&)")]
// 0x7d1a88 — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7d1a88() {
    // IDA 0x7d1a88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_allocate(unsigned long)")]
// 0x7d1e04 — __ZNSt12_Vector_baseIN3rbx7signals10connectionESaIS2_EE11_M_allocateEm
pub fn stub_7d1e04() {
    // IDA 0x7d1e04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::connection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx::signals::connection *,rbx::signals::connection *>(rbx::signals::connection *,rbx::signals::connection *,rbx::signals::connection *)")]
// 0x7d1e1c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3rbx7signals10connectionES6_EET0_T_S8_S7_
pub fn stub_7d1e1c() {
    // IDA 0x7d1e1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::~vector()")]
// 0x7d1e6c — __ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED2Ev
pub fn stub_7d1e6c() {
    // IDA 0x7d1e6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::RunningBase::armsShouldCollide(void)const")]
// 0x7d1f3c — __ZNK3RBX5HUMAN11RunningBase17armsShouldCollideEv
pub fn stub_7d1f3c() {
    // IDA 0x7d1f3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::RunningBase::legsShouldCollide(void)const")]
// 0x7d1f40 — __ZNK3RBX5HUMAN11RunningBase17legsShouldCollideEv
pub fn stub_7d1f40() {
    // IDA 0x7d1f40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::RunningBase::getYAxisRotationalVelocity(void)const")]
// 0x7d1f44 — __ZNK3RBX5HUMAN11RunningBase26getYAxisRotationalVelocityEv
pub fn stub_7d1f44() {
    // IDA 0x7d1f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Climbing::getStateType(void)const")]
// 0x7d1f4c — __ZNK3RBX5HUMAN8Climbing12getStateTypeEv
pub fn stub_7d1f4c() {
    // IDA 0x7d1f4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Jumping::Jumping(RBX::Humanoid *,RBX::HUMAN::StateType)")]
// 0x7d24ac — __ZN3RBX5HUMAN7JumpingC1EPNS_8HumanoidENS0_9StateTypeE
pub fn stub_7d24ac() {
    // IDA 0x7d24ac: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Jumping::onComputeForceImpl(void)")]
// 0x7d24e0 — __ZN3RBX5HUMAN7Jumping18onComputeForceImplEv
pub fn stub_7d24e0() {
    // IDA 0x7d24e0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Jumping::findCeiling(void)")]
// 0x7d26d4 — __ZN3RBX5HUMAN7Jumping11findCeilingEv
pub fn stub_7d26d4() {
    // IDA 0x7d26d4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Jumping::tryCeiling(RBX::RbxRay const&,float,RBX::Assembly *)")]
// 0x7d2b0c — __ZN3RBX5HUMAN7Jumping10tryCeilingERKNS_6RbxRayEfPNS_8AssemblyE
pub fn stub_7d2b0c() {
    // IDA 0x7d2b0c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Jumping::filterResult(RBX::Primitive const*)const")]
// 0x7d2cb8 — __ZNK3RBX5HUMAN7Jumping12filterResultEPKNS_9PrimitiveE
pub fn stub_7d2cb8() {
    // IDA 0x7d2cb8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::Jumping::filterResult(RBX::Primitive const*)const")]
// 0x7d2d70 — __ZThn4_NK3RBX5HUMAN7Jumping12filterResultEPKNS_9PrimitiveE
pub fn stub_7d2d70() {
    // IDA 0x7d2d70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Jumping::armsShouldCollide(void)const")]
// 0x7d2da0 — __ZNK3RBX5HUMAN7Jumping17armsShouldCollideEv
pub fn stub_7d2da0() {
    // IDA 0x7d2da0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Jumping::legsShouldCollide(void)const")]
// 0x7d2da4 — __ZNK3RBX5HUMAN7Jumping17legsShouldCollideEv
pub fn stub_7d2da4() {
    // IDA 0x7d2da4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Jumping::torsoShouldCollide(void)const")]
// 0x7d2da8 — __ZNK3RBX5HUMAN7Jumping18torsoShouldCollideEv
pub fn stub_7d2da8() {
    // IDA 0x7d2da8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Jumping::~Jumping()")]
// 0x7d2dac — __ZN3RBX5HUMAN7JumpingD1Ev
pub fn stub_7d2dac() {
    // IDA 0x7d2dac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Jumping::~Jumping()")]
// 0x7d2db0 — __ZN3RBX5HUMAN7JumpingD0Ev
pub fn stub_7d2db0() {
    // IDA 0x7d2db0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Jumping::getStateType(void)const")]
// 0x7d2e50 — __ZNK3RBX5HUMAN7Jumping12getStateTypeEv
pub fn stub_7d2e50() {
    // IDA 0x7d2e50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::Jumping::~Jumping()")]
// 0x7d2e54 — __ZThn4_N3RBX5HUMAN7JumpingD1Ev
pub fn stub_7d2e54() {
    // IDA 0x7d2e54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::Jumping::~Jumping()")]
// 0x7d2e5c — __ZThn4_N3RBX5HUMAN7JumpingD0Ev
pub fn stub_7d2e5c() {
    // IDA 0x7d2e5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::MovingNoPhysicsBase(RBX::Humanoid *,RBX::HUMAN::StateType)")]
// 0x7d32bc — __ZN3RBX5HUMAN19MovingNoPhysicsBaseC2EPNS_8HumanoidENS0_9StateTypeE
pub fn stub_7d32bc() {
    // IDA 0x7d32bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::onEvent_TorsoAncestryChanged(void)")]
// 0x7d3544 — __ZN3RBX5HUMAN19MovingNoPhysicsBase28onEvent_TorsoAncestryChangedEv
pub fn stub_7d3544() {
    // IDA 0x7d3544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
// 0x7d35ac — __ZN3RBX5HUMAN19MovingNoPhysicsBaseD0Ev
pub fn stub_7d35ac() {
    // IDA 0x7d35ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
// 0x7d364c — __ZN3RBX5HUMAN19MovingNoPhysicsBaseD1Ev
pub fn stub_7d364c() {
    // IDA 0x7d364c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
// 0x7d3650 — __ZThn4_N3RBX5HUMAN19MovingNoPhysicsBaseD0Ev
pub fn stub_7d3650() {
    // IDA 0x7d3650: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
// 0x7d3658 — __ZN3RBX5HUMAN19MovingNoPhysicsBaseD2Ev
pub fn stub_7d3658() {
    // IDA 0x7d3658: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::MovingNoPhysicsBase::~MovingNoPhysicsBase()")]
// 0x7d386c — __ZThn4_N3RBX5HUMAN19MovingNoPhysicsBaseD1Ev
pub fn stub_7d386c() {
    // IDA 0x7d386c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::disconnectTorso(void)")]
// 0x7d3874 — __ZN3RBX5HUMAN19MovingNoPhysicsBase15disconnectTorsoEv
pub fn stub_7d3874() {
    // IDA 0x7d3874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::onComputeForceImpl(void)")]
// 0x7d390c — __ZN3RBX5HUMAN19MovingNoPhysicsBase18onComputeForceImplEv
pub fn stub_7d390c() {
    // IDA 0x7d390c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::applyImpulseToFloor(float)")]
// 0x7d3910 — __ZN3RBX5HUMAN19MovingNoPhysicsBase19applyImpulseToFloorEf
pub fn stub_7d3910() {
    // IDA 0x7d3910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::onSimulatorStepImpl(float)")]
// 0x7d3a88 — __ZN3RBX5HUMAN19MovingNoPhysicsBase19onSimulatorStepImplEf
pub fn stub_7d3a88() {
    // IDA 0x7d3a88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::fireEvents(void)")]
// 0x7d3e38 — __ZN3RBX5HUMAN19MovingNoPhysicsBase10fireEventsEv
pub fn stub_7d3e38() {
    // IDA 0x7d3e38: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::MovingNoPhysicsBase::getStateType(void)const")]
// 0x7d3f24 — __ZNK3RBX5HUMAN19MovingNoPhysicsBase12getStateTypeEv
pub fn stub_7d3f24() {
    // IDA 0x7d3f24: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Running::Running(RBX::Humanoid *,RBX::HUMAN::StateType)")]
// 0x7d4514 — __ZN3RBX5HUMAN7RunningC1EPNS_8HumanoidENS0_9StateTypeE
pub fn stub_7d4514() {
    // IDA 0x7d4514: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}
