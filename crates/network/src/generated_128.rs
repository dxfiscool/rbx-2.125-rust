//! Auto-generated skeletons for rbx-network — RakNet|RBX::Network|Replicator filtered EA-sorted asc
//! Filter: RakNet|RBX::Network|Replicator (case-insensitive) -> 4797 funcs, 2358 already stubbed (2439 remaining before batch)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x99e168..0x9b006c | existing 13390 -> 13490 total (filtered EA-sorted asc, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x99e168 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::create_buckets(unsigned long)")]
pub fn stub_99e168() -> ! {
    todo!("0x99e168 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm")
}

// 0x99e218 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEES6_SC_NSB_19SystemAddressHasherESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS7_RKT_
// type: void __fastcall(int *, const RakNet::SystemAddress *, RakNet::SystemAddress *this, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>,RakNet::SystemAddress,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::SystemAddressHasher,std::equal_to<RakNet::SystemAddress>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>(RakNet::SystemAddress const&,boost::unordered::detail::emplace_args1<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>> const&)")]
pub fn stub_99e218() -> ! {
    todo!("0x99e218 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEES6_SC_NSB_19SystemAddressHasherESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS7_RKT_")
}

// 0x99e43c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
// type: int __fastcall(int, _QWORD **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>(boost::unordered::detail::emplace_args1<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>> const&)")]
pub fn stub_99e43c() -> ! {
    todo!("0x99e43c __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_")
}

// 0x99e6f8 — __ZNSt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEC2ERS2_RKS7_
// type: int __fastcall(int, __int64 *, char *__src, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>::pair(RakNet::SystemAddress const&,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats const&)")]
pub fn stub_99e6f8() -> ! {
    todo!("0x99e6f8 __ZNSt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEC2ERS2_RKS7_")
}

// 0x99e8f8 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE
// type: int __fastcall(int, _DWORD *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, RBX::TaskScheduler::Job *, int, pthread_mutex_t *, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::StatsUpdateJob(boost::shared_ptr<RakNet::RakPeerInterface>,RBX::DataModel *)")]
pub fn stub_99e8f8() -> ! {
    todo!("0x99e8f8 __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE")
}

// 0x99ef14 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobD1Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::StatsUpdateJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::~StatsUpdateJob()")]
pub fn stub_99ef14() -> ! {
    todo!("0x99ef14 __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobD1Ev")
}

// 0x99ef20 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobD0Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::StatsUpdateJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::~StatsUpdateJob()")]
pub fn stub_99ef20() -> ! {
    todo!("0x99ef20 __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobD0Ev")
}

