//! core shard CS — 100 core stubs EA-sorted, next uncovered after CR 0x71611c (strict RBX|boost|std|rbx earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Allocator<RBX::BallBallConnector>::operator delete(void *)")]
// 0x6e743c — __ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv
pub fn stub_6e743c() -> ! {
    todo!("0x6e743c __ZN3RBX9AllocatorINS_17BallBallConnectorEEdlEPv")
}

#[doc(alias = "RBX::Kernel::Kernel(RBX::IStage *)")]
// 0x6e7648 — __ZN3RBX6KernelC1EPNS_6IStageE
pub fn stub_6e7648() -> ! {
    todo!("0x6e7648 __ZN3RBX6KernelC1EPNS_6IStageE")
}

#[doc(alias = "RBX::Kernel::Kernel(RBX::IStage *)")]
// 0x6e764c — __ZN3RBX6KernelC2EPNS_6IStageE
pub fn stub_6e764c() -> ! {
    todo!("0x6e764c __ZN3RBX6KernelC2EPNS_6IStageE")
}

#[doc(alias = "RBX::Kernel::~Kernel()")]
// 0x6e77fc — __ZN3RBX6KernelD0Ev
pub fn stub_6e77fc() -> ! {
    todo!("0x6e77fc __ZN3RBX6KernelD0Ev")
}

#[doc(alias = "RBX::Kernel::~Kernel()")]
// 0x6e789c — __ZN3RBX6KernelD1Ev
pub fn stub_6e789c() -> ! {
    todo!("0x6e789c __ZN3RBX6KernelD1Ev")
}

#[doc(alias = "RBX::Kernel::~Kernel()")]
// 0x6e78a0 — __ZN3RBX6KernelD2Ev
pub fn stub_6e78a0() -> ! {
    todo!("0x6e78a0 __ZN3RBX6KernelD2Ev")
}

#[doc(alias = "RBX::Kernel::validateConnector(RBX::Connector *)const")]
// 0x6e7a80 — __ZNK3RBX6Kernel17validateConnectorEPNS_9ConnectorE
pub fn stub_6e7a80() -> ! {
    todo!("0x6e7a80 __ZNK3RBX6Kernel17validateConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::Kernel::validateConnectorBody(RBX::Body *)const")]
// 0x6e7aac — __ZNK3RBX6Kernel21validateConnectorBodyEPNS_4BodyE
pub fn stub_6e7aac() -> ! {
    todo!("0x6e7aac __ZNK3RBX6Kernel21validateConnectorBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::Kernel::insertBody(RBX::Body *)")]
// 0x6e7af8 — __ZN3RBX6Kernel10insertBodyEPNS_4BodyE
pub fn stub_6e7af8() -> ! {
    todo!("0x6e7af8 __ZN3RBX6Kernel10insertBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::Kernel::removeBody(RBX::Body *)")]
// 0x6e7b80 — __ZN3RBX6Kernel10removeBodyEPNS_4BodyE
pub fn stub_6e7b80() -> ! {
    todo!("0x6e7b80 __ZN3RBX6Kernel10removeBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::Kernel::insertPoint(RBX::Point *)")]
// 0x6e7d1c — __ZN3RBX6Kernel11insertPointEPNS_5PointE
pub fn stub_6e7d1c() -> ! {
    todo!("0x6e7d1c __ZN3RBX6Kernel11insertPointEPNS_5PointE")
}

#[doc(alias = "RBX::Kernel::insertConnector(RBX::Connector *)")]
// 0x6e7e18 — __ZN3RBX6Kernel15insertConnectorEPNS_9ConnectorE
pub fn stub_6e7e18() -> ! {
    todo!("0x6e7e18 __ZN3RBX6Kernel15insertConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::Kernel::removePoint(RBX::Point *)")]
// 0x6e7e80 — __ZN3RBX6Kernel11removePointEPNS_5PointE
pub fn stub_6e7e80() -> ! {
    todo!("0x6e7e80 __ZN3RBX6Kernel11removePointEPNS_5PointE")
}

#[doc(alias = "RBX::Kernel::removeConnector(RBX::Connector *)")]
// 0x6e7ee8 — __ZN3RBX6Kernel15removeConnectorEPNS_9ConnectorE
pub fn stub_6e7ee8() -> ! {
    todo!("0x6e7ee8 __ZN3RBX6Kernel15removeConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::Kernel::numFreeFallBodies(void)const")]
