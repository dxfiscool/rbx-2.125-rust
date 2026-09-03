//! network generated_network_next107 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator|RakPeer|BitStream (5109 matched, 4099 in global set, this shard: 100 of 1010 fresh EA-sorted asc)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: 100 stubs | range 0xa7d2ac..0xb23cd8 | rbx_core::SharedPtr (not boost::shared_ptr) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xa7d2ac — __ZN14DataStructures10MemoryPoolIN6RakNet6PacketEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::Packet>::Allocate(char const*,unsigned int)")]
pub fn stub_a7d2ac() -> ! {
    todo!("0xa7d2ac DataStructures::MemoryPool<RakNet::Packet>::Allocate(char const*,unsigned int)")
}

// 0xa7d3d8 — __ZN14DataStructures10MemoryPoolIN6RakNet6PacketEE7ReleaseEPS2_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::Packet>::Release(RakNet::Packet*,char const*,unsigned int)")]
pub fn stub_a7d3d8() -> ! {
    todo!("0xa7d3d8 DataStructures::MemoryPool<RakNet::Packet>::Release(RakNet::Packet*,char const*,unsigned int)")
}

// 0xad5300 — __ZN6RakNet16PluginInterface216OnRakPeerStartupEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::OnRakPeerStartup(void)")]
pub fn stub_ad5300() -> ! {
    todo!("0xad5300 RakNet::PluginInterface2::OnRakPeerStartup(void)")
}

// 0xad5308 — __ZN6RakNet16PluginInterface218OnClosedConnectionERKNS_13SystemAddressENS_10RakNetGUIDENS_24PI2_LostConnectionReasonE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnClosedConnection(RakNet::SystemAddress const&,RakNet::RakNetGUID,RakNet::PI2_LostConnectionReason)")]
pub fn stub_ad5308() -> ! {
    todo!("0xad5308 RakNet::PluginInterface2::OnClosedConnection(RakNet::SystemAddress const&,RakNet::RakNetGUID,RakNet::PI2_LostConnectionReason)")
}

// 0xad5310 — __ZN6RakNet16PluginInterface225OnFailedConnectionAttemptEPNS_6PacketENS_33PI2_FailedConnectionAttemptReasonE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnFailedConnectionAttempt(RakNet::Packet *,RakNet::PI2_FailedConnectionAttemptReason)")]
pub fn stub_ad5310() -> ! {
    todo!("0xad5310 RakNet::PluginInterface2::OnFailedConnectionAttempt(RakNet::Packet *,RakNet::PI2_FailedConnectionAttemptReason)")
}

// 0xad5314 — __ZNK6RakNet16PluginInterface220UsesReliabilityLayerEv
// type: int __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::UsesReliabilityLayer(void)const")]
pub fn stub_ad5314() -> ! {
    todo!("0xad5314 RakNet::PluginInterface2::UsesReliabilityLayer(void)const")
}

// 0xad5318 — __ZN6RakNet16PluginInterface218OnDirectSocketSendEPKcjNS_13SystemAddressE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnDirectSocketSend(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_ad5318() -> ! {
    todo!("0xad5318 RakNet::PluginInterface2::OnDirectSocketSend(char const*,unsigned int,RakNet::SystemAddress)")
}

// 0xad5320 — __ZN6RakNet16PluginInterface229OnReliabilityLayerPacketErrorEPKcjNS_13SystemAddressE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnReliabilityLayerPacketError(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_ad5320() -> ! {
    todo!("0xad5320 RakNet::PluginInterface2::OnReliabilityLayerPacketError(char const*,unsigned int,RakNet::SystemAddress)")
}

// 0xad5324 — __ZN6RakNet16PluginInterface216OnInternalPacketEPNS_14InternalPacketEjNS_13SystemAddressEji
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")]
pub fn stub_ad5324() -> ! {
    todo!("0xad5324 RakNet::PluginInterface2::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")
}

// 0xae10a8 — __ZNK3RBX7Network10Replicator14getRakNetStatsEv
// type: char *__fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::getRakNetStats(void)const")]
pub fn stub_ae10a8() -> ! {
    todo!("0xae10a8 RBX::Network::Replicator::getRakNetStats(void)const")
}

