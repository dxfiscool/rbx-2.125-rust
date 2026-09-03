//! core shard ox — 100 core stubs EA-sorted, 0xadb040..0xb0d0b4 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered, global-deduped).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list_av_2<RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::type> boost::bind<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>,RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>(void (RBX::Network::InterpolatingPhysicsReceiver::*)(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>),RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
// 0xadb040 — __ZN5boost4bindIvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS3_EEPS3_S5_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_
// type: void __fastcall(int, int, pthread_mutex_t *, int, int *)
pub fn stub_0xadb040() -> ! {
    todo!("0xadb040 __ZN5boost4bindIvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS3_EEPS3_S5_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> *,RBX::Network::InterpolatingPhysicsReceiver::Job *,boost::detail::shared_count &)")]
// 0xadcab0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xadcab0() -> ! {
    todo!("0xadcab0 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> const*,RBX::Network::InterpolatingPhysicsReceiver::Job *)const")]
// 0xadcc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xadcc60() -> ! {
    todo!("0xadcc60 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
// 0xadcf0c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED1Ev
// type: void()
pub fn stub_0xadcf0c() -> ! {
    todo!("0xadcf0c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
// 0xadcf10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xadcf10() -> ! {
    todo!("0xadcf10 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::dispose(void)")]
// 0xadcf1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xadcf1c() -> ! {
    todo!("0xadcf1c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_deleter(std::type_info const&)")]
// 0xadcf30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xadcf30() -> ! {
    todo!("0xadcf30 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_untyped_deleter(void)")]
// 0xadcf34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xadcf34() -> ! {
    todo!("0xadcf34 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::operator()(RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)const")]
// 0xadd37c — __ZNK5boost4_mfi3mf1IvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS4_EEEclEPS4_S6_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xadd37c() -> ! {
    todo!("0xadd37c __ZNK5boost4_mfi3mf1IvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS4_EEEclEPS4_S6_")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>::list2(boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>)")]
// 0xadd834 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX7Network28InterpolatingPhysicsReceiverEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: _DWORD *__fastcall(_DWORD *, int, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xadd834() -> ! {
    todo!("0xadd834 __ZN5boost3_bi5list2INS0_5valueIPN3RBX7Network28InterpolatingPhysicsReceiverEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History> *,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History *,boost::detail::shared_count &)")]
