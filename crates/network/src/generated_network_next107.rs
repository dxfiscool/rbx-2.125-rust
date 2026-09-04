//! network generated_network_next107 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator|RakPeer|BitStream (5109 matched, 4099 in global set, this shard: 100 of 1010 fresh EA-sorted asc)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: 100 stubs | range 0xa7d2ac..0xb23cd8 | rbx_core::SharedPtr (not boost::shared_ptr) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xa7d2ac — __ZN14DataStructures10MemoryPoolIN6RakNet6PacketEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::Packet>::Allocate(char const*,unsigned int)")]
pub fn stub_a7d2ac() -> crate::socket::Packet {
 // IDA 0xa7d2ac: pool blocks stay engine-side.
 crate::socket::packet_allocate()
}

// 0xa7d3d8 — __ZN14DataStructures10MemoryPoolIN6RakNet6PacketEE7ReleaseEPS2_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::Packet>::Release(RakNet::Packet*,char const*,unsigned int)")]
pub fn stub_a7d3d8(packet: crate::socket::Packet) {
 // IDA 0xa7d3d8: return to the pool (drop).
 crate::socket::packet_release(packet);
}

// 0xad5300 — __ZN6RakNet16PluginInterface216OnRakPeerStartupEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::OnRakPeerStartup(void)")]
pub fn stub_ad5300(plugin: &crate::socket::PluginInterface2) {
 // IDA 0xad5300: default hook is empty.
 plugin.on_rak_peer_startup();
}

// 0xad5308 — __ZN6RakNet16PluginInterface218OnClosedConnectionERKNS_13SystemAddressENS_10RakNetGUIDENS_24PI2_LostConnectionReasonE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnClosedConnection(RakNet::SystemAddress const&,RakNet::RakNetGUID,RakNet::PI2_LostConnectionReason)")]
pub fn stub_ad5308(plugin: &crate::socket::PluginInterface2) {
 // IDA 0xad5308: default hook is empty.
 plugin.on_closed_connection();
}

// 0xad5310 — __ZN6RakNet16PluginInterface225OnFailedConnectionAttemptEPNS_6PacketENS_33PI2_FailedConnectionAttemptReasonE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnFailedConnectionAttempt(RakNet::Packet *,RakNet::PI2_FailedConnectionAttemptReason)")]
pub fn stub_ad5310(plugin: &crate::socket::PluginInterface2) {
 // IDA 0xad5310: default hook is empty.
 plugin.on_failed_connection_attempt();
}

// 0xad5314 — __ZNK6RakNet16PluginInterface220UsesReliabilityLayerEv
// type: int __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::UsesReliabilityLayer(void)const")]
pub fn stub_ad5314(plugin: &crate::socket::PluginInterface2) -> bool {
 // IDA 0xad5314: MOVS R0, #0.
 plugin.uses_reliability_layer()
}

// 0xad5318 — __ZN6RakNet16PluginInterface218OnDirectSocketSendEPKcjNS_13SystemAddressE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnDirectSocketSend(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_ad5318(plugin: &crate::socket::PluginInterface2) {
 // IDA 0xad5318: default hook is empty.
 plugin.on_direct_socket_send();
}

// 0xad5320 — __ZN6RakNet16PluginInterface229OnReliabilityLayerPacketErrorEPKcjNS_13SystemAddressE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnReliabilityLayerPacketError(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_ad5320(plugin: &crate::socket::PluginInterface2) {
 // IDA 0xad5320: default hook is empty.
 plugin.on_reliability_layer_packet_error();
}

// 0xad5324 — __ZN6RakNet16PluginInterface216OnInternalPacketEPNS_14InternalPacketEjNS_13SystemAddressEji
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")]
pub fn stub_ad5324(plugin: &crate::socket::PluginInterface2) {
 // IDA 0xad5324: bare BX LR.
 plugin.on_internal_packet();
}

// 0xae10a8 — __ZNK3RBX7Network10Replicator14getRakNetStatsEv
// type: char *__fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::getRakNetStats(void)const")]
pub fn stub_ae10a8(flag: u32) -> bool {
 // IDA 0xae10a8: null unless the +0x4b0 flag is set.
 crate::replicator::has_rak_net_stats(flag)
}

pub fn stub_ae10b8(stats_hook: bool) -> crate::replicator::ReplicatorInit {
 // IDA 0xae10b8: vtable/descriptorBucket/queue/rolling-window init above;
 // mutex, pool, and DataModel wiring stay engine-side.
 crate::replicator::replicator_init(stats_hook)
}

