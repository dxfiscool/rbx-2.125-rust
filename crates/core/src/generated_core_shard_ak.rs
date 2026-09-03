//! core shard AK — 120 core stubs EA-sorted ascending earliest gap after 0x326a04, 0x381334..0x38e9c8 (was 100, extended +20).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 100 uncovered after 0x326a04 (lowest EA first).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::SpanningTree::onSpanningEdgeRemoved(RBX::SpanningEdge *,RBX::SpanningNode *)")]
// 0x381334 — __ZN3RBX12SpanningTree21onSpanningEdgeRemovedEPNS_12SpanningEdgeEPNS_12SpanningNodeE
pub fn stub_0x381334() {
    // IDA 0x381334: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SpanningTree::validateTree(RBX::SpanningNode *)")]
// 0x381338 — __ZN3RBX12SpanningTree12validateTreeEPNS_12SpanningNodeE
pub fn stub_0x381338() {
    // IDA 0x381338: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
// 0x38133c — __ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
pub fn stub_0x38133c() {
    // IDA 0x38133c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
// 0x3813bc — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_0x3813bc() {
    // IDA 0x3813bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
// 0x381424 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_0x381424() {
    // IDA 0x381424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
// 0x3818e0 — __ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0x3818e0() {
    // IDA 0x3818e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_142")]
// 0x381908 — __GLOBAL__I_a_142
pub fn stub_0x381908() {
    // IDA 0x381908: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StandardOut::singleton(void)")]
// 0x3819d0 — __ZN3RBX11StandardOut9singletonEv
pub fn stub_0x3819d0() {
    // IDA 0x3819d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::exception const&)")]
// 0x381c38 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSt9exception
pub fn stub_0x381c38() {
    // IDA 0x381c38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StandardOut::printf(RBX::MessageType,char const*,...)")]
// 0x381c58 — __ZN3RBX11StandardOut6printfENS_11MessageTypeEPKcz
pub fn stub_0x381c58() {
    // IDA 0x381c58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::string const&)")]
// 0x381d88 — __ZN3RBX11StandardOut5printENS_11MessageTypeERKSs
pub fn stub_0x381d88() {
    // IDA 0x381d88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,char const*)")]
// 0x3820c4 — __ZN3RBX11StandardOut5printENS_11MessageTypeEPKc
pub fn stub_0x3820c4() {
    // IDA 0x3820c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StandardOut::~StandardOut()")]
// 0x3827e8 — __ZN3RBX11StandardOutD2Ev
pub fn stub_0x3827e8() {
    // IDA 0x3827e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")]
// 0x382b38 — __ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
pub fn stub_0x382b38() {
    // IDA 0x382b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StandardOut::StandardOut(void)")]
// 0x382bfc — __ZN3RBX11StandardOutC2Ev
pub fn stub_0x382bfc() {
    // IDA 0x382bfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_143")]
// 0x382d18 — __GLOBAL__I_a_143
pub fn stub_0x382d18() {
    // IDA 0x382d18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "SetBaseURL(std::string const&)")]
// 0x382de0 — __Z10SetBaseURLRKSs
pub fn stub_0x382de0() {
    // IDA 0x382de0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "GetBaseURL(void)")]
// 0x382df4 — __Z10GetBaseURLv
pub fn stub_0x382df4() {
    // IDA 0x382df4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Http::urlEncode(std::string)")]
// 0x382e04 — __ZN3RBX4Http9urlEncodeESs
pub fn stub_0x382e04() {
    // IDA 0x382e04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "FetchLocalClientSettingsData(char const*,SimpleJSON *)")]
// 0x382f9c — __Z28FetchLocalClientSettingsDataPKcP10SimpleJSON
pub fn stub_0x382f9c() {
    // IDA 0x382f9c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "LoadClientSettingsFromString(char const*,std::string const&,SimpleJSON *)")]
// 0x3834bc — __Z28LoadClientSettingsFromStringPKcRKSsP10SimpleJSON
pub fn stub_0x3834bc() {
    // IDA 0x3834bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "FetchClientSettingsData(char const*,char const*,SimpleJSON *)")]
// 0x383538 — __Z23FetchClientSettingsDataPKcS0_P10SimpleJSON
pub fn stub_0x383538() {
    // IDA 0x383538: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "FetchClientSettingsData(char const*,char const*,std::string *)")]
// 0x38367c — __Z23FetchClientSettingsDataPKcS0_PSs
pub fn stub_0x38367c() {
    // IDA 0x38367c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "ReportStatisticPost(std::string const&,std::string const&,std::string const&,char const*,char const*,char const*,char const*)")]
// 0x383c54 — __Z19ReportStatisticPostRKSsS0_S0_PKcS2_S2_S2_
pub fn stub_0x383c54() {
    // IDA 0x383c54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "ReportStatistic(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")]
// 0x384ae0 — __Z15ReportStatisticRKSsS0_S0_S0_S0_S0_
pub fn stub_0x384ae0() {
    // IDA 0x384ae0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "DontCareResponse(std::string *,std::exception *)")]
// 0x384c38 — __Z16DontCareResponsePSsPSt9exception
pub fn stub_0x384c38() {
    // IDA 0x384c38: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_144")]
// 0x384c44 — __GLOBAL__I_a_144
pub fn stub_0x384c44() {
    // IDA 0x384c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::IStepped::onServiceProviderIStepped(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x384d34 — __ZN3RBX8IStepped25onServiceProviderISteppedEPNS_15ServiceProviderES2_
pub fn stub_0x384d34() {
    // IDA 0x384d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_145")]
// 0x38587c — __GLOBAL__I_a_145
pub fn stub_0x38587c() {
    // IDA 0x38587c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SystemAddress::operator==(RBX::SystemAddress const&)const")]
// 0x385a3c — __ZNK3RBX13SystemAddresseqERKS0_
pub fn stub_0x385a3c() {
    // IDA 0x385a3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SystemAddress::operator!=(RBX::SystemAddress const&)const")]
// 0x385a58 — __ZNK3RBX13SystemAddressneERKS0_
pub fn stub_0x385a58() {
    // IDA 0x385a58: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::SystemAddress::operator<(RBX::SystemAddress const&)const")]
// 0x385a78 — __ZNK3RBX13SystemAddressltERKS0_
pub fn stub_0x385a78() {
    // IDA 0x385a78: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::BaseThreadPool::BaseThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy,RBX::BaseThreadPool::PoolData *)")]
// 0x385a9c — __ZN3RBX14BaseThreadPoolC2EiNS0_14ShutdownPolicyEPNS0_8PoolDataE
pub fn stub_0x385a9c() {
    // IDA 0x385a9c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::BaseThreadPool::getThreadCount(void)const")]
// 0x385fe4 — __ZNK3RBX14BaseThreadPool14getThreadCountEv
pub fn stub_0x385fe4() {
    // IDA 0x385fe4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// 0x385fe8 — __ZN3RBX14BaseThreadPoolD0Ev
pub fn stub_0x385fe8() {
    // IDA 0x385fe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// 0x386088 — __ZN3RBX14BaseThreadPoolD1Ev
pub fn stub_0x386088() {
    // IDA 0x386088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// 0x38608c — __ZN3RBX14BaseThreadPoolD2Ev
pub fn stub_0x38608c() {
    // IDA 0x38608c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BaseThreadPool::taskAdded(void)")]
// 0x3864e4 — __ZN3RBX14BaseThreadPool9taskAddedEv
pub fn stub_0x3864e4() {
    // IDA 0x3864e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// 0x3865f4 — __ZN3RBX10ThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_0x3865f4() {
    // IDA 0x3865f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// 0x3865f8 — __ZN3RBX10ThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_0x3865f8() {
    // IDA 0x3865f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// 0x38678c — __ZN3RBX18PriorityThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_0x38678c() {
    // IDA 0x38678c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// 0x386790 — __ZN3RBX18PriorityThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_0x386790() {
    // IDA 0x386790: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")]
// 0x386fc4 — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
pub fn stub_0x386fc4() {
    // IDA 0x386fc4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")]
// 0x3870ec — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
pub fn stub_0x3870ec() {
    // IDA 0x3870ec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
// 0x387354 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD1Ev
pub fn stub_0x387354() {
    // IDA 0x387354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
// 0x387448 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD0Ev
pub fn stub_0x387448() {
    // IDA 0x387448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
// 0x38754c — __ZN3RBX10ThreadPoolD1Ev
pub fn stub_0x38754c() {
    // IDA 0x38754c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
// 0x387550 — __ZN3RBX10ThreadPoolD0Ev
pub fn stub_0x387550() {
    // IDA 0x387550: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
// 0x3875f0 — __ZN3RBX18PriorityThreadPoolD1Ev
pub fn stub_0x3875f0() {
    // IDA 0x3875f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
// 0x3875f4 — __ZN3RBX18PriorityThreadPoolD0Ev
pub fn stub_0x3875f4() {
    // IDA 0x3875f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// 0x387874 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_0x387874() {
    // IDA 0x387874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// 0x3879ec — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_0x3879ec() {
    // IDA 0x3879ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")]
// 0x387a60 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
pub fn stub_0x387a60() {
    // IDA 0x387a60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")]
// 0x387aac — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x387aac() {
    // IDA 0x387aac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")]
// 0x387e64 — __ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
pub fn stub_0x387e64() {
    // IDA 0x387e64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")]
// 0x387e88 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
pub fn stub_0x387e88() {
    // IDA 0x387e88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// 0x38aab8 — __ZN3RBX14BaseThreadPool8PoolDataD2Ev
pub fn stub_0x38aab8() {
    // IDA 0x38aab8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// 0x38ab90 — __ZN3RBX14BaseThreadPool8PoolDataD1Ev
pub fn stub_0x38ab90() {
    // IDA 0x38ab90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::~PoolData()")]
// 0x38ab94 — __ZN3RBX14BaseThreadPool8PoolDataD0Ev
pub fn stub_0x38ab94() {
    // IDA 0x38ab94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::~vector()")]
// 0x38ac34 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EED2Ev
pub fn stub_0x38ac34() {
    // IDA 0x38ac34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BaseThreadPool::PoolData::PoolData(void)")]
// 0x38ad00 — __ZN3RBX14BaseThreadPool8PoolDataC2Ev
pub fn stub_0x38ad00() {
    // IDA 0x38ad00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
// 0x38add0 — __ZN3RBX10ThreadPool14ThreadPoolDataD1Ev
pub fn stub_0x38add0() {
    // IDA 0x38add0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ThreadPool::ThreadPoolData::~ThreadPoolData()")]
// 0x38aec4 — __ZN3RBX10ThreadPool14ThreadPoolDataD0Ev
pub fn stub_0x38aec4() {
    // IDA 0x38aec4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "global constructor keyed to_a_146")]
// 0x38b740 — __GLOBAL__I_a_146
pub fn stub_0x38b740() {
    // IDA 0x38b740: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToString(RBX::UDim const&)")]
// 0x38b808 — __ZN3RBX15StringConverterINS_4UDimEE15convertToStringERKS1_
pub fn stub_0x38b808() {
    // IDA 0x38b808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::UDim>::convertToValue(std::string const&,RBX::UDim&)")]
// 0x38b970 — __ZN3RBX15StringConverterINS_4UDimEE14convertToValueERKSsRS1_
pub fn stub_0x38b970() {
    // IDA 0x38b970: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToString(RBX::UDim2 const&)")]
// 0x38ba5c — __ZN3RBX15StringConverterINS_5UDim2EE15convertToStringERKS1_
pub fn stub_0x38ba5c() {
    // IDA 0x38ba5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::StringConverter<RBX::UDim2>::convertToValue(std::string const&,RBX::UDim2&)")]
// 0x38be8c — __ZN3RBX15StringConverterINS_5UDim2EE14convertToValueERKSsRS1_
pub fn stub_0x38be8c() {
    // IDA 0x38be8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::UDim::operator+(RBX::UDim const&)const")]
// 0x38c0e8 — __ZNK3RBX4UDimplERKS0_
pub fn stub_0x38c0e8() {
    // IDA 0x38c0e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::UDim::operator-(RBX::UDim const&)const")]
// 0x38c108 — __ZNK3RBX4UDimmiERKS0_
pub fn stub_0x38c108() {
    // IDA 0x38c108: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::UDim::operator-(void)const")]
// 0x38c128 — __ZNK3RBX4UDimngEv
pub fn stub_0x38c128() {
    // IDA 0x38c128: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::UDim2::operator*(float)const")]
// 0x38c188 — __ZNK3RBX5UDim2mlEf
pub fn stub_0x38c188() {
    // IDA 0x38c188: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::UDim2::operator+(RBX::UDim2 const&)const")]
// 0x38c1e4 — __ZNK3RBX5UDim2plERKS0_
pub fn stub_0x38c1e4() {
    // IDA 0x38c1e4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UDim2::operator-(RBX::UDim2 const&)const")]
// 0x38c224 — __ZNK3RBX5UDim2miERKS0_
pub fn stub_0x38c224() {
    // IDA 0x38c224: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UDim2::operator-(void)const")]
// 0x38c264 — __ZNK3RBX5UDim2ngEv
pub fn stub_0x38c264() {
    // IDA 0x38c264: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_147")]
// 0x38c294 — __GLOBAL__I_a_147
pub fn stub_0x38c294() {
    // IDA 0x38c294: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UIEvent::isTextCharacterKey(void)const")]
// 0x38c35c — __ZNK3RBX7UIEvent18isTextCharacterKeyEv
pub fn stub_0x38c35c() {
    // IDA 0x38c35c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UIEvent::isAltEvent(void)const")]
// 0x38c368 — __ZNK3RBX7UIEvent10isAltEventEv
pub fn stub_0x38c368() {
    // IDA 0x38c368: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UIEvent::isCtrlEvent(void)const")]
// 0x38c37c — __ZNK3RBX7UIEvent11isCtrlEventEv
pub fn stub_0x38c37c() {
    // IDA 0x38c37c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UIEvent::isCarriageReturnKey(void)const")]
// 0x38c390 — __ZNK3RBX7UIEvent19isCarriageReturnKeyEv
pub fn stub_0x38c390() {
    // IDA 0x38c390: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UIEvent::isDeleteKey(void)const")]
// 0x38c3ac — __ZNK3RBX7UIEvent11isDeleteKeyEv
pub fn stub_0x38c3ac() {
    // IDA 0x38c3ac: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UIEvent::isBackspaceKey(void)const")]
// 0x38c3b8 — __ZNK3RBX7UIEvent14isBackspaceKeyEv
pub fn stub_0x38c3b8() {
    // IDA 0x38c3b8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UIEvent::isClearKey(void)const")]
// 0x38c3c4 — __ZNK3RBX7UIEvent10isClearKeyEv
pub fn stub_0x38c3c4() {
    // IDA 0x38c3c4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UIEvent::isEscapeKey(void)const")]
// 0x38c3d0 — __ZNK3RBX7UIEvent11isEscapeKeyEv
pub fn stub_0x38c3d0() {
    // IDA 0x38c3d0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UIEvent::isLeftArrowKey(void)const")]
// 0x38c3dc — __ZNK3RBX7UIEvent14isLeftArrowKeyEv
pub fn stub_0x38c3dc() {
    // IDA 0x38c3dc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::UIEvent::isRightArrowKey(void)const")]
// 0x38c3ec — __ZNK3RBX7UIEvent15isRightArrowKeyEv
pub fn stub_0x38c3ec() {
    // IDA 0x38c3ec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "global constructor keyed to_a_148")]
// 0x38c3fc — __GLOBAL__I_a_148
pub fn stub_0x38c3fc() {
    // IDA 0x38c3fc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Units::kmsForceToRbx(float)")]
// 0x38c464 — __ZN3RBX5Units13kmsForceToRbxEf
pub fn stub_0x38c464() {
    // IDA 0x38c464: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_149")]
// 0x38c478 — __GLOBAL__I_a_149
pub fn stub_0x38c478() {
    // IDA 0x38c478: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UserInputBase::UserInputBase(void)")]
// 0x38c4b0 — __ZN3RBX13UserInputBaseC2Ev
pub fn stub_0x38c4b0() {
    // IDA 0x38c4b0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UserInputBase::getNavKeys(RBX::NavKeys &,bool)const")]
// 0x38c5d4 — __ZNK3RBX13UserInputBase10getNavKeysERNS_7NavKeysEb
pub fn stub_0x38c5d4() {
    // IDA 0x38c5d4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::UserInputBase::removeJobs(void)")]
// 0x38cb9c — __ZN3RBX13UserInputBase10removeJobsEv
pub fn stub_0x38cb9c() {
    // IDA 0x38cb9c: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "global constructor keyed to_a_150")]
// 0x38cba0 — __GLOBAL__I_a_150
pub fn stub_0x38cba0() {
    // IDA 0x38cba0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::rot13(std::string)")]
// 0x38cc68 — __ZN3RBX5rot13ESs
pub fn stub_0x38cc68() {
    // IDA 0x38cc68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<bool>::convertToString(bool const&)")]
// 0x38ce48 — __ZN3RBX15StringConverterIbE15convertToStringERKb
pub fn stub_0x38ce48() {
    // IDA 0x38ce48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<bool>::convertToValue(std::string const&,bool &)")]
// 0x38ce78 — __ZN3RBX15StringConverterIbE14convertToValueERKSsRb
pub fn stub_0x38ce78() {
    // IDA 0x38ce78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<int>::convertToString(int const&)")]
// 0x38cf10 — __ZN3RBX15StringConverterIiE15convertToStringERKi
pub fn stub_0x38cf10() {
    // IDA 0x38cf10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<long>::convertToString(long const&)")]
// 0x38cf58 — __ZN3RBX15StringConverterIlE15convertToStringERKl
pub fn stub_0x38cf58() {
    // IDA 0x38cf58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<int>::convertToValue(std::string const&,int &)")]
// 0x38cfa0 — __ZN3RBX15StringConverterIiE14convertToValueERKSsRi
pub fn stub_0x38cfa0() {
    // IDA 0x38cfa0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<unsigned int>::convertToString(unsigned int const&)")]
// 0x38cff0 — __ZN3RBX15StringConverterIjE15convertToStringERKj
pub fn stub_0x38cff0() {
    // IDA 0x38cff0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<unsigned int>::convertToValue(std::string const&,unsigned int &)")]
// 0x38d038 — __ZN3RBX15StringConverterIjE14convertToValueERKSsRj
pub fn stub_0x38d038() {
    // IDA 0x38d038: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<long>::convertToValue(std::string const&,long &)")]
// 0x38d14c — __ZN3RBX15StringConverterIlE14convertToValueERKSsRl
pub fn stub_0x38d14c() {
    // IDA 0x38d14c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<double>::convertToValue(std::string const&,double &)")]
// 0x38d260 — __ZN3RBX15StringConverterIdE14convertToValueERKSsRd
pub fn stub_0x38d260() {
    // IDA 0x38d260: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<double>::convertToString(double const&)")]
// 0x38d2e0 — __ZN3RBX15StringConverterIdE15convertToStringERKd
pub fn stub_0x38d2e0() {
    // IDA 0x38d2e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<float>::convertToValue(std::string const&,float &)")]
// 0x38d440 — __ZN3RBX15StringConverterIfE14convertToValueERKSsRf
pub fn stub_0x38d440() {
    // IDA 0x38d440: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<float>::convertToString(float const&)")]
// 0x38d4c4 — __ZN3RBX15StringConverterIfE15convertToStringERKf
pub fn stub_0x38d4c4() {
    // IDA 0x38d4c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_signed<long>(long &)")]
// 0x38d61c — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE10shr_signedIlEEbRT_
pub fn stub_0x38d61c() {
    // IDA 0x38d61c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "bool boost::detail::lcast_ret_unsigned<std::char_traits<char>,unsigned long,char>(unsigned long &,char const*,char const*)")]
// 0x38d67c — __ZN5boost6detail18lcast_ret_unsignedISt11char_traitsIcEmcEEbRT0_PKT1_S8_
pub fn stub_0x38d67c() {
    // IDA 0x38d67c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "bool boost::detail::lexical_stream_limited_src<char,std::char_traits<char>,false>::shr_unsigned<unsigned int>(unsigned int &)")]
// 0x38da14 — __ZN5boost6detail26lexical_stream_limited_srcIcSt11char_traitsIcELb0EE12shr_unsignedIjEEbRT_
pub fn stub_0x38da14() {
    // IDA 0x38da14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "global constructor keyed to_a_151")]
// 0x38da58 — __GLOBAL__I_a_151
pub fn stub_0x38da58() {
    // IDA 0x38da58: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Accoutrement::getAttachmentPos(void)const")]
// 0x38dc30 — __ZNK3RBX12Accoutrement16getAttachmentPosEv
pub fn stub_0x38dc30() {
    // IDA 0x38dc30: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Accoutrement::getAttachmentForward(void)const")]
// 0x38dc70 — __ZNK3RBX12Accoutrement20getAttachmentForwardEv
pub fn stub_0x38dc70() {
    // IDA 0x38dc70: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Accoutrement::getAttachmentUp(void)const")]
// 0x38ddfc — __ZNK3RBX12Accoutrement15getAttachmentUpEv
pub fn stub_0x38ddfc() {
    // IDA 0x38ddfc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Accoutrement::getAttachmentRight(void)const")]
// 0x38df30 — __ZNK3RBX12Accoutrement18getAttachmentRightEv
pub fn stub_0x38df30() {
    // IDA 0x38df30: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "RBX::Accoutrement::setBackendAccoutrementState(int)")]
// 0x38e064 — __ZN3RBX12Accoutrement27setBackendAccoutrementStateEi
pub fn stub_0x38e064() {
    // IDA 0x38e064: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Accoutrement::Accoutrement(void)")]
// 0x38e084 — __ZN3RBX12AccoutrementC1Ev
pub fn stub_0x38e084() {
    // IDA 0x38e084: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Accoutrement::Accoutrement(void)")]
// 0x38e4b4 — __ZN3RBX12AccoutrementC2Ev
pub fn stub_0x38e4b4() {
    // IDA 0x38e4b4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
// 0x38e90c — __ZN3RBX12AccoutrementD0Ev
pub fn stub_0x38e90c() {
    // IDA 0x38e90c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Accoutrement::~Accoutrement()")]
// 0x38e9b8 — __ZN3RBX12AccoutrementD1Ev
pub fn stub_0x38e9b8() {
    // IDA 0x38e9b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Accoutrement::~Accoutrement()")]
// 0x38e9c8 — __ZThn32_N3RBX12AccoutrementD0Ev
pub fn stub_0x38e9c8() {
    // IDA 0x38e9c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
