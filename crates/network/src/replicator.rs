//! `RBX::Network::ServerReplicator` packet helpers: character requests,
//! property acknowledgements, and SFFlag serialization.
//!
//! Decompiled from `readRequestCharacter` (0x9dcfb8),
//! `readPropAcknowledgement` (0x9dd5f8), and `serializeSFFlags` (0x9e2024).
//! Stream reads use [`crate::bitstream::BitStream`]; DataModel writes,
//! `PropSync`, and logging stay engine-side.

#![allow(dead_code)]

use super::bitstream::BitStream;

/// One `readRequestCharacter` packet (IDA 0x9dcfb8): the model id, the
/// player name, and the resolved remote-player instance. Resolution feeds
/// `processRequestCharacter` engine-side.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterRequest {
    pub model_id: u32,
    pub player_name: String,
    pub instance: u32,
}

/// `ServerReplicator::readRequestCharacter` (IDA 0x9dcfb8): reads a u32
/// model id, a string player name, and an instance ref; an unresolvable
/// ref throws `"Couldn't resolve remotePlayer %s from %s"` (panics here).
/// The verbose `Replication:` log and `processRequestCharacter` stay
/// engine-side.
pub fn read_request_character(
    stream: &mut BitStream,
    instance: Option<u32>,
    readable_guid: &str,
    address: &str,
) -> CharacterRequest {
    // IDA 0x9dcfb8: `operator>><uint>`, `operator>><std::string>`,
    // `deserializeInstanceRef`.
    let model_id = stream.read_u32().expect("BitStream >> unsigned int failed");
    let player_name = stream.read_string();
    match instance {
        Some(id) => CharacterRequest { model_id, player_name, instance: id },
        None => panic!("Couldn't resolve remotePlayer {readable_guid} from {address}"),
    }
}

/// `ServerReplicator::readPropAcknowledgement` outcome (IDA 0x9dd5f8):
/// reads an int index plus an instance ref; an unresolvable ref returns
/// the lookup verdict without touching `PropSync`. The
/// `descriptor.isMemberOf` assert and the
/// `PropSync::Master::onReceivedAcknowledgement(this + 5900, …)` forward
/// stay engine-side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropAckOutcome {
    Unresolved,
    Acknowledged { index: i32, instance: Option<u32>, descriptor: u32 },
}

pub fn prop_ack_outcome(
    resolved: bool,
    index: i32,
    instance: Option<u32>,
    descriptor: u32,
) -> PropAckOutcome {
    // IDA 0x9dd5f8: `deserializeInstanceRef != 1 → return it`; else pair
    // up `(descriptor, instance + 36)` and forward.
    if resolved {
        PropAckOutcome::Acknowledged { index, instance, descriptor }
    } else {
        PropAckOutcome::Unresolved
    }
}

/// `ServerReplicator::sendTop` header byte (IDA 0x9dbed4..0x9dbed6):
/// `operator<<(BitStream &, uchar)` with `0x81` (disasm).
pub const SEND_TOP_BYTE: u8 = 129;

/// `ServerReplicator::sendTop` (IDA 0x9dbe34): logs
/// "ServerReplicator:sendTop - begin", writes 129 plus the
/// streaming-enabled byte (disasm `operator<<(uchar)` /
/// `operator<<(bool)`, 0x9dbed6/0x9dbeea) with the optional flag word
/// behind virtual+192(18) (0x9dbf00..0x9dbf14), serializes each
/// replicable instance id (queuing `NewInstanceItem`s behind
/// `DelayAddTopReplicationInstance`; non-replicable ones are logged and
/// disconnected, 0x9dbf26..0x9dc424), and sends at priority 3
/// (0x9dc4d6). Instance iteration, replication vetting, and the send
/// stay engine-side.
pub fn send_top(
    stream: &mut BitStream,
    streaming_enabled: bool,
    extra_flag: Option<bool>,
    instance_ids: &[u32],
    serialize_id: &mut dyn FnMut(&mut BitStream, u32),
    queue_new_instance: &mut dyn FnMut(u32),
    send: &mut dyn FnMut(&mut BitStream),
) {
    stream.write_u8(SEND_TOP_BYTE);
    stream.write_bool(streaming_enabled);
    if let Some(flag) = extra_flag {
        stream.write_bool(flag);
    }
    for &id in instance_ids {
        serialize_id(stream, id);
        queue_new_instance(id);
    }
    send(stream);
}

/// `ServerReplicator::readPropAcknowledgement` (IDA 0x9dd5f8): reads the
/// event id int (0x9dd606), the descriptor index in `bits` (0x9dd618,
/// `vector::_M_range_check` on overflow), and the instance ref (a miss
/// returns 0, 0x9dd63a); on a hit asserts descriptor membership
/// (property.h:255, 0x9dd65e..0x9dd6b2) and forwards to
/// `PropSync::Master::onReceivedAcknowledgement` (0x9dd6b2).
/// Descriptor tables and `PropSync` stay engine-side behind `on_ack`.
/// Returns whether the ack applied.
pub fn read_prop_acknowledgement(
    stream: &mut BitStream,
    descriptor_bits: u32,
    descriptor_count: usize,
    instance_present: bool,
    member_matches: bool,
    on_ack: &mut dyn FnMut(i32) -> bool,
) -> bool {
    let event_id = stream.read_i32().expect("BitStream >> int failed");
    let index = stream.read_bits(descriptor_bits as u8).expect("BitStream ReadBits failed");
    assert!((index as usize) < descriptor_count, "vector::_M_range_check");
    if !instance_present {
        return false;
    }
    debug_assert!(
        member_matches,
        "!instance || descriptor.isMemberOf(instance) ../App/include/reflection/property.h line: 255"
    );
    on_ack(event_id)
}

/// One synchronized flag for [`serialize_sf_flags`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SynchronizedFlag<'a> {
    pub name: &'a str,
    pub value: &'a str,
}

/// `serializeSFFlag(name, value)` (IDA 0x9e2024 via
/// `FLog::ForEachVariable`): one flag as a string pair. The exact field
/// encoding follows the `operator<<` string framing.
pub fn serialize_sf_flag(stream: &mut BitStream, flag: SynchronizedFlag<'_>) {
    stream.write_string(flag.name);
    stream.write_string(flag.value);
}

/// `ServerReplicator::serializeSFFlags` (IDA 0x9e2024):
/// `Write<ushort>(GetNumSynchronizedVariable())`, then one
/// `serializeSFFlag` per flag (`FastVarType` 2).
pub fn serialize_sf_flags(stream: &mut BitStream, flags: &[SynchronizedFlag<'_>]) {
    // IDA 0x9e2024: count first (big-endian ushort template)…
    stream.write_u16(flags.len() as u16);
    // …then `ForEachVariable(serializeSFFlag, stream, 2)`.
    for flag in flags {
        serialize_sf_flag(stream, *flag);
    }
}

/// `ServerReplicator::processRequestCharacter` outcome (IDA 0x9dd6c8):
/// logs `"Received remotePlayer %s from %s"`, then a null remote logs +
/// throws, a mismatched remote logs `"RequestCharacter - RemotePlayer is
/// wrong"` + throws, else a virtual proceeds. Throws have no visible text
/// in the decompile, so this returns the branch instead of panicking.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharacterProcess {
    NullRemote,
    WrongRemote,
    Proceed,
}

pub fn process_request_character(remote_present: bool, remote_matches: bool) -> CharacterProcess {
    if !remote_present {
        return CharacterProcess::NullRemote;
    }
    if !remote_matches {
        return CharacterProcess::WrongRemote;
    }
    CharacterProcess::Proceed
}

