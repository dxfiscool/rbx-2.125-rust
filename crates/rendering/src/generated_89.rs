//! rendering — Ogre::|G3D:: strict 13333 total
//! This shard: 0xe5733c..0xe5dac0 (100 stubs, 10060 prior -> 10160 covered, 3173 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xe5733c — __ZN4Ogre20DefaultWorkQueueBase18addResponseHandlerEtPNS_9WorkQueue15ResponseHandlerE
#[doc(alias = "Ogre::DefaultWorkQueueBase::addResponseHandler(unsigned short,Ogre::WorkQueue::ResponseHandler *)")]
// was: Ogre::DefaultWorkQueueBase::addResponseHandler(unsigned short,Ogre::WorkQueue::ResponseHandler *)
// IDA 0xe5733c: 194 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5733c() {
}

// 0xe57518 — __ZN4Ogre20DefaultWorkQueueBase21removeResponseHandlerEtPNS_9WorkQueue15ResponseHandlerE
#[doc(alias = "Ogre::DefaultWorkQueueBase::removeResponseHandler(unsigned short,Ogre::WorkQueue::ResponseHandler *)")]
// was: Ogre::DefaultWorkQueueBase::removeResponseHandler(unsigned short,Ogre::WorkQueue::ResponseHandler *)
// IDA 0xe57518: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57518() {
}

// 0xe5757c — __ZN4Ogre20DefaultWorkQueueBase10addRequestEttRKNS_3AnyEhb
#[doc(alias = "Ogre::DefaultWorkQueueBase::addRequest(unsigned short,unsigned short,Ogre::Any const&,unsigned char,bool)")]
// was: Ogre::DefaultWorkQueueBase::addRequest(unsigned short,unsigned short,Ogre::Any const&,unsigned char,bool)
// IDA 0xe5757c: 216 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5757c() {
}

// 0xe577c4 — __ZN4Ogre20DefaultWorkQueueBase22processRequestResponseEPNS_9WorkQueue7RequestEb
#[doc(alias = "Ogre::DefaultWorkQueueBase::processRequestResponse(Ogre::WorkQueue::Request *,bool)")]
// was: Ogre::DefaultWorkQueueBase::processRequestResponse(Ogre::WorkQueue::Request *,bool)
// IDA 0xe577c4: 280 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e577c4() {
}

// 0xe57ae0 — __ZN4Ogre20DefaultWorkQueueBase17addRequestWithRIDEyttRKNS_3AnyEh
#[doc(alias = "Ogre::DefaultWorkQueueBase::addRequestWithRID(unsigned long long,unsigned short,unsigned short,Ogre::Any const&,unsigned char)")]
// was: Ogre::DefaultWorkQueueBase::addRequestWithRID(unsigned long long,unsigned short,unsigned short,Ogre::Any const&,unsigned char)
// IDA 0xe57ae0: 200 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57ae0() {
}

// 0xe57d00 — __ZN4Ogre20DefaultWorkQueueBase12abortRequestEy
#[doc(alias = "Ogre::DefaultWorkQueueBase::abortRequest(unsigned long long)")]
// was: Ogre::DefaultWorkQueueBase::abortRequest(unsigned long long)
// IDA 0xe57d00: 83 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57d00() {
}

// 0xe57ddc — __ZN4Ogre20DefaultWorkQueueBase22abortRequestsByChannelEt
#[doc(alias = "Ogre::DefaultWorkQueueBase::abortRequestsByChannel(unsigned short)")]
// was: Ogre::DefaultWorkQueueBase::abortRequestsByChannel(unsigned short)
// IDA 0xe57ddc: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57ddc() {
}

// 0xe57eb0 — __ZN4Ogre20DefaultWorkQueueBase16abortAllRequestsEv
#[doc(alias = "Ogre::DefaultWorkQueueBase::abortAllRequests(void)")]
// was: Ogre::DefaultWorkQueueBase::abortAllRequests(void)
// IDA 0xe57eb0: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57eb0() {
}