// 0x99efc0 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::StatsUpdateJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_99efc0() -> ! {
    todo!("0x99efc0 __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0x99efdc — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_99efdc() -> ! {
    todo!("0x99efdc __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0x99eff8 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::ConcurrentRakPeer::StatsUpdateJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_99eff8() -> ! {
    todo!("0x99eff8 __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE")
}

// 0x99f428 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob11updateStatsERSt4pairIKN6RakNet13SystemAddressENS2_15ConnectionStatsEEPNS4_16RakPeerInterfaceE
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::updateStats(std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats> &,RakNet::RakPeerInterface *)")]
pub fn stub_99f428() -> ! {
    todo!("0x99f428 __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob11updateStatsERSt4pairIKN6RakNet13SystemAddressENS2_15ConnectionStatsEEPNS4_16RakPeerInterfaceE")
}

// 0x99f5a8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEE9find_nodeERS7_
// type: int __fastcall(_DWORD *, RakNet::SystemAddress *this)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::find_node(RakNet::SystemAddress const&)const")]
pub fn stub_99f5a8() -> ! {
    todo!("0x99f5a8 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEE9find_nodeERS7_")
}

// 0x99f644 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobD2Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::StatsUpdateJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::~StatsUpdateJob()")]
pub fn stub_99f644() -> ! {
    todo!("0x99f644 __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobD2Ev")
}

// 0x99f81c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEES6_SC_NSB_19SystemAddressHasherESt8equal_toIS6_EEEED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>,RakNet::SystemAddress,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::SystemAddressHasher,std::equal_to<RakNet::SystemAddress>>>::~table()")]
pub fn stub_99f81c() -> ! {
    todo!("0x99f81c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEES6_SC_NSB_19SystemAddressHasherESt8equal_toIS6_EEEED2Ev")
}

// 0x99f938 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::~table()")]
pub fn stub_99f938() -> ! {
    todo!("0x99f938 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEED2Ev")
}

// 0x99f9b8 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE
// type: int __fastcall(int, void *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, int, int, int, int, int, RBX::TaskScheduler::Job *, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::PacketJob(boost::shared_ptr<RakNet::RakPeerInterface>,RBX::DataModel *)")]
pub fn stub_99f9b8() -> ! {
    todo!("0x99f9b8 __ZN3RBX7Network17ConcurrentRakPeer9PacketJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE")
}

// 0x99ff54 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJobD1Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::PacketJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::~PacketJob()")]
pub fn stub_99ff54() -> ! {
    todo!("0x99ff54 __ZN3RBX7Network17ConcurrentRakPeer9PacketJobD1Ev")
}

// 0x99ff60 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJobD0Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::PacketJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::~PacketJob()")]
pub fn stub_99ff60() -> ! {
    todo!("0x99ff60 __ZN3RBX7Network17ConcurrentRakPeer9PacketJobD0Ev")
}

// 0x9a0000 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: _QWORD *__fastcall(_QWORD *result, int)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9a0000() -> ! {
    todo!("0x9a0000 __ZN3RBX7Network17ConcurrentRakPeer9PacketJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0x9a0030 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::PacketJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9a0030() -> ! {
    todo!("0x9a0030 __ZN3RBX7Network17ConcurrentRakPeer9PacketJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0x9a0208 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::ConcurrentRakPeer::PacketJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_9a0208() -> ! {
    todo!("0x9a0208 __ZN3RBX7Network17ConcurrentRakPeer9PacketJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE")
}

// 0x9a07dc — __ZN3rbx22timestamped_safe_queueIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEE14pop_if_presentERS5_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "rbx::timestamped_safe_queue<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>::pop_if_present(RBX::Network::ConcurrentRakPeer::PacketJob::SendData&)")]
pub fn stub_9a07dc() -> ! {
    todo!("0x9a07dc __ZN3rbx22timestamped_safe_queueIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEE14pop_if_presentERS5_")
}

// 0x9a0a34 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJobD2Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::PacketJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::~PacketJob()")]
pub fn stub_9a0a34() -> ! {
    todo!("0x9a0a34 __ZN3RBX7Network17ConcurrentRakPeer9PacketJobD2Ev")
}

// 0x9a0be4 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::~deque()")]
pub fn stub_9a0be4() -> ! {
    todo!("0x9a0be4 __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EED2Ev")
}

// 0x9a0d88 — __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::_M_initialize_map(unsigned long)")]
pub fn stub_9a0d88() -> ! {
    todo!("0x9a0d88 __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EE17_M_initialize_mapEm")
}

// 0x9a0f6c — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EEC2ERKSA_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>> const&)")]
pub fn stub_9a0f6c() -> ! {
    todo!("0x9a0f6c __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EEC2ERKSA_")
}

// 0x9a10c4 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEERKS9_PSA_ES0_IS9_RS9_PS9_EET0_T_SI_SH_St12__false_type
// type: void __fastcall(int *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int *, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*>>(std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*>,std::__false_type)")]
pub fn stub_9a10c4() -> ! {
    todo!("0x9a10c4 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEERKS9_PSA_ES0_IS9_RS9_PS9_EET0_T_SI_SH_St12__false_type")
}

// 0x9a1930 — __ZN3RBX7Network16SenderDictionaryIPKNS_4NameEE4sendERN6RakNet9BitStreamES4_
// type: unsigned int __fastcall(int, RakNet::BitStream *this, int)
#[doc(alias = "RBX::Network::SenderDictionary<RBX::Name const*>::send(RakNet::BitStream &,RBX::Name const*)")]
pub fn stub_9a1930() -> ! {
    todo!("0x9a1930 __ZN3RBX7Network16SenderDictionaryIPKNS_4NameEE4sendERN6RakNet9BitStreamES4_")
}

// 0x9a19f8 — __ZN3RBX7Network18ReceiverDictionaryISsE10setDefaultERSs
// type: int __fastcall(std::string *)
#[doc(alias = "RBX::Network::ReceiverDictionary<std::string>::setDefault(std::string &)")]
pub fn stub_9a19f8() -> ! {
    todo!("0x9a19f8 __ZN3RBX7Network18ReceiverDictionaryISsE10setDefaultERSs")
}

// 0x9a1a0c — __ZN3RBX7Network24ReceiverStringDictionary5learnEhRKSs
// type: void __fastcall(RBX::Network::ReceiverStringDictionary *this, int, const std::string *)
#[doc(alias = "RBX::Network::ReceiverStringDictionary::learn(unsigned char,std::string const&)")]
pub fn stub_9a1a0c() -> ! {
    todo!("0x9a1a0c __ZN3RBX7Network24ReceiverStringDictionary5learnEhRKSs")
}

// 0x9a1d80 — __ZN3RBX7Network24ReceiverStringDictionary3getEhRSs
// type: int __fastcall(RBX::Network::ReceiverStringDictionary *this, int, std::string *)
#[doc(alias = "RBX::Network::ReceiverStringDictionary::get(unsigned char,std::string &)")]
pub fn stub_9a1d80() -> ! {
    todo!("0x9a1d80 __ZN3RBX7Network24ReceiverStringDictionary3getEhRSs")
}

// 0x9a2160 — __ZN3RBX7Network22SharedStringDictionary15serializeStringERKSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringDictionary *this, const std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringDictionary::serializeString(std::string const&,RakNet::BitStream &)")]
pub fn stub_9a2160() -> ! {
    todo!("0x9a2160 __ZN3RBX7Network22SharedStringDictionary15serializeStringERKSsRN6RakNet9BitStreamE")
}

// 0x9a2170 — __ZN3RBX7Network22SharedStringDictionary15serializeStringERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
// type: void __fastcall(int, _DWORD *, int)
#[doc(alias = "RBX::Network::SharedStringDictionary::serializeString(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
pub fn stub_9a2170() -> ! {
    todo!("0x9a2170 __ZN3RBX7Network22SharedStringDictionary15serializeStringERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")
}

// 0x9a2294 — __ZN3RBX7Network22SharedStringDictionary17deserializeStringERSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringDictionary *this, std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringDictionary::deserializeString(std::string &,RakNet::BitStream &)")]
pub fn stub_9a2294() -> ! {
    todo!("0x9a2294 __ZN3RBX7Network22SharedStringDictionary17deserializeStringERSsRN6RakNet9BitStreamE")
}

// 0x9a22a8 — __ZN3RBX7Network22SharedStringDictionary17deserializeStringERNS_10Reflection8PropertyERN6RakNet9BitStreamE
// type: void __fastcall(int, _DWORD *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringDictionary::deserializeString(RBX::Reflection::Property &,RakNet::BitStream &)")]
pub fn stub_9a22a8() -> ! {
    todo!("0x9a22a8 __ZN3RBX7Network22SharedStringDictionary17deserializeStringERNS_10Reflection8PropertyERN6RakNet9BitStreamE")
}

// 0x9a23d4 — __ZN3RBX7Network31SharedStringProtectedDictionaryC1Eb
// type: RBX::Network::SharedStringProtectedDictionary *__fastcall(RBX::Network::SharedStringProtectedDictionary *this, bool)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::SharedStringProtectedDictionary(bool)")]
pub fn stub_9a23d4() -> ! {
    todo!("0x9a23d4 __ZN3RBX7Network31SharedStringProtectedDictionaryC1Eb")
}

// 0x9a2514 — __ZN3RBX7Network31SharedStringProtectedDictionary15serializeStringERKSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringProtectedDictionary *this, const std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::serializeString(std::string const&,RakNet::BitStream &)")]
pub fn stub_9a2514() -> ! {
    todo!("0x9a2514 __ZN3RBX7Network31SharedStringProtectedDictionary15serializeStringERKSsRN6RakNet9BitStreamE")
}

// 0x9a2524 — __ZN3RBX7Network31SharedStringProtectedDictionary15serializeStringERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE
// type: void __fastcall(int, _DWORD *, int)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::serializeString(RBX::Reflection::ConstProperty const&,RakNet::BitStream &)")]
pub fn stub_9a2524() -> ! {
    todo!("0x9a2524 __ZN3RBX7Network31SharedStringProtectedDictionary15serializeStringERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamE")
}

// 0x9a2648 — __ZN3RBX7Network31SharedStringProtectedDictionary17deserializeStringERSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringProtectedDictionary *this, std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::deserializeString(std::string &,RakNet::BitStream &)")]
pub fn stub_9a2648() -> ! {
    todo!("0x9a2648 __ZN3RBX7Network31SharedStringProtectedDictionary17deserializeStringERSsRN6RakNet9BitStreamE")
}

// 0x9a265c — __ZN3RBX7Network31SharedStringProtectedDictionary17deserializeStringERNS_10Reflection8PropertyERN6RakNet9BitStreamE
// type: int __fastcall(int, _DWORD *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::deserializeString(RBX::Reflection::Property &,RakNet::BitStream &)")]
pub fn stub_9a265c() -> ! {
    todo!("0x9a265c __ZN3RBX7Network31SharedStringProtectedDictionary17deserializeStringERNS_10Reflection8PropertyERN6RakNet9BitStreamE")
}

// 0x9a2790 — __ZN3RBX7Network16SenderDictionaryISsE4sendERN6RakNet9BitStreamERKSs
// type: void __fastcall(int, RakNet::BitStream *, const std::string *)
#[doc(alias = "RBX::Network::SenderDictionary<std::string>::send(RakNet::BitStream &,std::string const&)")]
pub fn stub_9a2790() -> ! {
    todo!("0x9a2790 __ZN3RBX7Network16SenderDictionaryISsE4sendERN6RakNet9BitStreamERKSs")
}

// 0x9a2990 — __ZN3RBX7Network18ReceiverDictionaryISsE7receiveERN6RakNet9BitStreamERSs
// type: int __fastcall(int, RakNet::BitStream *this, std::string *)
#[doc(alias = "RBX::Network::ReceiverDictionary<std::string>::receive(RakNet::BitStream &,std::string &)")]
pub fn stub_9a2990() -> ! {
    todo!("0x9a2990 __ZN3RBX7Network18ReceiverDictionaryISsE7receiveERN6RakNet9BitStreamERSs")
}

// 0x9a29f4 — __ZN3RBX7Network24ReceiverStringDictionary7receiveISsEEbRN6RakNet9BitStreamERT_
// type: int __fastcall(RBX::Network::ReceiverStringDictionary *, RakNet::BitStream *this, std::string *)
#[doc(alias = "bool RBX::Network::ReceiverStringDictionary::receive<std::string>(RakNet::BitStream &,std::string &)")]
pub fn stub_9a29f4() -> ! {
    todo!("0x9a29f4 __ZN3RBX7Network24ReceiverStringDictionary7receiveISsEEbRN6RakNet9BitStreamERT_")
}

// 0x9a3918 — __ZN3RBX7Network21DirectPhysicsReceiver13receivePacketERN6RakNet9BitStreamEyPNS0_15ReplicatorStats20PhysicsReceiverStatsE
// type: void __fastcall(struct _Unwind_Exception *, RakNet::BitStream *, unsigned __int64, int)
#[doc(alias = "RBX::Network::DirectPhysicsReceiver::receivePacket(RakNet::BitStream &,unsigned long long,RBX::Network::ReplicatorStats::PhysicsReceiverStats *)")]
pub fn stub_9a3918() -> ! {
    todo!("0x9a3918 __ZN3RBX7Network21DirectPhysicsReceiver13receivePacketERN6RakNet9BitStreamEyPNS0_15ReplicatorStats20PhysicsReceiverStatsE")
}

// 0x9a4660 — __ZN3RBX7Network22ErrorCompPhysicsSenderC1ERNS0_10ReplicatorE
// type: int __fastcall(RBX::Network::ErrorCompPhysicsSender *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::ErrorCompPhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_9a4660() -> ! {
    todo!("0x9a4660 __ZN3RBX7Network22ErrorCompPhysicsSenderC1ERNS0_10ReplicatorE")
}

// 0x9a466c — __ZN3RBX7Network22ErrorCompPhysicsSenderC2ERNS0_10ReplicatorE
// type: RBX::Network::PhysicsSender *__fastcall(RBX::Network::ErrorCompPhysicsSender *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::ErrorCompPhysicsSender(RBX::Network::Replicator &)")]
pub fn stub_9a466c() -> ! {
    todo!("0x9a466c __ZN3RBX7Network22ErrorCompPhysicsSenderC2ERNS0_10ReplicatorE")
}

// 0x9a4d78 — __ZN3RBX7Network22ErrorCompPhysicsSenderD0Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::~ErrorCompPhysicsSender()")]
pub fn stub_9a4d78() -> ! {
    todo!("0x9a4d78 __ZN3RBX7Network22ErrorCompPhysicsSenderD0Ev")
}

// 0x9a4e18 — __ZN3RBX7Network22ErrorCompPhysicsSenderD1Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::~ErrorCompPhysicsSender()")]
pub fn stub_9a4e18() -> ! {
    todo!("0x9a4e18 __ZN3RBX7Network22ErrorCompPhysicsSenderD1Ev")
}

// 0x9a4e24 — __ZN3RBX7Network22ErrorCompPhysicsSenderD2Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender *__hidden this)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::~ErrorCompPhysicsSender()")]
pub fn stub_9a4e24() -> ! {
    todo!("0x9a4e24 __ZN3RBX7Network22ErrorCompPhysicsSenderD2Ev")
}

// 0x9a5008 — __ZN3RBX7Network22ErrorCompPhysicsSender4stepEv
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender *this, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::step(void)")]
pub fn stub_9a5008() -> ! {
    todo!("0x9a5008 __ZN3RBX7Network22ErrorCompPhysicsSender4stepEv")
}

// 0x9a5d8c — __ZN3RBX7Network22ErrorCompPhysicsSender9addNuggetERNS_12PartInstanceE
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender *this, RBX::PartInstance *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::addNugget(RBX::PartInstance &)")]
pub fn stub_9a5d8c() -> ! {
    todo!("0x9a5d8c __ZN3RBX7Network22ErrorCompPhysicsSender9addNuggetERNS_12PartInstanceE")
}

// 0x9a5fa0 — __ZN3RBX7Network22ErrorCompPhysicsSender16onAddingAssemblyEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, _DWORD *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::onAddingAssembly(boost::shared_ptr<RBX::Instance>)")]
pub fn stub_9a5fa0() -> ! {
    todo!("0x9a5fa0 __ZN3RBX7Network22ErrorCompPhysicsSender16onAddingAssemblyEN5boost10shared_ptrINS_8InstanceEEE")
}

// 0x9a629c — __ZN3RBX7Network22ErrorCompPhysicsSender10addNugget2EN5boost10shared_ptrINS_12PartInstanceEEE
// type: void __fastcall(int, pthread_mutex_t **)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::addNugget2(boost::shared_ptr<RBX::PartInstance>)")]
pub fn stub_9a629c() -> ! {
    todo!("0x9a629c __ZN3RBX7Network22ErrorCompPhysicsSender10addNugget2EN5boost10shared_ptrINS_12PartInstanceEEE")
}

// 0x9a74a4 — __ZN3RBX7Network22ErrorCompPhysicsSender12removeNuggetEN5boost10shared_ptrIKNS_12PartInstanceEEE
// type: int __fastcall(_DWORD *, unsigned int *, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::removeNugget(boost::shared_ptr<RBX::PartInstance const>)")]
pub fn stub_9a74a4() -> ! {
    todo!("0x9a74a4 __ZN3RBX7Network22ErrorCompPhysicsSender12removeNuggetEN5boost10shared_ptrIKNS_12PartInstanceEEE")
}

// 0x9a75f0 — __ZN3RBX7Network22ErrorCompPhysicsSender6Nugget12computeErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
// type: bool __fastcall(RBX::Network::ErrorCompPhysicsSender::Nugget *this, const G3D::CoordinateFrame *, const RBX::ModelInstance *, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::Nugget::computeError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
pub fn stub_9a75f0() -> ! {
    todo!("0x9a75f0 __ZN3RBX7Network22ErrorCompPhysicsSender6Nugget12computeErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi")
}

// 0x9a7894 — __ZN3RBX7Network22ErrorCompPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
pub fn stub_9a7894() -> ! {
    todo!("0x9a7894 __ZN3RBX7Network22ErrorCompPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE")
}

// 0x9a88ec — __ZN3RBX7Network22ErrorCompPhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender *this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::writeAssembly(RakNet::BitStream &,RBX::Assembly const*)")]
pub fn stub_9a88ec() -> ! {
    todo!("0x9a88ec __ZN3RBX7Network22ErrorCompPhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE")
}

// 0x9a8bd8 — __ZN3RBX11shared_fromINS_7Network18PhysicsPacketCacheEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
#[doc(alias = "boost::shared_ptr<RBX::Network::PhysicsPacketCache> RBX::shared_from<RBX::Network::PhysicsPacketCache>(RBX::Network::PhysicsPacketCache*)")]
pub fn stub_9a8bd8() -> ! {
    todo!("0x9a8bd8 __ZN3RBX11shared_fromINS_7Network18PhysicsPacketCacheEEEN5boost10shared_ptrIT_EEPS5_")
}

// 0x9a8e70 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network22ErrorCompPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
// type: int __fastcall(_DWORD *, void *, void *, char *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>)")]
pub fn stub_9a8e70() -> ! {
    todo!("0x9a8e70 __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network22ErrorCompPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_")
}

// 0x9a9048 — __ZN5boost10shared_ptrIN6RakNet9BitStreamEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "boost::shared_ptr<RakNet::BitStream>::reset(void)")]
pub fn stub_9a9048() -> ! {
    todo!("0x9a9048 __ZN5boost10shared_ptrIN6RakNet9BitStreamEE5resetEv")
}

// 0x9a90ec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *)")]
pub fn stub_9a90ec() -> ! {
    todo!("0x9a90ec __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_")
}

// 0x9a92c4 — __ZN5boost9intrusive17rbtree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE12insert_equalINS0_6detail16key_nodeptr_compISt7greaterIN3RBX7Network22ErrorCompPhysicsSender6NuggetEENS0_11rbtree_implINS0_6setoptINS7_16base_hook_traitsISD_S4_LNS0_14link_mode_typeE0ESD_Li3EEESE_mLb1EEEEEEEEEPNS0_11rbtree_nodeIS3_EERKSP_SR_SR_T_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "boost::intrusive::rbtree_node<void *> * boost::intrusive::rbtree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_equal<boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>>(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>)")]
pub fn stub_9a92c4() -> ! {
    todo!("0x9a92c4 __ZN5boost9intrusive17rbtree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE12insert_equalINS0_6detail16key_nodeptr_compISt7greaterIN3RBX7Network22ErrorCompPhysicsSender6NuggetEENS0_11rbtree_implINS0_6setoptINS7_16base_hook_traitsISD_S4_LNS0_14link_mode_typeE0ESD_Li3EEESE_mLb1EEEEEEEEEPNS0_11rbtree_nodeIS3_EERKSP_SR_SR_T_")
}

// 0x9a9474 — __ZN5boost9intrusive6detail15tree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE18insert_equal_checkINS1_16key_nodeptr_compISt7greaterIN3RBX7Network22ErrorCompPhysicsSender6NuggetEENS0_11rbtree_implINS0_6setoptINS1_16base_hook_traitsISD_S5_LNS0_14link_mode_typeE0ESD_Li3EEESE_mLb1EEEEEEEEEvRKPNS0_11rbtree_nodeIS4_EESR_SR_T_RNS6_18insert_commit_dataEPm
// type: double **__fastcall(double **result, double **, int, int, int, double ***)
#[doc(alias = "void boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_equal_check<boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>>(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>,boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_commit_data &,unsigned long *)")]
pub fn stub_9a9474() -> ! {
    todo!("0x9a9474 __ZN5boost9intrusive6detail15tree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE18insert_equal_checkINS1_16key_nodeptr_compISt7greaterIN3RBX7Network22ErrorCompPhysicsSender6NuggetEENS0_11rbtree_implINS0_6setoptINS1_16base_hook_traitsISD_S5_LNS0_14link_mode_typeE0ESD_Li3EEESE_mLb1EEEEEEEEEvRKPNS0_11rbtree_nodeIS4_EESR_SR_T_RNS6_18insert_commit_dataEPm")
}

// 0x9a9604 — __ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2INS1_IS3_EES9_EERKS_IT_T0_E
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>(std::pair const&<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>)")]
pub fn stub_9a9604() -> ! {
    todo!("0x9a9604 __ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2INS1_IS3_EES9_EERKS_IT_T0_E")
}

// 0x9a97b8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_
// type: void __fastcall(_DWORD *, _DWORD *, unsigned int *, int, char, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")]
pub fn stub_9a97b8() -> ! {
    todo!("0x9a97b8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_")
}

// 0x9a99c0 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")]
pub fn stub_9a99c0() -> ! {
    todo!("0x9a99c0 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_")
}

// 0x9a9b00 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_9a9b00() -> ! {
    todo!("0x9a9b00 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm")
}