/// `ServerReplicator::filterReceivedChangedProperty` verdict (IDA
/// 0x9ddef4): asserts the instance and its membership (ServerReplicator.cpp:1008,
/// property.h:255 — debug-only, engine-side), then `PropSync::Master`
/// acceptance short-circuits true; otherwise the `NetworkFilter` verdict
/// decides, defaulting to true with no filter. Returns whether to apply
/// the change.
pub fn filter_received_changed_property(prop_sync_accepted: bool, filter: Option<bool>) -> bool {
    // IDA 0x9ddef4: `v19 = 1; if (onReceived != 1) { filter path }`.
    if prop_sync_accepted {
        return true;
    }
    filter.unwrap_or(true)
}

/// `ServerReplicator::filterReceivedParent` (IDA 0x9dee84): asserts the
/// instance (ServerReplicator.cpp:1040, debug-only, engine-side), then the
/// `NetworkFilter::filterParent` verdict decides when a filter is
/// installed, defaulting to accept. Returns whether to apply the change.
pub fn filter_received_parent(filter: Option<bool>) -> bool {
    filter.unwrap_or(true)
}

/// `ServerReplicator::sendItemsPacket` (IDA 0x9dcbd8): the base
/// `Replicator::sendItemsPacket` runs first; on success with the
/// unbuffered path and the +6072 flag, up to `JoinSendExtraItemCount`
/// extra rounds run while the base keeps producing (0x9dcbf8..0x9dcc2e).
/// The queue and flags stay engine-side behind `base`.
pub fn send_items_packet(
    base: &mut dyn FnMut() -> bool,
    unbuffered: bool,
    extra_rounds: bool,
    extra_count: u8,
) -> bool {
    // IDA 0x9dcbe2..0x9dcbe8.
    if !base() {
        return false;
    }
    // IDA 0x9dcbf8..0x9dcc2e.
    if !(unbuffered && extra_rounds) {
        return true;
    }
    let mut sent = 0;
    while sent < extra_count {
        if !base() {
            return false;
        }
        sent += 1;
    }
    true
}

/// `ServerReplicator::installRemotePlayer` (IDA 0x9dc8e4): stamps the
/// remote player's network address, parents it under `Players`, logs
/// "ServerReplicator:InstallRemotePlayer - LoadCharacter", and — when
/// the +157 appearance flag is set — runs `Player::loadCharacter` with
/// the appearance name (0x9dc986..0x9dca02). Address/instance wiring
/// stays engine-side. Returns whether the character load ran.
pub fn install_remote_player(load_character: bool) -> bool {
    load_character
}

/// `writeChangedProperty` item type (IDA 0x9e013c:
/// `Item::writeItemType(stream, 3)`).
pub const CHANGED_PROPERTY_ITEM_TYPE: u8 = 3;

/// One `writeChangedProperty` packet (IDA 0x9e013c): the instance, the
/// property descriptor id, and the `onPropertySend(...) == 0` flag bit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChangedProperty {
    pub instance: Option<super::id_serializer::GuidData>,
    pub descriptor: u32,
    pub sync_flag: bool,
}

/// `ServerReplicator::writeChangedProperty` (IDA 0x9e013c): pairs
/// `(descriptor, instance + 36)` with a debug membership assert, gates on
/// the should-send predicate, then writes `[itemType = 3][id][propIndex]`
/// (verbose log + packet-count engine-side), the `onPropertySend == 0`
/// flag bit, and the value through the +312 virtual
/// (`serializePropertyValue`, engine-side value codec supplied by the
/// caller).
pub fn write_changed_property(
    stream: &mut BitStream,
    serializer: &mut super::id_serializer::IdSerializer,
    sender: &super::id_serializer::DescriptorSender,
    packet: &ChangedProperty,
    should_send: bool,
    write_value: impl FnOnce(&mut BitStream),
) {
    // IDA 0x9e013c: `if (shouldSend(this, pair) != 1) return` (nothing
    // written when gated).
    if !should_send {
        return;
    }
    super::item::write_item_type(stream, CHANGED_PROPERTY_ITEM_TYPE);
    serializer.serialize_id(stream, packet.instance);
    sender.send_index(stream, packet.descriptor);
    stream.write_bool(packet.sync_flag);
    write_value(stream);
}

/// `ServerReplicator::writeChangedRefProperty` (IDA 0x9e06cc): same packet
/// as [`write_changed_property`] plus the trailing ref-target guid —
/// a null name writes 8 zero bits, otherwise the full `serializeId`.
pub fn write_changed_ref_property(
    stream: &mut BitStream,
    serializer: &mut super::id_serializer::IdSerializer,
    sender: &super::id_serializer::DescriptorSender,
    packet: &ChangedProperty,
    should_send: bool,
    target: super::id_serializer::GuidData,
    write_value: impl FnOnce(&mut BitStream),
) {
    if !should_send {
        return;
    }
    super::item::write_item_type(stream, CHANGED_PROPERTY_ITEM_TYPE);
    serializer.serialize_id(stream, packet.instance);
    sender.send_index(stream, packet.descriptor);
    stream.write_bool(packet.sync_flag);
    // IDA 0x9e06cc tail: null target name → 8 zero bits, else `serializeId`.
    if target.name == super::id_serializer::NULL_NAME {
        stream.write_u8(0);
    } else {
        serializer.serialize_guid(stream, &target);
    }
    write_value(stream);
}

/// `ServerReplicator::onServiceProvider` (IDA 0x9e16cc): a live `StreamJob`
/// is dropped via `TaskScheduler::remove` (0x9e16fc..0x9e1844); with a new
/// provider a missing `Workspace` throws `runtime_error("ServerReplicator
/// unable to find workspace.")` (0x9e1bd8..0x9e1c1c), while a streaming
/// workspace starts the `StreamJob` on the scheduler
/// (0x9e187c..0x9e1aec); then the base `Replicator::onServiceProvider`
/// runs (0x9e1ba8). Scheduler/service wiring stays engine-side. Returns
/// the streaming flag written to +3720.
pub fn replicator_on_service_provider(
    new_provider: bool,
    workspace_present: bool,
    workspace_streaming: bool,
    parts_streaming_enabled: bool,
) -> bool {
    // IDA 0x9e1bd8..0x9e1c1c: the throw mirrors as a panic.
    if new_provider && !workspace_present {
        panic!("ServerReplicator unable to find workspace.");
    }
    // IDA 0x9e187c.
    new_provider && workspace_streaming && parts_streaming_enabled
}

/// `NetworkFilter::filterIfAssociatedWithOtherPlayer<1>` (IDA 0x9e29d8):
/// without `BasicNetworkCharacterFiltering` nothing filters
/// (0x9e2a38..0x9e2a3e). Otherwise the instance's enclosing `Player` is
/// found by parent walk (0x9e2a9e..0x9e2b18; none → pass); when the
/// player has a character differing from the instance, the change is
/// filtered and the out-param verdict set (0x9e2b2c..0x9e2b50).
/// Parent/character lookup stays engine-side. Returns
/// `(filter, verdict)`.
pub fn filter_if_associated_with_other_player(
    basic_filtering: bool,
    player_present: bool,
    character_present: bool,
    is_own_character: bool,
) -> (bool, bool) {
    if !basic_filtering || !player_present {
        return (false, false);
    }
    if character_present && !is_own_character {
        return (true, true);
    }
    (false, false)
}

/// `ServerReplicator::serializePropertyValue` (IDA 0x9e0c14): the
/// reflection type-switch over the value codec stays engine-side; this
/// runs the caller-supplied writer (the +312 virtual's payload).
pub fn serialize_property_value(stream: &mut BitStream, write: impl FnOnce(&mut BitStream)) {
    write(stream);
}

/// `ServerReplicator::deserializePropertyValue` (IDA 0x9e10f4): the
/// reflection type-switch stays engine-side; this runs the
/// caller-supplied reader.
pub fn deserialize_property_value(stream: &mut BitStream, mut read: impl FnMut(&mut BitStream)) {
    read(stream);
}