// 0xae10b8 — __ZN3RBX7Network10ReplicatorC2EN6RakNet13SystemAddressEN5boost10shared_ptrINS0_17ConcurrentRakPeerEEEPNS_15NetworkSettingsEb
// type: RBX::Network::IdSerializer *__fastcall(RBX::Network::IdSerializer *, unsigned int, unsigned int, unsigned int, unsigned int, int, int *, int, char)
#[doc(alias = "RBX::Network::Replicator::Replicator(RakNet::SystemAddress,boost::shared_ptr<RBX::Network::ConcurrentRakPeer>,RBX::NetworkSettings *,bool)")]
pub fn stub_ae10b8() -> ! {
    todo!("0xae10b8 RBX::Network::Replicator::Replicator(RakNet::SystemAddress,boost::shared_ptr<RBX::Network::ConcurrentRakPeer>,RBX::NetworkSettings *,bool)")
}

// 0xae1f8c — __ZN3RBX7Network10Replicator18pushIncomingPacketEPN6RakNet6PacketE
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::pushIncomingPacket(RakNet::Packet *)")]
pub fn stub_ae1f8c() -> ! {
    todo!("0xae1f8c RBX::Network::Replicator::pushIncomingPacket(RakNet::Packet *)")
}

// 0xaec7d4 — __ZN3RBX7Network10Replicator23sendFilteredChatMessageERKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_
// type: void __fastcall(struct _Unwind_Exception *, int, int *, _DWORD *, int)
#[doc(alias = "RBX::Network::Replicator::sendFilteredChatMessage(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")]
pub fn stub_aec7d4() -> ! {
    todo!("0xaec7d4 RBX::Network::Replicator::sendFilteredChatMessage(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")
}

// 0xaff534 — __ZN3RBX7Network10Replicator8readItemERN6RakNet9BitStreamENS0_4Item8ItemTypeE
// type: void __fastcall(RBX::Network::Replicator *, const void **, const char *)
#[doc(alias = "RBX::Network::Replicator::readItem(RakNet::BitStream &,RBX::Network::Item::ItemType)")]
pub fn stub_aff534() -> ! {
    todo!("0xaff534 RBX::Network::Replicator::readItem(RakNet::BitStream &,RBX::Network::Item::ItemType)")
}

// 0xb002f0 — __ZN3RBX7Network10Replicator19readChangedPropertyERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::readChangedProperty(RakNet::BitStream &)")]
pub fn stub_b002f0() -> ! {
    todo!("0xb002f0 RBX::Network::Replicator::readChangedProperty(RakNet::BitStream &)")
}

// 0xb009cc — __ZN3RBX7Network10Replicator10readMarkerERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *, int, int)
#[doc(alias = "RBX::Network::Replicator::readMarker(RakNet::BitStream &)")]
pub fn stub_b009cc() -> ! {
    todo!("0xb009cc RBX::Network::Replicator::readMarker(RakNet::BitStream &)")
}

// 0xb00e44 — __ZN3RBX7Network10Replicator12readDataPingERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::readDataPing(RakNet::BitStream &)")]
pub fn stub_b00e44() -> ! {
    todo!("0xb00e44 RBX::Network::Replicator::readDataPing(RakNet::BitStream &)")
}

// 0xb0107c — __ZN3RBX7Network10Replicator19readEventInvocationERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::readEventInvocation(RakNet::BitStream &)")]
pub fn stub_b0107c() -> ! {
    todo!("0xb0107c RBX::Network::Replicator::readEventInvocation(RakNet::BitStream &)")
}

// 0xb01e04 — __ZN3RBX7Network10Replicator12readJoinDataERN6RakNet9BitStreamE
// type: unsigned int __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::readJoinData(RakNet::BitStream &)")]
pub fn stub_b01e04() -> ! {
    todo!("0xb01e04 RBX::Network::Replicator::readJoinData(RakNet::BitStream &)")
}

// 0xb02984 — __ZN3RBX7Network10Replicator13processPacketEPN6RakNet6PacketE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::Replicator::processPacket(RakNet::Packet *)")]
pub fn stub_b02984() -> ! {
    todo!("0xb02984 RBX::Network::Replicator::processPacket(RakNet::Packet *)")
}

// 0xb02e30 — __ZN3RBX7Network10Replicator9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(RBX::Network::Replicator *, RakNet::SystemAddress *)
#[doc(alias = "RBX::Network::Replicator::OnReceive(RakNet::Packet *)")]
pub fn stub_b02e30() -> ! {
    todo!("0xb02e30 RBX::Network::Replicator::OnReceive(RakNet::Packet *)")
}

