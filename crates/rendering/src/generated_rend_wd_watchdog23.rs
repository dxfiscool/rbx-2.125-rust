//! rendering shard rend_wd_watchdog23 — 120 stubs 0x7355bc..0x88d984 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre|G3D|Render|Adorn|View filtered 0 remaining -> 0, global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x7355bb
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7355bc — __ZN3RBX9FreeJointD1Ev
#[doc(alias = "RBX::FreeJoint::~FreeJoint()")]
#[doc(alias = "__ZN3RBX9FreeJointD1Ev")]
// IDA 0x7355bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7355bc() {
}

// 0x7355c0 — __ZN3RBX9FreeJointD0Ev
#[doc(alias = "RBX::FreeJoint::~FreeJoint()")]
#[doc(alias = "__ZN3RBX9FreeJointD0Ev")]
// IDA 0x7355c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7355c0() {
}

// 0x735660 — __ZNK3RBX9FreeJoint12getJointTypeEv
#[doc(alias = "RBX::FreeJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX9FreeJoint12getJointTypeEv")]
// IDA 0x735660: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735660() {
}

// 0x735664 — __ZThn32_N3RBX9FreeJointD1Ev
#[doc(alias = "non-virtual thunk toRBX::FreeJoint::~FreeJoint()")]
#[doc(alias = "__ZThn32_N3RBX9FreeJointD1Ev")]
// IDA 0x735664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_735664() {
}

// 0x73566c — __ZThn32_N3RBX9FreeJointD0Ev
#[doc(alias = "non-virtual thunk toRBX::FreeJoint::~FreeJoint()")]
#[doc(alias = "__ZThn32_N3RBX9FreeJointD0Ev")]
// IDA 0x73566c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73566c() {
}

// 0x735710 — __ZN3RBX11AnchorJointC2EPNS_9PrimitiveE
#[doc(alias = "RBX::AnchorJoint::AnchorJoint(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11AnchorJointC2EPNS_9PrimitiveE")]
// IDA 0x735710: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735710() {
}

// 0x7357dc — __ZN3RBX11AnchorJointD1Ev
#[doc(alias = "RBX::AnchorJoint::~AnchorJoint()")]
#[doc(alias = "__ZN3RBX11AnchorJointD1Ev")]
// IDA 0x7357dc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7357dc() {
}

// 0x7357e0 — __ZN3RBX11AnchorJointD0Ev
#[doc(alias = "RBX::AnchorJoint::~AnchorJoint()")]
#[doc(alias = "__ZN3RBX11AnchorJointD0Ev")]
// IDA 0x7357e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7357e0() {
}

// 0x735880 — __ZNK3RBX11AnchorJoint12getJointTypeEv
#[doc(alias = "RBX::AnchorJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX11AnchorJoint12getJointTypeEv")]
// IDA 0x735880: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735880() {
}

// 0x735884 — __ZThn32_N3RBX11AnchorJointD1Ev
#[doc(alias = "non-virtual thunk toRBX::AnchorJoint::~AnchorJoint()")]
#[doc(alias = "__ZThn32_N3RBX11AnchorJointD1Ev")]
// IDA 0x735884: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_735884() {
}

// 0x73588c — __ZThn32_N3RBX11AnchorJointD0Ev
#[doc(alias = "non-virtual thunk toRBX::AnchorJoint::~AnchorJoint()")]
#[doc(alias = "__ZThn32_N3RBX11AnchorJointD0Ev")]
// IDA 0x73588c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73588c() {
}

// 0x735930 — __GLOBAL__I_a_320
#[doc(alias = "global constructor keyed to_a_320")]
#[doc(alias = "__GLOBAL__I_a_320")]
// IDA 0x735930: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_735930() {
}

// 0x735a2c — __ZN3RBX13HumanoidStageC1EPNS_6IStageEPNS_5WorldE
#[doc(alias = "RBX::HumanoidStage::HumanoidStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13HumanoidStageC1EPNS_6IStageEPNS_5WorldE")]
// IDA 0x735a2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_735a2c() {
}

// 0x735a30 — __ZN3RBX13HumanoidStageC2EPNS_6IStageEPNS_5WorldE
#[doc(alias = "RBX::HumanoidStage::HumanoidStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13HumanoidStageC2EPNS_6IStageEPNS_5WorldE")]
// IDA 0x735a30: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735a30() {
}