/// `RBX::Network::ServerReplicator::ServerStatsItem` (IDA 0x9e3194): the
/// per-replicator stats row created by `Creatable::create`; the live
/// counters stay engine-side, so the crate keeps a stateless marker.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ServerStatsItem;

/// `Creatable<Instance>::create<ServerStatsItem>` (IDA 0x9e3194).
pub fn create_stats_item() -> ServerStatsItem {
    ServerStatsItem
}

/// `Replicator::isLegalReceiveInstance` (IDA 0x9e4388): the body returns
/// 1 unconditionally (0x9e438a).
pub fn is_legal_receive_instance() -> bool {
    true
}

/// `PropSync::Master::onReceivedPropertyChanged` (IDA 0x9e4fc0): hashes
/// the (descriptor, instance) pair into the ack table (0x9e5030..0x9e5048);
/// a known entry whose stored value differs bumps the change counter and
/// returns true (0x9e50be..0x9e50c8), otherwise false. The table stays
/// engine-side; the caller supplies the lookup outcome and the counter.
pub fn on_received_property_changed(
    known: bool,
    value_differs: bool,
    bump: &mut dyn FnMut(),
) -> bool {
    if known && value_differs {
        bump();
        return true;
    }
    false
}

/// Spawn-name packet marker (IDA 0x9dca6c): a leading `143` (`0x8F`) byte
/// selects the initial-spawn-name parse (`IgnoreBits(8)` + log).
pub const SPAWN_NAME_BYTE: u8 = 143;

/// `ServerReplicator::OnReceive` verdict (IDA 0x9dca6c): an address
/// mismatch returns 1 (ignored); the spawn-name packet parses + logs and
/// returns 0; everything else forwards to `Replicator::OnReceive`
/// (engine-side).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiveVerdict {
    Ignored,
    SpawnName,
    Forward,
}

pub fn on_receive(address_matches: bool, first_byte: Option<u8>) -> ReceiveVerdict {
    // IDA 0x9dca6c: `SystemAddress::operator!=` → 1; `*payload == 143` →
    // spawn parse → 0; else delegate.
    if !address_matches {
        return ReceiveVerdict::Ignored;
    }
    if first_byte == Some(SPAWN_NAME_BYTE) {
        return ReceiveVerdict::SpawnName;
    }
    ReceiveVerdict::Forward
}

/// `ServerReplicator::receiveCluster` (IDA 0x9d86ec): pure tail-call to
/// `Replicator::receiveCluster` (engine-side); runs the caller-supplied
/// forward.
pub fn receive_cluster(forward: impl FnOnce()) {
    forward();
}

/// One `PropSync::Master` item (IDA 0x9e5540): the change version, whether
/// it was already sent, and the re-send stamp.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PropSyncItem {
    pub version: u32,
    pub version_sent: bool,
    pub stamp: f64,
}

/// `PropSync::Master::onPropertyChanged` (IDA 0x9e5540): emplaces the
/// property key into the master map; a fresh entry asserts `version == 0`
/// (PropertySynchronization.h:155) and `!isVersionSent` (:156), stamps
/// `now + delay`, and queues the key, while an existing version-sent
/// entry bumps the version, clears the flag, and re-stamps
/// (0x9e56d4..0x9e56f4). The map/queue stay engine-side. Returns the
/// resulting entry plus whether it was queued.
pub fn on_property_changed(
    entry: Option<PropSyncItem>,
    now: f64,
    delay: f64,
) -> (PropSyncItem, bool) {
    match entry {
        // IDA 0x9e55c6..0x9e56d2: fresh emplace, stamp, queue push.
        None => (
            PropSyncItem { version: 0, version_sent: false, stamp: now + delay },
            true,
        ),
        // IDA 0x9e56d4..0x9e56f4: existing entry re-arms only when sent.
        Some(mut item) => {
            if item.version_sent {
                item.version += 1;
                item.version_sent = false;
                item.stamp = now + delay;
            }
            (item, false)
        }
    }
}

/// `PropSync::Master::onReceivedAcknowledgement` (IDA 0x9e5928): hashes
/// the (descriptor, instance) pair into the ack table
/// (0x9e5996..0x9e59ae); on a key hit records the event id and returns
/// it (0x9e5a06..0x9e5a08), else returns 0. The table stays engine-side.
pub fn on_received_acknowledgement(known: bool, event_id: i32) -> i32 {
    if known {
        event_id
    } else {
        0
    }
}

/// `Replicator::isLegalSendInstance` (IDA 0x9e5bb8): returns 1
/// unconditionally (0x9e5bba).
pub fn is_legal_send_instance() -> bool {
    true
}

/// `ServerReplicator::canSendItems` (IDA 0x9e5bc0): returns 1
/// unconditionally (0x9e5bc2).
pub fn can_send_items() -> bool {
    true
}

/// `ServerReplicator::canUseProtocolVersion` (IDA 0x9d7414): an unset
/// minimum (+1516 == 0) accepts everything, else the peer version must
/// reach it (0x9d742e).
pub fn can_use_protocol_version(min_version: u32, version: u32) -> bool {
    min_version == 0 || min_version >= version
}

/// `ServerReplicator::isLegalDeleteInstance` (IDA 0x9d91d8): the +1505
/// parent filter runs first — a veto fires the delete signal and its
/// out-param decides (0x9d920e..0x9d986a); without a veto the +1527
/// functor decides, defaulting to legal with no functor
/// (0x9d9498..0x9d9888). Filter and signal stay engine-side.
pub fn is_legal_delete_instance(
    parent_filter_out: Option<bool>,
    signal_out: Option<bool>,
    functor_pass: Option<bool>,
) -> bool {
    if let Some(out) = parent_filter_out {
        // Veto path: empty slots fall back to the filter out-param.
        return !signal_out.unwrap_or(out);
    }
    functor_pass.unwrap_or(true)
}

/// `ServerReplicator::isLegalReceiveInstance` (IDA 0x9d9f78): already
/// registered script sources are refused (0x9da058..0x9da064), `Message`
/// instances are refused (0x9da118..0x9da122), and an existing remote
/// player throws `runtime_error("remotePlayer already exists")`
/// (0x9da966..0x9da9c2, mirrored as a panic). Survivors take the
/// [`is_legal_delete_instance`] filter/signal/functor path
/// (0x9da1e6..0x9da910). Reflection stays engine-side.
pub fn is_legal_receive_instance_filtered(
    script_registered: bool,
    message_veto: bool,
    remote_player_exists: bool,
    parent_filter_out: Option<bool>,
    signal_out: Option<bool>,
    functor_pass: Option<bool>,
) -> bool {
    if script_registered || message_veto {
        return false;
    }
    if remote_player_exists {
        panic!("remotePlayer already exists");
    }
    is_legal_delete_instance(parent_filter_out, signal_out, functor_pass)
}

/// `ServerReplicator::isLegalReceiveEvent` (IDA 0x9db1d4): local-player
/// events skip to the filter; otherwise the +1531 functor runs and the
/// receive signal fires (0x9db2fe..0x9db6e8). No functor means legal.
/// Reflection and the signal stay engine-side. Returns whether the event
/// passes.
pub fn is_legal_receive_event(filter: Option<bool>, fire_signal: &mut dyn FnMut()) -> bool {
    match filter {
        None => true,
        Some(pass) => {
            fire_signal();
            pass
        }
    }
}

/// Guarded receive properties (IDA 0x9dbb8c): `Player`s refuse `Name` and
/// `userId`; everything else refuses the script sources.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReceiveProp {
    Name,
    UserId,
    EmbeddedSource,
    SourceCodeId,
    Other,
}