// 0xb04818 — __ZThn1180_N3RBX7Network10Replicator9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(int, RakNet::SystemAddress *)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::OnReceive(RakNet::Packet *)")]
pub fn stub_b04818() -> ! {
    todo!("0xb04818 non-virtual thunk toRBX::Network::Replicator::OnReceive(RakNet::Packet *)")
}

// 0xb04828 — __ZN3RBX7Network10Replicator16OnInternalPacketEPN6RakNet14InternalPacketEjNS2_13SystemAddressEji
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")]
pub fn stub_b04828() -> ! {
    todo!("0xb04828 RBX::Network::Replicator::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")
}

// 0xb04a98 — __ZThn1180_N3RBX7Network10Replicator16OnInternalPacketEPN6RakNet14InternalPacketEjNS2_13SystemAddressEji
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")]
pub fn stub_b04a98() -> ! {
    todo!("0xb04a98 non-virtual thunk toRBX::Network::Replicator::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")
}

// 0xb07980 — __ZN5boost4bindIvN3RBX7Network10ReplicatorERKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_NS8_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEENSG_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISN_T0_T1_T2_T3_T4_EENSL_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSQ_FSN_SR_SS_ST_SU_ESX_SY_SZ_S10_S11_
// type: void __fastcall(int, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&,boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Network::Replicator::*)(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&),boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_b07980() -> ! {
    todo!("0xb07980 boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&,boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Network::Replicator::*)(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&),boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0xb08aa8 — __ZN3RBX7Network16SenderDictionaryINS_13SystemAddressEE4sendERN6RakNet9BitStreamERKS2_
// type: unsigned int __fastcall(int, RakNet::BitStream *this, int *)
#[doc(alias = "RBX::Network::SenderDictionary<RBX::SystemAddress>::send(RakNet::BitStream &,RBX::SystemAddress const&)")]
pub fn stub_b08aa8() -> ! {
    todo!("0xb08aa8 RBX::Network::SenderDictionary<RBX::SystemAddress>::send(RakNet::BitStream &,RBX::SystemAddress const&)")
}

// 0xb0ceb4 — __ZN3RBX7Network10Replicator16serializeSFFlagsERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::serializeSFFlags(RakNet::BitStream &)")]
pub fn stub_b0ceb4() -> ! {
    todo!("0xb0ceb4 RBX::Network::Replicator::serializeSFFlags(RakNet::BitStream &)")
}

// 0xb0ceb8 — __ZN3RBX7Network10Replicator18deserializeSFFlagsERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::deserializeSFFlags(RakNet::BitStream &)")]
pub fn stub_b0ceb8() -> ! {
    todo!("0xb0ceb8 RBX::Network::Replicator::deserializeSFFlags(RakNet::BitStream &)")
}

// 0xb0ced0 — __ZN6RakNet16PluginInterface28OnAttachEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::OnAttach(void)")]
pub fn stub_b0ced0() -> ! {
    todo!("0xb0ced0 RakNet::PluginInterface2::OnAttach(void)")
}

// 0xb0ced8 — __ZN6RakNet16PluginInterface26UpdateEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::Update(void)")]
pub fn stub_b0ced8() -> ! {
    todo!("0xb0ced8 RakNet::PluginInterface2::Update(void)")
}

// 0xb0cee0 — __ZN6RakNet16PluginInterface217OnRakPeerShutdownEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::OnRakPeerShutdown(void)")]
pub fn stub_b0cee0() -> ! {
    todo!("0xb0cee0 RakNet::PluginInterface2::OnRakPeerShutdown(void)")
}

// 0xb0cee8 — __ZN6RakNet16PluginInterface215OnNewConnectionERKNS_13SystemAddressENS_10RakNetGUIDEb
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnNewConnection(RakNet::SystemAddress const&,RakNet::RakNetGUID,bool)")]
pub fn stub_b0cee8() -> ! {
    todo!("0xb0cee8 RakNet::PluginInterface2::OnNewConnection(RakNet::SystemAddress const&,RakNet::RakNetGUID,bool)")
}

// 0xb0cef8 — __ZN6RakNet16PluginInterface221OnDirectSocketReceiveEPKcjNS_13SystemAddressE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnDirectSocketReceive(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_b0cef8() -> ! {
    todo!("0xb0cef8 RakNet::PluginInterface2::OnDirectSocketReceive(char const*,unsigned int,RakNet::SystemAddress)")
}