// 0xaddea4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int, int, int, void *, int)
pub fn stub_0xaddea4() -> ! {
    todo!("0xaddea4 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
// 0xade088 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED1Ev
// type: void()
pub fn stub_0xade088() -> ! {
    todo!("0xade088 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
// 0xade08c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xade08c() -> ! {
    todo!("0xade08c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::dispose(void)")]
// 0xade098 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE7disposeEv
// type: void __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xade098() -> ! {
    todo!("0xade098 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_deleter(std::type_info const&)")]
// 0xade180 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xade180() -> ! {
    todo!("0xade180 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_untyped_deleter(void)")]
// 0xade184 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xade184() -> ! {
    todo!("0xade184 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade188 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD1Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_0xade188() -> ! {
    todo!("0xade188 __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD1Ev")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade194 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD0Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_0xade194() -> ! {
    todo!("0xade194 __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD0Ev")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xade234 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xade234() -> ! {
    todo!("0xade234 __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xade250 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_0xade250() -> ! {
    todo!("0xade250 __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade4b4 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD2Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_0xade4b4() -> ! {
    todo!("0xade4b4 __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD2Ev")
}

#[doc(alias = "RBX::Network::ReplicatorJob::~ReplicatorJob()")]
// 0xade658 — __ZN3RBX7Network13ReplicatorJobD0Ev
// type: void __fastcall(RBX::Network::ReplicatorJob *__hidden this)
pub fn stub_0xade658() -> ! {
    todo!("0xade658 __ZN3RBX7Network13ReplicatorJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::sendMarker(void)")]
// 0xaded58 — __ZN3RBX7Network10Replicator10sendMarkerEv
// type: void __fastcall(RBX::Network::Replicator *this, _DWORD *)
pub fn stub_0xaded58() -> ! {
    todo!("0xaded58 __ZN3RBX7Network10Replicator10sendMarkerEv")
}

#[doc(alias = "RBX::Network::Replicator::closeConnection(void)")]
// 0xadf958 — __ZN3RBX7Network10Replicator15closeConnectionEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xadf958() -> ! {
    todo!("0xadf958 __ZN3RBX7Network10Replicator15closeConnectionEv")
}

#[doc(alias = "RBX::Network::Replicator::getPlayer(void)")]
// 0xadfa08 — __ZN3RBX7Network10Replicator9getPlayerEv
// type: void __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xadfa08() -> ! {
    todo!("0xadfa08 __ZN3RBX7Network10Replicator9getPlayerEv")
}

#[doc(alias = "RBX::Network::Replicator::getRakStatsString(int)")]
// 0xadfc3c — __ZN3RBX7Network10Replicator17getRakStatsStringEi
// type: int __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xadfc3c() -> ! {
    todo!("0xadfc3c __ZN3RBX7Network10Replicator17getRakStatsStringEi")
}

#[doc(alias = "RBX::Network::Replicator::disableProcessPackets(void)")]
// 0xadfc9c — __ZN3RBX7Network10Replicator21disableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xadfc9c() -> ! {
    todo!("0xadfc9c __ZN3RBX7Network10Replicator21disableProcessPacketsEv")
}

#[doc(alias = "RBX::Network::Replicator::enableProcessPackets(void)")]
// 0xadfca8 — __ZN3RBX7Network10Replicator20enableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator::ProcessPacketsJob **this)
pub fn stub_0xadfca8() -> ! {
    todo!("0xadfca8 __ZN3RBX7Network10Replicator20enableProcessPacketsEv")
}

#[doc(alias = "RBX::Network::Replicator::getPort(void)const")]
// 0xadfcb8 — __ZNK3RBX7Network10Replicator7getPortEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xadfcb8() -> ! {
    todo!("0xadfcb8 __ZNK3RBX7Network10Replicator7getPortEv")
}

#[doc(alias = "RBX::Network::Replicator::getIpAddress(void)const")]
// 0xadfcc8 — __ZNK3RBX7Network10Replicator12getIpAddressEv
// type: int __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xadfcc8() -> ! {
    todo!("0xadfcc8 __ZNK3RBX7Network10Replicator12getIpAddressEv")
}

#[doc(alias = "RBX::Network::Replicator::getDefault(RBX::Name const&)")]
// 0xae0594 — __ZN3RBX7Network10Replicator10getDefaultERKNS_4NameE
// type: int __fastcall(RBX::Network::Replicator *this, const char **)
pub fn stub_0xae0594() -> ! {
    todo!("0xae0594 __ZN3RBX7Network10Replicator10getDefaultERKNS_4NameE")
}

#[doc(alias = "RBX::Network::ReplicatorJob::canSendPacket(rbx_core::SharedPtr<RBX::Network::Replicator> &,PacketPriority)")]
// 0xae1000 — __ZN3RBX7Network13ReplicatorJob13canSendPacketERN5boost10shared_ptrINS0_10ReplicatorEEE14PacketPriority
// type: bool __fastcall(int *, int)
pub fn stub_0xae1000() -> ! {
    todo!("0xae1000 __ZN3RBX7Network13ReplicatorJob13canSendPacketERN5boost10shared_ptrINS0_10ReplicatorEEE14PacketPriority")
}

#[doc(alias = "RBX::Network::Replicator::getBufferCountAvailable(int,PacketPriority)")]
// 0xae1058 — __ZN3RBX7Network10Replicator23getBufferCountAvailableEi14PacketPriority
// type: int __fastcall(int, int, int)
pub fn stub_0xae1058() -> ! {
    todo!("0xae1058 __ZN3RBX7Network10Replicator23getBufferCountAvailableEi14PacketPriority")
}

