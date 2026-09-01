//! core shard CQ — 100 core stubs EA-sorted, next uncovered after CP 0x6dffd0 (strict RBX|boost|std|rbx earliest gap).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x6e0130 — __ZThn32_N3RBX5Stats14TypedStatsItemIdED1Ev
// was: `non-virtual thunk to'RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()
pub fn stub_6e0130() -> ! {
    todo!("0x6e0130 __ZThn32_N3RBX5Stats14TypedStatsItemIdED1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
// 0x6e0278 — __ZThn32_N3RBX5Stats14TypedStatsItemIdED0Ev
// was: `non-virtual thunk to'RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()
pub fn stub_6e0278() -> ! {
    todo!("0x6e0278 __ZThn32_N3RBX5Stats14TypedStatsItemIdED0Ev")
}

#[doc(alias = "RBX::GuiTarget::process(RBX::GuiEvent const&)")]
// 0x6e12c0 — __ZN3RBX9GuiTarget7processERKNS_8GuiEventE
pub fn stub_6e12c0() -> ! {
    todo!("0x6e12c0 __ZN3RBX9GuiTarget7processERKNS_8GuiEventE")
}

#[doc(alias = "RBX::Body::Body(void)")]
// 0x6e205c — __ZN3RBX4BodyC1Ev
pub fn stub_6e205c() -> ! {
    todo!("0x6e205c __ZN3RBX4BodyC1Ev")
}

#[doc(alias = "RBX::Body::Body(void)")]
// 0x6e2060 — __ZN3RBX4BodyC2Ev
pub fn stub_6e2060() -> ! {
    todo!("0x6e2060 __ZN3RBX4BodyC2Ev")
}

#[doc(alias = "RBX::Body::getNextStateIndex(void)")]
// 0x6e2244 — __ZN3RBX4Body17getNextStateIndexEv
pub fn stub_6e2244() -> ! {
    todo!("0x6e2244 __ZN3RBX4Body17getNextStateIndexEv")
}

#[doc(alias = "RBX::Body::~Body()")]
// 0x6e2258 — __ZN3RBX4BodyD0Ev
pub fn stub_6e2258() -> ! {
    todo!("0x6e2258 __ZN3RBX4BodyD0Ev")
}

#[doc(alias = "RBX::Body::~Body()")]
// 0x6e230c — __ZN3RBX4BodyD1Ev
pub fn stub_6e230c() -> ! {
    todo!("0x6e230c __ZN3RBX4BodyD1Ev")
}

#[doc(alias = "RBX::Body::~Body()")]
// 0x6e2310 — __ZN3RBX4BodyD2Ev
pub fn stub_6e2310() -> ! {
    todo!("0x6e2310 __ZN3RBX4BodyD2Ev")
}

#[doc(alias = "RBX::Body::advanceStateIndex(void)")]
// 0x6e264c — __ZN3RBX4Body17advanceStateIndexEv
pub fn stub_6e264c() -> ! {
    todo!("0x6e264c __ZN3RBX4Body17advanceStateIndexEv")
}

#[doc(alias = "RBX::Body::initStaticData(void)")]
// 0x6e2664 — __ZN3RBX4Body14initStaticDataEv
pub fn stub_6e2664() -> ! {
    todo!("0x6e2664 __ZN3RBX4Body14initStaticDataEv")
}

#[doc(alias = "RBX::Body::getWorldBody(void)")]
// 0x6e2724 — __ZN3RBX4Body12getWorldBodyEv
pub fn stub_6e2724() -> ! {
    todo!("0x6e2724 __ZN3RBX4Body12getWorldBodyEv")
}

#[doc(alias = "RBX::Body::validateParentCofmDirty(void)")]
// 0x6e2750 — __ZN3RBX4Body23validateParentCofmDirtyEv
pub fn stub_6e2750() -> ! {
    todo!("0x6e2750 __ZN3RBX4Body23validateParentCofmDirtyEv")
}

#[doc(alias = "RBX::Body::makeCofmDirty(void)")]
// 0x6e27fc — __ZN3RBX4Body13makeCofmDirtyEv
pub fn stub_6e27fc() -> ! {
    todo!("0x6e27fc __ZN3RBX4Body13makeCofmDirtyEv")
}

#[doc(alias = "RBX::Body::resetRoot(RBX::Body*)")]
// 0x6e2a0c — __ZN3RBX4Body9resetRootEPS0_
pub fn stub_6e2a0c() -> ! {
    todo!("0x6e2a0c __ZN3RBX4Body9resetRootEPS0_")
}

#[doc(alias = "RBX::Body::onParentChanging(void)")]
// 0x6e2aec — __ZN3RBX4Body16onParentChangingEv
pub fn stub_6e2aec() -> ! {
    todo!("0x6e2aec __ZN3RBX4Body16onParentChangingEv")
}

#[doc(alias = "RBX::Body::onParentChanged(RBX::IndexedTree *)")]
// 0x6e2d10 — __ZN3RBX4Body15onParentChangedEPNS_11IndexedTreeE
pub fn stub_6e2d10() -> ! {
    todo!("0x6e2d10 __ZN3RBX4Body15onParentChangedEPNS_11IndexedTreeE")
}

#[doc(alias = "RBX::Body::onChildAdding(RBX::IndexedTree *)")]
// 0x6e2e04 — __ZN3RBX4Body13onChildAddingEPNS_11IndexedTreeE
pub fn stub_6e2e04() -> ! {
    todo!("0x6e2e04 __ZN3RBX4Body13onChildAddingEPNS_11IndexedTreeE")
}

#[doc(alias = "RBX::Body::refreshCofm(void)")]
// 0x6e2e08 — __ZN3RBX4Body11refreshCofmEv
pub fn stub_6e2e08() -> ! {
    todo!("0x6e2e08 __ZN3RBX4Body11refreshCofmEv")
}

#[doc(alias = "RBX::Body::onChildAdded(RBX::IndexedTree *)")]
// 0x6e3000 — __ZN3RBX4Body12onChildAddedEPNS_11IndexedTreeE
pub fn stub_6e3000() -> ! {
    todo!("0x6e3000 __ZN3RBX4Body12onChildAddedEPNS_11IndexedTreeE")
}

#[doc(alias = "RBX::Body::onChildRemoved(RBX::IndexedTree *)")]
// 0x6e30b0 — __ZN3RBX4Body14onChildRemovedEPNS_11IndexedTreeE
pub fn stub_6e30b0() -> ! {
    todo!("0x6e30b0 __ZN3RBX4Body14onChildRemovedEPNS_11IndexedTreeE")
}

#[doc(alias = "RBX::Body::setMeInParent(RBX::Link *)")]
// 0x6e3220 — __ZN3RBX4Body13setMeInParentEPNS_4LinkE
pub fn stub_6e3220() -> ! {
    todo!("0x6e3220 __ZN3RBX4Body13setMeInParentEPNS_4LinkE")
}

#[doc(alias = "RBX::Body::setPv(RBX::PV const&,RBX::BodyPvSetter const&)")]
// 0x6e32f0 — __ZN3RBX4Body5setPvERKNS_2PVERKNS_12BodyPvSetterE
pub fn stub_6e32f0() -> ! {
    todo!("0x6e32f0 __ZN3RBX4Body5setPvERKNS_2PVERKNS_12BodyPvSetterE")
}

#[doc(alias = "RBX::Body::setVelocity(RBX::Velocity const&,RBX::BodyPvSetter const&)")]
// 0x6e33c8 — __ZN3RBX4Body11setVelocityERKNS_8VelocityERKNS_12BodyPvSetterE
pub fn stub_6e33c8() -> ! {
    todo!("0x6e33c8 __ZN3RBX4Body11setVelocityERKNS_8VelocityERKNS_12BodyPvSetterE")
}

#[doc(alias = "RBX::Body::setCanThrottle(bool,RBX::BodyPvSetter const&)")]
// 0x6e3414 — __ZN3RBX4Body14setCanThrottleEbRKNS_12BodyPvSetterE
pub fn stub_6e3414() -> ! {
    todo!("0x6e3414 __ZN3RBX4Body14setCanThrottleEbRKNS_12BodyPvSetterE")
}

#[doc(alias = "RBX::Body::setMass(float)")]
// 0x6e341c — __ZN3RBX4Body7setMassEf
pub fn stub_6e341c() -> ! {
    todo!("0x6e341c __ZN3RBX4Body7setMassEf")
}

#[doc(alias = "RBX::Body::getBranchCofmPos(void)")]
// 0x6e3554 — __ZN3RBX4Body16getBranchCofmPosEv
pub fn stub_6e3554() -> ! {
    todo!("0x6e3554 __ZN3RBX4Body16getBranchCofmPosEv")
}

#[doc(alias = "RBX::Body::getBranchCofmOffset(void)")]
// 0x6e3618 — __ZN3RBX4Body19getBranchCofmOffsetEv
pub fn stub_6e3618() -> ! {
    todo!("0x6e3618 __ZN3RBX4Body19getBranchCofmOffsetEv")
}

#[doc(alias = "RBX::Body::getBranchCofmCoordinateFrame(void)")]
// 0x6e3710 — __ZN3RBX4Body28getBranchCofmCoordinateFrameEv
pub fn stub_6e3710() -> ! {
    todo!("0x6e3710 __ZN3RBX4Body28getBranchCofmCoordinateFrameEv")
}

#[doc(alias = "RBX::Body::kineticEnergy(void)")]
// 0x6e3744 — __ZN3RBX4Body13kineticEnergyEv
pub fn stub_6e3744() -> ! {
    todo!("0x6e3744 __ZN3RBX4Body13kineticEnergyEv")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::Allocator(void)")]
// 0x6e3844 — __ZN3RBX9AllocatorINS_4BodyEEC2Ev
pub fn stub_6e3844() -> ! {
    todo!("0x6e3844 __ZN3RBX9AllocatorINS_4BodyEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator new(unsigned long)")]
// 0x6e38a8 — __ZN3RBX9AllocatorINS_7SimBodyEEnwEm
pub fn stub_6e38a8() -> ! {
    todo!("0x6e38a8 __ZN3RBX9AllocatorINS_7SimBodyEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator delete(void *)")]
// 0x6e3918 — __ZN3RBX9AllocatorINS_7SimBodyEEdlEPv
pub fn stub_6e3918() -> ! {
    todo!("0x6e3918 __ZN3RBX9AllocatorINS_7SimBodyEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::operator delete(void *)")]
// 0x6e3958 — __ZN3RBX9AllocatorINS_4BodyEEdlEPv
pub fn stub_6e3958() -> ! {
    todo!("0x6e3958 __ZN3RBX9AllocatorINS_4BodyEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator delete(void *)")]
// 0x6e3998 — __ZN3RBX9AllocatorINS_4CofmEEdlEPv
pub fn stub_6e3998() -> ! {
    todo!("0x6e3998 __ZN3RBX9AllocatorINS_4CofmEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator new(unsigned long)")]
// 0x6e39d4 — __ZN3RBX9AllocatorINS_4CofmEEnwEm
pub fn stub_6e39d4() -> ! {
    todo!("0x6e39d4 __ZN3RBX9AllocatorINS_4CofmEEnwEm")
}

#[doc(alias = "RBX::Body::getIWorld(void)")]
// 0x6e3a44 — __ZN3RBX4Body9getIWorldEv
pub fn stub_6e3a44() -> ! {
    todo!("0x6e3a44 __ZN3RBX4Body9getIWorldEv")
}

#[doc(alias = "RBX::Body::getBranchIWorld(void)")]
// 0x6e3a70 — __ZN3RBX4Body15getBranchIWorldEv
pub fn stub_6e3a70() -> ! {
    todo!("0x6e3a70 __ZN3RBX4Body15getBranchIWorldEv")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::releaseMemory(void)")]
// 0x6e3ba4 — __ZN3RBX9AllocatorINS_4BodyEE13releaseMemoryEv
pub fn stub_6e3ba4() -> ! {
    todo!("0x6e3ba4 __ZN3RBX9AllocatorINS_4BodyEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Cofm::Cofm(RBX::Body *)")]
// 0x6e3d88 — __ZN3RBX4CofmC1EPNS_4BodyE
pub fn stub_6e3d88() -> ! {
    todo!("0x6e3d88 __ZN3RBX4CofmC1EPNS_4BodyE")
}

#[doc(alias = "RBX::Cofm::updateIfDirty(void)")]
// 0x6e3db0 — __ZN3RBX4Cofm13updateIfDirtyEv
pub fn stub_6e3db0() -> ! {
    todo!("0x6e3db0 __ZN3RBX4Cofm13updateIfDirtyEv")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::Allocator(void)")]
// 0x6e4370 — __ZN3RBX9AllocatorINS_4CofmEEC2Ev
pub fn stub_6e4370() -> ! {
    todo!("0x6e4370 __ZN3RBX9AllocatorINS_4CofmEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::Cofm>::releaseMemory(void)")]
// 0x6e43d4 — __ZN3RBX9AllocatorINS_4CofmEE13releaseMemoryEv
pub fn stub_6e43d4() -> ! {
    todo!("0x6e43d4 __ZN3RBX9AllocatorINS_4CofmEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Connector::computeCanThrottle(void)")]
// 0x6e4550 — __ZN3RBX9Connector18computeCanThrottleEv
pub fn stub_6e4550() -> ! {
    todo!("0x6e4550 __ZN3RBX9Connector18computeCanThrottleEv")
}

#[doc(alias = "RBX::PointToPointBreakConnector::getBody(RBX::Connector::BodyIndex)")]
// 0x6e4584 — __ZN3RBX26PointToPointBreakConnector7getBodyENS_9Connector9BodyIndexE
pub fn stub_6e4584() -> ! {
    todo!("0x6e4584 __ZN3RBX26PointToPointBreakConnector7getBodyENS_9Connector9BodyIndexE")
}

#[doc(alias = "RBX::RotateConnector::reset(void)")]
// 0x6e46ec — __ZN3RBX15RotateConnector5resetEv
pub fn stub_6e46ec() -> ! {
    todo!("0x6e46ec __ZN3RBX15RotateConnector5resetEv")
}

#[doc(alias = "RBX::RotateConnector::getBody(RBX::Connector::BodyIndex)")]
// 0x6e4760 — __ZN3RBX15RotateConnector7getBodyENS_9Connector9BodyIndexE
pub fn stub_6e4760() -> ! {
    todo!("0x6e4760 __ZN3RBX15RotateConnector7getBodyENS_9Connector9BodyIndexE")
}

#[doc(alias = "RBX::RotateConnector::setRotationalGoal(float)")]
// 0x6e48a8 — __ZN3RBX15RotateConnector17setRotationalGoalEf
pub fn stub_6e48a8() -> ! {
    todo!("0x6e48a8 __ZN3RBX15RotateConnector17setRotationalGoalEf")
}

#[doc(alias = "RBX::RotateConnector::setVelocityGoal(float)")]
// 0x6e49c0 — __ZN3RBX15RotateConnector15setVelocityGoalEf
pub fn stub_6e49c0() -> ! {
    todo!("0x6e49c0 __ZN3RBX15RotateConnector15setVelocityGoalEf")
}

#[doc(alias = "RBX::RotateConnector::stepGoals(void)")]
// 0x6e4a10 — __ZN3RBX15RotateConnector9stepGoalsEv
pub fn stub_6e4a10() -> ! {
    todo!("0x6e4a10 __ZN3RBX15RotateConnector9stepGoalsEv")
}

#[doc(alias = "RBX::RotateConnector::computeForce(bool)")]
// 0x6e4a4c — __ZN3RBX15RotateConnector12computeForceEb
pub fn stub_6e4a4c() -> ! {
    todo!("0x6e4a4c __ZN3RBX15RotateConnector12computeForceEb")
}

#[doc(alias = "RBX::PointToPointBreakConnector::potentialEnergy(void)")]
// 0x6e4b4c — __ZN3RBX26PointToPointBreakConnector15potentialEnergyEv
pub fn stub_6e4b4c() -> ! {
    todo!("0x6e4b4c __ZN3RBX26PointToPointBreakConnector15potentialEnergyEv")
}

#[doc(alias = "RBX::PointToPointBreakConnector::computeForce(bool)")]
// 0x6e4ba8 — __ZN3RBX26PointToPointBreakConnector12computeForceEb
pub fn stub_6e4ba8() -> ! {
    todo!("0x6e4ba8 __ZN3RBX26PointToPointBreakConnector12computeForceEb")
}

#[doc(alias = "RBX::NormalBreakConnector::computeForce(bool)")]
// 0x6e4c68 — __ZN3RBX20NormalBreakConnector12computeForceEb
pub fn stub_6e4c68() -> ! {
    todo!("0x6e4c68 __ZN3RBX20NormalBreakConnector12computeForceEb")
}

#[doc(alias = "RBX::NormalBreakConnector::~NormalBreakConnector()")]
// 0x6e4d78 — __ZN3RBX20NormalBreakConnectorD1Ev
pub fn stub_6e4d78() -> ! {
    todo!("0x6e4d78 __ZN3RBX20NormalBreakConnectorD1Ev")
}

#[doc(alias = "RBX::NormalBreakConnector::~NormalBreakConnector()")]
// 0x6e4d7c — __ZN3RBX20NormalBreakConnectorD0Ev
pub fn stub_6e4d7c() -> ! {
    todo!("0x6e4d7c __ZN3RBX20NormalBreakConnectorD0Ev")
}

#[doc(alias = "RBX::RotateConnector::~RotateConnector()")]
// 0x6e4d80 — __ZN3RBX15RotateConnectorD1Ev
pub fn stub_6e4d80() -> ! {
    todo!("0x6e4d80 __ZN3RBX15RotateConnectorD1Ev")
}

#[doc(alias = "RBX::RotateConnector::~RotateConnector()")]
// 0x6e4d84 — __ZN3RBX15RotateConnectorD0Ev
pub fn stub_6e4d84() -> ! {
    todo!("0x6e4d84 __ZN3RBX15RotateConnectorD0Ev")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::operator delete(void *)")]
// 0x6e4d88 — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv
pub fn stub_6e4d88() -> ! {
    todo!("0x6e4d88 __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv")
}

#[doc(alias = "RBX::Constants::longUiStepsPerSec(void)")]
// 0x6e4ef4 — __ZN3RBX9Constants17longUiStepsPerSecEv
pub fn stub_6e4ef4() -> ! {
    todo!("0x6e4ef4 __ZN3RBX9Constants17longUiStepsPerSecEv")
}

#[doc(alias = "RBX::Constants::worldStepsPerLongUiStep(void)")]
// 0x6e4ef8 — __ZN3RBX9Constants23worldStepsPerLongUiStepEv
pub fn stub_6e4ef8() -> ! {
    todo!("0x6e4ef8 __ZN3RBX9Constants23worldStepsPerLongUiStepEv")
}

#[doc(alias = "RBX::Constants::uiStepsPerSec(void)")]
// 0x6e4efc — __ZN3RBX9Constants13uiStepsPerSecEv
pub fn stub_6e4efc() -> ! {
    todo!("0x6e4efc __ZN3RBX9Constants13uiStepsPerSecEv")
}

#[doc(alias = "RBX::Constants::worldStepsPerUiStep(void)")]
// 0x6e4f14 — __ZN3RBX9Constants19worldStepsPerUiStepEv
pub fn stub_6e4f14() -> ! {
    todo!("0x6e4f14 __ZN3RBX9Constants19worldStepsPerUiStepEv")
}

#[doc(alias = "RBX::Constants::kernelStepsPerWorldStep(void)")]
// 0x6e4f2c — __ZN3RBX9Constants23kernelStepsPerWorldStepEv
pub fn stub_6e4f2c() -> ! {
    todo!("0x6e4f2c __ZN3RBX9Constants23kernelStepsPerWorldStepEv")
}

#[doc(alias = "RBX::Constants::worldStepsPerSec(void)")]
// 0x6e4f30 — __ZN3RBX9Constants16worldStepsPerSecEv
pub fn stub_6e4f30() -> ! {
    todo!("0x6e4f30 __ZN3RBX9Constants16worldStepsPerSecEv")
}

#[doc(alias = "RBX::Constants::impulseSolverMaxIterations(void)")]
// 0x6e4f54 — __ZN3RBX9Constants26impulseSolverMaxIterationsEv
pub fn stub_6e4f54() -> ! {
    todo!("0x6e4f54 __ZN3RBX9Constants26impulseSolverMaxIterationsEv")
}

#[doc(alias = "RBX::Constants::impulseSolverAccuracy(void)")]
// 0x6e4f58 — __ZN3RBX9Constants21impulseSolverAccuracyEv
pub fn stub_6e4f58() -> ! {
    todo!("0x6e4f58 __ZN3RBX9Constants21impulseSolverAccuracyEv")
}

#[doc(alias = "RBX::Constants::impulseSolverAccuracyScalar(void)")]
// 0x6e4f64 — __ZN3RBX9Constants27impulseSolverAccuracyScalarEv
pub fn stub_6e4f64() -> ! {
    todo!("0x6e4f64 __ZN3RBX9Constants27impulseSolverAccuracyScalarEv")
}

#[doc(alias = "RBX::Constants::impulseSolverSymStateTorqueBound(void)")]
// 0x6e4f6c — __ZN3RBX9Constants32impulseSolverSymStateTorqueBoundEv
pub fn stub_6e4f6c() -> ! {
    todo!("0x6e4f6c __ZN3RBX9Constants32impulseSolverSymStateTorqueBoundEv")
}

#[doc(alias = "RBX::Constants::impulseSolverSymStateForceBound(void)")]
// 0x6e4f78 — __ZN3RBX9Constants31impulseSolverSymStateForceBoundEv
pub fn stub_6e4f78() -> ! {
    todo!("0x6e4f78 __ZN3RBX9Constants31impulseSolverSymStateForceBoundEv")
}

#[doc(alias = "RBX::Constants::uiDt(void)")]
// 0x6e4f84 — __ZN3RBX9Constants4uiDtEv
pub fn stub_6e4f84() -> ! {
    todo!("0x6e4f84 __ZN3RBX9Constants4uiDtEv")
}

#[doc(alias = "RBX::Constants::longUiStepDt(void)")]
// 0x6e4fb4 — __ZN3RBX9Constants12longUiStepDtEv
pub fn stub_6e4fb4() -> ! {
    todo!("0x6e4fb4 __ZN3RBX9Constants12longUiStepDtEv")
}

#[doc(alias = "RBX::Constants::worldDt(void)")]
// 0x6e4fc0 — __ZN3RBX9Constants7worldDtEv
pub fn stub_6e4fc0() -> ! {
    todo!("0x6e4fc0 __ZN3RBX9Constants7worldDtEv")
}

#[doc(alias = "RBX::Constants::kernelDt(void)")]
// 0x6e4ff8 — __ZN3RBX9Constants8kernelDtEv
pub fn stub_6e4ff8() -> ! {
    todo!("0x6e4ff8 __ZN3RBX9Constants8kernelDtEv")
}

#[doc(alias = "RBX::Constants::freeFallDt(void)")]
// 0x6e5030 — __ZN3RBX9Constants10freeFallDtEv
pub fn stub_6e5030() -> ! {
    todo!("0x6e5030 __ZN3RBX9Constants10freeFallDtEv")
}

#[doc(alias = "RBX::Constants::getElasticMultiplier(float)")]
// 0x6e5068 — __ZN3RBX9Constants20getElasticMultiplierEf
pub fn stub_6e5068() -> ! {
    todo!("0x6e5068 __ZN3RBX9Constants20getElasticMultiplierEf")
}

#[doc(alias = "RBX::Constants::getKmsMaxJointForce(float,float)")]
// 0x6e50e8 — __ZN3RBX9Constants19getKmsMaxJointForceEff
pub fn stub_6e50e8() -> ! {
    todo!("0x6e50e8 __ZN3RBX9Constants19getKmsMaxJointForceEff")
}

#[doc(alias = "RBX::ContactConnector::percentActive(void)")]
// 0x6e5798 — __ZN3RBX16ContactConnector13percentActiveEv
pub fn stub_6e5798() -> ! {
    todo!("0x6e5798 __ZN3RBX16ContactConnector13percentActiveEv")
}

#[doc(alias = "RBX::ContactConnector::computeRelativeVelocity(void)")]
// 0x6e59b4 — __ZN3RBX16ContactConnector23computeRelativeVelocityEv
pub fn stub_6e59b4() -> ! {
    todo!("0x6e59b4 __ZN3RBX16ContactConnector23computeRelativeVelocityEv")
}

#[doc(alias = "RBX::ContactConnector::getReordedSimBody(RBX::SimBody *&,RBX::SimBody *&,RBX::Body *&,RBX::PairParams &)")]
// 0x6e59d8 — __ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RPNS_4BodyERNS_10PairParamsE
pub fn stub_6e59d8() -> ! {
    todo!("0x6e59d8 __ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RPNS_4BodyERNS_10PairParamsE")
}

#[doc(alias = "RBX::ContactConnector::getReordedSimBody(RBX::SimBody *&,RBX::SimBody *&,RBX::PairParams &)")]
// 0x6e5b1c — __ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RNS_10PairParamsE
pub fn stub_6e5b1c() -> ! {
    todo!("0x6e5b1c __ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RNS_10PairParamsE")
}

#[doc(alias = "RBX::ContactConnector::computeForce(bool)")]
// 0x6e5db0 — __ZN3RBX16ContactConnector12computeForceEb
pub fn stub_6e5db0() -> ! {
    todo!("0x6e5db0 __ZN3RBX16ContactConnector12computeForceEb")
}

#[doc(alias = "RBX::ContactConnector::computeImpulse(float &)")]
// 0x6e629c — __ZN3RBX16ContactConnector14computeImpulseERf
pub fn stub_6e629c() -> ! {
    todo!("0x6e629c __ZN3RBX16ContactConnector14computeImpulseERf")
}

#[doc(alias = "RBX::ContactConnector::applyContactPointForSymmetryDetection(RBX::SimBody *,RBX::SimBody *,RBX::PairParams const&,float)")]
// 0x6e6b30 — __ZN3RBX16ContactConnector37applyContactPointForSymmetryDetectionEPNS_7SimBodyES2_RKNS_10PairParamsEf
pub fn stub_6e6b30() -> ! {
    todo!("0x6e6b30 __ZN3RBX16ContactConnector37applyContactPointForSymmetryDetectionEPNS_7SimBodyES2_RKNS_10PairParamsEf")
}

#[doc(alias = "RBX::ContactConnector::updateContactPoint(void)")]
// 0x6e6d30 — __ZN3RBX16ContactConnector18updateContactPointEv
pub fn stub_6e6d30() -> ! {
    todo!("0x6e6d30 __ZN3RBX16ContactConnector18updateContactPointEv")
}

#[doc(alias = "RBX::BallBallConnector::updateContactPoint(void)")]
// 0x6e6e20 — __ZN3RBX17BallBallConnector18updateContactPointEv
pub fn stub_6e6e20() -> ! {
    todo!("0x6e6e20 __ZN3RBX17BallBallConnector18updateContactPointEv")
}

#[doc(alias = "RBX::BallBlockConnector::updateContactPoint(void)")]
// 0x6e6ed4 — __ZN3RBX18BallBlockConnector18updateContactPointEv
pub fn stub_6e6ed4() -> ! {
    todo!("0x6e6ed4 __ZN3RBX18BallBlockConnector18updateContactPointEv")
}

#[doc(alias = "RBX::BallBlockConnector::computeBallPlane(RBX::PairParams &)")]
// 0x6e6f0c — __ZN3RBX18BallBlockConnector16computeBallPlaneERNS_10PairParamsE
pub fn stub_6e6f0c() -> ! {
    todo!("0x6e6f0c __ZN3RBX18BallBlockConnector16computeBallPlaneERNS_10PairParamsE")
}

#[doc(alias = "RBX::BallBlockConnector::computeBallEdge(RBX::PairParams &)")]
// 0x6e70c0 — __ZN3RBX18BallBlockConnector15computeBallEdgeERNS_10PairParamsE
pub fn stub_6e70c0() -> ! {
    todo!("0x6e70c0 __ZN3RBX18BallBlockConnector15computeBallEdgeERNS_10PairParamsE")
}

#[doc(alias = "RBX::BallBlockConnector::computeBallPoint(RBX::PairParams &)")]
// 0x6e7254 — __ZN3RBX18BallBlockConnector16computeBallPointERNS_10PairParamsE
pub fn stub_6e7254() -> ! {
    todo!("0x6e7254 __ZN3RBX18BallBlockConnector16computeBallPointERNS_10PairParamsE")
}

#[doc(alias = "RBX::PairParams::operator==(RBX::PairParams const&)")]
// 0x6e7340 — __ZN3RBX10PairParamseqERKS0_
pub fn stub_6e7340() -> ! {
    todo!("0x6e7340 __ZN3RBX10PairParamseqERKS0_")
}

#[doc(alias = "RBX::ContactConnector::getConnectorKernelType(void)const")]
// 0x6e73d0 — __ZNK3RBX16ContactConnector22getConnectorKernelTypeEv
pub fn stub_6e73d0() -> ! {
    todo!("0x6e73d0 __ZNK3RBX16ContactConnector22getConnectorKernelTypeEv")
}

#[doc(alias = "RBX::ContactConnector::~ContactConnector()")]
// 0x6e73d4 — __ZN3RBX16ContactConnectorD1Ev
pub fn stub_6e73d4() -> ! {
    todo!("0x6e73d4 __ZN3RBX16ContactConnectorD1Ev")
}

#[doc(alias = "RBX::ContactConnector::~ContactConnector()")]
// 0x6e73d8 — __ZN3RBX16ContactConnectorD0Ev
pub fn stub_6e73d8() -> ! {
    todo!("0x6e73d8 __ZN3RBX16ContactConnectorD0Ev")
}

#[doc(alias = "RBX::ContactConnector::getBody(RBX::Connector::BodyIndex)")]
// 0x6e73dc — __ZN3RBX16ContactConnector7getBodyENS_9Connector9BodyIndexE
pub fn stub_6e73dc() -> ! {
    todo!("0x6e73dc __ZN3RBX16ContactConnector7getBodyENS_9Connector9BodyIndexE")
}

#[doc(alias = "RBX::BallBallConnector::~BallBallConnector()")]
// 0x6e73ec — __ZN3RBX17BallBallConnectorD1Ev
pub fn stub_6e73ec() -> ! {
    todo!("0x6e73ec __ZN3RBX17BallBallConnectorD1Ev")
}

#[doc(alias = "RBX::BallBallConnector::~BallBallConnector()")]
// 0x6e73f0 — __ZN3RBX17BallBallConnectorD0Ev
pub fn stub_6e73f0() -> ! {
    todo!("0x6e73f0 __ZN3RBX17BallBallConnectorD0Ev")
}

#[doc(alias = "RBX::BallBlockConnector::~BallBlockConnector()")]
// 0x6e73f4 — __ZN3RBX18BallBlockConnectorD1Ev
pub fn stub_6e73f4() -> ! {
    todo!("0x6e73f4 __ZN3RBX18BallBlockConnectorD1Ev")
}

#[doc(alias = "RBX::BallBlockConnector::~BallBlockConnector()")]
// 0x6e73f8 — __ZN3RBX18BallBlockConnectorD0Ev
pub fn stub_6e73f8() -> ! {
    todo!("0x6e73f8 __ZN3RBX18BallBlockConnectorD0Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockConnector>::operator delete(void *)")]
// 0x6e73fc — __ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv
pub fn stub_6e73fc() -> ! {
    todo!("0x6e73fc __ZN3RBX9AllocatorINS_18BallBlockConnectorEEdlEPv")
}