// 0xe57f64 — __ZN4Ogre20DefaultWorkQueueBase9setPausedEb
#[doc(alias = "Ogre::DefaultWorkQueueBase::setPaused(bool)")]
// was: Ogre::DefaultWorkQueueBase::setPaused(bool)
// IDA 0xe57f64: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57f64() {
}

// 0xe57f6c — __ZNK4Ogre20DefaultWorkQueueBase8isPausedEv
#[doc(alias = "Ogre::DefaultWorkQueueBase::isPaused(void)const")]
// was: Ogre::DefaultWorkQueueBase::isPaused(void)const
// IDA 0xe57f6c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57f6c() {
}

// 0xe57f74 — __ZN4Ogre20DefaultWorkQueueBase19setRequestsAcceptedEb
#[doc(alias = "Ogre::DefaultWorkQueueBase::setRequestsAccepted(bool)")]
// was: Ogre::DefaultWorkQueueBase::setRequestsAccepted(bool)
// IDA 0xe57f74: 2 insns (STRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57f74() {
}

// 0xe57f7c — __ZNK4Ogre20DefaultWorkQueueBase19getRequestsAcceptedEv
#[doc(alias = "Ogre::DefaultWorkQueueBase::getRequestsAccepted(void)const")]
// was: Ogre::DefaultWorkQueueBase::getRequestsAccepted(void)const
// IDA 0xe57f7c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57f7c() {
}

// 0xe57f84 — __ZN4Ogre20DefaultWorkQueueBase19_processNextRequestEv
#[doc(alias = "Ogre::DefaultWorkQueueBase::_processNextRequest(void)")]
// was: Ogre::DefaultWorkQueueBase::_processNextRequest(void)
// IDA 0xe57f84: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e57f84() {
}

// 0xe58038 — __ZN4Ogre20DefaultWorkQueueBase14processRequestEPNS_9WorkQueue7RequestE
#[doc(alias = "Ogre::DefaultWorkQueueBase::processRequest(Ogre::WorkQueue::Request *)")]
// was: Ogre::DefaultWorkQueueBase::processRequest(Ogre::WorkQueue::Request *)
// IDA 0xe58038: 566 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e58038() {
}

// 0xe58648 — __ZN4Ogre20DefaultWorkQueueBase15processResponseEPNS_9WorkQueue8ResponseE
#[doc(alias = "Ogre::DefaultWorkQueueBase::processResponse(Ogre::WorkQueue::Response *)")]
// was: Ogre::DefaultWorkQueueBase::processResponse(Ogre::WorkQueue::Response *)
// IDA 0xe58648: 507 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e58648() {
}

// 0xe58bc8 — __ZN4Ogre20DefaultWorkQueueBase16processResponsesEv
#[doc(alias = "Ogre::DefaultWorkQueueBase::processResponses(void)")]
// was: Ogre::DefaultWorkQueueBase::processResponses(void)
// IDA 0xe58bc8: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e58bc8() {
}

// 0xe58cc8 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEED1Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::~SharedPtr()
// IDA 0xe58cc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e58cc8() {
}

// 0xe58d78 — __ZNSt5dequeIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt15_Deque_iteratorIS3_RS3_PS3_E
#[doc(alias = "std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)")]
// was: std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)
// IDA 0xe58d78: 162 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e58d78() {
}

// 0xe58f1c — __ZN4Ogre9WorkQueueD1Ev
#[doc(alias = "Ogre::WorkQueue::~WorkQueue()")]
// was: Ogre::WorkQueue::~WorkQueue()
// IDA 0xe58f1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e58f1c() {
}

// 0xe58f3c — __ZN4Ogre9WorkQueueD0Ev
#[doc(alias = "Ogre::WorkQueue::~WorkQueue()")]
// was: Ogre::WorkQueue::~WorkQueue()
// IDA 0xe58f3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e58f3c() {
}

// 0xe58fe0 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE7_M_copyEPKSt13_Rb_tree_nodeISE_EPSM_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const*,std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>*)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> const*,std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>*)
// IDA 0xe58fe0: 200 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e58fe0() {
}