// 0xae1f8c — __ZN3RBX7Network10Replicator18pushIncomingPacketEPN6RakNet6PacketE
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::pushIncomingPacket(RakNet::Packet *)")]
pub fn stub_ae1f8c(
    queue: &mut std::collections::VecDeque<crate::replicator::TimestampedPacket>,
    time: f64,
    packet: u32,
    scheduler: u32,
    rescheduler: u32,
    reschedule: &mut dyn FnMut(),
) {
    // IDA 0xae1f8c: timestamp + push under the +3628 mutex, reschedule the +1568 job on match.
    crate::replicator::push_incoming_packet(queue, time, packet, scheduler, rescheduler, reschedule);
}

pub fn stub_aec7d4(
    dest_is_self: bool,
    has_player: bool,
    filter_type: u32,
    mapping_empty: bool,
    text_empty: bool,
) -> Option<crate::replicator::ChatFilterWrite> {
    // IDA 0xaec7d4: self-address/filter drops, then original-vs-filtered
    // pick; the bitstream copy and priority-1/rel-2 Send stay engine-side.
    crate::replicator::send_filtered_chat_decision(
        dest_is_self,
        has_player,
        filter_type,
        mapping_empty,
        text_empty,
    )
}

// 0xaff534 — __ZN3RBX7Network10Replicator8readItemERN6RakNet9BitStreamENS0_4Item8ItemTypeE
// type: void __fastcall(RBX::Network::Replicator *, const void **, const char *)
#[doc(alias = "RBX::Network::Replicator::readItem(RakNet::BitStream &,RBX::Network::Item::ItemType)")]
pub fn stub_aff534(item_type: u32) -> crate::replicator::ReplicatorItemTarget {
    // IDA 0xaff534: switch on the ItemType; the FLog trace and per-arm readers stay engine-side.
    crate::replicator::replicator_read_item_target(item_type)
}

pub fn stub_b002f0(
    stream: &mut crate::bitstream::BitStream,
    descriptor_bits: u8,
    descriptor_count: usize,
    instance_present: bool,
    filtered: bool,
    legal_prop: bool,
) -> Option<usize> {
    // IDA 0xb002f0: index bits + range check, legality gate clears the
    // instance; deserializeInstanceRef/logging/deserializeProperty stay
    // engine-side.
    let index = crate::replicator::changed_property_index(stream, descriptor_bits, descriptor_count);
    crate::replicator::changed_property_keep(instance_present, filtered, legal_prop).then_some(index)
}

pub fn stub_b009cc(
    stream: &mut crate::bitstream::BitStream,
    front: Option<i32>,
    chunk_defer_voxel_updates: bool,
    fire: &mut dyn FnMut(),
    done_loading: &mut dyn FnMut(),
) {
    // IDA 0xb009cc: long id read, verbose + fast logs, front-id assert
    // then fireReturned/pop; terrain tail gated on the chunk-defer flag.
    let id = crate::replicator::read_marker_id(stream);
    assert!(
        crate::replicator::marker_front_matches(front, id),
        "id==incomingMarkers.front()->id"
    );
    fire();
    if crate::replicator::should_done_loading_terrain(chunk_defer_voxel_updates) {
        done_loading();
    }
}

// 0xb00e44 — __ZN3RBX7Network10Replicator12readDataPingERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::readDataPing(RakNet::BitStream &)")]
pub fn stub_b00e44(
    stream: &mut crate::bitstream::BitStream,
    now_ms: u32,
    stamp: &mut crate::replicator::PingStamp,
    ping_back: &mut dyn FnMut(crate::replicator::DataPing),
) -> crate::replicator::DataPingAction {
    // IDA 0xb00e44: bool/u64/u32 wire reads, virtual +308, sample-or-queue, stamp + stats.
    let ping = crate::replicator::read_data_ping(stream);
    let action = crate::replicator::data_ping_action(&ping, now_ms);
    if action == crate::replicator::DataPingAction::QueuePingBack {
        ping_back(ping);
    }
    crate::replicator::stamp_data_ping(stamp, now_ms);
    action
}
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::readEventInvocation(RakNet::BitStream &)")]
pub fn stub_b0107c(
    stream: &mut crate::bitstream::BitStream,
    event_bits: u8,
    event_count: usize,
    legal_receive: bool,
    invoke: &mut dyn FnMut(usize),
) {
    // IDA 0xb0107c: event index bits + range check, +208 legality gate,
    // then deserializeEventInvocation + fire engine-side.
    let index = crate::replicator::read_event_index(stream, event_bits, event_count);
    if crate::replicator::event_keep(legal_receive) {
        invoke(index);
    }
}