// 0x9a9ca8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_9a9ca8() -> ! {
    todo!("0x9a9ca8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm")
}

// 0x9a9d58 — __ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKSA_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget> const&)")]
pub fn stub_9a9d58() -> ! {
    todo!("0x9a9d58 __ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKSA_")
}

// 0x9a9f14 — __ZNSt4pairIN5boost10shared_ptrIN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKS4_RKS7_
// type: int __fastcall(int, _DWORD *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(boost::shared_ptr<RBX::PartInstance> const&,RBX::Network::ErrorCompPhysicsSender::Nugget const&)")]
pub fn stub_9a9f14() -> ! {
    todo!("0x9a9f14 __ZNSt4pairIN5boost10shared_ptrIN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKS4_RKS7_")
}

// 0x9aa9f4 — __ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const")]
pub fn stub_9aa9f4() -> ! {
    todo!("0x9aa9f4 __ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_")
}

// 0x9aac70 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network22ErrorCompPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_9aac70() -> ! {
    todo!("0x9aac70 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network22ErrorCompPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev")
}

// 0x9aaccc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network22ErrorCompPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_9aaccc() -> ! {
    todo!("0x9aaccc __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network22ErrorCompPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev")
}

// 0x9aadd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network22ErrorCompPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
pub fn stub_9aadd8() -> ! {
    todo!("0x9aadd8 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network22ErrorCompPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")
}