// 0xb0cf00 — __ZN6RakNet16PluginInterface25OnAckEjNS_13SystemAddressEj
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnAck(unsigned int,RakNet::SystemAddress,unsigned int)")]
pub fn stub_b0cf00() -> ! {
    todo!("0xb0cf00 RakNet::PluginInterface2::OnAck(unsigned int,RakNet::SystemAddress,unsigned int)")
}

// 0xb0db10 — __ZN3RBX7Network10Replicator14SendClusterJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")]
pub fn stub_b0db10() -> ! {
    todo!("0xb0db10 RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")
}

// 0xb0dbdc — __ZN3RBX7Network10Replicator14SendClusterJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")]
pub fn stub_b0dbdc() -> ! {
    todo!("0xb0dbdc RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")
}

// 0xb0dcbc — __ZN3RBX7Network10Replicator14SendClusterJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b0dcbc() -> ! {
    todo!("0xb0dcbc RBX::Network::Replicator::SendClusterJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xb0dfd8 — __ZN3RBX7Network13ReplicatorJobD1Ev
// type: void __fastcall(RBX::Network::ReplicatorJob *__hidden this)
#[doc(alias = "RBX::Network::ReplicatorJob::~ReplicatorJob()")]
pub fn stub_b0dfd8() -> ! {
    todo!("0xb0dfd8 RBX::Network::ReplicatorJob::~ReplicatorJob()")
}

// 0xb139fc — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE16_M_push_back_auxERKS5_
// type: void __fastcall(_DWORD *, int *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::deque<boost::shared_ptr<RBX::Network::Marker>,std::allocator<boost::shared_ptr<RBX::Network::Marker>>>::_M_push_back_aux(boost::shared_ptr<RBX::Network::Marker> const&)")]
pub fn stub_b139fc() -> ! {
    todo!("0xb139fc std::deque<boost::shared_ptr<RBX::Network::Marker>,std::allocator<boost::shared_ptr<RBX::Network::Marker>>>::_M_push_back_aux(boost::shared_ptr<RBX::Network::Marker> const&)")
}

// 0xb13d44 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
#[doc(alias = "std::deque<boost::shared_ptr<RBX::Network::Marker>,std::allocator<boost::shared_ptr<RBX::Network::Marker>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_b13d44() -> ! {
    todo!("0xb13d44 std::deque<boost::shared_ptr<RBX::Network::Marker>,std::allocator<boost::shared_ptr<RBX::Network::Marker>>>::_M_reallocate_map(unsigned long,bool)")
}

// 0xb14fe0 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>)")]
pub fn stub_b14fe0() -> ! {
    todo!("0xb14fe0 boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>)")
}

// 0xb15f50 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE11encodeCellsINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
// type: unsigned int __fastcall(int, const G3D::Vector3int16 *, int, RakNet::BitStream *, signed int)
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b15f50() -> ! {
    todo!("0xb15f50 void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *,int)const")
}

// 0xb173b0 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE11encodeCellsINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
// type: unsigned int __fastcall(int, const G3D::Vector3int16 *, RBX::Network::ClusterUpdateBuffer *, RakNet::BitStream *, signed int)
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *,int)const")]
pub fn stub_b173b0() -> ! {
    todo!("0xb173b0 void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *,int)const")
}

// 0xb18564 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE11encodeCellsINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
// type: unsigned int __fastcall(G3D::Vector3int16 *, const G3D::Vector3int16 *, int *, RakNet::BitStream *, signed int)
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::ClusterCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b18564() -> ! {
    todo!("0xb18564 void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::ClusterCellIterator &,RakNet::BitStream *,int)const")
}

// 0xb1c5cc — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::list4(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_b1c5cc() -> ! {
    todo!("0xb1c5cc boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::list4(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")
}

// 0xb1c790 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_b1c790() -> ! {
    todo!("0xb1c790 boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")
}

// 0xb1c954 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEEEC2ES8_SB_SD_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>)")]
pub fn stub_b1c954() -> ! {
    todo!("0xb1c954 boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>)")
}

// 0xb1cb18 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEEEC2ES8_SB_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>)")]
pub fn stub_b1cb18() -> ! {
    todo!("0xb1cb18 boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>)")
}

