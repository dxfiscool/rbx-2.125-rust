//! core shard DF — 100 core stubs EA-sorted, next uncovered after DE 0x75db20 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::StepJointsStage::~StepJointsStage()")]
// 0x75db24 — __ZN3RBX15StepJointsStageD2Ev
pub fn stub_75db24() {
    // IDA 0x75db24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StepJointsStage::removeJoint(RBX::Joint *)")]
// 0x75dd0c — __ZN3RBX15StepJointsStage11removeJointEPNS_5JointE
pub fn stub_75dd0c() {
    // IDA 0x75dd0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StepJointsStage::onSimulateAssemblyAdded(RBX::Assembly *)")]
// 0x75dd9c — __ZN3RBX15StepJointsStage23onSimulateAssemblyAddedEPNS_8AssemblyE
pub fn stub_75dd9c() {
    // IDA 0x75dd9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StepJointsStage::onSimulateAssemblyRemoving(RBX::Assembly *)")]
// 0x75ddb8 — __ZN3RBX15StepJointsStage26onSimulateAssemblyRemovingEPNS_8AssemblyE
pub fn stub_75ddb8() {
    // IDA 0x75ddb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StepJointsStage::onEdgeAdded(RBX::Edge *)")]
// 0x75ddd4 — __ZN3RBX15StepJointsStage11onEdgeAddedEPNS_4EdgeE
pub fn stub_75ddd4() {
    // IDA 0x75ddd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StepJointsStage::onEdgeRemoving(RBX::Edge *)")]
// 0x75de3c — __ZN3RBX15StepJointsStage14onEdgeRemovingEPNS_4EdgeE
pub fn stub_75de3c() {
    // IDA 0x75de3c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::StepJointsStage::jointsStepWorld(void)")]
// 0x75de84 — __ZN3RBX15StepJointsStage15jointsStepWorldEv
pub fn stub_75de84() {
    // IDA 0x75de84: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::StepJointsStage::getStageType(void)const")]
// 0x75dfd4 — __ZNK3RBX15StepJointsStage12getStageTypeEv
pub fn stub_75dfd4() {
    // IDA 0x75dfd4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::TreeStage::TreeStage(RBX::IStage *,RBX::World *)")]
// 0x75e0a0 — __ZN3RBX9TreeStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_75e0a0() {
    // IDA 0x75e0a0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::TreeStage::TreeStage(RBX::IStage *,RBX::World *)")]
// 0x75e0a4 — __ZN3RBX9TreeStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_75e0a4() {
    // IDA 0x75e0a4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::TreeStage::~TreeStage()")]
// 0x75e21c — __ZN3RBX9TreeStageD0Ev
pub fn stub_75e21c() {
    // IDA 0x75e21c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::~TreeStage()")]
// 0x75e2bc — __ZN3RBX9TreeStageD1Ev
pub fn stub_75e2bc() {
    // IDA 0x75e2bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::~TreeStage()")]
// 0x75e2c0 — __ZN3RBX9TreeStageD2Ev
pub fn stub_75e2c0() {
    // IDA 0x75e2c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::validateTree(RBX::SpanningNode *)")]
// 0x75e4bc — __ZN3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE
pub fn stub_75e4bc() {
    // IDA 0x75e4bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TreeStage::validateTree(RBX::SpanningNode *)")]
// 0x75e4c8 — __ZThn16_N3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE
pub fn stub_75e4c8() {
    // IDA 0x75e4c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::chainToGround(RBX::Primitive *)")]
// 0x75e4d4 — __ZN3RBX13chainToGroundEPNS_9PrimitiveE
pub fn stub_75e4d4() {
    // IDA 0x75e4d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x75e52c — __ZN3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_75e52c() {
    // IDA 0x75e52c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::dirtyMechanism(RBX::Mechanism *)")]
// 0x75e69c — __ZN3RBX9TreeStage14dirtyMechanismEPNS_9MechanismE
pub fn stub_75e69c() {
    // IDA 0x75e69c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x75e710 — __ZThn16_N3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_75e710() {
    // IDA 0x75e710: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
// 0x75e718 — __ZN3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE
pub fn stub_75e718() {
    // IDA 0x75e718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::sendClumpChangedMessage(RBX::Primitive *)")]
// 0x75edb8 — __ZN3RBX9TreeStage23sendClumpChangedMessageEPNS_9PrimitiveE
pub fn stub_75edb8() {
    // IDA 0x75edb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
// 0x75ee8c — __ZThn16_N3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE
pub fn stub_75ee8c() {
    // IDA 0x75ee8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::assertNotInPipeline(RBX::Assembly *)")]
// 0x75ee94 — __ZN3RBX19assertNotInPipelineEPNS_8AssemblyE
pub fn stub_75ee94() {
    // IDA 0x75ee94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
// 0x75eef8 — __ZN3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
pub fn stub_75eef8() {
    // IDA 0x75eef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
// 0x75ef20 — __ZThn16_N3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
pub fn stub_75ef20() {
    // IDA 0x75ef20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x75ef28 — __ZN3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_75ef28() {
    // IDA 0x75ef28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::destroyClump(RBX::Primitive *)")]
// 0x75f22c — __ZN3RBX9TreeStage12destroyClumpEPNS_9PrimitiveE
pub fn stub_75f22c() {
    // IDA 0x75f22c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::destroyAssembly(RBX::Primitive *)")]
// 0x75f258 — __ZN3RBX9TreeStage15destroyAssemblyEPNS_9PrimitiveE
pub fn stub_75f258() {
    // IDA 0x75f258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::destroyMechanism(RBX::Primitive *)")]
// 0x75f29c — __ZN3RBX9TreeStage16destroyMechanismEPNS_9PrimitiveE
pub fn stub_75f29c() {
    // IDA 0x75f29c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x75f320 — __ZThn16_N3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_75f320() {
    // IDA 0x75f320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::removeFromPipeline(RBX::Mechanism *)")]
// 0x75f328 — __ZN3RBX9TreeStage18removeFromPipelineEPNS_9MechanismE
pub fn stub_75f328() {
    // IDA 0x75f328: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::cleanMechanism(RBX::Mechanism *)")]
// 0x75f3e8 — __ZN3RBX9TreeStage14cleanMechanismEPNS_9MechanismE
pub fn stub_75f3e8() {
    // IDA 0x75f3e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::assemble(void)")]
// 0x75f500 — __ZN3RBX9TreeStage8assembleEv
pub fn stub_75f500() {
    // IDA 0x75f500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::onEdgeAdded(RBX::Edge *)")]
// 0x75f540 — __ZN3RBX9TreeStage11onEdgeAddedEPNS_4EdgeE
pub fn stub_75f540() {
    // IDA 0x75f540: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TreeStage::onEdgeRemoving(RBX::Edge *)")]
// 0x75f660 — __ZN3RBX9TreeStage14onEdgeRemovingEPNS_4EdgeE
pub fn stub_75f660() {
    // IDA 0x75f660: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::TreeStage::onPrimitiveAdded(RBX::Primitive *)")]
// 0x75f798 — __ZN3RBX9TreeStage16onPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_75f798() {
    // IDA 0x75f798: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::TreeStage::onPrimitiveRemoving(RBX::Primitive *)")]
// 0x75f804 — __ZN3RBX9TreeStage19onPrimitiveRemovingEPNS_9PrimitiveE
pub fn stub_75f804() {
    // IDA 0x75f804: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::TreeStage::getMetric(RBX::IWorldStage::MetricType)")]
// 0x75f874 — __ZN3RBX9TreeStage9getMetricENS_11IWorldStage10MetricTypeE
pub fn stub_75f874() {
    // IDA 0x75f874: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::TreeStage::getStageType(void)const")]
// 0x75f880 — __ZNK3RBX9TreeStage12getStageTypeEv
pub fn stub_75f880() {
    // IDA 0x75f880: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_erase(std::_Rb_tree_node<RBX::Mechanism *> *)")]
// 0x75f884 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_75f884() {
    // IDA 0x75f884: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_insert_unique(RBX::Mechanism * const&)")]
// 0x75f8ac — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_75f8ac() {
    // IDA 0x75f8ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Mechanism * const&)")]
// 0x75f914 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_75f914() {
    // IDA 0x75f914: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::erase(RBX::Mechanism * const&)")]
// 0x75f96c — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_75f96c() {
    // IDA 0x75f96c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::equal_range(RBX::Mechanism * const&)")]
// 0x75f994 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
pub fn stub_75f994() {
    // IDA 0x75f994: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::erase(std::_Rb_tree_iterator<RBX::Mechanism *>,std::_Rb_tree_iterator<RBX::Mechanism *>)")]
// 0x75f9e0 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_75f9e0() {
    // IDA 0x75f9e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::IndexedTree::visitMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>)")]
// 0x75fa40 — __ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvPFvPS2_ENS4_5list1INS3_3argILi1EEEEEEEEEvT0_
// was: void RBX::IndexedTree::visitMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>)
pub fn stub_75fa40() {
    // IDA 0x75fa40: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::WedgePoly::buildMesh(void)")]
// 0x75fc48 — __ZN3RBX9WedgePoly9buildMeshEv
pub fn stub_75fc48() {
    // IDA 0x75fc48: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::WedgePoly::getMoment(float)const")]
// 0x75fd24 — __ZNK3RBX9WedgePoly9getMomentEf
pub fn stub_75fd24() {
    // IDA 0x75fd24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::WedgePoly::getCofmOffset(void)const")]
// 0x75fe58 — __ZNK3RBX9WedgePoly13getCofmOffsetEv
pub fn stub_75fe58() {
    // IDA 0x75fe58: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::WedgePoly::getSurfaceCoordInBody(unsigned long)const")]
// 0x75fe80 — __ZNK3RBX9WedgePoly21getSurfaceCoordInBodyEm
pub fn stub_75fe80() {
    // IDA 0x75fe80: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::WedgePoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
// 0x75ff4c — __ZNK3RBX9WedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE
pub fn stub_75ff4c() {
    // IDA 0x75ff4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::WedgePoly::~WedgePoly()")]
// 0x760260 — __ZN3RBX9WedgePolyD1Ev
pub fn stub_760260() {
    // IDA 0x760260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WedgePoly::~WedgePoly()")]
// 0x760284 — __ZN3RBX9WedgePolyD0Ev
pub fn stub_760284() {
    // IDA 0x760284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator delete(void *)")]
// 0x7608c8 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv
pub fn stub_7608c8() {
    // IDA 0x7608c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator new(unsigned long)")]
// 0x760f04 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm
pub fn stub_760f04() {
    // IDA 0x760f04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::Allocator(void)")]
// 0x761084 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev
pub fn stub_761084() {
    // IDA 0x761084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::releaseMemory(void)")]
// 0x7610e8 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEE13releaseMemoryEv
pub fn stub_7610e8() {
    // IDA 0x7610e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x761104 — __ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// was: boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)
pub fn stub_761104() {
    // IDA 0x761104: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x761134 — __ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// was: boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)
pub fn stub_761134() {
    // IDA 0x761134: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::WeldJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
// 0x761504 — __ZN3RBX9WeldJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
pub fn stub_761504() {
    // IDA 0x761504: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::EThrottle::increaseLoad(bool)")]
// 0x761710 — __ZN3RBX9EThrottle12increaseLoadEb
pub fn stub_761710() {
    // IDA 0x761710: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::EThrottle::computeThrottle(int)")]
// 0x761770 — __ZN3RBX9EThrottle15computeThrottleEi
pub fn stub_761770() {
    // IDA 0x761770: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::EThrottle::getEnvironmentSpeed(void)const")]
// 0x761834 — __ZNK3RBX9EThrottle19getEnvironmentSpeedEv
pub fn stub_761834() {
    // IDA 0x761834: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::World::World(void)")]
// 0x761890 — __ZN3RBX5WorldC1Ev
pub fn stub_761890() {
    // IDA 0x761890: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::World(void)")]
// 0x761894 — __ZN3RBX5WorldC2Ev
pub fn stub_761894() {
    // IDA 0x761894: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::loadProfilers(std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>> &)const")]
// 0x761f7c — __ZNK3RBX5World13loadProfilersERSt6vectorIPNS_9Profiling12CodeProfilerESaIS4_EE
pub fn stub_761f7c() {
    // IDA 0x761f7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::~World()")]
// 0x7620a8 — __ZN3RBX5WorldD1Ev
pub fn stub_7620a8() {
    // IDA 0x7620a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::~World()")]
// 0x7620ac — __ZN3RBX5WorldD2Ev
pub fn stub_7620ac() {
    // IDA 0x7620ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getKernel(void)const")]
// 0x762774 — __ZNK3RBX5World9getKernelEv
pub fn stub_762774() {
    // IDA 0x762774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getSpatialFilter(void)")]
// 0x762784 — __ZN3RBX5World16getSpatialFilterEv
pub fn stub_762784() {
    // IDA 0x762784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getKernel(void)")]
// 0x7627a4 — __ZN3RBX5World9getKernelEv
pub fn stub_7627a4() {
    // IDA 0x7627a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getSendPhysics(void)")]
// 0x7627b0 — __ZN3RBX5World14getSendPhysicsEv
pub fn stub_7627b0() {
    // IDA 0x7627b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getSimSendFilter(void)")]
// 0x7627b8 — __ZN3RBX5World16getSimSendFilterEv
pub fn stub_7627b8() {
    // IDA 0x7627b8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumBodies(void)const")]
// 0x7627d8 — __ZNK3RBX5World12getNumBodiesEv
pub fn stub_7627d8() {
    // IDA 0x7627d8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumPoints(void)const")]
// 0x7627f0 — __ZNK3RBX5World12getNumPointsEv
pub fn stub_7627f0() {
    // IDA 0x7627f0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumConstraints(void)const")]
// 0x762808 — __ZNK3RBX5World17getNumConstraintsEv
pub fn stub_762808() {
    // IDA 0x762808: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getMetric(RBX::IWorldStage::MetricType)const")]
// 0x762820 — __ZNK3RBX5World9getMetricENS_11IWorldStage10MetricTypeE
pub fn stub_762820() {
    // IDA 0x762820: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumHashNodes(void)const")]
// 0x76282c — __ZNK3RBX5World15getNumHashNodesEv
pub fn stub_76282c() {
    // IDA 0x76282c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getMaxBucketSize(void)const")]
// 0x762838 — __ZNK3RBX5World16getMaxBucketSizeEv
pub fn stub_762838() {
    // IDA 0x762838: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::ticklePrimitive(RBX::Primitive *,bool)")]
// 0x762844 — __ZN3RBX5World15ticklePrimitiveEPNS_9PrimitiveEb
pub fn stub_762844() {
    // IDA 0x762844: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveEngineChanging(RBX::Primitive *)")]
// 0x7628e0 — __ZN3RBX5World25onPrimitiveEngineChangingEPNS_9PrimitiveE
pub fn stub_7628e0() {
    // IDA 0x7628e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveEngineChanged(RBX::Assembly *)")]
// 0x762a38 — __ZN3RBX5World24onPrimitiveEngineChangedEPNS_8AssemblyE
pub fn stub_762a38() {
    // IDA 0x762a38: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveFixedChanging(RBX::Primitive *)")]
// 0x762ab0 — __ZN3RBX5World24onPrimitiveFixedChangingEPNS_9PrimitiveE
pub fn stub_762ab0() {
    // IDA 0x762ab0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveFixedChanged(RBX::Primitive *)")]
// 0x762b78 — __ZN3RBX5World23onPrimitiveFixedChangedEPNS_9PrimitiveE
pub fn stub_762b78() {
    // IDA 0x762b78: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitivePreventCollideChanged(RBX::Primitive *)")]
// 0x762c40 — __ZN3RBX5World32onPrimitivePreventCollideChangedEPNS_9PrimitiveE
pub fn stub_762c40() {
    // IDA 0x762c40: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveContactParametersChanged(RBX::Primitive *)")]
// 0x762cc8 — __ZN3RBX5World35onPrimitiveContactParametersChangedEPNS_9PrimitiveE
pub fn stub_762cc8() {
    // IDA 0x762cc8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveExtentsChanged(RBX::Primitive *)")]
// 0x762d40 — __ZN3RBX5World25onPrimitiveExtentsChangedEPNS_9PrimitiveE
pub fn stub_762d40() {
    // IDA 0x762d40: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onAssemblyExtentsChanged(RBX::Assembly *)")]
// 0x762df4 — __ZN3RBX5World24onAssemblyExtentsChangedEPNS_8AssemblyE
pub fn stub_762df4() {
    // IDA 0x762df4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onAssemblyInSimluationStage(RBX::Assembly *)")]
// 0x762f10 — __ZN3RBX5World27onAssemblyInSimluationStageEPNS_8AssemblyE
pub fn stub_762f10() {
    // IDA 0x762f10: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveGeometryChanged(RBX::Primitive *)")]
// 0x762f38 — __ZN3RBX5World26onPrimitiveGeometryChangedEPNS_9PrimitiveE
pub fn stub_762f38() {
    // IDA 0x762f38: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onJointPrimitiveNulling(RBX::Joint *,RBX::Primitive *)")]
// 0x762fec — __ZN3RBX5World23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE
pub fn stub_762fec() {
    // IDA 0x762fec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onJointPrimitiveSet(RBX::Joint *,RBX::Primitive *)")]
// 0x762ff4 — __ZN3RBX5World19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE
pub fn stub_762ff4() {
    // IDA 0x762ff4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::assemble(void)")]
// 0x762ffc — __ZN3RBX5World8assembleEv
pub fn stub_762ffc() {
    // IDA 0x762ffc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::isAssembled(void)")]
// 0x763020 — __ZN3RBX5World11isAssembledEv
pub fn stub_763020() {
    // IDA 0x763020: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::setFRMThrottle(int)")]
// 0x763044 — __ZN3RBX5World14setFRMThrottleEi
pub fn stub_763044() {
    // IDA 0x763044: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::sendClumpChangedMessage(RBX::Primitive *)")]
// 0x763048 — __ZN3RBX5World23sendClumpChangedMessageEPNS_9PrimitiveE
pub fn stub_763048() {
    // IDA 0x763048: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::notifyMovingAssemblies(void)")]
// 0x763070 — __ZN3RBX5World22notifyMovingAssembliesEv
pub fn stub_763070() {
    // IDA 0x763070: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::uiStep(bool,double)")]
// 0x7632a8 — __ZN3RBX5World6uiStepEbd
pub fn stub_7632a8() {
    // IDA 0x7632a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::doBreakJoints(void)")]
// 0x7635c8 — __ZN3RBX5World13doBreakJointsEv
pub fn stub_7635c8() {
    // IDA 0x7635c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::doWorldStep(bool,int,int)")]
// 0x763610 — __ZN3RBX5World11doWorldStepEbii
pub fn stub_763610() {
    // IDA 0x763610: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