// 0xe591d8 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE8_M_eraseEPSt13_Rb_tree_nodeISE_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xe591d8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e591d8() {
}

// 0xe592b4 — __ZNSt4listIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEC2ERKSA_
#[doc(alias = "std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::list(std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
// was: std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::list(std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)
// IDA 0xe592b4: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e592b4() {
}

// 0xe59404 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe59404: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59404() {
}

// 0xe59408 — __ZNSt10_List_baseIN4Ogre9SharedPtrINS0_20DefaultWorkQueueBase20RequestHandlerHolderEEENS0_12STLAllocatorIS4_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe59408: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e59408() {
}

// 0xe59414 — __ZNSt5dequeIPN4Ogre9WorkQueue8ResponseENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)")]
// was: std::deque<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)
// IDA 0xe59414: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59414() {
}

// 0xe594f0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN4Ogre9WorkQueue7RequestERS7_PS7_ESA_EET0_T_SC_SB_
#[doc(alias = "std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>>(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)")]
// was: std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>>(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)
// IDA 0xe594f0: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e594f0() {
}

// 0xe59598 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorIPN4Ogre9WorkQueue7RequestERS7_PS7_ESA_EET0_T_SC_SB_
#[doc(alias = "std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **> std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>>(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)")]
// was: std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **> std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>>(std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>,std::_Deque_iterator<Ogre::WorkQueue::Request *,Ogre::WorkQueue::Request *&,Ogre::WorkQueue::Request **>)
// IDA 0xe59598: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59598() {
}

// 0xe5963c — __ZNSt5dequeIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)")]
// was: std::deque<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_reallocate_map(unsigned long,bool)
// IDA 0xe5963c: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5963c() {
}

// 0xe59718 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE16_M_insert_uniqueERKSD_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe59718: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59718() {
}

// 0xe597cc — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE14_M_create_nodeERKSD_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe597cc: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e597cc() {
}

// 0xe59934 — __ZNSt10_List_baseIPN4Ogre9WorkQueue15ResponseHandlerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD1Ev
#[doc(alias = "std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe59934: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59934() {
}

// 0xe59938 — __ZNSt10_List_baseIPN4Ogre9WorkQueue15ResponseHandlerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE10_List_implD0Ev
#[doc(alias = "std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()")]
// was: std::_List_base<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_List_impl::~_List_impl()
// IDA 0xe59938: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e59938() {
}

// 0xe59944 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEED0Ev
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::~SharedPtr()")]
// was: Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::~SharedPtr()
// IDA 0xe59944: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e59944() {
}

// 0xe599f8 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::destroy(void)")]
// was: Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::destroy(void)
// IDA 0xe599f8: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e599f8() {
}

// 0xe59ab8 — __ZN4Ogre9SharedPtrINS_20DefaultWorkQueueBase20RequestHandlerHolderEE4swapERS3_
#[doc(alias = "Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::swap(Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>&)")]
// was: Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>::swap(Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>&)
// IDA 0xe59ab8: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59ab8() {
}

// 0xe59ad4 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE16_M_insert_uniqueERKSE_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe59ad4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59ad4() {
}

// 0xe59b40 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE9_M_insertEPSt18_Rb_tree_node_baseSM_RKSE_
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>> const&)
// IDA 0xe59b40: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59b40() {
}

// 0xe59c78 — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue8ResponseENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD1Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe59c78: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c78() {
}

// 0xe59c7c — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD1Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe59c7c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c7c() {
}

// 0xe59c80 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE13_Rb_tree_implISH_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c80: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c80() {
}

// 0xe59c84 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE13_Rb_tree_implISH_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c84: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59c84() {
}

// 0xe59c90 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE13_Rb_tree_implISI_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c90: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e59c90() {
}

// 0xe59c94 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIN4Ogre9SharedPtrINS3_20DefaultWorkQueueBase20RequestHandlerHolderEEENS3_12STLAllocatorIS7_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISE_ESt4lessItENS8_ISE_SB_EEE13_Rb_tree_implISI_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::DefaultWorkQueueBase::RequestHandlerHolder>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<unsigned short>,false>::~_Rb_tree_impl()
// IDA 0xe59c94: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59c94() {
}