// 0xb01e04 — __ZN3RBX7Network10Replicator12readJoinDataERN6RakNet9BitStreamE
// type: unsigned int __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::readJoinData(RakNet::BitStream &)")]
pub fn stub_b01e04(stream: &mut crate::bitstream::BitStream, packed: bool) -> u32 {
    // IDA 0xb01e04: packed 5-bit groups (+3720 flag) or a plain u32; decompress + readInstanceNew loop engine-side.
    crate::replicator::join_data_instance_count(stream, packed)
}

// 0xb02984 — __ZN3RBX7Network10Replicator13processPacketEPN6RakNet6PacketE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::Replicator::processPacket(RakNet::Packet *)")]
pub fn stub_b02984(
    first_byte: u8,
    has_physics: bool,
    inner_marker: u8,
) -> crate::replicator::ProcessPacketKind {
    // IDA 0xb02984: first-byte dispatch; the physics arm asserts the
    // inner ID_PHYSICS byte, the BitStream wrap and per-arm readers stay
    // engine-side.
    let kind = crate::replicator::process_packet_kind(first_byte, has_physics);
    if kind == crate::replicator::ProcessPacketKind::Physics {
        assert!(
            crate::replicator::physics_inner_valid(inner_marker),
            "id==ID_PHYSICS"
        );
    }
    kind
}
// 0xb02e30 — __ZN3RBX7Network10Replicator9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(RBX::Network::Replicator *, RakNet::SystemAddress *)
#[doc(alias = "RBX::Network::Replicator::OnReceive(RakNet::Packet *)")]
pub fn stub_b02e30(
    address_matches: bool,
    first_byte: u8,
    forwarded: Option<bool>,
) -> u32 {
    // IDA 0xb02e30: foreign addresses return 1; otherwise the first-byte
    // arm runs (schema/chat/disconnect/physics engine-side) with the
    // verdict above.
    if !address_matches {
        return 1;
    }
    let action = crate::replicator::on_receive_action(first_byte);
    crate::replicator::on_receive_verdict(action, forwarded)
}
// 0xb04818 — __ZThn1180_N3RBX7Network10Replicator9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(int, RakNet::SystemAddress *)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::OnReceive(RakNet::Packet *)")]
pub fn stub_b04818(
    address_matches: bool,
    first_byte: u8,
    forwarded: Option<bool>,
) -> u32 {
    // IDA 0xb04818: __ZThn1180 — this -= 1180, tail-call Replicator::OnReceive (0xb02e30).
    stub_b02e30(address_matches, first_byte, forwarded)
}

// 0xb04828 — __ZN3RBX7Network10Replicator16OnInternalPacketEPN6RakNet14InternalPacketEjNS2_13SystemAddressEji
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")]
pub fn stub_b04828(
    counts: &mut crate::replicator::SplitCounts,
    count: bool,
    split_count: u32,
    log_split: bool,
    first_fragment: bool,
    msg_id: u8,
    size_bytes: u32,
    log: &mut dyn FnMut(u8, u32, u32),
) {
    // IDA 0xb04828: split/whole counters at this[690]/this[689] plus the conditional split log.
    crate::replicator::on_internal_packet(
        counts, count, split_count, log_split, first_fragment, msg_id, size_bytes, log,
    );
}

// 0xb04a98 — __ZThn1180_N3RBX7Network10Replicator16OnInternalPacketEPN6RakNet14InternalPacketEjNS2_13SystemAddressEji
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::OnInternalPacket(RakNet::InternalPacket *,unsigned int,RakNet::SystemAddress,unsigned int,int)")]
pub fn stub_b04a98(
    counts: &mut crate::replicator::SplitCounts,
    count: bool,
    split_count: u32,
    log_split: bool,
    first_fragment: bool,
    msg_id: u8,
    size_bytes: u32,
    log: &mut dyn FnMut(u8, u32, u32),
) {
    // IDA 0xb04a98: __ZThn1180 — this -= 1180, tail-call Replicator::OnInternalPacket (0xb04828).
    stub_b04828(counts, count, split_count, log_split, first_fragment, msg_id, size_bytes, log);
}

// 0xb07980 — __ZN5boost4bindIvN3RBX7Network10ReplicatorERKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_NS8_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEENSG_ILi4EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf4ISN_T0_T1_T2_T3_T4_EENSL_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSQ_FSN_SR_SS_ST_SU_ESX_SY_SZ_S10_S11_
// type: void __fastcall(int, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list_av_5<boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&,boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Network::Replicator::*)(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&),boost::shared_ptr<RBX::Network::Replicator>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_b07980(
    target: SharedPtr<crate::replicator::Marker>,
) -> crate::replicator::ChatFilterCall {
    // IDA 0xb07980: bind(mf4 sendFilteredChatMessage, replicator, _1.._4);
    // the list5 retain is the Arc clone, the call forwards four args.
    crate::replicator::bind_chat_filter(target)
}