// 0x735b18 — __ZN3RBX13HumanoidStageD0Ev
#[doc(alias = "RBX::HumanoidStage::~HumanoidStage()")]
#[doc(alias = "__ZN3RBX13HumanoidStageD0Ev")]
// IDA 0x735b18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_735b18() {
}

// 0x735bb8 — __ZN3RBX13HumanoidStageD1Ev
#[doc(alias = "RBX::HumanoidStage::~HumanoidStage()")]
#[doc(alias = "__ZN3RBX13HumanoidStageD1Ev")]
// IDA 0x735bb8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_735bb8() {
}

// 0x735bbc — __ZN3RBX13HumanoidStageD2Ev
#[doc(alias = "RBX::HumanoidStage::~HumanoidStage()")]
#[doc(alias = "__ZN3RBX13HumanoidStageD2Ev")]
// IDA 0x735bbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_735bbc() {
}

// 0x735d20 — __ZN3RBX13HumanoidStage10toHumanoidEPNS_8AssemblyE
#[doc(alias = "RBX::HumanoidStage::toHumanoid(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13HumanoidStage10toHumanoidEPNS_8AssemblyE")]
// IDA 0x735d20: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735d20() {
}

// 0x735de4 — __ZN3RBX13HumanoidStage12fromHumanoidEPNS_8AssemblyE
#[doc(alias = "RBX::HumanoidStage::fromHumanoid(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13HumanoidStage12fromHumanoidEPNS_8AssemblyE")]
// IDA 0x735de4: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735de4() {
}

// 0x735ea4 — __ZN3RBX13HumanoidStage15onAssemblyAddedEPNS_8AssemblyE
#[doc(alias = "RBX::HumanoidStage::onAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13HumanoidStage15onAssemblyAddedEPNS_8AssemblyE")]
// IDA 0x735ea4: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735ea4() {
}

// 0x735edc — __ZN3RBX13HumanoidStage18onAssemblyRemovingEPNS_8AssemblyE
#[doc(alias = "RBX::HumanoidStage::onAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13HumanoidStage18onAssemblyRemovingEPNS_8AssemblyE")]
// IDA 0x735edc: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735edc() {
}

// 0x735f10 — __ZNK3RBX13HumanoidStage12getStageTypeEv
#[doc(alias = "RBX::HumanoidStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX13HumanoidStage12getStageTypeEv")]
// IDA 0x735f10: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_735f10() {
}

// 0x735f14 — __GLOBAL__I_a_321
#[doc(alias = "global constructor keyed to_a_321")]
#[doc(alias = "__GLOBAL__I_a_321")]
// IDA 0x735f14: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_735f14() {
}

// 0x736044 — __ZN3RBX7IMovingC2Ev
#[doc(alias = "RBX::IMoving::IMoving(void)")]
#[doc(alias = "__ZN3RBX7IMovingC2Ev")]
// IDA 0x736044: 9 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736044() {
}

// 0x736060 — __ZN3RBX7IMoving10makeMovingEv
#[doc(alias = "RBX::IMoving::makeMoving(void)")]
#[doc(alias = "__ZN3RBX7IMoving10makeMovingEv")]
// IDA 0x736060: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736060() {
}

// 0x7360d8 — __ZN3RBX7IMoving16setMovingManagerEPNS_14IMovingManagerE
#[doc(alias = "RBX::IMoving::setMovingManager(RBX::IMovingManager *)")]
#[doc(alias = "__ZN3RBX7IMoving16setMovingManagerEPNS_14IMovingManagerE")]
// IDA 0x7360d8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7360d8() {
}

// 0x736108 — __ZN3RBX14IMovingManager6removeEPNS_7IMovingE
#[doc(alias = "RBX::IMovingManager::remove(RBX::IMoving *)")]
#[doc(alias = "__ZN3RBX14IMovingManager6removeEPNS_7IMovingE")]
// IDA 0x736108: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736108() {
}

// 0x73614c — __ZN3RBX7IMoving10checkSleepEv
#[doc(alias = "RBX::IMoving::checkSleep(void)")]
#[doc(alias = "__ZN3RBX7IMoving10checkSleepEv")]
// IDA 0x73614c: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73614c() {
}

