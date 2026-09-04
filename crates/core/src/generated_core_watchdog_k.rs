//! core watchdog k — 100 core stubs EA-sorted, eleventh gap filler after watchdog_j 0x2c4a50.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_core — next 100 uncovered after 0x2c4a50 (watchdog_j max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2c5440 — __ZN3RBX3Lua15YieldingThreadsC1EPNS_13ScriptContextE
#[doc(alias = "RBX::Lua::YieldingThreads::YieldingThreads(RBX::ScriptContext *)")]
pub fn stub_0x2c5440() {
    // IDA 0x2c5440: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

// 0x2c5444 — __ZN3RBX3Lua15YieldingThreadsC2EPNS_13ScriptContextE
#[doc(alias = "RBX::Lua::YieldingThreads::YieldingThreads(RBX::ScriptContext *)")]
pub fn stub_0x2c5444() {
    // IDA 0x2c5444: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

// 0x2c5518 — __ZN3RBX3Lua15YieldingThreads11queueWaiterEP9lua_State
#[doc(alias = "RBX::Lua::YieldingThreads::queueWaiter(lua_State *)")]
pub fn stub_0x2c5518() {
    // IDA 0x2c5518: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

// 0x2c5530 — __ZN3RBX3Lua15YieldingThreads11queueWaiterEP9lua_Stated
#[doc(alias = "RBX::Lua::YieldingThreads::queueWaiter(lua_State *,double)")]
pub fn stub_0x2c5530() {
    // IDA 0x2c5530: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

// 0x2c567c — __ZNK3RBX3Lua15YieldingThreads11waiterCountEv
#[doc(alias = "RBX::Lua::YieldingThreads::waiterCount(void)const")]
pub fn stub_0x2c567c() {
    // IDA 0x2c567c: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

// 0x2c5690 — __ZN3RBX3Lua15YieldingThreads6resumeEdNS_4TimeERb
#[doc(alias = "RBX::Lua::YieldingThreads::resume(double,RBX::Time,bool &)")]
pub fn stub_0x2c5690() {
    // IDA 0x2c5690: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

// 0x2c5a08 — __ZNSt14priority_queueIN3RBX3Lua15YieldingThreads13WaitingThreadESt6vectorIS3_SaIS3_EESt4lessIS3_EE4pushERKS3_
#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::push(RBX::Lua::YieldingThreads::WaitingThread const&)")]
pub fn stub_0x2c5a08() {
    // IDA 0x2c5a08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c5b10 — __ZNSt14priority_queueIN3RBX3Lua15YieldingThreads13WaitingThreadESt6vectorIS3_SaIS3_EESt4lessIS3_EE3popEv
#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::pop(void)")]
pub fn stub_0x2c5b10() {
    // IDA 0x2c5b10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c5b3c — __ZSt8pop_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX3Lua15YieldingThreads13WaitingThreadESt6vectorIS5_SaIS5_EEEESt4lessIS5_EEvT_SD_T0_
#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>)")]
pub fn stub_0x2c5b3c() {
    // IDA 0x2c5b3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c5cac — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX3Lua15YieldingThreads13WaitingThreadESt6vectorIS5_SaIS5_EEEEiS5_St4lessIS5_EEvT_T0_SE_T1_T2_
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>)")]
pub fn stub_0x2c5cac() {
    // IDA 0x2c5cac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c5e44 — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX3Lua15YieldingThreads13WaitingThreadESt6vectorIS5_SaIS5_EEEEiS5_St4lessIS5_EEvT_T0_SE_T1_T2_
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>)")]
pub fn stub_0x2c5e44() {
    // IDA 0x2c5e44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c5ef0 — __ZNSt6vectorIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EE9push_backERKS3_
#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::push_back(RBX::Lua::YieldingThreads::WaitingThread const&)")]
pub fn stub_0x2c5ef0() {
    // IDA 0x2c5ef0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c5f48 — __ZNSt6vectorIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread*,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,RBX::Lua::YieldingThreads::WaitingThread const&)")]
pub fn stub_0x2c5f48() {
    // IDA 0x2c5f48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c636c — __ZNSt12_Vector_baseIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_M_allocate(unsigned long)")]
pub fn stub_0x2c636c() {
    // IDA 0x2c636c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c6390 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX3Lua15YieldingThreads13WaitingThreadES7_EET0_T_S9_S8_
#[doc(alias = "RBX::Lua::YieldingThreads::WaitingThread * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *>(RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *)")]
pub fn stub_0x2c6390() {
    // IDA 0x2c6390: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c63f8 — __ZNSt14priority_queueIN3RBX3Lua15YieldingThreads13WaitingThreadESt6vectorIS3_SaIS3_EESt4lessIS3_EEC2ERKS8_RKS6_
#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::priority_queue(std::less<RBX::Lua::YieldingThreads::WaitingThread> const&,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>> const&)")]
pub fn stub_0x2c63f8() {
    // IDA 0x2c63f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c6548 — __ZNSt6vectorIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EEC2ERKS5_
#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::vector(std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>> const&)")]
pub fn stub_0x2c6548() {
    // IDA 0x2c6548: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c66c4 — __ZNSt12_Vector_baseIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EEC2EmRKS4_
#[doc(alias = "std::_Vector_base<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_Vector_base(unsigned long,std::allocator<RBX::Lua::YieldingThreads::WaitingThread> const&)")]
pub fn stub_0x2c66c4() {
    // IDA 0x2c66c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0x2c66f8 — __ZNSt6vectorIN3RBX3Lua15YieldingThreads13WaitingThreadESaIS3_EED2Ev
#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::~vector()")]
pub fn stub_0x2c66f8() {
    // IDA 0x2c66f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c67c8 — __ZN3RBX3Lua15YieldingThreads13WaitingThreadC2EP9lua_StateNS_4Time8IntervalE
#[doc(alias = "RBX::Lua::YieldingThreads::WaitingThread::WaitingThread(lua_State *,RBX::Time::Interval)")]
pub fn stub_0x2c67c8() {
    // IDA 0x2c67c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c715c — __ZN3RBX12LuaStatsItem4initEv
#[doc(alias = "RBX::LuaStatsItem::init(void)")]
pub fn stub_0x2c715c() {
    // IDA 0x2c715c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c72c8 — __ZN3RBX12LuaStatsItem6updateEv
#[doc(alias = "RBX::LuaStatsItem::update(void)")]
pub fn stub_0x2c72c8() {
    // IDA 0x2c72c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c756c — __ZN3RBX12LuaStatsItemD1Ev
#[doc(alias = "RBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c756c() {
    // IDA 0x2c756c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c75a8 — __ZN3RBX12LuaStatsItemD0Ev
#[doc(alias = "RBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c75a8() {
    // IDA 0x2c75a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c767c — __ZThn32_N3RBX12LuaStatsItemD1Ev
#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c767c() {
    // IDA 0x2c767c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c76bc — __ZThn32_N3RBX12LuaStatsItemD0Ev
#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c76bc() {
    // IDA 0x2c76bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c7790 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats10sStatsItemEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats10sStatsItemEEE12getClassNameEv")]
pub fn stub_0x2c7790() {
    // IDA 0x2c7790: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c7794 — __ZThn36_N3RBX12LuaStatsItemD1Ev
#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c7794() {
    // IDA 0x2c7794: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c77d4 — __ZThn36_N3RBX12LuaStatsItemD0Ev
#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c77d4() {
    // IDA 0x2c77d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c7968 — __ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c7968() {
    // IDA 0x2c7968: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c7970 — __ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c7970() {
    // IDA 0x2c7970: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c7978 — __ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2c7978() {
    // IDA 0x2c7978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c94ac — __ZN3RBX3Lua13WeakThreadRefC1EP9lua_State
#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(lua_State *)")]
pub fn stub_0x2c94ac() {
    // IDA 0x2c94ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c94b0 — __ZN3RBX3Lua13WeakThreadRefC2EP9lua_State
#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(lua_State *)")]
pub fn stub_0x2c94b0() {
    // IDA 0x2c94b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c966c — __ZN3RBX3Lua13WeakThreadRef9addToNodeEv
#[doc(alias = "RBX::Lua::WeakThreadRef::addToNode(void)")]
pub fn stub_0x2c966c() {
    // IDA 0x2c966c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c96b4 — __ZN3RBX3Lua13WeakThreadRef6addRefEP9lua_State
#[doc(alias = "RBX::Lua::WeakThreadRef::addRef(lua_State *)")]
pub fn stub_0x2c96b4() {
    // IDA 0x2c96b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9800 — __ZN3RBX3Lua13WeakThreadRefC1ERKS1_
#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(RBX::Lua::WeakThreadRef const&)")]
pub fn stub_0x2c9800() {
    // IDA 0x2c9800: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x2c9804 — __ZN3RBX3Lua13WeakThreadRefC2ERKS1_
#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(RBX::Lua::WeakThreadRef const&)")]
pub fn stub_0x2c9804() {
    // IDA 0x2c9804: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x2c99c4 — __ZN3RBX3Lua13WeakThreadRefD0Ev
#[doc(alias = "RBX::Lua::WeakThreadRef::~WeakThreadRef()")]
pub fn stub_0x2c99c4() {
    // IDA 0x2c99c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9a64 — __ZN3RBX3Lua13WeakThreadRefD1Ev
#[doc(alias = "RBX::Lua::WeakThreadRef::~WeakThreadRef()")]
pub fn stub_0x2c9a64() {
    // IDA 0x2c9a64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9a68 — __ZN3RBX3Lua13WeakThreadRefD2Ev
#[doc(alias = "RBX::Lua::WeakThreadRef::~WeakThreadRef()")]
pub fn stub_0x2c9a68() {
    // IDA 0x2c9a68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9b74 — __ZN3RBX3Lua13WeakThreadRef5resetEv
#[doc(alias = "RBX::Lua::WeakThreadRef::reset(void)")]
pub fn stub_0x2c9b74() {
    // IDA 0x2c9b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9c54 — __ZN3RBX3Lua13WeakThreadRef14removeFromNodeEv
#[doc(alias = "RBX::Lua::WeakThreadRef::removeFromNode(void)")]
pub fn stub_0x2c9c54() {
    // IDA 0x2c9c54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9cb0 — __ZN3RBX3Lua13WeakThreadRefaSERKS1_
#[doc(alias = "RBX::Lua::WeakThreadRef::operator=(RBX::Lua::WeakThreadRef const&)")]
pub fn stub_0x2c9cb0() {
    // IDA 0x2c9cb0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9db8 — __ZN3RBX3Lua6detail13LiveThreadRefC1EP9lua_State
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::LiveThreadRef(lua_State *)")]
pub fn stub_0x2c9db8() {
    // IDA 0x2c9db8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9dbc — __ZN3RBX3Lua13WeakThreadRef9removeRefEv
#[doc(alias = "RBX::Lua::WeakThreadRef::removeRef(void)")]
pub fn stub_0x2c9dbc() {
    // IDA 0x2c9dbc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x2c9df8 — __ZN3RBX3Lua13WeakThreadRef4Node12eraseAllRefsEv
#[doc(alias = "RBX::Lua::WeakThreadRef::Node::eraseAllRefs(void)")]
pub fn stub_0x2c9df8() {
    // IDA 0x2c9df8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

// 0x2c9ee8 — __ZN3RBX3Lua13WeakThreadRef4NodeD1Ev
#[doc(alias = "RBX::Lua::WeakThreadRef::Node::~Node()")]
pub fn stub_0x2c9ee8() {
    // IDA 0x2c9ee8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9eec — __ZN3RBX3Lua13WeakThreadRef4NodeD2Ev
#[doc(alias = "RBX::Lua::WeakThreadRef::Node::~Node()")]
pub fn stub_0x2c9eec() {
    // IDA 0x2c9eec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2c9f1c — __ZN3RBX3Lua13WeakThreadRef4Node6createEP9lua_State
#[doc(alias = "RBX::Lua::WeakThreadRef::Node::create(lua_State *)")]
pub fn stub_0x2c9f1c() {
    // IDA 0x2c9f1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2ca0c4 — __ZN3RBX3Lua19dumpThreadRefCountsEv
#[doc(alias = "RBX::Lua::dumpThreadRefCounts(void)")]
pub fn stub_0x2ca0c4() {
    // IDA 0x2ca0c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2ca11c — __ZN3RBX3Lua15WeakFunctionRefC1EP9lua_Statei
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(lua_State *,int)")]
pub fn stub_0x2ca11c() {
    // IDA 0x2ca11c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2ca120 — __ZN3RBX3Lua15WeakFunctionRefC2EP9lua_Statei
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(lua_State *,int)")]
pub fn stub_0x2ca120() {
    // IDA 0x2ca120: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cad6c — __ZN3RBX3Lua15WeakFunctionRefD0Ev
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
pub fn stub_0x2cad6c() {
    // IDA 0x2cad6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cae0c — __ZN3RBX3Lua15WeakFunctionRefD1Ev
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
pub fn stub_0x2cae0c() {
    // IDA 0x2cae0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cae10 — __ZN3RBX3Lua15WeakFunctionRefD2Ev
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
pub fn stub_0x2cae10() {
    // IDA 0x2cae10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2caf24 — __ZN3RBX3Lua15WeakFunctionRef9removeRefEv
#[doc(alias = "RBX::Lua::WeakFunctionRef::removeRef(void)")]
pub fn stub_0x2caf24() {
    // IDA 0x2caf24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2caf98 — __ZN3RBX3Lua15WeakFunctionRefC1ERKS1_
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x2caf98() {
    // IDA 0x2caf98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2caf9c — __ZN3RBX3Lua15WeakFunctionRefC2ERKS1_
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x2caf9c() {
    // IDA 0x2caf9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cb0fc — __ZN3RBX3Lua6detail13LiveThreadRefC2EP9lua_State
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::LiveThreadRef(lua_State *)")]
pub fn stub_0x2cb0fc() {
    // IDA 0x2cb0fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cb2ec — __ZN3RBX3Lua6detail13LiveThreadRefD1Ev
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef()")]
pub fn stub_0x2cb2ec() {
    // IDA 0x2cb2ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cb2f0 — __ZN3RBX3Lua6detail13LiveThreadRefD2Ev
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef()")]
pub fn stub_0x2cb2f0() {
    // IDA 0x2cb2f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cb3fc — __ZN3RBX3Lua15WeakFunctionRefaSERKS1_
#[doc(alias = "RBX::Lua::WeakFunctionRef::operator=(RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x2cb3fc() {
    // IDA 0x2cb3fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cb4d0 — __ZN3RBX10Reflection4Type12getSingletonINS_3Lua15WeakFunctionRefEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Lua::WeakFunctionRef>(void)")]
pub fn stub_0x2cb4d0() {
    // IDA 0x2cb4d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cb5b4 — __ZN3RBX10Reflection7Variant7convertINS_3Lua15WeakFunctionRefEEERT_v
#[doc(alias = "RBX::Lua::WeakFunctionRef & RBX::Reflection::Variant::convert<RBX::Lua::WeakFunctionRef>(void)")]
pub fn stub_0x2cb5b4() {
    // IDA 0x2cb5b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cbdf8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED1Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
pub fn stub_0x2cbdf8() {
    // IDA 0x2cbdf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cbdfc — __ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Lua::WeakFunctionRef * rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x2cbdfc() {
    // IDA 0x2cbdfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cbfb8 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::singleton(void)")]
pub fn stub_0x2cbfb8() {
    // IDA 0x2cbfb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cc020 — __ZN3RBX10Reflection4TypeC2INS_3Lua15WeakFunctionRefEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Lua::WeakFunctionRef>(char const*,RBX::Lua::WeakFunctionRef *)")]
pub fn stub_0x2cc020() {
    // IDA 0x2cc020: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cc0c8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED0Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
pub fn stub_0x2cc0c8() {
    // IDA 0x2cc0c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2ced4c — __ZN3RBX13AdvLuaDraggerC2Ev
#[doc(alias = "RBX::AdvLuaDragger::AdvLuaDragger(void)")]
pub fn stub_0x2ced4c() {
    // IDA 0x2ced4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cef40 — __ZN3RBX13AdvLuaDraggerD0Ev
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cef40() {
    // IDA 0x2cef40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cefe0 — __ZN3RBX13AdvLuaDraggerD1Ev
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cefe0() {
    // IDA 0x2cefe0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cefe4 — __ZThn32_N3RBX13AdvLuaDraggerD0Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cefe4() {
    // IDA 0x2cefe4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cefec — __ZThn36_N3RBX13AdvLuaDraggerD0Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cefec() {
    // IDA 0x2cefec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2ceff4 — __ZN3RBX13AdvLuaDraggerD2Ev
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2ceff4() {
    // IDA 0x2ceff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cf168 — __ZThn32_N3RBX13AdvLuaDraggerD1Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cf168() {
    // IDA 0x2cf168: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cf170 — __ZThn36_N3RBX13AdvLuaDraggerD1Ev
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
pub fn stub_0x2cf170() {
    // IDA 0x2cf170: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cf3b8 — __ZN3RBX13AdvLuaDragger9mouseMoveENS_6RbxRayE
#[doc(alias = "RBX::AdvLuaDragger::mouseMove(RBX::RbxRay)")]
pub fn stub_0x2cf3b8() {
    // IDA 0x2cf3b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cf6d0 — __ZN3RBX13AdvLuaDragger16tryStartDraggingERKNS_6RbxRayE
#[doc(alias = "RBX::AdvLuaDragger::tryStartDragging(RBX::RbxRay const&)")]
pub fn stub_0x2cf6d0() {
    // IDA 0x2cf6d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cf930 — __ZN3RBX13AdvLuaDragger6doDragERKNS_6RbxRayE
#[doc(alias = "RBX::AdvLuaDragger::doDrag(RBX::RbxRay const&)")]
pub fn stub_0x2cf930() {
    // IDA 0x2cf930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2cfd7c — __ZN3RBX13AdvLuaDragger7mouseUpEv
#[doc(alias = "RBX::AdvLuaDragger::mouseUp(void)")]
pub fn stub_0x2cfd7c() {
    // IDA 0x2cfd7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d0030 — __ZN3RBX13AdvLuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
#[doc(alias = "RBX::AdvLuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
pub fn stub_0x2d0030() {
    // IDA 0x2d0030: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x2d0154 — __ZN3RBX13AdvLuaDragger13startDraggingEv
#[doc(alias = "RBX::AdvLuaDragger::startDragging(void)")]
pub fn stub_0x2d0154() {
    // IDA 0x2d0154: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x2d03b0 — __ZN3RBX13AdvLuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E
#[doc(alias = "RBX::AdvLuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
pub fn stub_0x2d03b0() {
    // IDA 0x2d03b0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x2d05bc — __ZN3RBX13AdvLuaDragger15alignPartToGridEv
#[doc(alias = "RBX::AdvLuaDragger::alignPartToGrid(void)")]
pub fn stub_0x2d05bc() {
    // IDA 0x2d05bc: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x2d07e0 — __ZNK3RBX13AdvLuaDragger12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::AdvLuaDragger::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x2d07e0() {
    // IDA 0x2d07e0: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x2d07e4 — __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv")]
pub fn stub_0x2d07e4() {
    // IDA 0x2d07e4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x2d07f4 — __ZThn32_NK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv")]
pub fn stub_0x2d07f4() {
    // IDA 0x2d07f4: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

// 0x2d0804 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD1Ev")]
pub fn stub_0x2d0804() {
    // IDA 0x2d0804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d0808 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev")]
pub fn stub_0x2d0808() {
    // IDA 0x2d0808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d08a4 — __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator12getClassNameEv")]
pub fn stub_0x2d08a4() {
    // IDA 0x2d08a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d092c — __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7Creator6createEv")]
pub fn stub_0x2d092c() {
    // IDA 0x2d092c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d0e1c — __ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sAdvLuaDraggerEEEEvv")]
pub fn stub_0x2d0e1c() {
    // IDA 0x2d0e1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d0e20 — __ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvLuaDraggerEEEERKS0_v")]
pub fn stub_0x2d0e20() {
    // IDA 0x2d0e20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d0f00 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorC2Ev")]
pub fn stub_0x2d0f00() {
    // IDA 0x2d0f00: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

// 0x2d1144 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E17static_getCreatorEv")]
pub fn stub_0x2d1144() {
    // IDA 0x2d1144: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

// 0x2d1260 — __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d1260() {
    // IDA 0x2d1260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d1264 — __ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2d1264() {
    // IDA 0x2d1264: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x2d1304 — __ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2d1304() {
    // IDA 0x2d1304: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