// 0x9aaef4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network22ErrorCompPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
pub fn stub_9aaef4() -> ! {
    todo!("0x9aaef4 __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network22ErrorCompPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")
}

// 0x9ab160 — __ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::Instance>)const")]
pub fn stub_9ab160() -> ! {
    todo!("0x9ab160 __ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_")
}

// 0x9ab4a0 — __ZNK3RBX15ServiceProvider4findINS_7Network18PhysicsPacketCacheEEEPT_v
// type: __guard *__fastcall(_DWORD *, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache * RBX::ServiceProvider::find<RBX::Network::PhysicsPacketCache>(void)const")]
pub fn stub_9ab4a0() -> ! {
    todo!("0x9ab4a0 __ZNK3RBX15ServiceProvider4findINS_7Network18PhysicsPacketCacheEEEPT_v")
}

// 0x9abb18 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network18PhysicsPacketCacheEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::PhysicsPacketCache>(void)")]
pub fn stub_9abb18() -> ! {
    todo!("0x9abb18 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network18PhysicsPacketCacheEEEvv")
}

// 0x9abbe0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()")]
pub fn stub_9abbe0() -> ! {
    todo!("0x9abbe0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev")
}

// 0x9ac3ec — __ZN3RBX7Network19GuidRegistryServiceC1Ev
// type: int __fastcall(RBX::Network::GuidRegistryService *this)
#[doc(alias = "RBX::Network::GuidRegistryService::GuidRegistryService(void)")]
pub fn stub_9ac3ec() -> ! {
    todo!("0x9ac3ec __ZN3RBX7Network19GuidRegistryServiceC1Ev")
}