// 0x6e7f50 — __ZNK3RBX6Kernel17numFreeFallBodiesEv
pub fn stub_6e7f50() -> ! {
    todo!("0x6e7f50 __ZNK3RBX6Kernel17numFreeFallBodiesEv")
}

#[doc(alias = "RBX::Kernel::numRealTimeBodies(void)const")]
// 0x6e7f58 — __ZNK3RBX6Kernel17numRealTimeBodiesEv
pub fn stub_6e7f58() -> ! {
    todo!("0x6e7f58 __ZNK3RBX6Kernel17numRealTimeBodiesEv")
}

#[doc(alias = "RBX::Kernel::numJointBodies(void)const")]
// 0x6e7f60 — __ZNK3RBX6Kernel14numJointBodiesEv
pub fn stub_6e7f60() -> ! {
    todo!("0x6e7f60 __ZNK3RBX6Kernel14numJointBodiesEv")
}

#[doc(alias = "RBX::Kernel::numContactBodies(void)const")]
// 0x6e7f68 — __ZNK3RBX6Kernel16numContactBodiesEv
pub fn stub_6e7f68() -> ! {
    todo!("0x6e7f68 __ZNK3RBX6Kernel16numContactBodiesEv")
}

#[doc(alias = "RBX::Kernel::numLeafBodies(void)const")]
// 0x6e7f70 — __ZNK3RBX6Kernel13numLeafBodiesEv
pub fn stub_6e7f70() -> ! {
    todo!("0x6e7f70 __ZNK3RBX6Kernel13numLeafBodiesEv")
}

#[doc(alias = "RBX::Kernel::numPoints(void)const")]
// 0x6e7f78 — __ZNK3RBX6Kernel9numPointsEv
pub fn stub_6e7f78() -> ! {
    todo!("0x6e7f78 __ZNK3RBX6Kernel9numPointsEv")
}

#[doc(alias = "RBX::Kernel::numHumanoidConnectors(void)const")]
// 0x6e7f80 — __ZNK3RBX6Kernel21numHumanoidConnectorsEv
pub fn stub_6e7f80() -> ! {
    todo!("0x6e7f80 __ZNK3RBX6Kernel21numHumanoidConnectorsEv")
}

#[doc(alias = "RBX::Kernel::numRealTimeConnectors(void)const")]
// 0x6e7f88 — __ZNK3RBX6Kernel21numRealTimeConnectorsEv
pub fn stub_6e7f88() -> ! {
    todo!("0x6e7f88 __ZNK3RBX6Kernel21numRealTimeConnectorsEv")
}

#[doc(alias = "RBX::Kernel::numSecondPassConnectors(void)const")]
// 0x6e7f90 — __ZNK3RBX6Kernel23numSecondPassConnectorsEv
pub fn stub_6e7f90() -> ! {
    todo!("0x6e7f90 __ZNK3RBX6Kernel23numSecondPassConnectorsEv")
}

#[doc(alias = "RBX::Kernel::numJointConnectors(void)const")]
// 0x6e7f98 — __ZNK3RBX6Kernel18numJointConnectorsEv
pub fn stub_6e7f98() -> ! {
    todo!("0x6e7f98 __ZNK3RBX6Kernel18numJointConnectorsEv")
}

#[doc(alias = "RBX::Kernel::numContactConnectors(void)const")]
// 0x6e7fa0 — __ZNK3RBX6Kernel20numContactConnectorsEv
pub fn stub_6e7fa0() -> ! {
    todo!("0x6e7fa0 __ZNK3RBX6Kernel20numContactConnectorsEv")
}

#[doc(alias = "RBX::Kernel::numConnectors(void)const")]
// 0x6e7fa8 — __ZNK3RBX6Kernel13numConnectorsEv
pub fn stub_6e7fa8() -> ! {
    todo!("0x6e7fa8 __ZNK3RBX6Kernel13numConnectorsEv")
}

#[doc(alias = "RBX::Kernel::searchForDuplicatePoint(RBX::Point *)")]
// 0x6e7fc0 — __ZN3RBX6Kernel23searchForDuplicatePointEPNS_5PointE
pub fn stub_6e7fc0() -> ! {
    todo!("0x6e7fc0 __ZN3RBX6Kernel23searchForDuplicatePointEPNS_5PointE")
}