/// `ServerReplicator::isLegalReceiveProperty` (IDA 0x9dbb8c): the
/// descriptor must avoid the guarded pair (0x9dbbd8..0x9dbcd8).
pub fn is_legal_receive_property(is_player: bool, prop: ReceiveProp) -> bool {
    match (is_player, prop) {
        (true, ReceiveProp::Name) | (true, ReceiveProp::UserId) => false,
        (false, ReceiveProp::EmbeddedSource) | (false, ReceiveProp::SourceCodeId) => false,
        _ => true,
    }
}

/// `ServerReplicator::isLegalSendProperty` (IDA 0x9dbd58): returns 1
/// unconditionally (0x9dbd5a).
pub fn is_legal_send_property() -> bool {
    true
}

/// Protocol-gated `Lighting` properties (IDA 0x9dbd5c).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightingProp {
    GlobalShadows,
    Outlines,
    OutdoorAmbient,
    Other,
}

/// `ServerReplicator::canReplicateProperty` (IDA 0x9dbd5c): non-`Lighting`
/// instances always replicate (0x9dbd6c); `GlobalShadows` needs protocol
/// > 9, `OutdoorAmbient` > 10, and `Outlines` > 13 (0x9dbd78..0x9dbe2e,
/// each logging the attempt).
pub fn can_replicate_property(is_lighting: bool, prop: LightingProp, protocol: i32) -> bool {
    if !is_lighting {
        return true;
    }
    match prop {
        LightingProp::GlobalShadows => protocol > 9,
        LightingProp::OutdoorAmbient => protocol > 10,
        LightingProp::Outlines => protocol > 13,
        LightingProp::Other => true,
    }
}

/// `ServerReplicator::onSentMarker` (IDA 0x9dbd20): builds the physics
/// sender (0x9dbd30), flushes a live `StreamJob` with `sendPackets(-1)`
/// (0x9dbd34..0x9dbd48), and clears +6072 (0x9dbd52). Sender/stream work
/// stays engine-side.
// BUG: the original at 0x9dbd54 returns the constant 6072 (the field
/// offset), not the cleared byte; both are truthy, so this returns true.
pub fn on_sent_marker(
    stream_job_present: bool,
    create_sender: &mut dyn FnMut(),
    send_packets: &mut dyn FnMut(),
) -> bool {
    create_sender();
    if stream_job_present {
        send_packets();
    }
    true
}

/// `ServerReplicator::filterPhysics` (IDA 0x9e0004): a passing base
/// `Replicator::filterPhysics` accepts (0x9e0012..0x9e0014); otherwise the
/// `CFrame` membership asserts (property.h:255) and the verdict comes
/// from `PropSync::Master::onReceivedPropertyChanged` (0x9e0090).
pub fn filter_physics(
    base_pass: bool,
    member_matches: bool,
    changed_known: bool,
    changed_differs: bool,
    bump: &mut dyn FnMut(),
) -> bool {
    if base_pass {
        return true;
    }
    debug_assert!(
        member_matches,
        "!instance || descriptor.isMemberOf(instance) ../App/include/reflection/property.h line: 255"
    );
    on_received_property_changed(changed_known, changed_differs, bump)
}

/// `ServerReplicator::dataOutStep` (IDA 0x9e0098): expires the master
/// items (0x9e00a4), then runs the base `Replicator::dataOutStep`
/// (0x9e00aa). Queue work stays engine-side behind the closures.
pub fn data_out_step(expire_items: &mut dyn FnMut(), base_step: &mut dyn FnMut()) {
    expire_items();
    base_step();
}

/// `Replicator::StatsItem::update` (IDA 0x9e9c98): refreshes the stats
/// rows (`formatValue`/`formatRate`/`formatMem` over receiver,
/// replicator, and peer counters, 0x9e9d4c..0x9e9f22) behind the +56
/// owner lock. All rows stay engine-side; the caller runs the refresh.
pub fn base_stats_item_update(refresh: &mut dyn FnMut()) {
    refresh();
}

/// `ServerReplicator::ServerStatsItem::update` (IDA 0x9e9728): runs the
/// base update first (0x9e9748), then the per-replicator rows
/// (`formatValue` over the +5916/+1576/+820/+780/+524 counters,
/// 0x9e9890..0x9e9922) when the replicator is present. Rows stay
/// engine-side behind the closures.
pub fn server_stats_item_update(
    replicator_present: bool,
    base_refresh: &mut dyn FnMut(),
    extra_refresh: &mut dyn FnMut(),
) {
    base_refresh();
    if replicator_present {
        extra_refresh();
    }
}

/// `ServerReplicator::readPlayerSimulationRegion` (IDA 0x9d8700): without
/// a player, or a player without a character head, there is no region
/// (0x9d871e..0x9d8736). Otherwise the head's `xz` plus the radius select
/// the range and the `StreamJob` adjusts it (0x9d873e..0x9d875a,
/// engine-side). Returns the head part id.
pub fn read_player_simulation_region(
    head: Option<(u32, f32, f32, f32)>,
    adjust: &mut dyn FnMut(f32, f32, f32),
) -> Option<u32> {
    let (id, x, z, radius) = head?;
    adjust(x, z, radius);
    Some(id)
}

/// `ServerReplicator::checkDistributedReceive` (IDA 0x9d8768): the root
/// moving part's mechanism must be this part's, and its network owner
/// must be this replicator's address (0x9d877a..0x9d87b4).
pub fn check_distributed_receive(owner_is_self: bool, mechanism_matches: bool) -> bool {
    owner_is_self && mechanism_matches
}

/// `ServerReplicator::checkDistributedSend` (IDA 0x9d87b8): asserts the
/// part (ServerReplicator.cpp:231); without a root mechanism the match
/// fails, otherwise the owner must differ and the mechanism must match
/// (0x9d8824..0x9d885a).
pub fn check_distributed_send(
    part_present: bool,
    mechanism_matches: bool,
    owner_is_self: bool,
) -> bool {
    debug_assert!(part_present, "part Client/Network/ServerReplicator.cpp line: 231");
    mechanism_matches && !owner_is_self
}

/// `ServerReplicator::checkDistributedSendFast` (IDA 0x9d885c): asserts
/// the part (:243) and the mechanism-root invariant (:244), then reports
/// whether the owner differs (0x9d8906..0x9d8920).
pub fn check_distributed_send_fast(
    part_present: bool,
    root_matches: bool,
    owner_is_self: bool,
) -> bool {
    debug_assert!(part_present, "part Client/Network/ServerReplicator.cpp line: 243");
    debug_assert!(
        root_matches,
        "getConstMechanismRootMovingPart(part) == part Client/Network/ServerReplicator.cpp line: 244"
    );
    !owner_is_self
}

/// `ServerReplicator::rebroadcastEvent` (IDA 0x9d8924): forwards to
/// `EventInvocation::replicateEvent` (0x9d892e, engine-side).
pub fn rebroadcast_event(replicated: bool) -> bool {
    replicated
}

/// `Replicator::rebroadcastEvent` (IDA 0x9e7f20): empty body.
pub fn base_rebroadcast_event() {}

/// `Replicator::isLegalReceiveEvent` (IDA 0x9e7f18): returns 1
/// unconditionally (0x9e7f1a).
pub fn base_is_legal_receive_event() -> bool {
    true
}

/// `Replicator::canReplicateProperty` (IDA 0x9e7f28): returns 1
/// unconditionally (0x9e7f2a).
pub fn base_can_replicate_property() -> bool {
    true
}