#[doc(alias = "RBX::Network::Replicator::onStatisticsChanged(RBX::Network::ConcurrentRakPeerStats const&)")]
// 0xae1f7c — __ZN3RBX7Network10Replicator19onStatisticsChangedERKNS0_22ConcurrentRakPeerStatsE
// type: void *__fastcall(int, const void *)
pub fn stub_0xae1f7c() -> ! {
    todo!("0xae1f7c __ZN3RBX7Network10Replicator19onStatisticsChangedERKNS0_22ConcurrentRakPeerStatsE")
}

#[doc(alias = "RBX::Network::Replicator::createPhysicsReceiver(RBX::NetworkSettings::PhysicsReceiveMethod,bool)")]
// 0xae22e8 — __ZN3RBX7Network10Replicator21createPhysicsReceiverENS_15NetworkSettings20PhysicsReceiveMethodEb
// type: void __fastcall(_DWORD *, int, char, int, int, int, int, int, int, int, int, void *, void *, int, int, int, int, int)
pub fn stub_0xae22e8() -> ! {
    todo!("0xae22e8 __ZN3RBX7Network10Replicator21createPhysicsReceiverENS_15NetworkSettings20PhysicsReceiveMethodEb")
}

#[doc(alias = "RBX::Network::Replicator::clearIncomingPackets(void)")]
// 0xae2948 — __ZN3RBX7Network10Replicator20clearIncomingPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae2948() -> ! {
    todo!("0xae2948 __ZN3RBX7Network10Replicator20clearIncomingPacketsEv")
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae29b8 — __ZN3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_0xae29b8() -> ! {
    todo!("0xae29b8 __ZN3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae2a58 — __ZN3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_0xae2a58() -> ! {
    todo!("0xae2a58 __ZN3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2a64 — __ZThn32_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_0xae2a64() -> ! {
    todo!("0xae2a64 __ZThn32_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2b08 — __ZThn36_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_0xae2b08() -> ! {
    todo!("0xae2b08 __ZThn36_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2bac — __ZThn1180_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_0xae2bac() -> ! {
    todo!("0xae2bac __ZThn1180_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2c50 — __ZThn1192_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_0xae2c50() -> ! {
    todo!("0xae2c50 __ZThn1192_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae2cf4 — __ZN3RBX7Network10ReplicatorD2Ev
// type: void __fastcall(struct _Unwind_Exception *lpuexcpt, int, int)
pub fn stub_0xae2cf4() -> ! {
    todo!("0xae2cf4 __ZN3RBX7Network10ReplicatorD2Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3aa8 — __ZThn32_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this, int, int)
pub fn stub_0xae3aa8() -> ! {
    todo!("0xae3aa8 __ZThn32_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ab4 — __ZThn36_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae3ab4() -> ! {
    todo!("0xae3ab4 __ZThn36_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ac0 — __ZThn1180_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae3ac0() -> ! {
    todo!("0xae3ac0 __ZThn1180_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ad0 — __ZThn1192_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae3ad0() -> ! {
    todo!("0xae3ad0 __ZThn1192_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::closeReplicationItem(RBX::Network::Replicator::ReplicationData &)")]
// 0xae5f20 — __ZN3RBX7Network10Replicator20closeReplicationItemERNS1_15ReplicationDataE
// type: int __fastcall(int)
pub fn stub_0xae5f20() -> ! {
    todo!("0xae5f20 __ZN3RBX7Network10Replicator20closeReplicationItemERNS1_15ReplicationDataE")
}

#[doc(alias = "RBX::Network::Replicator::physicsSenderStats(void)")]
// 0xae5f44 — __ZN3RBX7Network10Replicator18physicsSenderStatsEv
// type: char *__fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae5f44() -> ! {
    todo!("0xae5f44 __ZN3RBX7Network10Replicator18physicsSenderStatsEv")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xae5f4c — __ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_0xae5f4c() -> ! {
    todo!("0xae5f4c __ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::SendClusterJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xae603c — __ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_0xae603c() -> ! {
    todo!("0xae603c __ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::getAdjustedMtuSize(void)const")]
// 0xae6238 — __ZNK3RBX7Network10Replicator18getAdjustedMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae6238() -> ! {
    todo!("0xae6238 __ZNK3RBX7Network10Replicator18getAdjustedMtuSizeEv")
}

#[doc(alias = "RBX::Network::Replicator::clusterOutStep(void)")]
// 0xae62ac — __ZN3RBX7Network10Replicator14clusterOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae62ac() -> ! {
    todo!("0xae62ac __ZN3RBX7Network10Replicator14clusterOutStepEv")
}

#[doc(alias = "RBX::Network::Replicator::requestDisconnect(void)")]
// 0xae6410 — __ZN3RBX7Network10Replicator17requestDisconnectEv
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Instance *)
pub fn stub_0xae6410() -> ! {
    todo!("0xae6410 __ZN3RBX7Network10Replicator17requestDisconnectEv")
}

#[doc(alias = "RBX::Network::Replicator::dataOutStep(void)")]
// 0xae6848 — __ZN3RBX7Network10Replicator11dataOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae6848() -> ! {
    todo!("0xae6848 __ZN3RBX7Network10Replicator11dataOutStepEv")
}

#[doc(alias = "RBX::Network::Replicator::isInStreamedRegions(RBX::Extents const&)")]
// 0xae6f38 — __ZN3RBX7Network10Replicator19isInStreamedRegionsERKNS_7ExtentsE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Extents *)
pub fn stub_0xae6f38() -> ! {
    todo!("0xae6f38 __ZN3RBX7Network10Replicator19isInStreamedRegionsERKNS_7ExtentsE")
}

#[doc(alias = "RBX::Network::Replicator::updateStatsItem(RBX::Stats::StatsService *)")]
// 0xaebf24 — __ZN3RBX7Network10Replicator15updateStatsItemEPNS_5Stats12StatsServiceE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Stats::StatsService *)
pub fn stub_0xaebf24() -> ! {
    todo!("0xaebf24 __ZN3RBX7Network10Replicator15updateStatsItemEPNS_5Stats12StatsServiceE")
}

#[doc(alias = "RBX::Network::Replicator::createPhysicsSender(RBX::NetworkSettings::PhysicsSendMethod)")]
// 0xaece78 — __ZN3RBX7Network10Replicator19createPhysicsSenderENS_15NetworkSettings17PhysicsSendMethodE
// type: void __fastcall(_DWORD *, int, int, int)
pub fn stub_0xaece78() -> ! {
    todo!("0xaece78 __ZN3RBX7Network10Replicator19createPhysicsSenderENS_15NetworkSettings17PhysicsSendMethodE")
}

#[doc(alias = "RBX::Network::Replicator::incomingPacketsCount(void)const")]
// 0xaed8a8 — __ZNK3RBX7Network10Replicator20incomingPacketsCountEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xaed8a8() -> ! {
    todo!("0xaed8a8 __ZNK3RBX7Network10Replicator20incomingPacketsCountEv")
}

#[doc(alias = "RBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xaf7c18 — __ZN3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
pub fn stub_0xaf7c18() -> ! {
    todo!("0xaf7c18 __ZN3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xaf7ce0 — __ZThn1196_N3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
pub fn stub_0xaf7ce0() -> ! {
    todo!("0xaf7ce0 __ZThn1196_N3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")
}

#[doc(alias = "RBX::Network::LogError(RBX::Network::Replicator *,std::string const&)")]
// 0xafac40 — __ZN3RBX7Network8LogErrorEPNS0_10ReplicatorERKSs
// type: void __fastcall(RBX::DataModel **this, char **, const std::string *)
pub fn stub_0xafac40() -> ! {
    todo!("0xafac40 __ZN3RBX7Network8LogErrorEPNS0_10ReplicatorERKSs")
}

#[doc(alias = "RBX::Network::Replicator::processNextIncomingPacket(void)")]
// 0xafb800 — __ZN3RBX7Network10Replicator25processNextIncomingPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xafb800() -> ! {
    todo!("0xafb800 __ZN3RBX7Network10Replicator25processNextIncomingPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::sendItemsPacket(void)")]
// 0xafbb48 — __ZN3RBX7Network10Replicator15sendItemsPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xafbb48() -> ! {
    todo!("0xafbb48 __ZN3RBX7Network10Replicator15sendItemsPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::isInitialDataSent(void)")]
// 0xafbd8c — __ZN3RBX7Network10Replicator17isInitialDataSentEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xafbd8c() -> ! {
    todo!("0xafbd8c __ZN3RBX7Network10Replicator17isInitialDataSentEv")
}

#[doc(alias = "RBX::Network::Replicator::sendClusterChunk(RBX::StreamRegion::Id const&)")]
// 0xafbdb8 — __ZN3RBX7Network10Replicator16sendClusterChunkERKNS_12StreamRegion2IdE
// type: void __fastcall(int, double *, int, int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xafbdb8() -> ! {
    todo!("0xafbdb8 __ZN3RBX7Network10Replicator16sendClusterChunkERKNS_12StreamRegion2IdE")
}

#[doc(alias = "RBX::Network::Replicator::sendClusterPacket(void)")]
// 0xafc5d8 — __ZN3RBX7Network10Replicator17sendClusterPacketEv
// type: RBX::Network::IdSerializer *__fastcall(RBX::Network::Replicator *this, int, int, const void *)
pub fn stub_0xafc5d8() -> ! {
    todo!("0xafc5d8 __ZN3RBX7Network10Replicator17sendClusterPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::sendDataPing(void)")]
// 0xb020f8 — __ZN3RBX7Network10Replicator12sendDataPingEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb020f8() -> ! {
    todo!("0xb020f8 __ZN3RBX7Network10Replicator12sendDataPingEv")
}

#[doc(alias = "RBX::Network::Replicator::getPhysicsMtuSize(void)const")]
// 0xb04ad4 — __ZNK3RBX7Network10Replicator17getPhysicsMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xb04ad4() -> ! {
    todo!("0xb04ad4 __ZNK3RBX7Network10Replicator17getPhysicsMtuSizeEv")
}

#[doc(alias = "RBX::Network::Replicator::getMetric(std::string const&)const")]
// 0xb04b48 — __ZNK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
pub fn stub_0xb04b48() -> ! {
    todo!("0xb04b48 __ZNK3RBX7Network10Replicator9getMetricERKSs")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::getMetric(std::string const&)const")]
// 0xb04f70 — __ZThn1192_NK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
pub fn stub_0xb04f70() -> ! {
    todo!("0xb04f70 __ZThn1192_NK3RBX7Network10Replicator9getMetricERKSs")
}

#[doc(alias = "RBX::Network::Replicator::getMetricValue(std::string const&)const")]
// 0xb04f80 — __ZNK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
pub fn stub_0xb04f80() -> ! {
    todo!("0xb04f80 __ZNK3RBX7Network10Replicator14getMetricValueERKSs")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::getMetricValue(std::string const&)const")]
// 0xb05000 — __ZThn1192_NK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
pub fn stub_0xb05000() -> ! {
    todo!("0xb05000 __ZThn1192_NK3RBX7Network10Replicator14getMetricValueERKSs")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator> RBX::shared_from<RBX::Network::Replicator>(RBX::Network::Replicator*)")]
// 0xb05b88 — __ZN3RBX11shared_fromINS_7Network10ReplicatorEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_0xb05b88() -> ! {
    todo!("0xb05b88 __ZN3RBX11shared_fromINS_7Network10ReplicatorEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::ClusterReplicationData(void)")]
// 0xb05e1c — __ZN3RBX7Network10Replicator22ClusterReplicationDataC1Ev
// type: RBX::Network::Replicator::ClusterReplicationData *__fastcall(RBX::Network::Replicator::ClusterReplicationData *this)
pub fn stub_0xb05e1c() -> ! {
    todo!("0xb05e1c __ZN3RBX7Network10Replicator22ClusterReplicationDataC1Ev")
}

#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::~ClusterReplicationData()")]
// 0xb060c0 — __ZN3RBX7Network10Replicator22ClusterReplicationDataD1Ev
// type: void __fastcall(RBX::Network::Replicator::ClusterReplicationData *__hidden this)
pub fn stub_0xb060c0() -> ! {
    todo!("0xb060c0 __ZN3RBX7Network10Replicator22ClusterReplicationDataD1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClusterPacketCache>::reset(void)")]
// 0xb06dd8 — __ZN5boost10shared_ptrIN3RBX7Network18ClusterPacketCacheEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb06dd8() -> ! {
    todo!("0xb06dd8 __ZN5boost10shared_ptrIN3RBX7Network18ClusterPacketCacheEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender>::reset(void)")]
// 0xb06e78 — __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSenderEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb06e78() -> ! {
    todo!("0xb06e78 __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSenderEE5resetEv")
}

#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// 0xb06f18 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xb06f18() -> ! {
    todo!("0xb06f18 __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob>::reset(void)")]
// 0xb071d8 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator11SendDataJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb071d8() -> ! {
    todo!("0xb071d8 __ZN5boost10shared_ptrIN3RBX7Network10Replicator11SendDataJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob>::reset(void)")]
// 0xb07278 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator14SendClusterJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07278() -> ! {
    todo!("0xb07278 __ZN5boost10shared_ptrIN3RBX7Network10Replicator14SendClusterJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob>::reset(void)")]
// 0xb07318 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator17ProcessPacketsJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07318() -> ! {
    todo!("0xb07318 __ZN5boost10shared_ptrIN3RBX7Network10Replicator17ProcessPacketsJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::PingJob>::reset(void)")]
