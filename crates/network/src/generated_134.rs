//! Auto-generated skeletons for rbx-network — RakNet|RBX::Network|Replicator filtered EA-sorted asc
//! Filter: RakNet|RBX::Network|Replicator (case-insensitive) -> 4797 funcs, 2858 already stubbed (1939 remaining before batch)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x9d7028..0x9e7f28 | existing 13990 -> 14090 total (filtered EA-sorted asc, rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
// 0x9d7028 — __ZN3RBX7Network16ServerReplicator15createStatsItemEv
// type: void __fastcall(RBX::Network::ServerReplicator *this)
#[doc(alias = "RBX::Network::ServerReplicator::createStatsItem(void)")]
pub fn stub_9d7028() -> ! {
    todo!("0x9d7028 __ZN3RBX7Network16ServerReplicator15createStatsItemEv")
}

// 0x9d7414 — __ZNK3RBX7Network16ServerReplicator21canUseProtocolVersionEi
// type: bool __fastcall(RBX::Network::ServerReplicator *this, int)
#[doc(alias = "RBX::Network::ServerReplicator::canUseProtocolVersion(int)const")]
pub fn stub_9d7414() -> ! {
    todo!("0x9d7414 __ZNK3RBX7Network16ServerReplicator21canUseProtocolVersionEi")
}

// 0x9d7430 — __ZN3RBX7Network16ServerReplicatorC1EN6RakNet13SystemAddressEPNS0_6ServerEPNS_15NetworkSettingsE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Network::ServerReplicator::ServerReplicator(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)")]
pub fn stub_9d7430(table: &mut crate::player::ReplicatorTable) -> u32 {
    // IDA 0x9d7430 (C1): `new ServerReplicator` + control block, owner-wired; returns the handle.
    table.create()
}

// 0x9d744c — __ZN3RBX7Network16ServerReplicatorC2EN6RakNet13SystemAddressEPNS0_6ServerEPNS_15NetworkSettingsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *, pthread_mutex_t *, RBX::ServiceProvider *, pthread_mutex_t *)
#[doc(alias = "RBX::Network::ServerReplicator::ServerReplicator(RakNet::SystemAddress,RBX::Network::Server *,RBX::NetworkSettings *)")]
pub fn stub_9d744c(table: &mut crate::player::ReplicatorTable) -> u32 {
    // IDA 0x9d744c (C2): `new ServerReplicator` + control block, owner-wired; returns the handle.
    table.create()

}
// 0x9d7e54 — __ZN3RBX7Network16ServerReplicatorD0Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d7e54(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d7e54 (D0): D2 then `operator delete`; the crate drops the handle.
    table.remove(handle);
}

// 0x9d7ef4 — __ZN3RBX7Network16ServerReplicatorD1Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d7ef4(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d7ef4 (D1): full teardown in place; the crate drops the handle.
    table.remove(handle);
}

// 0x9d7f00 — __ZThn32_N3RBX7Network16ServerReplicatorD0Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d7f00(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d7f00 (ZThn32 D0): adjusts `this`, then D0.
    table.remove(handle);
}

// 0x9d7fa4 — __ZThn36_N3RBX7Network16ServerReplicatorD0Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d7fa4(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d7fa4 (ZThn36 D0): adjusts `this`, then D0.
    table.remove(handle);
}

// 0x9d8048 — __ZThn1180_N3RBX7Network16ServerReplicatorD0Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d8048(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d8048 (ZThn1180 D0): adjusts `this`, then D0.
    table.remove(handle);
}

// 0x9d80ec — __ZThn1192_N3RBX7Network16ServerReplicatorD0Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d80ec(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d80ec (ZThn1192 D0): adjusts `this`, then D0.
    table.remove(handle);
}

// 0x9d8190 — __ZN3RBX7Network16ServerReplicatorD2Ev
// type: void __fastcall(struct _Unwind_Exception *this)
#[doc(alias = "RBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d8190(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d8190 (D2): full teardown in place; the crate drops the handle.
    table.remove(handle);
}

// 0x9d86b4 — __ZThn32_N3RBX7Network16ServerReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d86b4(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d86b4 (ZThn32 D1): adjusts `this`, then D1.
    table.remove(handle);
}

// 0x9d86c0 — __ZThn36_N3RBX7Network16ServerReplicatorD1Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d86c0(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d86c0 (ZThn36 D1): adjusts `this`, then D1.
    table.remove(handle);
}

// 0x9d86cc — __ZThn1180_N3RBX7Network16ServerReplicatorD1Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d86cc(table: &mut crate::player::ReplicatorTable, handle: u32) {
    // IDA 0x9d86cc (ZThn1180 D1): adjusts `this`, then D1.
    table.remove(handle);
}

// 0x9d86dc — __ZThn1192_N3RBX7Network16ServerReplicatorD1Ev
// type: void __fastcall(RBX::Network::ServerReplicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::~ServerReplicator()")]
pub fn stub_9d86dc() -> ! {
    todo!("0x9d86dc __ZThn1192_N3RBX7Network16ServerReplicatorD1Ev")
}

// 0x9d86ec — __ZN3RBX7Network16ServerReplicator14receiveClusterERN6RakNet9BitStreamEPNS_8InstanceE
// type: void __fastcall(RBX::Network::ServerReplicator *this, RakNet::BitStream *, RBX::Instance *)
#[doc(alias = "RBX::Network::ServerReplicator::receiveCluster(RakNet::BitStream &,RBX::Instance *)")]
pub fn stub_9d86ec(forward: impl FnOnce()) {
    // IDA 0x9d86ec: pure tail-call to `Replicator::receiveCluster` (engine-side).
    crate::replicator::receive_cluster(forward);
}