// 0x9ac3f8 — __ZN3RBX7Network19GuidRegistryServiceC2Ev
// type: RBX::Network::GuidRegistryService *__fastcall(RBX::Network::GuidRegistryService *this)
#[doc(alias = "RBX::Network::GuidRegistryService::GuidRegistryService(void)")]
pub fn stub_9ac3f8() -> ! {
    todo!("0x9ac3f8 __ZN3RBX7Network19GuidRegistryServiceC2Ev")
}

// 0x9ac6cc — __ZN3RBX7Network19GuidRegistryServiceD0Ev
// type: void __fastcall(RBX::Network::GuidRegistryService *__hidden this)
#[doc(alias = "RBX::Network::GuidRegistryService::~GuidRegistryService()")]
pub fn stub_9ac6cc() -> ! {
    todo!("0x9ac6cc __ZN3RBX7Network19GuidRegistryServiceD0Ev")
}

// 0x9ac7c0 — __ZN3RBX7Network19GuidRegistryServiceD1Ev
// type: void __fastcall(RBX::Network::GuidRegistryService *__hidden this)
#[doc(alias = "RBX::Network::GuidRegistryService::~GuidRegistryService()")]
pub fn stub_9ac7c0() -> ! {
    todo!("0x9ac7c0 __ZN3RBX7Network19GuidRegistryServiceD1Ev")
}

