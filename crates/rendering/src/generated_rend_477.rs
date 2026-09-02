//! rendering shard 477 — 100 stubs 0x756a74..0x75da80 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 51730->51830 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc after shard 476 (0x756a74..0x75da80)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// 0x756a74 — __ZN3RBX10SleepStageD0Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::~SleepStage()")]
#[doc(alias = "__ZN3RBX10SleepStageD0Ev")]
pub fn stub_756a74() -> ! {
    todo!("0x756a74 RBX::SleepStage::~SleepStage()")
}
// 0x756b14 — __ZN3RBX10SleepStageD1Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::~SleepStage()")]
#[doc(alias = "__ZN3RBX10SleepStageD1Ev")]
pub fn stub_756b14() -> ! {
    todo!("0x756b14 RBX::SleepStage::~SleepStage()")
}
// 0x756b18 — __ZN3RBX10SleepStageD2Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::~SleepStage()")]
#[doc(alias = "__ZN3RBX10SleepStageD2Ev")]
pub fn stub_756b18() -> ! {
    todo!("0x756b18 RBX::SleepStage::~SleepStage()")
}
// 0x7573d8 — __ZN3RBX10SleepStage14stepSleepStageEiib
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, int, int, bool)
#[doc(alias = "RBX::SleepStage::stepSleepStage(int,int,bool)")]
#[doc(alias = "__ZN3RBX10SleepStage14stepSleepStageEiib")]
pub fn stub_7573d8() -> ! {
    todo!("0x7573d8 RBX::SleepStage::stepSleepStage(int,int,bool)")
}
// 0x7578ac — __ZN3RBX10SleepStage10doContactsERA2_NS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE
#[doc(alias = "RBX::SleepStage::doContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> (&)[2])")]
#[doc(alias = "__ZN3RBX10SleepStage10doContactsERA2_NS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE")]
pub fn stub_7578ac() -> ! {
    todo!("0x7578ac RBX::SleepStage::doContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> (&)[2])")
}
// 0x75798c — __ZN3RBX10SleepStage19stepAssembliesAwakeEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::stepAssembliesAwake(void)")]
#[doc(alias = "__ZN3RBX10SleepStage19stepAssembliesAwakeEv")]
pub fn stub_75798c() -> ! {
    todo!("0x75798c RBX::SleepStage::stepAssembliesAwake(void)")
}
// 0x757c2c — __ZN3RBX10SleepStage30stepAssembliesSleepingCheckingEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::stepAssembliesSleepingChecking(void)")]
#[doc(alias = "__ZN3RBX10SleepStage30stepAssembliesSleepingCheckingEv")]
pub fn stub_757c2c() -> ! {
    todo!("0x757c2c RBX::SleepStage::stepAssembliesSleepingChecking(void)")
}
// 0x757ea8 — __ZN3RBX10SleepStage10stepJointsEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::stepJoints(void)")]
#[doc(alias = "__ZN3RBX10SleepStage10stepJointsEv")]
pub fn stub_757ea8() -> ! {
    todo!("0x757ea8 RBX::SleepStage::stepJoints(void)")
}
// 0x7580dc — __ZN3RBX10SleepStage12stepContactsERNS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE
#[doc(alias = "RBX::SleepStage::stepContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> &)")]
#[doc(alias = "__ZN3RBX10SleepStage12stepContactsERNS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE")]
pub fn stub_7580dc() -> ! {
    todo!("0x7580dc RBX::SleepStage::stepContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> &)")
}
// 0x758624 — __ZN3RBX10SleepStage14wakeAssembliesERSt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEiNS_3Sim13AssemblyStateE
// type: int __fastcall(int, int, int, void *)
#[doc(alias = "RBX::SleepStage::wakeAssemblies(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> &,int,RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage14wakeAssembliesERSt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEiNS_3Sim13AssemblyStateE")]
pub fn stub_758624() -> ! {
    todo!("0x758624 RBX::SleepStage::wakeAssemblies(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> &,int,RBX::Sim::AssemblyState)")
}
// 0x758958 — __ZN3RBX10SleepStage8traverseEPNS_8AssemblyERSt5dequeIS2_SaIS2_EEi
// type: int __fastcall(int, RBX::Assembly *this)
#[doc(alias = "RBX::SleepStage::traverse(RBX::Assembly *,std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>> &,int)")]
#[doc(alias = "__ZN3RBX10SleepStage8traverseEPNS_8AssemblyERSt5dequeIS2_SaIS2_EEi")]
pub fn stub_758958() -> ! {
    todo!("0x758958 RBX::SleepStage::traverse(RBX::Assembly *,std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>> &,int)")
}
// 0x758adc — __ZN3RBX11canThrottleEPNS_4EdgeE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::canThrottle(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX11canThrottleEPNS_4EdgeE")]
pub fn stub_758adc() -> ! {
    todo!("0x758adc RBX::canThrottle(RBX::Edge *)")
}
// 0x758b40 — __ZN3RBX10SleepStage18changeContactStateERKSt6vectorIPNS_7ContactESaIS3_EENS_3Sim9EdgeStateE
#[doc(alias = "RBX::SleepStage::changeContactState(std::vector<RBX::Contact *,std::allocator<RBX::Contact *>> const&,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage18changeContactStateERKSt6vectorIPNS_7ContactESaIS3_EENS_3Sim9EdgeStateE")]
pub fn stub_758b40() -> ! {
    todo!("0x758b40 RBX::SleepStage::changeContactState(std::vector<RBX::Contact *,std::allocator<RBX::Contact *>> const&,RBX::Sim::EdgeState)")
}
// 0x758b78 — __ZN3RBX10SleepStage16changeJointStateERKSt6vectorIPNS_5JointESaIS3_EENS_3Sim9EdgeStateE
#[doc(alias = "RBX::SleepStage::changeJointState(std::vector<RBX::Joint *,std::allocator<RBX::Joint *>> const&,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage16changeJointStateERKSt6vectorIPNS_5JointESaIS3_EENS_3Sim9EdgeStateE")]
pub fn stub_758b78() -> ! {
    todo!("0x758b78 RBX::SleepStage::changeJointState(std::vector<RBX::Joint *,std::allocator<RBX::Joint *>> const&,RBX::Sim::EdgeState)")
}
// 0x758bb0 — __ZN3RBX10SleepStage25computeStateFromNeighborsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::computeStateFromNeighbors(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage25computeStateFromNeighborsEPNS_8AssemblyE")]
pub fn stub_758bb0() -> ! {
    todo!("0x758bb0 RBX::SleepStage::computeStateFromNeighbors(RBX::Assembly *)")
}
// 0x758c4c — __ZN3RBX10SleepStage19changeAssemblyStateERKSt6vectorIPNS_8AssemblyESaIS3_EENS_3Sim13AssemblyStateE
#[doc(alias = "RBX::SleepStage::changeAssemblyState(std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>> const&,RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage19changeAssemblyStateERKSt6vectorIPNS_8AssemblyESaIS3_EENS_3Sim13AssemblyStateE")]
pub fn stub_758c4c() -> ! {
    todo!("0x758c4c RBX::SleepStage::changeAssemblyState(std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>> const&,RBX::Sim::AssemblyState)")
}
// 0x758c84 — __ZN3RBX10SleepStage19changeAssemblyStateEPNS_8AssemblyENS_3Sim13AssemblyStateE
// type: int __fastcall(RBX::IStage *, RBX::IPipelined *this)
#[doc(alias = "RBX::SleepStage::changeAssemblyState(RBX::Assembly *,RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage19changeAssemblyStateEPNS_8AssemblyENS_3Sim13AssemblyStateE")]
pub fn stub_758c84() -> ! {
    todo!("0x758c84 RBX::SleepStage::changeAssemblyState(RBX::Assembly *,RBX::Sim::AssemblyState)")
}
// 0x758f40 — __ZN3RBX10SleepStage8wakeEdgeEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::wakeEdge(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage8wakeEdgeEPNS_4EdgeE")]
pub fn stub_758f40() -> ! {
    todo!("0x758f40 RBX::SleepStage::wakeEdge(RBX::Edge *)")
}
// 0x759080 — __ZN3RBX10SleepStage11isAffectingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::isAffecting(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage11isAffectingEPNS_4EdgeE")]
pub fn stub_759080() -> ! {
    todo!("0x759080 RBX::SleepStage::isAffecting(RBX::Edge *)")
}
// 0x759144 — __ZN3RBX10SleepStage18changeContactStateEPNS_7ContactENS_3Sim9EdgeStateE
#[doc(alias = "RBX::SleepStage::changeContactState(RBX::Contact *,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage18changeContactStateEPNS_7ContactENS_3Sim9EdgeStateE")]
pub fn stub_759144() -> ! {
    todo!("0x759144 RBX::SleepStage::changeContactState(RBX::Contact *,RBX::Sim::EdgeState)")
}
// 0x75942c — __ZN3RBX10SleepStage16changeJointStateEPNS_5JointENS_3Sim9EdgeStateE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SleepStage::changeJointState(RBX::Joint *,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage16changeJointStateEPNS_5JointENS_3Sim9EdgeStateE")]
pub fn stub_75942c() -> ! {
    todo!("0x75942c RBX::SleepStage::changeJointState(RBX::Joint *,RBX::Sim::EdgeState)")
}
// 0x759578 — __ZN3RBX10SleepStage9wakeEventEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::wakeEvent(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage9wakeEventEPNS_4EdgeE")]
pub fn stub_759578() -> ! {
    todo!("0x759578 RBX::SleepStage::wakeEvent(RBX::Edge *)")
}
// 0x75959c — __ZN3RBX10SleepStage9wakeEventEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::wakeEvent(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage9wakeEventEPNS_8AssemblyE")]
pub fn stub_75959c() -> ! {
    todo!("0x75959c RBX::SleepStage::wakeEvent(RBX::Assembly *)")
}
// 0x7595c0 — __ZN3RBX10SleepStage18recursiveWakeEventEPNS_7ContactE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Contact *)
#[doc(alias = "RBX::SleepStage::recursiveWakeEvent(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX10SleepStage18recursiveWakeEventEPNS_7ContactE")]
pub fn stub_7595c0() -> ! {
    todo!("0x7595c0 RBX::SleepStage::recursiveWakeEvent(RBX::Contact *)")
}
// 0x7595e4 — __ZN3RBX10SleepStage18recursiveWakeEventEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::recursiveWakeEvent(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage18recursiveWakeEventEPNS_8AssemblyE")]
pub fn stub_7595e4() -> ! {
    todo!("0x7595e4 RBX::SleepStage::recursiveWakeEvent(RBX::Assembly *)")
}
// 0x759608 — __ZN3RBX10SleepStage20highVelocityNewTouchEPNS_7ContactE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Contact *)
#[doc(alias = "RBX::SleepStage::highVelocityNewTouch(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX10SleepStage20highVelocityNewTouchEPNS_7ContactE")]
pub fn stub_759608() -> ! {
    todo!("0x759608 RBX::SleepStage::highVelocityNewTouch(RBX::Contact *)")
}
// 0x7596f0 — __ZN3RBX10SleepStage10stateToSetENS_3Sim13AssemblyStateE
#[doc(alias = "RBX::SleepStage::stateToSet(RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage10stateToSetENS_3Sim13AssemblyStateE")]
pub fn stub_7596f0() -> ! {
    todo!("0x7596f0 RBX::SleepStage::stateToSet(RBX::Sim::AssemblyState)")
}
// 0x759778 — __ZN3RBX10SleepStage24onExternalTickleAssemblyEPNS_8AssemblyEb
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *, bool)
#[doc(alias = "RBX::SleepStage::onExternalTickleAssembly(RBX::Assembly *,bool)")]
#[doc(alias = "__ZN3RBX10SleepStage24onExternalTickleAssemblyEPNS_8AssemblyEb")]
pub fn stub_759778() -> ! {
    todo!("0x759778 RBX::SleepStage::onExternalTickleAssembly(RBX::Assembly *,bool)")
}
// 0x7597c0 — __ZN3RBX10SleepStage15onAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::onAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage15onAssemblyAddedEPNS_8AssemblyE")]
pub fn stub_7597c0() -> ! {
    todo!("0x7597c0 RBX::SleepStage::onAssemblyAdded(RBX::Assembly *)")
}
// 0x7599fc — __ZN3RBX10SleepStage18onAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::onAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage18onAssemblyRemovingEPNS_8AssemblyE")]
pub fn stub_7599fc() -> ! {
    todo!("0x7599fc RBX::SleepStage::onAssemblyRemoving(RBX::Assembly *)")
}
// 0x759b68 — __ZN3RBX10SleepStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_759b68() -> ! {
    todo!("0x759b68 RBX::SleepStage::onEdgeAdded(RBX::Edge *)")
}
// 0x759d40 — __ZN3RBX10SleepStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage14onEdgeRemovingEPNS_4EdgeE")]
pub fn stub_759d40() -> ! {
    todo!("0x759d40 RBX::SleepStage::onEdgeRemoving(RBX::Edge *)")
}
// 0x759ed8 — __ZN3RBX10SleepStage9getMetricENS_11IWorldStage10MetricTypeE
#[doc(alias = "RBX::SleepStage::getMetric(RBX::IWorldStage::MetricType)")]
#[doc(alias = "__ZN3RBX10SleepStage9getMetricENS_11IWorldStage10MetricTypeE")]
pub fn stub_759ed8() -> ! {
    todo!("0x759ed8 RBX::SleepStage::getMetric(RBX::IWorldStage::MetricType)")
}
// 0x759efc — __ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE
// type: _DWORD __fastcall(RBX::IPipelined *__hidden this, RBX::IStage *)
#[doc(alias = "RBX::IPipelined::inOrDownstreamOfStage(RBX::IStage *)const")]
#[doc(alias = "__ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE")]
pub fn stub_759efc() -> ! {
    todo!("0x759efc RBX::IPipelined::inOrDownstreamOfStage(RBX::IStage *)const")
}
// 0x759fbc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_")]
pub fn stub_759fbc() -> ! {
    todo!("0x759fbc std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")
}
// 0x759fdc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9pop_frontEv
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9pop_frontEv")]
pub fn stub_759fdc() -> ! {
    todo!("0x759fdc std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::pop_front(void)")
}
// 0x75a00c — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::push_back(RBX::Contact * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE9push_backERKS2_")]
pub fn stub_75a00c() -> ! {
    todo!("0x75a00c std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::push_back(RBX::Contact * const&)")
}
// 0x75a038 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::resize(unsigned long,RBX::Contact *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE6resizeEmS2_")]
pub fn stub_75a038() -> ! {
    todo!("0x75a038 std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::resize(unsigned long,RBX::Contact *)")
}
// 0x75a06c — __ZNSt6vectorIPN3RBX5JointESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::resize(unsigned long,RBX::Joint *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE6resizeEmS2_")]
pub fn stub_75a06c() -> ! {
    todo!("0x75a06c std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::resize(unsigned long,RBX::Joint *)")
}
// 0x75a0a0 — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_")]
pub fn stub_75a0a0() -> ! {
    todo!("0x75a0a0 std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")
}
// 0x75a0cc — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::resize(unsigned long,RBX::Assembly *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE6resizeEmS2_")]
pub fn stub_75a0cc() -> ! {
    todo!("0x75a0cc std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::resize(unsigned long,RBX::Assembly *)")
}
// 0x75a100 — __ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_
#[doc(alias = "RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc>::fastRemove(RBX::Contact*)")]
#[doc(alias = "__ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_")]
pub fn stub_75a100() -> ! {
    todo!("0x75a100 RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc>::fastRemove(RBX::Contact*)")
}
// 0x75a1d4 — __ZN3RBX4Body24resetImpulseAccumulatorsEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::resetImpulseAccumulators(void)")]
#[doc(alias = "__ZN3RBX4Body24resetImpulseAccumulatorsEv")]
pub fn stub_75a1d4() -> ! {
    todo!("0x75a1d4 RBX::Body::resetImpulseAccumulators(void)")
}
// 0x75a208 — __ZN3RBX4Body22resetForceAccumulatorsEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::resetForceAccumulators(void)")]
#[doc(alias = "__ZN3RBX4Body22resetForceAccumulatorsEv")]
pub fn stub_75a208() -> ! {
    todo!("0x75a208 RBX::Body::resetForceAccumulators(void)")
}
// 0x75a240 — __ZNK3RBX10SleepStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX10SleepStage12getStageTypeEv")]
pub fn stub_75a240() -> ! {
    todo!("0x75a240 RBX::SleepStage::getStageType(void)const")
}
// 0x75a540 — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,unsigned long,RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_75a540() -> ! {
    todo!("0x75a540 std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,unsigned long,RBX::Assembly * const&)")
}
// 0x75a6a8 — __ZNSt12_Vector_baseIPN3RBX8AssemblyESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX8AssemblyESaIS2_EE11_M_allocateEm")]
pub fn stub_75a6a8() -> ! {
    todo!("0x75a6a8 std::_Vector_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate(unsigned long)")
}
// 0x75a6c0 — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_75a6c0() -> ! {
    todo!("0x75a6c0 std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,RBX::Assembly * const&)")
}
// 0x75a7a0 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,unsigned long,RBX::Joint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_75a7a0() -> ! {
    todo!("0x75a7a0 std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,unsigned long,RBX::Joint * const&)")
}
// 0x75a908 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,unsigned long,RBX::Contact * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_75a908() -> ! {
    todo!("0x75a908 std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,unsigned long,RBX::Contact * const&)")
}
// 0x75aa70 — __ZNSt12_Vector_baseIPN3RBX7ContactESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX7ContactESaIS2_EE11_M_allocateEm")]
pub fn stub_75aa70() -> ! {
    todo!("0x75aa70 std::_Vector_base<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_allocate(unsigned long)")
}
// 0x75aa88 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,RBX::Contact * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_75aa88() -> ! {
    todo!("0x75aa88 std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,RBX::Contact * const&)")
}
// 0x75ab68 — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_push_back_aux(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_")]
pub fn stub_75ab68() -> ! {
    todo!("0x75ab68 std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_push_back_aux(RBX::Assembly * const&)")
}
// 0x75aba0 — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE22_M_reserve_map_at_backEm")]
pub fn stub_75aba0() -> ! {
    todo!("0x75aba0 std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reserve_map_at_back(unsigned long)")
}
// 0x75abbc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb")]
pub fn stub_75abbc() -> ! {
    todo!("0x75abbc std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reallocate_map(unsigned long,bool)")
}
// 0x75ac94 — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_allocate_mapEm")]
pub fn stub_75ac94() -> ! {
    todo!("0x75ac94 std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate_map(unsigned long)")
}
// 0x75acac — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EED2Ev
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EED2Ev")]
pub fn stub_75acac() -> ! {
    todo!("0x75acac std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::~_Deque_base()")
}
// 0x75acd8 — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE17_M_initialize_mapEm")]
pub fn stub_75acd8() -> ! {
    todo!("0x75acd8 std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_initialize_map(unsigned long)")
}
// 0x75ae30 — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_create_nodesEPPS2_S6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_create_nodes(RBX::Assembly ***,RBX::Assembly ***)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_create_nodesEPPS2_S6_")]
pub fn stub_75ae30() -> ! {
    todo!("0x75ae30 std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_create_nodes(RBX::Assembly ***,RBX::Assembly ***)")
}
// 0x75b0e8 — __GLOBAL__I_a_350
#[doc(alias = "global constructor keyed to_a_350")]
#[doc(alias = "__GLOBAL__I_a_350")]
pub fn stub_75b0e8() -> ! {
    todo!("0x75b0e8 `global constructor keyed to'_a_350")
}
// 0x75b24c — __ZN3RBX9SnapJoint18compatibleSurfacesEPNS_9PrimitiveES2_NS_8NormalIdES3_
#[doc(alias = "RBX::SnapJoint::compatibleSurfaces(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9SnapJoint18compatibleSurfacesEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
pub fn stub_75b24c() -> ! {
    todo!("0x75b24c RBX::SnapJoint::compatibleSurfaces(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")
}
// 0x75b2a0 — __ZN3RBX9SnapJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::SnapJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9SnapJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
pub fn stub_75b2a0() -> ! {
    todo!("0x75b2a0 RBX::SnapJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")
}
// 0x75b3e0 — __GLOBAL__I_a_351
#[doc(alias = "global constructor keyed to_a_351")]
#[doc(alias = "__GLOBAL__I_a_351")]
pub fn stub_75b3e0() -> ! {
    todo!("0x75b3e0 `global constructor keyed to'_a_351")
}
// 0x75b4a8 — __ZN3RBX13SpatialFilterC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SpatialFilter::SpatialFilter(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SpatialFilterC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75b4a8() -> ! {
    todo!("0x75b4a8 RBX::SpatialFilter::SpatialFilter(RBX::IStage *,RBX::World *)")
}
// 0x75b4ac — __ZN3RBX13SpatialFilterC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SpatialFilter::SpatialFilter(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SpatialFilterC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75b4ac() -> ! {
    todo!("0x75b4ac RBX::SpatialFilter::SpatialFilter(RBX::IStage *,RBX::World *)")
}
// 0x75b664 — __ZN3RBX13SpatialFilterD0Ev
// type: void __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::~SpatialFilter()")]
#[doc(alias = "__ZN3RBX13SpatialFilterD0Ev")]
pub fn stub_75b664() -> ! {
    todo!("0x75b664 RBX::SpatialFilter::~SpatialFilter()")
}
// 0x75b704 — __ZN3RBX13SpatialFilterD1Ev
// type: void __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::~SpatialFilter()")]
#[doc(alias = "__ZN3RBX13SpatialFilterD1Ev")]
pub fn stub_75b704() -> ! {
    todo!("0x75b704 RBX::SpatialFilter::~SpatialFilter()")
}
// 0x75b708 — __ZN3RBX13SpatialFilterD2Ev
// type: void __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::~SpatialFilter()")]
#[doc(alias = "__ZN3RBX13SpatialFilterD2Ev")]
pub fn stub_75b708() -> ! {
    todo!("0x75b708 RBX::SpatialFilter::~SpatialFilter()")
}
// 0x75b8ec — __ZN3RBX13SpatialFilter11changePhaseERNS0_16MoveInstructionsE
#[doc(alias = "RBX::SpatialFilter::changePhase(RBX::SpatialFilter::MoveInstructions &)")]
#[doc(alias = "__ZN3RBX13SpatialFilter11changePhaseERNS0_16MoveInstructionsE")]
pub fn stub_75b8ec() -> ! {
    todo!("0x75b8ec RBX::SpatialFilter::changePhase(RBX::SpatialFilter::MoveInstructions &)")
}
// 0x75ba70 — __ZN3RBX13SpatialFilter8moveIntoERNS0_16MoveInstructionsE
// type: int __fastcall(RBX::IStage *)
#[doc(alias = "RBX::SpatialFilter::moveInto(RBX::SpatialFilter::MoveInstructions &)")]
#[doc(alias = "__ZN3RBX13SpatialFilter8moveIntoERNS0_16MoveInstructionsE")]
pub fn stub_75ba70() -> ! {
    todo!("0x75ba70 RBX::SpatialFilter::moveInto(RBX::SpatialFilter::MoveInstructions &)")
}
// 0x75bd10 — __ZN3RBX13SpatialFilter12addressMatchEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::addressMatch(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter12addressMatchEPNS_8AssemblyE")]
pub fn stub_75bd10() -> ! {
    todo!("0x75bd10 RBX::SpatialFilter::addressMatch(RBX::Assembly *)")
}
// 0x75bd34 — __ZN3RBX13SpatialFilter18isNotClientAddressEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::isNotClientAddress(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter18isNotClientAddressEPNS_8AssemblyE")]
pub fn stub_75bd34() -> ! {
    todo!("0x75bd34 RBX::SpatialFilter::isNotClientAddress(RBX::Assembly *)")
}
// 0x75bdc0 — __ZN3RBX13SpatialFilter17inClientSimRegionEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::inClientSimRegion(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter17inClientSimRegionEPNS_8AssemblyE")]
pub fn stub_75bdc0() -> ! {
    todo!("0x75bdc0 RBX::SpatialFilter::inClientSimRegion(RBX::Assembly *)")
}
// 0x75be00 — __ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::updateNetworkIsSleeping(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE")]
pub fn stub_75be00() -> ! {
    todo!("0x75be00 RBX::updateNetworkIsSleeping(RBX::Assembly *)")
}
// 0x75be24 — __ZN3RBX13SpatialFilter14filterAssemblyEPNS_8AssemblyEb
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *, bool)
#[doc(alias = "RBX::SpatialFilter::filterAssembly(RBX::Assembly *,bool)")]
#[doc(alias = "__ZN3RBX13SpatialFilter14filterAssemblyEPNS_8AssemblyEb")]
pub fn stub_75be24() -> ! {
    todo!("0x75be24 RBX::SpatialFilter::filterAssembly(RBX::Assembly *,bool)")
}
// 0x75bf20 — __ZN3RBX13SpatialFilter16filterAssembliesEv
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::filterAssemblies(void)")]
#[doc(alias = "__ZN3RBX13SpatialFilter16filterAssembliesEv")]
pub fn stub_75bf20() -> ! {
    todo!("0x75bf20 RBX::SpatialFilter::filterAssemblies(void)")
}
// 0x75c1d0 — __ZN3RBX13SpatialFilter7moveAllENS_8Assembly11FilterPhaseE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::SpatialFilter::moveAll(RBX::Assembly::FilterPhase)")]
#[doc(alias = "__ZN3RBX13SpatialFilter7moveAllENS_8Assembly11FilterPhaseE")]
pub fn stub_75c1d0() -> ! {
    todo!("0x75c1d0 RBX::SpatialFilter::moveAll(RBX::Assembly::FilterPhase)")
}
// 0x75c3b8 — __ZN3RBX13SpatialFilter10filterStepEv
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::filterStep(void)")]
#[doc(alias = "__ZN3RBX13SpatialFilter10filterStepEv")]
pub fn stub_75c3b8() -> ! {
    todo!("0x75c3b8 RBX::SpatialFilter::filterStep(void)")
}
// 0x75c3d8 — __ZN3RBX13SpatialFilter25onMovingAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::onMovingAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter25onMovingAssemblyRootAddedEPNS_8AssemblyE")]
pub fn stub_75c3d8() -> ! {
    todo!("0x75c3d8 RBX::SpatialFilter::onMovingAssemblyRootAdded(RBX::Assembly *)")
}
// 0x75c4a8 — __ZN3RBX13SpatialFilter24onFixedAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::onFixedAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter24onFixedAssemblyRootAddedEPNS_8AssemblyE")]
pub fn stub_75c4a8() -> ! {
    todo!("0x75c4a8 RBX::SpatialFilter::onFixedAssemblyRootAdded(RBX::Assembly *)")
}
// 0x75c570 — __ZN3RBX13SpatialFilter22onAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::onAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter22onAssemblyRootRemovingEPNS_8AssemblyE")]
pub fn stub_75c570() -> ! {
    todo!("0x75c570 RBX::SpatialFilter::onAssemblyRootRemoving(RBX::Assembly *)")
}
// 0x75c5f4 — __ZNK3RBX10IPipelined17downstreamOfStageEPNS_6IStageE
// type: _DWORD __fastcall(RBX::IPipelined *__hidden this, RBX::IStage *)
#[doc(alias = "RBX::IPipelined::downstreamOfStage(RBX::IStage *)const")]
#[doc(alias = "__ZNK3RBX10IPipelined17downstreamOfStageEPNS_6IStageE")]
pub fn stub_75c5f4() -> ! {
    todo!("0x75c5f4 RBX::IPipelined::downstreamOfStage(RBX::IStage *)const")
}
// 0x75c804 — __ZNK3RBX13SpatialFilter12getStageTypeEv
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX13SpatialFilter12getStageTypeEv")]
pub fn stub_75c804() -> ! {
    todo!("0x75c804 RBX::SpatialFilter::getStageType(void)const")
}
// 0x75c808 — __ZN3RBX6IStage9getKernelEv
// type: _DWORD __fastcall(RBX::IStage *__hidden this)
#[doc(alias = "RBX::IStage::getKernel(void)")]
#[doc(alias = "__ZN3RBX6IStage9getKernelEv")]
pub fn stub_75c808() -> ! {
    todo!("0x75c808 RBX::IStage::getKernel(void)")
}
// 0x75c868 — __ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE
#[doc(alias = "RBX::IWorldStage::getMetric(RBX::IWorldStage::MetricType)")]
#[doc(alias = "__ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE")]
pub fn stub_75c868() -> ! {
    todo!("0x75c868 RBX::IWorldStage::getMetric(RBX::IWorldStage::MetricType)")
}
// 0x75cb98 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert_unique(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
pub fn stub_75cb98() -> ! {
    todo!("0x75cb98 std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert_unique(RBX::Assembly * const&)")
}
// 0x75cc00 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
pub fn stub_75cc00() -> ! {
    todo!("0x75cc00 std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Assembly * const&)")
}
// 0x75cc58 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")]
pub fn stub_75cc58() -> ! {
    todo!("0x75cc58 std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(RBX::Assembly * const&)")
}
// 0x75cc80 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::equal_range(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")]
pub fn stub_75cc80() -> ! {
    todo!("0x75cc80 std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::equal_range(RBX::Assembly * const&)")
}
// 0x75cccc — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(std::_Rb_tree_iterator<RBX::Assembly *>,std::_Rb_tree_iterator<RBX::Assembly *>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")]
pub fn stub_75cccc() -> ! {
    todo!("0x75cccc std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(std::_Rb_tree_iterator<RBX::Assembly *>,std::_Rb_tree_iterator<RBX::Assembly *>)")
}
// 0x75cd2c — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_erase(std::_Rb_tree_node<RBX::Assembly *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
pub fn stub_75cd2c() -> ! {
    todo!("0x75cd2c std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_erase(std::_Rb_tree_node<RBX::Assembly *> *)")
}
// 0x75cf18 — __ZN3RBX6IStageD1Ev
// type: void __fastcall(RBX::IStage *__hidden this)
#[doc(alias = "RBX::IStage::~IStage()")]
#[doc(alias = "__ZN3RBX6IStageD1Ev")]
pub fn stub_75cf18() -> ! {
    todo!("0x75cf18 RBX::IStage::~IStage()")
}
// 0x75cf3c — __ZN3RBX6IStageD0Ev
// type: void __fastcall(RBX::IStage *__hidden this)
#[doc(alias = "RBX::IStage::~IStage()")]
#[doc(alias = "__ZN3RBX6IStageD0Ev")]
pub fn stub_75cf3c() -> ! {
    todo!("0x75cf3c RBX::IStage::~IStage()")
}
// 0x75d1b8 — __GLOBAL__I_a_352
#[doc(alias = "global constructor keyed to_a_352")]
#[doc(alias = "__GLOBAL__I_a_352")]
pub fn stub_75d1b8() -> ! {
    todo!("0x75d1b8 `global constructor keyed to'_a_352")
}
// 0x75d45c — __ZN3RBX17SpatialHashStatic7getHashEiRKNS_12Vector3int32E
// type: _DWORD __fastcall(RBX::SpatialHashStatic *__hidden this, int, const RBX::Vector3int32 *)
#[doc(alias = "RBX::SpatialHashStatic::getHash(int,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX17SpatialHashStatic7getHashEiRKNS_12Vector3int32E")]
pub fn stub_75d45c() -> ! {
    todo!("0x75d45c RBX::SpatialHashStatic::getHash(int,RBX::Vector3int32 const&)")
}
// 0x75d534 — __ZN3RBX17SpatialHashStatic13computeMinMaxEiRKNS_7ExtentsERNS_12Vector3int32ES5_
// type: _DWORD __fastcall(RBX::SpatialHashStatic *__hidden this, int, const RBX::Extents *, RBX::Vector3int32 *, RBX::Vector3int32 *)
#[doc(alias = "RBX::SpatialHashStatic::computeMinMax(int,RBX::Extents const&,RBX::Vector3int32 &,RBX::Vector3int32 &)")]
#[doc(alias = "__ZN3RBX17SpatialHashStatic13computeMinMaxEiRKNS_7ExtentsERNS_12Vector3int32ES5_")]
pub fn stub_75d534() -> ! {
    todo!("0x75d534 RBX::SpatialHashStatic::computeMinMax(int,RBX::Extents const&,RBX::Vector3int32 &,RBX::Vector3int32 &)")
}
// 0x75d650 — __GLOBAL__I_a_353
#[doc(alias = "global constructor keyed to_a_353")]
#[doc(alias = "__GLOBAL__I_a_353")]
pub fn stub_75d650() -> ! {
    todo!("0x75d650 `global constructor keyed to'_a_353")
}
// 0x75d8ec — __ZN3RBX15StepJointsStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::StepJointsStage::StepJointsStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX15StepJointsStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75d8ec() -> ! {
    todo!("0x75d8ec RBX::StepJointsStage::StepJointsStage(RBX::IStage *,RBX::World *)")
}
// 0x75d8f0 — __ZN3RBX15StepJointsStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::StepJointsStage::StepJointsStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX15StepJointsStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75d8f0() -> ! {
    todo!("0x75d8f0 RBX::StepJointsStage::StepJointsStage(RBX::IStage *,RBX::World *)")
}
// 0x75da80 — __ZN3RBX15StepJointsStageD0Ev
// type: void __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::~StepJointsStage()")]
#[doc(alias = "__ZN3RBX15StepJointsStageD0Ev")]
pub fn stub_75da80() -> ! {
    todo!("0x75da80 RBX::StepJointsStage::~StepJointsStage()")
}
