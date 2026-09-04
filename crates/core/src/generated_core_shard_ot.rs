//! core shard ot — 100 core stubs EA-sorted, 0x8a6bd0..0x8e4134 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::HUMAN::Swimming::onComputeForceImpl(void)")]
// 0x8a6bd0 — __ZN3RBX5HUMAN8Swimming18onComputeForceImplEv
// type: RBX::Body *__fastcall(RBX::HUMAN::Swimming *this)
pub fn stub_0x8a6bd0() {
    // IDA 0x8a6bd0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Swimming::onSimulatorStepImpl(float)")]
// 0x8a7118 — __ZN3RBX5HUMAN8Swimming19onSimulatorStepImplEf
// type: RBX::Velocity *__fastcall(RBX::HUMAN::Swimming *this, float32_t)
pub fn stub_0x8a7118() {
    // IDA 0x8a7118: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Swimming::fireEvents(void)")]
// 0x8a7238 — __ZN3RBX5HUMAN8Swimming10fireEventsEv
// type: int __fastcall(RBX::HUMAN::Swimming *this)
pub fn stub_0x8a7238() {
    // IDA 0x8a7238: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Swimming::~Swimming()")]
// 0x8a7374 — __ZN3RBX5HUMAN8SwimmingD1Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
pub fn stub_0x8a7374() {
    // IDA 0x8a7374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::~Swimming()")]
// 0x8a7378 — __ZN3RBX5HUMAN8SwimmingD0Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
pub fn stub_0x8a7378() {
    // IDA 0x8a7378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::getStateType(void)const")]
// 0x8a7418 — __ZNK3RBX5HUMAN8Swimming12getStateTypeEv
// type: int __fastcall(RBX::HUMAN::Swimming *this)
pub fn stub_0x8a7418() {
    // IDA 0x8a7418: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::Swimming::~Swimming()")]
// 0x8a741c — __ZThn4_N3RBX5HUMAN8SwimmingD1Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
pub fn stub_0x8a741c() {
    // IDA 0x8a741c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HUMAN::Swimming::~Swimming()")]
// 0x8a7424 — __ZThn4_N3RBX5HUMAN8SwimmingD0Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
pub fn stub_0x8a7424() {
    // IDA 0x8a7424: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::UintSet(void)")]
// 0x8a781c — __ZN3RBX7UintSetC1Ev
// type: int __fastcall(int this)
pub fn stub_0x8a781c() {
    // IDA 0x8a781c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::size(void)const")]
// 0x8a7840 — __ZNK3RBX7UintSet4sizeEv
// type: int __fastcall(RBX::UintSet *this)
pub fn stub_0x8a7840() {
    // IDA 0x8a7840: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::insert(unsigned int)")]
// 0x8a7844 — __ZN3RBX7UintSet6insertEj
// type: _DWORD __fastcall(RBX::UintSet *__hidden this, unsigned int)
pub fn stub_0x8a7844() {
    // IDA 0x8a7844: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::contains(unsigned int)")]
// 0x8a7948 — __ZN3RBX7UintSet8containsEj
// type: bool __fastcall(RBX::UintSet *this, unsigned int)
pub fn stub_0x8a7948() {
    // IDA 0x8a7948: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::pop_smallest(unsigned int *)")]
// 0x8a798c — __ZN3RBX7UintSet12pop_smallestEPj
// type: unsigned int __fastcall(RBX::UintSet *this, unsigned int *, int)
pub fn stub_0x8a798c() {
    // IDA 0x8a798c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::pop_front(unsigned int *)")]
// 0x8a7b90 — __ZN3RBX17DoubleEndedVectorIjE9pop_frontEPj
// type: int __fastcall(int *, _DWORD *, int)
pub fn stub_0x8a7b90() {
    // IDA 0x8a7b90: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::grow(void)")]
// 0x8a7c00 — __ZN3RBX17DoubleEndedVectorIjE4growEv
// type: void __fastcall(int *, int, int, int, int, int, int, void *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x8a7c00() {
    // IDA 0x8a7c00: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::~ManualJointHelper()")]
// 0x8a7e34 — __ZN3RBX17ManualJointHelperD1Ev
// type: void __fastcall(RBX::ManualJointHelper *__hidden this)
pub fn stub_0x8a7e34() {
    // IDA 0x8a7e34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::~ManualJointHelper()")]
// 0x8a7e38 — __ZN3RBX17ManualJointHelperD2Ev
// type: void __fastcall(RBX::ManualJointHelper *__hidden this)
pub fn stub_0x8a7e38() {
    // IDA 0x8a7e38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::ManualJointHelper(void)")]
// 0x8a80e4 — __ZN3RBX17ManualJointHelperC1Ev
// type: int __fastcall(int this)
pub fn stub_0x8a80e4() {
    // IDA 0x8a80e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::clearAndDeleteJointSurfacePairs(void)")]
// 0x8a8134 — __ZN3RBX17ManualJointHelper31clearAndDeleteJointSurfacePairsEv
// type: int __fastcall(int this)
pub fn stub_0x8a8134() {
    // IDA 0x8a8134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::findPermissibleJointSurfacePairs(void)")]
// 0x8a816c — __ZN3RBX17ManualJointHelper32findPermissibleJointSurfacePairsEv
// type: void __fastcall(RBX::ManualJointHelper *this)
pub fn stub_0x8a816c() {
    // IDA 0x8a816c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::createJointSurfacePair(RBX::Primitive &,unsigned long &,RBX::Primitive &,unsigned long &)")]
// 0x8a8478 — __ZN3RBX17ManualJointHelper22createJointSurfacePairERNS_9PrimitiveERmS2_S3_
// type: void __fastcall(struct _Unwind_Exception *this, RBX::Primitive *, const RBX::Primitive *, RBX::Primitive *, const RBX::Primitive *)
pub fn stub_0x8a8478() {
    // IDA 0x8a8478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::createJoints(void)")]
// 0x8a9070 — __ZN3RBX17ManualJointHelper12createJointsEv
// type: int __fastcall(RBX::ManualJointHelper *this)
pub fn stub_0x8a9070() {
    // IDA 0x8a9070: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::createJointsIfEnabledFromGui(void)")]
// 0x8a909c — __ZN3RBX17ManualJointHelper28createJointsIfEnabledFromGuiEv
// type: int __fastcall(RBX::ManualJointHelper *this)
pub fn stub_0x8a909c() {
    // IDA 0x8a909c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::render3dAdorn(RBX::Adorn *)")]
// 0x8a90d8 — __ZN3RBX17ManualJointHelper13render3dAdornEPNS_5AdornE
// type: int __fastcall(RBX::ManualJointHelper *this, RBX::Adorn *)
pub fn stub_0x8a90d8() {
    // IDA 0x8a90d8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::StudAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a911c — __ZN3RBX24StudAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(RBX::StudAutoJointSurfacePair *this, RBX::Adorn *)
pub fn stub_0x8a911c() {
    // IDA 0x8a911c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::GlueAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9430 — __ZN3RBX24GlueAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(RBX::GlueAutoJointSurfacePair *this, RBX::Adorn *)
pub fn stub_0x8a9430() {
    // IDA 0x8a9430: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WeldAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9744 — __ZN3RBX24WeldAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(RBX::WeldAutoJointSurfacePair *this, RBX::Adorn *)
pub fn stub_0x8a9744() {
    // IDA 0x8a9744: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HingeAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9a58 — __ZN3RBX25HingeAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(RBX::HingeAutoJointSurfacePair *this, RBX::Adorn *)
pub fn stub_0x8a9a58() {
    // IDA 0x8a9a58: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DisallowedJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9d6c — __ZN3RBX26DisallowedJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(RBX::DisallowedJointSurfacePair *this, RBX::Adorn *)
pub fn stub_0x8a9d6c() {
    // IDA 0x8a9d6c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8aa080 — __ZN3RBX22ManualJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(RBX::ManualJointSurfacePair *this, RBX::Adorn *)
pub fn stub_0x8aa080() {
    // IDA 0x8aa080: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointSurfacePair::createJoint(void)")]
// 0x8aa3bc — __ZN3RBX22ManualJointSurfacePair11createJointEv
// type: void __fastcall(RBX::ManualJointSurfacePair *this, const RBX::Primitive *)
pub fn stub_0x8aa3bc() {
    // IDA 0x8aa3bc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TerrainManualJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8aafa4 — __ZN3RBX29TerrainManualJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(const G3D::Vector3int16 **this, RBX::Adorn *)
pub fn stub_0x8aafa4() {
    // IDA 0x8aafa4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DisallowedTerrainJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8ab104 — __ZN3RBX33DisallowedTerrainJointSurfacePair11dynamicDrawEPNS_5AdornE
// type: void __fastcall(const G3D::Vector3int16 **this, RBX::Adorn *)
pub fn stub_0x8ab104() {
    // IDA 0x8ab104: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TerrainManualJointSurfacePair::createJoint(void)")]
// 0x8ab238 — __ZN3RBX29TerrainManualJointSurfacePair11createJointEv
// type: void __fastcall(RBX::TerrainManualJointSurfacePair *this)
pub fn stub_0x8ab238() {
    // IDA 0x8ab238: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::push_back(RBX::ConstraintSurfacePair * const&)")]
// 0x8ab7bc — __ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x8ab7bc() {
    // IDA 0x8ab7bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")]
// 0x8ab85c — __ZN3RBX24StudAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::StudAutoJointSurfacePair *__hidden this)
pub fn stub_0x8ab85c() {
    // IDA 0x8ab85c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")]
// 0x8ab860 — __ZN3RBX24StudAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::StudAutoJointSurfacePair *__hidden this)
pub fn stub_0x8ab860() {
    // IDA 0x8ab860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ConstraintSurfacePair::createJoint(void)")]
// 0x8ab900 — __ZN3RBX21ConstraintSurfacePair11createJointEv
// type: void __fastcall(RBX::ConstraintSurfacePair *this)
pub fn stub_0x8ab900() {
    // IDA 0x8ab900: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")]
// 0x8ab904 — __ZN3RBX24WeldAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::WeldAutoJointSurfacePair *__hidden this)
pub fn stub_0x8ab904() {
    // IDA 0x8ab904: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")]
// 0x8ab908 — __ZN3RBX24WeldAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::WeldAutoJointSurfacePair *__hidden this)
pub fn stub_0x8ab908() {
    // IDA 0x8ab908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")]
// 0x8ab9a8 — __ZN3RBX24GlueAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::GlueAutoJointSurfacePair *__hidden this)
pub fn stub_0x8ab9a8() {
    // IDA 0x8ab9a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")]
// 0x8ab9ac — __ZN3RBX24GlueAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::GlueAutoJointSurfacePair *__hidden this)
pub fn stub_0x8ab9ac() {
    // IDA 0x8ab9ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")]
// 0x8aba4c — __ZN3RBX25HingeAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::HingeAutoJointSurfacePair *__hidden this)
pub fn stub_0x8aba4c() {
    // IDA 0x8aba4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")]
// 0x8aba50 — __ZN3RBX25HingeAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::HingeAutoJointSurfacePair *__hidden this)
pub fn stub_0x8aba50() {
    // IDA 0x8aba50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")]
// 0x8abaf0 — __ZN3RBX26DisallowedJointSurfacePairD1Ev
// type: void __fastcall(RBX::DisallowedJointSurfacePair *__hidden this)
pub fn stub_0x8abaf0() {
    // IDA 0x8abaf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")]
// 0x8abaf4 — __ZN3RBX26DisallowedJointSurfacePairD0Ev
// type: void __fastcall(RBX::DisallowedJointSurfacePair *__hidden this)
pub fn stub_0x8abaf4() {
    // IDA 0x8abaf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")]
// 0x8abb94 — __ZN3RBX22ManualJointSurfacePairD1Ev
// type: void __fastcall(RBX::ManualJointSurfacePair *__hidden this)
pub fn stub_0x8abb94() {
    // IDA 0x8abb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")]
// 0x8abb98 — __ZN3RBX22ManualJointSurfacePairD0Ev
// type: void __fastcall(RBX::ManualJointSurfacePair *__hidden this)
pub fn stub_0x8abb98() {
    // IDA 0x8abb98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::operator()(RBX::UserInputService::SwipeDirection)const")]
// 0x8bfd1c — __ZNK5boost9function1IvN3RBX16UserInputService14SwipeDirectionEEclES3_
// type: void __fastcall(_DWORD *, int)
pub fn stub_0x8bfd1c() {
    // IDA 0x8bfd1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<void,RBX::UserInputService::SwipeDirection>::assign_to_own(boost::function1<void,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c0304 — __ZN5boost9function1IvN3RBX16UserInputService14SwipeDirectionEE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
pub fn stub_0x8c0304() {
    // IDA 0x8c0304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType * rbx::any_cast<RBX::MarketplaceService::CurrencyType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x8d40f0 — __ZN3rbx8any_castIN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
pub fn stub_0x8d40f0() {
    // IDA 0x8d40f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType & rbx::any_cast<RBX::MarketplaceService::CurrencyType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8d414c — __ZN3rbx8any_castIRN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x8d414c() {
    // IDA 0x8d414c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::resize(unsigned long,RBX::MarketplaceService::CurrencyType)")]
// 0x8d4240 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
pub fn stub_0x8d4240() {
    // IDA 0x8d4240: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::push_back(RBX::MarketplaceService::CurrencyType const&)")]
// 0x8d4278 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x8d4278() {
    // IDA 0x8d4278: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::MarketplaceService::CurrencyType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::operator[](RBX::Name const* const&)")]
// 0x8d42a4 — __ZNSt3mapIPKN3RBX4NameENS0_18MarketplaceService12CurrencyTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x8d42a4() {
    // IDA 0x8d42a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0x8d42fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0x8d42fc() {
    // IDA 0x8d42fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0x8d43b0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x8d43b0() {
    // IDA 0x8d43b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType> const&)")]
// 0x8d4408 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_0x8d4408() {
    // IDA 0x8d4408: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::MarketplaceService::CurrencyType*,std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>>,RBX::MarketplaceService::CurrencyType const&)")]
// 0x8d4474 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0x8d4474() {
    // IDA 0x8d4474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_allocate(unsigned long)")]
// 0x8d4558 — __ZNSt12_Vector_baseIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x8d4558() {
    // IDA 0x8d4558: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *>(RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *,RBX::MarketplaceService::CurrencyType *)")]
// 0x8d4570 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18MarketplaceService12CurrencyTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
pub fn stub_0x8d4570() {
    // IDA 0x8d4570: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::MarketplaceService::CurrencyType*,std::vector<RBX::MarketplaceService::CurrencyType,std::allocator<RBX::MarketplaceService::CurrencyType>>>,unsigned long,RBX::MarketplaceService::CurrencyType const&)")]
// 0x8d45b0 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x8d45b0() {
    // IDA 0x8d45b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::MarketplaceService::CurrencyType>> *)")]
// 0x8e0950 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18MarketplaceService12CurrencyTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x8e0950() {
    // IDA 0x8e0950: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiBase2d::GuiBase2d(char const*)")]
// 0x8e1134 — __ZN3RBX9GuiBase2dC2EPKc
// type: RBX::GuiBase *__fastcall(RBX::GuiBase2d *this, const char *)
pub fn stub_0x8e1134() {
    // IDA 0x8e1134: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiBase2d::getRect2D(void)const")]
// 0x8e1764 — __ZNK3RBX9GuiBase2d9getRect2DEv
// type: __int64 __fastcall(__int64 this)
pub fn stub_0x8e1764() {
    // IDA 0x8e1764: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiBase2d::getAbsoluteSize(void)const")]
// 0x8e17d4 — __ZNK3RBX9GuiBase2d15getAbsoluteSizeEv
// type: char *__fastcall(RBX::GuiBase2d *this)
pub fn stub_0x8e17d4() {
    // IDA 0x8e17d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiBase2d::getAbsolutePosition(void)const")]
// 0x8e17fc — __ZNK3RBX9GuiBase2d19getAbsolutePositionEv
// type: char *__fastcall(RBX::GuiBase2d *this)
pub fn stub_0x8e17fc() {
    // IDA 0x8e17fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1800 — __ZN3RBX9GuiBase2dD1Ev
// type: void __fastcall(RBX::GuiBase2d *__hidden this)
pub fn stub_0x8e1800() {
    // IDA 0x8e1800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase2d::~GuiBase2d()")]
// 0x8e18bc — __ZN3RBX9GuiBase2dD0Ev
// type: void __fastcall(RBX::GuiBase2d *__hidden this)
pub fn stub_0x8e18bc() {
    // IDA 0x8e18bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase::process(RBX::GuiEvent const&)")]
// 0x8e19b0 — __ZN3RBX7GuiBase7processERKNS_8GuiEventE
// type: _QWORD *__fastcall(_QWORD *result)
pub fn stub_0x8e19b0() {
    // IDA 0x8e19b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiBase2d::canProcessMeAndDescendants(void)const")]
// 0x8e19bc — __ZNK3RBX9GuiBase2d26canProcessMeAndDescendantsEv
// type: int __fastcall(RBX::GuiBase2d *this)
pub fn stub_0x8e19bc() {
    // IDA 0x8e19bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::~GuiBase2d()")]
// 0x8e19c0 — __ZThn32_N3RBX9GuiBase2dD1Ev
// type: void __fastcall(RBX::GuiBase2d *__hidden this)
pub fn stub_0x8e19c0() {
    // IDA 0x8e19c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1a78 — __ZThn32_N3RBX9GuiBase2dD0Ev
// type: void __fastcall(RBX::GuiBase2d *__hidden this)
pub fn stub_0x8e1a78() {
    // IDA 0x8e1a78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1b70 — __ZThn36_N3RBX9GuiBase2dD1Ev
// type: void __fastcall(RBX::GuiBase2d *__hidden this)
pub fn stub_0x8e1b70() {
    // IDA 0x8e1b70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase2d::~GuiBase2d()")]
// 0x8e1c28 — __ZThn36_N3RBX9GuiBase2dD0Ev
// type: void __fastcall(RBX::GuiBase2d *__hidden this)
pub fn stub_0x8e1c28() {
    // IDA 0x8e1c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiBase::process(RBX::GuiEvent const&)")]
// 0x8e1cf8 — __ZThn92_N3RBX7GuiBase7processERKNS_8GuiEventE
// type: _QWORD *__fastcall(_QWORD *result)
pub fn stub_0x8e1cf8() {
    // IDA 0x8e1cf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::GuiLayerCollector(char const*)")]
// 0x8e2920 — __ZN3RBX17GuiLayerCollectorC2EPKc
// type: RBX::GuiBase2d *__fastcall(RBX::GuiLayerCollector *this, const char *)
pub fn stub_0x8e2920() {
    // IDA 0x8e2920: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2b9c — __ZN3RBX17GuiLayerCollectorD0Ev
// type: void __fastcall(RBX::GuiLayerCollector *__hidden this)
pub fn stub_0x8e2b9c() {
    // IDA 0x8e2b9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c3c — __ZN3RBX17GuiLayerCollectorD1Ev
// type: void __fastcall(RBX::GuiLayerCollector *__hidden this)
pub fn stub_0x8e2c3c() {
    // IDA 0x8e2c3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c40 — __ZThn32_N3RBX17GuiLayerCollectorD0Ev
// type: void __fastcall(RBX::GuiLayerCollector *__hidden this)
pub fn stub_0x8e2c40() {
    // IDA 0x8e2c40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c48 — __ZThn36_N3RBX17GuiLayerCollectorD0Ev
// type: void __fastcall(RBX::GuiLayerCollector *__hidden this)
pub fn stub_0x8e2c48() {
    // IDA 0x8e2c48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2c50 — __ZN3RBX17GuiLayerCollectorD2Ev
// type: void __fastcall(RBX::GuiLayerCollector *__hidden this)
pub fn stub_0x8e2c50() {
    // IDA 0x8e2c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2e20 — __ZThn32_N3RBX17GuiLayerCollectorD1Ev
// type: void __fastcall(RBX::GuiLayerCollector *__hidden this)
pub fn stub_0x8e2e20() {
    // IDA 0x8e2e20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiLayerCollector::~GuiLayerCollector()")]
// 0x8e2e28 — __ZThn36_N3RBX17GuiLayerCollectorD1Ev
// type: void __fastcall(RBX::GuiLayerCollector *__hidden this)
pub fn stub_0x8e2e28() {
    // IDA 0x8e2e28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::loadZVectors(void)")]
// 0x8e30e0 — __ZN3RBX17GuiLayerCollector12loadZVectorsEv
// type: void __fastcall(const shared_count *this)
pub fn stub_0x8e30e0() {
    // IDA 0x8e30e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::render2d(RBX::Adorn *)")]
// 0x8e32c8 — __ZN3RBX17GuiLayerCollector8render2dEPNS_5AdornE
// type: int __fastcall(RBX::GuiLayerCollector *this, RBX::Adorn *)
pub fn stub_0x8e32c8() {
    // IDA 0x8e32c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiLayerCollector::render2d(RBX::Adorn *)")]
// 0x8e32d4 — __ZThn96_N3RBX17GuiLayerCollector8render2dEPNS_5AdornE
// type: int __fastcall(RBX::GuiLayerCollector *this, RBX::Adorn *)
pub fn stub_0x8e32d4() {
    // IDA 0x8e32d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::process(RBX::GuiEvent const&)")]
// 0x8e348c — __ZN3RBX17GuiLayerCollector7processERKNS_8GuiEventE
// type: unsigned int __fastcall(_DWORD *, RBX::GuiLayerCollector *, unsigned int *)
pub fn stub_0x8e348c() {
    // IDA 0x8e348c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiLayerCollector::processDescendants(RBX::GuiEvent const&)")]
// 0x8e3534 — __ZN3RBX17GuiLayerCollector18processDescendantsERKNS_8GuiEventE
// type: int __fastcall(int, const shared_count *this, int)
pub fn stub_0x8e3534() {
    // IDA 0x8e3534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::GuiLayerCollector::process(RBX::GuiEvent const&)")]
// 0x8e365c — __ZThn92_N3RBX17GuiLayerCollector7processERKNS_8GuiEventE
// type: unsigned int __fastcall(_DWORD *, int, unsigned int *)
pub fn stub_0x8e365c() {
    // IDA 0x8e365c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>,std::allocator<std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>>::push_back(std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>> const&)")]
// 0x8e3668 — __ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE9push_backERKS6_
// type: int __fastcall(int, int)
pub fn stub_0x8e3668() {
    // IDA 0x8e3668: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::push_back(boost::shared_ptr<RBX::GuiBase> const&)")]
// 0x8e36a8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE9push_backERKS4_
// type: int __fastcall(int, const shared_count *)
pub fn stub_0x8e36a8() {
    // IDA 0x8e36a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::resize(unsigned long,boost::shared_ptr<RBX::GuiBase>)")]
// 0x8e36f8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE6resizeEmS4_
// type: int __fastcall(_DWORD *, unsigned int)
pub fn stub_0x8e36f8() {
    // IDA 0x8e36f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_erase_at_end(boost::shared_ptr<RBX::GuiBase>*)")]
// 0x8e3868 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE15_M_erase_at_endEPS4_
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *result, int)
pub fn stub_0x8e3868() {
    // IDA 0x8e3868: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::GuiBase>*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>,unsigned long,boost::shared_ptr<RBX::GuiBase> const&)")]
// 0x8e3898 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: void __fastcall(int *, struct _Unwind_Exception *, int, const shared_count *)
pub fn stub_0x8e3898() {
    // IDA 0x8e3898: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Vector_base<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_allocate(unsigned long)")]
// 0x8e3e98 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x8e3e98() {
    // IDA 0x8e3e98: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::GuiBase> *,unsigned long,boost::shared_ptr<RBX::GuiBase>>(boost::shared_ptr<RBX::GuiBase> *,unsigned long,boost::shared_ptr<RBX::GuiBase> const&,std::__false_type)")]
// 0x8e3eb0 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX7GuiBaseEEEmS4_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, shared_count *, int, int, int, int, int, void *, int)
pub fn stub_0x8e3eb0() {
    // IDA 0x8e3eb0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::shared_ptr<RBX::GuiBase>::operator=(boost::shared_ptr<RBX::GuiBase> const&)")]
// 0x8e3fd8 — __ZN5boost10shared_ptrIN3RBX7GuiBaseEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
pub fn stub_0x8e3fd8() {
    // IDA 0x8e3fd8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::shared_ptr<RBX::GuiBase> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *>(boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *,boost::shared_ptr<RBX::GuiBase> *)")]
// 0x8e4010 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX7GuiBaseEEES8_EET0_T_SA_S9_
// type: int __fastcall(int, int, int)
pub fn stub_0x8e4010() {
    // IDA 0x8e4010: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::GuiBase>*,std::vector<boost::shared_ptr<RBX::GuiBase>,std::allocator<boost::shared_ptr<RBX::GuiBase>>>>,boost::shared_ptr<RBX::GuiBase> const&)")]
// 0x8e4134 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: void __fastcall(int *, char *, const shared_count *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
pub fn stub_0x8e4134() {
    // IDA 0x8e4134: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