// 0x9d8700 — __ZN3RBX7Network16ServerReplicator26readPlayerSimulationRegionERNS_7Region213WeightedPointE
// type: RBX::PartInstance *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "RBX::Network::ServerReplicator::readPlayerSimulationRegion(RBX::Region2::WeightedPoint &)")]
pub fn stub_9d8700() -> ! {
    todo!("0x9d8700 __ZN3RBX7Network16ServerReplicator26readPlayerSimulationRegionERNS_7Region213WeightedPointE")
}

// 0x9d8768 — __ZN3RBX7Network16ServerReplicator23checkDistributedReceiveEPNS_12PartInstanceE
// type: bool __fastcall(RBX::Network::ServerReplicator *this, RBX::Mechanism **)
#[doc(alias = "RBX::Network::ServerReplicator::checkDistributedReceive(RBX::PartInstance *)")]
pub fn stub_9d8768() -> ! {
    todo!("0x9d8768 __ZN3RBX7Network16ServerReplicator23checkDistributedReceiveEPNS_12PartInstanceE")
}

// 0x9d87b8 — __ZN3RBX7Network16ServerReplicator20checkDistributedSendEPKNS_12PartInstanceE
// type: int __fastcall(RBX::Network::ServerReplicator *this, RBX::Mechanism **, int)
#[doc(alias = "RBX::Network::ServerReplicator::checkDistributedSend(RBX::PartInstance const*)")]
pub fn stub_9d87b8() -> ! {
    todo!("0x9d87b8 __ZN3RBX7Network16ServerReplicator20checkDistributedSendEPKNS_12PartInstanceE")
}

// 0x9d885c — __ZN3RBX7Network16ServerReplicator24checkDistributedSendFastEPKNS_12PartInstanceE
// type: bool __fastcall(RBX::Network::ServerReplicator *this, RBX::Mechanism **, int)
#[doc(alias = "RBX::Network::ServerReplicator::checkDistributedSendFast(RBX::PartInstance const*)")]
pub fn stub_9d885c() -> ! {
    todo!("0x9d885c __ZN3RBX7Network16ServerReplicator24checkDistributedSendFastEPKNS_12PartInstanceE")
}

// 0x9d8924 — __ZN3RBX7Network16ServerReplicator16rebroadcastEventERNS_10Reflection15EventInvocationE
// type: int __fastcall(RBX::Network::ServerReplicator *this, RBX::Reflection::EventInvocation *)
#[doc(alias = "RBX::Network::ServerReplicator::rebroadcastEvent(RBX::Reflection::EventInvocation &)")]
pub fn stub_9d8924() -> ! {
    todo!("0x9d8924 __ZN3RBX7Network16ServerReplicator16rebroadcastEventERNS_10Reflection15EventInvocationE")
}

// 0x9d8930 — __ZN3RBX7Network16ServerReplicator24shouldDelayAddingToWorldEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::Network::Player **, RBX::Network::Player **)
#[doc(alias = "RBX::Network::ServerReplicator::shouldDelayAddingToWorld(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_9d8930() -> ! {
    todo!("0x9d8930 __ZN3RBX7Network16ServerReplicator24shouldDelayAddingToWorldEN5boost10shared_ptrINS_8InstanceEEE")
}

// 0x9d8ef0 — __ZN3RBX7Network16ServerReplicator26addTopReplicationContainerEPNS_8InstanceEbbN5boost8functionIFvNS4_10shared_ptrIS2_EEEEE
// type: void __fastcall(int, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ServerReplicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
pub fn stub_9d8ef0() -> ! {
    todo!("0x9d8ef0 __ZN3RBX7Network16ServerReplicator26addTopReplicationContainerEPNS_8InstanceEbbN5boost8functionIFvNS4_10shared_ptrIS2_EEEEE")
}

// 0x9d91d8 — __ZN3RBX7Network16ServerReplicator21isLegalDeleteInstanceEPNS_8InstanceE
// type: bool __fastcall(RBX::Network::ServerReplicator *this, struct _Unwind_Exception *)
#[doc(alias = "RBX::Network::ServerReplicator::isLegalDeleteInstance(RBX::Instance *)")]
pub fn stub_9d91d8() -> ! {
    todo!("0x9d91d8 __ZN3RBX7Network16ServerReplicator21isLegalDeleteInstanceEPNS_8InstanceE")
}

// 0x9d9f78 — __ZN3RBX7Network16ServerReplicator22isLegalReceiveInstanceEPNS_8InstanceES3_
// type: bool __fastcall(RBX::Network::ServerReplicator *this, RBX::Instance *, RBX::Instance *, int)
#[doc(alias = "RBX::Network::ServerReplicator::isLegalReceiveInstance(RBX::Instance *,RBX::Instance *)")]
pub fn stub_9d9f78() -> ! {
    todo!("0x9d9f78 __ZN3RBX7Network16ServerReplicator22isLegalReceiveInstanceEPNS_8InstanceES3_")
}

// 0x9db1d4 — __ZN3RBX7Network16ServerReplicator19isLegalReceiveEventEPNS_8InstanceERKNS_10Reflection15EventDescriptorE
// type: bool __fastcall(RBX::Network::ServerReplicator *this, pthread_mutex_t **, const RBX::Reflection::EventDescriptor *)
#[doc(alias = "RBX::Network::ServerReplicator::isLegalReceiveEvent(RBX::Instance *,RBX::Reflection::EventDescriptor const&)")]
pub fn stub_9db1d4() -> ! {
    todo!("0x9db1d4 __ZN3RBX7Network16ServerReplicator19isLegalReceiveEventEPNS_8InstanceERKNS_10Reflection15EventDescriptorE")
}