// 0xb08aa8 — __ZN3RBX7Network16SenderDictionaryINS_13SystemAddressEE4sendERN6RakNet9BitStreamERKS2_
// type: unsigned int __fastcall(int, RakNet::BitStream *this, int *)
#[doc(alias = "RBX::Network::SenderDictionary<RBX::SystemAddress>::send(RakNet::BitStream &,RBX::SystemAddress const&)")]
pub fn stub_b08aa8(
    dict: &mut crate::socket::SenderDictionary,
    stream: &mut crate::bitstream::BitStream,
    address: &crate::socket::SystemAddress,
) -> u32 {
    // IDA 0xb08aa8: broadcast/known/new arms over the +0 map and the +1048 counter.
    dict.send(stream, address)
}

// 0xb0ceb4 — __ZN3RBX7Network10Replicator16serializeSFFlagsERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::serializeSFFlags(RakNet::BitStream &)")]
pub fn stub_b0ceb4() {
    // IDA 0xb0ceb4: base Replicator::serializeSFFlags is empty (ServerReplicator overrides).
    crate::replicator::replicator_sf_flags_noop();
}

// 0xb0ceb8 — __ZN3RBX7Network10Replicator18deserializeSFFlagsERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::deserializeSFFlags(RakNet::BitStream &)")]
pub fn stub_b0ceb8() {
    // IDA 0xb0ceb8: base Replicator::deserializeSFFlags is empty (ServerReplicator overrides).
    crate::replicator::replicator_sf_flags_noop();
}

// 0xb0ced0 — __ZN6RakNet16PluginInterface28OnAttachEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::OnAttach(void)")]
pub fn stub_b0ced0(plugin: &crate::socket::PluginInterface2) {
    // IDA 0xb0ced0: default hook is empty.
    plugin.on_attach();
}

// 0xb0ced8 — __ZN6RakNet16PluginInterface26UpdateEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::Update(void)")]
pub fn stub_b0ced8(plugin: &crate::socket::PluginInterface2) {
    // IDA 0xb0ced8: default hook is empty.
    plugin.update();
}

// 0xb0cee0 — __ZN6RakNet16PluginInterface217OnRakPeerShutdownEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::OnRakPeerShutdown(void)")]
pub fn stub_b0cee0(plugin: &crate::socket::PluginInterface2) {
    // IDA 0xb0cee0: default hook is empty.
    plugin.on_rak_peer_shutdown();
}

// 0xb0cee8 — __ZN6RakNet16PluginInterface215OnNewConnectionERKNS_13SystemAddressENS_10RakNetGUIDEb
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnNewConnection(RakNet::SystemAddress const&,RakNet::RakNetGUID,bool)")]
pub fn stub_b0cee8(plugin: &crate::socket::PluginInterface2) {
    // IDA 0xb0cee8: default hook is empty.
    plugin.on_new_connection();
}

// 0xb0cef8 — __ZN6RakNet16PluginInterface221OnDirectSocketReceiveEPKcjNS_13SystemAddressE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnDirectSocketReceive(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_b0cef8(plugin: &crate::socket::PluginInterface2) {
    // IDA 0xb0cef8: default hook is empty.
    plugin.on_direct_socket_receive();
}

// 0xb0cf00 — __ZN6RakNet16PluginInterface25OnAckEjNS_13SystemAddressEj
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnAck(unsigned int,RakNet::SystemAddress,unsigned int)")]
pub fn stub_b0cf00(plugin: &crate::socket::PluginInterface2) {
    // IDA 0xb0cf00: default hook is empty.
    plugin.on_ack();
}

// 0xb0db10 — __ZN3RBX7Network10Replicator14SendClusterJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")]
pub fn stub_b0db10(job: crate::replicator::SendClusterJob) {
    // IDA 0xb0db10: D1 — vtable reset, +484 shared-count drop, base Job(this, -1); Rust drops.
    drop(job);
}

// 0xb0dbdc — __ZN3RBX7Network10Replicator14SendClusterJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::~SendClusterJob()")]
pub fn stub_b0dbdc(job: crate::replicator::SendClusterJob) {
    // IDA 0xb0dbdc: D0 — same teardown as D1 plus operator delete (IDA 0xb0dc50); Rust drops.
    drop(job);
}