// 0xb073b8 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator7PingJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb073b8() -> ! {
    todo!("0xb073b8 __ZN5boost10shared_ptrIN3RBX7Network10Replicator7PingJobEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClusterPacketCache> RBX::shared_from<RBX::Network::ClusterPacketCache>(RBX::Network::ClusterPacketCache*)")]
// 0xb076ec — __ZN3RBX11shared_fromINS_7Network18ClusterPacketCacheEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_0xb076ec() -> ! {
    todo!("0xb076ec __ZN3RBX11shared_fromINS_7Network18ClusterPacketCacheEEEN5boost10shared_ptrIT_EEPS5_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer>::reset(void)")]
// 0xb07dec — __ZN5boost10shared_ptrIN3RBX7Network17ConcurrentRakPeerEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07dec() -> ! {
    todo!("0xb07dec __ZN5boost10shared_ptrIN3RBX7Network17ConcurrentRakPeerEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::StatsItem>::reset(void)")]
// 0xb07e8c — __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StatsItemEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07e8c() -> ! {
    todo!("0xb07e8c __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StatsItemEE5resetEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>::shared_ptr<RBX::Network::SharedStringDictionary>(RBX::Network::SharedStringDictionary *)")]
// 0xb07f30 — __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEC1IS3_EEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, boost::detail::shared_count *, int, int, void *, int)
pub fn stub_0xb07f30() -> ! {
    todo!("0xb07f30 __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEC1IS3_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>::operator=(rbx_core::SharedPtr<RBX::Network::SharedStringDictionary> const&)")]