/// `ServerReplicator::shouldDelayAddingToWorld` (IDA 0x9d8930):
/// non-`Player` instances never delay (0x9d8962..0x9d8a32). A `Player`
/// with an existing remote throws `runtime_error("remotePlayer already
/// exists")` (0x9d8a9a..0x9d8d6e, mirrored as a panic); otherwise the
/// remote installs with stream listeners and an early spawn calculation
/// (engine-side), and the verdict is whether player authentication is
/// enabled with its flag set (0x9d8c28..0x9d8c46).
pub fn should_delay_adding_to_world(
    is_player: bool,
    remote_exists: bool,
    auth_enabled: bool,
    auth_flag: bool,
    installed: &mut dyn FnMut(),
) -> bool {
    if !is_player {
        return false;
    }
    if remote_exists {
        panic!("remotePlayer already exists");
    }
    installed();
    auth_enabled && auth_flag
}

/// `ServerReplicator::addTopReplicationContainer` (IDA 0x9d8ef0): runs
/// the base `Replicator::addTopReplicationContainer` (0x9d8f8c,
/// engine-side), then fires the completion functor unless
/// `DelayAddTopReplicationInstance` defers it (0x9d8fc4..0x9d8fe6).
pub fn add_top_replication_container(
    base_add: &mut dyn FnMut(),
    delay_flag: bool,
    immediate: bool,
    on_added: &mut dyn FnMut(),
) {
    base_add();
    if !delay_flag || immediate {
        on_added();
    }
}

/// `ServerReplicator::readItem` dispatch (IDA 0x9dcc34): 8 reads a
/// character request, 9 logs `"Rocky item found"` and throws
/// `runtime_error("rocky")`, 0xA reads a prop acknowledgement, 0xC reads
/// an `(int, short)` quota update, 0xE/0xF read region/instance removals,
/// and anything else falls through to `Replicator::readItem`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncomingItem {
    RequestCharacter,
    PropAck,
    Quota,
    RegionRemoval,
    InstanceRemoval,
    Base(u8),
}

pub fn read_item_kind(item_type: u8) -> IncomingItem {
    match item_type {
        8 => IncomingItem::RequestCharacter,
        // IDA 0x9dcc34 case 9: log + `throw runtime_error("rocky")`.
        9 => panic!("rocky"),
        0xA => IncomingItem::PropAck,
        0xC => IncomingItem::Quota,
        0xE => IncomingItem::RegionRemoval,
        0xF => IncomingItem::InstanceRemoval,
        other => IncomingItem::Base(other),
    }
}