// 0x736214 — __ZN3RBX7IMoving11notifyMovedEv
#[doc(alias = "RBX::IMoving::notifyMoved(void)")]
#[doc(alias = "__ZN3RBX7IMoving11notifyMovedEv")]
// IDA 0x736214: 22 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736214() {
}

// 0x736250 — __ZN3RBX7IMoving10forceSleepEv
#[doc(alias = "RBX::IMoving::forceSleep(void)")]
#[doc(alias = "__ZN3RBX7IMoving10forceSleepEv")]
// IDA 0x736250: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736250() {
}

// 0x736264 — __ZN3RBX14IMovingManagerC2Ev
#[doc(alias = "RBX::IMovingManager::IMovingManager(void)")]
#[doc(alias = "__ZN3RBX14IMovingManagerC2Ev")]
// IDA 0x736264: 14 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736264() {
}

// 0x73628c — __ZN3RBX14IMovingManagerD0Ev
#[doc(alias = "RBX::IMovingManager::~IMovingManager()")]
#[doc(alias = "__ZN3RBX14IMovingManagerD0Ev")]
// IDA 0x73628c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_73628c() {
}

// 0x73632c — __ZN3RBX14IMovingManagerD1Ev
#[doc(alias = "RBX::IMovingManager::~IMovingManager()")]
#[doc(alias = "__ZN3RBX14IMovingManagerD1Ev")]
// IDA 0x73632c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_73632c() {
}

// 0x736330 — __ZN3RBX14IMovingManagerD2Ev
#[doc(alias = "RBX::IMovingManager::~IMovingManager()")]
#[doc(alias = "__ZN3RBX14IMovingManagerD2Ev")]
// IDA 0x736330: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_736330() {
}

// 0x73649c — __ZN3RBX14IMovingManager17onMovingHeartbeatEv
#[doc(alias = "RBX::IMovingManager::onMovingHeartbeat(void)")]
#[doc(alias = "__ZN3RBX14IMovingManager17onMovingHeartbeatEv")]
// IDA 0x73649c: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73649c() {
}

// 0x736564 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_insert_unique(RBX::IMoving * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// IDA 0x736564: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736564() {
}

// 0x7365cc — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::IMoving * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// IDA 0x7365cc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7365cc() {
}

// 0x736624 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::erase(RBX::IMoving * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")]
// IDA 0x736624: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736624() {
}

// 0x73664c — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::equal_range(RBX::IMoving * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")]
// IDA 0x73664c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73664c() {
}

// 0x736698 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::erase(std::_Rb_tree_iterator<RBX::IMoving *>,std::_Rb_tree_iterator<RBX::IMoving *>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")]
// IDA 0x736698: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736698() {
}

// 0x7366f8 — __ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_erase(std::_Rb_tree_node<RBX::IMoving *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// IDA 0x7366f8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7366f8() {
}

// 0x736720 — __ZNK3RBX10IPipelined9getKernelEv
#[doc(alias = "RBX::IPipelined::getKernel(void)const")]
#[doc(alias = "__ZNK3RBX10IPipelined9getKernelEv")]
// IDA 0x736720: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736720() {
}

// 0x736788 — __ZNK3RBX10IPipelined8getStageENS_6IStage9StageTypeE
#[doc(alias = "RBX::IPipelined::getStage(RBX::IStage::StageType)const")]
#[doc(alias = "__ZNK3RBX10IPipelined8getStageENS_6IStage9StageTypeE")]
// IDA 0x736788: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736788() {
}

// 0x736808 — __ZN3RBX10IPipelined11putInKernelEPNS_6KernelE
#[doc(alias = "RBX::IPipelined::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX10IPipelined11putInKernelEPNS_6KernelE")]
// IDA 0x736808: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_736808() {
}

// 0x73680c — __ZN3RBX10IPipelined10putInStageEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::putInStage(RBX::IStage *)")]
#[doc(alias = "__ZN3RBX10IPipelined10putInStageEPNS_6IStageE")]
// IDA 0x73680c: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73680c() {
}

// 0x73693c — __ZN3RBX10IPipelined16removeFromKernelEv
#[doc(alias = "RBX::IPipelined::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX10IPipelined16removeFromKernelEv")]
// IDA 0x73693c: 57 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73693c() {
}