#[doc(alias = "RBX::Kernel::deletePoint(RBX::Point *)")]
// 0x6e827c — __ZN3RBX6Kernel11deletePointEPNS_5PointE
pub fn stub_6e827c() -> ! {
    todo!("0x6e827c __ZN3RBX6Kernel11deletePointEPNS_5PointE")
}

#[doc(alias = "RBX::Kernel::step(bool,int)")]
// 0x6e82fc — __ZN3RBX6Kernel4stepEbi
pub fn stub_6e82fc() -> ! {
    todo!("0x6e82fc __ZN3RBX6Kernel4stepEbi")
}

#[doc(alias = "RBX::Kernel::preStepThrottled(void)")]
// 0x6e837c — __ZN3RBX6Kernel16preStepThrottledEv
pub fn stub_6e837c() -> ! {
    todo!("0x6e837c __ZN3RBX6Kernel16preStepThrottledEv")
}

#[doc(alias = "RBX::Kernel::stepWorldThrottled(void)")]
// 0x6e84d0 — __ZN3RBX6Kernel18stepWorldThrottledEv
pub fn stub_6e84d0() -> ! {
    todo!("0x6e84d0 __ZN3RBX6Kernel18stepWorldThrottledEv")
}

#[doc(alias = "RBX::Kernel::preStep(void)")]
// 0x6e89c8 — __ZN3RBX6Kernel7preStepEv
pub fn stub_6e89c8() -> ! {
    todo!("0x6e89c8 __ZN3RBX6Kernel7preStepEv")
}

#[doc(alias = "RBX::Kernel::stepWorld(void)")]
// 0x6e8fe0 — __ZN3RBX6Kernel9stepWorldEv
pub fn stub_6e8fe0() -> ! {
    todo!("0x6e8fe0 __ZN3RBX6Kernel9stepWorldEv")
}

#[doc(alias = "RBX::Kernel::connectorSpringEnergy(void)const")]
// 0x6e9da8 — __ZNK3RBX6Kernel21connectorSpringEnergyEv
pub fn stub_6e9da8() -> ! {
    todo!("0x6e9da8 __ZNK3RBX6Kernel21connectorSpringEnergyEv")
}

#[doc(alias = "RBX::Kernel::bodyKineticEnergy(void)const")]
// 0x6ea004 — __ZNK3RBX6Kernel17bodyKineticEnergyEv
pub fn stub_6ea004() -> ! {
    todo!("0x6ea004 __ZNK3RBX6Kernel17bodyKineticEnergyEv")
}

#[doc(alias = "RBX::Kernel::fakeDeceptiveSolverIterations(void)const")]
// 0x6ea250 — __ZNK3RBX6Kernel29fakeDeceptiveSolverIterationsEv
pub fn stub_6ea250() -> ! {
    todo!("0x6ea250 __ZNK3RBX6Kernel29fakeDeceptiveSolverIterationsEv")
}

#[doc(alias = "RBX::Kernel::fakeDeceptiveMatrixSize(void)const")]
// 0x6ea2ac — __ZNK3RBX6Kernel23fakeDeceptiveMatrixSizeEv
pub fn stub_6ea2ac() -> ! {
    todo!("0x6ea2ac __ZNK3RBX6Kernel23fakeDeceptiveMatrixSizeEv")
}

#[doc(alias = "RBX::KernelData::insertBody(RBX::Body *)")]
// 0x6ea2e8 — __ZN3RBX10KernelData10insertBodyEPNS_4BodyE
pub fn stub_6ea2e8() -> ! {
    todo!("0x6ea2e8 __ZN3RBX10KernelData10insertBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::removeBody(RBX::Body *)")]
// 0x6ea44c — __ZN3RBX10KernelData10removeBodyEPNS_4BodyE
pub fn stub_6ea44c() -> ! {
    todo!("0x6ea44c __ZN3RBX10KernelData10removeBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::KernelData::addConnector(RBX::Connector *)")]
// 0x6ea5b4 — __ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE
pub fn stub_6ea5b4() -> ! {
    todo!("0x6ea5b4 __ZN3RBX10KernelData12addConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::IndexArray<RBX::Point,&RBX::Point::getKernelIndex>::fastRemove(RBX::Point*)")]
// 0x6ead54 — __ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ead54() -> ! {
    todo!("0x6ead54 __ZN3RBX10IndexArrayINS_5PointEXadL_ZNS1_14getKernelIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::KernelData::removeConnector(RBX::Connector *)")]