/// The 0xC arm's quota read (IDA 0x9dcc34): `operator>><int>` then
/// `operator>><short>`, forwarded to `StreamJob::updateClientQuota`.
pub fn read_quota(stream: &mut BitStream) -> (i32, i16) {
    let a = stream.read_i32().expect("BitStream >> int failed");
    let b = stream.read_i16().expect("BitStream >> short failed");
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request_bytes(model_id: u32, name: &str) -> Vec<u8> {
        let mut s = BitStream::new();
        s.write_u32(model_id);
        s.write_string(name);
        s.into_bytes()
    }

    #[test]
    fn request_character_roundtrip() {
        let mut r = BitStream::from_bytes(&request_bytes(7, "builder"));
        let req = read_request_character(&mut r, Some(42), "guid", "addr");
        assert_eq!(
            req,
            CharacterRequest { model_id: 7, player_name: "builder".to_owned(), instance: 42 }
        );
    }

    #[test]
    #[should_panic(expected = "Couldn't resolve remotePlayer guid-9 from 1.2.3.4")]
    fn request_character_unresolved_throws() {
        let mut r = BitStream::from_bytes(&request_bytes(7, "builder"));
        let _ = read_request_character(&mut r, None, "guid-9", "1.2.3.4");
    }

    #[test]
    fn prop_ack_branches() {
        assert_eq!(prop_ack_outcome(false, 3, None, 11), PropAckOutcome::Unresolved);
        assert_eq!(
            prop_ack_outcome(true, 3, Some(9), 11),
            PropAckOutcome::Acknowledged { index: 3, instance: Some(9), descriptor: 11 }
        );
    }

    #[test]
    fn sf_flags_framing() {
        let mut s = BitStream::new();
        serialize_sf_flags(
            &mut s,
            &[SynchronizedFlag { name: "a", value: "1" }, SynchronizedFlag { name: "bb", value: "" }],
        );
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_u16(), Some(2));
        // Per-flag pairs follow in order (see `serialize_sf_flag`).
        assert_eq!(r.read_string(), "a");
        assert_eq!(r.read_string(), "1");
        assert_eq!(r.read_string(), "bb");
        assert_eq!(r.read_string(), "");
    }

    #[test]
    fn process_branches() {
        assert_eq!(process_request_character(false, false), CharacterProcess::NullRemote);
        assert_eq!(process_request_character(true, false), CharacterProcess::WrongRemote);
        assert_eq!(process_request_character(true, true), CharacterProcess::Proceed);
    }

    #[test]
    fn filter_verdicts() {
        assert!(filter_received_changed_property(true, Some(false)));
        assert!(filter_received_changed_property(false, None));
        assert!(filter_received_changed_property(false, Some(true)));
        assert!(!filter_received_changed_property(false, Some(false)));
    }

    #[test]
    fn parent_filter_defaults_accept() {
        assert!(filter_received_parent(None));
        assert!(filter_received_parent(Some(true)));
        assert!(!filter_received_parent(Some(false)));
    }

    #[test]
    fn changed_property_packet_layout() {
        // IDA 0x9e013c: gate + `[3][id][propIndex][sync][value]`.
        use super::super::id_serializer::{DescriptorSender, GuidData, IdSerializer};
        let mut serializer = IdSerializer::new();
        serializer.set_max_guid_index_bit(32);
        let name = serializer.declare_name("Part");
        let sender = DescriptorSender::new(&[7]);
        let packet = ChangedProperty {
            instance: Some(GuidData { name, index: 1 }),
            descriptor: 7,
            sync_flag: true,
        };
        // Gated off: nothing written.
        let mut s = BitStream::new();
        write_changed_property(&mut s, &mut serializer, &sender, &packet, false, |_| panic!("gated"));
        assert_eq!(s.bits_written(), 0);
        // Open: decode field by field. Fresh sender: code byte 0x80,
        // `"Part"` string, 32 index bits; prop index 0 in 1 bit; flag; value.
        let mut s = BitStream::new();
        write_changed_property(&mut s, &mut serializer, &sender, &packet, true, |st| st.write_u8(0xAA));
        assert_eq!(s.bits_written(), 2 + 8 + 64 + 32 + 1 + 1 + 8);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_bits(2), Some(3));
        assert_eq!(r.read_u8(), Some(0x80));
        assert_eq!(r.read_string(), "Part");
        assert_eq!(r.read_bits(32), Some(1));
        assert_eq!(r.read_bits(1), Some(0));
        assert_eq!(r.read_bit(), Some(true));
        assert_eq!(r.read_u8(), Some(0xAA));
    }

    #[test]
    fn changed_ref_property_null_target_is_one_zero_byte() {
        use super::super::id_serializer::{DescriptorSender, GuidData, IdSerializer, NULL_NAME};
        let mut serializer = IdSerializer::new();
        serializer.set_max_guid_index_bit(32);
        let name = serializer.declare_name("Part");
        let sender = DescriptorSender::new(&[7]);
        let packet = ChangedProperty {
            instance: Some(GuidData { name, index: 1 }),
            descriptor: 7,
            sync_flag: false,
        };
        // IDA 0x9e06cc tail: null target name → a single zero byte, then the value.
        let mut s = BitStream::new();
        write_changed_ref_property(
            &mut s,
            &mut serializer,
            &sender,
            &packet,
            true,
            GuidData { name: NULL_NAME, index: 0 },
            |st| st.write_u8(0xBB),
        );
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_bits(2), Some(3));
        assert_eq!(r.read_u8(), Some(0x80));
        assert_eq!(r.read_string(), "Part");
        assert_eq!(r.read_bits(32), Some(1));
        assert_eq!(r.read_bits(1), Some(0));
        assert_eq!(r.read_bit(), Some(false));
        // IDA 0x9e06cc tail: null target name → a single zero byte…
        assert_eq!(r.read_u8(), Some(0));
        // …then the value.
        assert_eq!(r.read_u8(), Some(0xBB));
    }

    #[test]
    fn property_value_hooks_delegate() {
        let mut s = BitStream::new();
        serialize_property_value(&mut s, |st| st.write_u8(9));
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut seen = 0u8;
        deserialize_property_value(&mut r, |st| seen = st.read_u8().expect("byte"));
        assert_eq!(seen, 9);
    }

    #[test]
    fn receive_verdicts() {
        // IDA 0x9dca6c: mismatch ignores, 143 parses the spawn name, else forward.
        assert_eq!(on_receive(false, None), ReceiveVerdict::Ignored);
        assert_eq!(on_receive(false, Some(143)), ReceiveVerdict::Ignored);
        assert_eq!(on_receive(true, Some(143)), ReceiveVerdict::SpawnName);
        assert_eq!(on_receive(true, Some(0)), ReceiveVerdict::Forward);
        assert_eq!(on_receive(true, None), ReceiveVerdict::Forward);
        let mut forwarded = false;
        receive_cluster(|| forwarded = true);
        assert!(forwarded);
    }

    #[test]
    fn item_dispatch_table() {
        use IncomingItem::*;
        assert_eq!(read_item_kind(8), RequestCharacter);
        assert_eq!(read_item_kind(0xA), PropAck);
        assert_eq!(read_item_kind(0xC), Quota);
        assert_eq!(read_item_kind(0xE), RegionRemoval);
        assert_eq!(read_item_kind(0xF), InstanceRemoval);
        assert_eq!(read_item_kind(3), Base(3));
    }

    #[test]
    #[should_panic(expected = "rocky")]
    fn rocky_item_throws() {
        let _ = read_item_kind(9);
    }

    #[test]
    fn quota_reads_int_then_short() {
        let mut s = BitStream::new();
        s.write_i32(-70000);
        s.write_i16(1234);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(read_quota(&mut r), (-70000, 1234));
    }

    #[test]
    fn send_top_frames_header_and_instances() {
        // IDA 0x9dbe34: 129 + streaming byte + optional flag, per-id serialize/queue, send.
        let mut s = BitStream::new();
        let mut seen = Vec::new();
        let mut queued = Vec::new();
        let mut sent = false;
        send_top(
            &mut s, true, Some(false), &[7, 9],
            &mut |st, id| { seen.push(id); st.write_u32(id); },
            &mut |id| queued.push(id),
            &mut |_| sent = true,
        );
        assert!(sent);
        assert_eq!(seen, vec![7, 9]);
        assert_eq!(queued, vec![7, 9]);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_u8(), Some(SEND_TOP_BYTE));
        assert_eq!(r.read_bool(), Some(true));
        assert_eq!(r.read_bool(), Some(false));
        assert_eq!(r.read_u32(), Some(7));
    }

    #[test]
    fn prop_ack_reads_event_index_and_ref() {
        // IDA 0x9dd5f8: event id, bit index, instance gate, PropSync forward.
        let mut s = BitStream::new();
        s.write_i32(42);
        s.write_bits(2, 2);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut acked = -1;
        let applied = read_prop_acknowledgement(&mut r, 2, 4, true, true, &mut |id| { acked = id; true });
        assert!(applied);
        assert_eq!(acked, 42);
        let mut s = BitStream::new();
        s.write_i32(1);
        s.write_bits(0, 2);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert!(!read_prop_acknowledgement(&mut r, 2, 4, false, true, &mut |_| panic!("no forward on miss")));
    }

    #[test]
    fn items_packet_extras_need_base() {
        // IDA 0x9dcbd8: base miss -> false; extras stop at the first base miss.
        assert!(!send_items_packet(&mut || false, true, true, 3));
        assert!(send_items_packet(&mut || true, false, false, 0));
        let mut calls = 0;
        assert!(!send_items_packet(&mut || { calls += 1; calls < 3 }, true, true, 5));
        assert_eq!(calls, 3);
        let mut calls = 0;
        assert!(send_items_packet(&mut || { calls += 1; true }, true, true, 2));
        assert_eq!(calls, 3);
    }

    #[test]
    fn remote_install_reports_character_load() {
        // IDA 0x9dc8e4: the +157 flag gates loadCharacter.
        assert!(install_remote_player(true));
        assert!(!install_remote_player(false));
    }

    #[test]
    fn provider_drives_streaming_flag() {
        // IDA 0x9e16cc: no provider -> false; streaming workspace -> true.
        assert!(!replicator_on_service_provider(false, false, false, false));
        assert!(!replicator_on_service_provider(true, true, false, true));
        assert!(!replicator_on_service_provider(true, true, true, false));
        assert!(replicator_on_service_provider(true, true, true, true));
    }

    #[test]
    #[should_panic(expected = "unable to find workspace")]
    fn provider_without_workspace_throws() {
        // IDA 0x9e1bd8: runtime_error mirrors as a panic.
        let _ = replicator_on_service_provider(true, false, false, false);
    }

    #[test]
    fn associated_filter_needs_other_character() {
        // IDA 0x9e29d8: flag off / no player / own character pass; other's filters.
        assert_eq!(filter_if_associated_with_other_player(false, true, true, false), (false, false));
        assert_eq!(filter_if_associated_with_other_player(true, false, true, false), (false, false));
        assert_eq!(filter_if_associated_with_other_player(true, true, false, false), (false, false));
        assert_eq!(filter_if_associated_with_other_player(true, true, true, true), (false, false));
        assert_eq!(filter_if_associated_with_other_player(true, true, true, false), (true, true));
    }

    #[test]
    fn legal_stats_and_changed_ack() {
        // IDA 0x9e4388/0x9e3194/0x9e4fc0.
        assert!(is_legal_receive_instance());
        assert_eq!(create_stats_item(), ServerStatsItem);
        let mut bumps = 0;
        assert!(on_received_property_changed(true, true, &mut || bumps += 1));
        assert_eq!(bumps, 1);
        assert!(!on_received_property_changed(true, false, &mut || bumps += 1));
        assert!(!on_received_property_changed(false, true, &mut || bumps += 1));
        assert_eq!(bumps, 1);
    }

    #[test]
    fn property_change_fresh_queues_sent_bumps() {
        // IDA 0x9e5540: fresh stamps + queues; sent re-arms; quiet entries rest.
        let (item, queued) = on_property_changed(None, 10.0, 1.5);
        assert_eq!(item, PropSyncItem { version: 0, version_sent: false, stamp: 11.5 });
        assert!(queued);
        let sent = PropSyncItem { version: 3, version_sent: true, stamp: 0.0 };
        let (item, queued) = on_property_changed(Some(sent), 20.0, 2.0);
        assert_eq!(item, PropSyncItem { version: 4, version_sent: false, stamp: 22.0 });
        assert!(!queued);
        let quiet = PropSyncItem { version: 1, version_sent: false, stamp: 5.0 };
        assert_eq!(on_property_changed(Some(quiet), 20.0, 2.0), (quiet, false));
    }

    #[test]
    fn received_ack_records_on_hit() {
        // IDA 0x9e5928: hit returns the event id, miss returns 0.
        assert_eq!(on_received_acknowledgement(true, 42), 42);
        assert_eq!(on_received_acknowledgement(false, 42), 0);
        assert!(is_legal_send_instance());
        assert!(can_send_items());
    }

    #[test]
    fn legal_gates_compose() {
        // IDA 0x9d7414/0x9d91d8/0x9d9f78/0x9db1d4/0x9dbb8c/0x9dbd58.
        assert!(can_use_protocol_version(0, 99));
        assert!(can_use_protocol_version(12, 12));
        assert!(can_use_protocol_version(12, 11));
        assert!(!can_use_protocol_version(12, 13));
        assert!(is_legal_delete_instance(None, None, None));
        assert!(!is_legal_delete_instance(None, None, Some(false)));
        assert!(!is_legal_delete_instance(Some(true), None, None));
        assert!(is_legal_delete_instance(Some(true), Some(false), None));
        assert!(!is_legal_receive_instance_filtered(true, false, false, None, None, None));
        assert!(!is_legal_receive_instance_filtered(false, true, false, None, None, None));
        assert!(is_legal_receive_instance_filtered(false, false, false, None, None, Some(true)));
        let mut fired = false;
        assert!(is_legal_receive_event(None, &mut || fired = true));
        assert!(!fired);
        assert!(!is_legal_receive_event(Some(false), &mut || fired = true));
        assert!(fired);
        assert!(!is_legal_receive_property(true, ReceiveProp::Name));
        assert!(!is_legal_receive_property(true, ReceiveProp::UserId));
        assert!(is_legal_receive_property(true, ReceiveProp::Other));
        assert!(!is_legal_receive_property(false, ReceiveProp::EmbeddedSource));
        assert!(!is_legal_receive_property(false, ReceiveProp::SourceCodeId));
        assert!(is_legal_receive_property(false, ReceiveProp::Other));
        assert!(is_legal_send_property());
    }

    #[test]
    #[should_panic(expected = "remotePlayer already exists")]
    fn receive_instance_duplicate_remote_throws() {
        // IDA 0x9da966: the duplicate remote throw mirrors as a panic.
        let _ = is_legal_receive_instance_filtered(false, false, true, None, None, None);
    }

    #[test]
    fn replicate_thresholds_and_physics() {
        // IDA 0x9dbd5c/0x9e0004/0x9dbd20/0x9e0098.
        assert!(can_replicate_property(false, LightingProp::GlobalShadows, 0));
        assert!(can_replicate_property(true, LightingProp::Other, 0));
        assert!(!can_replicate_property(true, LightingProp::GlobalShadows, 9));
        assert!(can_replicate_property(true, LightingProp::GlobalShadows, 10));
        assert!(!can_replicate_property(true, LightingProp::OutdoorAmbient, 10));
        assert!(can_replicate_property(true, LightingProp::OutdoorAmbient, 11));
        assert!(!can_replicate_property(true, LightingProp::Outlines, 13));
        assert!(can_replicate_property(true, LightingProp::Outlines, 14));
        assert!(filter_physics(true, false, false, false, &mut || panic!("short-circuit")));
        let mut bumps = 0;
        assert!(filter_physics(false, true, true, true, &mut || bumps += 1));
        assert_eq!(bumps, 1);
        let order = core::cell::RefCell::new(Vec::new());
        on_sent_marker(true, &mut || order.borrow_mut().push("sender"), &mut || order.borrow_mut().push("packets"));
        assert_eq!(order.into_inner(), vec!["sender", "packets"]);
        let order = core::cell::RefCell::new(Vec::new());
        data_out_step(&mut || order.borrow_mut().push("expire"), &mut || order.borrow_mut().push("base"));
        assert_eq!(order.into_inner(), vec!["expire", "base"]);
    }

    #[test]
    fn distributed_checks_need_ownership() {
        // IDA 0x9d8700/0x9d8768/0x9d87b8/0x9d885c/0x9d8924.
        assert_eq!(read_player_simulation_region(None, &mut |_, _, _| panic!("no adjust")), None);
        let mut adjusted = (0.0, 0.0, 0.0);
        let head = read_player_simulation_region(
            Some((7, 1.0, 2.0, 30.0)),
            &mut |x, z, r| adjusted = (x, z, r),
        );
        assert_eq!(head, Some(7));
        assert_eq!(adjusted, (1.0, 2.0, 30.0));
        assert!(check_distributed_receive(true, true));
        assert!(!check_distributed_receive(true, false));
        assert!(!check_distributed_receive(false, true));
        assert!(check_distributed_send(true, true, false));
        assert!(!check_distributed_send(true, true, true));
        assert!(!check_distributed_send(true, false, false));
        assert!(check_distributed_send_fast(true, true, false));
        assert!(!check_distributed_send_fast(true, true, true));
        assert!(rebroadcast_event(true));
        assert!(!rebroadcast_event(false));
        base_rebroadcast_event();
        assert!(base_is_legal_receive_event());
        assert!(base_can_replicate_property());
    }

    #[test]
    fn delay_needs_player_and_auth() {
        // IDA 0x9d8930: non-players pass through; auth gates the delay.
        let mut installed = false;
        assert!(!should_delay_adding_to_world(false, false, true, true, &mut || installed = true));
        assert!(!installed);
        assert!(should_delay_adding_to_world(true, false, true, true, &mut || installed = true));
        assert!(installed);
        assert!(!should_delay_adding_to_world(true, false, true, false, &mut || {}));
        assert!(!should_delay_adding_to_world(true, false, false, true, &mut || {}));
    }

    #[test]
    #[should_panic(expected = "remotePlayer already exists")]
    fn delay_duplicate_remote_throws() {
        // IDA 0x9d8a9a: the duplicate remote throw mirrors as a panic.
        let _ = should_delay_adding_to_world(true, true, false, false, &mut || {});
    }

    #[test]
    fn top_container_defers_behind_flag() {
        // IDA 0x9d8ef0: base always runs; the functor fires unless deferred.
        let order = core::cell::RefCell::new(Vec::new());
        add_top_replication_container(
            &mut || order.borrow_mut().push("base"),
            true, false,
            &mut || order.borrow_mut().push("added"),
        );
        assert_eq!(order.into_inner(), vec!["base"]);
        let order = core::cell::RefCell::new(Vec::new());
        add_top_replication_container(
            &mut || order.borrow_mut().push("base"),
            false, false,
            &mut || order.borrow_mut().push("added"),
        );
        assert_eq!(order.into_inner(), vec!["base", "added"]);
    }

    #[test]
    fn stats_update_runs_base_then_extras() {
        // IDA 0x9e9728/0x9e9c98: base rows first, extras only when present.
        let order = core::cell::RefCell::new(Vec::new());
        server_stats_item_update(
            true,
            &mut || order.borrow_mut().push("base"),
            &mut || order.borrow_mut().push("extra"),
        );
        assert_eq!(order.into_inner(), vec!["base", "extra"]);
        let order = core::cell::RefCell::new(Vec::new());
        server_stats_item_update(
            false,
            &mut || order.borrow_mut().push("base"),
            &mut || order.borrow_mut().push("extra"),
        );
        assert_eq!(order.into_inner(), vec!["base"]);
        let mut ran = false;
        base_stats_item_update(&mut || ran = true);
        assert!(ran);
    }
    #[test]
    fn client_replicator_dispatch_gates() {
        // IDA 0x97afc8/0x97b010: ctor no-op.
        init_client_replicator();
        // IDA 0x97be3c: mismatch only from the server with kind 142.
        let order = core::cell::RefCell::new(Vec::new());
        let mut recv = |server: bool, kind: u8| {
            client_replicator_on_receive(
                server,
                kind,
                &mut || { order.borrow_mut().push("base"); 7 },
                &mut || order.borrow_mut().push("mm"),
            )
        };
        assert_eq!(recv(false, 142), 7);
        assert_eq!(recv(true, 100), 7);
        assert_eq!(recv(true, 142), 1);
        assert_eq!(order.borrow().as_slice(), ["base", "base", "mm"]);
        // IDA 0x97c3fc/0x97ca44/0x97cf08: config/cluster/item gates.
        let order = core::cell::RefCell::new(Vec::new());
        process_packet(false, &mut || order.borrow_mut().push("cfg"), &mut || order.borrow_mut().push("base"));
        process_packet(true, &mut || order.borrow_mut().push("cfg"), &mut || order.borrow_mut().push("base"));
        client_receive_cluster(false, &mut || order.borrow_mut().push("count"), &mut || order.borrow_mut().push("base"));
        client_receive_cluster(true, &mut || order.borrow_mut().push("count"), &mut || order.borrow_mut().push("base"));
        read_client_item(7, &mut || order.borrow_mut().push("stream"), &mut || order.borrow_mut().push("base"));
        read_client_item(13, &mut || order.borrow_mut().push("stream"), &mut || order.borrow_mut().push("base"));
        assert_eq!(order.borrow().as_slice(), ["base", "cfg", "base", "count", "base", "base", "stream"]);
        // IDA 0x97cf1c: step dispatch plus join accumulation.
        let order = core::cell::RefCell::new(Vec::new());
        read_stream_data(0, &mut || order.borrow_mut().push("region"), &mut || order.borrow_mut().push("adv"), 3, &mut |n| order.borrow_mut().push(if n == 3 { "j3" } else { "j?" }));
        read_stream_data(2, &mut || order.borrow_mut().push("region"), &mut || order.borrow_mut().push("adv"), 0, &mut |n| order.borrow_mut().push(if n == 0 { "j0" } else { "j?" }));
        assert_eq!(order.borrow().as_slice(), ["region", "j3", "adv", "j0"]);
    }
    #[test]
    fn client_changed_property_gates() {
        // IDA 0x97e7b4: gated ack write.
        let order = core::cell::RefCell::new(Vec::new());
        write_prop_acknowledgement(true, &mut || order.borrow_mut().push("w"), &mut || order.borrow_mut().push("s"));
        write_prop_acknowledgement(false, &mut || order.borrow_mut().push("w"), &mut || order.borrow_mut().push("s"));
        assert_eq!(order.borrow().as_slice(), ["w", "s"]);
        // IDA 0x97e88c/0x97e8b4: ack then base.
        let order = core::cell::RefCell::new(Vec::new());
        write_client_changed_property(&mut || order.borrow_mut().push("ack"), &mut || order.borrow_mut().push("base"));
        assert_eq!(order.borrow().as_slice(), ["ack", "base"]);
        // IDA 0x97d444: streaming parent skips base, cframe runs both.
        let order = core::cell::RefCell::new(Vec::new());
        read_client_changed_property(false, true, false, &mut || order.borrow_mut().push("sp"), &mut || order.borrow_mut().push("base"));
        read_client_changed_property(true, true, false, &mut || order.borrow_mut().push("sp"), &mut || order.borrow_mut().push("base"));
        read_client_changed_property(true, false, true, &mut || order.borrow_mut().push("sp"), &mut || order.borrow_mut().push("base"));
        read_client_changed_property(true, false, false, &mut || order.borrow_mut().push("sp"), &mut || order.borrow_mut().push("base"));
        assert_eq!(order.borrow().as_slice(), ["base", "sp", "sp", "base", "base"]);
        // IDA 0x97ed00: per-pair apply.
        let mut stream = BitStream::new();
        stream.write_u16(3);
        let mut n = 0;
        deserialize_sf_flags(&mut stream, &mut || n += 1);
        assert_eq!(n, 3);
    }
    }