// 0x7369e8 — __ZN3RBX10IPipelined15removeFromStageEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::removeFromStage(RBX::IStage *)")]
#[doc(alias = "__ZN3RBX10IPipelined15removeFromStageEPNS_6IStageE")]
// IDA 0x7369e8: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7369e8() {
}

// 0x736b10 — __ZN3RBX10IPipelined13putInPipelineEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::putInPipeline(RBX::IStage *)")]
#[doc(alias = "__ZN3RBX10IPipelined13putInPipelineEPNS_6IStageE")]
// IDA 0x736b10: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736b10() {
}

// 0x736bb0 — __ZN3RBX10IPipelined18removeFromPipelineEPNS_6IStageE
#[doc(alias = "RBX::IPipelined::removeFromPipeline(RBX::IStage *)")]
#[doc(alias = "__ZN3RBX10IPipelined18removeFromPipelineEPNS_6IStageE")]
// IDA 0x736bb0: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736bb0() {
}

// 0x736c94 — __GLOBAL__I_a_322
#[doc(alias = "global constructor keyed to_a_322")]
#[doc(alias = "__GLOBAL__I_a_322")]
// IDA 0x736c94: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_736c94() {
}

// 0x736ccc — __ZN3RBX11IWorldStage11onEdgeAddedEPNS_4EdgeE
#[doc(alias = "RBX::IWorldStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX11IWorldStage11onEdgeAddedEPNS_4EdgeE")]
// IDA 0x736ccc: 37 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736ccc() {
}

// 0x736d38 — __ZN3RBX11IWorldStage14onEdgeRemovingEPNS_4EdgeE
#[doc(alias = "RBX::IWorldStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX11IWorldStage14onEdgeRemovingEPNS_4EdgeE")]
// IDA 0x736d38: 37 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736d38() {
}

// 0x736da4 — __ZN3RBX11IWorldStageD1Ev
#[doc(alias = "RBX::IWorldStage::~IWorldStage()")]
#[doc(alias = "__ZN3RBX11IWorldStageD1Ev")]
// IDA 0x736da4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_736da4() {
}

// 0x736dc8 — __ZN3RBX11IWorldStageD0Ev
#[doc(alias = "RBX::IWorldStage::~IWorldStage()")]
#[doc(alias = "__ZN3RBX11IWorldStageD0Ev")]
// IDA 0x736dc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_736dc8() {
}

// 0x736e80 — __GLOBAL__I_a_323
#[doc(alias = "global constructor keyed to_a_323")]
#[doc(alias = "__GLOBAL__I_a_323")]
// IDA 0x736e80: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_736e80() {
}

// 0x736f30 — __ZN3RBX10Reflection8EnumDescINS_5Joint9JointTypeEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Joint9JointTypeEEC1Ev")]
// IDA 0x736f30: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_736f30() {
}

// 0x736f34 — __ZN3RBX10Reflection8EnumDescINS_5Joint9JointTypeEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::EnumDesc(void)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Joint9JointTypeEEC2Ev")]
// IDA 0x736f34: 198 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_736f34() {
}

// 0x7374b8 — __ZN3RBX5JointC2Ev
#[doc(alias = "RBX::Joint::Joint(void)")]
#[doc(alias = "__ZN3RBX5JointC2Ev")]
// IDA 0x7374b8: 218 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7374b8() {
}

// 0x737730 — __ZN3RBX5JointD0Ev
#[doc(alias = "RBX::Joint::~Joint()")]
#[doc(alias = "__ZN3RBX5JointD0Ev")]
// IDA 0x737730: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_737730() {
}

// 0x7377d0 — __ZN3RBX5JointD1Ev
#[doc(alias = "RBX::Joint::~Joint()")]
#[doc(alias = "__ZN3RBX5JointD1Ev")]
// IDA 0x7377d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7377d0() {
}

// 0x7377d4 — __ZThn32_N3RBX5JointD0Ev
#[doc(alias = "non-virtual thunk toRBX::Joint::~Joint()")]
#[doc(alias = "__ZThn32_N3RBX5JointD0Ev")]
// IDA 0x7377d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7377d4() {
}

// 0x7377dc — __ZN3RBX5JointD2Ev
#[doc(alias = "RBX::Joint::~Joint()")]
#[doc(alias = "__ZN3RBX5JointD2Ev")]
// IDA 0x7377dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7377dc() {
}