// 0xb2058c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network31SharedStringProtectedDictionaryES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::SharedStringProtectedDictionary,RBX::Network::SharedStringProtectedDictionary>(boost::shared_ptr<RBX::Network::SharedStringProtectedDictionary> *,RBX::Network::SharedStringProtectedDictionary *,boost::detail::shared_count &)")]
pub fn stub_b2058c() -> ! {
    todo!("0xb2058c void boost::detail::sp_pointer_construct<RBX::Network::SharedStringProtectedDictionary,RBX::Network::SharedStringProtectedDictionary>(boost::shared_ptr<RBX::Network::SharedStringProtectedDictionary> *,RBX::Network::SharedStringProtectedDictionary *,boost::detail::shared_count &)")
}

// 0xb20850 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20850() -> ! {
    todo!("0xb20850 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")
}

// 0xb20854 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20854() -> ! {
    todo!("0xb20854 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")
}

// 0xb20860 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::dispose(void)")]
pub fn stub_b20860() -> ! {
    todo!("0xb20860 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::dispose(void)")
}

// 0xb209b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_deleter(std::type_info const&)")]
pub fn stub_b209b0() -> ! {
    todo!("0xb209b0 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_deleter(std::type_info const&)")
}

// 0xb209b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_untyped_deleter(void)")]
pub fn stub_b209b4() -> ! {
    todo!("0xb209b4 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_untyped_deleter(void)")
}

// 0xb20d10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20d10() -> ! {
    todo!("0xb20d10 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")
}

// 0xb20d14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20d14() -> ! {
    todo!("0xb20d14 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")
}

// 0xb20d20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::dispose(void)")]
pub fn stub_b20d20() -> ! {
    todo!("0xb20d20 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::dispose(void)")
}

// 0xb20e64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_deleter(std::type_info const&)")]
pub fn stub_b20e64() -> ! {
    todo!("0xb20e64 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_deleter(std::type_info const&)")
}

// 0xb20e68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_untyped_deleter(void)")]
pub fn stub_b20e68() -> ! {
    todo!("0xb20e68 boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_untyped_deleter(void)")
}

// 0xb20e6c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23TopNErrorsPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::TopNErrorsPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::TopNErrorsPhysicsSender *,boost::detail::shared_count &)")]
pub fn stub_b20e6c() -> ! {
    todo!("0xb20e6c void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::TopNErrorsPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::TopNErrorsPhysicsSender *,boost::detail::shared_count &)")
}

// 0xb21004 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21004() -> ! {
    todo!("0xb21004 boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")
}

// 0xb21008 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21008() -> ! {
    todo!("0xb21008 boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")
}

// 0xb21014 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::dispose(void)")]
pub fn stub_b21014() -> ! {
    todo!("0xb21014 boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::dispose(void)")
}

// 0xb21028 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_deleter(std::type_info const&)")]
pub fn stub_b21028() -> ! {
    todo!("0xb21028 boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_deleter(std::type_info const&)")
}

// 0xb2102c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_untyped_deleter(void)")]
pub fn stub_b2102c() -> ! {
    todo!("0xb2102c boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_untyped_deleter(void)")
}

// 0xb21030 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::RoundRobinPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::RoundRobinPhysicsSender *,boost::detail::shared_count &)")]
pub fn stub_b21030() -> ! {
    todo!("0xb21030 void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::RoundRobinPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::RoundRobinPhysicsSender *,boost::detail::shared_count &)")
}

// 0xb211c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b211c8() -> ! {
    todo!("0xb211c8 boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")
}

// 0xb211cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b211cc() -> ! {
    todo!("0xb211cc boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")
}

// 0xb211d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::dispose(void)")]
pub fn stub_b211d8() -> ! {
    todo!("0xb211d8 boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::dispose(void)")
}

// 0xb211ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_deleter(std::type_info const&)")]
pub fn stub_b211ec() -> ! {
    todo!("0xb211ec boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_deleter(std::type_info const&)")
}

// 0xb211f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_untyped_deleter(void)")]
pub fn stub_b211f0() -> ! {
    todo!("0xb211f0 boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_untyped_deleter(void)")
}

// 0xb211f4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender2>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender2 *,boost::detail::shared_count &)")]
pub fn stub_b211f4() -> ! {
    todo!("0xb211f4 void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender2>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender2 *,boost::detail::shared_count &)")
}