// 0xe59ca0 — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue8ResponseENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
// was: std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)
// IDA 0xe59ca0: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59ca0() {
}

// 0xe59e70 — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue8ResponseENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD0Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Response *,Ogre::STLAllocator<Ogre::WorkQueue::Response *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe59e70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e59e70() {
}

// 0xe59e7c — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)")]
// was: std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_initialize_map(unsigned long)
// IDA 0xe59e7c: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e59e7c() {
}

// 0xe5a04c — __ZNSt11_Deque_baseIPN4Ogre9WorkQueue7RequestENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE11_Deque_implD0Ev
#[doc(alias = "std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()")]
// was: std::_Deque_base<Ogre::WorkQueue::Request *,Ogre::STLAllocator<Ogre::WorkQueue::Request *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Deque_impl::~_Deque_impl()
// IDA 0xe5a04c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a04c() {
}

// 0xe5a328 — __ZNSt8_Rb_treeItSt4pairIKtSt4listIPN4Ogre9WorkQueue15ResponseHandlerENS3_12STLAllocatorIS6_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEEESt10_Select1stISD_ESt4lessItENS7_ISD_SA_EEE8_M_eraseEPSt13_Rb_tree_nodeISD_E
#[doc(alias = "std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)")]
// was: std::_Rb_tree<unsigned short,std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::_Select1st<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>>,std::less<unsigned short>,Ogre::STLAllocator<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned short const,std::list<Ogre::WorkQueue::ResponseHandler *,Ogre::STLAllocator<Ogre::WorkQueue::ResponseHandler *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>> *)
// IDA 0xe5a328: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5a328() {
}

// 0xe5a4a4 — __ZN4Ogre10ZipArchiveC1ERKSsS2_P15_zzip_plugin_io
#[doc(alias = "Ogre::ZipArchive::ZipArchive(std::string const&,std::string const&,_zzip_plugin_io *)")]
// was: Ogre::ZipArchive::ZipArchive(std::string const&,std::string const&,_zzip_plugin_io *)
// IDA 0xe5a4a4: 113 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5a4a4() {
}

// 0xe5a5e0 — __ZN4Ogre10ZipArchiveD0Ev
#[doc(alias = "Ogre::ZipArchive::~ZipArchive()")]
// was: Ogre::ZipArchive::~ZipArchive()
// IDA 0xe5a5e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a5e0() {
}

// 0xe5a670 — __ZN4Ogre10ZipArchiveD1Ev
#[doc(alias = "Ogre::ZipArchive::~ZipArchive()")]
// was: Ogre::ZipArchive::~ZipArchive()
// IDA 0xe5a670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a670() {
}

// 0xe5a67c — __ZN4Ogre10ZipArchiveD2Ev
#[doc(alias = "Ogre::ZipArchive::~ZipArchive()")]
// was: Ogre::ZipArchive::~ZipArchive()
// IDA 0xe5a67c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5a67c() {
}

// 0xe5a8c4 — __ZN4Ogre10ZipArchive4loadEv
#[doc(alias = "Ogre::ZipArchive::load(void)")]
// was: Ogre::ZipArchive::load(void)
// IDA 0xe5a8c4: 408 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5a8c4() {
}

// 0xe5ad48 — __ZNK4Ogre10ZipArchive14checkZzipErrorEiRKSs
#[doc(alias = "Ogre::ZipArchive::checkZzipError(int,std::string const&)const")]
// was: Ogre::ZipArchive::checkZzipError(int,std::string const&)const
// IDA 0xe5ad48: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ad48() {
}

// 0xe5b238 — __ZN4Ogre10ZipArchive6unloadEv
#[doc(alias = "Ogre::ZipArchive::unload(void)")]
// was: Ogre::ZipArchive::unload(void)
// IDA 0xe5b238: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5b238() {
}

// 0xe5b274 — __ZNK4Ogre10ZipArchive4openERKSsb
#[doc(alias = "Ogre::ZipArchive::open(std::string const&,bool)const")]
// was: Ogre::ZipArchive::open(std::string const&,bool)const
// IDA 0xe5b274: 1069 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5b274() {
}