// 0x737924 — __ZThn32_N3RBX5JointD1Ev
#[doc(alias = "non-virtual thunk toRBX::Joint::~Joint()")]
#[doc(alias = "__ZThn32_N3RBX5JointD1Ev")]
// IDA 0x737924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_737924() {
}

// 0x73792c — __ZN3RBX5Joint18getJointWorldCoordEi
#[doc(alias = "RBX::Joint::getJointWorldCoord(int)")]
#[doc(alias = "__ZN3RBX5Joint18getJointWorldCoordEi")]
// IDA 0x73792c: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73792c() {
}

// 0x7379c0 — __ZN3RBX5Joint11notifyMovedEv
#[doc(alias = "RBX::Joint::notifyMoved(void)")]
#[doc(alias = "__ZN3RBX5Joint11notifyMovedEv")]
// IDA 0x7379c0: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7379c0() {
}

// 0x7379ec — __ZN3RBX5Joint14findConstJointEPKNS_9PrimitiveENS0_9JointTypeE
#[doc(alias = "RBX::Joint::findConstJoint(RBX::Primitive const*,RBX::Joint::JointType)")]
#[doc(alias = "__ZN3RBX5Joint14findConstJointEPKNS_9PrimitiveENS0_9JointTypeE")]
// IDA 0x7379ec: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7379ec() {
}

// 0x737a24 — __ZN3RBX5Joint13getConstJointEPKNS_9PrimitiveENS0_9JointTypeE
#[doc(alias = "RBX::Joint::getConstJoint(RBX::Primitive const*,RBX::Joint::JointType)")]
#[doc(alias = "__ZN3RBX5Joint13getConstJointEPKNS_9PrimitiveENS0_9JointTypeE")]
// IDA 0x737a24: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737a24() {
}

// 0x737a80 — __ZN3RBX5Joint8getJointEPNS_9PrimitiveENS0_9JointTypeE
#[doc(alias = "RBX::Joint::getJoint(RBX::Primitive *,RBX::Joint::JointType)")]
#[doc(alias = "__ZN3RBX5Joint8getJointEPNS_9PrimitiveENS0_9JointTypeE")]
// IDA 0x737a80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_737a80() {
}

// 0x737a84 — __ZN3RBX5Joint13setJointOwnerEPNS_11IJointOwnerE
#[doc(alias = "RBX::Joint::setJointOwner(RBX::IJointOwner *)")]
#[doc(alias = "__ZN3RBX5Joint13setJointOwnerEPNS_11IJointOwnerE")]
// IDA 0x737a84: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737a84() {
}

// 0x737afc — __ZNK3RBX5Joint13getJointOwnerEv
#[doc(alias = "RBX::Joint::getJointOwner(void)const")]
#[doc(alias = "__ZNK3RBX5Joint13getJointOwnerEv")]
// IDA 0x737afc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737afc() {
}

// 0x737b00 — __ZN3RBX5Joint12setPrimitiveEiPNS_9PrimitiveE
#[doc(alias = "RBX::Joint::setPrimitive(int,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5Joint12setPrimitiveEiPNS_9PrimitiveE")]
// IDA 0x737b00: 55 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737b00() {
}

// 0x737c74 — __ZN3RBX5Joint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_ff
#[doc(alias = "RBX::Joint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId,float,float)")]
#[doc(alias = "__ZN3RBX5Joint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_ff")]
// IDA 0x737c74: 278 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737c74() {
}

// 0x737f80 — __ZN3RBX5Joint15FacesOverlappedEPKNS_9PrimitiveEmS3_mf
#[doc(alias = "RBX::Joint::FacesOverlapped(RBX::Primitive const*,unsigned long,RBX::Primitive const*,unsigned long,float)")]
#[doc(alias = "__ZN3RBX5Joint15FacesOverlappedEPKNS_9PrimitiveEmS3_mf")]
// IDA 0x737f80: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737f80() {
}

// 0x737fc8 — __ZN3RBX5Joint18canBuildJointTightEPNS_9PrimitiveES2_NS_8NormalIdES3_
#[doc(alias = "RBX::Joint::canBuildJointTight(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX5Joint18canBuildJointTightEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
// IDA 0x737fc8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737fc8() {
}

