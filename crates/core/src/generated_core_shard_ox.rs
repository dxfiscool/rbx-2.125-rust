//! core shard ox — 100 core stubs EA-sorted, 0xadb040..0xb0d0b4 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered, global-deduped).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list_av_2<RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::type> boost::bind<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>,RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>(void (RBX::Network::InterpolatingPhysicsReceiver::*)(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>),RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
// 0xadb040 — __ZN5boost4bindIvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS3_EEPS3_S5_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_
// type: void __fastcall(int, int, pthread_mutex_t *, int, int *)
pub fn stub_0xadb040() {
    // IDA 0xadb040: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> *,RBX::Network::InterpolatingPhysicsReceiver::Job *,boost::detail::shared_count &)")]
// 0xadcab0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xadcab0() {
    // IDA 0xadcab0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> const*,RBX::Network::InterpolatingPhysicsReceiver::Job *)const")]
// 0xadcc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xadcc60() {
    // IDA 0xadcc60: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
// 0xadcf0c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED1Ev
// type: void()
pub fn stub_0xadcf0c() {
    // IDA 0xadcf0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
// 0xadcf10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xadcf10() {
    // IDA 0xadcf10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::dispose(void)")]
// 0xadcf1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xadcf1c() {
    // IDA 0xadcf1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_deleter(std::type_info const&)")]
// 0xadcf30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xadcf30() {
    // IDA 0xadcf30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_untyped_deleter(void)")]
// 0xadcf34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xadcf34() {
    // IDA 0xadcf34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::operator()(RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)const")]
// 0xadd37c — __ZNK5boost4_mfi3mf1IvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS4_EEEclEPS4_S6_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xadd37c() {
    // IDA 0xadd37c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>::list2(boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>)")]
// 0xadd834 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX7Network28InterpolatingPhysicsReceiverEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: _DWORD *__fastcall(_DWORD *, int, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xadd834() {
    // IDA 0xadd834: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History> *,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History *,boost::detail::shared_count &)")]
// 0xaddea4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int, int, int, void *, int)
pub fn stub_0xaddea4() {
    // IDA 0xaddea4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
// 0xade088 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED1Ev
// type: void()
pub fn stub_0xade088() {
    // IDA 0xade088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
// 0xade08c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xade08c() {
    // IDA 0xade08c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::dispose(void)")]
// 0xade098 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE7disposeEv
// type: void __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xade098() {
    // IDA 0xade098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_deleter(std::type_info const&)")]
// 0xade180 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xade180() {
    // IDA 0xade180: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_untyped_deleter(void)")]
// 0xade184 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xade184() {
    // IDA 0xade184: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade188 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD1Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_0xade188() {
    // IDA 0xade188: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade194 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD0Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_0xade194() {
    // IDA 0xade194: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xade234 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xade234() {
    // IDA 0xade234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xade250 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_0xade250() {
    // IDA 0xade250: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade4b4 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD2Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_0xade4b4() {
    // IDA 0xade4b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::ReplicatorJob::~ReplicatorJob()")]
// 0xade658 — __ZN3RBX7Network13ReplicatorJobD0Ev
// type: void __fastcall(RBX::Network::ReplicatorJob *__hidden this)
pub fn stub_0xade658() {
    // IDA 0xade658: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::sendMarker(void)")]
// 0xaded58 — __ZN3RBX7Network10Replicator10sendMarkerEv
// type: void __fastcall(RBX::Network::Replicator *this, _DWORD *)
pub fn stub_0xaded58() {
    // IDA 0xaded58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::closeConnection(void)")]
// 0xadf958 — __ZN3RBX7Network10Replicator15closeConnectionEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xadf958() {
    // IDA 0xadf958: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::getPlayer(void)")]
// 0xadfa08 — __ZN3RBX7Network10Replicator9getPlayerEv
// type: void __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xadfa08() {
    // IDA 0xadfa08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::getRakStatsString(int)")]
// 0xadfc3c — __ZN3RBX7Network10Replicator17getRakStatsStringEi
// type: int __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xadfc3c() {
    // IDA 0xadfc3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::disableProcessPackets(void)")]
// 0xadfc9c — __ZN3RBX7Network10Replicator21disableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xadfc9c() {
    // IDA 0xadfc9c: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::enableProcessPackets(void)")]
// 0xadfca8 — __ZN3RBX7Network10Replicator20enableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator::ProcessPacketsJob **this)
pub fn stub_0xadfca8() {
    // IDA 0xadfca8: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::getPort(void)const")]
// 0xadfcb8 — __ZNK3RBX7Network10Replicator7getPortEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xadfcb8() {
    // IDA 0xadfcb8: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::getIpAddress(void)const")]
// 0xadfcc8 — __ZNK3RBX7Network10Replicator12getIpAddressEv
// type: int __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xadfcc8() {
    // IDA 0xadfcc8: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::getDefault(RBX::Name const&)")]
// 0xae0594 — __ZN3RBX7Network10Replicator10getDefaultERKNS_4NameE
// type: int __fastcall(RBX::Network::Replicator *this, const char **)
pub fn stub_0xae0594() {
    // IDA 0xae0594: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::ReplicatorJob::canSendPacket(rbx_core::SharedPtr<RBX::Network::Replicator> &,PacketPriority)")]
// 0xae1000 — __ZN3RBX7Network13ReplicatorJob13canSendPacketERN5boost10shared_ptrINS0_10ReplicatorEEE14PacketPriority
// type: bool __fastcall(int *, int)
pub fn stub_0xae1000() {
    // IDA 0xae1000: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::getBufferCountAvailable(int,PacketPriority)")]
// 0xae1058 — __ZN3RBX7Network10Replicator23getBufferCountAvailableEi14PacketPriority
// type: int __fastcall(int, int, int)
pub fn stub_0xae1058() {
    // IDA 0xae1058: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::onStatisticsChanged(RBX::Network::ConcurrentRakPeerStats const&)")]
// 0xae1f7c — __ZN3RBX7Network10Replicator19onStatisticsChangedERKNS0_22ConcurrentRakPeerStatsE
// type: void *__fastcall(int, const void *)
pub fn stub_0xae1f7c() {
    // IDA 0xae1f7c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::createPhysicsReceiver(RBX::NetworkSettings::PhysicsReceiveMethod,bool)")]
// 0xae22e8 — __ZN3RBX7Network10Replicator21createPhysicsReceiverENS_15NetworkSettings20PhysicsReceiveMethodEb
// type: void __fastcall(_DWORD *, int, char, int, int, int, int, int, int, int, int, void *, void *, int, int, int, int, int)
pub fn stub_0xae22e8() {
    // IDA 0xae22e8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::clearIncomingPackets(void)")]
// 0xae2948 — __ZN3RBX7Network10Replicator20clearIncomingPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae2948() {
    // IDA 0xae2948: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae29b8 — __ZN3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_0xae29b8() {
    // IDA 0xae29b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae2a58 — __ZN3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_0xae2a58() {
    // IDA 0xae2a58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2a64 — __ZThn32_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_0xae2a64() {
    // IDA 0xae2a64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2b08 — __ZThn36_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_0xae2b08() {
    // IDA 0xae2b08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2bac — __ZThn1180_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_0xae2bac() {
    // IDA 0xae2bac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2c50 — __ZThn1192_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_0xae2c50() {
    // IDA 0xae2c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae2cf4 — __ZN3RBX7Network10ReplicatorD2Ev
// type: void __fastcall(struct _Unwind_Exception *lpuexcpt, int, int)
pub fn stub_0xae2cf4() {
    // IDA 0xae2cf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3aa8 — __ZThn32_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this, int, int)
pub fn stub_0xae3aa8() {
    // IDA 0xae3aa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ab4 — __ZThn36_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae3ab4() {
    // IDA 0xae3ab4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ac0 — __ZThn1180_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae3ac0() {
    // IDA 0xae3ac0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ad0 — __ZThn1192_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae3ad0() {
    // IDA 0xae3ad0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::closeReplicationItem(RBX::Network::Replicator::ReplicationData &)")]
// 0xae5f20 — __ZN3RBX7Network10Replicator20closeReplicationItemERNS1_15ReplicationDataE
// type: int __fastcall(int)
pub fn stub_0xae5f20() {
    // IDA 0xae5f20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::physicsSenderStats(void)")]
// 0xae5f44 — __ZN3RBX7Network10Replicator18physicsSenderStatsEv
// type: char *__fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae5f44() {
    // IDA 0xae5f44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xae5f4c — __ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_0xae5f4c() {
    // IDA 0xae5f4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::SendClusterJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xae603c — __ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_0xae603c() {
    // IDA 0xae603c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::getAdjustedMtuSize(void)const")]
// 0xae6238 — __ZNK3RBX7Network10Replicator18getAdjustedMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xae6238() {
    // IDA 0xae6238: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::clusterOutStep(void)")]
// 0xae62ac — __ZN3RBX7Network10Replicator14clusterOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae62ac() {
    // IDA 0xae62ac: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::requestDisconnect(void)")]
// 0xae6410 — __ZN3RBX7Network10Replicator17requestDisconnectEv
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Instance *)
pub fn stub_0xae6410() {
    // IDA 0xae6410: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::dataOutStep(void)")]
// 0xae6848 — __ZN3RBX7Network10Replicator11dataOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xae6848() {
    // IDA 0xae6848: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::isInStreamedRegions(RBX::Extents const&)")]
// 0xae6f38 — __ZN3RBX7Network10Replicator19isInStreamedRegionsERKNS_7ExtentsE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Extents *)
pub fn stub_0xae6f38() {
    // IDA 0xae6f38: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::updateStatsItem(RBX::Stats::StatsService *)")]
// 0xaebf24 — __ZN3RBX7Network10Replicator15updateStatsItemEPNS_5Stats12StatsServiceE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Stats::StatsService *)
pub fn stub_0xaebf24() {
    // IDA 0xaebf24: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::createPhysicsSender(RBX::NetworkSettings::PhysicsSendMethod)")]
// 0xaece78 — __ZN3RBX7Network10Replicator19createPhysicsSenderENS_15NetworkSettings17PhysicsSendMethodE
// type: void __fastcall(_DWORD *, int, int, int)
pub fn stub_0xaece78() {
    // IDA 0xaece78: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::incomingPacketsCount(void)const")]
// 0xaed8a8 — __ZNK3RBX7Network10Replicator20incomingPacketsCountEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xaed8a8() {
    // IDA 0xaed8a8: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xaf7c18 — __ZN3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
pub fn stub_0xaf7c18() {
    // IDA 0xaf7c18: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// 0xaf7ce0 — __ZThn1196_N3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
pub fn stub_0xaf7ce0() {
    // IDA 0xaf7ce0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::LogError(RBX::Network::Replicator *,std::string const&)")]
// 0xafac40 — __ZN3RBX7Network8LogErrorEPNS0_10ReplicatorERKSs
// type: void __fastcall(RBX::DataModel **this, char **, const std::string *)
pub fn stub_0xafac40() {
    // IDA 0xafac40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::processNextIncomingPacket(void)")]
// 0xafb800 — __ZN3RBX7Network10Replicator25processNextIncomingPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xafb800() {
    // IDA 0xafb800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::sendItemsPacket(void)")]
// 0xafbb48 — __ZN3RBX7Network10Replicator15sendItemsPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xafbb48() {
    // IDA 0xafbb48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::isInitialDataSent(void)")]
// 0xafbd8c — __ZN3RBX7Network10Replicator17isInitialDataSentEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xafbd8c() {
    // IDA 0xafbd8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Network::Replicator::sendClusterChunk(RBX::StreamRegion::Id const&)")]
// 0xafbdb8 — __ZN3RBX7Network10Replicator16sendClusterChunkERKNS_12StreamRegion2IdE
// type: void __fastcall(int, double *, int, int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xafbdb8() {
    // IDA 0xafbdb8: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::sendClusterPacket(void)")]
// 0xafc5d8 — __ZN3RBX7Network10Replicator17sendClusterPacketEv
// type: RBX::Network::IdSerializer *__fastcall(RBX::Network::Replicator *this, int, int, const void *)
pub fn stub_0xafc5d8() {
    // IDA 0xafc5d8: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::sendDataPing(void)")]
// 0xb020f8 — __ZN3RBX7Network10Replicator12sendDataPingEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb020f8() {
    // IDA 0xb020f8: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::getPhysicsMtuSize(void)const")]
// 0xb04ad4 — __ZNK3RBX7Network10Replicator17getPhysicsMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_0xb04ad4() {
    // IDA 0xb04ad4: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::getMetric(std::string const&)const")]
// 0xb04b48 — __ZNK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
pub fn stub_0xb04b48() {
    // IDA 0xb04b48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::getMetric(std::string const&)const")]
// 0xb04f70 — __ZThn1192_NK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
pub fn stub_0xb04f70() {
    // IDA 0xb04f70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::getMetricValue(std::string const&)const")]
// 0xb04f80 — __ZNK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
pub fn stub_0xb04f80() {
    // IDA 0xb04f80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::getMetricValue(std::string const&)const")]
// 0xb05000 — __ZThn1192_NK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
pub fn stub_0xb05000() {
    // IDA 0xb05000: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator> RBX::shared_from<RBX::Network::Replicator>(RBX::Network::Replicator*)")]
// 0xb05b88 — __ZN3RBX11shared_fromINS_7Network10ReplicatorEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_0xb05b88() {
    // IDA 0xb05b88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::ClusterReplicationData(void)")]
// 0xb05e1c — __ZN3RBX7Network10Replicator22ClusterReplicationDataC1Ev
// type: RBX::Network::Replicator::ClusterReplicationData *__fastcall(RBX::Network::Replicator::ClusterReplicationData *this)
pub fn stub_0xb05e1c() {
    // IDA 0xb05e1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::~ClusterReplicationData()")]
// 0xb060c0 — __ZN3RBX7Network10Replicator22ClusterReplicationDataD1Ev
// type: void __fastcall(RBX::Network::Replicator::ClusterReplicationData *__hidden this)
pub fn stub_0xb060c0() {
    // IDA 0xb060c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClusterPacketCache>::reset(void)")]
// 0xb06dd8 — __ZN5boost10shared_ptrIN3RBX7Network18ClusterPacketCacheEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb06dd8() {
    // IDA 0xb06dd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PhysicsSender>::reset(void)")]
// 0xb06e78 — __ZN5boost10shared_ptrIN3RBX7Network13PhysicsSenderEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb06e78() {
    // IDA 0xb06e78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TaskScheduler::remove(rbx_core::SharedPtr<RBX::TaskScheduler::Job>)")]
// 0xb06f18 — __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE
// type: void __fastcall(int, int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xb06f18() {
    // IDA 0xb06f18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob>::reset(void)")]
// 0xb071d8 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator11SendDataJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb071d8() {
    // IDA 0xb071d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob>::reset(void)")]
// 0xb07278 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator14SendClusterJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07278() {
    // IDA 0xb07278: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob>::reset(void)")]
// 0xb07318 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator17ProcessPacketsJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07318() {
    // IDA 0xb07318: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::PingJob>::reset(void)")]
// 0xb073b8 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator7PingJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb073b8() {
    // IDA 0xb073b8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClusterPacketCache> RBX::shared_from<RBX::Network::ClusterPacketCache>(RBX::Network::ClusterPacketCache*)")]
// 0xb076ec — __ZN3RBX11shared_fromINS_7Network18ClusterPacketCacheEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_0xb076ec() {
    // IDA 0xb076ec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer>::reset(void)")]
// 0xb07dec — __ZN5boost10shared_ptrIN3RBX7Network17ConcurrentRakPeerEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07dec() {
    // IDA 0xb07dec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::StatsItem>::reset(void)")]
// 0xb07e8c — __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StatsItemEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0xb07e8c() {
    // IDA 0xb07e8c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>::shared_ptr<RBX::Network::SharedStringDictionary>(RBX::Network::SharedStringDictionary *)")]
// 0xb07f30 — __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEC1IS3_EEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, boost::detail::shared_count *, int, int, void *, int)
pub fn stub_0xb07f30() {
    // IDA 0xb07f30: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::SharedStringDictionary>::operator=(rbx_core::SharedPtr<RBX::Network::SharedStringDictionary> const&)")]
// 0xb08350 — __ZN5boost10shared_ptrIN3RBX7Network22SharedStringDictionaryEEaSERKS4_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0xb08350() {
    // IDA 0xb08350: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Replicator> RBX::weak_from<RBX::Network::Replicator>(RBX::Network::Replicator*)")]
// 0xb0a6e8 — __ZN3RBX9weak_fromINS_7Network10ReplicatorEEEN5boost8weak_ptrIT_EEPS5_
// type: void __fastcall(int, int)
pub fn stub_0xb0a6e8() {
    // IDA 0xb0a6e8: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::wake(void)")]
// 0xb0c974 — __ZN3RBX7Network10Replicator17ProcessPacketsJob4wakeEv
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this)
pub fn stub_0xb0c974() {
    // IDA 0xb0c974: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::getClassName(void)const")]
// 0xb0cc30 — __ZNK3RBX7Network10Replicator12getClassNameEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0cc30() {
    // IDA 0xb0cc30: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::requestCharacter(void)")]
// 0xb0cc40 — __ZN3RBX7Network10Replicator16requestCharacterEv
// type: void __fastcall __noreturn(RBX::Network::Replicator *this)
pub fn stub_0xb0cc40() {
    // IDA 0xb0cc40: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::postProcessPacket(void)")]
// 0xb0ce80 — __ZN3RBX7Network10Replicator17postProcessPacketEv
// type: void __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0ce80() {
    // IDA 0xb0ce80: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::Network::Replicator::onSentMarker(long)")]
// 0xb0cea8 — __ZN3RBX7Network10Replicator12onSentMarkerEl
// type: void __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_0xb0cea8() {
    // IDA 0xb0cea8: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Network::Replicator::processSendStats(unsigned int)")]
// 0xb0ceb0 — __ZN3RBX7Network10Replicator16processSendStatsEj
// type: void __fastcall(RBX::Network::Replicator *this, unsigned int)
pub fn stub_0xb0ceb0() {
    // IDA 0xb0ceb0: player/network handle owned by the network/datamodel crates — carrier no-op in core.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::getClassName(void)const")]
// 0xb0cec0 — __ZThn32_NK3RBX7Network10Replicator12getClassNameEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0cec0() {
    // IDA 0xb0cec0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::UsesReliabilityLayer(void)const")]
// 0xb0cef0 — __ZThn1180_NK3RBX7Network10Replicator20UsesReliabilityLayerEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_0xb0cef0() {
    // IDA 0xb0cef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::~SendDataJob()")]
// 0xb0cf08 — __ZN3RBX7Network10Replicator11SendDataJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *__hidden this)
pub fn stub_0xb0cf08() {
    // IDA 0xb0cf08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::~SendDataJob()")]
// 0xb0cfd4 — __ZN3RBX7Network10Replicator11SendDataJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *__hidden this)
pub fn stub_0xb0cfd4() {
    // IDA 0xb0cfd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb0d0b4 — __ZN3RBX7Network10Replicator11SendDataJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xb0d0b4() {
    // IDA 0xb0d0b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