// 0x6eae28 — __ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE
pub fn stub_6eae28() -> ! {
    todo!("0x6eae28 __ZN3RBX10KernelData15removeConnectorEPNS_9ConnectorE")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getContactBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0x6eb28c — __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_19getContactBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_6eb28c() -> ! {
    todo!("0x6eb28c __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_19getContactBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::KernelData::addLeafBodies(RBX::Body *)")]
// 0x6eb364 — __ZN3RBX10KernelData13addLeafBodiesEPNS_4BodyE
pub fn stub_6eb364() -> ! {
    todo!("0x6eb364 __ZN3RBX10KernelData13addLeafBodiesEPNS_4BodyE")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getFreeFallBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0x6eb488 — __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getFreeFallBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_6eb488() -> ! {
    todo!("0x6eb488 __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getFreeFallBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::Kernel::getStageType(void)const")]
// 0x6eb560 — __ZNK3RBX6Kernel12getStageTypeEv
pub fn stub_6eb560() -> ! {
    todo!("0x6eb560 __ZNK3RBX6Kernel12getStageTypeEv")
}

#[doc(alias = "RBX::Kernel::getKernel(void)")]
// 0x6eb564 — __ZN3RBX6Kernel9getKernelEv
pub fn stub_6eb564() -> ! {
    todo!("0x6eb564 __ZN3RBX6Kernel9getKernelEv")
}

#[doc(alias = "RBX::KernelData::addLeafBody(RBX::Body *)")]
// 0x6ebb60 — __ZN3RBX10KernelData11addLeafBodyEPNS_4BodyE
pub fn stub_6ebb60() -> ! {
    todo!("0x6ebb60 __ZN3RBX10KernelData11addLeafBodyEPNS_4BodyE")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getHumanoidIndex>::fastRemove(RBX::Connector*)")]
// 0x6ebfe0 — __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getHumanoidIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ebfe0() -> ! {
    todo!("0x6ebfe0 __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getHumanoidIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getSecondPassIndex>::fastRemove(RBX::Connector*)")]
// 0x6ec0b4 — __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_18getSecondPassIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ec0b4() -> ! {
    todo!("0x6ec0b4 __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_18getSecondPassIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getRealTimeIndex>::fastRemove(RBX::Connector*)")]
// 0x6ec188 — __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getRealTimeIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ec188() -> ! {
    todo!("0x6ec188 __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_16getRealTimeIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getJointIndex>::fastRemove(RBX::Connector*)")]
// 0x6ec25c — __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_13getJointIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ec25c() -> ! {
    todo!("0x6ec25c __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_13getJointIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Connector,&RBX::Connector::getContactIndex>::fastRemove(RBX::Connector*)")]
// 0x6ec330 — __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_15getContactIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ec330() -> ! {
    todo!("0x6ec330 __ZN3RBX10IndexArrayINS_9ConnectorEXadL_ZNS1_15getContactIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::KernelData::addBodyToNewList(RBX::SimBody *)")]
// 0x6ec404 — __ZN3RBX10KernelData16addBodyToNewListEPNS_7SimBodyE
pub fn stub_6ec404() -> ! {
    todo!("0x6ec404 __ZN3RBX10KernelData16addBodyToNewListEPNS_7SimBodyE")
}

#[doc(alias = "RBX::KernelData::removeBodyFromCurrentList(RBX::SimBody *)")]
// 0x6ec7cc — __ZN3RBX10KernelData25removeBodyFromCurrentListEPNS_7SimBodyE
pub fn stub_6ec7cc() -> ! {
    todo!("0x6ec7cc __ZN3RBX10KernelData25removeBodyFromCurrentListEPNS_7SimBodyE")
}

#[doc(alias = "RBX::KernelData::removeLeafBodies(RBX::Body *)")]
// 0x6ec840 — __ZN3RBX10KernelData16removeLeafBodiesEPNS_4BodyE
pub fn stub_6ec840() -> ! {
    todo!("0x6ec840 __ZN3RBX10KernelData16removeLeafBodiesEPNS_4BodyE")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getRealTimeBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0x6ec928 — __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getRealTimeBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ec928() -> ! {
    todo!("0x6ec928 __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_20getRealTimeBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::SimBody,&RBX::SimBody::getJointBodyIndex>::fastRemove(RBX::SimBody*)")]
// 0x6eca00 — __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_17getJointBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_6eca00() -> ! {
    todo!("0x6eca00 __ZN3RBX10IndexArrayINS_7SimBodyEXadL_ZNS1_17getJointBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::Body,&RBX::Body::getLeafBodyIndex>::fastRemove(RBX::Body*)")]