// 0x9dbb8c — __ZN3RBX7Network16ServerReplicator22isLegalReceivePropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
// type: bool __fastcall(RBX::Network::ServerReplicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::ServerReplicator::isLegalReceiveProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_9dbb8c() -> ! {
    todo!("0x9dbb8c __ZN3RBX7Network16ServerReplicator22isLegalReceivePropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE")
}

// 0x9dbd20 — __ZN3RBX7Network16ServerReplicator12onSentMarkerEl
// type: int __fastcall(RBX::Network::ServerReplicator *this, int, int, int)
#[doc(alias = "RBX::Network::ServerReplicator::onSentMarker(long)")]
pub fn stub_9dbd20() -> ! {
    todo!("0x9dbd20 __ZN3RBX7Network16ServerReplicator12onSentMarkerEl")
}

// 0x9dbd58 — __ZN3RBX7Network16ServerReplicator19isLegalSendPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::ServerReplicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::ServerReplicator::isLegalSendProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_9dbd58() -> ! {
    todo!("0x9dbd58 __ZN3RBX7Network16ServerReplicator19isLegalSendPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE")
}

// 0x9dbd5c — __ZN3RBX7Network16ServerReplicator20canReplicatePropertyERKNS_10Reflection13ConstPropertyE
// type: bool __fastcall(_DWORD *, int *)
#[doc(alias = "RBX::Network::ServerReplicator::canReplicateProperty(RBX::Reflection::ConstProperty const&)")]
pub fn stub_9dbd5c() -> ! {
    todo!("0x9dbd5c __ZN3RBX7Network16ServerReplicator20canReplicatePropertyERKNS_10Reflection13ConstPropertyE")
}

// 0x9dbe34 — __ZN3RBX7Network16ServerReplicator7sendTopEPN6RakNet16RakPeerInterfaceE
// type: int __fastcall(RBX::Network::Replicator *, int, int, const void *)
#[doc(alias = "RBX::Network::ServerReplicator::sendTop(RakNet::RakPeerInterface *)")]
#[allow(clippy::too_many_arguments)]
pub fn stub_9dbe34(
    stream: &mut crate::bitstream::BitStream,
    streaming_enabled: bool,
    extra_flag: Option<bool>,
    instance_ids: &[u32],
    serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream, u32),
    queue_new_instance: &mut dyn FnMut(u32),
    send: &mut dyn FnMut(&mut crate::bitstream::BitStream),
) {
    // IDA 0x9dbe34: 129 header + id loop + priority-3 send.
    crate::replicator::send_top(stream, streaming_enabled, extra_flag, instance_ids, serialize_id, queue_new_instance, send);
}

// 0x9dc8e4 — __ZN3RBX7Network16ServerReplicator19installRemotePlayerESs
// type: void __fastcall(RBX::Instance **, const std::string *)
#[doc(alias = "RBX::Network::ServerReplicator::installRemotePlayer(std::string)")]
pub fn stub_9dc8e4(load_character: bool) -> bool {
    // IDA 0x9dc8e4: address stamp + Players parent; the +157 flag gates loadCharacter.
    crate::replicator::install_remote_player(load_character)
}

// 0x9dca6c — __ZN3RBX7Network16ServerReplicator9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(char *, unsigned int *)
#[doc(alias = "RBX::Network::ServerReplicator::OnReceive(RakNet::Packet *)")]
pub fn stub_9dca6c(address_matches: bool, first_byte: Option<u8>) -> crate::replicator::ReceiveVerdict {
    // IDA 0x9dca6c: mismatch → ignored; leading 143 → spawn-name parse; else forward to `Replicator::OnReceive`.
    crate::replicator::on_receive(address_matches, first_byte)
}

// 0x9dcbc8 — __ZThn1180_N3RBX7Network16ServerReplicator9OnReceiveEPN6RakNet6PacketE
// type: int __fastcall(int, unsigned int *)
#[doc(alias = "non-virtual thunk toRBX::Network::ServerReplicator::OnReceive(RakNet::Packet *)")]
pub fn stub_9dcbc8(address_matches: bool, first_byte: Option<u8>) -> crate::replicator::ReceiveVerdict {
    // IDA 0x9dcbc8 (ZThn1180 OnReceive): adjusts `this`, then OnReceive.
    crate::replicator::on_receive(address_matches, first_byte)

}
// 0x9dcbd8 — __ZN3RBX7Network16ServerReplicator15sendItemsPacketEv
// type: int __fastcall(RBX::Network::ServerReplicator *this)
#[doc(alias = "RBX::Network::ServerReplicator::sendItemsPacket(void)")]
pub fn stub_9dcbd8(
    base: &mut dyn FnMut() -> bool,
    unbuffered: bool,
    extra_rounds: bool,
    extra_count: u8,
) -> bool {
    // IDA 0x9dcbd8: base items packet plus bounded extras.
    crate::replicator::send_items_packet(base, unbuffered, extra_rounds, extra_count)
}

// 0x9dcc34 — __ZN3RBX7Network16ServerReplicator8readItemERN6RakNet9BitStreamENS0_4Item8ItemTypeE
// type: void __fastcall(RBX::Network::Replicator::StreamJob **, RakNet::BitStream *, const char *)
#[doc(alias = "RBX::Network::ServerReplicator::readItem(RakNet::BitStream &,RBX::Network::Item::ItemType)")]
pub fn stub_9dcc34(item_type: u8) -> crate::replicator::IncomingItem {
    // IDA 0x9dcc34: 8 → character request, 9 → throw `"rocky"`, 0xA → prop ack, 0xC → quota, 0xE/0xF → removals, else base.
    crate::replicator::read_item_kind(item_type)
}