/// `ClientReplicator::ClientReplicator` C1 (IDA 0x97afc8) / C2 (IDA
/// 0x97b010): C1 delegates to C2; member and vtable init stays
/// engine-side.
pub fn init_client_replicator() {}

/// `ClientReplicator::OnReceive` (IDA 0x97be3c) and its non-virtual thunk
/// (IDA 0x97c3ec): packets not from the server, or not of type 142, go to
/// `Replicator::OnReceive`. A type-142 packet from the server is a
/// protocol mismatch: it logs, fires the connection-failed signal, and
/// requests a disconnect. Always returns 1.
pub fn client_replicator_on_receive(
 from_server: bool,
 kind: u8,
 base: &mut dyn FnMut() -> u32,
 mismatch: &mut dyn FnMut(),
) -> u32 {
 if from_server && kind == 142 {
 mismatch();
 1
 } else {
 base()
 }
}

/// `ClientReplicator::processPacket` (IDA 0x97c3fc): kind 129 carries the
/// streaming handshake (physics sender/receiver, GC job, guid
/// re-registration); everything else goes to `Replicator::processPacket`.
pub fn process_packet(
 is_streaming_config: bool,
 configure: &mut dyn FnMut(),
 base: &mut dyn FnMut(),
) {
 if is_streaming_config {
 configure();
 } else {
 base();
 }
}