// 0x737fe8 — __ZN3RBX5Joint18canBuildJointLooseEPNS_9PrimitiveES2_NS_8NormalIdES3_
#[doc(alias = "RBX::Joint::canBuildJointLoose(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX5Joint18canBuildJointLooseEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
// IDA 0x737fe8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_737fe8() {
}

// 0x738008 — __ZN3RBX13getJointSize2EPKNS_5JointE
#[doc(alias = "RBX::getJointSize2(RBX::Joint const*)")]
#[doc(alias = "__ZN3RBX13getJointSize2EPKNS_5JointE")]
// IDA 0x738008: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_738008() {
}

// 0x738028 — __ZNK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE
#[doc(alias = "RBX::Joint::isHeavierThan(RBX::SpanningEdge const*)const")]
#[doc(alias = "__ZNK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE")]
// IDA 0x738028: 188 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_738028() {
}

// 0x73825c — __ZThn32_NK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE
#[doc(alias = "non-virtual thunk toRBX::Joint::isHeavierThan(RBX::SpanningEdge const*)const")]
#[doc(alias = "__ZThn32_NK3RBX5Joint13isHeavierThanEPKNS_12SpanningEdgeE")]
// IDA 0x73825c: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_73825c() {
}

// 0x738268 — __ZN3RBX5Joint9otherNodeEPNS_12SpanningNodeE
#[doc(alias = "RBX::Joint::otherNode(RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX5Joint9otherNodeEPNS_12SpanningNodeE")]
// IDA 0x738268: 13 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_738268() {
}

// 0x738284 — __ZThn32_N3RBX5Joint9otherNodeEPNS_12SpanningNodeE
#[doc(alias = "non-virtual thunk toRBX::Joint::otherNode(RBX::SpanningNode *)")]
#[doc(alias = "__ZThn32_N3RBX5Joint9otherNodeEPNS_12SpanningNodeE")]
// IDA 0x738284: 13 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_738284() {
}

// 0x88b7a4 — __ZThn32_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88b7a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b7a4() {
}

// 0x88b7ac — __ZThn32_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88b7ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b7ac() {
}

// 0x88b850 — __ZThn36_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x88b850: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b850() {
}

// 0x88b858 — __ZThn36_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_6ButtonELZNS_7sButtonEENS_14FactoryProductIS2_NS_8InstanceELZNS_7sButtonEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x88b858: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b858() {
}

// 0x88b8fc — __ZN3RBX10Reflection9EventDescINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Button,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Button::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
// IDA 0x88b8fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88b8fc() {
}

// 0x88b9b0 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Button,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Button::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// IDA 0x88b9b0: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88b9b0() {
}

// 0x88bbb4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Button,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Button::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
// IDA 0x88bbb4: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88bbb4() {
}

// 0x88bc28 — __ZNK3RBX10Reflection13EventDescBaseINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Button,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Button::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_6ButtonEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x88bc28: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88bc28() {
}

// 0x88bc3c — __ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Button,void ()(bool),1>::BoundFuncDesc(void (RBX::Button::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x88bc3c: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88bc3c() {
}

// 0x88bdb4 — __ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Button,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x88bdb4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88bdb4() {
}

// 0x88bde4 — __ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Button,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EED0Ev")]
// IDA 0x88bde4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88bde4() {
}

// 0x88beb8 — __ZNK3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Button,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_6ButtonEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x88beb8: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88beb8() {
}

// 0x88beec — __ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EEC2EMS2_FS6_SsSsSsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EEC2EMS2_FS6_SsSsSsEPKcSC_SC_SC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x88beec: 214 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88beec() {
}

// 0x88c104 — __ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EE16declareSignatureEPKcNS0_7VariantESA_SB_SA_SB_")]
// IDA 0x88c104: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88c104() {
}

// 0x88c16c — __ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EED0Ev")]
// IDA 0x88c16c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88c16c() {
}

// 0x88c248 — __ZNK3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Toolbar,boost::shared_ptr<RBX::Instance> ()(std::string,std::string,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_7ToolbarEFN5boost10shared_ptrINS_8InstanceEEESsSsSsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x88c248: 214 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88c248() {
}