// 0xb08350 — __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEaSERKS4_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0xb08350() -> ! {
    todo!("0xb08350 __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEaSERKS4_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Replicator> RBX::weak_from<RBX::Network::Replicator>(RBX::Network::Replicator*)")]
// 0xb0a6e8 — __ZN3RBX9weak_fromINS_7Network10ReplicatorEEEN5boost8weak_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_0xb0a6e8() -> ! {
    todo!("0xb0a6e8 __ZN3RBX9weak_fromINS_7Network10ReplicatorEEEN5boost8weak_ptrIT_EEPS5_")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::wake(void)")]
// 0xb0c974 — __ZN3RBX7Network10Replicator17ProcessPacketsJob4wakeEv
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this)
pub fn stub_0xb0c974() -> ! {
    todo!("0xb0c974 __ZN3RBX7Network10Replicator17ProcessPacketsJob4wakeEv")
}

#[doc(alias = "RBX::Network::Replicator::getClassName(void)const")]
// 0xb0cc30 — __ZNK3RBX7Network10Replicator12getClassNameEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0cc30() -> ! {
    todo!("0xb0cc30 __ZNK3RBX7Network10Replicator12getClassNameEv")
}

#[doc(alias = "RBX::Network::Replicator::requestCharacter(void)")]
// 0xb0cc40 — __ZN3RBX7Network10Replicator16requestCharacterEv
// type: void __fastcall __noreturn(RBX::Network::Replicator *this)
pub fn stub_0xb0cc40() -> ! {
    todo!("0xb0cc40 __ZN3RBX7Network10Replicator16requestCharacterEv")
}