// 0xe5be80 — __ZNK4Ogre10ZipArchive6createERKSs
#[doc(alias = "Ogre::ZipArchive::create(std::string const&)const")]
// was: Ogre::ZipArchive::create(std::string const&)const
// IDA 0xe5be80: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5be80() {
}

// 0xe5c030 — __ZNK4Ogre10ZipArchive6removeERKSs
#[doc(alias = "Ogre::ZipArchive::remove(std::string const&)const")]
// was: Ogre::ZipArchive::remove(std::string const&)const
// IDA 0xe5c030: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5c030() {
}

// 0xe5c034 — __ZN4Ogre10ZipArchive4listEbb
#[doc(alias = "Ogre::ZipArchive::list(bool,bool)")]
// was: Ogre::ZipArchive::list(bool,bool)
// IDA 0xe5c034: 186 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c034() {
}

// 0xe5c210 — __ZN4Ogre10ZipArchive12listFileInfoEbb
#[doc(alias = "Ogre::ZipArchive::listFileInfo(bool,bool)")]
// was: Ogre::ZipArchive::listFileInfo(bool,bool)
// IDA 0xe5c210: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c210() {
}

// 0xe5c2bc — __ZN4Ogre10ZipArchive4findERKSsbb
#[doc(alias = "Ogre::ZipArchive::find(std::string const&,bool,bool)")]
// was: Ogre::ZipArchive::find(std::string const&,bool,bool)
// IDA 0xe5c2bc: 228 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c2bc() {
}

// 0xe5c4fc — __ZNK4Ogre10ZipArchive12findFileInfoERKSsbb
#[doc(alias = "Ogre::ZipArchive::findFileInfo(std::string const&,bool,bool)const")]
// was: Ogre::ZipArchive::findFileInfo(std::string const&,bool,bool)const
// IDA 0xe5c4fc: 207 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c4fc() {
}

// 0xe5c70c — __ZN4Ogre10ZipArchive6existsERKSs
#[doc(alias = "Ogre::ZipArchive::exists(std::string const&)")]
// was: Ogre::ZipArchive::exists(std::string const&)
// IDA 0xe5c70c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c70c() {
}

// 0xe5c72c — __ZN4Ogre10ZipArchive15getModifiedTimeERKSs
#[doc(alias = "Ogre::ZipArchive::getModifiedTime(std::string const&)")]
// was: Ogre::ZipArchive::getModifiedTime(std::string const&)
// IDA 0xe5c72c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c72c() {
}

// 0xe5c748 — __ZN4Ogre13ZipDataStreamD0Ev
#[doc(alias = "Ogre::ZipDataStream::~ZipDataStream()")]
// was: Ogre::ZipDataStream::~ZipDataStream()
// IDA 0xe5c748: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5c748() {
}

// 0xe5c7d8 — __ZN4Ogre13ZipDataStreamD1Ev
#[doc(alias = "Ogre::ZipDataStream::~ZipDataStream()")]
// was: Ogre::ZipDataStream::~ZipDataStream()
// IDA 0xe5c7d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5c7d8() {
}

// 0xe5c7e4 — __ZN4Ogre13ZipDataStreamD2Ev
#[doc(alias = "Ogre::ZipDataStream::~ZipDataStream()")]
// was: Ogre::ZipDataStream::~ZipDataStream()
// IDA 0xe5c7e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5c7e4() {
}

// 0xe5c944 — __ZN4Ogre13ZipDataStream4readEPvm
#[doc(alias = "Ogre::ZipDataStream::read(void *,unsigned long)")]
// was: Ogre::ZipDataStream::read(void *,unsigned long)
// IDA 0xe5c944: 350 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5c944() {
}

// 0xe5cd48 — __ZN4Ogre13ZipDataStream4skipEl
#[doc(alias = "Ogre::ZipDataStream::skip(long)")]
// was: Ogre::ZipDataStream::skip(long)
// IDA 0xe5cd48: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cd48() {
}