// 0xb2138c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
pub fn stub_b2138c() -> ! {
    todo!("0xb2138c boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")
}

// 0xb21390 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
pub fn stub_b21390() -> ! {
    todo!("0xb21390 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")
}

// 0xb2139c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::dispose(void)")]
pub fn stub_b2139c() -> ! {
    todo!("0xb2139c boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::dispose(void)")
}

// 0xb213b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_deleter(std::type_info const&)")]
pub fn stub_b213b0() -> ! {
    todo!("0xb213b0 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_deleter(std::type_info const&)")
}

// 0xb213b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_untyped_deleter(void)")]
pub fn stub_b213b4() -> ! {
    todo!("0xb213b4 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_untyped_deleter(void)")
}

// 0xb213b8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender *,boost::detail::shared_count &)")]
pub fn stub_b213b8() -> ! {
    todo!("0xb213b8 void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender *,boost::detail::shared_count &)")
}

// 0xb21550 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21550() -> ! {
    todo!("0xb21550 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")
}

// 0xb21554 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21554() -> ! {
    todo!("0xb21554 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")
}

// 0xb21560 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::dispose(void)")]
pub fn stub_b21560() -> ! {
    todo!("0xb21560 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::dispose(void)")
}

// 0xb21574 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_deleter(std::type_info const&)")]
pub fn stub_b21574() -> ! {
    todo!("0xb21574 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_deleter(std::type_info const&)")
}

// 0xb21578 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_untyped_deleter(void)")]
pub fn stub_b21578() -> ! {
    todo!("0xb21578 boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_untyped_deleter(void)")
}

// 0xb21844 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSEPSH_
// type: int32_t **__fastcall(int32_t **, int32_t *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot*)")]
pub fn stub_b21844() -> ! {
    todo!("0xb21844 boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot*)")
}

// 0xb218f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSERKSI_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot> const&)")]
pub fn stub_b218f8() -> ! {
    todo!("0xb218f8 boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot> const&)")
}

// 0xb21bec — __ZNK3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::connected(void)const")]
pub fn stub_b21bec() -> ! {
    todo!("0xb21bec rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::connected(void)const")
}

// 0xb21bf8 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")]
pub fn stub_b21bf8() -> ! {
    todo!("0xb21bf8 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")
}

// 0xb21c28 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")]
pub fn stub_b21c28() -> ! {
    todo!("0xb21c28 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")
}

// 0xb21c58 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE6removeEPNSF_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::remove(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot *)")]
pub fn stub_b21c58() -> ! {
    todo!("0xb21c58 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::remove(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot *)")
}

// 0xb21d44 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_b21d44() -> ! {
    todo!("0xb21d44 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::safe_static_init_mutex(void)")
}

// 0xb21e28 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
pub fn stub_b21e28() -> ! {
    todo!("0xb21e28 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")
}

// 0xb21fa4 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
pub fn stub_b21fa4() -> ! {
    todo!("0xb21fa4 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")
}

// 0xb21fb0 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
pub fn stub_b21fb0() -> ! {
    todo!("0xb21fb0 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")
}

// 0xb22064 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")]
pub fn stub_b22064() -> ! {
    todo!("0xb22064 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")
}

// 0xb220c0 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")]
pub fn stub_b220c0() -> ! {
    todo!("0xb220c0 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")
}

// 0xb221c8 — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list5(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_b221c8() -> ! {
    todo!("0xb221c8 boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list5(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")
}

// 0xb22618 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_b22618() -> ! {
    todo!("0xb22618 boost::_bi::storage4<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0xb22a68 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>)")]
pub fn stub_b22a68() -> ! {
    todo!("0xb22a68 boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>)")
}

// 0xb2332c — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
pub fn stub_b2332c() -> ! {
    todo!("0xb2332c __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")
}

// 0xb23cd8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(boost::shared_ptr<RBX::Network::Replicator::PingJob> *,RBX::Network::Replicator::PingJob *,boost::detail::shared_count &)")]
pub fn stub_b23cd8() -> ! {
    todo!("0xb23cd8 void boost::detail::sp_pointer_construct<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(boost::shared_ptr<RBX::Network::Replicator::PingJob> *,RBX::Network::Replicator::PingJob *,boost::detail::shared_count &)")
}