#[doc(alias = "RBX::Network::Replicator::postProcessPacket(void)")]
// 0xb0ce80 — __ZN3RBX7Network10Replicator17postProcessPacketEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0ce80() -> ! {
    todo!("0xb0ce80 __ZN3RBX7Network10Replicator17postProcessPacketEv")
}

#[doc(alias = "RBX::Network::Replicator::onSentMarker(long)")]
// 0xb0cea8 — __ZN3RBX7Network10Replicator12onSentMarkerEl
// type: void __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xb0cea8() -> ! {
    todo!("0xb0cea8 __ZN3RBX7Network10Replicator12onSentMarkerEl")
}

#[doc(alias = "RBX::Network::Replicator::processSendStats(unsigned int)")]
// 0xb0ceb0 — __ZN3RBX7Network10Replicator16processSendStatsEj
// type: void __fastcall(RBX::Network::Replicator *this, unsigned int)
pub fn stub_0xb0ceb0() -> ! {
    todo!("0xb0ceb0 __ZN3RBX7Network10Replicator16processSendStatsEj")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::getClassName(void)const")]
// 0xb0cec0 — __ZThn32_NK3RBX7Network10Replicator12getClassNameEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0cec0() -> ! {
    todo!("0xb0cec0 __ZThn32_NK3RBX7Network10Replicator12getClassNameEv")
}

#[doc(alias = "`non-virtual thunk toRBX::Network::Replicator::UsesReliabilityLayer(void)const")]
// 0xb0cef0 — __ZThn1180_NK3RBX7Network10Replicator20UsesReliabilityLayerEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0cef0() -> ! {
    todo!("0xb0cef0 __ZThn1180_NK3RBX7Network10Replicator20UsesReliabilityLayerEv")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::~SendDataJob()")]
// 0xb0cf08 — __ZN3RBX7Network10Replicator11SendDataJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *__hidden this)
pub fn stub_0xb0cf08() -> ! {
    todo!("0xb0cf08 __ZN3RBX7Network10Replicator11SendDataJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::~SendDataJob()")]
// 0xb0cfd4 — __ZN3RBX7Network10Replicator11SendDataJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *__hidden this)
pub fn stub_0xb0cfd4() -> ! {
    todo!("0xb0cfd4 __ZN3RBX7Network10Replicator11SendDataJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb0d0b4 — __ZN3RBX7Network10Replicator11SendDataJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xb0d0b4() -> ! {
    todo!("0xb0d0b4 __ZN3RBX7Network10Replicator11SendDataJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}
