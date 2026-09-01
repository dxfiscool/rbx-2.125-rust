//! core shard CY — 100 core stubs EA-sorted, next uncovered after CX 0x73649c (strict RBX|boost|std|rbx earliest gap).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

// 0x736564 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_insert_unique(RBX::IMoving * const&)")]
pub fn stub_736564() -> ! {
    todo!("0x736564 __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

// 0x7365cc — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::IMoving * const&)")]
pub fn stub_7365cc() -> ! {
    todo!("0x7365cc __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

// 0x736624 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::erase(RBX::IMoving * const&)")]
pub fn stub_736624() -> ! {
    todo!("0x736624 __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")
}

// 0x73664c — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::equal_range(RBX::IMoving * const&)")]
pub fn stub_73664c() -> ! {
    todo!("0x73664c __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")
}

// 0x736698 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::erase(std::_Rb_tree_iterator<RBX::IMoving *>,std::_Rb_tree_iterator<RBX::IMoving *>)")]
pub fn stub_736698() -> ! {
    todo!("0x736698 __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")
}

// 0x7366f8 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_erase(std::_Rb_tree_node<RBX::IMoving *> *)")]
pub fn stub_7366f8() -> ! {
    todo!("0x7366f8 __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

// 0x736720 — __ZNK3RBX10IPipelined9getKernelEv
#[doc(alias = "RBX::IPipelined::getKernel(void)const")]
pub fn stub_736720() -> ! {
    todo!("0x736720 __ZNK3RBX10IPipelined9getKernelEv")
}

// 0x736788 — __ZNK3RBX10IPipelined8getStageENS_6IStage9StageTypeE
#[doc(alias = "RBX::IPipelined::getStage(RBX::IStage::StageType)const")]
pub fn stub_736788() -> ! {
    todo!("0x736788 __ZNK3RBX10IPipelined8getStageENS_6IStage9StageTypeE")
}

// 0x736808 — __ZN3RBX10IPipelined11putInKernelEPNS_6KernelE
#[doc(alias = "RBX::IPipelined::putInKernel(RBX::Kernel *)")]
pub fn stub_736808() -> ! {
    todo!("0x736808 __ZN3RBX10IPipelined11putInKernelEPNS_6KernelE")
}

// 0x73680c — __ZN3RBX10IPipelined10putInStageEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::putInStage(RBX::IStage *)")]
pub fn stub_73680c() -> ! {
    todo!("0x73680c __ZN3RBX10IPipelined10putInStageEPNS_6IStageE")
}

// 0x73693c — __ZN3RBX10IPipelined16removeFromKernelEv
#[doc(alias = "RBX::IPipelined::removeFromKernel(void)")]
pub fn stub_73693c() -> ! {
    todo!("0x73693c __ZN3RBX10IPipelined16removeFromKernelEv")
}

// 0x7369e8 — __ZN3RBX10IPipelined15removeFromStageEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::removeFromStage(RBX::IStage *)")]
pub fn stub_7369e8() -> ! {
    todo!("0x7369e8 __ZN3RBX10IPipelined15removeFromStageEPNS_6IStageE")
}

// 0x736b10 — __ZN3RBX10IPipelined13putInPipelineEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::putInPipeline(RBX::IStage *)")]
pub fn stub_736b10() -> ! {
    todo!("0x736b10 __ZN3RBX10IPipelined13putInPipelineEPNS_6IStageE")
}

// 0x736bb0 — __ZN3RBX10IPipelined18removeFromPipelineEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::removeFromPipeline(RBX::IStage *)")]
pub fn stub_736bb0() -> ! {
    todo!("0x736bb0 __ZN3RBX10IPipelined18removeFromPipelineEPNS_6IStageE")
}

// 0x736ccc — __ZN3RBX11IWorldStage11onEdgeAddedEPNS_4EdgeE
#[doc(alias = "RBX::IWorldStage::onEdgeAdded(RBX::Edge *)")]
pub fn stub_736ccc() -> ! {
    todo!("0x736ccc __ZN3RBX11IWorldStage11onEdgeAddedEPNS_4EdgeE")
}

// 0x736d38 — __ZN3RBX11IWorldStage14onEdgeRemovingEPNS_4EdgeE
#[doc(alias = "RBX::IWorldStage::onEdgeRemoving(RBX::Edge *)")]
pub fn stub_736d38() -> ! {
    todo!("0x736d38 __ZN3RBX11IWorldStage14onEdgeRemovingEPNS_4EdgeE")
}

// 0x736da4 — __ZN3RBX11IWorldStageD1Ev
#[doc(alias = "RBX::IWorldStage::~IWorldStage()")]
pub fn stub_736da4() -> ! {
    todo!("0x736da4 __ZN3RBX11IWorldStageD1Ev")
}

// 0x736dc8 — __ZN3RBX11IWorldStageD0Ev
#[doc(alias = "RBX::IWorldStage::~IWorldStage()")]
pub fn stub_736dc8() -> ! {
    todo!("0x736dc8 __ZN3RBX11IWorldStageD0Ev")
}

// 0x7374b8 — __ZN3RBX5JointC2Ev
#[doc(alias = "RBX::Joint::Joint(void)")]
pub fn stub_7374b8() -> ! {
    todo!("0x7374b8 __ZN3RBX5JointC2Ev")
}

// 0x737730 — __ZN3RBX5JointD0Ev
#[doc(alias = "RBX::Joint::~Joint()")]
pub fn stub_737730() -> ! {
    todo!("0x737730 __ZN3RBX5JointD0Ev")
}

// 0x7377d0 — __ZN3RBX5JointD1Ev
#[doc(alias = "RBX::Joint::~Joint()")]
pub fn stub_7377d0() -> ! {
    todo!("0x7377d0 __ZN3RBX5JointD1Ev")
}

// 0x7377d4 — __ZThn32_N3RBX5JointD0Ev
#[doc(alias = "non-virtual thunk to RBX::Joint::~Joint()")]
// was: non-virtual thunk to RBX::Joint::~Joint()
pub fn stub_7377d4() -> ! {
    todo!("0x7377d4 __ZThn32_N3RBX5JointD0Ev")
}

// 0x7377dc — __ZN3RBX5JointD2Ev
#[doc(alias = "RBX::Joint::~Joint()")]
pub fn stub_7377dc() -> ! {
    todo!("0x7377dc __ZN3RBX5JointD2Ev")
}

// 0x737924 — __ZThn32_N3RBX5JointD1Ev
#[doc(alias = "non-virtual thunk to RBX::Joint::~Joint()")]
// was: non-virtual thunk to RBX::Joint::~Joint()
pub fn stub_737924() -> ! {
    todo!("0x737924 __ZThn32_N3RBX5JointD1Ev")
}

// 0x73792c — __ZN3RBX5Joint18getJointWorldCoordEi
#[doc(alias = "RBX::Joint::getJointWorldCoord(int)")]
pub fn stub_73792c() -> ! {
    todo!("0x73792c __ZN3RBX5Joint18getJointWorldCoordEi")
}

// 0x7379c0 — __ZN3RBX5Joint11notifyMovedEv
#[doc(alias = "RBX::Joint::notifyMoved(void)")]
pub fn stub_7379c0() -> ! {
    todo!("0x7379c0 __ZN3RBX5Joint11notifyMovedEv")
}

// 0x7379ec — __ZN3RBX5Joint14findConstJointEPKNS_9PrimitiveENS0_9JointTypeE
#[doc(alias = "RBX::Joint::findConstJoint(RBX::Primitive const*,RBX::Joint::JointType)")]
pub fn stub_7379ec() -> ! {
    todo!("0x7379ec __ZN3RBX5Joint14findConstJointEPKNS_9PrimitiveENS0_9JointTypeE")
}

// 0x737a24 — __ZN3RBX5Joint13getConstJointEPKNS_9PrimitiveENS0_9JointTypeE
#[doc(alias = "RBX::Joint::getConstJoint(RBX::Primitive const*,RBX::Joint::JointType)")]
pub fn stub_737a24() -> ! {
    todo!("0x737a24 __ZN3RBX5Joint13getConstJointEPKNS_9PrimitiveENS0_9JointTypeE")
}

// 0x737a80 — __ZN3RBX5Joint8getJointEPNS_9PrimitiveENS0_9JointTypeE
#[doc(alias = "RBX::Joint::getJoint(RBX::Primitive *,RBX::Joint::JointType)")]
pub fn stub_737a80() -> ! {
    todo!("0x737a80 __ZN3RBX5Joint8getJointEPNS_9PrimitiveENS0_9JointTypeE")
}

// 0x737a84 — __ZN3RBX5Joint13setJointOwnerEPNS_11IJointOwnerE
#[doc(alias = "RBX::Joint::setJointOwner(RBX::IJointOwner *)")]
pub fn stub_737a84() -> ! {
    todo!("0x737a84 __ZN3RBX5Joint13setJointOwnerEPNS_11IJointOwnerE")
}

// 0x737afc — __ZNK3RBX5Joint13getJointOwnerEv
#[doc(alias = "RBX::Joint::getJointOwner(void)const")]
pub fn stub_737afc() -> ! {
    todo!("0x737afc __ZNK3RBX5Joint13getJointOwnerEv")
}

// 0x737b00 — __ZN3RBX5Joint12setPrimitiveEiPNS_9PrimitiveE
#[doc(alias = "RBX::Joint::setPrimitive(int,RBX::Primitive *)")]
pub fn stub_737b00() -> ! {
    todo!("0x737b00 __ZN3RBX5Joint12setPrimitiveEiPNS_9PrimitiveE")
}

// 0x737c74 — __ZN3RBX5Joint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_ff
#[doc(alias = "RBX::Joint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId,float,float)")]
pub fn stub_737c74() -> ! {
    todo!("0x737c74 __ZN3RBX5Joint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_ff")
}

// 0x737f80 — __ZN3RBX5Joint15FacesOverlappedEPKNS_9PrimitiveEmS3_mf
#[doc(alias = "RBX::Joint::FacesOverlapped(RBX::Primitive const*,unsigned long,RBX::Primitive const*,unsigned long,float)")]
pub fn stub_737f80() -> ! {
    todo!("0x737f80 __ZN3RBX5Joint15FacesOverlappedEPKNS_9PrimitiveEmS3_mf")
}

// 0x737fc8 — __ZN3RBX5Joint18canBuildJointTightEPNS_9PrimitiveES2_NS_8NormalIdES3_
#[doc(alias = "RBX::Joint::canBuildJointTight(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
pub fn stub_737fc8() -> ! {
    todo!("0x737fc8 __ZN3RBX5Joint18canBuildJointTightEPNS_9PrimitiveES2_NS_8NormalIdES3_")
}

// 0x737fe8 — __ZN3RBX5Joint18canBuildJointLooseEPNS_9PrimitiveES2_NS_8NormalIdES3_
#[doc(alias = "RBX::Joint::canBuildJointLoose(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
pub fn stub_737fe8() -> ! {
    todo!("0x737fe8 __ZN3RBX5Joint18canBuildJointLooseEPNS_9PrimitiveES2_NS_8NormalIdES3_")
}

// 0x738008 — __ZN3RBX13getJointSize2EPKNS_5JointE
#[doc(alias = "RBX::getJointSize2(RBX::Joint const*)")]
pub fn stub_738008() -> ! {
    todo!("0x738008 __ZN3RBX13getJointSize2EPKNS_5JointE")
}

// 0x738028 — __ZNK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE
#[doc(alias = "RBX::Joint::isHeavierThan(RBX::SpanningEdge const*)const")]
pub fn stub_738028() -> ! {
    todo!("0x738028 __ZNK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE")
}

// 0x73825c — __ZThn32_NK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE
#[doc(alias = "non-virtual thunk to RBX::Joint::isHeavierThan(RBX::SpanningEdge const*)const")]
// was: non-virtual thunk to RBX::Joint::isHeavierThan(RBX::SpanningEdge const*)const
pub fn stub_73825c() -> ! {
    todo!("0x73825c __ZThn32_NK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE")
}

// 0x738268 — __ZN3RBX5Joint9otherNodeEPNS_12SpanningNodeE
#[doc(alias = "RBX::Joint::otherNode(RBX::SpanningNode *)")]
pub fn stub_738268() -> ! {
    todo!("0x738268 __ZN3RBX5Joint9otherNodeEPNS_12SpanningNodeE")
}

// 0x738284 — __ZThn32_N3RBX5Joint9otherNodeEPNS_12SpanningNodeE
#[doc(alias = "non-virtual thunk to RBX::Joint::otherNode(RBX::SpanningNode *)")]
// was: non-virtual thunk to RBX::Joint::otherNode(RBX::SpanningNode *)
pub fn stub_738284() -> ! {
    todo!("0x738284 __ZThn32_N3RBX5Joint9otherNodeEPNS_12SpanningNodeE")
}

// 0x7382a4 — __ZNK3RBX5Joint14otherConstNodeEPKNS_12SpanningNodeE
#[doc(alias = "RBX::Joint::otherConstNode(RBX::SpanningNode const*)const")]
pub fn stub_7382a4() -> ! {
    todo!("0x7382a4 __ZNK3RBX5Joint14otherConstNodeEPKNS_12SpanningNodeE")
}

// 0x7382c0 — __ZThn32_NK3RBX5Joint14otherConstNodeEPKNS_12SpanningNodeE
#[doc(alias = "non-virtual thunk to RBX::Joint::otherConstNode(RBX::SpanningNode const*)const")]
// was: non-virtual thunk to RBX::Joint::otherConstNode(RBX::SpanningNode const*)const
pub fn stub_7382c0() -> ! {
    todo!("0x7382c0 __ZThn32_NK3RBX5Joint14otherConstNodeEPKNS_12SpanningNodeE")
}

// 0x7382e0 — __ZN3RBX5Joint7getNodeEi
#[doc(alias = "RBX::Joint::getNode(int)")]
pub fn stub_7382e0() -> ! {
    todo!("0x7382e0 __ZN3RBX5Joint7getNodeEi")
}

// 0x7382f0 — __ZThn32_N3RBX5Joint7getNodeEi
#[doc(alias = "non-virtual thunk to RBX::Joint::getNode(int)")]
// was: non-virtual thunk to RBX::Joint::getNode(int)
pub fn stub_7382f0() -> ! {
    todo!("0x7382f0 __ZThn32_N3RBX5Joint7getNodeEi")
}

// 0x738300 — __ZNK3RBX5Joint12getConstNodeEi
#[doc(alias = "RBX::Joint::getConstNode(int)const")]
pub fn stub_738300() -> ! {
    todo!("0x738300 __ZNK3RBX5Joint12getConstNodeEi")
}

// 0x738310 — __ZThn32_NK3RBX5Joint12getConstNodeEi
#[doc(alias = "non-virtual thunk to RBX::Joint::getConstNode(int)const")]
// was: non-virtual thunk to RBX::Joint::getConstNode(int)const
pub fn stub_738310() -> ! {
    todo!("0x738310 __ZThn32_NK3RBX5Joint12getConstNodeEi")
}

// 0x738320 — __ZN3RBX5Joint22FaceVerticesOverlappedEPKNS_9PrimitiveEmS3_mf
#[doc(alias = "RBX::Joint::FaceVerticesOverlapped(RBX::Primitive const*,unsigned long,RBX::Primitive const*,unsigned long,float)")]
pub fn stub_738320() -> ! {
    todo!("0x738320 __ZN3RBX5Joint22FaceVerticesOverlappedEPKNS_9PrimitiveEmS3_mf")
}

// 0x73867c — __ZN3RBX5Joint19FaceEdgesOverlappedEPKNS_9PrimitiveEmS3_mf
#[doc(alias = "RBX::Joint::FaceEdgesOverlapped(RBX::Primitive const*,unsigned long,RBX::Primitive const*,unsigned long,float)")]
pub fn stub_73867c() -> ! {
    todo!("0x73867c __ZN3RBX5Joint19FaceEdgesOverlappedEPKNS_9PrimitiveEmS3_mf")
}

// 0x738d30 — __ZN3RBX5Joint24getSurfaceTypeFromNormalERKNS_9PrimitiveERKNS_8NormalIdE
#[doc(alias = "RBX::Joint::getSurfaceTypeFromNormal(RBX::Primitive const&,RBX::NormalId const&)")]
pub fn stub_738d30() -> ! {
    todo!("0x738d30 __ZN3RBX5Joint24getSurfaceTypeFromNormalERKNS_9PrimitiveERKNS_8NormalIdE")
}

// 0x738d4c — __ZN3RBX5Joint27compatibleForHingeAutoJointERKNS_9PrimitiveERmS3_S4_
#[doc(alias = "RBX::Joint::compatibleForHingeAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
pub fn stub_738d4c() -> ! {
    todo!("0x738d4c __ZN3RBX5Joint27compatibleForHingeAutoJointERKNS_9PrimitiveERmS3_S4_")
}

// 0x738dc8 — __ZN3RBX5Joint26compatibleForGlueAutoJointERKNS_9PrimitiveERmS3_S4_
#[doc(alias = "RBX::Joint::compatibleForGlueAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
pub fn stub_738dc8() -> ! {
    todo!("0x738dc8 __ZN3RBX5Joint26compatibleForGlueAutoJointERKNS_9PrimitiveERmS3_S4_")
}

// 0x738e0c — __ZN3RBX5Joint26compatibleForWeldAutoJointERKNS_9PrimitiveERmS3_S4_
#[doc(alias = "RBX::Joint::compatibleForWeldAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
pub fn stub_738e0c() -> ! {
    todo!("0x738e0c __ZN3RBX5Joint26compatibleForWeldAutoJointERKNS_9PrimitiveERmS3_S4_")
}

// 0x738e58 — __ZN3RBX5Joint26compatibleForStudAutoJointERKNS_9PrimitiveERmS3_S4_
#[doc(alias = "RBX::Joint::compatibleForStudAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
pub fn stub_738e58() -> ! {
    todo!("0x738e58 __ZN3RBX5Joint26compatibleForStudAutoJointERKNS_9PrimitiveERmS3_S4_")
}

// 0x738ed0 — __ZN3RBX5Joint23inCompatibleForAnyJointERKNS_9PrimitiveERmS3_S4_
#[doc(alias = "RBX::Joint::inCompatibleForAnyJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
pub fn stub_738ed0() -> ! {
    todo!("0x738ed0 __ZN3RBX5Joint23inCompatibleForAnyJointERKNS_9PrimitiveERmS3_S4_")
}

// 0x738f48 — __ZN3RBX5Joint26positionedForStudAutoJointERKNS_9PrimitiveERmS3_S4_
#[doc(alias = "RBX::Joint::positionedForStudAutoJoint(RBX::Primitive const&,unsigned long &,RBX::Primitive const&,unsigned long &)")]
pub fn stub_738f48() -> ! {
    todo!("0x738f48 __ZN3RBX5Joint26positionedForStudAutoJointERKNS_9PrimitiveERmS3_S4_")
}

// 0x7393ac — __ZN3RBX10IPipelined9findWorldEv
#[doc(alias = "RBX::IPipelined::findWorld(void)")]
pub fn stub_7393ac() -> ! {
    todo!("0x7393ac __ZN3RBX10IPipelined9findWorldEv")
}

// 0x7393d0 — __ZNK3RBX5Joint12getJointTypeEv
#[doc(alias = "RBX::Joint::getJointType(void)const")]
pub fn stub_7393d0() -> ! {
    todo!("0x7393d0 __ZNK3RBX5Joint12getJointTypeEv")
}

// 0x739424 — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::resize(unsigned long,RBX::Joint::JointType)")]
pub fn stub_739424() -> ! {
    todo!("0x739424 __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE6resizeEmS2_")
}

// 0x739458 — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::push_back(RBX::Joint::JointType const&)")]
pub fn stub_739458() -> ! {
    todo!("0x739458 __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE9push_backERKS2_")
}

// 0x739480 — __ZNSt3mapIPKN3RBX4NameENS0_5Joint9JointTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Joint::JointType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_739480() -> ! {
    todo!("0x739480 __ZNSt3mapIPKN3RBX4NameENS0_5Joint9JointTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

// 0x7394d8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
pub fn stub_7394d8() -> ! {
    todo!("0x7394d8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

// 0x73958c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
pub fn stub_73958c() -> ! {
    todo!("0x73958c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

// 0x7395e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
pub fn stub_7395e4() -> ! {
    todo!("0x7395e4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

// 0x73964c — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Joint::JointType*,std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>>,RBX::Joint::JointType const&)")]
pub fn stub_73964c() -> ! {
    todo!("0x73964c __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

// 0x739730 — __ZNSt12_Vector_baseIN3RBX5Joint9JointTypeESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_allocate(unsigned long)")]
pub fn stub_739730() -> ! {
    todo!("0x739730 __ZNSt12_Vector_baseIN3RBX5Joint9JointTypeESaIS2_EE11_M_allocateEm")
}

// 0x739748 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Joint9JointTypeES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Joint::JointType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Joint::JointType *,RBX::Joint::JointType *>(RBX::Joint::JointType *,RBX::Joint::JointType *,RBX::Joint::JointType *)")]
pub fn stub_739748() -> ! {
    todo!("0x739748 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Joint9JointTypeES6_EET0_T_S8_S7_")
}

// 0x739784 — __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Joint::JointType*,std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>>,unsigned long,RBX::Joint::JointType const&)")]
pub fn stub_739784() -> ! {
    todo!("0x739784 __ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

// 0x739e3c — __ZN3RBX12JointBuilder7canJoinEPNS_9PrimitiveES2_
#[doc(alias = "RBX::JointBuilder::canJoin(RBX::Primitive *,RBX::Primitive *)")]
pub fn stub_739e3c() -> ! {
    todo!("0x739e3c __ZN3RBX12JointBuilder7canJoinEPNS_9PrimitiveES2_")
}

// 0x73a0f0 — __ZN3RBX10JointStageC1EPNS_6IStageEPNS_5WorldE
#[doc(alias = "RBX::JointStage::JointStage(RBX::IStage *,RBX::World *)")]
pub fn stub_73a0f0() -> ! {
    todo!("0x73a0f0 __ZN3RBX10JointStageC1EPNS_6IStageEPNS_5WorldE")
}

// 0x73a0f4 — __ZN3RBX10JointStageC2EPNS_6IStageEPNS_5WorldE
#[doc(alias = "RBX::JointStage::JointStage(RBX::IStage *,RBX::World *)")]
pub fn stub_73a0f4() -> ! {
    todo!("0x73a0f4 __ZN3RBX10JointStageC2EPNS_6IStageEPNS_5WorldE")
}

// 0x73a20c — __ZN3RBX10JointStageD0Ev
#[doc(alias = "RBX::JointStage::~JointStage()")]
pub fn stub_73a20c() -> ! {
    todo!("0x73a20c __ZN3RBX10JointStageD0Ev")
}

// 0x73a2ac — __ZN3RBX10JointStageD1Ev
#[doc(alias = "RBX::JointStage::~JointStage()")]
pub fn stub_73a2ac() -> ! {
    todo!("0x73a2ac __ZN3RBX10JointStageD1Ev")
}

// 0x73a2b0 — __ZN3RBX10JointStageD2Ev
#[doc(alias = "RBX::JointStage::~JointStage()")]
pub fn stub_73a2b0() -> ! {
    todo!("0x73a2b0 __ZN3RBX10JointStageD2Ev")
}

// 0x73a514 — __ZN3RBX10JointStage20moveEdgeToDownstreamEPNS_4EdgeE
#[doc(alias = "RBX::JointStage::moveEdgeToDownstream(RBX::Edge *)")]
pub fn stub_73a514() -> ! {
    todo!("0x73a514 __ZN3RBX10JointStage20moveEdgeToDownstreamEPNS_4EdgeE")
}

// 0x73a580 — __ZN3RBX10JointStage21edgeHasPrimitivesHereEPNS_4EdgeE
#[doc(alias = "RBX::JointStage::edgeHasPrimitivesHere(RBX::Edge *)")]
pub fn stub_73a580() -> ! {
    todo!("0x73a580 __ZN3RBX10JointStage21edgeHasPrimitivesHereEPNS_4EdgeE")
}

// 0x73a5a4 — __ZN3RBX10JointStage24removeEdgeFromDownstreamEPNS_4EdgeE
#[doc(alias = "RBX::JointStage::removeEdgeFromDownstream(RBX::Edge *)")]
pub fn stub_73a5a4() -> ! {
    todo!("0x73a5a4 __ZN3RBX10JointStage24removeEdgeFromDownstreamEPNS_4EdgeE")
}

// 0x73a610 — __ZN3RBX10JointStage20edgeHasPrimitiveHereEPNS_4EdgeEPNS_9PrimitiveE
#[doc(alias = "RBX::JointStage::edgeHasPrimitiveHere(RBX::Edge *,RBX::Primitive *)")]
pub fn stub_73a610() -> ! {
    todo!("0x73a610 __ZN3RBX10JointStage20edgeHasPrimitiveHereEPNS_4EdgeEPNS_9PrimitiveE")
}

// 0x73a6b4 — __ZN3RBX10JointStage19visitAddedPrimitiveEPNS_9PrimitiveEPNS_5JointERSt6vectorIS4_SaIS4_EE
#[doc(alias = "RBX::JointStage::visitAddedPrimitive(RBX::Primitive *,RBX::Joint *,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>> &)")]
pub fn stub_73a6b4() -> ! {
    todo!("0x73a6b4 __ZN3RBX10JointStage19visitAddedPrimitiveEPNS_9PrimitiveEPNS_5JointERSt6vectorIS4_SaIS4_EE")
}

// 0x73a744 — __ZN3RBX10JointStage16onPrimitiveAddedEPNS_9PrimitiveE
#[doc(alias = "RBX::JointStage::onPrimitiveAdded(RBX::Primitive *)")]
pub fn stub_73a744() -> ! {
    todo!("0x73a744 __ZN3RBX10JointStage16onPrimitiveAddedEPNS_9PrimitiveE")
}

// 0x73a94c — __ZN3RBX10JointStage19removeJointFromHereEPNS_5JointE
#[doc(alias = "RBX::JointStage::removeJointFromHere(RBX::Joint *)")]
pub fn stub_73a94c() -> ! {
    todo!("0x73a94c __ZN3RBX10JointStage19removeJointFromHereEPNS_5JointE")
}

// 0x73a9dc — __ZN3RBX10JointStage19onPrimitiveRemovingEPNS_9PrimitiveE
#[doc(alias = "RBX::JointStage::onPrimitiveRemoving(RBX::Primitive *)")]
pub fn stub_73a9dc() -> ! {
    todo!("0x73a9dc __ZN3RBX10JointStage19onPrimitiveRemovingEPNS_9PrimitiveE")
}

// 0x73acec — __ZN3RBX10JointStage12putJointHereEPNS_5JointE
#[doc(alias = "RBX::JointStage::putJointHere(RBX::Joint *)")]
pub fn stub_73acec() -> ! {
    todo!("0x73acec __ZN3RBX10JointStage12putJointHereEPNS_5JointE")
}

// 0x73ad78 — __ZN3RBX10JointStage11onEdgeAddedEPNS_4EdgeE
#[doc(alias = "RBX::JointStage::onEdgeAdded(RBX::Edge *)")]
pub fn stub_73ad78() -> ! {
    todo!("0x73ad78 __ZN3RBX10JointStage11onEdgeAddedEPNS_4EdgeE")
}

// 0x73ae5c — __ZN3RBX10JointStage14onEdgeRemovingEPNS_4EdgeE
#[doc(alias = "RBX::JointStage::onEdgeRemoving(RBX::Edge *)")]
pub fn stub_73ae5c() -> ! {
    todo!("0x73ae5c __ZN3RBX10JointStage14onEdgeRemovingEPNS_4EdgeE")
}

// 0x73af78 — __ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE10removePairERKS2_RKS4_
#[doc(alias = "RBX::BiMultiMap<RBX::Primitive *,RBX::Joint *>::removePair(RBX::Primitive * const&,RBX::Joint * const&)")]
pub fn stub_73af78() -> ! {
    todo!("0x73af78 __ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE10removePairERKS2_RKS4_")
}

// 0x73b0a4 — __ZNK3RBX10JointStage12getStageTypeEv
#[doc(alias = "RBX::JointStage::getStageType(void)const")]
pub fn stub_73b0a4() -> ! {
    todo!("0x73b0a4 __ZNK3RBX10JointStage12getStageTypeEv")
}

// 0x73b0a8 — __ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE9pairInMapERKS2_RKS4_
#[doc(alias = "RBX::BiMultiMap<RBX::Primitive *,RBX::Joint *>::pairInMap(RBX::Primitive * const&,RBX::Joint * const&)")]
pub fn stub_73b0a8() -> ! {
    todo!("0x73b0a8 __ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE9pairInMapERKS2_RKS4_")
}

// 0x73b118 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE15_M_insert_equalERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_insert_equal(std::pair<RBX::Primitive * const,RBX::Joint *> const&)")]
pub fn stub_73b118() -> ! {
    todo!("0x73b118 __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE15_M_insert_equalERKS7_")
}

// 0x73b144 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Primitive * const,RBX::Joint *> const&)")]
pub fn stub_73b144() -> ! {
    todo!("0x73b144 __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

// 0x73b19c — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::erase(RBX::Primitive * const&)")]
pub fn stub_73b19c() -> ! {
    todo!("0x73b19c __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")
}

// 0x73b1c4 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::equal_range(RBX::Primitive * const&)")]
pub fn stub_73b1c4() -> ! {
    todo!("0x73b1c4 __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")
}

// 0x73b210 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::erase(std::_Rb_tree_iterator<RBX::Primitive *>,std::_Rb_tree_iterator<RBX::Primitive *>)")]
pub fn stub_73b210() -> ! {
    todo!("0x73b210 __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")
}

// 0x73b2a8 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Primitive * const,RBX::Joint *>> *)")]
pub fn stub_73b2a8() -> ! {
    todo!("0x73b2a8 __ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

// 0x73b398 — __ZN3RBX11KernelJoint11putInKernelEPNS_6KernelE
#[doc(alias = "RBX::KernelJoint::putInKernel(RBX::Kernel *)")]
pub fn stub_73b398() -> ! {
    todo!("0x73b398 __ZN3RBX11KernelJoint11putInKernelEPNS_6KernelE")
}

// 0x73b3b4 — __ZN3RBX11KernelJoint16removeFromKernelEv
#[doc(alias = "RBX::KernelJoint::removeFromKernel(void)")]
pub fn stub_73b3b4() -> ! {
    todo!("0x73b3b4 __ZN3RBX11KernelJoint16removeFromKernelEv")
}

// 0x73bb4c — __ZNSt3mapIPKN3RBX4NameENS0_11SurfaceTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
#[doc(alias = "std::map<RBX::Name const*,RBX::SurfaceType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_73bb4c() -> ! {
    todo!("0x73bb4c __ZNSt3mapIPKN3RBX4NameENS0_11SurfaceTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

// 0x73bba4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
pub fn stub_73bba4() -> ! {
    todo!("0x73bba4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

// 0x73bc58 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
pub fn stub_73bc58() -> ! {
    todo!("0x73bc58 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

// 0x73bcb0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
pub fn stub_73bcb0() -> ! {
    todo!("0x73bcb0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}