// 0x6ecad8 — __ZN3RBX10IndexArrayINS_4BodyEXadL_ZNS1_16getLeafBodyIndexEvEEE10fastRemoveEPS1_
pub fn stub_6ecad8() -> ! {
    todo!("0x6ecad8 __ZN3RBX10IndexArrayINS_4BodyEXadL_ZNS1_16getLeafBodyIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::KernelData::~KernelData()")]
// 0x6ecea8 — __ZN3RBX10KernelDataD2Ev
pub fn stub_6ecea8() -> ! {
    todo!("0x6ecea8 __ZN3RBX10KernelDataD2Ev")
}

#[doc(alias = "RBX::KernelData::KernelData(void)")]
// 0x6ed7c0 — __ZN3RBX10KernelDataC2Ev
pub fn stub_6ed7c0() -> ! {
    todo!("0x6ed7c0 __ZN3RBX10KernelDataC2Ev")
}

#[doc(alias = "RBX::Link::Link(void)")]
// 0x6ee018 — __ZN3RBX4LinkC2Ev
pub fn stub_6ee018() -> ! {
    todo!("0x6ee018 __ZN3RBX4LinkC2Ev")
}

#[doc(alias = "RBX::Link::~Link()")]
// 0x6ee0fc — __ZN3RBX4LinkD2Ev
pub fn stub_6ee0fc() -> ! {
    todo!("0x6ee0fc __ZN3RBX4LinkD2Ev")
}

#[doc(alias = "RBX::Link::dirty(void)")]
// 0x6ee100 — __ZN3RBX4Link5dirtyEv
pub fn stub_6ee100() -> ! {
    todo!("0x6ee100 __ZN3RBX4Link5dirtyEv")
}

#[doc(alias = "RBX::Link::getChildInParent(void)")]
// 0x6ee16c — __ZN3RBX4Link16getChildInParentEv
pub fn stub_6ee16c() -> ! {
    todo!("0x6ee16c __ZN3RBX4Link16getChildInParentEv")
}

#[doc(alias = "RBX::GeoPair::GeoPair(void)")]
// 0x6ee578 — __ZN3RBX7GeoPairC1Ev
pub fn stub_6ee578() -> ! {
    todo!("0x6ee578 __ZN3RBX7GeoPairC1Ev")
}

#[doc(alias = "RBX::GeoPair::computePointPlane(RBX::PairParams &)")]
// 0x6ee590 — __ZN3RBX7GeoPair17computePointPlaneERNS_10PairParamsE
pub fn stub_6ee590() -> ! {
    todo!("0x6ee590 __ZN3RBX7GeoPair17computePointPlaneERNS_10PairParamsE")
}

#[doc(alias = "RBX::GeoPair::computeEdgeEdgePlane2(RBX::PairParams &)")]
// 0x6ee760 — __ZN3RBX7GeoPair21computeEdgeEdgePlane2ERNS_10PairParamsE
pub fn stub_6ee760() -> ! {
    todo!("0x6ee760 __ZN3RBX7GeoPair21computeEdgeEdgePlane2ERNS_10PairParamsE")
}

#[doc(alias = "RBX::GeoPair::computeEdgeEdge(RBX::PairParams &)")]
// 0x6ef018 — __ZN3RBX7GeoPair15computeEdgeEdgeERNS_10PairParamsE
pub fn stub_6ef018() -> ! {
    todo!("0x6ef018 __ZN3RBX7GeoPair15computeEdgeEdgeERNS_10PairParamsE")
}

#[doc(alias = "RBX::Point::Point(RBX::Body *)")]
// 0x6ef420 — __ZN3RBX5PointC1EPNS_4BodyE
pub fn stub_6ef420() -> ! {
    todo!("0x6ef420 __ZN3RBX5PointC1EPNS_4BodyE")
}

#[doc(alias = "RBX::Point::Point(RBX::Body *)")]
// 0x6ef424 — __ZN3RBX5PointC2EPNS_4BodyE
pub fn stub_6ef424() -> ! {
    todo!("0x6ef424 __ZN3RBX5PointC2EPNS_4BodyE")
}