// 0x9dcfb8 — __ZN3RBX7Network16ServerReplicator20readRequestCharacterERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::ServerReplicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::ServerReplicator::readRequestCharacter(RakNet::BitStream &)")]
pub fn stub_9dcfb8(
    stream: &mut crate::bitstream::BitStream,
    instance: Option<u32>,
    readable_guid: &str,
    address: &str,
) -> crate::replicator::CharacterRequest {
    // IDA 0x9dcfb8: u32 model id + player-name string + instance ref; unresolvable refs throw; feeds `processRequestCharacter`.
    crate::replicator::read_request_character(stream, instance, readable_guid, address)
}

// 0x9dd5f8 — __ZN3RBX7Network16ServerReplicator23readPropAcknowledgementERN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::ServerReplicator *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::ServerReplicator::readPropAcknowledgement(RakNet::BitStream &)")]
pub fn stub_9dd5f8(
    resolved: bool,
    index: i32,
    instance: Option<u32>,
    descriptor: u32,
) -> crate::replicator::PropAckOutcome {
    // IDA 0x9dd5f8: int index + instance ref; unresolved returns the lookup verdict, else pairs `(descriptor, instance)` for `PropSync`.
    crate::replicator::prop_ack_outcome(resolved, index, instance, descriptor)
}

// 0x9dd6c8 — __ZN3RBX7Network16ServerReplicator23processRequestCharacterEPNS_8InstanceENS_4Guid4DataEjSs
// type: void __fastcall(_DWORD *, int, int, int, int, const std::string *)
#[doc(alias = "RBX::Network::ServerReplicator::processRequestCharacter(RBX::Instance *,RBX::Guid::Data,unsigned int,std::string)")]
pub fn stub_9dd6c8(remote_present: bool, remote_matches: bool) -> crate::replicator::CharacterProcess {
    // IDA 0x9dd6c8: null remote logs + throws, mismatched remote logs + throws, else a virtual proceeds.
    crate::replicator::process_request_character(remote_present, remote_matches)
}

// 0x9ddef4 — __ZN3RBX7Network16ServerReplicator29filterReceivedChangedPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::ServerReplicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::ServerReplicator::filterReceivedChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_9ddef4(prop_sync_accepted: bool, filter: Option<bool>) -> bool {
    // IDA 0x9ddef4: asserts + `PropSync` short-circuit, else the `NetworkFilter` verdict (absent → accept).
    crate::replicator::filter_received_changed_property(prop_sync_accepted, filter)
}

// 0x9dee84 — __ZN3RBX7Network16ServerReplicator20filterReceivedParentEPNS_8InstanceES3_
// type: int __fastcall(RBX::Network::ServerReplicator *this, struct _Unwind_Exception *, RBX::Instance *)
#[doc(alias = "RBX::Network::ServerReplicator::filterReceivedParent(RBX::Instance *,RBX::Instance *)")]
pub fn stub_9dee84(filter: Option<bool>) -> bool {
    // IDA 0x9dee84: asserts the instance (debug-only), then the `NetworkFilter::filterParent` verdict (absent → accept).
    crate::replicator::filter_received_parent(filter)
}

// 0x9e0004 — __ZN3RBX7Network16ServerReplicator13filterPhysicsEPNS_12PartInstanceE
// type: int __fastcall(RBX::Network::ServerReplicator *this, RBX::PartInstance *)
#[doc(alias = "RBX::Network::ServerReplicator::filterPhysics(RBX::PartInstance *)")]
pub fn stub_9e0004() -> ! {
    todo!("0x9e0004 __ZN3RBX7Network16ServerReplicator13filterPhysicsEPNS_12PartInstanceE")
}

// 0x9e0098 — __ZN3RBX7Network16ServerReplicator11dataOutStepEv
// type: void __fastcall(RBX::Network::ServerReplicator *this)
#[doc(alias = "RBX::Network::ServerReplicator::dataOutStep(void)")]
pub fn stub_9e0098() -> ! {
    todo!("0x9e0098 __ZN3RBX7Network16ServerReplicator11dataOutStepEv")
}

// 0x9e00b0 — __ZN3RBX7Network16ServerReplicator17onPropertyChangedEPNS_8InstanceEPKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::ServerReplicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::ServerReplicator::onPropertyChanged(RBX::Instance *,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_9e00b0() -> ! {
    todo!("0x9e00b0 __ZN3RBX7Network16ServerReplicator17onPropertyChangedEPNS_8InstanceEPKNS_10Reflection18PropertyDescriptorE")
}


// 0x9e06cc — __ZN3RBX7Network16ServerReplicator23writeChangedRefPropertyEPKNS_8InstanceERKNS_10Reflection21RefPropertyDescriptorERKNS_4Guid4DataERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::ServerReplicator *this, const RBX::Instance *, const RBX::Reflection::RefPropertyDescriptor *, const RBX::Guid::Data *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::ServerReplicator::writeChangedRefProperty(RBX::Instance const*,RBX::Reflection::RefPropertyDescriptor const&,RBX::Guid::Data const&,RakNet::BitStream &)")]
pub fn stub_9e06cc(
    stream: &mut crate::bitstream::BitStream,
    serializer: &mut crate::id_serializer::IdSerializer,
    sender: &crate::id_serializer::DescriptorSender,
    packet: &crate::replicator::ChangedProperty,
    should_send: bool,
    target: crate::id_serializer::GuidData,
    write_value: impl FnOnce(&mut crate::bitstream::BitStream),
) {
    // IDA 0x9e06cc: same packet as `writeChangedProperty` plus the trailing ref-target guid (null name → 8 zero bits).
    crate::replicator::write_changed_ref_property(stream, serializer, sender, packet, should_send, target, write_value)
}

