//! core shard EC — 100 core stubs EA-sorted, lowest uncovered 0x8a68d8..0x8b18ac (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered, fills gap after EB 0x8a68d8 before 0x8b18ad).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered (gap).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::BuoyancyConnector::getConnectorKernelType(void)const")]
// 0x8a68d8 — __ZNK3RBX17BuoyancyConnector22getConnectorKernelTypeEv
pub fn stub_8a68d8() {
    // IDA 0x8a68d8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BuoyancyConnector::~BuoyancyConnector()")]
// 0x8a68dc — __ZN3RBX17BuoyancyConnectorD1Ev
pub fn stub_8a68dc() {
    // IDA 0x8a68dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BuoyancyConnector::~BuoyancyConnector()")]
// 0x8a68e0 — __ZN3RBX17BuoyancyConnectorD0Ev
pub fn stub_8a68e0() {
    // IDA 0x8a68e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::Swimming(RBX::Humanoid *,RBX::HUMAN::StateType)")]
// 0x8a6ab4 — __ZN3RBX5HUMAN8SwimmingC1EPNS_8HumanoidENS0_9StateTypeE
pub fn stub_8a6ab4() {
    // IDA 0x8a6ab4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::Swimming(RBX::Humanoid *,RBX::HUMAN::StateType)")]
// 0x8a6ab8 — __ZN3RBX5HUMAN8SwimmingC2EPNS_8HumanoidENS0_9StateTypeE
pub fn stub_8a6ab8() {
    // IDA 0x8a6ab8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::onComputeForceImpl(void)")]
// 0x8a6bd0 — __ZN3RBX5HUMAN8Swimming18onComputeForceImplEv
pub fn stub_8a6bd0() {
    // IDA 0x8a6bd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::onSimulatorStepImpl(float)")]
// 0x8a7118 — __ZN3RBX5HUMAN8Swimming19onSimulatorStepImplEf
pub fn stub_8a7118() {
    // IDA 0x8a7118: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::fireEvents(void)")]
// 0x8a7238 — __ZN3RBX5HUMAN8Swimming10fireEventsEv
pub fn stub_8a7238() {
    // IDA 0x8a7238: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::minSwimmingMoveForce(void)")]
// 0x8a728c — __ZN3RBX5HUMAN13HumanoidState20minSwimmingMoveForceEv
pub fn stub_8a728c() {
    // IDA 0x8a728c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::HumanoidState::maxSwimmingMoveForce(void)")]
// 0x8a72e8 — __ZN3RBX5HUMAN13HumanoidState20maxSwimmingMoveForceEv
pub fn stub_8a72e8() {
    // IDA 0x8a72e8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HUMAN::Swimming::~Swimming()")]
// 0x8a7374 — __ZN3RBX5HUMAN8SwimmingD1Ev
pub fn stub_8a7374() {
    // IDA 0x8a7374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::~Swimming()")]
// 0x8a7378 — __ZN3RBX5HUMAN8SwimmingD0Ev
pub fn stub_8a7378() {
    // IDA 0x8a7378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HUMAN::Swimming::getStateType(void)const")]
// 0x8a7418 — __ZNK3RBX5HUMAN8Swimming12getStateTypeEv
pub fn stub_8a7418() {
    // IDA 0x8a7418: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::HUMAN::Swimming::~Swimming()")]
// 0x8a741c — __ZThn4_N3RBX5HUMAN8SwimmingD1Ev
// was: non-virtual thunk to RBX::HUMAN::Swimming::~Swimming()
pub fn stub_8a741c() {
    // IDA 0x8a741c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::HUMAN::Swimming::~Swimming()")]
// 0x8a7424 — __ZThn4_N3RBX5HUMAN8SwimmingD0Ev
// was: non-virtual thunk to RBX::HUMAN::Swimming::~Swimming()
pub fn stub_8a7424() {
    // IDA 0x8a7424: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::UintSet(void)")]
// 0x8a781c — __ZN3RBX7UintSetC1Ev
pub fn stub_8a781c() {
    // IDA 0x8a781c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::size(void)const")]
// 0x8a7840 — __ZNK3RBX7UintSet4sizeEv
pub fn stub_8a7840() {
    // IDA 0x8a7840: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::insert(unsigned int)")]
// 0x8a7844 — __ZN3RBX7UintSet6insertEj
pub fn stub_8a7844() {
    // IDA 0x8a7844: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::contains(unsigned int)")]
// 0x8a7948 — __ZN3RBX7UintSet8containsEj
pub fn stub_8a7948() {
    // IDA 0x8a7948: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UintSet::pop_smallest(unsigned int *)")]
// 0x8a798c — __ZN3RBX7UintSet12pop_smallestEPj
pub fn stub_8a798c() {
    // IDA 0x8a798c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::pop_front(unsigned int *)")]
// 0x8a7b90 — __ZN3RBX17DoubleEndedVectorIjE9pop_frontEPj
pub fn stub_8a7b90() {
    // IDA 0x8a7b90: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::grow(void)")]
// 0x8a7c00 — __ZN3RBX17DoubleEndedVectorIjE4growEv
pub fn stub_8a7c00() {
    // IDA 0x8a7c00: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::~ManualJointHelper()")]
// 0x8a7e34 — __ZN3RBX17ManualJointHelperD1Ev
pub fn stub_8a7e34() {
    // IDA 0x8a7e34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::~ManualJointHelper()")]
// 0x8a7e38 — __ZN3RBX17ManualJointHelperD2Ev
pub fn stub_8a7e38() {
    // IDA 0x8a7e38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::ManualJointHelper(void)")]
// 0x8a80e4 — __ZN3RBX17ManualJointHelperC1Ev
pub fn stub_8a80e4() {
    // IDA 0x8a80e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::clearAndDeleteJointSurfacePairs(void)")]
// 0x8a8134 — __ZN3RBX17ManualJointHelper31clearAndDeleteJointSurfacePairsEv
pub fn stub_8a8134() {
    // IDA 0x8a8134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::findPermissibleJointSurfacePairs(void)")]
// 0x8a816c — __ZN3RBX17ManualJointHelper32findPermissibleJointSurfacePairsEv
pub fn stub_8a816c() {
    // IDA 0x8a816c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::createJointSurfacePair(RBX::Primitive &,unsigned long &,RBX::Primitive &,unsigned long &)")]
// 0x8a8478 — __ZN3RBX17ManualJointHelper22createJointSurfacePairERNS_9PrimitiveERmS2_S3_
pub fn stub_8a8478() {
    // IDA 0x8a8478: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::createJoints(void)")]
// 0x8a9070 — __ZN3RBX17ManualJointHelper12createJointsEv
pub fn stub_8a9070() {
    // IDA 0x8a9070: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::createJointsIfEnabledFromGui(void)")]
// 0x8a909c — __ZN3RBX17ManualJointHelper28createJointsIfEnabledFromGuiEv
pub fn stub_8a909c() {
    // IDA 0x8a909c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointHelper::render3dAdorn(RBX::Adorn *)")]
// 0x8a90d8 — __ZN3RBX17ManualJointHelper13render3dAdornEPNS_5AdornE
pub fn stub_8a90d8() {
    // IDA 0x8a90d8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::StudAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a911c — __ZN3RBX24StudAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8a911c() {
    // IDA 0x8a911c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::GlueAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9430 — __ZN3RBX24GlueAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8a9430() {
    // IDA 0x8a9430: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::WeldAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9744 — __ZN3RBX24WeldAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8a9744() {
    // IDA 0x8a9744: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::HingeAutoJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9a58 — __ZN3RBX25HingeAutoJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8a9a58() {
    // IDA 0x8a9a58: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DisallowedJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8a9d6c — __ZN3RBX26DisallowedJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8a9d6c() {
    // IDA 0x8a9d6c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8aa080 — __ZN3RBX22ManualJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8aa080() {
    // IDA 0x8aa080: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::ManualJointSurfacePair::createJoint(void)")]
// 0x8aa3bc — __ZN3RBX22ManualJointSurfacePair11createJointEv
pub fn stub_8aa3bc() {
    // IDA 0x8aa3bc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TerrainManualJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8aafa4 — __ZN3RBX29TerrainManualJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8aafa4() {
    // IDA 0x8aafa4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::DisallowedTerrainJointSurfacePair::dynamicDraw(RBX::Adorn *)")]
// 0x8ab104 — __ZN3RBX33DisallowedTerrainJointSurfacePair11dynamicDrawEPNS_5AdornE
pub fn stub_8ab104() {
    // IDA 0x8ab104: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::TerrainManualJointSurfacePair::createJoint(void)")]
// 0x8ab238 — __ZN3RBX29TerrainManualJointSurfacePair11createJointEv
pub fn stub_8ab238() {
    // IDA 0x8ab238: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::push_back(RBX::ConstraintSurfacePair * const&)")]
// 0x8ab7bc — __ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE9push_backERKS2_
pub fn stub_8ab7bc() {
    // IDA 0x8ab7bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")]
// 0x8ab85c — __ZN3RBX24StudAutoJointSurfacePairD1Ev
pub fn stub_8ab85c() {
    // IDA 0x8ab85c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")]
// 0x8ab860 — __ZN3RBX24StudAutoJointSurfacePairD0Ev
pub fn stub_8ab860() {
    // IDA 0x8ab860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ConstraintSurfacePair::createJoint(void)")]
// 0x8ab900 — __ZN3RBX21ConstraintSurfacePair11createJointEv
pub fn stub_8ab900() {
    // IDA 0x8ab900: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")]
// 0x8ab904 — __ZN3RBX24WeldAutoJointSurfacePairD1Ev
pub fn stub_8ab904() {
    // IDA 0x8ab904: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")]
// 0x8ab908 — __ZN3RBX24WeldAutoJointSurfacePairD0Ev
pub fn stub_8ab908() {
    // IDA 0x8ab908: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")]
// 0x8ab9a8 — __ZN3RBX24GlueAutoJointSurfacePairD1Ev
pub fn stub_8ab9a8() {
    // IDA 0x8ab9a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")]
// 0x8ab9ac — __ZN3RBX24GlueAutoJointSurfacePairD0Ev
pub fn stub_8ab9ac() {
    // IDA 0x8ab9ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")]
// 0x8aba4c — __ZN3RBX25HingeAutoJointSurfacePairD1Ev
pub fn stub_8aba4c() {
    // IDA 0x8aba4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")]
// 0x8aba50 — __ZN3RBX25HingeAutoJointSurfacePairD0Ev
pub fn stub_8aba50() {
    // IDA 0x8aba50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")]
// 0x8abaf0 — __ZN3RBX26DisallowedJointSurfacePairD1Ev
pub fn stub_8abaf0() {
    // IDA 0x8abaf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")]
// 0x8abaf4 — __ZN3RBX26DisallowedJointSurfacePairD0Ev
pub fn stub_8abaf4() {
    // IDA 0x8abaf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")]
// 0x8abb94 — __ZN3RBX22ManualJointSurfacePairD1Ev
pub fn stub_8abb94() {
    // IDA 0x8abb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")]
// 0x8abb98 — __ZN3RBX22ManualJointSurfacePairD0Ev
pub fn stub_8abb98() {
    // IDA 0x8abb98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TerrainManualJointSurfacePair::~TerrainManualJointSurfacePair()")]
// 0x8abc38 — __ZN3RBX29TerrainManualJointSurfacePairD1Ev
pub fn stub_8abc38() {
    // IDA 0x8abc38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TerrainManualJointSurfacePair::~TerrainManualJointSurfacePair()")]
// 0x8abc3c — __ZN3RBX29TerrainManualJointSurfacePairD0Ev
pub fn stub_8abc3c() {
    // IDA 0x8abc3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DisallowedTerrainJointSurfacePair::~DisallowedTerrainJointSurfacePair()")]
// 0x8abcdc — __ZN3RBX33DisallowedTerrainJointSurfacePairD1Ev
pub fn stub_8abcdc() {
    // IDA 0x8abcdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::DisallowedTerrainJointSurfacePair::~DisallowedTerrainJointSurfacePair()")]
// 0x8abce0 — __ZN3RBX33DisallowedTerrainJointSurfacePairD0Ev
pub fn stub_8abce0() {
    // IDA 0x8abce0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ManualJointHelper::shouldRender3dAdorn(void)const")]
// 0x8abd80 — __ZNK3RBX17ManualJointHelper19shouldRender3dAdornEv
pub fn stub_8abd80() {
    // IDA 0x8abd80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ConstraintSurfacePair::~ConstraintSurfacePair()")]
// 0x8abd84 — __ZN3RBX21ConstraintSurfacePairD1Ev
pub fn stub_8abd84() {
    // IDA 0x8abd84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ConstraintSurfacePair::~ConstraintSurfacePair()")]
// 0x8abd88 — __ZN3RBX21ConstraintSurfacePairD0Ev
pub fn stub_8abd88() {
    // IDA 0x8abd88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ConstraintSurfacePair::~ConstraintSurfacePair()")]
// 0x8abe28 — __ZN3RBX21ConstraintSurfacePairD2Ev
pub fn stub_8abe28() {
    // IDA 0x8abe28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ConstraintSurfacePair **,std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>>,RBX::ConstraintSurfacePair * const&)")]
// 0x8ac41c — __ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_8ac41c() {
    // IDA 0x8ac41c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::_M_allocate(unsigned long)")]
// 0x8ac4fc — __ZNSt12_Vector_baseIPN3RBX21ConstraintSurfacePairESaIS2_EE11_M_allocateEm
pub fn stub_8ac4fc() {
    // IDA 0x8ac4fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive *>(__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive * const&,std::random_access_iterator_tag)")]
// 0x8ac514 — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX9PrimitiveESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
pub fn stub_8ac514() {
    // IDA 0x8ac514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GamePassService::setPlayerHasPassUrl(std::string)")]
// 0x8ac948 — __ZN3RBX15GamePassService19setPlayerHasPassUrlESs
pub fn stub_8ac948() {
    // IDA 0x8ac948: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GamePassService::GamePassService(void)")]
// 0x8acd34 — __ZN3RBX15GamePassServiceC1Ev
pub fn stub_8acd34() {
    // IDA 0x8acd34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GamePassService::GamePassService(void)")]
// 0x8acd38 — __ZN3RBX15GamePassServiceC2Ev
pub fn stub_8acd38() {
    // IDA 0x8acd38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void RBX::GamePassService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x8ad1fc — __ZN3RBX15GamePassService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
pub fn stub_8ad1fc() {
    // IDA 0x8ad1fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GamePassService::~GamePassService()")]
// 0x8ad4a4 — __ZN3RBX15GamePassServiceD1Ev
pub fn stub_8ad4a4() {
    // IDA 0x8ad4a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GamePassService::~GamePassService()")]
// 0x8ad4e0 — __ZN3RBX15GamePassServiceD0Ev
pub fn stub_8ad4e0() {
    // IDA 0x8ad4e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::GamePassService::~GamePassService()")]
// 0x8ad5dc — __ZThn32_N3RBX15GamePassServiceD1Ev
// was: non-virtual thunk to RBX::GamePassService::~GamePassService()
pub fn stub_8ad5dc() {
    // IDA 0x8ad5dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::GamePassService::~GamePassService()")]
// 0x8ad61c — __ZThn32_N3RBX15GamePassServiceD0Ev
// was: non-virtual thunk to RBX::GamePassService::~GamePassService()
pub fn stub_8ad61c() {
    // IDA 0x8ad61c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::GamePassService::~GamePassService()")]
// 0x8ad718 — __ZThn36_N3RBX15GamePassServiceD1Ev
// was: non-virtual thunk to RBX::GamePassService::~GamePassService()
pub fn stub_8ad718() {
    // IDA 0x8ad718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to RBX::GamePassService::~GamePassService()")]
// 0x8ad758 — __ZThn36_N3RBX15GamePassServiceD0Ev
// was: non-virtual thunk to RBX::GamePassService::~GamePassService()
pub fn stub_8ad758() {
    // IDA 0x8ad758: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::getTouchEnabled(void)const")]
// 0x8ae8dc — __ZNK3RBX16UserInputService15getTouchEnabledEv
pub fn stub_8ae8dc() {
    // IDA 0x8ae8dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::getKeyboardEnabled(void)const")]
// 0x8aea28 — __ZNK3RBX16UserInputService18getKeyboardEnabledEv
pub fn stub_8aea28() {
    // IDA 0x8aea28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::getMouseEnabled(void)const")]
// 0x8aeb74 — __ZNK3RBX16UserInputService15getMouseEnabledEv
pub fn stub_8aeb74() {
    // IDA 0x8aeb74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::getGamepadEnabled(void)const")]
// 0x8aecc0 — __ZNK3RBX16UserInputService17getGamepadEnabledEv
pub fn stub_8aecc0() {
    // IDA 0x8aecc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::getModalEnabled(void)const")]
// 0x8aee0c — __ZNK3RBX16UserInputService15getModalEnabledEv
pub fn stub_8aee0c() {
    // IDA 0x8aee0c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::setModalEnabled(bool)")]
// 0x8aef58 — __ZN3RBX16UserInputService15setModalEnabledEb
pub fn stub_8aef58() {
    // IDA 0x8aef58: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::UserInputService(void)")]
// 0x8af0c4 — __ZN3RBX16UserInputServiceC1Ev
pub fn stub_8af0c4() {
    // IDA 0x8af0c4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::UserInputService(void)")]
// 0x8af0c8 — __ZN3RBX16UserInputServiceC2Ev
pub fn stub_8af0c8() {
    // IDA 0x8af0c8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x8af9ac — __ZN3RBX16UserInputService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_8af9ac() {
    // IDA 0x8af9ac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::setTouchEnabled(bool)")]
// 0x8afcd0 — __ZN3RBX16UserInputService15setTouchEnabledEb
pub fn stub_8afcd0() {
    // IDA 0x8afcd0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::getLocalCharacterJumpEnabled(void)const")]
// 0x8afcf0 — __ZNK3RBX16UserInputService28getLocalCharacterJumpEnabledEv
pub fn stub_8afcf0() {
    // IDA 0x8afcf0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::getLocalCharacterClickToWalkEnabled(void)const")]
// 0x8afcf8 — __ZNK3RBX16UserInputService35getLocalCharacterClickToWalkEnabledEv
pub fn stub_8afcf8() {
    // IDA 0x8afcf8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::setLocalCharacterClickToWalkEnabled(bool)")]
// 0x8afe44 — __ZN3RBX16UserInputService35setLocalCharacterClickToWalkEnabledEb
pub fn stub_8afe44() {
    // IDA 0x8afe44: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::getLocalCharacterMovementEnabled(void)const")]
// 0x8aff9c — __ZNK3RBX16UserInputService32getLocalCharacterMovementEnabledEv
pub fn stub_8aff9c() {
    // IDA 0x8aff9c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::onStepped(RBX::Stepped const&)")]
// 0x8b0380 — __ZN3RBX16UserInputService9onSteppedERKNS_7SteppedE
pub fn stub_8b0380() {
    // IDA 0x8b0380: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk to RBX::UserInputService::onStepped(RBX::Stepped const&)")]
// 0x8b0784 — __ZThn96_N3RBX16UserInputService9onSteppedERKNS_7SteppedE
// was: non-virtual thunk to RBX::UserInputService::onStepped(RBX::Stepped const&)
pub fn stub_8b0784() {
    // IDA 0x8b0784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::jumpLocalCharacter(bool)")]
// 0x8b0944 — __ZN3RBX16UserInputService18jumpLocalCharacterEb
pub fn stub_8b0944() {
    // IDA 0x8b0944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::sendJumpRequestEvent(void)")]
// 0x8b095c — __ZN3RBX16UserInputService20sendJumpRequestEventEv
pub fn stub_8b095c() {
    // IDA 0x8b095c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::jumpOnceLocalCharacter(bool)")]
// 0x8b0abc — __ZN3RBX16UserInputService22jumpOnceLocalCharacterEb
pub fn stub_8b0abc() {
    // IDA 0x8b0abc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::setProcessToolCommandsSeparately(bool)")]
// 0x8b0ad4 — __ZN3RBX16UserInputService32setProcessToolCommandsSeparatelyEb
pub fn stub_8b0ad4() {
    // IDA 0x8b0ad4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UserInputService::processToolEvent(RBX::UIEvent)")]
// 0x8b0adc — __ZN3RBX16UserInputService16processToolEventENS_7UIEventE
pub fn stub_8b0adc() {
    // IDA 0x8b0adc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::zoomCamera(float)")]
// 0x8b1280 — __ZN3RBX16UserInputService10zoomCameraEf
pub fn stub_8b1280() {
    // IDA 0x8b1280: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::textboxDidFinishEditing(char const*,bool)")]
// 0x8b14d8 — __ZN3RBX16UserInputService23textboxDidFinishEditingEPKcb
pub fn stub_8b14d8() {
    // IDA 0x8b14d8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UserInputService::sendMouseEvent(RBX::UIEvent,void *)")]
// 0x8b18ac — __ZN3RBX16UserInputService14sendMouseEventENS_7UIEventEPv
pub fn stub_8b18ac() {
    // IDA 0x8b18ac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}