/// `ClientReplicator::receiveCluster` (IDA 0x97ca44): streaming clients
/// count the terrain region, then `Replicator::receiveCluster` runs.
pub fn client_receive_cluster(streaming: bool, count: &mut dyn FnMut(), base: &mut dyn FnMut()) {
 if streaming {
 count();
 }
 base();
}

/// `ClientReplicator::readItem` (IDA 0x97cf08): item 13 is stream data,
/// everything else goes to `Replicator::readItem`.
pub fn read_client_item(kind: u8, stream_data: &mut dyn FnMut(), base: &mut dyn FnMut()) {
 if kind == 13 {
 stream_data();
 } else {
 base();
 }
}

/// `ClientReplicator::readStreamData` (IDA 0x97cf1c): two flag bits pick
/// the region step — 0 reads the region id off the wire, 1..3 advance
/// the cached coordinates — then `readJoinData` instances are counted.
/// Timing averages stay engine-side.
pub fn read_stream_data(
 step: u8,
 read_region: &mut dyn FnMut(),
 advance: &mut dyn FnMut(),
 joined: u32,
 add_joins: &mut dyn FnMut(u32),
) {
 if step == 0 {
 read_region();
 } else {
 advance();
 }
 add_joins(joined);
}

/// `ClientReplicator::writePropAcknowledgementIfNeeded` (IDA 0x97e7b4):
/// `PropSync::Slave::onPropertySend` decides; when it does not accept,
/// the item-type, value, descriptor, and id go out. The membership
/// assert stays engine-side.
pub fn write_prop_acknowledgement(
 sync_accepted: bool,
 write: &mut dyn FnMut(),
 serialize: &mut dyn FnMut(),
) {
 if !sync_accepted {
 write();
 serialize();
 }
}

/// `ClientReplicator::writeChangedProperty` (IDA 0x97e88c) and
/// `writeChangedRefProperty` (IDA 0x97e8b4): the acknowledgement first,
/// then the base write.
pub fn write_client_changed_property(ack: &mut dyn FnMut(), base: &mut dyn FnMut()) {
 ack();
 base();
}

/// `ClientReplicator::readChangedProperty` (IDA 0x97d444): the slave
/// sync observes the flag engine-side; streaming Parent changes notify
/// the GC job and stream out without the base read, streaming CFrame
/// changes queue interpolation ahead of the base read, and everything
/// else goes straight to the base read.
pub fn read_client_changed_property(
 streaming: bool,
 is_parent: bool,
 is_cframe: bool,
 special: &mut dyn FnMut(),
 base: &mut dyn FnMut(),
) {
 if streaming && is_parent {
 special();
 return;
 }
 if streaming && is_cframe {
 special();
 }
 base();
}

/// `ClientReplicator::deserializeSFFlags` (IDA 0x97ed00): a `u16` count
/// followed by that many server flag name/value pairs (RakString
/// framing stays engine-side), each applied via `SetValueFromServer`.
pub fn deserialize_sf_flags(stream: &mut BitStream, apply: &mut dyn FnMut()) {
 let count = stream.read_u16().unwrap_or(0);
 for _ in 0..count {
 apply();
 }
}