// 0x9ac8a4 — __ZThn32_N3RBX7Network19GuidRegistryServiceD0Ev
// type: void __fastcall(RBX::Network::GuidRegistryService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::GuidRegistryService::~GuidRegistryService()")]
pub fn stub_9ac8a4() -> ! {
    todo!("0x9ac8a4 __ZThn32_N3RBX7Network19GuidRegistryServiceD0Ev")
}

// 0x9ac99c — __ZThn36_N3RBX7Network19GuidRegistryServiceD0Ev
// type: void __fastcall(RBX::Network::GuidRegistryService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::GuidRegistryService::~GuidRegistryService()")]
pub fn stub_9ac99c() -> ! {
    todo!("0x9ac99c __ZThn36_N3RBX7Network19GuidRegistryServiceD0Ev")
}

// 0x9aca94 — __ZThn32_N3RBX7Network19GuidRegistryServiceD1Ev
// type: void __fastcall(RBX::Network::GuidRegistryService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::GuidRegistryService::~GuidRegistryService()")]
pub fn stub_9aca94() -> ! {
    todo!("0x9aca94 __ZThn32_N3RBX7Network19GuidRegistryServiceD1Ev")
}

// 0x9acb74 — __ZThn36_N3RBX7Network19GuidRegistryServiceD1Ev
// type: void __fastcall(RBX::Network::GuidRegistryService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::GuidRegistryService::~GuidRegistryService()")]
pub fn stub_9acb74() -> ! {
    todo!("0x9acb74 __ZThn36_N3RBX7Network19GuidRegistryServiceD1Ev")
}

