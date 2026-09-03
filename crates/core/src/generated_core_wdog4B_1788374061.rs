//! core wdog4B — 100 core stubs EA-sorted asc distinct not yet in any crate.
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 uncovered not yet in any crate filtered for core (excludes Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua|Script etc).
//! Range: 0x6dd434..0x6e6d30 | core candidates remaining 10908 before batch | distinct check vs global stubs
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled/mangled")] + pub fn stub_0x<ADDR>() { todo!("0x<ADDR>") }
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, single quotes and backticks removed, backslashes normalized.
//! Model: muse-spark

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x6dd434 — __ZThn36_N3RBX13ArrowToolBaseD0Ev
// type: void __fastcall(RBX::ArrowToolBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArrowToolBase::~ArrowToolBase()")]
#[doc(alias = "__ZThn36_N3RBX13ArrowToolBaseD0Ev")]
pub fn stub_0x6dd434() {
    // IDA 0x6dd434: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6de750 — __ZNK3RBX5World26getEnvironmentSpeedPercentEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getEnvironmentSpeedPercent(void)const")]
#[doc(alias = "__ZNK3RBX5World26getEnvironmentSpeedPercentEv")]
pub fn stub_0x6de750() {
    // IDA 0x6de750: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6de954 — __ZNK3RBX5World16getNumPrimitivesEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumPrimitives(void)const")]
#[doc(alias = "__ZNK3RBX5World16getNumPrimitivesEv")]
pub fn stub_0x6de954() {
    // IDA 0x6de954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6de958 — __ZNK3RBX5World12getNumJointsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumJoints(void)const")]
#[doc(alias = "__ZNK3RBX5World12getNumJointsEv")]
pub fn stub_0x6de958() {
    // IDA 0x6de958: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6de95c — __ZNK3RBX5World14getNumContactsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumContacts(void)const")]
#[doc(alias = "__ZNK3RBX5World14getNumContactsEv")]
pub fn stub_0x6de95c() {
    // IDA 0x6de95c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x6de960 — __ZNK3RBX5World15getNumLinkCallsEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::getNumLinkCalls(void)const")]
#[doc(alias = "__ZNK3RBX5World15getNumLinkCallsEv")]
pub fn stub_0x6de960() {
    // IDA 0x6de960: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x6ded30 — __ZN3RBX5Stats14TypedStatsItemIiE6updateEv
#[doc(alias = "RBX::Stats::TypedStatsItem<int>::update(void)")]
#[doc(alias = "__ZN3RBX5Stats14TypedStatsItemIiE6updateEv")]
pub fn stub_0x6ded30() {
    // IDA 0x6ded30: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x6ded50 — __ZThn32_N3RBX5Stats14TypedStatsItemIiED1Ev
#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
#[doc(alias = "__ZThn32_N3RBX5Stats14TypedStatsItemIiED1Ev")]
pub fn stub_0x6ded50() {
    // IDA 0x6ded50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6dee98 — __ZThn32_N3RBX5Stats14TypedStatsItemIiED0Ev
#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
#[doc(alias = "__ZThn32_N3RBX5Stats14TypedStatsItemIiED0Ev")]
pub fn stub_0x6dee98() {
    // IDA 0x6dee98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6deff8 — __ZThn36_N3RBX5Stats14TypedStatsItemIiED1Ev
#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
#[doc(alias = "__ZThn36_N3RBX5Stats14TypedStatsItemIiED1Ev")]
pub fn stub_0x6deff8() {
    // IDA 0x6deff8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6df140 — __ZThn36_N3RBX5Stats14TypedStatsItemIiED0Ev
#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
#[doc(alias = "__ZThn36_N3RBX5Stats14TypedStatsItemIiED0Ev")]
pub fn stub_0x6df140() {
    // IDA 0x6df140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6df7f8 — __ZN3RBX5Stats14TypedStatsItemIfE6updateEv
#[doc(alias = "RBX::Stats::TypedStatsItem<float>::update(void)")]
#[doc(alias = "__ZN3RBX5Stats14TypedStatsItemIfE6updateEv")]
pub fn stub_0x6df7f8() {
    // IDA 0x6df7f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6df818 — __ZThn32_N3RBX5Stats14TypedStatsItemIfED0Ev
#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
#[doc(alias = "__ZThn32_N3RBX5Stats14TypedStatsItemIfED0Ev")]
pub fn stub_0x6df818() {
    // IDA 0x6df818: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6dfe88 — __ZN3RBX5Stats14TypedStatsItemIdED1Ev
#[doc(alias = "RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
#[doc(alias = "__ZN3RBX5Stats14TypedStatsItemIdED1Ev")]
pub fn stub_0x6dfe88() {
    // IDA 0x6dfe88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6dffd0 — __ZN3RBX5Stats14TypedStatsItemIdED0Ev
#[doc(alias = "RBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
#[doc(alias = "__ZN3RBX5Stats14TypedStatsItemIdED0Ev")]
pub fn stub_0x6dffd0() {
    // IDA 0x6dffd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e0130 — __ZThn32_N3RBX5Stats14TypedStatsItemIdED1Ev
#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
#[doc(alias = "__ZThn32_N3RBX5Stats14TypedStatsItemIdED1Ev")]
pub fn stub_0x6e0130() {
    // IDA 0x6e0130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e0278 — __ZThn32_N3RBX5Stats14TypedStatsItemIdED0Ev
#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<double>::~TypedStatsItem()")]
#[doc(alias = "__ZThn32_N3RBX5Stats14TypedStatsItemIdED0Ev")]
pub fn stub_0x6e0278() {
    // IDA 0x6e0278: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e12c0 — __ZN3RBX9GuiTarget7processERKNS_8GuiEventE
#[doc(alias = "RBX::GuiTarget::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX9GuiTarget7processERKNS_8GuiEventE")]
pub fn stub_0x6e12c0() {
    // IDA 0x6e12c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e205c — __ZN3RBX4BodyC1Ev
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::Body(void)")]
#[doc(alias = "__ZN3RBX4BodyC1Ev")]
pub fn stub_0x6e205c() {
    // IDA 0x6e205c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e2060 — __ZN3RBX4BodyC2Ev
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::Body(void)")]
#[doc(alias = "__ZN3RBX4BodyC2Ev")]
pub fn stub_0x6e2060() {
    // IDA 0x6e2060: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e2244 — __ZN3RBX4Body17getNextStateIndexEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getNextStateIndex(void)")]
#[doc(alias = "__ZN3RBX4Body17getNextStateIndexEv")]
pub fn stub_0x6e2244() {
    // IDA 0x6e2244: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e2258 — __ZN3RBX4BodyD0Ev
// type: void __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::~Body()")]
#[doc(alias = "__ZN3RBX4BodyD0Ev")]
pub fn stub_0x6e2258() {
    // IDA 0x6e2258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e230c — __ZN3RBX4BodyD1Ev
// type: void __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::~Body()")]
#[doc(alias = "__ZN3RBX4BodyD1Ev")]
pub fn stub_0x6e230c() {
    // IDA 0x6e230c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e2310 — __ZN3RBX4BodyD2Ev
// type: void __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::~Body()")]
#[doc(alias = "__ZN3RBX4BodyD2Ev")]
pub fn stub_0x6e2310() {
    // IDA 0x6e2310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e264c — __ZN3RBX4Body17advanceStateIndexEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::advanceStateIndex(void)")]
#[doc(alias = "__ZN3RBX4Body17advanceStateIndexEv")]
pub fn stub_0x6e264c() {
    // IDA 0x6e264c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e2664 — __ZN3RBX4Body14initStaticDataEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::initStaticData(void)")]
#[doc(alias = "__ZN3RBX4Body14initStaticDataEv")]
pub fn stub_0x6e2664() {
    // IDA 0x6e2664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e2724 — __ZN3RBX4Body12getWorldBodyEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getWorldBody(void)")]
#[doc(alias = "__ZN3RBX4Body12getWorldBodyEv")]
pub fn stub_0x6e2724() {
    // IDA 0x6e2724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e2750 — __ZN3RBX4Body23validateParentCofmDirtyEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::validateParentCofmDirty(void)")]
#[doc(alias = "__ZN3RBX4Body23validateParentCofmDirtyEv")]
pub fn stub_0x6e2750() {
    // IDA 0x6e2750: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e27fc — __ZN3RBX4Body13makeCofmDirtyEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::makeCofmDirty(void)")]
#[doc(alias = "__ZN3RBX4Body13makeCofmDirtyEv")]
pub fn stub_0x6e27fc() {
    // IDA 0x6e27fc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e2a0c — __ZN3RBX4Body9resetRootEPS0_
// type: _DWORD __fastcall(RBX::Body *__hidden this, RBX::Body *)
#[doc(alias = "RBX::Body::resetRoot(RBX::Body*)")]
#[doc(alias = "__ZN3RBX4Body9resetRootEPS0_")]
pub fn stub_0x6e2a0c() {
    // IDA 0x6e2a0c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e2aec — __ZN3RBX4Body16onParentChangingEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::onParentChanging(void)")]
#[doc(alias = "__ZN3RBX4Body16onParentChangingEv")]
pub fn stub_0x6e2aec() {
    // IDA 0x6e2aec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e2d10 — __ZN3RBX4Body15onParentChangedEPNS_11IndexedTreeE
// type: _DWORD __fastcall(RBX::Body *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::Body::onParentChanged(RBX::IndexedTree *)")]
#[doc(alias = "__ZN3RBX4Body15onParentChangedEPNS_11IndexedTreeE")]
pub fn stub_0x6e2d10() {
    // IDA 0x6e2d10: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e2e04 — __ZN3RBX4Body13onChildAddingEPNS_11IndexedTreeE
// type: _DWORD __fastcall(RBX::Body *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::Body::onChildAdding(RBX::IndexedTree *)")]
#[doc(alias = "__ZN3RBX4Body13onChildAddingEPNS_11IndexedTreeE")]
pub fn stub_0x6e2e04() {
    // IDA 0x6e2e04: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e2e08 — __ZN3RBX4Body11refreshCofmEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::refreshCofm(void)")]
#[doc(alias = "__ZN3RBX4Body11refreshCofmEv")]
pub fn stub_0x6e2e08() {
    // IDA 0x6e2e08: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3000 — __ZN3RBX4Body12onChildAddedEPNS_11IndexedTreeE
// type: _DWORD __fastcall(RBX::Body *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::Body::onChildAdded(RBX::IndexedTree *)")]
#[doc(alias = "__ZN3RBX4Body12onChildAddedEPNS_11IndexedTreeE")]
pub fn stub_0x6e3000() {
    // IDA 0x6e3000: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e30b0 — __ZN3RBX4Body14onChildRemovedEPNS_11IndexedTreeE
// type: _DWORD __fastcall(RBX::Body *__hidden this, RBX::IndexedTree *)
#[doc(alias = "RBX::Body::onChildRemoved(RBX::IndexedTree *)")]
#[doc(alias = "__ZN3RBX4Body14onChildRemovedEPNS_11IndexedTreeE")]
pub fn stub_0x6e30b0() {
    // IDA 0x6e30b0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3220 — __ZN3RBX4Body13setMeInParentEPNS_4LinkE
// type: _DWORD __fastcall(RBX::Body *__hidden this, RBX::Link *)
#[doc(alias = "RBX::Body::setMeInParent(RBX::Link *)")]
#[doc(alias = "__ZN3RBX4Body13setMeInParentEPNS_4LinkE")]
pub fn stub_0x6e3220() {
    // IDA 0x6e3220: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e32f0 — __ZN3RBX4Body5setPvERKNS_2PVERKNS_12BodyPvSetterE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Body::setPv(RBX::PV const&,RBX::BodyPvSetter const&)")]
#[doc(alias = "__ZN3RBX4Body5setPvERKNS_2PVERKNS_12BodyPvSetterE")]
pub fn stub_0x6e32f0() {
    // IDA 0x6e32f0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e33c8 — __ZN3RBX4Body11setVelocityERKNS_8VelocityERKNS_12BodyPvSetterE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Body::setVelocity(RBX::Velocity const&,RBX::BodyPvSetter const&)")]
#[doc(alias = "__ZN3RBX4Body11setVelocityERKNS_8VelocityERKNS_12BodyPvSetterE")]
pub fn stub_0x6e33c8() {
    // IDA 0x6e33c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3414 — __ZN3RBX4Body14setCanThrottleEbRKNS_12BodyPvSetterE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Body::setCanThrottle(bool,RBX::BodyPvSetter const&)")]
#[doc(alias = "__ZN3RBX4Body14setCanThrottleEbRKNS_12BodyPvSetterE")]
pub fn stub_0x6e3414() {
    // IDA 0x6e3414: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e341c — __ZN3RBX4Body7setMassEf
// type: _DWORD __fastcall(RBX::Body *__hidden this, float)
#[doc(alias = "RBX::Body::setMass(float)")]
#[doc(alias = "__ZN3RBX4Body7setMassEf")]
pub fn stub_0x6e341c() {
    // IDA 0x6e341c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3554 — __ZN3RBX4Body16getBranchCofmPosEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getBranchCofmPos(void)")]
#[doc(alias = "__ZN3RBX4Body16getBranchCofmPosEv")]
pub fn stub_0x6e3554() {
    // IDA 0x6e3554: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3618 — __ZN3RBX4Body19getBranchCofmOffsetEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getBranchCofmOffset(void)")]
#[doc(alias = "__ZN3RBX4Body19getBranchCofmOffsetEv")]
pub fn stub_0x6e3618() {
    // IDA 0x6e3618: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3710 — __ZN3RBX4Body28getBranchCofmCoordinateFrameEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getBranchCofmCoordinateFrame(void)")]
#[doc(alias = "__ZN3RBX4Body28getBranchCofmCoordinateFrameEv")]
pub fn stub_0x6e3710() {
    // IDA 0x6e3710: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3744 — __ZN3RBX4Body13kineticEnergyEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::kineticEnergy(void)")]
#[doc(alias = "__ZN3RBX4Body13kineticEnergyEv")]
pub fn stub_0x6e3744() {
    // IDA 0x6e3744: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3844 — __ZN3RBX9AllocatorINS_4BodyEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::Body>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4BodyEEC2Ev")]
pub fn stub_0x6e3844() {
    // IDA 0x6e3844: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e38a8 — __ZN3RBX9AllocatorINS_7SimBodyEEnwEm
#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_7SimBodyEEnwEm")]
pub fn stub_0x6e38a8() {
    // IDA 0x6e38a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3918 — __ZN3RBX9AllocatorINS_7SimBodyEEdlEPv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Allocator<RBX::SimBody>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_7SimBodyEEdlEPv")]
pub fn stub_0x6e3918() {
    // IDA 0x6e3918: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3958 — __ZN3RBX9AllocatorINS_4BodyEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::Body>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4BodyEEdlEPv")]
pub fn stub_0x6e3958() {
    // IDA 0x6e3958: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3998 — __ZN3RBX9AllocatorINS_4CofmEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4CofmEEdlEPv")]
pub fn stub_0x6e3998() {
    // IDA 0x6e3998: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e39d4 — __ZN3RBX9AllocatorINS_4CofmEEnwEm
#[doc(alias = "RBX::Allocator<RBX::Cofm>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4CofmEEnwEm")]
pub fn stub_0x6e39d4() {
    // IDA 0x6e39d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3a44 — __ZN3RBX4Body9getIWorldEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getIWorld(void)")]
#[doc(alias = "__ZN3RBX4Body9getIWorldEv")]
pub fn stub_0x6e3a44() {
    // IDA 0x6e3a44: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3a70 — __ZN3RBX4Body15getBranchIWorldEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getBranchIWorld(void)")]
#[doc(alias = "__ZN3RBX4Body15getBranchIWorldEv")]
pub fn stub_0x6e3a70() {
    // IDA 0x6e3a70: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3ba4 — __ZN3RBX9AllocatorINS_4BodyEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::Body>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4BodyEE13releaseMemoryEv")]
pub fn stub_0x6e3ba4() {
    // IDA 0x6e3ba4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3d88 — __ZN3RBX4CofmC1EPNS_4BodyE
// type: _DWORD __fastcall(RBX::Cofm *__hidden this, RBX::Body *)
#[doc(alias = "RBX::Cofm::Cofm(RBX::Body *)")]
#[doc(alias = "__ZN3RBX4CofmC1EPNS_4BodyE")]
pub fn stub_0x6e3d88() {
    // IDA 0x6e3d88: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e3db0 — __ZN3RBX4Cofm13updateIfDirtyEv
// type: _DWORD __fastcall(RBX::Cofm *__hidden this)
#[doc(alias = "RBX::Cofm::updateIfDirty(void)")]
#[doc(alias = "__ZN3RBX4Cofm13updateIfDirtyEv")]
pub fn stub_0x6e3db0() {
    // IDA 0x6e3db0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4370 — __ZN3RBX9AllocatorINS_4CofmEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::Cofm>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4CofmEEC2Ev")]
pub fn stub_0x6e4370() {
    // IDA 0x6e4370: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e43d4 — __ZN3RBX9AllocatorINS_4CofmEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::Cofm>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4CofmEE13releaseMemoryEv")]
pub fn stub_0x6e43d4() {
    // IDA 0x6e43d4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4550 — __ZN3RBX9Connector18computeCanThrottleEv
// type: _DWORD __fastcall(RBX::Connector *__hidden this)
#[doc(alias = "RBX::Connector::computeCanThrottle(void)")]
#[doc(alias = "__ZN3RBX9Connector18computeCanThrottleEv")]
pub fn stub_0x6e4550() {
    // IDA 0x6e4550: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4584 — __ZN3RBX26PointToPointBreakConnector7getBodyENS_9Connector9BodyIndexE
#[doc(alias = "RBX::PointToPointBreakConnector::getBody(RBX::Connector::BodyIndex)")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnector7getBodyENS_9Connector9BodyIndexE")]
pub fn stub_0x6e4584() {
    // IDA 0x6e4584: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e46ec — __ZN3RBX15RotateConnector5resetEv
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this)
#[doc(alias = "RBX::RotateConnector::reset(void)")]
#[doc(alias = "__ZN3RBX15RotateConnector5resetEv")]
pub fn stub_0x6e46ec() {
    // IDA 0x6e46ec: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4760 — __ZN3RBX15RotateConnector7getBodyENS_9Connector9BodyIndexE
#[doc(alias = "RBX::RotateConnector::getBody(RBX::Connector::BodyIndex)")]
#[doc(alias = "__ZN3RBX15RotateConnector7getBodyENS_9Connector9BodyIndexE")]
pub fn stub_0x6e4760() {
    // IDA 0x6e4760: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e48a8 — __ZN3RBX15RotateConnector17setRotationalGoalEf
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, float)
#[doc(alias = "RBX::RotateConnector::setRotationalGoal(float)")]
#[doc(alias = "__ZN3RBX15RotateConnector17setRotationalGoalEf")]
pub fn stub_0x6e48a8() {
    // IDA 0x6e48a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e49c0 — __ZN3RBX15RotateConnector15setVelocityGoalEf
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, float)
#[doc(alias = "RBX::RotateConnector::setVelocityGoal(float)")]
#[doc(alias = "__ZN3RBX15RotateConnector15setVelocityGoalEf")]
pub fn stub_0x6e49c0() {
    // IDA 0x6e49c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4a10 — __ZN3RBX15RotateConnector9stepGoalsEv
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this)
#[doc(alias = "RBX::RotateConnector::stepGoals(void)")]
#[doc(alias = "__ZN3RBX15RotateConnector9stepGoalsEv")]
pub fn stub_0x6e4a10() {
    // IDA 0x6e4a10: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4a4c — __ZN3RBX15RotateConnector12computeForceEb
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, bool)
#[doc(alias = "RBX::RotateConnector::computeForce(bool)")]
#[doc(alias = "__ZN3RBX15RotateConnector12computeForceEb")]
pub fn stub_0x6e4a4c() {
    // IDA 0x6e4a4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4b4c — __ZN3RBX26PointToPointBreakConnector15potentialEnergyEv
// type: _DWORD __fastcall(RBX::PointToPointBreakConnector *__hidden this)
#[doc(alias = "RBX::PointToPointBreakConnector::potentialEnergy(void)")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnector15potentialEnergyEv")]
pub fn stub_0x6e4b4c() {
    // IDA 0x6e4b4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4ba8 — __ZN3RBX26PointToPointBreakConnector12computeForceEb
// type: _DWORD __fastcall(RBX::PointToPointBreakConnector *__hidden this, bool)
#[doc(alias = "RBX::PointToPointBreakConnector::computeForce(bool)")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnector12computeForceEb")]
pub fn stub_0x6e4ba8() {
    // IDA 0x6e4ba8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4c68 — __ZN3RBX20NormalBreakConnector12computeForceEb
// type: _DWORD __fastcall(RBX::NormalBreakConnector *__hidden this, bool)
#[doc(alias = "RBX::NormalBreakConnector::computeForce(bool)")]
#[doc(alias = "__ZN3RBX20NormalBreakConnector12computeForceEb")]
pub fn stub_0x6e4c68() {
    // IDA 0x6e4c68: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4d78 — __ZN3RBX20NormalBreakConnectorD1Ev
// type: void __fastcall(RBX::NormalBreakConnector *__hidden this)
#[doc(alias = "RBX::NormalBreakConnector::~NormalBreakConnector()")]
#[doc(alias = "__ZN3RBX20NormalBreakConnectorD1Ev")]
pub fn stub_0x6e4d78() {
    // IDA 0x6e4d78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e4d7c — __ZN3RBX20NormalBreakConnectorD0Ev
// type: void __fastcall(RBX::NormalBreakConnector *__hidden this)
#[doc(alias = "RBX::NormalBreakConnector::~NormalBreakConnector()")]
#[doc(alias = "__ZN3RBX20NormalBreakConnectorD0Ev")]
pub fn stub_0x6e4d7c() {
    // IDA 0x6e4d7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e4d80 — __ZN3RBX15RotateConnectorD1Ev
// type: void __fastcall(RBX::RotateConnector *__hidden this)
#[doc(alias = "RBX::RotateConnector::~RotateConnector()")]
#[doc(alias = "__ZN3RBX15RotateConnectorD1Ev")]
pub fn stub_0x6e4d80() {
    // IDA 0x6e4d80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e4d84 — __ZN3RBX15RotateConnectorD0Ev
// type: void __fastcall(RBX::RotateConnector *__hidden this)
#[doc(alias = "RBX::RotateConnector::~RotateConnector()")]
#[doc(alias = "__ZN3RBX15RotateConnectorD0Ev")]
pub fn stub_0x6e4d84() {
    // IDA 0x6e4d84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e4d88 — __ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_20NormalBreakConnectorEEdlEPv")]
pub fn stub_0x6e4d88() {
    // IDA 0x6e4d88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e4ef4 — __ZN3RBX9Constants17longUiStepsPerSecEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::longUiStepsPerSec(void)")]
#[doc(alias = "__ZN3RBX9Constants17longUiStepsPerSecEv")]
pub fn stub_0x6e4ef4() {
    // IDA 0x6e4ef4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e4ef8 — __ZN3RBX9Constants23worldStepsPerLongUiStepEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::worldStepsPerLongUiStep(void)")]
#[doc(alias = "__ZN3RBX9Constants23worldStepsPerLongUiStepEv")]
pub fn stub_0x6e4ef8() {
    // IDA 0x6e4ef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6e4efc — __ZN3RBX9Constants13uiStepsPerSecEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::uiStepsPerSec(void)")]
#[doc(alias = "__ZN3RBX9Constants13uiStepsPerSecEv")]
pub fn stub_0x6e4efc() {
    // IDA 0x6e4efc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e4f14 — __ZN3RBX9Constants19worldStepsPerUiStepEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::worldStepsPerUiStep(void)")]
#[doc(alias = "__ZN3RBX9Constants19worldStepsPerUiStepEv")]
pub fn stub_0x6e4f14() {
    // IDA 0x6e4f14: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x6e4f2c — __ZN3RBX9Constants23kernelStepsPerWorldStepEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::kernelStepsPerWorldStep(void)")]
#[doc(alias = "__ZN3RBX9Constants23kernelStepsPerWorldStepEv")]
pub fn stub_0x6e4f2c() {
    // IDA 0x6e4f2c: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x6e4f30 — __ZN3RBX9Constants16worldStepsPerSecEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::worldStepsPerSec(void)")]
#[doc(alias = "__ZN3RBX9Constants16worldStepsPerSecEv")]
pub fn stub_0x6e4f30() {
    // IDA 0x6e4f30: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x6e4f54 — __ZN3RBX9Constants26impulseSolverMaxIterationsEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::impulseSolverMaxIterations(void)")]
#[doc(alias = "__ZN3RBX9Constants26impulseSolverMaxIterationsEv")]
pub fn stub_0x6e4f54() {
    // IDA 0x6e4f54: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x6e4f58 — __ZN3RBX9Constants21impulseSolverAccuracyEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::impulseSolverAccuracy(void)")]
#[doc(alias = "__ZN3RBX9Constants21impulseSolverAccuracyEv")]
pub fn stub_0x6e4f58() {
    // IDA 0x6e4f58: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x6e4f64 — __ZN3RBX9Constants27impulseSolverAccuracyScalarEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::impulseSolverAccuracyScalar(void)")]
#[doc(alias = "__ZN3RBX9Constants27impulseSolverAccuracyScalarEv")]
pub fn stub_0x6e4f64() {
    // IDA 0x6e4f64: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x6e4f6c — __ZN3RBX9Constants32impulseSolverSymStateTorqueBoundEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::impulseSolverSymStateTorqueBound(void)")]
#[doc(alias = "__ZN3RBX9Constants32impulseSolverSymStateTorqueBoundEv")]
pub fn stub_0x6e4f6c() {
    // IDA 0x6e4f6c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x6e4f78 — __ZN3RBX9Constants31impulseSolverSymStateForceBoundEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::impulseSolverSymStateForceBound(void)")]
#[doc(alias = "__ZN3RBX9Constants31impulseSolverSymStateForceBoundEv")]
pub fn stub_0x6e4f78() {
    // IDA 0x6e4f78: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x6e4f84 — __ZN3RBX9Constants4uiDtEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::uiDt(void)")]
#[doc(alias = "__ZN3RBX9Constants4uiDtEv")]
pub fn stub_0x6e4f84() {
    // IDA 0x6e4f84: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x6e4fb4 — __ZN3RBX9Constants12longUiStepDtEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::longUiStepDt(void)")]
#[doc(alias = "__ZN3RBX9Constants12longUiStepDtEv")]
pub fn stub_0x6e4fb4() {
    // IDA 0x6e4fb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x6e4fc0 — __ZN3RBX9Constants7worldDtEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::worldDt(void)")]
#[doc(alias = "__ZN3RBX9Constants7worldDtEv")]
pub fn stub_0x6e4fc0() {
    // IDA 0x6e4fc0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

// 0x6e4ff8 — __ZN3RBX9Constants8kernelDtEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::kernelDt(void)")]
#[doc(alias = "__ZN3RBX9Constants8kernelDtEv")]
pub fn stub_0x6e4ff8() {
    // IDA 0x6e4ff8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

// 0x6e5030 — __ZN3RBX9Constants10freeFallDtEv
// type: _DWORD __fastcall(RBX::Constants *__hidden this)
#[doc(alias = "RBX::Constants::freeFallDt(void)")]
#[doc(alias = "__ZN3RBX9Constants10freeFallDtEv")]
pub fn stub_0x6e5030() {
    // IDA 0x6e5030: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

// 0x6e5068 — __ZN3RBX9Constants20getElasticMultiplierEf
// type: _DWORD __fastcall(RBX::Constants *__hidden this, float)
#[doc(alias = "RBX::Constants::getElasticMultiplier(float)")]
#[doc(alias = "__ZN3RBX9Constants20getElasticMultiplierEf")]
pub fn stub_0x6e5068() {
    // IDA 0x6e5068: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

// 0x6e50e8 — __ZN3RBX9Constants19getKmsMaxJointForceEff
// type: _DWORD __fastcall(RBX::Constants *__hidden this, float, float)
#[doc(alias = "RBX::Constants::getKmsMaxJointForce(float,float)")]
#[doc(alias = "__ZN3RBX9Constants19getKmsMaxJointForceEff")]
pub fn stub_0x6e50e8() {
    // IDA 0x6e50e8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x6e5798 — __ZN3RBX16ContactConnector13percentActiveEv
// type: _DWORD __fastcall(RBX::ContactConnector *__hidden this)
#[doc(alias = "RBX::ContactConnector::percentActive(void)")]
#[doc(alias = "__ZN3RBX16ContactConnector13percentActiveEv")]
pub fn stub_0x6e5798() {
    // IDA 0x6e5798: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e59b4 — __ZN3RBX16ContactConnector23computeRelativeVelocityEv
// type: _DWORD __fastcall(RBX::ContactConnector *__hidden this)
#[doc(alias = "RBX::ContactConnector::computeRelativeVelocity(void)")]
#[doc(alias = "__ZN3RBX16ContactConnector23computeRelativeVelocityEv")]
pub fn stub_0x6e59b4() {
    // IDA 0x6e59b4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e59d8 — __ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RPNS_4BodyERNS_10PairParamsE
#[doc(alias = "RBX::ContactConnector::getReordedSimBody(RBX::SimBody *&,RBX::SimBody *&,RBX::Body *&,RBX::PairParams &)")]
#[doc(alias = "__ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RPNS_4BodyERNS_10PairParamsE")]
pub fn stub_0x6e59d8() {
    // IDA 0x6e59d8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e5b1c — __ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RNS_10PairParamsE
#[doc(alias = "RBX::ContactConnector::getReordedSimBody(RBX::SimBody *&,RBX::SimBody *&,RBX::PairParams &)")]
#[doc(alias = "__ZN3RBX16ContactConnector17getReordedSimBodyERPNS_7SimBodyES3_RNS_10PairParamsE")]
pub fn stub_0x6e5b1c() {
    // IDA 0x6e5b1c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e5db0 — __ZN3RBX16ContactConnector12computeForceEb
// type: _DWORD __fastcall(RBX::ContactConnector *__hidden this, bool)
#[doc(alias = "RBX::ContactConnector::computeForce(bool)")]
#[doc(alias = "__ZN3RBX16ContactConnector12computeForceEb")]
pub fn stub_0x6e5db0() {
    // IDA 0x6e5db0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e629c — __ZN3RBX16ContactConnector14computeImpulseERf
// type: _DWORD __fastcall(RBX::ContactConnector *__hidden this, float *)
#[doc(alias = "RBX::ContactConnector::computeImpulse(float &)")]
#[doc(alias = "__ZN3RBX16ContactConnector14computeImpulseERf")]
pub fn stub_0x6e629c() {
    // IDA 0x6e629c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e6b30 — __ZN3RBX16ContactConnector37applyContactPointForSymmetryDetectionEPNS_7SimBodyES2_RKNS_10PairParamsEf
// type: int __fastcall(int, int, int, int, float)
#[doc(alias = "RBX::ContactConnector::applyContactPointForSymmetryDetection(RBX::SimBody *,RBX::SimBody *,RBX::PairParams const&,float)")]
#[doc(alias = "__ZN3RBX16ContactConnector37applyContactPointForSymmetryDetectionEPNS_7SimBodyES2_RKNS_10PairParamsEf")]
pub fn stub_0x6e6b30() {
    // IDA 0x6e6b30: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x6e6d30 — __ZN3RBX16ContactConnector18updateContactPointEv
// type: _DWORD __fastcall(RBX::ContactConnector *__hidden this)
#[doc(alias = "RBX::ContactConnector::updateContactPoint(void)")]
#[doc(alias = "__ZN3RBX16ContactConnector18updateContactPointEv")]
pub fn stub_0x6e6d30() {
    // IDA 0x6e6d30: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
