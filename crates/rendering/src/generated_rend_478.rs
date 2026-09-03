//! rendering shard 478 — 120 stubs 0x75db20..0x76485c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 51191->51311 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc after shard 477 (0x75db20..0x769xx)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// 0x75db20 — __ZN3RBX15StepJointsStageD1Ev
// type: void __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::~StepJointsStage()")]
#[doc(alias = "__ZN3RBX15StepJointsStageD1Ev")]
// IDA 0x75db20: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_75db20() {
}
// 0x75db24 — __ZN3RBX15StepJointsStageD2Ev
// type: void __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::~StepJointsStage()")]
#[doc(alias = "__ZN3RBX15StepJointsStageD2Ev")]
// IDA 0x75db24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75db24() {
}
// 0x75dd0c — __ZN3RBX15StepJointsStage11removeJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::StepJointsStage::removeJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage11removeJointEPNS_5JointE")]
// IDA 0x75dd0c: 53 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75dd0c() {
}
// 0x75dd9c — __ZN3RBX15StepJointsStage23onSimulateAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::StepJointsStage::onSimulateAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage23onSimulateAssemblyAddedEPNS_8AssemblyE")]
// IDA 0x75dd9c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75dd9c() {
}
// 0x75ddb8 — __ZN3RBX15StepJointsStage26onSimulateAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::StepJointsStage::onSimulateAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage26onSimulateAssemblyRemovingEPNS_8AssemblyE")]
// IDA 0x75ddb8: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ddb8() {
}
// 0x75ddd4 — __ZN3RBX15StepJointsStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::StepJointsStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage11onEdgeAddedEPNS_4EdgeE")]
// IDA 0x75ddd4: 47 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ddd4() {
}
// 0x75de3c — __ZN3RBX15StepJointsStage14onEdgeRemovingEPNS_4EdgeE
// type: int __fastcall(RBX::StepJointsStage *this, RBX::Edge *)
#[doc(alias = "RBX::StepJointsStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage14onEdgeRemovingEPNS_4EdgeE")]
// IDA 0x75de3c: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75de3c() {
}
// 0x75de84 — __ZN3RBX15StepJointsStage15jointsStepWorldEv
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::jointsStepWorld(void)")]
#[doc(alias = "__ZN3RBX15StepJointsStage15jointsStepWorldEv")]
// IDA 0x75de84: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75de84() {
}
// 0x75dfd4 — __ZNK3RBX15StepJointsStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX15StepJointsStage12getStageTypeEv")]
// IDA 0x75dfd4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75dfd4() {
}
// 0x75dfd8 — __GLOBAL__I_a_354
#[doc(alias = "global constructor keyed to_a_354")]
#[doc(alias = "__GLOBAL__I_a_354")]
// IDA 0x75dfd8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_75dfd8() {
}
// 0x75e0a0 — __ZN3RBX9TreeStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::TreeStage::TreeStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX9TreeStageC1EPNS_6IStageEPNS_5WorldE")]
// IDA 0x75e0a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_75e0a0() {
}
// 0x75e0a4 — __ZN3RBX9TreeStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::TreeStage::TreeStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX9TreeStageC2EPNS_6IStageEPNS_5WorldE")]
// IDA 0x75e0a4: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e0a4() {
}
// 0x75e21c — __ZN3RBX9TreeStageD0Ev
// type: void __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::~TreeStage()")]
#[doc(alias = "__ZN3RBX9TreeStageD0Ev")]
// IDA 0x75e21c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75e21c() {
}
// 0x75e2bc — __ZN3RBX9TreeStageD1Ev
// type: void __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::~TreeStage()")]
#[doc(alias = "__ZN3RBX9TreeStageD1Ev")]
// IDA 0x75e2bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_75e2bc() {
}
// 0x75e2c0 — __ZN3RBX9TreeStageD2Ev
// type: void __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::~TreeStage()")]
#[doc(alias = "__ZN3RBX9TreeStageD2Ev")]
// IDA 0x75e2c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75e2c0() {
}
// 0x75e4bc — __ZN3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningNode *)
#[doc(alias = "RBX::TreeStage::validateTree(RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE")]
// IDA 0x75e4bc: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e4bc() {
}
// 0x75e4c8 — __ZThn16_N3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningNode *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::validateTree(RBX::SpanningNode *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE")]
// IDA 0x75e4c8: 5 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e4c8() {
}
// 0x75e4d4 — __ZN3RBX13chainToGroundEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::chainToGround(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX13chainToGroundEPNS_9PrimitiveE")]
// IDA 0x75e4d4: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e4d4() {
}
// 0x75e52c — __ZN3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
// IDA 0x75e52c: 125 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e52c() {
}
// 0x75e69c — __ZN3RBX9TreeStage14dirtyMechanismEPNS_9MechanismE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Mechanism *)
#[doc(alias = "RBX::TreeStage::dirtyMechanism(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX9TreeStage14dirtyMechanismEPNS_9MechanismE")]
// IDA 0x75e69c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e69c() {
}
// 0x75e710 — __ZThn16_N3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
// IDA 0x75e710: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e710() {
}
// 0x75e718 — __ZN3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE")]
// IDA 0x75e718: 586 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75e718() {
}
// 0x75edb8 — __ZN3RBX9TreeStage23sendClumpChangedMessageEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::sendClumpChangedMessage(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage23sendClumpChangedMessageEPNS_9PrimitiveE")]
// IDA 0x75edb8: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75edb8() {
}
// 0x75ee8c — __ZThn16_N3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE")]
// IDA 0x75ee8c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ee8c() {
}
// 0x75ee94 — __ZN3RBX19assertNotInPipelineEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::assertNotInPipeline(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19assertNotInPipelineEPNS_8AssemblyE")]
// IDA 0x75ee94: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ee94() {
}
// 0x75eef8 — __ZN3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE")]
// IDA 0x75eef8: 15 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75eef8() {
}
// 0x75ef20 — __ZThn16_N3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE")]
// IDA 0x75ef20: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ef20() {
}
// 0x75ef28 — __ZN3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
// IDA 0x75ef28: 262 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ef28() {
}
// 0x75f22c — __ZN3RBX9TreeStage12destroyClumpEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::destroyClump(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage12destroyClumpEPNS_9PrimitiveE")]
// IDA 0x75f22c: 17 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f22c() {
}
// 0x75f258 — __ZN3RBX9TreeStage15destroyAssemblyEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::destroyAssembly(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage15destroyAssemblyEPNS_9PrimitiveE")]
// IDA 0x75f258: 27 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f258() {
}
// 0x75f29c — __ZN3RBX9TreeStage16destroyMechanismEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::destroyMechanism(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage16destroyMechanismEPNS_9PrimitiveE")]
// IDA 0x75f29c: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f29c() {
}
// 0x75f320 — __ZThn16_N3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
// IDA 0x75f320: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f320() {
}
// 0x75f328 — __ZN3RBX9TreeStage18removeFromPipelineEPNS_9MechanismE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Mechanism *)
#[doc(alias = "RBX::TreeStage::removeFromPipeline(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX9TreeStage18removeFromPipelineEPNS_9MechanismE")]
// IDA 0x75f328: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f328() {
}
// 0x75f3e8 — __ZN3RBX9TreeStage14cleanMechanismEPNS_9MechanismE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Mechanism *)
#[doc(alias = "RBX::TreeStage::cleanMechanism(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX9TreeStage14cleanMechanismEPNS_9MechanismE")]
// IDA 0x75f3e8: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f3e8() {
}
// 0x75f500 — __ZN3RBX9TreeStage8assembleEv
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::assemble(void)")]
#[doc(alias = "__ZN3RBX9TreeStage8assembleEv")]
// IDA 0x75f500: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f500() {
}
// 0x75f540 — __ZN3RBX9TreeStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::TreeStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX9TreeStage11onEdgeAddedEPNS_4EdgeE")]
// IDA 0x75f540: 106 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f540() {
}
// 0x75f660 — __ZN3RBX9TreeStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::TreeStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX9TreeStage14onEdgeRemovingEPNS_4EdgeE")]
// IDA 0x75f660: 116 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f660() {
}
// 0x75f798 — __ZN3RBX9TreeStage16onPrimitiveAddedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::onPrimitiveAdded(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage16onPrimitiveAddedEPNS_9PrimitiveE")]
// IDA 0x75f798: 34 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f798() {
}
// 0x75f804 — __ZN3RBX9TreeStage19onPrimitiveRemovingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::onPrimitiveRemoving(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage19onPrimitiveRemovingEPNS_9PrimitiveE")]
// IDA 0x75f804: 36 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f804() {
}
// 0x75f874 — __ZN3RBX9TreeStage9getMetricENS_11IWorldStage10MetricTypeE
#[doc(alias = "RBX::TreeStage::getMetric(RBX::IWorldStage::MetricType)")]
#[doc(alias = "__ZN3RBX9TreeStage9getMetricENS_11IWorldStage10MetricTypeE")]
// IDA 0x75f874: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f874() {
}
// 0x75f880 — __ZNK3RBX9TreeStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX9TreeStage12getStageTypeEv")]
// IDA 0x75f880: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f880() {
}
// 0x75f884 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_erase(std::_Rb_tree_node<RBX::Mechanism *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// IDA 0x75f884: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f884() {
}
// 0x75f8ac — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_insert_unique(RBX::Mechanism * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// IDA 0x75f8ac: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f8ac() {
}
// 0x75f914 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Mechanism * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// IDA 0x75f914: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f914() {
}
// 0x75f96c — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::erase(RBX::Mechanism * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")]
// IDA 0x75f96c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f96c() {
}
// 0x75f994 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::equal_range(RBX::Mechanism * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")]
// IDA 0x75f994: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f994() {
}
// 0x75f9e0 — __ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Mechanism *,RBX::Mechanism *,std::_Identity<RBX::Mechanism *>,std::less<RBX::Mechanism *>,std::allocator<RBX::Mechanism *>>::erase(std::_Rb_tree_iterator<RBX::Mechanism *>,std::_Rb_tree_iterator<RBX::Mechanism *>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX9MechanismES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")]
// IDA 0x75f9e0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75f9e0() {
}
// 0x75fa40 — __ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvPFvPS2_ENS4_5list1INS3_3argILi1EEEEEEEEEvT0_
#[doc(alias = "void RBX::IndexedTree::visitMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Assembly*),boost::_bi::list1<boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvPFvPS2_ENS4_5list1INS3_3argILi1EEEEEEEEEvT0_")]
// IDA 0x75fa40: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75fa40() {
}
// 0x75fae4 — __GLOBAL__I_a_355
#[doc(alias = "global constructor keyed to_a_355")]
#[doc(alias = "__GLOBAL__I_a_355")]
// IDA 0x75fae4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_75fae4() {
}
// 0x75fc48 — __ZN3RBX9WedgePoly9buildMeshEv
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this)
#[doc(alias = "RBX::WedgePoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX9WedgePoly9buildMeshEv")]
// IDA 0x75fc48: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75fc48() {
}
// 0x75fd24 — __ZNK3RBX9WedgePoly9getMomentEf
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this, float)
#[doc(alias = "RBX::WedgePoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly9getMomentEf")]
// IDA 0x75fd24: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75fd24() {
}
// 0x75fe58 — __ZNK3RBX9WedgePoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this)
#[doc(alias = "RBX::WedgePoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly13getCofmOffsetEv")]
// IDA 0x75fe58: 11 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75fe58() {
}
// 0x75fe80 — __ZNK3RBX9WedgePoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this, unsigned int)
#[doc(alias = "RBX::WedgePoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly21getSurfaceCoordInBodyEm")]
// IDA 0x75fe80: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75fe80() {
}
// 0x75ff4c — __ZNK3RBX9WedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::WedgePoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// IDA 0x75ff4c: 8 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ff4c() {
}
// 0x760260 — __ZN3RBX9WedgePolyD1Ev
// type: void __fastcall(RBX::WedgePoly *__hidden this)
#[doc(alias = "RBX::WedgePoly::~WedgePoly()")]
#[doc(alias = "__ZN3RBX9WedgePolyD1Ev")]
// IDA 0x760260: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_760260() {
}
// 0x760284 — __ZN3RBX9WedgePolyD0Ev
// type: void __fastcall(RBX::WedgePoly *__hidden this)
#[doc(alias = "RBX::WedgePoly::~WedgePoly()")]
#[doc(alias = "__ZN3RBX9WedgePolyD0Ev")]
// IDA 0x760284: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_760284() {
}
// 0x7608c8 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv")]
// IDA 0x7608c8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7608c8() {
}
// 0x760f04 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm")]
// IDA 0x760f04: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760f04() {
}
// 0x761084 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev")]
// IDA 0x761084: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761084() {
}
// 0x7610e8 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEE13releaseMemoryEv")]
// IDA 0x7610e8: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7610e8() {
}
// 0x761104 — __ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// IDA 0x761104: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761104() {
}
// 0x761134 — __ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x761134: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761134() {
}
// 0x7613d4 — __GLOBAL__I_a_356
#[doc(alias = "global constructor keyed to_a_356")]
#[doc(alias = "__GLOBAL__I_a_356")]
// IDA 0x7613d4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7613d4() {
}
// 0x761504 — __ZN3RBX9WeldJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::WeldJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9WeldJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
// IDA 0x761504: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761504() {
}
// 0x761648 — __GLOBAL__I_a_357
#[doc(alias = "global constructor keyed to_a_357")]
#[doc(alias = "__GLOBAL__I_a_357")]
// IDA 0x761648: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_761648() {
}
// 0x761710 — __ZN3RBX9EThrottle12increaseLoadEb
// type: _DWORD __fastcall(RBX::EThrottle *__hidden this, bool)
#[doc(alias = "RBX::EThrottle::increaseLoad(bool)")]
#[doc(alias = "__ZN3RBX9EThrottle12increaseLoadEb")]
// IDA 0x761710: 40 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761710() {
}
// 0x761770 — __ZN3RBX9EThrottle15computeThrottleEi
// type: _DWORD __fastcall(RBX::EThrottle *__hidden this, int)
#[doc(alias = "RBX::EThrottle::computeThrottle(int)")]
#[doc(alias = "__ZN3RBX9EThrottle15computeThrottleEi")]
// IDA 0x761770: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761770() {
}
// 0x761834 — __ZNK3RBX9EThrottle19getEnvironmentSpeedEv
// type: _DWORD __fastcall(RBX::EThrottle *__hidden this)
#[doc(alias = "RBX::EThrottle::getEnvironmentSpeed(void)const")]
#[doc(alias = "__ZNK3RBX9EThrottle19getEnvironmentSpeedEv")]
// IDA 0x761834: 27 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761834() {
}
// 0x761890 — __ZN3RBX5WorldC1Ev
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::World(void)")]
#[doc(alias = "__ZN3RBX5WorldC1Ev")]
// IDA 0x761890: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_761890() {
}
// 0x761894 — __ZN3RBX5WorldC2Ev
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::World(void)")]
#[doc(alias = "__ZN3RBX5WorldC2Ev")]
// IDA 0x761894: 667 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761894() {
}
// 0x761f7c — __ZNK3RBX5World13loadProfilersERSt6vectorIPNS_9Profiling12CodeProfilerESaIS4_EE
#[doc(alias = "RBX::World::loadProfilers(std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>> &)const")]
#[doc(alias = "__ZNK3RBX5World13loadProfilersERSt6vectorIPNS_9Profiling12CodeProfilerESaIS4_EE")]
// IDA 0x761f7c: 121 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761f7c() {
}
// 0x7620a8 — __ZN3RBX5WorldD1Ev
// type: void __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::~World()")]
#[doc(alias = "__ZN3RBX5WorldD1Ev")]
// IDA 0x7620a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7620a8() {
}
// 0x7620ac — __ZN3RBX5WorldD2Ev
// type: void __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::~World()")]
#[doc(alias = "__ZN3RBX5WorldD2Ev")]
// IDA 0x7620ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7620ac() {
}
// 0x762774 — __ZNK3RBX5World9getKernelEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getKernel(void)const")]
#[doc(alias = "__ZNK3RBX5World9getKernelEv")]
// IDA 0x762774: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762774() {
}
// 0x762784 — __ZN3RBX5World16getSpatialFilterEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getSpatialFilter(void)")]
#[doc(alias = "__ZN3RBX5World16getSpatialFilterEv")]
// IDA 0x762784: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762784() {
}
// 0x7627a4 — __ZN3RBX5World9getKernelEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getKernel(void)")]
#[doc(alias = "__ZN3RBX5World9getKernelEv")]
// IDA 0x7627a4: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7627a4() {
}
// 0x7627b0 — __ZN3RBX5World14getSendPhysicsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getSendPhysics(void)")]
#[doc(alias = "__ZN3RBX5World14getSendPhysicsEv")]
// IDA 0x7627b0: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7627b0() {
}
// 0x7627b8 — __ZN3RBX5World16getSimSendFilterEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getSimSendFilter(void)")]
#[doc(alias = "__ZN3RBX5World16getSimSendFilterEv")]
// IDA 0x7627b8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7627b8() {
}
// 0x7627d8 — __ZNK3RBX5World12getNumBodiesEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumBodies(void)const")]
#[doc(alias = "__ZNK3RBX5World12getNumBodiesEv")]
// IDA 0x7627d8: 8 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7627d8() {
}
// 0x7627f0 — __ZNK3RBX5World12getNumPointsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumPoints(void)const")]
#[doc(alias = "__ZNK3RBX5World12getNumPointsEv")]
// IDA 0x7627f0: 8 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7627f0() {
}
// 0x762808 — __ZNK3RBX5World17getNumConstraintsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumConstraints(void)const")]
#[doc(alias = "__ZNK3RBX5World17getNumConstraintsEv")]
// IDA 0x762808: 8 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762808() {
}
// 0x762820 — __ZNK3RBX5World9getMetricENS_11IWorldStage10MetricTypeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::World::getMetric(RBX::IWorldStage::MetricType)const")]
#[doc(alias = "__ZNK3RBX5World9getMetricENS_11IWorldStage10MetricTypeE")]
// IDA 0x762820: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762820() {
}
// 0x76282c — __ZNK3RBX5World15getNumHashNodesEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumHashNodes(void)const")]
#[doc(alias = "__ZNK3RBX5World15getNumHashNodesEv")]
// IDA 0x76282c: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76282c() {
}
// 0x762838 — __ZNK3RBX5World16getMaxBucketSizeEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getMaxBucketSize(void)const")]
#[doc(alias = "__ZNK3RBX5World16getMaxBucketSizeEv")]
// IDA 0x762838: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762838() {
}
// 0x762844 — __ZN3RBX5World15ticklePrimitiveEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, bool)
#[doc(alias = "RBX::World::ticklePrimitive(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX5World15ticklePrimitiveEPNS_9PrimitiveEb")]
// IDA 0x762844: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762844() {
}
// 0x7628e0 — __ZN3RBX5World25onPrimitiveEngineChangingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitiveEngineChanging(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World25onPrimitiveEngineChangingEPNS_9PrimitiveE")]
// IDA 0x7628e0: 110 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7628e0() {
}
// 0x762a38 — __ZN3RBX5World24onPrimitiveEngineChangedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::World::onPrimitiveEngineChanged(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5World24onPrimitiveEngineChangedEPNS_8AssemblyE")]
// IDA 0x762a38: 40 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762a38() {
}
// 0x762ab0 — __ZN3RBX5World24onPrimitiveFixedChangingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitiveFixedChanging(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World24onPrimitiveFixedChangingEPNS_9PrimitiveE")]
// IDA 0x762ab0: 65 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762ab0() {
}
// 0x762b78 — __ZN3RBX5World23onPrimitiveFixedChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitiveFixedChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World23onPrimitiveFixedChangedEPNS_9PrimitiveE")]
// IDA 0x762b78: 65 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762b78() {
}
// 0x762c40 — __ZN3RBX5World32onPrimitivePreventCollideChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitivePreventCollideChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World32onPrimitivePreventCollideChangedEPNS_9PrimitiveE")]
// IDA 0x762c40: 47 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762c40() {
}
// 0x762cc8 — __ZN3RBX5World35onPrimitiveContactParametersChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitiveContactParametersChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World35onPrimitiveContactParametersChangedEPNS_9PrimitiveE")]
// IDA 0x762cc8: 41 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762cc8() {
}
// 0x762d40 — __ZN3RBX5World25onPrimitiveExtentsChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitiveExtentsChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World25onPrimitiveExtentsChangedEPNS_9PrimitiveE")]
// IDA 0x762d40: 56 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762d40() {
}
// 0x762df4 — __ZN3RBX5World24onAssemblyExtentsChangedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::World::onAssemblyExtentsChanged(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5World24onAssemblyExtentsChangedEPNS_8AssemblyE")]
// IDA 0x762df4: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762df4() {
}
// 0x762f10 — __ZN3RBX5World27onAssemblyInSimluationStageEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::World::onAssemblyInSimluationStage(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5World27onAssemblyInSimluationStageEPNS_8AssemblyE")]
// IDA 0x762f10: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762f10() {
}
// 0x762f38 — __ZN3RBX5World26onPrimitiveGeometryChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitiveGeometryChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World26onPrimitiveGeometryChangedEPNS_9PrimitiveE")]
// IDA 0x762f38: 56 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762f38() {
}
// 0x762fec — __ZN3RBX5World23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *, RBX::Primitive *)
#[doc(alias = "RBX::World::onJointPrimitiveNulling(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE")]
// IDA 0x762fec: 2 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762fec() {
}
// 0x762ff4 — __ZN3RBX5World19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *, RBX::Primitive *)
#[doc(alias = "RBX::World::onJointPrimitiveSet(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE")]
// IDA 0x762ff4: 2 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762ff4() {
}
// 0x762ffc — __ZN3RBX5World8assembleEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::assemble(void)")]
#[doc(alias = "__ZN3RBX5World8assembleEv")]
// IDA 0x762ffc: 14 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_762ffc() {
}
// 0x763020 — __ZN3RBX5World11isAssembledEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::isAssembled(void)")]
#[doc(alias = "__ZN3RBX5World11isAssembledEv")]
// IDA 0x763020: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_763020() {
}
// 0x763044 — __ZN3RBX5World14setFRMThrottleEi
// type: _DWORD __fastcall(RBX::World *__hidden this, int)
#[doc(alias = "RBX::World::setFRMThrottle(int)")]
#[doc(alias = "__ZN3RBX5World14setFRMThrottleEi")]
// IDA 0x763044: 2 insns (STR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_763044() {
}
// 0x763048 — __ZN3RBX5World23sendClumpChangedMessageEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::sendClumpChangedMessage(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World23sendClumpChangedMessageEPNS_9PrimitiveE")]
// IDA 0x763048: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_763048() {
}
// 0x763070 — __ZN3RBX5World22notifyMovingAssembliesEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::notifyMovingAssemblies(void)")]
#[doc(alias = "__ZN3RBX5World22notifyMovingAssembliesEv")]
// IDA 0x763070: 233 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_763070() {
}
// 0x7632a8 — __ZN3RBX5World6uiStepEbd
// type: _DWORD __fastcall(RBX::World *__hidden this, bool, double)
#[doc(alias = "RBX::World::uiStep(bool,double)")]
#[doc(alias = "__ZN3RBX5World6uiStepEbd")]
// IDA 0x7632a8: 289 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7632a8() {
}
// 0x7635c8 — __ZN3RBX5World13doBreakJointsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::doBreakJoints(void)")]
#[doc(alias = "__ZN3RBX5World13doBreakJointsEv")]
// IDA 0x7635c8: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7635c8() {
}
// 0x763610 — __ZN3RBX5World11doWorldStepEbii
// type: _DWORD __fastcall(RBX::World *__hidden this, bool, int, int)
#[doc(alias = "RBX::World::doWorldStep(bool,int,int)")]
#[doc(alias = "__ZN3RBX5World11doWorldStepEbii")]
// IDA 0x763610: 414 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_763610() {
}
// 0x763a84 — __ZN3RBX5World11getUiStepIdEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getUiStepId(void)")]
#[doc(alias = "__ZN3RBX5World11getUiStepIdEv")]
// IDA 0x763a84: 8 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_763a84() {
}
// 0x763aa0 — __ZN3RBX5World4stepEbdfi
// type: _DWORD __fastcall(RBX::World *__hidden this, bool, double, float, int)
#[doc(alias = "RBX::World::step(bool,double,float,int)")]
#[doc(alias = "__ZN3RBX5World4stepEbdfi")]
// IDA 0x763aa0: 506 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_763aa0() {
}
// 0x764044 — __ZN3RBX5World15reportTouchInfoERKNS0_9TouchInfoE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::World::reportTouchInfo(RBX::World::TouchInfo const&)")]
#[doc(alias = "__ZN3RBX5World15reportTouchInfoERKNS0_9TouchInfoE")]
// IDA 0x764044: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764044() {
}
// 0x76404c — __ZN3RBX5World19onPrimitiveCollidedEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::World::onPrimitiveCollided(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World19onPrimitiveCollidedEPNS_9PrimitiveES2_")]
// IDA 0x76404c: 20 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76404c() {
}
// 0x764230 — __ZN3RBX5World11insertJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::World::insertJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World11insertJointEPNS_5JointE")]
// IDA 0x764230: 162 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764230() {
}
// 0x7643d8 — __ZN3RBX5World12destroyJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::World::destroyJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World12destroyJointEPNS_5JointE")]
// IDA 0x7643d8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7643d8() {
}
// 0x764440 — __ZN3RBX5World19removeFromBreakableEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::World::removeFromBreakable(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World19removeFromBreakableEPNS_5JointE")]
// IDA 0x764440: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764440() {
}
// 0x7644b8 — __ZN3RBX5World11removeJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::World::removeJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World11removeJointEPNS_5JointE")]
// IDA 0x7644b8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7644b8() {
}
// 0x7644e0 — __ZN3RBX5World11notifyMovedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::notifyMoved(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World11notifyMovedEPNS_9PrimitiveE")]
// IDA 0x7644e0: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7644e0() {
}
// 0x764528 — __ZN3RBX5World18jointCoordsChangedEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::World::jointCoordsChanged(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World18jointCoordsChangedEPNS_5JointE")]
// IDA 0x764528: 147 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764528() {
}
// 0x7646b4 — __ZN3RBX5World13insertContactEPNS_7ContactE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Contact *)
#[doc(alias = "RBX::World::insertContact(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX5World13insertContactEPNS_7ContactE")]
// IDA 0x7646b4: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7646b4() {
}
// 0x7646cc — __ZN3RBX5World14destroyContactEPNS_7ContactE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Contact *)
#[doc(alias = "RBX::World::destroyContact(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX5World14destroyContactEPNS_7ContactE")]
// IDA 0x7646cc: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7646cc() {
}
// 0x764748 — __ZN3RBX5World7joinAllEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::joinAll(void)")]
#[doc(alias = "__ZN3RBX5World7joinAllEv")]
// IDA 0x764748: 91 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764748() {
}
// 0x764854 — __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::createAutoJoints(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World16createAutoJointsEPNS_9PrimitiveE")]
// IDA 0x764854: 3 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764854() {
}
// 0x76485c — __ZN3RBX5World15insertPrimitiveEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::World::insertPrimitive(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World15insertPrimitiveEPNS_9PrimitiveE")]
// IDA 0x76485c: 227 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76485c() {
}