// 0x9add90 — __ZN3RBX7Network4Item13writeItemTypeERN6RakNet9BitStreamENS1_8ItemTypeE
// type: unsigned int __fastcall(RakNet::BitStream *this, int)
#[doc(alias = "RBX::Network::Item::writeItemType(RakNet::BitStream &,RBX::Network::Item::ItemType)")]
pub fn stub_9add90() -> ! {
    todo!("0x9add90 __ZN3RBX7Network4Item13writeItemTypeERN6RakNet9BitStreamENS1_8ItemTypeE")
}

// 0x9addcc — __ZN3RBX7Network4Item12readItemTypeERN6RakNet9BitStreamERNS1_8ItemTypeE
// type: int __fastcall(RakNet::BitStream *this, unsigned __int8 *)
#[doc(alias = "RBX::Network::Item::readItemType(RakNet::BitStream &,RBX::Network::Item::ItemType &)")]
pub fn stub_9addcc() -> ! {
    todo!("0x9addcc __ZN3RBX7Network4Item12readItemTypeERN6RakNet9BitStreamERNS1_8ItemTypeE")
}

// 0x9addf8 — __ZN3RBX7Network9ItemQueueC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RBX::Network::ItemQueue::ItemQueue(void)")]
pub fn stub_9addf8() -> ! {
    todo!("0x9addf8 __ZN3RBX7Network9ItemQueueC1Ev")
}