// 0x9e0c14 — __ZN3RBX7Network16ServerReplicator22serializePropertyValueERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamEb
// type: void __fastcall(RBX::Network::Replicator *, int *, RakNet::BitStream *, int)
#[doc(alias = "RBX::Network::ServerReplicator::serializePropertyValue(RBX::Reflection::ConstProperty const&,RakNet::BitStream &,bool)")]
pub fn stub_9e0c14(stream: &mut crate::bitstream::BitStream, write: impl FnOnce(&mut crate::bitstream::BitStream)) {
    // IDA 0x9e0c14: the reflection type-switch stays engine-side; runs the caller-supplied writer.
    crate::replicator::serialize_property_value(stream, write)
}

// 0x9e10f4 — __ZN3RBX7Network16ServerReplicator24deserializePropertyValueERN6RakNet9BitStreamENS_10Reflection8PropertyEb
// type: void __fastcall(RBX::Network::IdSerializer *, pthread_mutex_t *, int *, pthread_mutex_t *)
#[doc(alias = "RBX::Network::ServerReplicator::deserializePropertyValue(RakNet::BitStream &,RBX::Reflection::Property,bool)")]
pub fn stub_9e10f4(stream: &mut crate::bitstream::BitStream, mut read: impl FnMut(&mut crate::bitstream::BitStream)) {
    // IDA 0x9e10f4: the reflection type-switch stays engine-side; runs the caller-supplied reader.
    crate::replicator::deserialize_property_value(stream, read)
}

// 0x9e16cc — __ZN3RBX7Network16ServerReplicator17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(RBX::Network::ServerReplicator *this, pthread_mutex_t *, pthread_mutex_t *, int)
#[doc(alias = "RBX::Network::ServerReplicator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_9e16cc(
    new_provider: bool,
    workspace_present: bool,
    workspace_streaming: bool,
    parts_streaming_enabled: bool,
) -> bool {
    // IDA 0x9e16cc: StreamJob reset, workspace throw/gate, base provider call.
    crate::replicator::replicator_on_service_provider(new_provider, workspace_present, workspace_streaming, parts_streaming_enabled)
}

// 0x9e013c — __ZN3RBX7Network16ServerReplicator20writeChangedPropertyEPKNS_8InstanceERKNS_10Reflection18PropertyDescriptorERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::ServerReplicator *this, const RBX::Instance *, const RBX::Reflection::PropertyDescriptor *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::ServerReplicator::writeChangedProperty(RBX::Instance const*,RBX::Reflection::PropertyDescriptor const&,RakNet::BitStream &)")]
pub fn stub_9e013c(
    stream: &mut crate::bitstream::BitStream,
    serializer: &mut crate::id_serializer::IdSerializer,
    sender: &crate::id_serializer::DescriptorSender,
    packet: &crate::replicator::ChangedProperty,
    should_send: bool,
    write_value: impl FnOnce(&mut crate::bitstream::BitStream),
) {
    // IDA 0x9e013c: gate + `[itemType = 3][id][propIndex][syncFlag][value]` (log + packet-count engine-side).
    crate::replicator::write_changed_property(stream, serializer, sender, packet, should_send, write_value)
}

// 0x9e2024 — __ZN3RBX7Network16ServerReplicator16serializeSFFlagsERN6RakNet9BitStreamE
// type: _DWORD __fastcall(RBX::Network::ServerReplicator *__hidden this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::ServerReplicator::serializeSFFlags(RakNet::BitStream &)")]
pub fn stub_9e2024(
    stream: &mut crate::bitstream::BitStream,
    flags: &[crate::replicator::SynchronizedFlag<'_>],
) {
    // IDA 0x9e2024: `Write<ushort>(count)` then one `serializeSFFlag` per synchronized flag.
    crate::replicator::serialize_sf_flags(stream, flags)
}

// 0x9e2160 — __ZN3RBX10Reflection8EnumDescINS_7Network12FilterResultEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::addPair(RBX::Network::FilterResult,char const*)")]
pub fn stub_9e2160() {
    // IDA 0x9e2160: `EnumDesc<FilterResult>::addPair`; the enum table stays engine-side.
}

// 0x9e2688 — __ZN3RBX10Reflection7Variant14genericConvertINS_7Network12FilterResultEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::FilterResult & RBX::Reflection::Variant::genericConvert<RBX::Network::FilterResult>(void)")]
pub fn stub_9e2688(value: i32) -> i32 {
    // IDA 0x9e2688: `Variant::genericConvert<FilterResult>`; the Variant codec stays engine-side.
    value
}

// 0x9e29d8 — __ZN3RBX7Network13NetworkFilter33filterIfAssociatedWithOtherPlayerILNS0_12FilterResultE1EEEbPNS_8InstanceERS3_
// type: int __fastcall(_DWORD *, int, _DWORD *, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int)
#[doc(alias = "bool RBX::Network::NetworkFilter::filterIfAssociatedWithOtherPlayer<(RBX::Network::FilterResult)1>(RBX::Instance *,RBX::Network::FilterResult&)")]
pub fn stub_9e29d8(
    basic_filtering: bool,
    player_present: bool,
    character_present: bool,
    is_own_character: bool,
) -> (bool, bool) {
    // IDA 0x9e29d8: other-player association filters.
    crate::replicator::filter_if_associated_with_other_player(basic_filtering, player_present, character_present, is_own_character)
}

// 0x9e2b9c — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEES7_EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~BoundCallbackDesc()")]
pub fn stub_9e2b9c() {
    // IDA 0x9e2b9c: `BoundCallbackDesc<FilterResult(...)>` D1; descriptor state stays engine-side.
}

// 0x9e2cdc — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEEEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>)>::~BoundCallbackDesc()")]
pub fn stub_9e2cdc() {
    // IDA 0x9e2cdc: `BoundCallbackDesc<FilterResult(...)>` D1; descriptor state stays engine-side.
}

// 0x9e2e1c — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsNS0_7VariantEEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)>::~BoundCallbackDesc()")]
pub fn stub_9e2e1c() {
    // IDA 0x9e2e1c: `BoundCallbackDesc<FilterResult(...)>` D1; descriptor state stays engine-side.
}

// 0x9e2f5c — __ZN3RBX10Reflection17BoundCallbackDescIFNS_7Network12FilterResultEN5boost10shared_ptrINS_8InstanceEEESsEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<RBX::Network::FilterResult ()(rbx_core::SharedPtr<RBX::Instance>,std::string)>::~BoundCallbackDesc()")]
pub fn stub_9e2f5c() {
    // IDA 0x9e2f5c: `BoundCallbackDesc<FilterResult(...)>` D1; descriptor state stays engine-side.
}

// 0x9e309c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_9e309c() {
    // IDA 0x9e309c: `BoundFuncDesc<ServerReplicator, void(bool)>` D1; descriptor state stays engine-side.
}