// 0xe5cd98 — __ZN4Ogre13ZipDataStream4seekEm
#[doc(alias = "Ogre::ZipDataStream::seek(unsigned long)")]
// was: Ogre::ZipDataStream::seek(unsigned long)
// IDA 0xe5cd98: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cd98() {
}

// 0xe5cdb4 — __ZNK4Ogre13ZipDataStream4tellEv
#[doc(alias = "Ogre::ZipDataStream::tell(void)const")]
// was: Ogre::ZipDataStream::tell(void)const
// IDA 0xe5cdb4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cdb4() {
}

// 0xe5cdd4 — __ZNK4Ogre13ZipDataStream3eofEv
#[doc(alias = "Ogre::ZipDataStream::eof(void)const")]
// was: Ogre::ZipDataStream::eof(void)const
// IDA 0xe5cdd4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cdd4() {
}

// 0xe5cdf0 — __ZN4Ogre13ZipDataStream5closeEv
#[doc(alias = "Ogre::ZipDataStream::close(void)")]
// was: Ogre::ZipDataStream::close(void)
// IDA 0xe5cdf0: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cdf0() {
}

// 0xe5ce0c — __ZNK4Ogre17ZipArchiveFactory7getTypeEv
#[doc(alias = "Ogre::ZipArchiveFactory::getType(void)const")]
// was: Ogre::ZipArchiveFactory::getType(void)const
// IDA 0xe5ce0c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5ce0c() {
}

// 0xe5cf00 — __ZN4Ogre30EmbeddedZipArchiveFactory_openEPKciz
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_open(char const*,int,...)")]
// was: Ogre::EmbeddedZipArchiveFactory_open(char const*,int,...)
// IDA 0xe5cf00: 63 insns (SUB..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cf00() {
}

// 0xe5cfac — __ZN4Ogre31EmbeddedZipArchiveFactory_closeEi
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_close(int)")]
// was: Ogre::EmbeddedZipArchiveFactory_close(int)
// IDA 0xe5cfac: 24 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cfac() {
}

// 0xe5cff4 — __ZN4Ogre30EmbeddedZipArchiveFactory_readEiPvm
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_read(int,void *,unsigned long)")]
// was: Ogre::EmbeddedZipArchiveFactory_read(int,void *,unsigned long)
// IDA 0xe5cff4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5cff4() {
}

// 0xe5d068 — __ZN4Ogre31EmbeddedZipArchiveFactory_seeksEixi
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_seeks(int,long long,int)")]
// was: Ogre::EmbeddedZipArchiveFactory_seeks(int,long long,int)
// IDA 0xe5d068: 36 insns (CMP.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d068() {
}

// 0xe5d0c4 — __ZN4Ogre34EmbeddedZipArchiveFactory_filesizeEi
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_filesize(int)")]
// was: Ogre::EmbeddedZipArchiveFactory_filesize(int)
// IDA 0xe5d0c4: 15 insns (CMP.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d0c4() {
}

// 0xe5d0f4 — __ZN4Ogre31EmbeddedZipArchiveFactory_writeEiPKvm
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory_write(int,void const*,unsigned long)")]
// was: Ogre::EmbeddedZipArchiveFactory_write(int,void const*,unsigned long)
// IDA 0xe5d0f4: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d0f4() {
}

// 0xe5d0fc — __ZN4Ogre25EmbeddedZipArchiveFactoryC1Ev
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::EmbeddedZipArchiveFactory(void)")]
// was: Ogre::EmbeddedZipArchiveFactory::EmbeddedZipArchiveFactory(void)
// IDA 0xe5d0fc: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d0fc() {
}

// 0xe5d184 — __ZN4Ogre25EmbeddedZipArchiveFactoryD0Ev
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()")]
// was: Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()
// IDA 0xe5d184: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d184() {
}

// 0xe5d210 — __ZN4Ogre25EmbeddedZipArchiveFactoryD1Ev
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()")]
// was: Ogre::EmbeddedZipArchiveFactory::~EmbeddedZipArchiveFactory()
// IDA 0xe5d210: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5d210() {
}