// 0xb0dcbc — __ZN3RBX7Network10Replicator14SendClusterJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b0dcbc(elapsed: f64, rate_hz: f32, ctx: &crate::physics::SleepContext) -> f64 {
    // IDA 0xb0dcbc: stats float at +488 packed into the sleep double, into computeStandardSleepTime.
    crate::replicator::send_cluster_job_sleep_time(elapsed, rate_hz, ctx)
}

// 0xb0dfd8 — __ZN3RBX7Network13ReplicatorJobD1Ev
// type: void __fastcall(RBX::Network::ReplicatorJob *__hidden this)
#[doc(alias = "RBX::Network::ReplicatorJob::~ReplicatorJob()")]
pub fn stub_b0dfd8(job: crate::replicator::ReplicatorJob) {
    // IDA 0xb0dfd8: D1 — vtable reset, +484 shared-count drop, base Job(this, -1); Rust drops.
    drop(job);
}

// 0xb139fc — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE16_M_push_back_auxERKS5_
// type: void __fastcall(_DWORD *, int *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::deque<boost::shared_ptr<RBX::Network::Marker>,std::allocator<boost::shared_ptr<RBX::Network::Marker>>>::_M_push_back_aux(boost::shared_ptr<RBX::Network::Marker> const&)")]
pub fn stub_b139fc(
    queue: &mut std::collections::VecDeque<SharedPtr<crate::replicator::Marker>>,
    marker: SharedPtr<crate::replicator::Marker>,
) {
    // IDA 0xb139fc: refcounted push; spinlock-pool counting is Arc bookkeeping.
    crate::replicator::marker_queue_push(queue, marker);
}

// 0xb13d44 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
#[doc(alias = "std::deque<boost::shared_ptr<RBX::Network::Marker>,std::allocator<boost::shared_ptr<RBX::Network::Marker>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_b13d44(
    queue: &mut std::collections::VecDeque<SharedPtr<crate::replicator::Marker>>,
    extra: usize,
) {
    // IDA 0xb13d44: recenter-or-grow the chunk map; reserve keeps the growth edge.
    crate::replicator::marker_queue_reserve(queue, extra);
}

// 0xb14fe0 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>)")]
pub fn stub_b14fe0(
    target: &SharedPtr<crate::replicator::Marker>,
) -> SharedPtr<crate::replicator::Marker> {
    // IDA 0xb14fe0: copy the bound Replicator owner with net +1 retain.
    crate::replicator::bind_list1_replicator(target)
}

// 0xb15f50 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE11encodeCellsINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
// type: unsigned int __fastcall(int, const G3D::Vector3int16 *, int, RakNet::BitStream *, signed int)
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b15f50(
    stream: &mut crate::bitstream::BitStream,
    cells: &[(i32, i32, i32)],
    budget: i32,
    end_marker: u8,
) {
    // IDA 0xb15f50: OneQuarterClusterChunk iterator; per-cell 5/4/5 bits
    // plus 4/2/4 region headers and the 2-bit end marker, budget-limited.
    // Grid reads and encodeFromPosition stay engine-side.
    crate::replicator::voxel_encode_cells(stream, cells, budget, end_marker);
}

// 0xb173b0 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE11encodeCellsINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
// type: unsigned int __fastcall(int, const G3D::Vector3int16 *, RBX::Network::ClusterUpdateBuffer *, RakNet::BitStream *, signed int)
pub fn stub_b173b0(
    stream: &mut crate::bitstream::BitStream,
    cells: &[(i32, i32, i32)],
    budget: i32,
    end_marker: u8,
) {
    // IDA 0xb173b0: ClusterUpdateBuffer iterator; same 5/4/5 + 4/2/4 +
    // end-marker framing as 0xb15f50, grid side engine-side.
    crate::replicator::voxel_encode_cells(stream, cells, budget, end_marker);
}

// 0xb18564 — __ZNK3RBX5Voxel10SerializerINS0_4GridEE11encodeCellsINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
// type: unsigned int __fastcall(G3D::Vector3int16 *, const G3D::Vector3int16 *, int *, RakNet::BitStream *, signed int)
#[doc(alias = "void RBX::Voxel::Serializer<RBX::Voxel::Grid>::encodeCells<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::Voxel::Grid const*,RBX::ClusterCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b18564(
    stream: &mut crate::bitstream::BitStream,
    cells: &[(i32, i32, i32)],
    budget: i32,
    end_marker: u8,
) {
    // IDA 0xb18564: ClusterCellIterator; same framing as 0xb15f50, grid
    // side engine-side.
    crate::replicator::voxel_encode_cells(stream, cells, budget, end_marker);
}

// 0xb1c5cc — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::list4(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_b1c5cc(
    target_alive: bool,
    has_data: bool,
) -> crate::replicator::ReplicationBind {
    // IDA 0xb1c5cc: capture weak owner + raw data + arg<1> + arg<2>.
    crate::replicator::replication_bind4(target_alive, has_data)
}

