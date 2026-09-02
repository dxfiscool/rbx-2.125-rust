//! core wdog7H equiv — 100 core stubs EA-sorted asc distinct not yet in core (narrow filtered).
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/core/src — next 100 uncovered after 0x74b88c (prev max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsSendMethod*,std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>>,unsigned long,RBX::NetworkSettings::PhysicsSendMethod const&)")]
// 0x9ba268 — __ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
 // type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
pub fn stub_9ba268() -> ! {
    todo!("0x9ba268 __ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Network::PhysicsReceiver::setPhysics(RBX::MechanismItem const&,RBX::RemoteTime const&,unsigned int)")]
// 0x9be624 — __ZN3RBX7Network15PhysicsReceiver10setPhysicsERKNS_13MechanismItemERKNS_10RemoteTimeEj
 // type: void __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_9be624() -> ! {
    todo!("0x9be624 __ZN3RBX7Network15PhysicsReceiver10setPhysicsERKNS_13MechanismItemERKNS_10RemoteTimeEj")
}

#[doc(alias = "RBX::Network::PhysicsSender::sendTouches(PacketPriority)")]
// 0x9bfa90 — __ZN3RBX7Network13PhysicsSender11sendTouchesE14PacketPriority
 // type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, char, int, char, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_9bfa90() -> ! {
    todo!("0x9bfa90 __ZN3RBX7Network13PhysicsSender11sendTouchesE14PacketPriority")
}

#[doc(alias = "RBX::Network::PhysicsSender::PhysicsSender(RBX::Network::Replicator &)")]
// 0x9c0908 — __ZN3RBX7Network13PhysicsSenderC2ERNS0_10ReplicatorE
 // type: RBX::Network::PhysicsSender *__fastcall(RBX::Network::PhysicsSender *this, RBX::Network::Replicator *)
pub fn stub_9c0908() -> ! {
    todo!("0x9c0908 __ZN3RBX7Network13PhysicsSenderC2ERNS0_10ReplicatorE")
}

#[doc(alias = "RBX::Network::PhysicsSender::onTouchStep(RBX::TouchPair const&)")]
// 0x9c0a9c — __ZN3RBX7Network13PhysicsSender11onTouchStepERKNS_9TouchPairE
 // type: int __fastcall(int, int)
pub fn stub_9c0a9c() -> ! {
    todo!("0x9c0a9c __ZN3RBX7Network13PhysicsSender11onTouchStepERKNS_9TouchPairE")
}

#[doc(alias = "RBX::Network::PhysicsSender::connectTouches(void)")]
// 0x9c0ab8 — __ZN3RBX7Network13PhysicsSender14connectTouchesEv
 // type: void __fastcall(RBX::Network::PhysicsSender *this)
pub fn stub_9c0ab8() -> ! {
    todo!("0x9c0ab8 __ZN3RBX7Network13PhysicsSender14connectTouchesEv")
}

#[doc(alias = "RBX::Network::PhysicsSender::~PhysicsSender()")]
// 0x9c1ea4 — __ZN3RBX7Network13PhysicsSenderD0Ev
 // type: void __fastcall(RBX::Network::PhysicsSender *__hidden this)
pub fn stub_9c1ea4() -> ! {
    todo!("0x9c1ea4 __ZN3RBX7Network13PhysicsSenderD0Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::~PhysicsSender()")]
// 0x9c1f44 — __ZN3RBX7Network13PhysicsSenderD1Ev
 // type: void __fastcall(RBX::Network::PhysicsSender *__hidden this)
pub fn stub_9c1f44() -> ! {
    todo!("0x9c1f44 __ZN3RBX7Network13PhysicsSenderD1Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::~PhysicsSender()")]
// 0x9c1f50 — __ZN3RBX7Network13PhysicsSenderD2Ev
 // type: void __fastcall(RBX::Network::PhysicsSender *__hidden this)