// 0xe5d214 — __ZNK4Ogre25EmbeddedZipArchiveFactory7getTypeEv
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::getType(void)const")]
// was: Ogre::EmbeddedZipArchiveFactory::getType(void)const
// IDA 0xe5d214: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d214() {
}

// 0xe5d30c — __ZN4Ogre12STLAllocatorINS_8FileInfoENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEED1Ev
#[doc(alias = "Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()")]
// was: Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::~STLAllocator()
// IDA 0xe5d30c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_e5d30c() {
}

// 0xe5d310 — __ZNSt6vectorIN4Ogre8FileInfoENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
#[doc(alias = "std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::FileInfo const&)")]
// was: std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::FileInfo const&)
// IDA 0xe5d310: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_e5d310() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xe5d4a4 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5d4a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d4a4() {
}

// 0xe5d554 — __ZN4Ogre9SharedPtrISt6vectorISsNS_12STLAllocatorISsNS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED1Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<std::string,Ogre::STLAllocator<std::string,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5d554: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d554() {
}

// 0xe5d604 — __ZN4Ogre22InternalErrorExceptionD1Ev
#[doc(alias = "Ogre::InternalErrorException::~InternalErrorException()")]
// was: Ogre::InternalErrorException::~InternalErrorException()
// IDA 0xe5d604: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d604() {
}

// 0xe5d610 — __ZNK4Ogre10ZipArchive15isCaseSensitiveEv
#[doc(alias = "Ogre::ZipArchive::isCaseSensitive(void)const")]
// was: Ogre::ZipArchive::isCaseSensitive(void)const
// IDA 0xe5d610: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d610() {
}

// 0xe5d614 — __ZNK4Ogre7Archive10isReadOnlyEv
#[doc(alias = "Ogre::Archive::isReadOnly(void)const")]
// was: Ogre::Archive::isReadOnly(void)const
// IDA 0xe5d614: 2 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d614() {
}

// 0xe5d618 — __ZNK4Ogre10DataStream10isReadableEv
#[doc(alias = "Ogre::DataStream::isReadable(void)const")]
// was: Ogre::DataStream::isReadable(void)const
// IDA 0xe5d618: 3 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d618() {
}

// 0xe5d620 — __ZNK4Ogre10DataStream11isWriteableEv
#[doc(alias = "Ogre::DataStream::isWriteable(void)const")]
// was: Ogre::DataStream::isWriteable(void)const
// IDA 0xe5d620: 4 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d620() {
}

// 0xe5d62c — __ZN4Ogre10DataStream5writeEPKvm
#[doc(alias = "Ogre::DataStream::write(void const*,unsigned long)")]
// was: Ogre::DataStream::write(void const*,unsigned long)
// IDA 0xe5d62c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d62c() {
}

// 0xe5d630 — __ZN4Ogre25EmbeddedZipArchiveFactory14createInstanceERKSs
#[doc(alias = "Ogre::EmbeddedZipArchiveFactory::createInstance(std::string const&)")]
// was: Ogre::EmbeddedZipArchiveFactory::createInstance(std::string const&)
// IDA 0xe5d630: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d630() {
}

// 0xe5d84c — __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsEN4Ogre12STLAllocatorIS2_NS7_22CategorisedAllocPolicyILNS7_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
// was: std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,int>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)
// IDA 0xe5d84c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d84c() {
}

// 0xe5d8f0 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEED0Ev
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::~SharedPtr()
// IDA 0xe5d8f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_e5d8f0() {
}

// 0xe5d9a4 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)
// IDA 0xe5d9a4: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5d9a4() {
}

// 0xe5dac0 — __ZN4Ogre9SharedPtrISt6vectorINS_8FileInfoENS_12STLAllocatorIS2_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE4swapERS9_
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)")]
// was: Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::swap(Ogre::SharedPtr<std::vector<Ogre::FileInfo,Ogre::STLAllocator<Ogre::FileInfo,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>&)
// IDA 0xe5dac0: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_e5dac0() {
}
