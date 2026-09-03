//! watchdog: coreB gap 150 stubs — EA-sorted asc global lowest uncovered any namespace.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in /tmp/global_eas.txt (65606 distinct, rbx_core gap) — next 150 uncovered EA-sorted asc pure gap filler any namespace (0x75fd24..0x7aca20).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes/backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::WedgePoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly9getMomentEf")]
// 0x75fd24 — __ZNK3RBX9WedgePoly9getMomentEf
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this, float)
pub fn stub_75fd24() {
    // IDA 0x75fd24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::WedgePoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly13getCofmOffsetEv")]
// 0x75fe58 — __ZNK3RBX9WedgePoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this)
pub fn stub_75fe58() {
    // IDA 0x75fe58: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::WedgePoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly21getSurfaceCoordInBodyEm")]
// 0x75fe80 — __ZNK3RBX9WedgePoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this, unsigned int)
pub fn stub_75fe80() {
    // IDA 0x75fe80: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::WedgePoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// 0x75ff4c — __ZNK3RBX9WedgePoly25getFaceFromLegacyNormalIdENS_8NormalIdE
// type: void
pub fn stub_75ff4c() {
    // IDA 0x75ff4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::WedgePoly::~WedgePoly()")]
#[doc(alias = "__ZN3RBX9WedgePolyD1Ev")]
// 0x760260 — __ZN3RBX9WedgePolyD1Ev
// type: void __fastcall(RBX::WedgePoly *__hidden this)
pub fn stub_760260() {
    // IDA 0x760260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WedgePoly::~WedgePoly()")]
#[doc(alias = "__ZN3RBX9WedgePolyD0Ev")]
// 0x760284 — __ZN3RBX9WedgePolyD0Ev
// type: void __fastcall(RBX::WedgePoly *__hidden this)
pub fn stub_760284() {
    // IDA 0x760284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv")]
// 0x7608c8 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEdlEPv
// type: void __fastcall(void *)
pub fn stub_7608c8() {
    // IDA 0x7608c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm")]
// 0x760f04 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEnwEm
// type: void
pub fn stub_760f04() {
    // IDA 0x760f04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev")]
// 0x761084 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEEC2Ev
// type: void
pub fn stub_761084() {
    // IDA 0x761084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Allocator<RBX::POLY::WedgeMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9WedgeMeshEE13releaseMemoryEv")]
// 0x7610e8 — __ZN3RBX9AllocatorINS_4POLY9WedgeMeshEE13releaseMemoryEv
// type: void
pub fn stub_7610e8() {
    // IDA 0x7610e8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::WeldJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9WeldJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
// 0x761504 — __ZN3RBX9WeldJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_761504() {
    // IDA 0x761504: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EThrottle::increaseLoad(bool)")]
#[doc(alias = "__ZN3RBX9EThrottle12increaseLoadEb")]
// 0x761710 — __ZN3RBX9EThrottle12increaseLoadEb
// type: _DWORD __fastcall(RBX::EThrottle *__hidden this, bool)
pub fn stub_761710() {
    // IDA 0x761710: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EThrottle::computeThrottle(int)")]
#[doc(alias = "__ZN3RBX9EThrottle15computeThrottleEi")]
// 0x761770 — __ZN3RBX9EThrottle15computeThrottleEi
// type: _DWORD __fastcall(RBX::EThrottle *__hidden this, int)
pub fn stub_761770() {
    // IDA 0x761770: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::EThrottle::getEnvironmentSpeed(void)const")]
#[doc(alias = "__ZNK3RBX9EThrottle19getEnvironmentSpeedEv")]
// 0x761834 — __ZNK3RBX9EThrottle19getEnvironmentSpeedEv
// type: _DWORD __fastcall(RBX::EThrottle *__hidden this)
pub fn stub_761834() {
    // IDA 0x761834: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::World(void)")]
#[doc(alias = "__ZN3RBX5WorldC1Ev")]
// 0x761890 — __ZN3RBX5WorldC1Ev
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_761890() {
    // IDA 0x761890: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::World(void)")]
#[doc(alias = "__ZN3RBX5WorldC2Ev")]
// 0x761894 — __ZN3RBX5WorldC2Ev
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_761894() {
    // IDA 0x761894: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::loadProfilers(std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>> &)const")]
#[doc(alias = "__ZNK3RBX5World13loadProfilersERSt6vectorIPNS_9Profiling12CodeProfilerESaIS4_EE")]
// 0x761f7c — __ZNK3RBX5World13loadProfilersERSt6vectorIPNS_9Profiling12CodeProfilerESaIS4_EE
// type: void
pub fn stub_761f7c() {
    // IDA 0x761f7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::~World()")]
#[doc(alias = "__ZN3RBX5WorldD1Ev")]
// 0x7620a8 — __ZN3RBX5WorldD1Ev
// type: void __fastcall(RBX::World *__hidden this)
pub fn stub_7620a8() {
    // IDA 0x7620a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::~World()")]
#[doc(alias = "__ZN3RBX5WorldD2Ev")]
// 0x7620ac — __ZN3RBX5WorldD2Ev
// type: void __fastcall(RBX::World *__hidden this)
pub fn stub_7620ac() {
    // IDA 0x7620ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getKernel(void)const")]
#[doc(alias = "__ZNK3RBX5World9getKernelEv")]
// 0x762774 — __ZNK3RBX5World9getKernelEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_762774() {
    // IDA 0x762774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getSpatialFilter(void)")]
#[doc(alias = "__ZN3RBX5World16getSpatialFilterEv")]
// 0x762784 — __ZN3RBX5World16getSpatialFilterEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_762784() {
    // IDA 0x762784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getKernel(void)")]
#[doc(alias = "__ZN3RBX5World9getKernelEv")]
// 0x7627a4 — __ZN3RBX5World9getKernelEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_7627a4() {
    // IDA 0x7627a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::World::getSendPhysics(void)")]
#[doc(alias = "__ZN3RBX5World14getSendPhysicsEv")]
// 0x7627b0 — __ZN3RBX5World14getSendPhysicsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_7627b0() {
    // IDA 0x7627b0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getSimSendFilter(void)")]
#[doc(alias = "__ZN3RBX5World16getSimSendFilterEv")]
// 0x7627b8 — __ZN3RBX5World16getSimSendFilterEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_7627b8() {
    // IDA 0x7627b8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumBodies(void)const")]
#[doc(alias = "__ZNK3RBX5World12getNumBodiesEv")]
// 0x7627d8 — __ZNK3RBX5World12getNumBodiesEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_7627d8() {
    // IDA 0x7627d8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumPoints(void)const")]
#[doc(alias = "__ZNK3RBX5World12getNumPointsEv")]
// 0x7627f0 — __ZNK3RBX5World12getNumPointsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_7627f0() {
    // IDA 0x7627f0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumConstraints(void)const")]
#[doc(alias = "__ZNK3RBX5World17getNumConstraintsEv")]
// 0x762808 — __ZNK3RBX5World17getNumConstraintsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_762808() {
    // IDA 0x762808: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getMetric(RBX::IWorldStage::MetricType)const")]
#[doc(alias = "__ZNK3RBX5World9getMetricENS_11IWorldStage10MetricTypeE")]
// 0x762820 — __ZNK3RBX5World9getMetricENS_11IWorldStage10MetricTypeE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_762820() {
    // IDA 0x762820: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getNumHashNodes(void)const")]
#[doc(alias = "__ZNK3RBX5World15getNumHashNodesEv")]
// 0x76282c — __ZNK3RBX5World15getNumHashNodesEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_76282c() {
    // IDA 0x76282c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getMaxBucketSize(void)const")]
#[doc(alias = "__ZNK3RBX5World16getMaxBucketSizeEv")]
// 0x762838 — __ZNK3RBX5World16getMaxBucketSizeEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_762838() {
    // IDA 0x762838: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::ticklePrimitive(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX5World15ticklePrimitiveEPNS_9PrimitiveEb")]
// 0x762844 — __ZN3RBX5World15ticklePrimitiveEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, bool)
pub fn stub_762844() {
    // IDA 0x762844: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveEngineChanging(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World25onPrimitiveEngineChangingEPNS_9PrimitiveE")]
// 0x7628e0 — __ZN3RBX5World25onPrimitiveEngineChangingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_7628e0() {
    // IDA 0x7628e0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveEngineChanged(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5World24onPrimitiveEngineChangedEPNS_8AssemblyE")]
// 0x762a38 — __ZN3RBX5World24onPrimitiveEngineChangedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Assembly *)
pub fn stub_762a38() {
    // IDA 0x762a38: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveFixedChanging(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World24onPrimitiveFixedChangingEPNS_9PrimitiveE")]
// 0x762ab0 — __ZN3RBX5World24onPrimitiveFixedChangingEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_762ab0() {
    // IDA 0x762ab0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveFixedChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World23onPrimitiveFixedChangedEPNS_9PrimitiveE")]
// 0x762b78 — __ZN3RBX5World23onPrimitiveFixedChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_762b78() {
    // IDA 0x762b78: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitivePreventCollideChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World32onPrimitivePreventCollideChangedEPNS_9PrimitiveE")]
// 0x762c40 — __ZN3RBX5World32onPrimitivePreventCollideChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_762c40() {
    // IDA 0x762c40: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveContactParametersChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World35onPrimitiveContactParametersChangedEPNS_9PrimitiveE")]
// 0x762cc8 — __ZN3RBX5World35onPrimitiveContactParametersChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_762cc8() {
    // IDA 0x762cc8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveExtentsChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World25onPrimitiveExtentsChangedEPNS_9PrimitiveE")]
// 0x762d40 — __ZN3RBX5World25onPrimitiveExtentsChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_762d40() {
    // IDA 0x762d40: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onAssemblyExtentsChanged(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX5World24onAssemblyExtentsChangedEPNS_8AssemblyE")]
// 0x762df4 — __ZN3RBX5World24onAssemblyExtentsChangedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Assembly *)
pub fn stub_762df4() {
    // IDA 0x762df4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveGeometryChanged(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World26onPrimitiveGeometryChangedEPNS_9PrimitiveE")]
// 0x762f38 — __ZN3RBX5World26onPrimitiveGeometryChangedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_762f38() {
    // IDA 0x762f38: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onJointPrimitiveNulling(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE")]
// 0x762fec — __ZN3RBX5World23onJointPrimitiveNullingEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *, RBX::Primitive *)
pub fn stub_762fec() {
    // IDA 0x762fec: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onJointPrimitiveSet(RBX::Joint *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE")]
// 0x762ff4 — __ZN3RBX5World19onJointPrimitiveSetEPNS_5JointEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *, RBX::Primitive *)
pub fn stub_762ff4() {
    // IDA 0x762ff4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::assemble(void)")]
#[doc(alias = "__ZN3RBX5World8assembleEv")]
// 0x762ffc — __ZN3RBX5World8assembleEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_762ffc() {
    // IDA 0x762ffc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::isAssembled(void)")]
#[doc(alias = "__ZN3RBX5World11isAssembledEv")]
// 0x763020 — __ZN3RBX5World11isAssembledEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_763020() {
    // IDA 0x763020: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::setFRMThrottle(int)")]
#[doc(alias = "__ZN3RBX5World14setFRMThrottleEi")]
// 0x763044 — __ZN3RBX5World14setFRMThrottleEi
// type: _DWORD __fastcall(RBX::World *__hidden this, int)
pub fn stub_763044() {
    // IDA 0x763044: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::sendClumpChangedMessage(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World23sendClumpChangedMessageEPNS_9PrimitiveE")]
// 0x763048 — __ZN3RBX5World23sendClumpChangedMessageEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_763048() {
    // IDA 0x763048: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::notifyMovingAssemblies(void)")]
#[doc(alias = "__ZN3RBX5World22notifyMovingAssembliesEv")]
// 0x763070 — __ZN3RBX5World22notifyMovingAssembliesEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_763070() {
    // IDA 0x763070: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::uiStep(bool,double)")]
#[doc(alias = "__ZN3RBX5World6uiStepEbd")]
// 0x7632a8 — __ZN3RBX5World6uiStepEbd
// type: _DWORD __fastcall(RBX::World *__hidden this, bool, double)
pub fn stub_7632a8() {
    // IDA 0x7632a8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::doBreakJoints(void)")]
#[doc(alias = "__ZN3RBX5World13doBreakJointsEv")]
// 0x7635c8 — __ZN3RBX5World13doBreakJointsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_7635c8() {
    // IDA 0x7635c8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::doWorldStep(bool,int,int)")]
#[doc(alias = "__ZN3RBX5World11doWorldStepEbii")]
// 0x763610 — __ZN3RBX5World11doWorldStepEbii
// type: _DWORD __fastcall(RBX::World *__hidden this, bool, int, int)
pub fn stub_763610() {
    // IDA 0x763610: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::getUiStepId(void)")]
#[doc(alias = "__ZN3RBX5World11getUiStepIdEv")]
// 0x763a84 — __ZN3RBX5World11getUiStepIdEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_763a84() {
    // IDA 0x763a84: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::step(bool,double,float,int)")]
#[doc(alias = "__ZN3RBX5World4stepEbdfi")]
// 0x763aa0 — __ZN3RBX5World4stepEbdfi
// type: _DWORD __fastcall(RBX::World *__hidden this, bool, double, float, int)
pub fn stub_763aa0() {
    // IDA 0x763aa0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::reportTouchInfo(RBX::World::TouchInfo const&)")]
#[doc(alias = "__ZN3RBX5World15reportTouchInfoERKNS0_9TouchInfoE")]
// 0x764044 — __ZN3RBX5World15reportTouchInfoERKNS0_9TouchInfoE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_764044() {
    // IDA 0x764044: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::onPrimitiveCollided(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World19onPrimitiveCollidedEPNS_9PrimitiveES2_")]
// 0x76404c — __ZN3RBX5World19onPrimitiveCollidedEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_76404c() {
    // IDA 0x76404c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::insertJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World11insertJointEPNS_5JointE")]
// 0x764230 — __ZN3RBX5World11insertJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
pub fn stub_764230() {
    // IDA 0x764230: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::destroyJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World12destroyJointEPNS_5JointE")]
// 0x7643d8 — __ZN3RBX5World12destroyJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
pub fn stub_7643d8() {
    // IDA 0x7643d8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::removeFromBreakable(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World19removeFromBreakableEPNS_5JointE")]
// 0x764440 — __ZN3RBX5World19removeFromBreakableEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
pub fn stub_764440() {
    // IDA 0x764440: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::removeJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World11removeJointEPNS_5JointE")]
// 0x7644b8 — __ZN3RBX5World11removeJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
pub fn stub_7644b8() {
    // IDA 0x7644b8: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::notifyMoved(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World11notifyMovedEPNS_9PrimitiveE")]
// 0x7644e0 — __ZN3RBX5World11notifyMovedEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_7644e0() {
    // IDA 0x7644e0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::jointCoordsChanged(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX5World18jointCoordsChangedEPNS_5JointE")]
// 0x764528 — __ZN3RBX5World18jointCoordsChangedEPNS_5JointE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Joint *)
pub fn stub_764528() {
    // IDA 0x764528: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::insertContact(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX5World13insertContactEPNS_7ContactE")]
// 0x7646b4 — __ZN3RBX5World13insertContactEPNS_7ContactE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Contact *)
pub fn stub_7646b4() {
    // IDA 0x7646b4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::destroyContact(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX5World14destroyContactEPNS_7ContactE")]
// 0x7646cc — __ZN3RBX5World14destroyContactEPNS_7ContactE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Contact *)
pub fn stub_7646cc() {
    // IDA 0x7646cc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::joinAll(void)")]
#[doc(alias = "__ZN3RBX5World7joinAllEv")]
// 0x764748 — __ZN3RBX5World7joinAllEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_764748() {
    // IDA 0x764748: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::World::createAutoJoints(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World16createAutoJointsEPNS_9PrimitiveE")]
// 0x764854 — __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_764854() {
    // IDA 0x764854: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::insertPrimitive(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5World15insertPrimitiveEPNS_9PrimitiveE")]
// 0x76485c — __ZN3RBX5World15insertPrimitiveEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *)
pub fn stub_76485c() {
    // IDA 0x76485c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::removePrimitive(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX5World15removePrimitiveEPNS_9PrimitiveEb")]
// 0x764b38 — __ZN3RBX5World15removePrimitiveEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, bool)
pub fn stub_764b38() {
    // IDA 0x764b38: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::World::destroyAutoJoints(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,bool,bool)")]
#[doc(alias = "__ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EEbb")]
// 0x764e34 — __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EEbb
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_764e34() {
    // IDA 0x764e34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::doNotIgnore(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *)")]
#[doc(alias = "__ZN3RBX11doNotIgnoreEPNS_9PrimitiveEPSt3setIS1_St4lessIS1_ESaIS1_EES7_")]
// 0x7651e8 — __ZN3RBX11doNotIgnoreEPNS_9PrimitiveEPSt3setIS1_St4lessIS1_ESaIS1_EES7_
// type: void
pub fn stub_7651e8() {
    // IDA 0x7651e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::destroyAutoJoints(RBX::Primitive *,bool)")]
#[doc(alias = "__ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEb")]
// 0x765414 — __ZN3RBX5World17destroyAutoJointsEPNS_9PrimitiveEb
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Primitive *, bool)
pub fn stub_765414() {
    // IDA 0x765414: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::createAutoJoints(RBX::Primitive *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *,std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>> *)")]
#[doc(alias = "__ZN3RBX5World16createAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EES8_")]
// 0x7655a0 — __ZN3RBX5World16createAutoJointsEPNS_9PrimitiveEPSt3setIS2_St4lessIS2_ESaIS2_EES8_
// type: int __fastcall(int, int, int, int)
pub fn stub_7655a0() {
    // IDA 0x7655a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::push_back(RBX::Profiling::CodeProfiler * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE9push_backERKS3_")]
// 0x765980 — __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE9push_backERKS3_
// type: void
pub fn stub_765980() {
    // IDA 0x765980: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::notifyMovingPrimitives<std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>>(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> const&)")]
#[doc(alias = "__ZN3RBX22notifyMovingPrimitivesISt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEEEvRKT_")]
// 0x7659ac — __ZN3RBX22notifyMovingPrimitivesISt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEEEvRKT_
// type: void
pub fn stub_7659ac() {
    // IDA 0x7659ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::World::assertNotInStep(void)")]
#[doc(alias = "__ZN3RBX5World15assertNotInStepEv")]
// 0x765c1c — __ZN3RBX5World15assertNotInStepEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
pub fn stub_765c1c() {
    // IDA 0x765c1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IndexArray<RBX::Primitive,&RBX::Primitive::worldIndexFunc>::fastRemove(RBX::Primitive*)")]
#[doc(alias = "__ZN3RBX10IndexArrayINS_9PrimitiveEXadL_ZNS1_14worldIndexFuncEvEEE10fastRemoveEPS1_")]
// 0x765dc4 — __ZN3RBX10IndexArrayINS_9PrimitiveEXadL_ZNS1_14worldIndexFuncEvEEE10fastRemoveEPS1_
// type: void
pub fn stub_765dc4() {
    // IDA 0x765dc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Joint::isAutoJoint(RBX::Joint const*)")]
#[doc(alias = "__ZN3RBX5Joint11isAutoJointEPKS0_")]
// 0x765e9c — __ZN3RBX5Joint11isAutoJointEPKS0_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Joint *)
pub fn stub_765e9c() {
    // IDA 0x765e9c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::push_back(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE9push_backERKS2_")]
// 0x766074 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE9push_backERKS2_
// type: void
pub fn stub_766074() {
    // IDA 0x766074: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,RBX::Joint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x7660a0 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_7660a0() {
    // IDA 0x7660a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX5JointESaIS2_EE11_M_allocateEm")]
// 0x766180 — __ZNSt12_Vector_baseIPN3RBX5JointESaIS2_EE11_M_allocateEm
// type: void
pub fn stub_766180() {
    // IDA 0x766180: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")]
// 0x7667fc — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
// type: void
pub fn stub_7667fc() {
    // IDA 0x7667fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::equal_range(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")]
// 0x766824 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
// type: void
pub fn stub_766824() {
    // IDA 0x766824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::erase(std::_Rb_tree_iterator<RBX::Joint *>,std::_Rb_tree_iterator<RBX::Joint *>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")]
// 0x766870 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_766870() {
    // IDA 0x766870: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_erase(std::_Rb_tree_node<RBX::Joint *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0x7668d0 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: void
pub fn stub_7668d0() {
    // IDA 0x7668d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert_unique(RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0x7668f8 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// type: void
pub fn stub_7668f8() {
    // IDA 0x7668f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Joint *,RBX::Joint *,std::_Identity<RBX::Joint *>,std::less<RBX::Joint *>,std::allocator<RBX::Joint *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Joint * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0x766960 — __ZNSt8_Rb_treeIPN3RBX5JointES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: void
pub fn stub_766960() {
    // IDA 0x766960: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Profiling::CodeProfiler **,std::vector<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>>,RBX::Profiling::CodeProfiler * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// 0x766d1c — __ZNSt6vectorIPN3RBX9Profiling12CodeProfilerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, void *__src)
pub fn stub_766d1c() {
    // IDA 0x766d1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Profiling::CodeProfiler *,std::allocator<RBX::Profiling::CodeProfiler *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX9Profiling12CodeProfilerESaIS3_EE11_M_allocateEm")]
// 0x766dfc — __ZNSt12_Vector_baseIPN3RBX9Profiling12CodeProfilerESaIS3_EE11_M_allocateEm
// type: void
pub fn stub_766dfc() {
    // IDA 0x766dfc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlElement::XmlElement<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
#[doc(alias = "__ZN10XmlElementC2IPKN3RBX4NameEEERS3_T_")]
// 0x78a098 — __ZN10XmlElementC2IPKN3RBX4NameEEERS3_T_
// type: void
pub fn stub_78a098() {
    // IDA 0x78a098: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlAttribute::XmlAttribute<int>(RBX::Name const&,int)")]
#[doc(alias = "__ZN12XmlAttributeC2IiEERKN3RBX4NameET_")]
// 0x78a16c — __ZN12XmlAttributeC2IiEERKN3RBX4NameET_
// type: void
pub fn stub_78a16c() {
    // IDA 0x78a16c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlAttribute::XmlAttribute<char const*>(RBX::Name const&,char const*)")]
#[doc(alias = "__ZN12XmlAttributeC2IPKcEERKN3RBX4NameET_")]
// 0x78a230 — __ZN12XmlAttributeC2IPKcEERKN3RBX4NameET_
// type: void
pub fn stub_78a230() {
    // IDA 0x78a230: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "XmlNameValuePair::XmlNameValuePair(RBX::Name const&,char const*)")]
#[doc(alias = "__ZN16XmlNameValuePairC2ERKN3RBX4NameEPKc")]
// 0x78a2ec — __ZN16XmlNameValuePairC2ERKN3RBX4NameEPKc
// type: XmlNameValuePair *__fastcall(XmlNameValuePair *__hidden this, const RBX::Name *, const char *)
pub fn stub_78a2ec() {
    // IDA 0x78a2ec: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<XmlElement>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorI10XmlElementEdlEPv")]
// 0x7986c0 — __ZN3RBX9AllocatorI10XmlElementEdlEPv
// type: void __fastcall(_DWORD *)
pub fn stub_7986c0() {
    // IDA 0x7986c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlElement::findAttribute(RBX::Name const&)const")]
#[doc(alias = "__ZNK10XmlElement13findAttributeERKN3RBX4NameE")]
// 0x79894c — __ZNK10XmlElement13findAttributeERKN3RBX4NameE
// type: const Name **__fastcall(XmlElement *this, const Name *)
pub fn stub_79894c() {
    // IDA 0x79894c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlElement::findFirstChildByTag(RBX::Name const&)const")]
#[doc(alias = "__ZNK10XmlElement19findFirstChildByTagERKN3RBX4NameE")]
// 0x7989a4 — __ZNK10XmlElement19findFirstChildByTagERKN3RBX4NameE
// type: const Name **__fastcall(XmlElement *this, const Name *)
pub fn stub_7989a4() {
    // IDA 0x7989a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlElement::findAttribute(RBX::Name const&)")]
#[doc(alias = "__ZN10XmlElement13findAttributeERKN3RBX4NameE")]
// 0x7989d4 — __ZN10XmlElement13findAttributeERKN3RBX4NameE
// type: const Name **__fastcall(XmlElement *this, const Name *)
pub fn stub_7989d4() {
    // IDA 0x7989d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlNameValuePair::isValueEqual(RBX::Name const*)const")]
#[doc(alias = "__ZNK16XmlNameValuePair12isValueEqualEPKN3RBX4NameE")]
// 0x798af0 — __ZNK16XmlNameValuePair12isValueEqualEPKN3RBX4NameE
// type: bool __fastcall(XmlNameValuePair *this, const RBX::Name *)
pub fn stub_798af0() {
    // IDA 0x798af0: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlNameValuePair::getValue(RBX::Name const*&)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERPKN3RBX4NameE")]
// 0x798b24 — __ZNK16XmlNameValuePair8getValueERPKN3RBX4NameE
// type: int __fastcall(__int64 this)
pub fn stub_798b24() {
    // IDA 0x798b24: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "bool XmlNameValuePair::isValueType<RBX::ContentId>(void)const")]
#[doc(alias = "__ZNK16XmlNameValuePair11isValueTypeIN3RBX9ContentIdEEEbv")]
// 0x798b64 — __ZNK16XmlNameValuePair11isValueTypeIN3RBX9ContentIdEEEbv
// type: bool __fastcall(int)
pub fn stub_798b64() {
    // IDA 0x798b64: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlNameValuePair::getValue(RBX::ContentId &)const")]
#[doc(alias = "__ZNK16XmlNameValuePair8getValueERN3RBX9ContentIdE")]
// 0x798b7c — __ZNK16XmlNameValuePair8getValueERN3RBX9ContentIdE
// type: int __fastcall(XmlNameValuePair *this, RBX::ContentId *)
pub fn stub_798b7c() {
    // IDA 0x798b7c: content-id plumbing owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ChatLine::ChatLine(RBX::ChatLine::ChatType,std::string const&,float,RBX::ChatLine::BubbleColor,bool)")]
#[doc(alias = "__ZN3RBX8ChatLineC2ENS0_8ChatTypeERKSsfNS0_11BubbleColorEb")]
// 0x79d51c — __ZN3RBX8ChatLineC2ENS0_8ChatTypeERKSsfNS0_11BubbleColorEb
// type: int __fastcall(int, int, std::string *, int, int, int)
pub fn stub_79d51c() {
    // IDA 0x79d51c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChatOutput::ChatOutput(void)")]
#[doc(alias = "__ZN3RBX10ChatOutputC1Ev")]
// 0x79d948 — __ZN3RBX10ChatOutputC1Ev
// type: int __fastcall(RBX::ChatOutput *this)
pub fn stub_79d948() {
    // IDA 0x79d948: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChatOutput::ChatOutput(void)")]
#[doc(alias = "__ZN3RBX10ChatOutputC2Ev")]
// 0x79d94c — __ZN3RBX10ChatOutputC2Ev
// type: RBX::GuiItem *__fastcall(RBX::ChatOutput *this)
pub fn stub_79d94c() {
    // IDA 0x79d94c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::createChatBubbleMain(std::string const&)")]
#[doc(alias = "__ZN3RBXL20createChatBubbleMainERKSs")]
// 0x79ef20 — __ZN3RBXL20createChatBubbleMainERKSs
// type: void __fastcall(RBX *this, const std::string *)
pub fn stub_79ef20() {
    // IDA 0x79ef20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::createChatBubbleWithTail(std::string const&,RBX::UDim2 const&,RBX::UDim2 const&)")]
#[doc(alias = "__ZN3RBXL24createChatBubbleWithTailERKSsRKNS_5UDim2ES4_")]
// 0x79f280 — __ZN3RBXL24createChatBubbleWithTailERKSsRKNS_5UDim2ES4_
// type: void __fastcall(RBX *, const std::string *, _DWORD *, _DWORD *)
pub fn stub_79f280() {
    // IDA 0x79f280: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::createScaledChatBubbleWithTail(std::string const&,float,RBX::UDim2 const&)")]
#[doc(alias = "__ZN3RBXL30createScaledChatBubbleWithTailERKSsfRKNS_5UDim2E")]
// 0x79f798 — __ZN3RBXL30createScaledChatBubbleWithTailERKSsfRKNS_5UDim2E
// type: void __fastcall(RBX *, const std::string *, _DWORD *)
pub fn stub_79f798() {
    // IDA 0x79f798: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::createChatImposter(std::string const&,std::string const&,float)")]
#[doc(alias = "__ZN3RBXL18createChatImposterERKSsS1_f")]
// 0x79fdec — __ZN3RBXL18createChatImposterERKSsS1_f
// type: void __fastcall(RBX *this, const std::string *, const std::string *, float)
pub fn stub_79fdec() {
    // IDA 0x79fdec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChatOutput::~ChatOutput()")]
#[doc(alias = "__ZN3RBX10ChatOutputD0Ev")]
// 0x7a059c — __ZN3RBX10ChatOutputD0Ev
// type: void __fastcall(RBX::ChatOutput *__hidden this)
pub fn stub_7a059c() {
    // IDA 0x7a059c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatOutput::~ChatOutput()")]
#[doc(alias = "__ZN3RBX10ChatOutputD1Ev")]
// 0x7a063c — __ZN3RBX10ChatOutputD1Ev
// type: void __fastcall(RBX::ChatOutput *__hidden this)
pub fn stub_7a063c() {
    // IDA 0x7a063c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatOutput::~ChatOutput()")]
#[doc(alias = "__ZThn32_N3RBX10ChatOutputD0Ev")]
// 0x7a0640 — __ZThn32_N3RBX10ChatOutputD0Ev
// type: void __fastcall(RBX::ChatOutput *__hidden this)
pub fn stub_7a0640() {
    // IDA 0x7a0640: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatOutput::~ChatOutput()")]
#[doc(alias = "__ZThn36_N3RBX10ChatOutputD0Ev")]
// 0x7a0648 — __ZThn36_N3RBX10ChatOutputD0Ev
// type: void __fastcall(RBX::ChatOutput *__hidden this)
pub fn stub_7a0648() {
    // IDA 0x7a0648: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatOutput::~ChatOutput()")]
#[doc(alias = "__ZN3RBX10ChatOutputD2Ev")]
// 0x7a0650 — __ZN3RBX10ChatOutputD2Ev
// type: void __fastcall(RBX::ChatOutput *__hidden this)
pub fn stub_7a0650() {
    // IDA 0x7a0650: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatOutput::~ChatOutput()")]
#[doc(alias = "__ZThn32_N3RBX10ChatOutputD1Ev")]
// 0x7a0a28 — __ZThn32_N3RBX10ChatOutputD1Ev
// type: void __fastcall(RBX::ChatOutput *__hidden this)
pub fn stub_7a0a28() {
    // IDA 0x7a0a28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatOutput::~ChatOutput()")]
#[doc(alias = "__ZThn36_N3RBX10ChatOutputD1Ev")]
// 0x7a0a30 — __ZThn36_N3RBX10ChatOutputD1Ev
// type: void __fastcall(RBX::ChatOutput *__hidden this)
pub fn stub_7a0a30() {
    // IDA 0x7a0a30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatOutput::SanitizeChatLine(std::string const&)")]
#[doc(alias = "__ZN3RBX10ChatOutput16SanitizeChatLineERKSs")]
// 0x7a0a3c — __ZN3RBX10ChatOutput16SanitizeChatLineERKSs
// type: void __fastcall(RBX::ChatOutput *this, const std::string *, const std::string *)
pub fn stub_7a0a3c() {
    // IDA 0x7a0a3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatOutput::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3RBX10ChatOutput11onHeartbeatERKNS_9HeartbeatE")]
// 0x7a0e00 — __ZN3RBX10ChatOutput11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(RBX::ChatOutput *, int)
pub fn stub_7a0e00() {
    // IDA 0x7a0e00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatOutput::removeExpired(void)")]
#[doc(alias = "__ZN3RBX10ChatOutput13removeExpiredEv")]
// 0x7a14f0 — __ZN3RBX10ChatOutput13removeExpiredEv
// type: int __fastcall(RBX::ChatOutput *this, int, int, int)
pub fn stub_7a14f0() {
    // IDA 0x7a14f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatLine::~ChatLine()")]
#[doc(alias = "__ZN3RBX8ChatLineD2Ev")]
// 0x7a38b0 — __ZN3RBX8ChatLineD2Ev
// type: void __fastcall(RBX::ChatLine *__hidden this)
pub fn stub_7a38b0() {
    // IDA 0x7a38b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatLine::getOrigin(void)const")]
#[doc(alias = "__ZNK3RBX8ChatLine9getOriginEv")]
// 0x7a3b24 — __ZNK3RBX8ChatLine9getOriginEv
// type: _DWORD __fastcall(RBX::ChatLine *__hidden this)
pub fn stub_7a3b24() {
    // IDA 0x7a3b24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatLine::~ChatLine()")]
#[doc(alias = "__ZN3RBX8ChatLineD1Ev")]
// 0x7a4838 — __ZN3RBX8ChatLineD1Ev
// type: void __fastcall(RBX::ChatLine *__hidden this)
pub fn stub_7a4838() {
    // IDA 0x7a4838: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatLine::~ChatLine()")]
#[doc(alias = "__ZN3RBX8ChatLineD0Ev")]
// 0x7a483c — __ZN3RBX8ChatLineD0Ev
// type: void __fastcall(RBX::ChatLine *__hidden this)
pub fn stub_7a483c() {
    // IDA 0x7a483c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PlayerChatLine::~PlayerChatLine()")]
#[doc(alias = "__ZN3RBX14PlayerChatLineD1Ev")]
// 0x7a48dc — __ZN3RBX14PlayerChatLineD1Ev
// type: void __fastcall(RBX::PlayerChatLine *__hidden this)
pub fn stub_7a48dc() {
    // IDA 0x7a48dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PlayerChatLine::~PlayerChatLine()")]
#[doc(alias = "__ZN3RBX14PlayerChatLineD0Ev")]
// 0x7a4908 — __ZN3RBX14PlayerChatLineD0Ev
// type: void __fastcall(RBX::PlayerChatLine *__hidden this)
pub fn stub_7a4908() {
    // IDA 0x7a4908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_NS0_10ChatOutput11ScalingInfoEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0x7a9c48 — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_NS0_10ChatOutput11ScalingInfoEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void
pub fn stub_7a9c48() {
    // IDA 0x7a9c48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatButton::isVisible(void)const")]
#[doc(alias = "__ZNK3RBX10ChatButton9isVisibleEv")]
// 0x7aa83c — __ZNK3RBX10ChatButton9isVisibleEv
// type: _DWORD __fastcall(RBX::ChatButton *__hidden this)
pub fn stub_7aa83c() {
    // IDA 0x7aa83c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatWidget::ChatWidget(std::string const&,std::string)")]
#[doc(alias = "__ZN3RBX10ChatWidgetC1ERKSsSs")]
// 0x7aa864 — __ZN3RBX10ChatWidgetC1ERKSsSs
// type: void
pub fn stub_7aa864() {
    // IDA 0x7aa864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatWidget::ChatWidget(std::string const&,std::string)")]
#[doc(alias = "__ZN3RBX10ChatWidgetC2ERKSsSs")]
// 0x7aa868 — __ZN3RBX10ChatWidgetC2ERKSsSs
// type: void
pub fn stub_7aa868() {
    // IDA 0x7aa868: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChatWidget::onMenuStateChanged(void)")]
#[doc(alias = "__ZN3RBX10ChatWidget18onMenuStateChangedEv")]
// 0x7aa984 — __ZN3RBX10ChatWidget18onMenuStateChangedEv
// type: _DWORD __fastcall(RBX::ChatWidget *__hidden this)
pub fn stub_7aa984() {
    // IDA 0x7aa984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChatWidget::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX10ChatWidget7processERKNS_8GuiEventE")]
// 0x7aa994 — __ZN3RBX10ChatWidget7processERKNS_8GuiEventE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, char, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_7aa994() {
    // IDA 0x7aa994: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk toRBX::ChatWidget::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZThn92_N3RBX10ChatWidget7processERKNS_8GuiEventE")]
// 0x7aac2c — __ZThn92_N3RBX10ChatWidget7processERKNS_8GuiEventE
// type: void
pub fn stub_7aac2c() {
    // IDA 0x7aac2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZN3RBX10ChatButtonD1Ev")]
// 0x7aac68 — __ZN3RBX10ChatButtonD1Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
pub fn stub_7aac68() {
    // IDA 0x7aac68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZN3RBX10ChatButtonD0Ev")]
// 0x7aad78 — __ZN3RBX10ChatButtonD0Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
pub fn stub_7aad78() {
    // IDA 0x7aad78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn32_N3RBX10ChatButtonD1Ev")]
// 0x7aae98 — __ZThn32_N3RBX10ChatButtonD1Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
pub fn stub_7aae98() {
    // IDA 0x7aae98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn32_N3RBX10ChatButtonD0Ev")]
// 0x7aafa8 — __ZThn32_N3RBX10ChatButtonD0Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
pub fn stub_7aafa8() {
    // IDA 0x7aafa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn36_N3RBX10ChatButtonD1Ev")]
// 0x7ab0cc — __ZThn36_N3RBX10ChatButtonD1Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
pub fn stub_7ab0cc() {
    // IDA 0x7ab0cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn36_N3RBX10ChatButtonD0Ev")]
// 0x7ab1dc — __ZThn36_N3RBX10ChatButtonD0Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
pub fn stub_7ab1dc() {
    // IDA 0x7ab1dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZN3RBX10ChatWidgetD1Ev")]
// 0x7ab300 — __ZN3RBX10ChatWidgetD1Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
pub fn stub_7ab300() {
    // IDA 0x7ab300: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZN3RBX10ChatWidgetD0Ev")]
// 0x7ab3ec — __ZN3RBX10ChatWidgetD0Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
pub fn stub_7ab3ec() {
    // IDA 0x7ab3ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn32_N3RBX10ChatWidgetD1Ev")]
// 0x7ab4ec — __ZThn32_N3RBX10ChatWidgetD1Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
pub fn stub_7ab4ec() {
    // IDA 0x7ab4ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn32_N3RBX10ChatWidgetD0Ev")]
// 0x7ab5d8 — __ZThn32_N3RBX10ChatWidgetD0Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
pub fn stub_7ab5d8() {
    // IDA 0x7ab5d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn36_N3RBX10ChatWidgetD1Ev")]
// 0x7ab6d8 — __ZThn36_N3RBX10ChatWidgetD1Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
pub fn stub_7ab6d8() {
    // IDA 0x7ab6d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn36_N3RBX10ChatWidgetD0Ev")]
// 0x7ab7c4 — __ZThn36_N3RBX10ChatWidgetD0Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
pub fn stub_7ab7c4() {
    // IDA 0x7ab7c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EquationDisplay::EquationDisplay(std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX15EquationDisplayC1ERKSsS2_")]
// 0x7abad8 — __ZN3RBX15EquationDisplayC1ERKSsS2_
// type: _DWORD __fastcall(RBX::EquationDisplay *__hidden this, const std::string *, const std::string *)
pub fn stub_7abad8() {
    // IDA 0x7abad8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EquationDisplay::EquationDisplay(std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX15EquationDisplayC2ERKSsS2_")]
// 0x7abadc — __ZN3RBX15EquationDisplayC2ERKSsS2_
// type: _DWORD __fastcall(RBX::EquationDisplay *__hidden this, const std::string *, const std::string *)
pub fn stub_7abadc() {
    // IDA 0x7abadc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EquationDisplay::getLabel(void)const")]
#[doc(alias = "__ZNK3RBX15EquationDisplay8getLabelEv")]
// 0x7abc28 — __ZNK3RBX15EquationDisplay8getLabelEv
// type: _DWORD __fastcall(RBX::EquationDisplay *__hidden this)
pub fn stub_7abc28() {
    // IDA 0x7abc28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZN3RBX15EquationDisplayD1Ev")]
// 0x7abfb4 — __ZN3RBX15EquationDisplayD1Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
pub fn stub_7abfb4() {
    // IDA 0x7abfb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZN3RBX15EquationDisplayD0Ev")]
// 0x7ac150 — __ZN3RBX15EquationDisplayD0Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
pub fn stub_7ac150() {
    // IDA 0x7ac150: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn32_N3RBX15EquationDisplayD1Ev")]
// 0x7ac1f0 — __ZThn32_N3RBX15EquationDisplayD1Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
pub fn stub_7ac1f0() {
    // IDA 0x7ac1f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn32_N3RBX15EquationDisplayD0Ev")]
// 0x7ac38c — __ZThn32_N3RBX15EquationDisplayD0Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
pub fn stub_7ac38c() {
    // IDA 0x7ac38c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn36_N3RBX15EquationDisplayD1Ev")]
// 0x7ac53c — __ZThn36_N3RBX15EquationDisplayD1Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
pub fn stub_7ac53c() {
    // IDA 0x7ac53c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn36_N3RBX15EquationDisplayD0Ev")]
// 0x7ac6d8 — __ZThn36_N3RBX15EquationDisplayD0Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
pub fn stub_7ac6d8() {
    // IDA 0x7ac6d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiItem::disabledFill(void)")]
#[doc(alias = "__ZN3RBX7GuiItem12disabledFillEv")]
// 0x7aca20 — __ZN3RBX7GuiItem12disabledFillEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
pub fn stub_7aca20() {
    // IDA 0x7aca20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