// 0x88c4ac — __ZN3RBX10Reflection11Call3HelperINS_7ToolbarEMS2_FN5boost10shared_ptrINS_8InstanceEEESsSsSsESsSsSsS6_E4callEPS2_S8_RNS0_7VariantERKSsSE_SE_
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Toolbar,boost::shared_ptr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),std::string,std::string,std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Toolbar*,boost::shared_ptr<RBX::Instance> (RBX::Toolbar::*)(std::string,std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_7ToolbarEMS2_FN5boost10shared_ptrINS_8InstanceEEESsSsSsESsSsSsS6_E4callEPS2_S8_RNS0_7VariantERKSsSE_SE_")]
// IDA 0x88c4ac: 237 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88c4ac() {
}

// 0x88c748 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS2_FS6_SsEPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EEC2EMS2_FS6_SsEPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x88c748: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88c748() {
}

// 0x88c8c0 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x88c8c0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88c8c0() {
}

// 0x88c8f0 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED0Ev")]
// IDA 0x88c8f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88c8f0() {
}

// 0x88c9bc — __ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEESsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x88c9bc: 108 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88c9bc() {
}

// 0x88cafc — __ZN3RBX10Reflection11Call1HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEESsESsS6_E4callEPS2_S8_RNS0_7VariantERKSs
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Plugin,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),std::string,boost::shared_ptr<RBX::Instance>>::call(RBX::Plugin*,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEESsESsS6_E4callEPS2_S8_RNS0_7VariantERKSs")]
// IDA 0x88cafc: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88cafc() {
}

// 0x88cc7c — __ZN3RBX10Reflection9EventDescINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
// IDA 0x88cc7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88cc7c() {
}

// 0x88cd30 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// IDA 0x88cd30: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88cd30() {
}

// 0x88cf34 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
// IDA 0x88cf34: 38 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88cf34() {
}

// 0x88cfa8 — __ZNK3RBX10Reflection13EventDescBaseINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Plugin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Plugin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_6PluginEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
// IDA 0x88cfa8: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88cfa8() {
}

// 0x88cfbc — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::BoundFuncDesc(void (RBX::Plugin::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x88cfbc: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88cfbc() {
}

// 0x88d134 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
// IDA 0x88d134: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d134() {
}

// 0x88d164 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EED0Ev")]
// IDA 0x88d164: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88d164() {
}

// 0x88d238 — __ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFvbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x88d238: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d238() {
}

// 0x88d26c — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x88d26c: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d26c() {
}

// 0x88d370 — __ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev")]
// IDA 0x88d370: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88d370() {
}

// 0x88d424 — __ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Plugin,boost::shared_ptr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_6PluginEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x88d424: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d424() {
}

// 0x88d448 — __ZN3RBX10Reflection11Call0HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Plugin,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),boost::shared_ptr<RBX::Instance>>::call(RBX::Plugin*,boost::shared_ptr<RBX::Instance> (RBX::Plugin::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_6PluginEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE")]
// IDA 0x88d448: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d448() {
}

// 0x88d530 — __ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC2EMS2_FS6_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// IDA 0x88d530: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d530() {
}

// 0x88d634 — __ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED0Ev")]
// IDA 0x88d634: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88d634() {
}

// 0x88d6e8 — __ZNK3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PluginManager,boost::shared_ptr<RBX::Instance> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13PluginManagerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
// IDA 0x88d6e8: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d6e8() {
}

// 0x88d70c — __ZN3RBX10Reflection11Call0HelperINS_13PluginManagerEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::PluginManager,boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),boost::shared_ptr<RBX::Instance>>::call(RBX::PluginManager*,boost::shared_ptr<RBX::Instance> (RBX::PluginManager::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_13PluginManagerEMS2_FN5boost10shared_ptrINS_8InstanceEEEvES6_E4callEPS2_S8_RNS0_7VariantE")]
// IDA 0x88d70c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d70c() {
}

// 0x88d7f4 — __ZN3RBX13PluginManagerD2Ev
#[doc(alias = "RBX::PluginManager::~PluginManager()")]
#[doc(alias = "__ZN3RBX13PluginManagerD2Ev")]
// IDA 0x88d7f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_88d7f4() {
}

// 0x88d984 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13PluginManagerEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::PluginManager> RBX::Creatable<RBX::Instance>::create<RBX::PluginManager>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_13PluginManagerEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x88d984: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88d984() {
}