// 0x9e3104 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network16ServerReplicatorEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::ServerReplicator,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_9e3104() {
    // IDA 0x9e3104: `BoundFuncDesc<ServerReplicator, void()>` D1; descriptor state stays engine-side.
}

// 0x9e314c — __ZN3RBX10Reflection9EventDescINS_7Network16ServerReplicatorEFvibiEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::ServerReplicator,void ()(int,bool,int),rbx::signal<void ()(int,bool,int)>,rbx::signal<void ()(int,bool,int)> RBX::Network::ServerReplicator::*>::~EventDesc()")]
pub fn stub_9e314c() {
    // IDA 0x9e314c: `EventDesc<ServerReplicator, ...>` D1; descriptor state stays engine-side.
}

// 0x9e3194 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7Network16ServerReplicator15ServerStatsItemEN5boost10shared_ptrIS5_EEEENS8_IT_EET0_
// type: void __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, boost::detail::shared_count *, int, int, void *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ServerReplicator::ServerStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ServerReplicator::ServerStatsItem,rbx_core::SharedPtr<RBX::Network::ServerReplicator>>(rbx_core::SharedPtr<RBX::Network::ServerReplicator>)")]
pub fn stub_9e3194() -> crate::replicator::ServerStatsItem {
    // IDA 0x9e3194: `Creatable::create<ServerStatsItem>`; counters stay engine-side.
    crate::replicator::create_stats_item()
}

// 0x9e34e0 — __ZN3RBX11shared_fromINS_7Network16ServerReplicatorEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ServerReplicator> RBX::shared_from<RBX::Network::ServerReplicator>(RBX::Network::ServerReplicator*)")]
pub fn stub_9e34e0(table: &crate::player::ReplicatorTable, handle: u32) -> Option<u32> {
    // IDA 0x9e34e0: `shared_from<ServerReplicator>`; an expired owner throws `bad_weak_ptr`, mirrored as `None`.
    table.contains(handle).then_some(handle)
}

// 0x9e39e0 — __ZN3rbx7signals16signal_with_argsILi4EFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEEclES6_S8_S6_Ss
// type: void __fastcall(_DWORD *, int *, int, int *, std::string *)
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
pub fn stub_9e39e0(list: &crate::signal::SlotList, fire: impl FnMut()) {
    // IDA 0x9e39e0: `signal_with_args<4>::operator()` — walk slots via `next`, invoking each.
    crate::signal::emit_each(list, fire);
}

// 0x9e4034 — __ZNK5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEEclES6_
// type: int __fastcall(int *, int *)
#[doc(alias = "boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_9e4034(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9e4034: `function1<FilterResult(shared_ptr)>::operator()`; args stay engine-side.
    call()
}

// 0x9e4388 — __ZN3RBX7Network10Replicator22isLegalReceiveInstanceEPNS_8InstanceES3_
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::isLegalReceiveInstance(RBX::Instance *,RBX::Instance *)")]
pub fn stub_9e4388() -> bool {
    // IDA 0x9e4388..0x9e438a: returns 1 unconditionally.
    crate::replicator::is_legal_receive_instance()
}

// 0x9e4490 — __ZNK5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EclES6_S6_
// type: struct _Unwind_Exception *__fastcall(int *, int *, int *)
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_9e4490(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9e4490: `function2<FilterResult(shared_ptr, shared_ptr)>::operator()`; args stay engine-side.
    call()
}

// 0x9e49ec — __ZNK5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEclES6_Ss
// type: int __fastcall(int *, int *, const std::string *)
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string)const")]
pub fn stub_9e49ec(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9e49ec: `function2<FilterResult(shared_ptr, string)>::operator()`; args stay engine-side.
    call()
}

// 0x9e4fc0 — __ZN3RBX7Network8PropSync6Master25onReceivedPropertyChangedENS_10Reflection13ConstPropertyE
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "RBX::Network::PropSync::Master::onReceivedPropertyChanged(RBX::Reflection::ConstProperty)")]
pub fn stub_9e4fc0(known: bool, value_differs: bool, bump: &mut dyn FnMut()) -> bool {
    // IDA 0x9e4fc0: ack-table lookup; a differing known entry bumps and applies.
    crate::replicator::on_received_property_changed(known, value_differs, bump)
}

