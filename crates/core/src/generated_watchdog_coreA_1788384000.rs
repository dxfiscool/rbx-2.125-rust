//! core watchdog gap A — 150 core stubs EA-sorted 0x755834..
//! Source: ida/export.json (85545 funcs) filtered core namespace (excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound), EA-sorted, 150 uncovered from 0x755834 skipping global_eas.txt
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x755834 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,unsigned long,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_755834() {
    // IDA 0x755834: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75599c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker *>(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")]
pub fn stub_75599c() {
    // IDA 0x75599c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x755af4 — __ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_755af4() {
    // IDA 0x755af4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x755af8 — __ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_755af8() {
    // IDA 0x755af8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x755bf0 — __ZN3RBX13SimulateStageD0Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD0Ev")]
pub fn stub_755bf0() {
    // IDA 0x755bf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x755c90 — __ZN3RBX13SimulateStageD1Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD1Ev")]
pub fn stub_755c90() {
    // IDA 0x755c90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x755c94 — __ZN3RBX13SimulateStageD2Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD2Ev")]
pub fn stub_755c94() {
    // IDA 0x755c94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x755f34 — __ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::onAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE")]
pub fn stub_755f34() {
    // IDA 0x755f34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x756070 — __ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::putFirstMovingRootInSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE")]
pub fn stub_756070() {
    // IDA 0x756070: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x756130 — __ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::onAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE")]
pub fn stub_756130() {
    // IDA 0x756130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7561ac — __ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::removeLastMovingRootFromSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE")]
pub fn stub_7561ac() {
    // IDA 0x7561ac: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75627c — __ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::removeFromSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE")]
pub fn stub_75627c() {
    // IDA 0x75627c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7562f8 — __ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SimulateStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_7562f8() {
    // IDA 0x7562f8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x756320 — __ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SimulateStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE")]
pub fn stub_756320() {
    // IDA 0x756320: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75633c — __ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v
#[doc(alias = "RBX::Assembly * RBX::IndexedTree::getOneBelowRoot<RBX::Assembly>(void)")]
#[doc(alias = "__ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v")]
pub fn stub_75633c() {
    // IDA 0x75633c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7563a8 — __ZNK3RBX13SimulateStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX13SimulateStage12getStageTypeEv")]
pub fn stub_7563a8() {
    // IDA 0x7563a8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7563ac — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert_unique(std::pair<RBX::Assembly * const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_7563ac() {
    // IDA 0x7563ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x756414 — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Assembly * const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_756414() {
    // IDA 0x756414: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75646c — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,int>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_75646c() {
    // IDA 0x75646c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75655c — __ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75655c() {
    // IDA 0x75655c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x756560 — __ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_756560() {
    // IDA 0x756560: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x756a74 — __ZN3RBX10SleepStageD0Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::~SleepStage()")]
#[doc(alias = "__ZN3RBX10SleepStageD0Ev")]
pub fn stub_756a74() {
    // IDA 0x756a74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x756b14 — __ZN3RBX10SleepStageD1Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::~SleepStage()")]
#[doc(alias = "__ZN3RBX10SleepStageD1Ev")]
pub fn stub_756b14() {
    // IDA 0x756b14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x756b18 — __ZN3RBX10SleepStageD2Ev
// type: void __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::~SleepStage()")]
#[doc(alias = "__ZN3RBX10SleepStageD2Ev")]
pub fn stub_756b18() {
    // IDA 0x756b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7573d8 — __ZN3RBX10SleepStage14stepSleepStageEiib
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, int, int, bool)
#[doc(alias = "RBX::SleepStage::stepSleepStage(int,int,bool)")]
#[doc(alias = "__ZN3RBX10SleepStage14stepSleepStageEiib")]
pub fn stub_7573d8() {
    // IDA 0x7573d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x7578ac — __ZN3RBX10SleepStage10doContactsERA2_NS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE
#[doc(alias = "RBX::SleepStage::doContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> (&)[2])")]
#[doc(alias = "__ZN3RBX10SleepStage10doContactsERA2_NS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE")]
pub fn stub_7578ac() {
    // IDA 0x7578ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75798c — __ZN3RBX10SleepStage19stepAssembliesAwakeEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::stepAssembliesAwake(void)")]
#[doc(alias = "__ZN3RBX10SleepStage19stepAssembliesAwakeEv")]
pub fn stub_75798c() {
    // IDA 0x75798c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x757c2c — __ZN3RBX10SleepStage30stepAssembliesSleepingCheckingEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::stepAssembliesSleepingChecking(void)")]
#[doc(alias = "__ZN3RBX10SleepStage30stepAssembliesSleepingCheckingEv")]
pub fn stub_757c2c() {
    // IDA 0x757c2c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x757ea8 — __ZN3RBX10SleepStage10stepJointsEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::stepJoints(void)")]
#[doc(alias = "__ZN3RBX10SleepStage10stepJointsEv")]
pub fn stub_757ea8() {
    // IDA 0x757ea8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x7580dc — __ZN3RBX10SleepStage12stepContactsERNS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE
#[doc(alias = "RBX::SleepStage::stepContacts(RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc> &)")]
#[doc(alias = "__ZN3RBX10SleepStage12stepContactsERNS_10IndexArrayINS_7ContactEXadL_ZNS2_17steppingIndexFuncEvEEEE")]
pub fn stub_7580dc() {
    // IDA 0x7580dc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x758624 — __ZN3RBX10SleepStage14wakeAssembliesERSt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEiNS_3Sim13AssemblyStateE
// type: int __fastcall(int, int, int, void *)
#[doc(alias = "RBX::SleepStage::wakeAssemblies(std::set<RBX::Assembly *,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>> &,int,RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage14wakeAssembliesERSt3setIPNS_8AssemblyESt4lessIS3_ESaIS3_EEiNS_3Sim13AssemblyStateE")]
pub fn stub_758624() {
    // IDA 0x758624: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758958 — __ZN3RBX10SleepStage8traverseEPNS_8AssemblyERSt5dequeIS2_SaIS2_EEi
// type: int __fastcall(int, RBX::Assembly *this)
#[doc(alias = "RBX::SleepStage::traverse(RBX::Assembly *,std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>> &,int)")]
#[doc(alias = "__ZN3RBX10SleepStage8traverseEPNS_8AssemblyERSt5dequeIS2_SaIS2_EEi")]
pub fn stub_758958() {
    // IDA 0x758958: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758adc — __ZN3RBX11canThrottleEPNS_4EdgeE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::canThrottle(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX11canThrottleEPNS_4EdgeE")]
pub fn stub_758adc() {
    // IDA 0x758adc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758b40 — __ZN3RBX10SleepStage18changeContactStateERKSt6vectorIPNS_7ContactESaIS3_EENS_3Sim9EdgeStateE
#[doc(alias = "RBX::SleepStage::changeContactState(std::vector<RBX::Contact *,std::allocator<RBX::Contact *>> const&,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage18changeContactStateERKSt6vectorIPNS_7ContactESaIS3_EENS_3Sim9EdgeStateE")]
pub fn stub_758b40() {
    // IDA 0x758b40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758b78 — __ZN3RBX10SleepStage16changeJointStateERKSt6vectorIPNS_5JointESaIS3_EENS_3Sim9EdgeStateE
#[doc(alias = "RBX::SleepStage::changeJointState(std::vector<RBX::Joint *,std::allocator<RBX::Joint *>> const&,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage16changeJointStateERKSt6vectorIPNS_5JointESaIS3_EENS_3Sim9EdgeStateE")]
pub fn stub_758b78() {
    // IDA 0x758b78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758bb0 — __ZN3RBX10SleepStage25computeStateFromNeighborsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::computeStateFromNeighbors(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage25computeStateFromNeighborsEPNS_8AssemblyE")]
pub fn stub_758bb0() {
    // IDA 0x758bb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758c4c — __ZN3RBX10SleepStage19changeAssemblyStateERKSt6vectorIPNS_8AssemblyESaIS3_EENS_3Sim13AssemblyStateE
#[doc(alias = "RBX::SleepStage::changeAssemblyState(std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>> const&,RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage19changeAssemblyStateERKSt6vectorIPNS_8AssemblyESaIS3_EENS_3Sim13AssemblyStateE")]
pub fn stub_758c4c() {
    // IDA 0x758c4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758c84 — __ZN3RBX10SleepStage19changeAssemblyStateEPNS_8AssemblyENS_3Sim13AssemblyStateE
// type: int __fastcall(RBX::IStage *, RBX::IPipelined *this)
#[doc(alias = "RBX::SleepStage::changeAssemblyState(RBX::Assembly *,RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage19changeAssemblyStateEPNS_8AssemblyENS_3Sim13AssemblyStateE")]
pub fn stub_758c84() {
    // IDA 0x758c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x758f40 — __ZN3RBX10SleepStage8wakeEdgeEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::wakeEdge(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage8wakeEdgeEPNS_4EdgeE")]
pub fn stub_758f40() {
    // IDA 0x758f40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x759080 — __ZN3RBX10SleepStage11isAffectingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::isAffecting(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage11isAffectingEPNS_4EdgeE")]
pub fn stub_759080() {
    // IDA 0x759080: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x759144 — __ZN3RBX10SleepStage18changeContactStateEPNS_7ContactENS_3Sim9EdgeStateE
#[doc(alias = "RBX::SleepStage::changeContactState(RBX::Contact *,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage18changeContactStateEPNS_7ContactENS_3Sim9EdgeStateE")]
pub fn stub_759144() {
    // IDA 0x759144: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75942c — __ZN3RBX10SleepStage16changeJointStateEPNS_5JointENS_3Sim9EdgeStateE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::SleepStage::changeJointState(RBX::Joint *,RBX::Sim::EdgeState)")]
#[doc(alias = "__ZN3RBX10SleepStage16changeJointStateEPNS_5JointENS_3Sim9EdgeStateE")]
pub fn stub_75942c() {
    // IDA 0x75942c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x759578 — __ZN3RBX10SleepStage9wakeEventEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::wakeEvent(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage9wakeEventEPNS_4EdgeE")]
pub fn stub_759578() {
    // IDA 0x759578: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x75959c — __ZN3RBX10SleepStage9wakeEventEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::wakeEvent(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage9wakeEventEPNS_8AssemblyE")]
pub fn stub_75959c() {
    // IDA 0x75959c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7595c0 — __ZN3RBX10SleepStage18recursiveWakeEventEPNS_7ContactE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Contact *)
#[doc(alias = "RBX::SleepStage::recursiveWakeEvent(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX10SleepStage18recursiveWakeEventEPNS_7ContactE")]
pub fn stub_7595c0() {
    // IDA 0x7595c0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7595e4 — __ZN3RBX10SleepStage18recursiveWakeEventEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::recursiveWakeEvent(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage18recursiveWakeEventEPNS_8AssemblyE")]
pub fn stub_7595e4() {
    // IDA 0x7595e4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x759608 — __ZN3RBX10SleepStage20highVelocityNewTouchEPNS_7ContactE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Contact *)
#[doc(alias = "RBX::SleepStage::highVelocityNewTouch(RBX::Contact *)")]
#[doc(alias = "__ZN3RBX10SleepStage20highVelocityNewTouchEPNS_7ContactE")]
pub fn stub_759608() {
    // IDA 0x759608: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7596f0 — __ZN3RBX10SleepStage10stateToSetENS_3Sim13AssemblyStateE
#[doc(alias = "RBX::SleepStage::stateToSet(RBX::Sim::AssemblyState)")]
#[doc(alias = "__ZN3RBX10SleepStage10stateToSetENS_3Sim13AssemblyStateE")]
pub fn stub_7596f0() {
    // IDA 0x7596f0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x759778 — __ZN3RBX10SleepStage24onExternalTickleAssemblyEPNS_8AssemblyEb
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *, bool)
#[doc(alias = "RBX::SleepStage::onExternalTickleAssembly(RBX::Assembly *,bool)")]
#[doc(alias = "__ZN3RBX10SleepStage24onExternalTickleAssemblyEPNS_8AssemblyEb")]
pub fn stub_759778() {
    // IDA 0x759778: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7597c0 — __ZN3RBX10SleepStage15onAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::onAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage15onAssemblyAddedEPNS_8AssemblyE")]
pub fn stub_7597c0() {
    // IDA 0x7597c0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x7599fc — __ZN3RBX10SleepStage18onAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SleepStage::onAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX10SleepStage18onAssemblyRemovingEPNS_8AssemblyE")]
pub fn stub_7599fc() {
    // IDA 0x7599fc: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x759b68 — __ZN3RBX10SleepStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_759b68() {
    // IDA 0x759b68: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x759d40 — __ZN3RBX10SleepStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SleepStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX10SleepStage14onEdgeRemovingEPNS_4EdgeE")]
pub fn stub_759d40() {
    // IDA 0x759d40: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x759ed8 — __ZN3RBX10SleepStage9getMetricENS_11IWorldStage10MetricTypeE
#[doc(alias = "RBX::SleepStage::getMetric(RBX::IWorldStage::MetricType)")]
#[doc(alias = "__ZN3RBX10SleepStage9getMetricENS_11IWorldStage10MetricTypeE")]
pub fn stub_759ed8() {
    // IDA 0x759ed8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x759efc — __ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE
// type: _DWORD __fastcall(RBX::IPipelined *__hidden this, RBX::IStage *)
#[doc(alias = "RBX::IPipelined::inOrDownstreamOfStage(RBX::IStage *)const")]
#[doc(alias = "__ZNK3RBX10IPipelined21inOrDownstreamOfStageEPNS_6IStageE")]
pub fn stub_759efc() {
    // IDA 0x759efc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x759fbc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_")]
pub fn stub_759fbc() {
    // IDA 0x759fbc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x759fdc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9pop_frontEv
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE9pop_frontEv")]
pub fn stub_759fdc() {
    // IDA 0x759fdc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a00c — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::push_back(RBX::Contact * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE9push_backERKS2_")]
pub fn stub_75a00c() {
    // IDA 0x75a00c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a038 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::resize(unsigned long,RBX::Contact *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE6resizeEmS2_")]
pub fn stub_75a038() {
    // IDA 0x75a038: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a06c — __ZNSt6vectorIPN3RBX5JointESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::resize(unsigned long,RBX::Joint *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE6resizeEmS2_")]
pub fn stub_75a06c() {
    // IDA 0x75a06c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a0a0 — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::push_back(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE9push_backERKS2_")]
pub fn stub_75a0a0() {
    // IDA 0x75a0a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a0cc — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::resize(unsigned long,RBX::Assembly *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE6resizeEmS2_")]
pub fn stub_75a0cc() {
    // IDA 0x75a0cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a100 — __ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_
#[doc(alias = "RBX::IndexArray<RBX::Contact,&RBX::Contact::steppingIndexFunc>::fastRemove(RBX::Contact*)")]
#[doc(alias = "__ZN3RBX10IndexArrayINS_7ContactEXadL_ZNS1_17steppingIndexFuncEvEEE10fastRemoveEPS1_")]
pub fn stub_75a100() {
    // IDA 0x75a100: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a1d4 — __ZN3RBX4Body24resetImpulseAccumulatorsEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::resetImpulseAccumulators(void)")]
#[doc(alias = "__ZN3RBX4Body24resetImpulseAccumulatorsEv")]
pub fn stub_75a1d4() {
    // IDA 0x75a1d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a208 — __ZN3RBX4Body22resetForceAccumulatorsEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::resetForceAccumulators(void)")]
#[doc(alias = "__ZN3RBX4Body22resetForceAccumulatorsEv")]
pub fn stub_75a208() {
    // IDA 0x75a208: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a240 — __ZNK3RBX10SleepStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this)
#[doc(alias = "RBX::SleepStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX10SleepStage12getStageTypeEv")]
pub fn stub_75a240() {
    // IDA 0x75a240: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

// 0x75a540 — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,unsigned long,RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_75a540() {
    // IDA 0x75a540: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a6a8 — __ZNSt12_Vector_baseIPN3RBX8AssemblyESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX8AssemblyESaIS2_EE11_M_allocateEm")]
pub fn stub_75a6a8() {
    // IDA 0x75a6a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a6c0 — __ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Assembly **,std::vector<RBX::Assembly *,std::allocator<RBX::Assembly *>>>,RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX8AssemblyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_75a6c0() {
    // IDA 0x75a6c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a7a0 — __ZNSt6vectorIPN3RBX5JointESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Joint **,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>,unsigned long,RBX::Joint * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX5JointESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_75a7a0() {
    // IDA 0x75a7a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75a908 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,unsigned long,RBX::Contact * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_75a908() {
    // IDA 0x75a908: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75aa70 — __ZNSt12_Vector_baseIPN3RBX7ContactESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX7ContactESaIS2_EE11_M_allocateEm")]
pub fn stub_75aa70() {
    // IDA 0x75aa70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75aa88 — __ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Contact **,std::vector<RBX::Contact *,std::allocator<RBX::Contact *>>>,RBX::Contact * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX7ContactESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_75aa88() {
    // IDA 0x75aa88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75ab68 — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_push_back_aux(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE16_M_push_back_auxERKS2_")]
pub fn stub_75ab68() {
    // IDA 0x75ab68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75aba0 — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE22_M_reserve_map_at_backEm")]
pub fn stub_75aba0() {
    // IDA 0x75aba0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75abbc — __ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIPN3RBX8AssemblyESaIS2_EE17_M_reallocate_mapEmb")]
pub fn stub_75abbc() {
    // IDA 0x75abbc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75ac94 — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_allocate_mapEm")]
pub fn stub_75ac94() {
    // IDA 0x75ac94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75acac — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EED2Ev
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EED2Ev")]
pub fn stub_75acac() {
    // IDA 0x75acac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75acd8 — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE17_M_initialize_mapEm")]
pub fn stub_75acd8() {
    // IDA 0x75acd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ae30 — __ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_create_nodesEPPS2_S6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::Assembly *,std::allocator<RBX::Assembly *>>::_M_create_nodes(RBX::Assembly ***,RBX::Assembly ***)")]
#[doc(alias = "__ZNSt11_Deque_baseIPN3RBX8AssemblyESaIS2_EE15_M_create_nodesEPPS2_S6_")]
pub fn stub_75ae30() {
    // IDA 0x75ae30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75b24c — __ZN3RBX9SnapJoint18compatibleSurfacesEPNS_9PrimitiveES2_NS_8NormalIdES3_
#[doc(alias = "RBX::SnapJoint::compatibleSurfaces(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9SnapJoint18compatibleSurfacesEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
pub fn stub_75b24c() {
    // IDA 0x75b24c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75b2a0 — __ZN3RBX9SnapJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::SnapJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9SnapJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
pub fn stub_75b2a0() {
    // IDA 0x75b2a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75b4a8 — __ZN3RBX13SpatialFilterC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SpatialFilter::SpatialFilter(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SpatialFilterC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75b4a8() {
    // IDA 0x75b4a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75b4ac — __ZN3RBX13SpatialFilterC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SpatialFilter::SpatialFilter(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SpatialFilterC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75b4ac() {
    // IDA 0x75b4ac: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75b664 — __ZN3RBX13SpatialFilterD0Ev
// type: void __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::~SpatialFilter()")]
#[doc(alias = "__ZN3RBX13SpatialFilterD0Ev")]
pub fn stub_75b664() {
    // IDA 0x75b664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75b704 — __ZN3RBX13SpatialFilterD1Ev
// type: void __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::~SpatialFilter()")]
#[doc(alias = "__ZN3RBX13SpatialFilterD1Ev")]
pub fn stub_75b704() {
    // IDA 0x75b704: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75b708 — __ZN3RBX13SpatialFilterD2Ev
// type: void __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::~SpatialFilter()")]
#[doc(alias = "__ZN3RBX13SpatialFilterD2Ev")]
pub fn stub_75b708() {
    // IDA 0x75b708: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75b8ec — __ZN3RBX13SpatialFilter11changePhaseERNS0_16MoveInstructionsE
#[doc(alias = "RBX::SpatialFilter::changePhase(RBX::SpatialFilter::MoveInstructions &)")]
#[doc(alias = "__ZN3RBX13SpatialFilter11changePhaseERNS0_16MoveInstructionsE")]
pub fn stub_75b8ec() {
    // IDA 0x75b8ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ba70 — __ZN3RBX13SpatialFilter8moveIntoERNS0_16MoveInstructionsE
// type: int __fastcall(RBX::IStage *)
#[doc(alias = "RBX::SpatialFilter::moveInto(RBX::SpatialFilter::MoveInstructions &)")]
#[doc(alias = "__ZN3RBX13SpatialFilter8moveIntoERNS0_16MoveInstructionsE")]
pub fn stub_75ba70() {
    // IDA 0x75ba70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75bd10 — __ZN3RBX13SpatialFilter12addressMatchEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::addressMatch(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter12addressMatchEPNS_8AssemblyE")]
pub fn stub_75bd10() {
    // IDA 0x75bd10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75bd34 — __ZN3RBX13SpatialFilter18isNotClientAddressEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::isNotClientAddress(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter18isNotClientAddressEPNS_8AssemblyE")]
pub fn stub_75bd34() {
    // IDA 0x75bd34: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75bdc0 — __ZN3RBX13SpatialFilter17inClientSimRegionEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::inClientSimRegion(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter17inClientSimRegionEPNS_8AssemblyE")]
pub fn stub_75bdc0() {
    // IDA 0x75bdc0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75be24 — __ZN3RBX13SpatialFilter14filterAssemblyEPNS_8AssemblyEb
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *, bool)
#[doc(alias = "RBX::SpatialFilter::filterAssembly(RBX::Assembly *,bool)")]
#[doc(alias = "__ZN3RBX13SpatialFilter14filterAssemblyEPNS_8AssemblyEb")]
pub fn stub_75be24() {
    // IDA 0x75be24: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75bf20 — __ZN3RBX13SpatialFilter16filterAssembliesEv
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::filterAssemblies(void)")]
#[doc(alias = "__ZN3RBX13SpatialFilter16filterAssembliesEv")]
pub fn stub_75bf20() {
    // IDA 0x75bf20: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c1d0 — __ZN3RBX13SpatialFilter7moveAllENS_8Assembly11FilterPhaseE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::SpatialFilter::moveAll(RBX::Assembly::FilterPhase)")]
#[doc(alias = "__ZN3RBX13SpatialFilter7moveAllENS_8Assembly11FilterPhaseE")]
pub fn stub_75c1d0() {
    // IDA 0x75c1d0: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c3b8 — __ZN3RBX13SpatialFilter10filterStepEv
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::filterStep(void)")]
#[doc(alias = "__ZN3RBX13SpatialFilter10filterStepEv")]
pub fn stub_75c3b8() {
    // IDA 0x75c3b8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c3d8 — __ZN3RBX13SpatialFilter25onMovingAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::onMovingAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter25onMovingAssemblyRootAddedEPNS_8AssemblyE")]
pub fn stub_75c3d8() {
    // IDA 0x75c3d8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c4a8 — __ZN3RBX13SpatialFilter24onFixedAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::onFixedAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter24onFixedAssemblyRootAddedEPNS_8AssemblyE")]
pub fn stub_75c4a8() {
    // IDA 0x75c4a8: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c570 — __ZN3RBX13SpatialFilter22onAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SpatialFilter::onAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SpatialFilter22onAssemblyRootRemovingEPNS_8AssemblyE")]
pub fn stub_75c570() {
    // IDA 0x75c570: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c5f4 — __ZNK3RBX10IPipelined17downstreamOfStageEPNS_6IStageE
// type: _DWORD __fastcall(RBX::IPipelined *__hidden this, RBX::IStage *)
#[doc(alias = "RBX::IPipelined::downstreamOfStage(RBX::IStage *)const")]
#[doc(alias = "__ZNK3RBX10IPipelined17downstreamOfStageEPNS_6IStageE")]
pub fn stub_75c5f4() {
    // IDA 0x75c5f4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c804 — __ZNK3RBX13SpatialFilter12getStageTypeEv
// type: _DWORD __fastcall(RBX::SpatialFilter *__hidden this)
#[doc(alias = "RBX::SpatialFilter::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX13SpatialFilter12getStageTypeEv")]
pub fn stub_75c804() {
    // IDA 0x75c804: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c808 — __ZN3RBX6IStage9getKernelEv
// type: _DWORD __fastcall(RBX::IStage *__hidden this)
#[doc(alias = "RBX::IStage::getKernel(void)")]
#[doc(alias = "__ZN3RBX6IStage9getKernelEv")]
pub fn stub_75c808() {
    // IDA 0x75c808: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75c868 — __ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE
#[doc(alias = "RBX::IWorldStage::getMetric(RBX::IWorldStage::MetricType)")]
#[doc(alias = "__ZN3RBX11IWorldStage9getMetricENS0_10MetricTypeE")]
pub fn stub_75c868() {
    // IDA 0x75c868: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x75cb98 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert_unique(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
pub fn stub_75cb98() {
    // IDA 0x75cb98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75cc00 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
pub fn stub_75cc00() {
    // IDA 0x75cc00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75cc58 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")]
pub fn stub_75cc58() {
    // IDA 0x75cc58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75cc80 — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::equal_range(RBX::Assembly * const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")]
pub fn stub_75cc80() {
    // IDA 0x75cc80: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75cccc — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::erase(std::_Rb_tree_iterator<RBX::Assembly *>,std::_Rb_tree_iterator<RBX::Assembly *>)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")]
pub fn stub_75cccc() {
    // IDA 0x75cccc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75cd2c — __ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,RBX::Assembly *,std::_Identity<RBX::Assembly *>,std::less<RBX::Assembly *>,std::allocator<RBX::Assembly *>>::_M_erase(std::_Rb_tree_node<RBX::Assembly *> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
pub fn stub_75cd2c() {
    // IDA 0x75cd2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x75cf18 — __ZN3RBX6IStageD1Ev
// type: void __fastcall(RBX::IStage *__hidden this)
#[doc(alias = "RBX::IStage::~IStage()")]
#[doc(alias = "__ZN3RBX6IStageD1Ev")]
pub fn stub_75cf18() {
    // IDA 0x75cf18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75cf3c — __ZN3RBX6IStageD0Ev
// type: void __fastcall(RBX::IStage *__hidden this)
#[doc(alias = "RBX::IStage::~IStage()")]
#[doc(alias = "__ZN3RBX6IStageD0Ev")]
pub fn stub_75cf3c() {
    // IDA 0x75cf3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75d45c — __ZN3RBX17SpatialHashStatic7getHashEiRKNS_12Vector3int32E
// type: _DWORD __fastcall(RBX::SpatialHashStatic *__hidden this, int, const RBX::Vector3int32 *)
#[doc(alias = "RBX::SpatialHashStatic::getHash(int,RBX::Vector3int32 const&)")]
#[doc(alias = "__ZN3RBX17SpatialHashStatic7getHashEiRKNS_12Vector3int32E")]
pub fn stub_75d45c() {
    // IDA 0x75d45c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75d534 — __ZN3RBX17SpatialHashStatic13computeMinMaxEiRKNS_7ExtentsERNS_12Vector3int32ES5_
// type: _DWORD __fastcall(RBX::SpatialHashStatic *__hidden this, int, const RBX::Extents *, RBX::Vector3int32 *, RBX::Vector3int32 *)
#[doc(alias = "RBX::SpatialHashStatic::computeMinMax(int,RBX::Extents const&,RBX::Vector3int32 &,RBX::Vector3int32 &)")]
#[doc(alias = "__ZN3RBX17SpatialHashStatic13computeMinMaxEiRKNS_7ExtentsERNS_12Vector3int32ES5_")]
pub fn stub_75d534() {
    // IDA 0x75d534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75d8ec — __ZN3RBX15StepJointsStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::StepJointsStage::StepJointsStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX15StepJointsStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75d8ec() {
    // IDA 0x75d8ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75d8f0 — __ZN3RBX15StepJointsStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::StepJointsStage::StepJointsStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX15StepJointsStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75d8f0() {
    // IDA 0x75d8f0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x75da80 — __ZN3RBX15StepJointsStageD0Ev
// type: void __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::~StepJointsStage()")]
#[doc(alias = "__ZN3RBX15StepJointsStageD0Ev")]
pub fn stub_75da80() {
    // IDA 0x75da80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75db20 — __ZN3RBX15StepJointsStageD1Ev
// type: void __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::~StepJointsStage()")]
#[doc(alias = "__ZN3RBX15StepJointsStageD1Ev")]
pub fn stub_75db20() {
    // IDA 0x75db20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75db24 — __ZN3RBX15StepJointsStageD2Ev
// type: void __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::~StepJointsStage()")]
#[doc(alias = "__ZN3RBX15StepJointsStageD2Ev")]
pub fn stub_75db24() {
    // IDA 0x75db24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75dd0c — __ZN3RBX15StepJointsStage11removeJointEPNS_5JointE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Joint *)
#[doc(alias = "RBX::StepJointsStage::removeJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage11removeJointEPNS_5JointE")]
pub fn stub_75dd0c() {
    // IDA 0x75dd0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75dd9c — __ZN3RBX15StepJointsStage23onSimulateAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::StepJointsStage::onSimulateAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage23onSimulateAssemblyAddedEPNS_8AssemblyE")]
pub fn stub_75dd9c() {
    // IDA 0x75dd9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ddb8 — __ZN3RBX15StepJointsStage26onSimulateAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::StepJointsStage::onSimulateAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage26onSimulateAssemblyRemovingEPNS_8AssemblyE")]
pub fn stub_75ddb8() {
    // IDA 0x75ddb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ddd4 — __ZN3RBX15StepJointsStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::StepJointsStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_75ddd4() {
    // IDA 0x75ddd4: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75de3c — __ZN3RBX15StepJointsStage14onEdgeRemovingEPNS_4EdgeE
// type: int __fastcall(RBX::StepJointsStage *this, RBX::Edge *)
#[doc(alias = "RBX::StepJointsStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX15StepJointsStage14onEdgeRemovingEPNS_4EdgeE")]
pub fn stub_75de3c() {
    // IDA 0x75de3c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75de84 — __ZN3RBX15StepJointsStage15jointsStepWorldEv
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::jointsStepWorld(void)")]
#[doc(alias = "__ZN3RBX15StepJointsStage15jointsStepWorldEv")]
pub fn stub_75de84() {
    // IDA 0x75de84: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75dfd4 — __ZNK3RBX15StepJointsStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::StepJointsStage *__hidden this)
#[doc(alias = "RBX::StepJointsStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX15StepJointsStage12getStageTypeEv")]
pub fn stub_75dfd4() {
    // IDA 0x75dfd4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x75e0a0 — __ZN3RBX9TreeStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::TreeStage::TreeStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX9TreeStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75e0a0() {
    // IDA 0x75e0a0: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x75e0a4 — __ZN3RBX9TreeStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::TreeStage::TreeStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX9TreeStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75e0a4() {
    // IDA 0x75e0a4: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

// 0x75e21c — __ZN3RBX9TreeStageD0Ev
// type: void __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::~TreeStage()")]
#[doc(alias = "__ZN3RBX9TreeStageD0Ev")]
pub fn stub_75e21c() {
    // IDA 0x75e21c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e2bc — __ZN3RBX9TreeStageD1Ev
// type: void __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::~TreeStage()")]
#[doc(alias = "__ZN3RBX9TreeStageD1Ev")]
pub fn stub_75e2bc() {
    // IDA 0x75e2bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e2c0 — __ZN3RBX9TreeStageD2Ev
// type: void __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::~TreeStage()")]
#[doc(alias = "__ZN3RBX9TreeStageD2Ev")]
pub fn stub_75e2c0() {
    // IDA 0x75e2c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e4bc — __ZN3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningNode *)
#[doc(alias = "RBX::TreeStage::validateTree(RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE")]
pub fn stub_75e4bc() {
    // IDA 0x75e4bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e4c8 — __ZThn16_N3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningNode *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::validateTree(RBX::SpanningNode *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage12validateTreeEPNS_12SpanningNodeE")]
pub fn stub_75e4c8() {
    // IDA 0x75e4c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e4d4 — __ZN3RBX13chainToGroundEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::chainToGround(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX13chainToGroundEPNS_9PrimitiveE")]
pub fn stub_75e4d4() {
    // IDA 0x75e4d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e52c — __ZN3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
pub fn stub_75e52c() {
    // IDA 0x75e52c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e69c — __ZN3RBX9TreeStage14dirtyMechanismEPNS_9MechanismE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Mechanism *)
#[doc(alias = "RBX::TreeStage::dirtyMechanism(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX9TreeStage14dirtyMechanismEPNS_9MechanismE")]
pub fn stub_75e69c() {
    // IDA 0x75e69c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e710 — __ZThn16_N3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeAdding(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage20onSpanningEdgeAddingEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
pub fn stub_75e710() {
    // IDA 0x75e710: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75e718 — __ZN3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE")]
pub fn stub_75e718() {
    // IDA 0x75e718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75edb8 — __ZN3RBX9TreeStage23sendClumpChangedMessageEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::sendClumpChangedMessage(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage23sendClumpChangedMessageEPNS_9PrimitiveE")]
pub fn stub_75edb8() {
    // IDA 0x75edb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ee8c — __ZThn16_N3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeAdded(RBX::SpanningEdge *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage19onSpanningEdgeAddedEPNS_12SpanningEdgeE")]
pub fn stub_75ee8c() {
    // IDA 0x75ee8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ee94 — __ZN3RBX19assertNotInPipelineEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::assertNotInPipeline(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX19assertNotInPipelineEPNS_8AssemblyE")]
pub fn stub_75ee94() {
    // IDA 0x75ee94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75eef8 — __ZN3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE")]
pub fn stub_75eef8() {
    // IDA 0x75eef8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ef20 — __ZThn16_N3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeRemoving(RBX::SpanningEdge *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage22onSpanningEdgeRemovingEPNS_12SpanningEdgeE")]
pub fn stub_75ef20() {
    // IDA 0x75ef20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75ef28 — __ZN3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "RBX::TreeStage::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZN3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
pub fn stub_75ef28() {
    // IDA 0x75ef28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75f22c — __ZN3RBX9TreeStage12destroyClumpEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::destroyClump(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage12destroyClumpEPNS_9PrimitiveE")]
pub fn stub_75f22c() {
    // IDA 0x75f22c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75f258 — __ZN3RBX9TreeStage15destroyAssemblyEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::destroyAssembly(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage15destroyAssemblyEPNS_9PrimitiveE")]
pub fn stub_75f258() {
    // IDA 0x75f258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75f29c — __ZN3RBX9TreeStage16destroyMechanismEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::TreeStage::destroyMechanism(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX9TreeStage16destroyMechanismEPNS_9PrimitiveE")]
pub fn stub_75f29c() {
    // IDA 0x75f29c: simulation/instance gameplay wiring owned by the datamodel crate — carrier no-op in core.
}

// 0x75f320 — __ZThn16_N3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::SpanningEdge *, RBX::SpanningNode *)
#[doc(alias = "non-virtual thunk toRBX::TreeStage::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
#[doc(alias = "__ZThn16_N3RBX9TreeStage21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE")]
pub fn stub_75f320() {
    // IDA 0x75f320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75f328 — __ZN3RBX9TreeStage18removeFromPipelineEPNS_9MechanismE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Mechanism *)
#[doc(alias = "RBX::TreeStage::removeFromPipeline(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX9TreeStage18removeFromPipelineEPNS_9MechanismE")]
pub fn stub_75f328() {
    // IDA 0x75f328: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75f3e8 — __ZN3RBX9TreeStage14cleanMechanismEPNS_9MechanismE
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this, RBX::Mechanism *)
#[doc(alias = "RBX::TreeStage::cleanMechanism(RBX::Mechanism *)")]
#[doc(alias = "__ZN3RBX9TreeStage14cleanMechanismEPNS_9MechanismE")]
pub fn stub_75f3e8() {
    // IDA 0x75f3e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x75f500 — __ZN3RBX9TreeStage8assembleEv
// type: _DWORD __fastcall(RBX::TreeStage *__hidden this)
#[doc(alias = "RBX::TreeStage::assemble(void)")]
#[doc(alias = "__ZN3RBX9TreeStage8assembleEv")]
pub fn stub_75f500() {
    // IDA 0x75f500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