// 0x9ade08 — __ZN3RBX7Network9ItemQueueD1Ev
// type: void __fastcall(RBX::Network::ItemQueue *__hidden this)
#[doc(alias = "RBX::Network::ItemQueue::~ItemQueue()")]
pub fn stub_9ade08() -> ! {
    todo!("0x9ade08 __ZN3RBX7Network9ItemQueueD1Ev")
}

// 0x9adf3c — __ZNK3RBX7Network9ItemQueue5emptyEv
// type: bool __fastcall(RBX::Network::ItemQueue *this)
#[doc(alias = "RBX::Network::ItemQueue::empty(void)const")]
pub fn stub_9adf3c() -> ! {
    todo!("0x9adf3c __ZNK3RBX7Network9ItemQueue5emptyEv")
}

// 0x9adf58 — __ZNK3RBX7Network9ItemQueue4sizeEv
// type: int __fastcall(RBX::Network::ItemQueue *this)
#[doc(alias = "RBX::Network::ItemQueue::size(void)const")]
pub fn stub_9adf58() -> ! {
    todo!("0x9adf58 __ZNK3RBX7Network9ItemQueue4sizeEv")
}

// 0x9adf5c — __ZNK3RBX7Network9ItemQueue9head_waitEv
// type: double *__fastcall(RBX::Network::ItemQueue *this, int)
#[doc(alias = "RBX::Network::ItemQueue::head_wait(void)const")]
pub fn stub_9adf5c() -> ! {
    todo!("0x9adf5c __ZNK3RBX7Network9ItemQueue9head_waitEv")
}

// 0x9adf98 — __ZN3RBX7Network9ItemQueue9deleteAllEv
// type: int __fastcall(RBX::Network::ItemQueue *this)
#[doc(alias = "RBX::Network::ItemQueue::deleteAll(void)")]
pub fn stub_9adf98() -> ! {
    todo!("0x9adf98 __ZN3RBX7Network9ItemQueue9deleteAllEv")
}

// 0x9adfc8 — __ZN3RBX7Network9ItemQueue14pop_if_presentERPNS0_4ItemE
// type: int __fastcall(_DWORD *, _DWORD *, _DWORD *)
#[doc(alias = "RBX::Network::ItemQueue::pop_if_present(RBX::Network::Item *&)")]
pub fn stub_9adfc8() -> ! {
    todo!("0x9adfc8 __ZN3RBX7Network9ItemQueue14pop_if_presentERPNS0_4ItemE")
}

// 0x9ae0f0 — __ZN3RBX7Network9ItemQueue9push_backEPNS0_4ItemE
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "RBX::Network::ItemQueue::push_back(RBX::Network::Item *)")]
pub fn stub_9ae0f0() -> ! {
    todo!("0x9ae0f0 __ZN3RBX7Network9ItemQueue9push_backEPNS0_4ItemE")
}

// 0x9ae1bc — __ZN3RBX7Network9ItemQueue10push_frontEPNS0_4ItemE
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "RBX::Network::ItemQueue::push_front(RBX::Network::Item *)")]
pub fn stub_9ae1bc() -> ! {
    todo!("0x9ae1bc __ZN3RBX7Network9ItemQueue10push_frontEPNS0_4ItemE")
}

// 0x9b0060 — __ZN3RBX7Network15NetworkOwnerJobC1EN5boost10shared_ptrINS_9DataModelEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Network::NetworkOwnerJob::NetworkOwnerJob(boost::shared_ptr<RBX::DataModel>)")]
pub fn stub_9b0060() -> ! {
    todo!("0x9b0060 __ZN3RBX7Network15NetworkOwnerJobC1EN5boost10shared_ptrINS_9DataModelEEE")
}

// 0x9b006c — __ZN3RBX7Network15NetworkOwnerJobC2EN5boost10shared_ptrINS_9DataModelEEE
// type: int __fastcall(int, __int32 *, int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::TaskScheduler::Job *, int, int, int, int)
#[doc(alias = "RBX::Network::NetworkOwnerJob::NetworkOwnerJob(boost::shared_ptr<RBX::DataModel>)")]
pub fn stub_9b006c() -> ! {
    todo!("0x9b006c __ZN3RBX7Network15NetworkOwnerJobC2EN5boost10shared_ptrINS_9DataModelEEE")
}