// 0xb1c790 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEC2ES8_SB_SD_SE_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_b1c790(
    bind: &crate::replicator::ReplicationBind,
) -> crate::replicator::ReplicationBind {
    // IDA 0xb1c790: store the full 4-tuple.
    crate::replicator::replication_store4(bind)
}

// 0xb1c954 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEEEC2ES8_SB_SD_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>)")]
pub fn stub_b1c954(
    bind: &crate::replicator::ReplicationBind,
) -> crate::replicator::ReplicationBind3 {
    // IDA 0xb1c954: drop arg<2>, keep weak owner + data + arg<1>.
    crate::replicator::replication_store3(bind)
}

// 0xb1cb18 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEEEC2ES8_SB_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>)")]
pub fn stub_b1cb18(
    bind: &crate::replicator::ReplicationBind,
) -> crate::replicator::ReplicationBind2 {
    // IDA 0xb1cb18: keep only the weak owner and the raw data.
    crate::replicator::replication_store2(bind)
}

// 0xb2058c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network31SharedStringProtectedDictionaryES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::SharedStringProtectedDictionary,RBX::Network::SharedStringProtectedDictionary>(boost::shared_ptr<RBX::Network::SharedStringProtectedDictionary> *,RBX::Network::SharedStringProtectedDictionary *,boost::detail::shared_count &)")]
pub fn stub_b2058c() -> SharedPtr<crate::replicator::SharedStringProtectedDictionary> {
    // IDA 0xb2058c: publish the fresh protected-dictionary control block.
    crate::replicator::protected_string_dict()
}

// 0xb20850 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20850(dict: SharedPtr<crate::replicator::SharedStringProtectedDictionary>) {
    // IDA 0xb20850: D1 — dispose; Rust drops.
    crate::replicator::shared_dict_drop(dict);
}

// 0xb20854 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20854(dict: SharedPtr<crate::replicator::SharedStringProtectedDictionary>) {
    // IDA 0xb20854: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::shared_dict_drop(dict);
}

// 0xb20860 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::dispose(void)")]
pub fn stub_b20860(dict: SharedPtr<crate::replicator::SharedStringProtectedDictionary>) {
    // IDA 0xb20860: dispose runs the held destructor; Rust drops.
    crate::replicator::shared_dict_drop(dict);
}