pub fn stub_9c1f50() -> ! {
    todo!("0x9c1f50 __ZN3RBX7Network13PhysicsSenderD2Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
// 0x9c5830 — __ZN3RBX7Network13PhysicsSender8TouchJobD1Ev
 // type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
pub fn stub_9c5830() -> ! {
    todo!("0x9c5830 __ZN3RBX7Network13PhysicsSender8TouchJobD1Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
// 0x9c583c — __ZN3RBX7Network13PhysicsSender8TouchJobD0Ev
 // type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
pub fn stub_9c583c() -> ! {
    todo!("0x9c583c __ZN3RBX7Network13PhysicsSender8TouchJobD0Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x9c58dc — __ZN3RBX7Network13PhysicsSender8TouchJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
 // type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_9c58dc() -> ! {
    todo!("0x9c58dc __ZN3RBX7Network13PhysicsSender8TouchJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x9c58fc — __ZN3RBX7Network13PhysicsSender8TouchJob5errorERKNS_13TaskScheduler3Job5StatsE
 // type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_9c58fc() -> ! {
    todo!("0x9c58fc __ZN3RBX7Network13PhysicsSender8TouchJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::~TouchJob()")]
// 0x9c5e38 — __ZN3RBX7Network13PhysicsSender8TouchJobD2Ev
 // type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *__hidden this)
pub fn stub_9c5e38() -> ! {
    todo!("0x9c5e38 __ZN3RBX7Network13PhysicsSender8TouchJobD2Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
// 0x9c6168 — __ZN3RBX7Network13PhysicsSender3JobD1Ev
 // type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
pub fn stub_9c6168() -> ! {
    todo!("0x9c6168 __ZN3RBX7Network13PhysicsSender3JobD1Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
// 0x9c6174 — __ZN3RBX7Network13PhysicsSender3JobD0Ev
 // type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
pub fn stub_9c6174() -> ! {
    todo!("0x9c6174 __ZN3RBX7Network13PhysicsSender3JobD0Ev")
}

#[doc(alias = "RBX::Network::PhysicsSender::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x9c6214 — __ZN3RBX7Network13PhysicsSender3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE
 // type: void __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_9c6214() -> ! {
    todo!("0x9c6214 __ZN3RBX7Network13PhysicsSender3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::PhysicsSender::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x9c6234 — __ZN3RBX7Network13PhysicsSender3Job5errorERKNS_13TaskScheduler3Job5StatsE
 // type: int __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_9c6234() -> ! {
    todo!("0x9c6234 __ZN3RBX7Network13PhysicsSender3Job5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::PhysicsSender::Job::~Job()")]
// 0x9c6568 — __ZN3RBX7Network13PhysicsSender3JobD2Ev
 // type: void __fastcall(RBX::Network::PhysicsSender::Job *__hidden this)
pub fn stub_9c6568() -> ! {
    todo!("0x9c6568 __ZN3RBX7Network13PhysicsSender3JobD2Ev")
}

#[doc(alias = "RBX::Network::Server::start(int,int)")]
// 0x9c6da4 — __ZN3RBX7Network6Server5startEii
 // type: int __fastcall(RBX::Network::ConcurrentRakPeer **this, unsigned __int16, int, const void *)
pub fn stub_9c6da4() -> ! {
    todo!("0x9c6da4 __ZN3RBX7Network6Server5startEii")
}

#[doc(alias = "RBX::Network::Server::stop(int)")]
// 0x9c7234 — __ZN3RBX7Network6Server4stopEi
 // type: int __fastcall(RBX::Network::ConcurrentRakPeer **this, char *, int, const void *)
pub fn stub_9c7234() -> ! {
    todo!("0x9c7234 __ZN3RBX7Network6Server4stopEi")
}

#[doc(alias = "RBX::Network::Server::getClientCount(void)")]
// 0x9c72a8 — __ZN3RBX7Network6Server14getClientCountEv
 // type: _DWORD __fastcall(RBX::Network::Server *__hidden this)
pub fn stub_9c72a8() -> ! {
    todo!("0x9c72a8 __ZN3RBX7Network6Server14getClientCountEv")
}

#[doc(alias = "RBX::Network::Server::Server(void)")]
// 0x9c7444 — __ZN3RBX7Network6ServerC1Ev
 // type: int __fastcall(RBX::Network::Server *this)
pub fn stub_9c7444() -> ! {
    todo!("0x9c7444 __ZN3RBX7Network6ServerC1Ev")
}

#[doc(alias = "RBX::Network::Server::Server(void)")]
// 0x9c7450 — __ZN3RBX7Network6ServerC2Ev
 // type: RBX::Network::Peer *__fastcall(RBX::Network::Server *this)
pub fn stub_9c7450() -> ! {
    todo!("0x9c7450 __ZN3RBX7Network6ServerC2Ev")
}

#[doc(alias = "RBX::Network::Server::~Server()")]
// 0x9c7e78 — __ZN3RBX7Network6ServerD0Ev
 // type: void __fastcall(RBX::Network::Server *__hidden this)
pub fn stub_9c7e78() -> ! {
    todo!("0x9c7e78 __ZN3RBX7Network6ServerD0Ev")
}

#[doc(alias = "RBX::Network::Server::~Server()")]
// 0x9c7f18 — __ZN3RBX7Network6ServerD1Ev
 // type: void __fastcall(RBX::Network::Server *__hidden this)
pub fn stub_9c7f18() -> ! {
    todo!("0x9c7f18 __ZN3RBX7Network6ServerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
// 0x9c7f24 — __ZThn32_N3RBX7Network6ServerD0Ev
 // type: void __fastcall(RBX::Network::Server *__hidden this)
pub fn stub_9c7f24() -> ! {
    todo!("0x9c7f24 __ZThn32_N3RBX7Network6ServerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
// 0x9c7fc8 — __ZThn36_N3RBX7Network6ServerD0Ev
 // type: void __fastcall(RBX::Network::Server *__hidden this)
pub fn stub_9c7fc8() -> ! {
    todo!("0x9c7fc8 __ZThn36_N3RBX7Network6ServerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
// 0x9c806c — __ZThn92_N3RBX7Network6ServerD0Ev
 // type: void __fastcall(RBX::Network::Server *__hidden this)
pub fn stub_9c806c() -> ! {
    todo!("0x9c806c __ZThn92_N3RBX7Network6ServerD0Ev")
}

#[doc(alias = "RBX::Network::Server::~Server()")]
// 0x9c8110 — __ZN3RBX7Network6ServerD2Ev
 // type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
pub fn stub_9c8110() -> ! {
    todo!("0x9c8110 __ZN3RBX7Network6ServerD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
// 0x9c87d4 — __ZThn32_N3RBX7Network6ServerD1Ev
 // type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
pub fn stub_9c87d4() -> ! {
    todo!("0x9c87d4 __ZThn32_N3RBX7Network6ServerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
// 0x9c87e0 — __ZThn36_N3RBX7Network6ServerD1Ev
 // type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
pub fn stub_9c87e0() -> ! {
    todo!("0x9c87e0 __ZThn36_N3RBX7Network6ServerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Server::~Server()")]
// 0x9c87ec — __ZThn92_N3RBX7Network6ServerD1Ev
 // type: void __fastcall(RBX::Network::Server *this, int, int, const void *)
pub fn stub_9c87ec() -> ! {
    todo!("0x9c87ec __ZThn92_N3RBX7Network6ServerD1Ev")
}

#[doc(alias = "RBX::Network::Server::onCreateRakPeer(void)")]
// 0x9c8b20 — __ZN3RBX7Network6Server15onCreateRakPeerEv
 // type: int __fastcall(RBX::Network::ConcurrentRakPeer **this)
pub fn stub_9c8b20() -> ! {
    todo!("0x9c8b20 __ZN3RBX7Network6Server15onCreateRakPeerEv")
}

#[doc(alias = "RBX::Network::Server::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x9c8b88 — __ZN3RBX7Network6Server17onServiceProviderEPNS_15ServiceProviderES3_
 // type: void __fastcall(struct _Unwind_Exception *this, RBX::ServiceProvider *, pthread_mutex_t *, int)
pub fn stub_9c8b88() -> ! {
    todo!("0x9c8b88 __ZN3RBX7Network6Server17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "RBX::Network::Server::getPort(void)const")]
// 0x9caf1c — __ZNK3RBX7Network6Server7getPortEv
 // type: int __fastcall(RBX::Network::Server *this)
pub fn stub_9caf1c() -> ! {
    todo!("0x9caf1c __ZNK3RBX7Network6Server7getPortEv")
}

#[doc(alias = "RBX::Network::Server::setIsPlayerAuthenticationRequired(bool)")]
// 0x9caf48 — __ZN3RBX7Network6Server33setIsPlayerAuthenticationRequiredEb
 // type: int __fastcall(int this, bool)
pub fn stub_9caf48() -> ! {
    todo!("0x9caf48 __ZN3RBX7Network6Server33setIsPlayerAuthenticationRequiredEb")
}

#[doc(alias = "RBX::Network::ClusterPacketCache * RBX::ServiceProvider::create<RBX::Network::ClusterPacketCache>(void)const")]
// 0x9cbe78 — __ZNK3RBX15ServiceProvider6createINS_7Network18ClusterPacketCacheEEEPT_v
 // type: __guard *__fastcall(_DWORD *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
pub fn stub_9cbe78() -> ! {
    todo!("0x9cbe78 __ZNK3RBX15ServiceProvider6createINS_7Network18ClusterPacketCacheEEEPT_v")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache * RBX::ServiceProvider::create<RBX::Network::PhysicsPacketCache>(void)const")]
// 0x9cd308 — __ZNK3RBX15ServiceProvider6createINS_7Network18PhysicsPacketCacheEEEPT_v
 // type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
pub fn stub_9cd308() -> ! {
    todo!("0x9cd308 __ZNK3RBX15ServiceProvider6createINS_7Network18PhysicsPacketCacheEEEPT_v")
}

#[doc(alias = "RBX::StringConverter<RBX::Network::FilterResult>::convertToValue(std::string const&,RBX::Network::FilterResult&)")]
// 0x9d653c — __ZN3RBX15StringConverterINS_7Network12FilterResultEE14convertToValueERKSsRS2_
 // type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_9d653c() -> ! {
    todo!("0x9d653c __ZN3RBX15StringConverterINS_7Network12FilterResultEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::Network::ServerReplicator::setBasicFilteringEnabled(bool)")]
// 0x9d6ff4 — __ZN3RBX7Network16ServerReplicator24setBasicFilteringEnabledEb
 // type: void __fastcall(RBX::Network::ServerReplicator *this, int)
pub fn stub_9d6ff4() -> ! {
    todo!("0x9d6ff4 __ZN3RBX7Network16ServerReplicator24setBasicFilteringEnabledEb")
}

#[doc(alias = "RBX::Network::ServerReplicator::preventTerrainChanges(void)")]
// 0x9d701c — __ZN3RBX7Network16ServerReplicator21preventTerrainChangesEv
 // type: int __fastcall(int this)
pub fn stub_9d701c() -> ! {
    todo!("0x9d701c __ZN3RBX7Network16ServerReplicator21preventTerrainChangesEv")
}

#[doc(alias = "RBX::Network::ServerReplicator::createStatsItem(void)")]
// 0x9d7028 — __ZN3RBX7Network16ServerReplicator15createStatsItemEv
 // type: void __fastcall(RBX::Network::ServerReplicator *this)
pub fn stub_9d7028() -> ! {
    todo!("0x9d7028 __ZN3RBX7Network16ServerReplicator15createStatsItemEv")
}

#[doc(alias = "RBX::Network::ServerReplicator::canUseProtocolVersion(int)const")]
// 0x9d7414 — __ZNK3RBX7Network16ServerReplicator21canUseProtocolVersionEi
 // type: bool __fastcall(RBX::Network::ServerReplicator *this, int)
pub fn stub_9d7414() -> ! {
    todo!("0x9d7414 __ZNK3RBX7Network16ServerReplicator21canUseProtocolVersionEi")
}

#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d7e54 — __ZN3RBX7Network16ServerReplicatorD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d7e54() -> ! {
    todo!("0x9d7e54 __ZN3RBX7Network16ServerReplicatorD0Ev")
}

#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d7ef4 — __ZN3RBX7Network16ServerReplicatorD1Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d7ef4() -> ! {
    todo!("0x9d7ef4 __ZN3RBX7Network16ServerReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d7f00 — __ZThn32_N3RBX7Network16ServerReplicatorD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d7f00() -> ! {
    todo!("0x9d7f00 __ZThn32_N3RBX7Network16ServerReplicatorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d7fa4 — __ZThn36_N3RBX7Network16ServerReplicatorD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d7fa4() -> ! {
    todo!("0x9d7fa4 __ZThn36_N3RBX7Network16ServerReplicatorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d8048 — __ZThn1180_N3RBX7Network16ServerReplicatorD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d8048() -> ! {
    todo!("0x9d8048 __ZThn1180_N3RBX7Network16ServerReplicatorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d80ec — __ZThn1192_N3RBX7Network16ServerReplicatorD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d80ec() -> ! {
    todo!("0x9d80ec __ZThn1192_N3RBX7Network16ServerReplicatorD0Ev")
}

#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d8190 — __ZN3RBX7Network16ServerReplicatorD2Ev
 // type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_9d8190() -> ! {
    todo!("0x9d8190 __ZN3RBX7Network16ServerReplicatorD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d86b4 — __ZThn32_N3RBX7Network16ServerReplicatorD1Ev
 // type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_9d86b4() -> ! {
    todo!("0x9d86b4 __ZThn32_N3RBX7Network16ServerReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d86c0 — __ZThn36_N3RBX7Network16ServerReplicatorD1Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d86c0() -> ! {
    todo!("0x9d86c0 __ZThn36_N3RBX7Network16ServerReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d86cc — __ZThn1180_N3RBX7Network16ServerReplicatorD1Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d86cc() -> ! {
    todo!("0x9d86cc __ZThn1180_N3RBX7Network16ServerReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
// 0x9d86dc — __ZThn1192_N3RBX7Network16ServerReplicatorD1Ev
 // type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
pub fn stub_9d86dc() -> ! {
    todo!("0x9d86dc __ZThn1192_N3RBX7Network16ServerReplicatorD1Ev")
}

#[doc(alias = "RBX::Network::ServerReplicator::readPlayerSimulationRegion(RBX::Region2::WeightedPoint &)")]
// 0x9d8700 — __ZN3RBX7Network16ServerReplicator26readPlayerSimulationRegionERNS_7Region213WeightedPointE
 // type: RBX::PartInstance *__fastcall(_DWORD *, _DWORD *)
pub fn stub_9d8700() -> ! {
    todo!("0x9d8700 __ZN3RBX7Network16ServerReplicator26readPlayerSimulationRegionERNS_7Region213WeightedPointE")
}

#[doc(alias = "RBX::Network::ServerReplicator::onSentMarker(long)")]
// 0x9dbd20 — __ZN3RBX7Network16ServerReplicator12onSentMarkerEl
 // type: int __fastcall(RBX::Network::ServerReplicator *this, int, int, int)
pub fn stub_9dbd20() -> ! {
    todo!("0x9dbd20 __ZN3RBX7Network16ServerReplicator12onSentMarkerEl")
}

#[doc(alias = "RBX::Network::ServerReplicator::installRemotePlayer(std::string)")]
// 0x9dc8e4 — __ZN3RBX7Network16ServerReplicator19installRemotePlayerESs
 // type: void __fastcall(RBX::Instance **, const std::string *)
pub fn stub_9dc8e4() -> ! {
    todo!("0x9dc8e4 __ZN3RBX7Network16ServerReplicator19installRemotePlayerESs")
}

#[doc(alias = "RBX::Network::ServerReplicator::sendItemsPacket(void)")]
// 0x9dcbd8 — __ZN3RBX7Network16ServerReplicator15sendItemsPacketEv
 // type: int __fastcall(RBX::Network::ServerReplicator *this)
pub fn stub_9dcbd8() -> ! {
    todo!("0x9dcbd8 __ZN3RBX7Network16ServerReplicator15sendItemsPacketEv")
}

#[doc(alias = "RBX::Network::ServerReplicator::dataOutStep(void)")]
// 0x9e0098 — __ZN3RBX7Network16ServerReplicator11dataOutStepEv
 // type: void __fastcall(RBX::Network::ServerReplicator *this)
pub fn stub_9e0098() -> ! {
    todo!("0x9e0098 __ZN3RBX7Network16ServerReplicator11dataOutStepEv")
}

#[doc(alias = "RBX::Network::ServerReplicator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x9e16cc — __ZN3RBX7Network16ServerReplicator17onServiceProviderEPNS_15ServiceProviderES3_
 // type: void __fastcall(RBX::Network::ServerReplicator *this, pthread_mutex_t *, pthread_mutex_t *, int)
pub fn stub_9e16cc() -> ! {
    todo!("0x9e16cc __ZN3RBX7Network16ServerReplicator17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv")]
// 0x9e5ab8 — __ZNK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv
 // type: int __fastcall(int, int, int, int)
pub fn stub_9e5ab8() -> ! {
    todo!("0x9e5ab8 __ZNK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv")
}

#[doc(alias = "RBX::Network::ServerReplicator::canSendItems(void)")]
// 0x9e5bc0 — __ZN3RBX7Network16ServerReplicator12canSendItemsEv
 // type: int __fastcall(RBX::Network::ServerReplicator *this)
pub fn stub_9e5bc0() -> ! {
    todo!("0x9e5bc0 __ZN3RBX7Network16ServerReplicator12canSendItemsEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv")]
// 0x9e5bc4 — __ZThn32_NK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv
 // type: int __fastcall(int, int, int, int)
pub fn stub_9e5bc4() -> ! {
    todo!("0x9e5bc4 __ZThn32_NK3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEE12getClassNameEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::FilterResult>::destruct_func(char *)")]
// 0x9e5ef8 — __ZN3rbx14implementation12typed_holderIN3RBX7Network12FilterResultEE13destruct_funcEPc
 // type: void()
pub fn stub_9e5ef8() -> ! {
    todo!("0x9e5ef8 __ZN3rbx14implementation12typed_holderIN3RBX7Network12FilterResultEE13destruct_funcEPc")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network17sServerReplicatorEEEEvv")]
// 0x9e617c — __ZN3RBX4Name13callDoDeclareILZNS_7Network17sServerReplicatorEEEEvv
 // type: void()
pub fn stub_9e617c() -> ! {
    todo!("0x9e617c __ZN3RBX4Name13callDoDeclareILZNS_7Network17sServerReplicatorEEEEvv")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
// 0x9e8718 — __ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
 // type: void __fastcall(struct _Unwind_Exception *, int, int)
pub fn stub_9e8718() -> ! {
    todo!("0x9e8718 __ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
// 0x9e8724 — __ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
 // type: void __fastcall(struct _Unwind_Exception *, int, int)
pub fn stub_9e8724() -> ! {
    todo!("0x9e8724 __ZN3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
// 0x9e87c4 — __ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e87c4() -> ! {
    todo!("0x9e87c4 __ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
// 0x9e87d0 — __ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e87d0() -> ! {
    todo!("0x9e87d0 __ZThn32_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
// 0x9e8874 — __ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e8874() -> ! {
    todo!("0x9e8874 __ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
// 0x9e8880 — __ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e8880() -> ! {
    todo!("0x9e8880 __ZThn36_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")
}

#[doc(alias = "__ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
// 0x9e8924 — __ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e8924() -> ! {
    todo!("0x9e8924 __ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")
}

#[doc(alias = "__ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
// 0x9e8934 — __ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e8934() -> ! {
    todo!("0x9e8934 __ZThn1180_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")
}

#[doc(alias = "__ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")]
// 0x9e89d8 — __ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e89d8() -> ! {
    todo!("0x9e89d8 __ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED1Ev")
}

#[doc(alias = "__ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")]
// 0x9e89e8 — __ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev
 // type: void __fastcall(int, int, int)
pub fn stub_9e89e8() -> ! {
    todo!("0x9e89e8 __ZThn1192_N3RBX17NonFactoryProductINS_7Network10ReplicatorELZNS1_17sServerReplicatorEEED0Ev")
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
// 0x9e9460 — __ZN3RBX7Network10Replicator9StatsItemD2Ev
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
pub fn stub_9e9460() -> ! {
    todo!("0x9e9460 __ZN3RBX7Network10Replicator9StatsItemD2Ev")
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
// 0x9e967c — __ZN3RBX7Network16ServerReplicator15ServerStatsItemD1Ev
 // type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
pub fn stub_9e967c() -> ! {
    todo!("0x9e967c __ZN3RBX7Network16ServerReplicator15ServerStatsItemD1Ev")
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
// 0x9e9688 — __ZN3RBX7Network16ServerReplicator15ServerStatsItemD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
pub fn stub_9e9688() -> ! {
    todo!("0x9e9688 __ZN3RBX7Network16ServerReplicator15ServerStatsItemD0Ev")
}

#[doc(alias = "RBX::Network::ServerReplicator::ServerStatsItem::update(void)")]
// 0x9e9728 — __ZN3RBX7Network16ServerReplicator15ServerStatsItem6updateEv
 // type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *this)
pub fn stub_9e9728() -> ! {
    todo!("0x9e9728 __ZN3RBX7Network16ServerReplicator15ServerStatsItem6updateEv")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
// 0x9e9b30 — __ZThn32_N3RBX7Network16ServerReplicator15ServerStatsItemD1Ev
 // type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
pub fn stub_9e9b30() -> ! {
    todo!("0x9e9b30 __ZThn32_N3RBX7Network16ServerReplicator15ServerStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
// 0x9e9b3c — __ZThn32_N3RBX7Network16ServerReplicator15ServerStatsItemD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
pub fn stub_9e9b3c() -> ! {
    todo!("0x9e9b3c __ZThn32_N3RBX7Network16ServerReplicator15ServerStatsItemD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
// 0x9e9be4 — __ZThn36_N3RBX7Network16ServerReplicator15ServerStatsItemD1Ev
 // type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
pub fn stub_9e9be4() -> ! {
    todo!("0x9e9be4 __ZThn36_N3RBX7Network16ServerReplicator15ServerStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::ServerStatsItem::~ServerStatsItem()")]
// 0x9e9bf0 — __ZThn36_N3RBX7Network16ServerReplicator15ServerStatsItemD0Ev
 // type: void __fastcall(RBX::Network::ServerReplicator::ServerStatsItem *__hidden this)
pub fn stub_9e9bf0() -> ! {
    todo!("0x9e9bf0 __ZThn36_N3RBX7Network16ServerReplicator15ServerStatsItemD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::update(void)")]
// 0x9e9c98 — __ZN3RBX7Network10Replicator9StatsItem6updateEv
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *this)
pub fn stub_9e9c98() -> ! {
    todo!("0x9e9c98 __ZN3RBX7Network10Replicator9StatsItem6updateEv")
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
// 0x9ea2b8 — __ZN3RBX7Network10Replicator9StatsItemD1Ev
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
pub fn stub_9ea2b8() -> ! {
    todo!("0x9ea2b8 __ZN3RBX7Network10Replicator9StatsItemD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::StatsItem::~StatsItem()")]
// 0x9ea2c4 — __ZN3RBX7Network10Replicator9StatsItemD0Ev
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
pub fn stub_9ea2c4() -> ! {
    todo!("0x9ea2c4 __ZN3RBX7Network10Replicator9StatsItemD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem()")]
// 0x9ea364 — __ZThn32_N3RBX7Network10Replicator9StatsItemD1Ev
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
pub fn stub_9ea364() -> ! {
    todo!("0x9ea364 __ZThn32_N3RBX7Network10Replicator9StatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem()")]
// 0x9ea370 — __ZThn32_N3RBX7Network10Replicator9StatsItemD0Ev
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
pub fn stub_9ea370() -> ! {
    todo!("0x9ea370 __ZThn32_N3RBX7Network10Replicator9StatsItemD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem()")]
// 0x9ea414 — __ZThn36_N3RBX7Network10Replicator9StatsItemD1Ev
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
pub fn stub_9ea414() -> ! {
    todo!("0x9ea414 __ZThn36_N3RBX7Network10Replicator9StatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StatsItem::~StatsItem()")]
// 0x9ea420 — __ZThn36_N3RBX7Network10Replicator9StatsItemD0Ev
 // type: void __fastcall(RBX::Network::Replicator::StatsItem *__hidden this)
pub fn stub_9ea420() -> ! {
    todo!("0x9ea420 __ZThn36_N3RBX7Network10Replicator9StatsItemD0Ev")
}

#[doc(alias = "RBX::Network::RakStatsItem::~RakStatsItem()")]
// 0x9eaa38 — __ZN3RBX7Network12RakStatsItemD1Ev
 // type: void __fastcall(RBX::Network::RakStatsItem *__hidden this)
pub fn stub_9eaa38() -> ! {
    todo!("0x9eaa38 __ZN3RBX7Network12RakStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::RakStatsItem::~RakStatsItem()")]
// 0x9eaab0 — __ZThn32_N3RBX7Network12RakStatsItemD1Ev
 // type: void __fastcall(RBX::Network::RakStatsItem *__hidden this)
pub fn stub_9eaab0() -> ! {
    todo!("0x9eaab0 __ZThn32_N3RBX7Network12RakStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::RakStatsItem::~RakStatsItem()")]
// 0x9eab30 — __ZThn32_N3RBX7Network12RakStatsItemD0Ev
 // type: void __fastcall(RBX::Network::RakStatsItem *__hidden this)
pub fn stub_9eab30() -> ! {
    todo!("0x9eab30 __ZThn32_N3RBX7Network12RakStatsItemD0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::FilterResult>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>,std::pair<RBX::Name const* const,RBX::Network::FilterResult> const&)")]
// 0x9febe8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network12FilterResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
 // type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_9febe8() -> ! {
    todo!("0x9febe8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network12FilterResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::FilterResult>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::FilterResult>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::FilterResult> const&)")]
// 0x9fed9c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network12FilterResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
 // type: _Rb_tree_node_base *__fastcall(int, _DWORD *, int *)
pub fn stub_9fed9c() -> ! {
    todo!("0x9fed9c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network12FilterResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::FilterResult*,std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>>,RBX::Network::FilterResult const&)")]
// 0x9fee8c — __ZNSt6vectorIN3RBX7Network12FilterResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
 // type: char *__fastcall(int, char *, _DWORD *)
pub fn stub_9fee8c() -> ! {
    todo!("0x9fee8c __ZNSt6vectorIN3RBX7Network12FilterResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::FilterResult*,std::vector<RBX::Network::FilterResult,std::allocator<RBX::Network::FilterResult>>>,unsigned long,RBX::Network::FilterResult const&)")]
// 0x9fef9c — __ZNSt6vectorIN3RBX7Network12FilterResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
 // type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
pub fn stub_9fef9c() -> ! {
    todo!("0x9fef9c __ZNSt6vectorIN3RBX7Network12FilterResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "rbx::timestamped_safe_queue<RBX::Network::PropSync::detail::PropertyKey>::push(RBX::Network::PropSync::detail::PropertyKey const&)")]
// 0x9ff144 — __ZN3rbx22timestamped_safe_queueIN3RBX7Network8PropSync6detail11PropertyKeyEE4pushERKS5_
 // type: void __fastcall(int, __int64 *, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_9ff144() -> ! {
    todo!("0x9ff144 __ZN3rbx22timestamped_safe_queueIN3RBX7Network8PropSync6detail11PropertyKeyEE4pushERKS5_")
}

#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>>>::_M_reallocate_map(unsigned long,bool)")]
// 0x9ff2a8 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network8PropSync6detail11PropertyKeyEEESaIS8_EE17_M_reallocate_mapEmb
 // type: char *__fastcall(void **, unsigned int, int)
pub fn stub_9ff2a8() -> ! {
    todo!("0x9ff2a8 __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network8PropSync6detail11PropertyKeyEEESaIS8_EE17_M_reallocate_mapEmb")
}