// 0x9e50d4 — __ZNK5boost9function3IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEEclES6_SsS8_
// type: int __fastcall(int *, int *, const std::string *, int *)
#[doc(alias = "boost::function3<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant)const")]
pub fn stub_9e50d4(call: impl FnOnce() -> bool) -> bool {
    // IDA 0x9e50d4: `function3<FilterResult(shared_ptr, string, Variant)>::operator()`; args stay engine-side.
    call()
}

// 0x9e5540 — __ZN3RBX7Network8PropSync6Master17onPropertyChangedENS_10Reflection13ConstPropertyE
// type: void __fastcall(double *, int *, int, int)
#[doc(alias = "RBX::Network::PropSync::Master::onPropertyChanged(RBX::Reflection::ConstProperty)")]
pub fn stub_9e5540(
    entry: Option<crate::replicator::PropSyncItem>,
    now: f64,
    delay: f64,
) -> (crate::replicator::PropSyncItem, bool) {
    // IDA 0x9e5540: fresh entries stamp + queue; sent ones bump and re-stamp.
    crate::replicator::on_property_changed(entry, now, delay)
}

// 0x9e5700 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection18PropertyDescriptorEE4sendERN6RakNet9BitStreamEPKS3_
// type: unsigned int __fastcall(_DWORD *, RakNet::BitStream *this, unsigned int)
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::PropertyDescriptor>::send(RakNet::BitStream &,RBX::Reflection::PropertyDescriptor const*)const")]
pub fn stub_9e5700(
    sender: &crate::id_serializer::DescriptorSender,
    stream: &mut crate::bitstream::BitStream,
    descriptor: u32,
) {
    // IDA 0x9e5700: WriteBits(index, bits); unknown -> all-ones mask.
    sender.send_index(stream, descriptor);
}

// 0x9e57c0 — __ZN3RBX7Network8PropSync6Master14onPropertySendENS_10Reflection13ConstPropertyE
// type: bool __fastcall(double *, int *, int, int)
#[doc(alias = "RBX::Network::PropSync::Master::onPropertySend(RBX::Reflection::ConstProperty)")]
pub fn stub_9e57c0(master_allows: bool) -> bool {
    // IDA 0x9e57c0: PropSync::Master gate on the ConstProperty; the master table stays engine-side.
    master_allows
}

// 0x9e5928 — __ZN3RBX7Network8PropSync6Master25onReceivedAcknowledgementENS_10Reflection13ConstPropertyEi
// type: int __fastcall(_DWORD *, int *, __guard *, int)
#[doc(alias = "RBX::Network::PropSync::Master::onReceivedAcknowledgement(RBX::Reflection::ConstProperty,int)")]
pub fn stub_9e5928(known: bool, event_id: i32) -> i32 {
    // IDA 0x9e5928: ack-table hit records the event id, miss returns 0.
    crate::replicator::on_received_acknowledgement(known, event_id)
}

// 0x9e5a18 — __ZN5boost10shared_ptrIN3RBX7Network10Replicator9StreamJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob>::reset(void)")]
pub fn stub_9e5a18() {
    // IDA 0x9e5a18: `shared_ptr<StreamJob>::reset`; the slot stays engine-side.
}

// 0x9e5bb8 — __ZN3RBX7Network10Replicator19isLegalSendInstanceEPKNS_8InstanceE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::isLegalSendInstance(RBX::Instance const*)")]
pub fn stub_9e5bb8() -> bool {
    // IDA 0x9e5bb8..0x9e5bba: returns 1 unconditionally.
    crate::replicator::is_legal_send_instance()
}

// 0x9e5bc0 — __ZN3RBX7Network16ServerReplicator12canSendItemsEv
// type: int __fastcall(RBX::Network::ServerReplicator *this)
#[doc(alias = "RBX::Network::ServerReplicator::canSendItems(void)")]
pub fn stub_9e5bc0() -> bool {
    // IDA 0x9e5bc0..0x9e5bc2: returns 1 unconditionally.
    crate::replicator::can_send_items()
}

// 0x9e5cc0 — __ZN6RakNet16PluginInterface28OnDetachEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
#[doc(alias = "RakNet::PluginInterface2::OnDetach(void)")]
pub fn stub_9e5cc0() {
    // IDA 0x9e5cc0: `PluginInterface::OnDetach`; no crate state.
}

// 0x9e5cc8 — __ZN6RakNet16PluginInterface216OnPushBackPacketEPKcjNS_13SystemAddressE
// type: void()
#[doc(alias = "RakNet::PluginInterface2::OnPushBackPacket(char const*,unsigned int,RakNet::SystemAddress)")]
pub fn stub_9e5cc8() {
    // IDA 0x9e5cc8: `PluginInterface2::OnPushBackPacket`; no crate state.
}

// 0x9e5ce0 — __ZN3RBX10Reflection8EnumDescINS_7Network12FilterResultEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::~EnumDesc()")]
pub fn stub_9e5ce0() {
    // IDA 0x9e5ce0: `EnumDesc<FilterResult>` D1; descriptor state stays engine-side.
}

// 0x9e5cf0 — __ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE14convertToValueEmRNS0_7VariantE
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_9e5cf0() {
    // IDA 0x9e5cf0: `EnumDesc<FilterResult>::convertToValue`; the enum codec stays engine-side.
}

// 0x9e5db0 — __ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToString(unsigned long,std::string &)const")]
pub fn stub_9e5db0() {
    // IDA 0x9e5db0: `EnumDesc<FilterResult>::convertToString`; the enum codec stays engine-side.
}

// 0x9e5ef8 — __ZN3rbx14implementation12typed_holderIN3RBX7Network12FilterResultEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::FilterResult>::destruct_func(char *)")]
pub fn stub_9e5ef8() {
    // IDA 0x9e5ef8: `typed_holder<FilterResult>::destruct_func`; no crate state.
}