#[doc(alias = "RBX::Point::step(void)")]
// 0x6ef510 — __ZN3RBX5Point4stepEv
pub fn stub_6ef510() -> ! {
    todo!("0x6ef510 __ZN3RBX5Point4stepEv")
}

#[doc(alias = "RBX::Point::forceToBody(void)")]
// 0x6ef5c4 — __ZN3RBX5Point11forceToBodyEv
pub fn stub_6ef5c4() -> ! {
    todo!("0x6ef5c4 __ZN3RBX5Point11forceToBodyEv")
}

#[doc(alias = "RBX::KernelIndex::~KernelIndex()")]
// 0x6ef73c — __ZN3RBX11KernelIndexD2Ev
pub fn stub_6ef73c() -> ! {
    todo!("0x6ef73c __ZN3RBX11KernelIndexD2Ev")
}

#[doc(alias = "RBX::Point::~Point()")]
// 0x6ef79c — __ZN3RBX5PointD1Ev
pub fn stub_6ef79c() -> ! {
    todo!("0x6ef79c __ZN3RBX5PointD1Ev")
}

#[doc(alias = "RBX::Point::~Point()")]
// 0x6ef7ac — __ZN3RBX5PointD0Ev
pub fn stub_6ef7ac() -> ! {
    todo!("0x6ef7ac __ZN3RBX5PointD0Ev")
}

#[doc(alias = "RBX::BallPlaneConnector::updateContactPoint(void)")]
// 0x6ef948 — __ZN3RBX18BallPlaneConnector18updateContactPointEv
pub fn stub_6ef948() -> ! {
    todo!("0x6ef948 __ZN3RBX18BallPlaneConnector18updateContactPointEv")
}

#[doc(alias = "RBX::BallEdgeConnector::updateContactPoint(void)")]
// 0x6efafc — __ZN3RBX17BallEdgeConnector18updateContactPointEv
pub fn stub_6efafc() -> ! {
    todo!("0x6efafc __ZN3RBX17BallEdgeConnector18updateContactPointEv")
}

#[doc(alias = "RBX::BallVertexConnector::updateContactPoint(void)")]
// 0x6efccc — __ZN3RBX19BallVertexConnector18updateContactPointEv
pub fn stub_6efccc() -> ! {
    todo!("0x6efccc __ZN3RBX19BallVertexConnector18updateContactPointEv")
}

#[doc(alias = "RBX::FaceVertexConnector::updateContactPoint(void)")]
// 0x6efdc0 — __ZN3RBX19FaceVertexConnector18updateContactPointEv
pub fn stub_6efdc0() -> ! {
    todo!("0x6efdc0 __ZN3RBX19FaceVertexConnector18updateContactPointEv")
}

#[doc(alias = "RBX::FaceEdgeConnector::updateContactPoint(void)")]
// 0x6efeb4 — __ZN3RBX17FaceEdgeConnector18updateContactPointEv
pub fn stub_6efeb4() -> ! {
    todo!("0x6efeb4 __ZN3RBX17FaceEdgeConnector18updateContactPointEv")
}

#[doc(alias = "RBX::EdgeEdgeConnector::updateContactPoint(void)")]
// 0x6f01dc — __ZN3RBX17EdgeEdgeConnector18updateContactPointEv
pub fn stub_6f01dc() -> ! {
    todo!("0x6f01dc __ZN3RBX17EdgeEdgeConnector18updateContactPointEv")
}

#[doc(alias = "RBX::FaceVertexConnector::~FaceVertexConnector()")]
// 0x6f0704 — __ZN3RBX19FaceVertexConnectorD1Ev
pub fn stub_6f0704() -> ! {
    todo!("0x6f0704 __ZN3RBX19FaceVertexConnectorD1Ev")
}

#[doc(alias = "RBX::FaceVertexConnector::~FaceVertexConnector()")]
// 0x6f0708 — __ZN3RBX19FaceVertexConnectorD0Ev
pub fn stub_6f0708() -> ! {
    todo!("0x6f0708 __ZN3RBX19FaceVertexConnectorD0Ev")
}

#[doc(alias = "RBX::FaceEdgeConnector::~FaceEdgeConnector()")]
// 0x6f070c — __ZN3RBX17FaceEdgeConnectorD1Ev
pub fn stub_6f070c() -> ! {
    todo!("0x6f070c __ZN3RBX17FaceEdgeConnectorD1Ev")
}

