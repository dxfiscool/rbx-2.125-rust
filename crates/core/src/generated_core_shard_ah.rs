//! core shard AH — 150 core stubs EA-sorted, earliest gap (lowest uncovered) after prior shards.
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 150 uncovered (lowest EA first).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::AsyncHttpQueue::setThreadPool(int)")]
// 0x2fad24 — __ZN3RBX14AsyncHttpQueue13setThreadPoolEi
pub fn stub_0x2fad24() -> ! {
    todo!("0x2fad24 RBX::AsyncHttpQueue::setThreadPool(int)")
}

#[doc(alias = "RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")]
// 0x2fae00 — __ZN3RBX14AsyncHttpQueue14resetStatsItemEPNS_15ServiceProviderE
pub fn stub_0x2fae00() -> ! {
    todo!("0x2fae00 RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")
}

#[doc(alias = "RBX::AsyncHttpQueue::getRequestQueueSize(void)const")]
// 0x2faf2c — __ZNK3RBX14AsyncHttpQueue19getRequestQueueSizeEv
pub fn stub_0x2faf2c() -> ! {
    todo!("0x2faf2c RBX::AsyncHttpQueue::getRequestQueueSize(void)const")
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
// 0x2faf68 — __ZN3RBX14AsyncHttpQueueD0Ev
pub fn stub_0x2faf68() -> ! {
    todo!("0x2faf68 RBX::AsyncHttpQueue::~AsyncHttpQueue()")
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
// 0x2fb008 — __ZN3RBX14AsyncHttpQueueD1Ev
pub fn stub_0x2fb008() -> ! {
    todo!("0x2fb008 RBX::AsyncHttpQueue::~AsyncHttpQueue()")
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
// 0x2fb00c — __ZN3RBX14AsyncHttpQueueD2Ev
pub fn stub_0x2fb00c() -> ! {
    todo!("0x2fb00c RBX::AsyncHttpQueue::~AsyncHttpQueue()")
}

#[doc(alias = "RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")]
// 0x2fb2ac — __ZN3RBX14AsyncHttpQueue11onHeartbeatERKNS_9HeartbeatE
pub fn stub_0x2fb2ac() -> ! {
    todo!("0x2fb2ac RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")
}

#[doc(alias = "RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")]
// 0x2fca04 — __ZN3RBX14AsyncHttpQueue19isRequestQueueEmptyEv
pub fn stub_0x2fca04() -> ! {
    todo!("0x2fca04 RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")
}

#[doc(alias = "RBX::checkContentUrl(std::string)")]
// 0x2fca3c — __ZN3RBXL15checkContentUrlESs
pub fn stub_0x2fca3c() -> ! {
    todo!("0x2fca3c RBX::checkContentUrl(std::string)")
}

#[doc(alias = "RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")]
// 0x2fd150 — __ZN3RBX14AsyncHttpQueue9FailedUrlC2EPKc
pub fn stub_0x2fd150() -> ! {
    todo!("0x2fd150 RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")
}

#[doc(alias = "RBX::AsyncHttpQueue::isUrlBad(std::string const&)")]
// 0x2fd220 — __ZN3RBX14AsyncHttpQueue8isUrlBadERKSs
pub fn stub_0x2fd220() -> ! {
    todo!("0x2fd220 RBX::AsyncHttpQueue::isUrlBad(std::string const&)")
}

#[doc(alias = "RBX::AsyncHttpQueue::syncRequest(std::string const&)")]
// 0x2fd910 — __ZN3RBX14AsyncHttpQueue11syncRequestERKSs
pub fn stub_0x2fd910() -> ! {
    todo!("0x2fd910 RBX::AsyncHttpQueue::syncRequest(std::string const&)")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
// 0x2fe654 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
pub fn stub_0x2fe654() -> ! {
    todo!("0x2fe654 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
// 0x2fea20 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
pub fn stub_0x2fea20() -> ! {
    todo!("0x2fea20 std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
// 0x2fea58 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
pub fn stub_0x2fea58() -> ! {
    todo!("0x2fea58 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
// 0x2feab0 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x2feab0() -> ! {
    todo!("0x2feab0 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
// 0x2fee5c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
pub fn stub_0x2fee5c() -> ! {
    todo!("0x2fee5c std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
// 0x2ff128 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
pub fn stub_0x2ff128() -> ! {
    todo!("0x2ff128 RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
// 0x2ff188 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_0x2ff188() -> ! {
    todo!("0x2ff188 std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
// 0x2ff2d4 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
pub fn stub_0x2ff2d4() -> ! {
    todo!("0x2ff2d4 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
// 0x2ff43c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
pub fn stub_0x2ff43c() -> ! {
    todo!("0x2ff43c std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
// 0x2ff674 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
pub fn stub_0x2ff674() -> ! {
    todo!("0x2ff674 std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")]
// 0x2ff758 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_
pub fn stub_0x2ff758() -> ! {
    todo!("0x2ff758 RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
// 0x2ff8c0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
pub fn stub_0x2ff8c0() -> ! {
    todo!("0x2ff8c0 RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
// 0x2ff91c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_
pub fn stub_0x2ff91c() -> ! {
    todo!("0x2ff91c RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
// 0x2ff978 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev
pub fn stub_0x2ff978() -> ! {
    todo!("0x2ff978 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
// 0x2ffa44 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_0x2ffa44() -> ! {
    todo!("0x2ffa44 std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
// 0x301b4c — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
pub fn stub_0x301b4c() -> ! {
    todo!("0x301b4c std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
// 0x301b80 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
pub fn stub_0x301b80() -> ! {
    todo!("0x301b80 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
// 0x301b98 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
pub fn stub_0x301b98() -> ! {
    todo!("0x301b98 std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
// 0x301f74 — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
pub fn stub_0x301f74() -> ! {
    todo!("0x301f74 std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
// 0x302028 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
pub fn stub_0x302028() -> ! {
    todo!("0x302028 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
// 0x302054 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
pub fn stub_0x302054() -> ! {
    todo!("0x302054 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
// 0x3021d4 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_0x3021d4() -> ! {
    todo!("0x3021d4 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")
}

#[doc(alias = "RBX::HttpQueueStatsItem::init(void)")]
// 0x3023dc — __ZN3RBX18HttpQueueStatsItem4initEv
pub fn stub_0x3023dc() -> ! {
    todo!("0x3023dc RBX::HttpQueueStatsItem::init(void)")
}

#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x30266c — __ZN3RBX18HttpQueueStatsItemD1Ev
pub fn stub_0x30266c() -> ! {
    todo!("0x30266c RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x3026a8 — __ZN3RBX18HttpQueueStatsItemD0Ev
pub fn stub_0x3026a8() -> ! {
    todo!("0x3026a8 RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

#[doc(alias = "RBX::HttpQueueStatsItem::update(void)")]
// 0x30277c — __ZN3RBX18HttpQueueStatsItem6updateEv
pub fn stub_0x30277c() -> ! {
    todo!("0x30277c RBX::HttpQueueStatsItem::update(void)")
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x3027d0 — __ZThn32_N3RBX18HttpQueueStatsItemD1Ev
pub fn stub_0x3027d0() -> ! {
    todo!("0x3027d0 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x302810 — __ZThn32_N3RBX18HttpQueueStatsItemD0Ev
pub fn stub_0x302810() -> ! {
    todo!("0x302810 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x3028e8 — __ZThn36_N3RBX18HttpQueueStatsItemD1Ev
pub fn stub_0x3028e8() -> ! {
    todo!("0x3028e8 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// 0x302928 — __ZThn36_N3RBX18HttpQueueStatsItemD0Ev
pub fn stub_0x302928() -> ! {
    todo!("0x302928 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
// 0x302cf8 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
pub fn stub_0x302cf8() -> ! {
    todo!("0x302cf8 std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")
}

#[doc(alias = "global constructor keyed to_a_106")]
// 0x302d20 — __GLOBAL__I_a_106
pub fn stub_0x302d20() -> ! {
    todo!("0x302d20 global constructor keyed to_a_106")
}

#[doc(alias = "RBX::Axes::Axes(int)")]
// 0x302eb8 — __ZN3RBX4AxesC1Ei
pub fn stub_0x302eb8() -> ! {
    todo!("0x302eb8 RBX::Axes::Axes(int)")
}

#[doc(alias = "RBX::Axes::normalIdToAxis(RBX::NormalId)")]
// 0x302ebc — __ZN3RBX4Axes14normalIdToAxisENS_8NormalIdE
pub fn stub_0x302ebc() -> ! {
    todo!("0x302ebc RBX::Axes::normalIdToAxis(RBX::NormalId)")
}

#[doc(alias = "RBX::Axes::getAxisByNormalId(RBX::NormalId)const")]
// 0x302ef0 — __ZNK3RBX4Axes17getAxisByNormalIdENS_8NormalIdE
pub fn stub_0x302ef0() -> ! {
    todo!("0x302ef0 RBX::Axes::getAxisByNormalId(RBX::NormalId)const")
}

#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")]
// 0x302f30 — __ZN3RBX15StringConverterINS_4AxesEE15convertToStringERKS1_
pub fn stub_0x302f30() -> ! {
    todo!("0x302f30 RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")
}

#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")]
// 0x303418 — __ZN3RBX15StringConverterINS_4AxesEE14convertToValueERKSsRS1_
pub fn stub_0x303418() -> ! {
    todo!("0x303418 RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")
}

#[doc(alias = "global constructor keyed to_a_107")]
// 0x304200 — __GLOBAL__I_a_107
pub fn stub_0x304200() -> ! {
    todo!("0x304200 global constructor keyed to_a_107")
}

#[doc(alias = "RBX::BrickColor::BrickMap::singleton(void)")]
// 0x3042c8 — __ZN3RBX10BrickColor8BrickMap9singletonEv
pub fn stub_0x3042c8() -> ! {
    todo!("0x3042c8 RBX::BrickColor::BrickMap::singleton(void)")
}

#[doc(alias = "RBX::BrickColor::colorPalette(void)")]
// 0x3043c4 — __ZN3RBX10BrickColor12colorPaletteEv
pub fn stub_0x3043c4() -> ! {
    todo!("0x3043c4 RBX::BrickColor::colorPalette(void)")
}

#[doc(alias = "RBX::BrickColor::getClosestPaletteIndex(void)const")]
// 0x3043dc — __ZNK3RBX10BrickColor22getClosestPaletteIndexEv
pub fn stub_0x3043dc() -> ! {
    todo!("0x3043dc RBX::BrickColor::getClosestPaletteIndex(void)const")
}

#[doc(alias = "RBX::BrickColor::parse(char const*)")]
// 0x3043fc — __ZN3RBX10BrickColor5parseEPKc
pub fn stub_0x3043fc() -> ! {
    todo!("0x3043fc RBX::BrickColor::parse(char const*)")
}

#[doc(alias = "RBX::BrickColor::random(void)")]
// 0x304468 — __ZN3RBX10BrickColor6randomEv
pub fn stub_0x304468() -> ! {
    todo!("0x304468 RBX::BrickColor::random(void)")
}

#[doc(alias = "RBX::BrickColor::BrickColor(int)")]
// 0x304568 — __ZN3RBX10BrickColorC1Ei
pub fn stub_0x304568() -> ! {
    todo!("0x304568 RBX::BrickColor::BrickColor(int)")
}

#[doc(alias = "RBX::BrickColor::BrickColor(int)")]
// 0x30456c — __ZN3RBX10BrickColorC2Ei
pub fn stub_0x30456c() -> ! {
    todo!("0x30456c RBX::BrickColor::BrickColor(int)")
}

#[doc(alias = "RBX::BrickColor::color4uint8(void)const")]
// 0x3045b0 — __ZNK3RBX10BrickColor11color4uint8Ev
pub fn stub_0x3045b0() -> ! {
    todo!("0x3045b0 RBX::BrickColor::color4uint8(void)const")
}

#[doc(alias = "RBX::BrickColor::color3uint8(void)const")]
// 0x304654 — __ZNK3RBX10BrickColor11color3uint8Ev
pub fn stub_0x304654() -> ! {
    todo!("0x304654 RBX::BrickColor::color3uint8(void)const")
}

#[doc(alias = "RBX::BrickColor::name(void)const")]
// 0x304674 — __ZNK3RBX10BrickColor4nameEv
pub fn stub_0x304674() -> ! {
    todo!("0x304674 RBX::BrickColor::name(void)const")
}

#[doc(alias = "RBX::BrickColor::color4(void)const")]
// 0x304710 — __ZNK3RBX10BrickColor6color4Ev
pub fn stub_0x304710() -> ! {
    todo!("0x304710 RBX::BrickColor::color4(void)const")
}

#[doc(alias = "RBX::BrickColor::color3(void)const")]
// 0x3047c4 — __ZNK3RBX10BrickColor6color3Ev
pub fn stub_0x3047c4() -> ! {
    todo!("0x3047c4 RBX::BrickColor::color3(void)const")
}

#[doc(alias = "RBX::hash_value(RBX::BrickColor const&)")]
// 0x3047ec — __ZN3RBX10hash_valueERKNS_10BrickColorE
pub fn stub_0x3047ec() -> ! {
    todo!("0x3047ec RBX::hash_value(RBX::BrickColor const&)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
// 0x304b70 — __ZN3RBX10BrickColor8BrickMapD1Ev
pub fn stub_0x304b70() -> ! {
    todo!("0x304b70 RBX::BrickColor::BrickMap::~BrickMap()")
}

#[doc(alias = "std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")]
// 0x304b74 — __ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_
pub fn stub_0x304b74() -> ! {
    todo!("0x304b74 std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0x304bcc — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0x304bcc() -> ! {
    todo!("0x304bcc std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0x304c80 — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0x304c80() -> ! {
    todo!("0x304c80 std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0x304cd8 — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0x304cd8() -> ! {
    todo!("0x304cd8 std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
// 0x304d40 — __ZN3RBX10BrickColor8BrickMapD2Ev
pub fn stub_0x304d40() -> ! {
    todo!("0x304d40 RBX::BrickColor::BrickMap::~BrickMap()")
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")]
// 0x304e3c — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev
pub fn stub_0x304e3c() -> ! {
    todo!("0x304e3c std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")]
// 0x304f0c — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x304f0c() -> ! {
    todo!("0x304f0c std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::BrickMap(void)")]
// 0x304f34 — __ZN3RBX10BrickColor8BrickMapC2Ev
pub fn stub_0x304f34() -> ! {
    todo!("0x304f34 RBX::BrickColor::BrickMap::BrickMap(void)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")]
// 0x30cbf8 — __ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs
pub fn stub_0x30cbf8() -> ! {
    todo!("0x30cbf8 RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")]
// 0x30cd98 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_
pub fn stub_0x30cd98() -> ! {
    todo!("0x30cd98 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")]
// 0x30cdc0 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x30cdc0() -> ! {
    todo!("0x30cdc0 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")]
// 0x30cea4 — __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm
pub fn stub_0x30cea4() -> ! {
    todo!("0x30cea4 std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")
}

#[doc(alias = "RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")]
// 0x30cebc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_
pub fn stub_0x30cebc() -> ! {
    todo!("0x30cebc RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")]
// 0x30cef8 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_
pub fn stub_0x30cef8() -> ! {
    todo!("0x30cef8 std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0x30cf54 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
pub fn stub_0x30cf54() -> ! {
    todo!("0x30cf54 std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")
}

#[doc(alias = "void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0x30d6d8 — __ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_
pub fn stub_0x30d6d8() -> ! {
    todo!("0x30d6d8 void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")]
// 0x30d71c — __ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm
pub fn stub_0x30d71c() -> ! {
    todo!("0x30d71c std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")]
// 0x30d740 — __ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type
pub fn stub_0x30d740() -> ! {
    todo!("0x30d740 void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0x30d88c — __ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_
pub fn stub_0x30d88c() -> ! {
    todo!("0x30d88c RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")]
// 0x30d8b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_
pub fn stub_0x30d8b8() -> ! {
    todo!("0x30d8b8 RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")]
// 0x30d914 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_
pub fn stub_0x30d914() -> ! {
    todo!("0x30d914 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(void)")]
// 0x30d948 — __ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv
pub fn stub_0x30d948() -> ! {
    todo!("0x30d948 RBX::BrickColor::BrickMap::generatePaletteMap(void)")
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")]
// 0x30da90 — __ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_
pub fn stub_0x30da90() -> ! {
    todo!("0x30da90 RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")]
// 0x30db44 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_
pub fn stub_0x30db44() -> ! {
    todo!("0x30db44 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")]
// 0x30db8c — __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_
pub fn stub_0x30db8c() -> ! {
    todo!("0x30db8c std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")]
// 0x30dbbc — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0x30dbbc() -> ! {
    todo!("0x30dbbc std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")
}

#[doc(alias = "RBX::CameraSubject::getContactManager(void)")]
// 0x30dd48 — __ZN3RBX13CameraSubject17getContactManagerEv
pub fn stub_0x30dd48() -> ! {
    todo!("0x30dd48 RBX::CameraSubject::getContactManager(void)")
}

#[doc(alias = "global constructor keyed to_a_108")]
// 0x30e1b0 — __GLOBAL__I_a_108
pub fn stub_0x30e1b0() -> ! {
    todo!("0x30e1b0 global constructor keyed to_a_108")
}

#[doc(alias = "RBX::Color::getColorByIndex(int)")]
// 0x30e3b8 — __ZN3RBX5Color15getColorByIndexEi
pub fn stub_0x30e3b8() -> ! {
    todo!("0x30e3b8 RBX::Color::getColorByIndex(int)")
}

#[doc(alias = "RBX::Color::colorFromIndex8(int)")]
// 0x30e580 — __ZN3RBX5Color15colorFromIndex8Ei
pub fn stub_0x30e580() -> ! {
    todo!("0x30e580 RBX::Color::colorFromIndex8(int)")
}

#[doc(alias = "RBX::Color::colorFromInt(unsigned int)")]
// 0x30e5c0 — __ZN3RBX5Color12colorFromIntEj
pub fn stub_0x30e5c0() -> ! {
    todo!("0x30e5c0 RBX::Color::colorFromInt(unsigned int)")
}

#[doc(alias = "RBX::Color::colorFromPointer(void *)")]
// 0x30e670 — __ZN3RBX5Color16colorFromPointerEPv
pub fn stub_0x30e670() -> ! {
    todo!("0x30e670 RBX::Color::colorFromPointer(void *)")
}

#[doc(alias = "global constructor keyed to_a_109")]
// 0x30e67c — __GLOBAL__I_a_109
pub fn stub_0x30e67c() -> ! {
    todo!("0x30e67c global constructor keyed to_a_109")
}

#[doc(alias = "RBX::ContentFilter::setFilterUrl(std::string)")]
// 0x30e6b4 — __ZN3RBX13ContentFilter12setFilterUrlESs
pub fn stub_0x30e6b4() -> ! {
    todo!("0x30e6b4 RBX::ContentFilter::setFilterUrl(std::string)")
}

#[doc(alias = "RBX::ContentFilter::setFilterLimits(int,int)")]
// 0x30e6bc — __ZN3RBX13ContentFilter15setFilterLimitsEii
pub fn stub_0x30e6bc() -> ! {
    todo!("0x30e6bc RBX::ContentFilter::setFilterLimits(int,int)")
}

#[doc(alias = "RBX::ContentFilter::ContentFilter(void)")]
// 0x30e6c8 — __ZN3RBX13ContentFilterC1Ev
pub fn stub_0x30e6c8() -> ! {
    todo!("0x30e6c8 RBX::ContentFilter::ContentFilter(void)")
}

#[doc(alias = "RBX::ContentFilter::ContentFilter(void)")]
// 0x30e6cc — __ZN3RBX13ContentFilterC2Ev
pub fn stub_0x30e6cc() -> ! {
    todo!("0x30e6cc RBX::ContentFilter::ContentFilter(void)")
}

#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// 0x30e868 — __ZN3RBX13ContentFilterD0Ev
pub fn stub_0x30e868() -> ! {
    todo!("0x30e868 RBX::ContentFilter::~ContentFilter()")
}

#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// 0x30e908 — __ZN3RBX13ContentFilterD1Ev
pub fn stub_0x30e908() -> ! {
    todo!("0x30e908 RBX::ContentFilter::~ContentFilter()")
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e90c — __ZThn32_N3RBX13ContentFilterD0Ev
pub fn stub_0x30e90c() -> ! {
    todo!("0x30e90c non-virtual thunk toRBX::ContentFilter::~ContentFilter()")
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e914 — __ZThn36_N3RBX13ContentFilterD0Ev
pub fn stub_0x30e914() -> ! {
    todo!("0x30e914 non-virtual thunk toRBX::ContentFilter::~ContentFilter()")
}

#[doc(alias = "RBX::ContentFilter::~ContentFilter()")]
// 0x30e91c — __ZN3RBX13ContentFilterD2Ev
pub fn stub_0x30e91c() -> ! {
    todo!("0x30e91c RBX::ContentFilter::~ContentFilter()")
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e96c — __ZThn32_N3RBX13ContentFilterD1Ev
pub fn stub_0x30e96c() -> ! {
    todo!("0x30e96c non-virtual thunk toRBX::ContentFilter::~ContentFilter()")
}

#[doc(alias = "non-virtual thunk toRBX::ContentFilter::~ContentFilter()")]
// 0x30e974 — __ZThn36_N3RBX13ContentFilterD1Ev
pub fn stub_0x30e974() -> ! {
    todo!("0x30e974 non-virtual thunk toRBX::ContentFilter::~ContentFilter()")
}

#[doc(alias = "RBX::ContentFilter::truncateString(std::string &)")]
// 0x30e97c — __ZN3RBX13ContentFilter14truncateStringERSs
pub fn stub_0x30e97c() -> ! {
    todo!("0x30e97c RBX::ContentFilter::truncateString(std::string &)")
}

#[doc(alias = "RBX::ContentFilter::getStringState(std::string &)")]
// 0x30eab0 — __ZN3RBX13ContentFilter14getStringStateERSs
pub fn stub_0x30eab0() -> ! {
    todo!("0x30eab0 RBX::ContentFilter::getStringState(std::string &)")
}

#[doc(alias = "RBX::ContentFilter::isContentFilterReady(std::string const&)")]
// 0x30eadc — __ZN3RBX13ContentFilter20isContentFilterReadyERKSs
pub fn stub_0x30eadc() -> ! {
    todo!("0x30eadc RBX::ContentFilter::isContentFilterReady(std::string const&)")
}

#[doc(alias = "RBX::ContentFilter::isStringSafe(std::string &)")]
// 0x30ee70 — __ZN3RBX13ContentFilter12isStringSafeERSs
pub fn stub_0x30ee70() -> ! {
    todo!("0x30ee70 RBX::ContentFilter::isStringSafe(std::string &)")
}

#[doc(alias = "RBX::ContentFilter::cleanTable(void)")]
// 0x30eebc — __ZN3RBX13ContentFilter10cleanTableEv
pub fn stub_0x30eebc() -> ! {
    todo!("0x30eebc RBX::ContentFilter::cleanTable(void)")
}

#[doc(alias = "RBX::ContentFilter::doFilterRequest(std::string)")]
// 0x30f0a0 — __ZN3RBX13ContentFilter15doFilterRequestESs
pub fn stub_0x30f0a0() -> ! {
    todo!("0x30f0a0 RBX::ContentFilter::doFilterRequest(std::string)")
}

#[doc(alias = "std::map<std::string,RBX::ContentFilter::ResultEntry,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::operator[](std::string const&)")]
// 0x310284 — __ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
pub fn stub_0x310284() -> ! {
    todo!("0x310284 std::map<std::string,RBX::ContentFilter::ResultEntry,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::operator[](std::string const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x310424 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0x310424() -> ! {
    todo!("0x310424 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x310510 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0x310510() -> ! {
    todo!("0x310510 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x310560 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0x310560() -> ! {
    todo!("0x310560 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_create_node(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0x3105e4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_0x3105e4() -> ! {
    todo!("0x3105e4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_create_node(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::lower_bound(std::string const&)")]
// 0x3106c4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
pub fn stub_0x3106c4() -> ! {
    todo!("0x3106c4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::lower_bound(std::string const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::string const&)")]
// 0x3106f4 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs
pub fn stub_0x3106f4() -> ! {
    todo!("0x3106f4 std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::string const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>,std::_Rb_tree_iterator<std::string>)")]
// 0x31071c — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_
pub fn stub_0x31071c() -> ! {
    todo!("0x31071c std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>,std::_Rb_tree_iterator<std::string>)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>)")]
// 0x310770 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE
pub fn stub_0x310770() -> ! {
    todo!("0x310770 std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::lower_bound(std::string const&)")]
// 0x310798 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs
pub fn stub_0x310798() -> ! {
    todo!("0x310798 std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::lower_bound(std::string const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::upper_bound(std::string const&)")]
// 0x3107c8 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs
pub fn stub_0x3107c8() -> ! {
    todo!("0x3107c8 std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::upper_bound(std::string const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::find(std::string const&)")]
// 0x312a54 — __ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs
pub fn stub_0x312a54() -> ! {
    todo!("0x312a54 std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::find(std::string const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::find(std::string const&)")]
// 0x312aa4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_0x312aa4() -> ! {
    todo!("0x312aa4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::find(std::string const&)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>)")]
// 0x312af4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
pub fn stub_0x312af4() -> ! {
    todo!("0x312af4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>)")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ContentFilter::ResultEntry>> *)")]
// 0x314a10 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x314a10() -> ! {
    todo!("0x314a10 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ContentFilter::ResultEntry>> *)")
}

#[doc(alias = "global constructor keyed to_a_110")]
// 0x314a40 — __GLOBAL__I_a_110
pub fn stub_0x314a40() -> ! {
    todo!("0x314a40 global constructor keyed to_a_110")
}

#[doc(alias = "RBX::operator<(RBX::ContentId const&,RBX::ContentId const&)")]
// 0x314c84 — __ZN3RBXltERKNS_9ContentIdES2_
pub fn stub_0x314c84() -> ! {
    todo!("0x314c84 RBX::operator<(RBX::ContentId const&,RBX::ContentId const&)")
}

#[doc(alias = "RBX::operator!=(RBX::ContentId const&,RBX::ContentId const&)")]
// 0x314c90 — __ZN3RBXneERKNS_9ContentIdES2_
pub fn stub_0x314c90() -> ! {
    todo!("0x314c90 RBX::operator!=(RBX::ContentId const&,RBX::ContentId const&)")
}

#[doc(alias = "RBX::operator==(RBX::ContentId const&,RBX::ContentId const&)")]
// 0x314ca8 — __ZN3RBXeqERKNS_9ContentIdES2_
pub fn stub_0x314ca8() -> ! {
    todo!("0x314ca8 RBX::operator==(RBX::ContentId const&,RBX::ContentId const&)")
}

#[doc(alias = "RBX::ContentId::fromUrl(std::string const&)")]
// 0x314cbc — __ZN3RBX9ContentId7fromUrlERKSs
pub fn stub_0x314cbc() -> ! {
    todo!("0x314cbc RBX::ContentId::fromUrl(std::string const&)")
}

#[doc(alias = "RBX::ContentId::CorrectBackslash(std::string &)")]
// 0x314cc8 — __ZN3RBX9ContentId16CorrectBackslashERSs
pub fn stub_0x314cc8() -> ! {
    todo!("0x314cc8 RBX::ContentId::CorrectBackslash(std::string &)")
}

#[doc(alias = "RBX::ContentId::convertAssetId(std::string const&)")]
// 0x314d14 — __ZN3RBX9ContentId14convertAssetIdERKSs
pub fn stub_0x314d14() -> ! {
    todo!("0x314d14 RBX::ContentId::convertAssetId(std::string const&)")
}

#[doc(alias = "anonymous namespace::createIdUrl(std::string &,std::string const&,std::string const&)")]
// 0x314f94 — __ZN12_GLOBAL__N_111createIdUrlERSsRKSsS2_
pub fn stub_0x314f94() -> ! {
    todo!("0x314f94 anonymous namespace::createIdUrl(std::string &,std::string const&,std::string const&)")
}

#[doc(alias = "RBX::ContentId::convertToLegacyContent(std::string const&)")]
// 0x315004 — __ZN3RBX9ContentId22convertToLegacyContentERKSs
pub fn stub_0x315004() -> ! {
    todo!("0x315004 RBX::ContentId::convertToLegacyContent(std::string const&)")
}

#[doc(alias = "RBX::ContentId::getAssetId(void)const")]
// 0x31507c — __ZNK3RBX9ContentId10getAssetIdEv
pub fn stub_0x31507c() -> ! {
    todo!("0x31507c RBX::ContentId::getAssetId(void)const")
}

#[doc(alias = "RBX::ContentId::fromAssets(char const*)")]
// 0x31530c — __ZN3RBX9ContentId10fromAssetsEPKc
pub fn stub_0x31530c() -> ! {
    todo!("0x31530c RBX::ContentId::fromAssets(char const*)")
}

#[doc(alias = "RBX::LegacyContentTable::~LegacyContentTable()")]
// 0x315514 — __ZN3RBX18LegacyContentTableD1Ev
pub fn stub_0x315514() -> ! {
    todo!("0x315514 RBX::LegacyContentTable::~LegacyContentTable()")
}

#[doc(alias = "global constructor keyed to_a_111")]
// 0x315594 — __GLOBAL__I_a_111
pub fn stub_0x315594() -> ! {
    todo!("0x315594 global constructor keyed to_a_111")
}

#[doc(alias = "RBX::FileSystem::getUserDirectory(bool,RBX::FileSystemDir,char const*)")]
// 0x315680 — __ZN3RBX10FileSystem16getUserDirectoryEbNS_13FileSystemDirEPKc
pub fn stub_0x315680() -> ! {
    todo!("0x315680 RBX::FileSystem::getUserDirectory(bool,RBX::FileSystemDir,char const*)")
}

#[doc(alias = "RBX::FileSystem::clearCacheDirectory(char const*,int)")]
// 0x315ba4 — __ZN3RBX10FileSystem19clearCacheDirectoryEPKci
pub fn stub_0x315ba4() -> ! {
    todo!("0x315ba4 RBX::FileSystem::clearCacheDirectory(char const*,int)")
}

#[doc(alias = "RBX::FileSystem::getCacheDirectory(bool,char const*)")]
// 0x315dc8 — __ZN3RBX10FileSystem17getCacheDirectoryEbPKc
pub fn stub_0x315dc8() -> ! {
    todo!("0x315dc8 RBX::FileSystem::getCacheDirectory(bool,char const*)")
}

#[doc(alias = "RBX::FileSystem::getBaseCacheDirectory(bool)")]
// 0x315dd4 — __ZN3RBX10FileSystem21getBaseCacheDirectoryEb
pub fn stub_0x315dd4() -> ! {
    todo!("0x315dd4 RBX::FileSystem::getBaseCacheDirectory(bool)")
}

#[doc(alias = "global constructor keyed to_a_112")]
// 0x3164c8 — __GLOBAL__I_a_112
pub fn stub_0x3164c8() -> ! {
    todo!("0x3164c8 global constructor keyed to_a_112")
}

#[doc(alias = "RBX::Http::getCdnResponceLock(void)")]
// 0x3165a0 — __ZN3RBX4Http18getCdnResponceLockEv
pub fn stub_0x3165a0() -> ! {
    todo!("0x3165a0 RBX::Http::getCdnResponceLock(void)")
}

#[doc(alias = "RBX::Http::init(RBX::Http::API)")]
// 0x3165b0 — __ZN3RBX4Http4initENS0_3APIE
pub fn stub_0x3165b0() -> ! {
    todo!("0x3165b0 RBX::Http::init(RBX::Http::API)")
}

#[doc(alias = "RBX::Http::ThrowIfFailure(bool,char const*,char const*)")]
// 0x316738 — __ZN3RBX4Http14ThrowIfFailureEbPKcS2_
pub fn stub_0x316738() -> ! {
    todo!("0x316738 RBX::Http::ThrowIfFailure(bool,char const*,char const*)")
}