// 0x9e5f00 — __ZN3RBX10Reflection8EnumDescINS_7Network12FilterResultEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::~EnumDesc()")]
pub fn stub_9e5f00() {
    // IDA 0x9e5f00: `EnumDesc<FilterResult>` D2; descriptor state stays engine-side.
}

// 0x9e63f8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator9StreamJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> *,RBX::Network::Replicator::StreamJob *,boost::detail::shared_count &)")]
pub fn stub_9e63f8() -> ! {
    todo!("0x9e63f8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator9StreamJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

// 0x9e65a8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator9StreamJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::StreamJob,RBX::Network::Replicator::StreamJob>(rbx_core::SharedPtr<RBX::Network::Replicator::StreamJob> const*,RBX::Network::Replicator::StreamJob *)const")]
pub fn stub_9e65a8() -> ! {
    todo!("0x9e65a8 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator9StreamJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x9e6854 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p()")]
pub fn stub_9e6854() -> ! {
    todo!("0x9e6854 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED1Ev")
}

// 0x9e6858 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::~sp_counted_impl_p()")]
pub fn stub_9e6858() -> ! {
    todo!("0x9e6858 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEED0Ev")
}

// 0x9e6864 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::dispose(void)")]
pub fn stub_9e6864() -> ! {
    todo!("0x9e6864 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE7disposeEv")
}

// 0x9e6878 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_deleter(std::type_info const&)")]
pub fn stub_9e6878() -> ! {
    todo!("0x9e6878 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE11get_deleterERKSt9type_info")
}

// 0x9e687c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::StreamJob>::get_untyped_deleter(void)")]
pub fn stub_9e687c() -> ! {
    todo!("0x9e687c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator9StreamJobEE19get_untyped_deleterEv")
}

// 0x9e6b78 — __ZN5boost9function3IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function3<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::Reflection::Variant>::dummy::nonnull(void)")]
pub fn stub_9e6b78() -> ! {
    todo!("0x9e6b78 __ZN5boost9function3IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEE5dummy7nonnullEv")
}

// 0x9e765c — __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::dummy::nonnull(void)")]
pub fn stub_9e765c() -> ! {
    todo!("0x9e765c __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsE5dummy7nonnullEv")
}

// 0x9e7660 — __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_E5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")]
pub fn stub_9e7660() -> ! {
    todo!("0x9e7660 __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_E5dummy7nonnullEv")
}

// 0x9e7664 — __ZN5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")]
pub fn stub_9e7664() -> ! {
    todo!("0x9e7664 __ZN5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEE5dummy7nonnullEv")
}

// 0x9e7668 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4nextERNS2_13intrusive_ptrINSA_4slotEEE
// type: int __fastcall(int, int32_t **)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot> &)")]
pub fn stub_9e7668() -> ! {
    todo!("0x9e7668 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4nextERNS2_13intrusive_ptrINSA_4slotEEE")
}

// 0x9e7870 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotEEaSERKSD_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot> const&)")]
pub fn stub_9e7870() -> ! {
    todo!("0x9e7870 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotEEaSERKSD_")
}

// 0x9e7928 — __ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2IN6RakNet13SystemAddressEN5boost10shared_ptrINS2_17ConcurrentRakPeerEEEPNS_15NetworkSettingsEbEET_T0_T1_T2_
// type: int __fastcall(int, unsigned int, unsigned int, unsigned int, unsigned int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, struct _Unwind_Exception *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2IN6RakNet13SystemAddressEN5boost10shared_ptrINS2_17ConcurrentRakPeerEEEPNS_15NetworkSettingsEbEET_T0_T1_T2_")]
pub fn stub_9e7928() -> ! {
    todo!("0x9e7928 __ZN3RBX10Reflection9DescribedINS_7Network16ServerReplicatorELZNS2_17sServerReplicatorEENS_17NonFactoryProductINS2_10ReplicatorELZNS2_17sServerReplicatorEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EEC2IN6RakNet13SystemAddressEN5boost10shared_ptrINS2_17ConcurrentRakPeerEEEPNS_15NetworkSettingsEbEET_T0_T1_T2_")
}

// 0x9e7f18 — __ZN3RBX7Network10Replicator19isLegalReceiveEventEPNS_8InstanceERKNS_10Reflection15EventDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, const RBX::Reflection::EventDescriptor *)
#[doc(alias = "RBX::Network::Replicator::isLegalReceiveEvent(RBX::Instance *,RBX::Reflection::EventDescriptor const&)")]
pub fn stub_9e7f18() -> ! {
    todo!("0x9e7f18 __ZN3RBX7Network10Replicator19isLegalReceiveEventEPNS_8InstanceERKNS_10Reflection15EventDescriptorE")
}

// 0x9e7f20 — __ZN3RBX7Network10Replicator16rebroadcastEventERNS_10Reflection15EventInvocationE
// type: void()
#[doc(alias = "RBX::Network::Replicator::rebroadcastEvent(RBX::Reflection::EventInvocation &)")]
pub fn stub_9e7f20() -> ! {
    todo!("0x9e7f20 __ZN3RBX7Network10Replicator16rebroadcastEventERNS_10Reflection15EventInvocationE")
}

// 0x9e7f28 — __ZN3RBX7Network10Replicator20canReplicatePropertyERKNS_10Reflection13ConstPropertyE
// type: int()
#[doc(alias = "RBX::Network::Replicator::canReplicateProperty(RBX::Reflection::ConstProperty const&)")]
pub fn stub_9e7f28() -> ! {
    todo!("0x9e7f28 __ZN3RBX7Network10Replicator20canReplicatePropertyERKNS_10Reflection13ConstPropertyE")
}