#[doc(alias = "RBX::FaceEdgeConnector::~FaceEdgeConnector()")]
// 0x6f0710 — __ZN3RBX17FaceEdgeConnectorD0Ev
pub fn stub_6f0710() -> ! {
    todo!("0x6f0710 __ZN3RBX17FaceEdgeConnectorD0Ev")
}

#[doc(alias = "RBX::EdgeEdgeConnector::~EdgeEdgeConnector()")]
// 0x6f0714 — __ZN3RBX17EdgeEdgeConnectorD1Ev
pub fn stub_6f0714() -> ! {
    todo!("0x6f0714 __ZN3RBX17EdgeEdgeConnectorD1Ev")
}

#[doc(alias = "RBX::EdgeEdgeConnector::~EdgeEdgeConnector()")]
// 0x6f0718 — __ZN3RBX17EdgeEdgeConnectorD0Ev
pub fn stub_6f0718() -> ! {
    todo!("0x6f0718 __ZN3RBX17EdgeEdgeConnectorD0Ev")
}

#[doc(alias = "RBX::BallVertexConnector::~BallVertexConnector()")]
// 0x6f071c — __ZN3RBX19BallVertexConnectorD1Ev
pub fn stub_6f071c() -> ! {
    todo!("0x6f071c __ZN3RBX19BallVertexConnectorD1Ev")
}

#[doc(alias = "RBX::BallVertexConnector::~BallVertexConnector()")]
// 0x6f0720 — __ZN3RBX19BallVertexConnectorD0Ev
pub fn stub_6f0720() -> ! {
    todo!("0x6f0720 __ZN3RBX19BallVertexConnectorD0Ev")
}

#[doc(alias = "RBX::BallEdgeConnector::~BallEdgeConnector()")]
// 0x6f0724 — __ZN3RBX17BallEdgeConnectorD1Ev
pub fn stub_6f0724() -> ! {
    todo!("0x6f0724 __ZN3RBX17BallEdgeConnectorD1Ev")
}

#[doc(alias = "RBX::BallEdgeConnector::~BallEdgeConnector()")]
// 0x6f0728 — __ZN3RBX17BallEdgeConnectorD0Ev
pub fn stub_6f0728() -> ! {
    todo!("0x6f0728 __ZN3RBX17BallEdgeConnectorD0Ev")
}

#[doc(alias = "RBX::BallPlaneConnector::~BallPlaneConnector()")]
// 0x6f072c — __ZN3RBX18BallPlaneConnectorD1Ev
pub fn stub_6f072c() -> ! {
    todo!("0x6f072c __ZN3RBX18BallPlaneConnectorD1Ev")
}

#[doc(alias = "RBX::BallPlaneConnector::~BallPlaneConnector()")]
// 0x6f0730 — __ZN3RBX18BallPlaneConnectorD0Ev
pub fn stub_6f0730() -> ! {
    todo!("0x6f0730 __ZN3RBX18BallPlaneConnectorD0Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallPlaneConnector>::operator delete(void *)")]
// 0x6f0734 — __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEdlEPv
pub fn stub_6f0734() -> ! {
    todo!("0x6f0734 __ZN3RBX9AllocatorINS_18BallPlaneConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallEdgeConnector>::operator delete(void *)")]
// 0x6f0774 — __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEdlEPv
pub fn stub_6f0774() -> ! {
    todo!("0x6f0774 __ZN3RBX9AllocatorINS_17BallEdgeConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::BallVertexConnector>::operator delete(void *)")]
// 0x6f07b4 — __ZN3RBX9AllocatorINS_19BallVertexConnectorEEdlEPv
pub fn stub_6f07b4() -> ! {
    todo!("0x6f07b4 __ZN3RBX9AllocatorINS_19BallVertexConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::operator delete(void *)")]
// 0x6f07f4 — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEdlEPv
pub fn stub_6f07f4() -> ! {
    todo!("0x6f07f4 __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::operator delete(void *)")]
// 0x6f0834 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEdlEPv
pub fn stub_6f0834() -> ! {
    todo!("0x6f0834 __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::operator delete(void *)")]
// 0x6f0874 — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEdlEPv
pub fn stub_6f0874() -> ! {
    todo!("0x6f0874 __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEdlEPv")
}
