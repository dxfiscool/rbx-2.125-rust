//! core shard FI — 100 core stubs EA-sorted, 0xf2dae4..0xf2e3d4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, gap repair FG 0xf2dad4 -> FH 0xf2e3e4, lowest uncovered).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap repair FG 0xf2dad4 -> FH 0xf2e3e4, lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")]
// 0xf2dae4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)
pub fn stub_f2dae4() -> ! {
    todo!("0xf2dae4 j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_")
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")]
// 0xf2db24 — j___ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_
pub fn stub_f2db24() -> ! {
    todo!("0xf2db24 j___ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
// 0xf2db34 — j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)
pub fn stub_f2db34() -> ! {
    todo!("0xf2db34 j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
// 0xf2db44 — j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)
pub fn stub_f2db44() -> ! {
    todo!("0xf2db44 j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf2db64 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f2db64() -> ! {
    todo!("0xf2db64 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// 0xf2db74 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub fn stub_f2db74() -> ! {
    todo!("0xf2db74 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf2db84 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f2db84() -> ! {
    todo!("0xf2db84 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const")]
// 0xf2dbc4 — j___ZNK5boost9function2IbRKSsPSsEclES2_S3_
pub fn stub_f2dbc4() -> ! {
    todo!("0xf2dbc4 j___ZNK5boost9function2IbRKSsPSsEclES2_S3_")
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
// 0xf2dbd4 — j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
pub fn stub_f2dbd4() -> ! {
    todo!("0xf2dbd4 j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv")
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
// 0xf2dbe4 — j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
pub fn stub_f2dbe4() -> ! {
    todo!("0xf2dbe4 j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
// 0xf2dbf4 — j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
pub fn stub_f2dbf4() -> ! {
    todo!("0xf2dbf4 j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
// 0xf2dc04 — j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_f2dc04() -> ! {
    todo!("0xf2dc04 j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
// 0xf2dc14 — j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
pub fn stub_f2dc14() -> ! {
    todo!("0xf2dc14 j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
// 0xf2dc24 — j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
pub fn stub_f2dc24() -> ! {
    todo!("0xf2dc24 j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev")
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
// 0xf2dc34 — j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
pub fn stub_f2dc34() -> ! {
    todo!("0xf2dc34 j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
// 0xf2dc44 — j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
pub fn stub_f2dc44() -> ! {
    todo!("0xf2dc44 j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
// 0xf2dc54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
pub fn stub_f2dc54() -> ! {
    todo!("0xf2dc54 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
// 0xf2dc64 — j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_f2dc64() -> ! {
    todo!("0xf2dc64 j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
// 0xf2dc74 — j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
pub fn stub_f2dc74() -> ! {
    todo!("0xf2dc74 j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
// 0xf2dc84 — j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_f2dc84() -> ! {
    todo!("0xf2dc84 j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
// 0xf2dc94 — j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
pub fn stub_f2dc94() -> ! {
    todo!("0xf2dc94 j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_")
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
// 0xf2dca4 — j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
pub fn stub_f2dca4() -> ! {
    todo!("0xf2dca4 j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv")
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
// 0xf2dcb4 — j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
pub fn stub_f2dcb4() -> ! {
    todo!("0xf2dcb4 j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
// 0xf2dcc4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_
pub fn stub_f2dcc4() -> ! {
    todo!("0xf2dcc4 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
// 0xf2dcd4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
pub fn stub_f2dcd4() -> ! {
    todo!("0xf2dcd4 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
// 0xf2dce4 — j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f2dce4() -> ! {
    todo!("0xf2dce4 j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")]
// 0xf2dcf4 — j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_
pub fn stub_f2dcf4() -> ! {
    todo!("0xf2dcf4 j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
// 0xf2dd04 — j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
pub fn stub_f2dd04() -> ! {
    todo!("0xf2dd04 j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
// 0xf2dd14 — j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
pub fn stub_f2dd14() -> ! {
    todo!("0xf2dd14 j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
// 0xf2dd24 — j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev
pub fn stub_f2dd24() -> ! {
    todo!("0xf2dd24 j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
// 0xf2dd34 — j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
pub fn stub_f2dd34() -> ! {
    todo!("0xf2dd34 j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_")
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")]
// 0xf2de34 — j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_
pub fn stub_f2de34() -> ! {
    todo!("0xf2de34 j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_")
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(void)")]
// 0xf2de44 — j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv
pub fn stub_f2de44() -> ! {
    todo!("0xf2de44 j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv")
}

#[doc(alias = "RBX::BrickColor::BrickMap::setRenderingSupportedPaletteSize(unsigned long)")]
// 0xf2de54 — j___ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm
pub fn stub_f2de54() -> ! {
    todo!("0xf2de54 j___ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm")
}

#[doc(alias = "RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")]
// 0xf2de64 — j___ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs
pub fn stub_f2de64() -> ! {
    todo!("0xf2de64 j___ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs")
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0xf2de74 — j___ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_
pub fn stub_f2de74() -> ! {
    todo!("0xf2de74 j___ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_")
}

#[doc(alias = "RBX::BrickColor::BrickMap::BrickMap(void)")]
// 0xf2de84 — j___ZN3RBX10BrickColor8BrickMapC2Ev
pub fn stub_f2de84() -> ! {
    todo!("0xf2de84 j___ZN3RBX10BrickColor8BrickMapC2Ev")
}

#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
// 0xf2de94 — j___ZN3RBX10BrickColor8BrickMapD2Ev
pub fn stub_f2de94() -> ! {
    todo!("0xf2de94 j___ZN3RBX10BrickColor8BrickMapD2Ev")
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")]
// 0xf2dea4 — j___ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm
pub fn stub_f2dea4() -> ! {
    todo!("0xf2dea4 j___ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")]
// 0xf2deb4 — j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm
pub fn stub_f2deb4() -> ! {
    todo!("0xf2deb4 j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")]
// 0xf2dec4 — j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_
pub fn stub_f2dec4() -> ! {
    todo!("0xf2dec4 j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_")
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")]
// 0xf2ded4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_
pub fn stub_f2ded4() -> ! {
    todo!("0xf2ded4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_")
}

#[doc(alias = "RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")]
// 0xf2dee4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_
pub fn stub_f2dee4() -> ! {
    todo!("0xf2dee4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")]
// 0xf2def4 — j___ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_
pub fn stub_f2def4() -> ! {
    todo!("0xf2def4 j___ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_")
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0xf2df04 — j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
pub fn stub_f2df04() -> ! {
    todo!("0xf2df04 j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")]
// 0xf2df14 — j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_
pub fn stub_f2df14() -> ! {
    todo!("0xf2df14 j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_")
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")]
// 0xf2df24 — j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev
pub fn stub_f2df24() -> ! {
    todo!("0xf2df24 j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")]
// 0xf2df34 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f2df34() -> ! {
    todo!("0xf2df34 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")]
// 0xf2df44 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_f2df44() -> ! {
    todo!("0xf2df44 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")]
// 0xf2df54 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_
pub fn stub_f2df54() -> ! {
    todo!("0xf2df54 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")]
// 0xf2df64 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_
pub fn stub_f2df64() -> ! {
    todo!("0xf2df64 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")]
// 0xf2df74 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_
pub fn stub_f2df74() -> ! {
    todo!("0xf2df74 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0xf2df84 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_f2df84() -> ! {
    todo!("0xf2df84 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0xf2df94 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_f2df94() -> ! {
    todo!("0xf2df94 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")]
// 0xf2dfa4 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f2dfa4() -> ! {
    todo!("0xf2dfa4 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")]
// 0xf2dfb4 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_f2dfb4() -> ! {
    todo!("0xf2dfb4 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")]
// 0xf2dfc4 — j___ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type
pub fn stub_f2dfc4() -> ! {
    todo!("0xf2dfc4 j___ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// 0xf2dfd4 — j___ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_
pub fn stub_f2dfd4() -> ! {
    todo!("0xf2dfd4 j___ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ContentFilter> RBX::weak_from<RBX::ContentFilter>(RBX::ContentFilter*)")]
// 0xf2e044 — j___ZN3RBX9weak_fromINS_13ContentFilterEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::ContentFilter> RBX::weak_from<RBX::ContentFilter>(RBX::ContentFilter*)
pub fn stub_f2e044() -> ! {
    todo!("0xf2e044 j___ZN3RBX9weak_fromINS_13ContentFilterEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter>(rbx_core::WeakPtr<RBX::ContentFilter> const&,boost::detail::sp_nothrow_tag)")]
// 0xf2e054 — j___ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter>(boost::weak_ptr<RBX::ContentFilter> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f2e054() -> ! {
    todo!("0xf2e054 j___ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
// 0xf2e064 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEC2ES7_S8_
// was: boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>)
pub fn stub_f2e064() -> ! {
    todo!("0xf2e064 j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEC2ES7_S8_")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
// 0xf2e084 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEC2ES7_S8_S9_
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>)
pub fn stub_f2e084() -> ! {
    todo!("0xf2e084 j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEC2ES7_S8_S9_")
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
// 0xf2e0a4 — j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_
// was: boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>)
pub fn stub_f2e0a4() -> ! {
    todo!("0xf2e0a4 j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_")
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0xf2e0b4 — j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEclIPFvPSsPSt9exceptionS9_SsENS0_5list2IRSE_RSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)
pub fn stub_f2e0b4() -> ! {
    todo!("0xf2e0b4 j___ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEclIPFvPSsPSt9exceptionS9_SsENS0_5list2IRSE_RSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
// 0xf2e0c4 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEC2ES7_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>)
pub fn stub_f2e0c4() -> ! {
    todo!("0xf2e0c4 j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEC2ES7_S8_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
// 0xf2e0d4 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEC2ES7_S8_S9_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>)
pub fn stub_f2e0d4() -> ! {
    todo!("0xf2e0d4 j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEC2ES7_S8_S9_")
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>)")]
// 0xf2e0e4 — j___ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_
// was: boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>)
pub fn stub_f2e0e4() -> ! {
    todo!("0xf2e0e4 j___ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS5_ISsEEEC2ES3_S4_SA_SB_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::ContentFilter>,std::string>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>(void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
// 0xf2e0f4 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX13ContentFilterEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list_av_2<boost::weak_ptr<RBX::ContentFilter>,std::string>::type> boost::bind<void,boost::weak_ptr<RBX::ContentFilter>,std::string,boost::weak_ptr<RBX::ContentFilter>,std::string>(void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::weak_ptr<RBX::ContentFilter>,std::string)
pub fn stub_f2e0f4() -> ! {
    todo!("0xf2e0f4 j___ZN5boost4bindIvNS_8weak_ptrIN3RBX13ContentFilterEEESsS4_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool>(void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool)")]
// 0xf2e104 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX13ContentFilterEEESsbS4_SsbEENS_3_bi6bind_tIT_PFS7_T0_T1_T2_ENS5_9list_av_3IT3_T4_T5_E4typeEEESC_SE_SF_SG_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list_av_3<boost::weak_ptr<RBX::ContentFilter>,std::string,bool>::type> boost::bind<void,boost::weak_ptr<RBX::ContentFilter>,std::string,bool,boost::weak_ptr<RBX::ContentFilter>,std::string,bool>(void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::weak_ptr<RBX::ContentFilter>,std::string,bool)
pub fn stub_f2e104() -> ! {
    todo!("0xf2e104 j___ZN5boost4bindIvNS_8weak_ptrIN3RBX13ContentFilterEEESsbS4_SsbEENS_3_bi6bind_tIT_PFS7_T0_T1_T2_ENS5_9list_av_3IT3_T4_T5_E4typeEEESC_SE_SF_SG_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>::type> boost::bind<void,std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string>(void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::ContentFilter>,std::string)")]
// 0xf2e114 — j___ZN5boost4bindIvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsNS_3argILi1EEENS8_ILi2EEES7_SsEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_
// was: boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::ContentFilter>,std::string>::type> boost::bind<void,std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string,boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::ContentFilter>,std::string>(void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::ContentFilter>,std::string)
pub fn stub_f2e114() -> ! {
    todo!("0xf2e114 j___ZN5boost4bindIvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsNS_3argILi1EEENS8_ILi2EEES7_SsEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf2e124 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f2e124() -> ! {
    todo!("0xf2e124 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf2e134 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f2e134() -> ! {
    todo!("0xf2e134 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf2e144 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f2e144() -> ! {
    todo!("0xf2e144 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list4INS_3argILi1EEENSF_ILi2EEENS3_5valueISB_EENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ContentFilter>::weak_ptr<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&,boost::detail::sp_enable_if_convertible<RBX::ContentFilter,RBX::ContentFilter>::type)")]
// 0xf2e184 — j___ZN5boost8weak_ptrIN3RBX13ContentFilterEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::ContentFilter>::weak_ptr<RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const&,boost::detail::sp_enable_if_convertible<RBX::ContentFilter,RBX::ContentFilter>::type)
pub fn stub_f2e184() -> ! {
    todo!("0xf2e184 j___ZN5boost8weak_ptrIN3RBX13ContentFilterEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)")]
// 0xf2e1d4 — j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEEvT_
// was: void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)
pub fn stub_f2e1d4() -> ! {
    todo!("0xf2e1d4 j___ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS6_5list4INS_3argILi1EEENSF_ILi2EEENS6_5valueISB_EENSI_ISsEEEEEEEEvT_")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf2e254 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f2e254() -> ! {
    todo!("0xf2e254 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0xf2e264 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_f2e264() -> ! {
    todo!("0xf2e264 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf2e274 — j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f2e274() -> ! {
    todo!("0xf2e274 j___ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8weak_ptrIN3RBX13ContentFilterEEESsENS8_5list4INS_3argILi1EEENSH_ILi2EEENS8_5valueISD_EENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "std::map<std::string,RBX::ContentFilter::ResultEntry,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::operator[](std::string const&)")]
// 0xf2e284 — j___ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
pub fn stub_f2e284() -> ! {
    todo!("0xf2e284 j___ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::lower_bound(std::string const&)")]
// 0xf2e294 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs
pub fn stub_f2e294() -> ! {
    todo!("0xf2e294 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::upper_bound(std::string const&)")]
// 0xf2e2a4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs
pub fn stub_f2e2a4() -> ! {
    todo!("0xf2e2a4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::find(std::string const&)")]
// 0xf2e2b4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs
pub fn stub_f2e2b4() -> ! {
    todo!("0xf2e2b4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::string const&)")]
// 0xf2e2c4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs
pub fn stub_f2e2c4() -> ! {
    todo!("0xf2e2c4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>)")]
// 0xf2e2d4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE
pub fn stub_f2e2d4() -> ! {
    todo!("0xf2e2d4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>,std::_Rb_tree_iterator<std::string>)")]
// 0xf2e2e4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_
pub fn stub_f2e2e4() -> ! {
    todo!("0xf2e2e4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::lower_bound(std::string const&)")]
// 0xf2e2f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
pub fn stub_f2e2f4() -> ! {
    todo!("0xf2e2f4 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_create_node(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0xf2e304 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_f2e304() -> ! {
    todo!("0xf2e304 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0xf2e314 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_f2e314() -> ! {
    todo!("0xf2e314 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0xf2e324 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_f2e324() -> ! {
    todo!("0xf2e324 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::find(std::string const&)")]
// 0xf2e334 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_f2e334() -> ! {
    todo!("0xf2e334 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>)")]
// 0xf2e344 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
pub fn stub_f2e344() -> ! {
    todo!("0xf2e344 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ContentFilter::ResultEntry>> *)")]
// 0xf2e354 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f2e354() -> ! {
    todo!("0xf2e354 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
// 0xf2e364 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_f2e364() -> ! {
    todo!("0xf2e364 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf2e374 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_f2e374() -> ! {
    todo!("0xf2e374 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf2e384 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_f2e384() -> ! {
    todo!("0xf2e384 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::filesystem::directory_iterator::directory_iterator(boost::filesystem::path const&)")]
// 0xf2e394 — j___ZN5boost10filesystem18directory_iteratorC2ERKNS0_4pathE
pub fn stub_f2e394() -> ! {
    todo!("0xf2e394 j___ZN5boost10filesystem18directory_iteratorC2ERKNS0_4pathE")
}

#[doc(alias = "boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<char *>::type>,boost::filesystem::path&>::type boost::filesystem::path::operator=<char *>(char * const&)")]
// 0xf2e3a4 — j___ZN5boost10filesystem4pathaSIPcEENS_9enable_ifINS0_11path_traits11is_pathableINS_5decayIT_E4typeEEERS1_E4typeERKS8_
pub fn stub_f2e3a4() -> ! {
    todo!("0xf2e3a4 j___ZN5boost10filesystem4pathaSIPcEENS_9enable_ifINS0_11path_traits11is_pathableINS_5decayIT_E4typeEEERS1_E4typeERKS8_")
}

#[doc(alias = "boost::enable_if<boost::filesystem::path_traits::is_pathable<boost::decay<std::string>::type>,boost::filesystem::path&>::type boost::filesystem::path::operator=<std::string>(std::string const&)")]
// 0xf2e3b4 — j___ZN5boost10filesystem4pathaSISsEENS_9enable_ifINS0_11path_traits11is_pathableINS_5decayIT_E4typeEEERS1_E4typeERKS7_
pub fn stub_f2e3b4() -> ! {
    todo!("0xf2e3b4 j___ZN5boost10filesystem4pathaSISsEENS_9enable_ifINS0_11path_traits11is_pathableINS_5decayIT_E4typeEEERS1_E4typeERKS7_")
}

#[doc(alias = "boost::filesystem::detail::dir_itr_imp::~dir_itr_imp()")]
// 0xf2e3c4 — j___ZN5boost10filesystem6detail11dir_itr_impD2Ev
pub fn stub_f2e3c4() -> ! {
    todo!("0xf2e3c4 j___ZN5boost10filesystem6detail11dir_itr_impD2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<boost::filesystem::detail::dir_itr_imp>::shared_ptr<boost::filesystem::detail::dir_itr_imp>(boost::filesystem::detail::dir_itr_imp *)")]
// 0xf2e3d4 — j___ZN5boost10shared_ptrINS_10filesystem6detail11dir_itr_impEEC2IS3_EEPT_
// was: boost::shared_ptr<boost::filesystem::detail::dir_itr_imp>::shared_ptr<boost::filesystem::detail::dir_itr_imp>(boost::filesystem::detail::dir_itr_imp *)
pub fn stub_f2e3d4() -> ! {
    todo!("0xf2e3d4 j___ZN5boost10shared_ptrINS_10filesystem6detail11dir_itr_impEEC2IS3_EEPT_")
}