// 0xb209b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_deleter(std::type_info const&)")]
pub fn stub_b209b0() -> *const () {
    // IDA 0xb209b0: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb209b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network31SharedStringProtectedDictionaryEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringProtectedDictionary>::get_untyped_deleter(void)")]
pub fn stub_b209b4() -> *const () {
    // IDA 0xb209b4: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb20d10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20d10(dict: SharedPtr<crate::string_dictionary::SharedStringDictionary>) {
    // IDA 0xb20d10: D1 — dispose; Rust drops.
    crate::replicator::shared_dict_drop(dict);
}

// 0xb20d14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::~sp_counted_impl_p()")]
pub fn stub_b20d14(dict: SharedPtr<crate::string_dictionary::SharedStringDictionary>) {
    // IDA 0xb20d14: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::shared_dict_drop(dict);
}

// 0xb20d20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::dispose(void)")]
pub fn stub_b20d20(dict: SharedPtr<crate::string_dictionary::SharedStringDictionary>) {
    // IDA 0xb20d20: dispose runs the held destructor; Rust drops.
    crate::replicator::shared_dict_drop(dict);
}

// 0xb20e64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_deleter(std::type_info const&)")]
pub fn stub_b20e64() -> *const () {
    // IDA 0xb20e64: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb20e68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22SharedStringDictionaryEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::SharedStringDictionary>::get_untyped_deleter(void)")]
pub fn stub_b20e68() -> *const () {
    // IDA 0xb20e68: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb20e6c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23TopNErrorsPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b20e6c() -> SharedPtr<crate::physics::TopNErrorsPhysicsSender> {
    // IDA 0xb20e6c: publish the fresh sender control block.
    crate::physics::top_n_errors_physics_sender()
}

// 0xb21004 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21004(sender: SharedPtr<crate::physics::TopNErrorsPhysicsSender>) {
    // IDA 0xb21004: D1 — dispose; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb21008 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21008(sender: SharedPtr<crate::physics::TopNErrorsPhysicsSender>) {
    // IDA 0xb21008: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb21014 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::dispose(void)")]
pub fn stub_b21014(sender: SharedPtr<crate::physics::TopNErrorsPhysicsSender>) {
    // IDA 0xb21014: dispose runs the held destructor; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb21028 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_deleter(std::type_info const&)")]
pub fn stub_b21028() -> *const () {
    // IDA 0xb21028: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb2102c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_untyped_deleter(void)")]
pub fn stub_b2102c() -> *const () {
    // IDA 0xb2102c: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb21030 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::RoundRobinPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::RoundRobinPhysicsSender *,boost::detail::shared_count &)")]
pub fn stub_b21030() -> SharedPtr<crate::physics::RoundRobinPhysicsSender> {
    // IDA 0xb21030: publish the fresh sender control block.
    crate::physics::round_robin_physics_sender()
}

// 0xb211c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b211c8(sender: SharedPtr<crate::physics::RoundRobinPhysicsSender>) {
    // IDA 0xb211c8: D1 — dispose; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb211cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b211cc(sender: SharedPtr<crate::physics::RoundRobinPhysicsSender>) {
    // IDA 0xb211cc: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb211d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::dispose(void)")]
pub fn stub_b211d8(sender: SharedPtr<crate::physics::RoundRobinPhysicsSender>) {
    // IDA 0xb211d8: dispose runs the held destructor; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb211ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_deleter(std::type_info const&)")]
pub fn stub_b211ec() -> *const () {
    // IDA 0xb211ec: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb211f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_untyped_deleter(void)")]
pub fn stub_b211f0() -> *const () {
    // IDA 0xb211f0: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb211f4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender2>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender2 *,boost::detail::shared_count &)")]
pub fn stub_b211f4() -> SharedPtr<crate::physics::ErrorCompPhysicsSender2> {
    // IDA 0xb211f4: publish the fresh sender control block.
    crate::physics::error_comp_physics_sender2()
}

// 0xb2138c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
pub fn stub_b2138c(sender: SharedPtr<crate::physics::ErrorCompPhysicsSender2>) {
    // IDA 0xb2138c: D1 — dispose; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb21390 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
pub fn stub_b21390(sender: SharedPtr<crate::physics::ErrorCompPhysicsSender2>) {
    // IDA 0xb21390: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb2139c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::dispose(void)")]
pub fn stub_b2139c(sender: SharedPtr<crate::physics::ErrorCompPhysicsSender2>) {
    // IDA 0xb2139c: dispose runs the held destructor; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb213b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_deleter(std::type_info const&)")]
pub fn stub_b213b0() -> *const () {
    // IDA 0xb213b0: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb213b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_untyped_deleter(void)")]
pub fn stub_b213b4() -> *const () {
    // IDA 0xb213b4: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb213b8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender>(boost::shared_ptr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender *,boost::detail::shared_count &)")]
pub fn stub_b213b8() -> SharedPtr<crate::physics::ErrorCompPhysicsSender> {
    // IDA 0xb213b8: publish the fresh sender control block.
    crate::physics::error_comp_physics_sender()
}

// 0xb21550 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21550(sender: SharedPtr<crate::physics::ErrorCompPhysicsSender>) {
    // IDA 0xb21550: D1 — dispose; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb21554 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
pub fn stub_b21554(sender: SharedPtr<crate::physics::ErrorCompPhysicsSender>) {
    // IDA 0xb21554: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb21560 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::dispose(void)")]
pub fn stub_b21560(sender: SharedPtr<crate::physics::ErrorCompPhysicsSender>) {
    // IDA 0xb21560: dispose runs the held destructor; Rust drops.
    crate::replicator::shared_dict_drop(sender);
}

// 0xb21574 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_deleter(std::type_info const&)")]
pub fn stub_b21574() -> *const () {
    // IDA 0xb21574: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb21578 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_untyped_deleter(void)")]
pub fn stub_b21578() -> *const () {
    // IDA 0xb21578: no custom deleter installed — null.
    crate::replicator::shared_null_deleter()
}

// 0xb21844 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSEPSH_
// type: int32_t **__fastcall(int32_t **, int32_t *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot*)")]
pub fn stub_b21844(
    slot: &mut SharedPtr<crate::replicator::ChatSlot>,
    next: &SharedPtr<crate::replicator::ChatSlot>,
) {
    // IDA 0xb21844: addref the raw slot, store it, release the old slot.
    crate::replicator::chat_slot_assign(slot, next);
}

// 0xb218f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSERKSI_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot> const&)")]
pub fn stub_b218f8(
    slot: &mut SharedPtr<crate::replicator::ChatSlot>,
    next: &SharedPtr<crate::replicator::ChatSlot>,
) {
    // IDA 0xb218f8: same retain-store-release via the const-ref copy.
    crate::replicator::chat_slot_assign(slot, next);
}

// 0xb21bec — __ZNK3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::connected(void)const")]
pub fn stub_b21bec(slot: &crate::replicator::ChatSlot) -> bool {
    // IDA 0xb21bec: the +12 signal link is set.
    crate::replicator::chat_slot_connected(slot)
}

// 0xb21bf8 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")]
pub fn stub_b21bf8(
    call: &crate::replicator::ChatFilterCall,
    addr: &crate::socket::SystemAddress,
    text: &str,
    filtered: &str,
    invoke: &mut dyn FnMut(&SharedPtr<crate::replicator::Marker>, &crate::socket::SystemAddress, &str, &str),
) {
    // IDA 0xb21bf8: vtable dispatch into the bound mf4 with four args.
    crate::replicator::chat_callable_call(call, addr, text, filtered, invoke);
}

// 0xb21c28 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")]
pub fn stub_b21c28(
    call: &crate::replicator::ChatFilterCall,
    addr: &crate::socket::SystemAddress,
    text: &str,
    filtered: &str,
    invoke: &mut dyn FnMut(&SharedPtr<crate::replicator::Marker>, &crate::socket::SystemAddress, &str, &str),
) {
    // IDA 0xb21c28: __ZThn4 — this -= 4, tail-call callable::call (0xb21bf8).
    stub_b21bf8(call, addr, text, filtered, invoke)
}

// 0xb21c58 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE6removeEPNSF_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::remove(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot *)")]
pub fn stub_b21c58(
    slots: &mut Vec<SharedPtr<crate::replicator::ChatSlot>>,
    target: &SharedPtr<crate::replicator::ChatSlot>,
) -> bool {
    // IDA 0xb21c58: assert live, log the removal, splice the slot out.
    crate::replicator::chat_signal_remove(slots, target)
}

// 0xb21d44 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_b21d44() {
    // IDA 0xb21d44: one-time slot mutex construction; Rust statics need none.
    crate::replicator::chat_slot_mutex_init();
}

// 0xb21e28 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
pub fn stub_b21e28(call: crate::replicator::ChatFilterCall) {
    // IDA 0xb21e28: D2 — vtable reset, shared_count drop, slot release.
    crate::replicator::chat_callable_drop(call);
}

// 0xb21fa4 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
pub fn stub_b21fa4(call: crate::replicator::ChatFilterCall) {
    // IDA 0xb21fa4: D1 — dispose; Rust drops.
    crate::replicator::chat_callable_drop(call);
}

// 0xb21fb0 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
pub fn stub_b21fb0(call: crate::replicator::ChatFilterCall) {
    // IDA 0xb21fb0: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::chat_callable_drop(call);
}

// 0xb22064 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")]
pub fn stub_b22064(slot: SharedPtr<crate::replicator::ChatSlot>) {
    // IDA 0xb22064: D1 — vtable reset plus chained slot release.
    crate::replicator::chat_slot_drop(slot);
}

// 0xb220c0 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")]
pub fn stub_b220c0(slot: SharedPtr<crate::replicator::ChatSlot>) {
    // IDA 0xb220c0: D0 — dispose plus operator delete; Rust drops.
    crate::replicator::chat_slot_drop(slot);
}

// 0xb221c8 — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list5(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_b221c8(
    target: &SharedPtr<crate::replicator::Marker>,
) -> crate::replicator::ChatFilterCall {
    // IDA 0xb221c8: retain the owner, capture the four placeholders.
    crate::replicator::chat_list5(target)
}

// 0xb22618 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_b22618(call: &crate::replicator::ChatFilterCall) -> crate::replicator::ChatFilterCall {
    // IDA 0xb22618: keep the owner plus the first three placeholders.
    crate::replicator::chat_store4(call)
}

// 0xb22a68 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>)")]
pub fn stub_b22a68(call: &crate::replicator::ChatFilterCall) -> crate::replicator::ChatFilterCall {
    // IDA 0xb22a68: keep only the owner and the first placeholder.
    crate::replicator::chat_store2(call)
}

// 0xb2332c — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
pub fn stub_b2332c(declared: &mut bool) -> bool {
    // IDA 0xb2332c: one-time Name::declare(&sClusterPacketCache).
    crate::replicator::declare_cluster_packet_cache(declared)
}

// 0xb23cd8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(boost::shared_ptr<RBX::Network::Replicator::PingJob> *,RBX::Network::Replicator::PingJob *,boost::detail::shared_count &)")]
pub fn stub_b23cd8() -> SharedPtr<crate::replicator::PingJob> {
    // IDA 0xb23cd8: publish the fresh PingJob control block.
    crate::replicator::ping_job()
}
