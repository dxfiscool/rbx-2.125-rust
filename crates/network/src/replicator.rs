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
        let recv = |server: bool, kind: u8| {
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
    #[test]
    fn stats_and_request_items() {
        // IDA 0x984648: 25 children in order.
        let mut names = Vec::new();
        describe_rak_stats(&mut |n| names.push(n));
        assert_eq!(names.len(), 25);
        assert_eq!(names[0], "messageDataBytesSentPerSec");
        assert_eq!(names[24], "packetlossTotal");
        // IDA 0x987044: ack-gated CFrame write, always 1.
        let order = core::cell::RefCell::new(Vec::new());
        let acked = |accepted: bool| {
            write_prop_acknowledgement(
                accepted,
                &mut || order.borrow_mut().push("w"),
                &mut || order.borrow_mut().push("s"),
            );
            1
        };
        assert_eq!(acked(true), 1);
        assert_eq!(acked(false), 1);
        assert_eq!(order.borrow().as_slice(), ["w", "s"]);
        // IDA 0x987790: capacity triple, always 1.
        let mut stream = BitStream::new();
        assert_eq!(
            write_capacity_update(&mut stream, 32, 7, &mut |s| s.write_u8(9)),
            1
        );
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!((r.read_u8(), r.read_i32(), r.read_i16()), (Some(9), Some(32), Some(7)));
        // IDA 0x9877c8: local player required, always 1.
        assert_eq!(
            write_request_character(Some(3), &mut || order.borrow_mut().push("t"), &mut || order.borrow_mut().push("r")),
            1
        );
        assert_eq!(order.borrow().as_slice(), ["w", "s", "t", "r"]);
    }

    #[test]
    #[should_panic(expected = "without a local Player")]
    fn request_character_without_player_panics() {
        write_request_character(None, &mut || {}, &mut || {});
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

/// `RakStatsItem::RakStatsItem` (IDA 0x984648): the `Stats` item plus one
/// bound child per RakNet counter below (`messageSendBuffer` and
/// `bytesInSendBuffer` each own four priority sub-children engine-side).
/// Construction stays engine-side; this emits the child names in order.
pub fn describe_rak_stats(emit: &mut dyn FnMut(&'static str)) {
 for name in [
 "messageDataBytesSentPerSec",
 "messageTotalBytesSentPerSec",
 "messageDataBytesResentPerSec",
 "messagesBytesReceivedPerSec",
 "messagesBytesReceivedAndIgnoredPerSec",
 "bytesSentPerSec",
 "bytesReceivedPerSec",
 "totalMessageBytesPushed",
 "totalMessageBytesSent",
 "totalMessageBytesResent",
 "totalMessagesBytesReceived",
 "totalMessagesBytesReceivedAndIgnored",
 "totalBytesSent",
 "totalBytesReceived",
 "connectionStartTime",
 "outgoingBandwidthLimitBytesPerSecond",
 "isLimitedByOutgoingBandwidthLimit",
 "congestionControlLimitBytesPerSecond",
 "isLimitedByCongestionControl",
 "messageSendBuffer",
 "bytesInSendBuffer",
 "messagesInResendQueue",
 "bytesInResendQueue",
 "packetlossLastSecond",
 "packetlossTotal",
 ] {
 emit(name);
 }
}

/// `ClientCapacityUpdateItem::write` (IDA 0x987790): the item type, an
/// `int`, and a `short`. Always returns 1.
pub fn write_capacity_update(
 stream: &mut BitStream,
 capacity: i32,
 short: i16,
 write_type: &mut dyn FnMut(&mut BitStream),
) -> u32 {
 write_type(stream);
 stream.write_i32(capacity);
 stream.write_i16(short);
 1
}

/// `RequestCharacterItem::write` (IDA 0x9877c8): needs the local player
/// (else `std::runtime_error`), then the item type, hack flags, spawn
/// name, and player id go out engine-side. Always returns 1.
pub fn write_request_character(
 local_player: Option<u32>,
 write_type: &mut dyn FnMut(),
 write_rest: &mut dyn FnMut(),
) -> u32 {
 if local_player.is_none() {
 panic!("Attempting to send a Character request without a local Player");
 }
 write_type();
 write_rest();
 1
}

/// One reflected property under the write loops (IDA 0xadfcdc/0xae03cc):
/// the replicability bit at +28 (`& 4`), whether its type serializes on
/// the string path (`std::string`, `ProtectedString`, `SystemAddress`, or
/// a ref property), and whether it is the parent property.
#[derive(Clone, Copy, Debug, Default)]
pub struct PropertyWriteCandidate {
    /// Replicability flags at descriptor +28.
    pub flags: u8,
    /// String-ish or ref-property type (IDA 0xadfe36/0xae0534).
    pub is_wire_string_or_ref: bool,
    /// `RBX::Instance::propParent` (IDA 0xae056c).
    pub is_parent_prop: bool,
}

/// `Replicator::writeNonCacheableProperties` (IDA 0xadfcdc): per
/// descriptor, the replicated virtual (+284) must accept and bit 4 must be
/// set; string-ish/ref types go to `writePropertiesInternal`.
pub fn write_non_cacheable_properties(
    candidates: &[PropertyWriteCandidate],
    mut replicated: impl FnMut(usize) -> bool,
    mut write: impl FnMut(usize),
) {
    for (index, candidate) in candidates.iter().enumerate() {
        if candidate.flags & 4 == 0 {
            continue;
        }
        if !replicated(index) {
            continue;
        }
        if !candidate.is_wire_string_or_ref {
            continue;
        }
        write(index);
    }
}

/// `Replicator::writeCacheableProperties` (IDA 0xae03cc): the complement
/// loop — bit 4 set and replicated, but string-ish types, the parent
/// property, and ref properties are skipped.
pub fn write_cacheable_properties(
    candidates: &[PropertyWriteCandidate],
    mut replicated: impl FnMut(usize) -> bool,
    mut write: impl FnMut(usize),
) {
    for (index, candidate) in candidates.iter().enumerate() {
        if candidate.flags & 4 == 0 {
            continue;
        }
        if !replicated(index) {
            continue;
        }
        if candidate.is_wire_string_or_ref || candidate.is_parent_prop {
            continue;
        }
        write(index);
    }
}

/// `Replicator::writePropertiesInternal` (IDA 0xadfe8c): bools always take
/// the changed-property path (+312); otherwise a missing default also
/// takes it (after a 0 bit), an at-default value takes a 1 bit, and a
/// changed value takes a 0 bit plus the changed path. Verbose `StandardOut`
/// logging stays engine-side.
pub fn write_property_internal(
    is_bool: bool,
    has_default: bool,
    equals_default: bool,
    write_bit: &mut dyn FnMut(bool),
    write_changed: &mut dyn FnMut(),
) {
    if is_bool {
        // IDA 0xadffa6: bools skip the default comparison.
        write_changed();
        return;
    }
    if !has_default {
        // IDA 0xae0002: no default writes 0 plus the changed path.
        write_bit(false);
        write_changed();
    } else if equals_default {
        // IDA 0xadff46: at-default writes a 1 bit.
        write_bit(true);
    } else {
        write_bit(false);
        write_changed();
    }
}

/// `Replicator::getRakNetStats` (IDA 0xae10a8): null unless the flag at
/// +0x4b0 is set, when the stats at +0xd10 are returned. Passthrough gate.
#[must_use]
pub fn has_rak_net_stats(flag: u32) -> bool {
    flag != 0
}

#[cfg(test)]
mod property_writer_tests {
    use super::*;

    #[test]
    fn cacheable_splits_on_type() {
        // IDA 0xadfcdc/0xae03cc: string-ish goes non-cacheable, plain goes
        // cacheable, unset bit 4 goes nowhere.
        let candidates = [
            PropertyWriteCandidate { flags: 4, is_wire_string_or_ref: true, is_parent_prop: false },
            PropertyWriteCandidate { flags: 4, is_wire_string_or_ref: false, is_parent_prop: false },
            PropertyWriteCandidate { flags: 0, is_wire_string_or_ref: true, is_parent_prop: false },
            PropertyWriteCandidate { flags: 4, is_wire_string_or_ref: false, is_parent_prop: true },
        ];
        let mut non_cached = Vec::new();
        write_non_cacheable_properties(&candidates, |_| true, &mut |i| non_cached.push(i));
        assert_eq!(non_cached, vec![0]);
        let mut cached = Vec::new();
        write_cacheable_properties(&candidates, |_| true, &mut |i| cached.push(i));
        assert_eq!(cached, vec![1]);
        assert!(has_rak_net_stats(1));
        assert!(!has_rak_net_stats(0));
    }

    #[test]
    fn internal_default_bits() {
        // IDA 0xadfe8c: bool skips bits; at-default writes 1; changed writes 0 + path.
        let mut bits = Vec::new();
        let mut changed = 0;
        write_property_internal(true, true, true, &mut |b| bits.push(b), &mut || changed += 1);
        assert!((bits.is_empty(), changed) == (true, 1));
        write_property_internal(false, false, false, &mut |b| bits.push(b), &mut || changed += 1);
        write_property_internal(false, true, true, &mut |b| bits.push(b), &mut || changed += 1);
        write_property_internal(false, true, false, &mut |b| bits.push(b), &mut || changed += 1);
        assert_eq!(bits, vec![false, true, false]);
        assert_eq!(changed, 3);
    }
}

/// `Replicator::readItem` dispatch (IDA 0xaff534): 1 deletes an instance,
/// 2 reads a new instance, 3 reads a changed property, 4 reads a marker,
/// 5/6 read a data ping, 7 reads an event invocation, and 0xB reads join
/// data. The `FLog::NetworkReadItem` trace and the per-arm readers stay
/// engine-side; only the switch is modeled here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReplicatorItemTarget {
    InstanceDelete,
    InstanceNew,
    ChangedProperty,
    Marker,
    DataPing,
    EventInvocation,
    JoinData,
}

/// `Replicator::readItem` switch (IDA 0xaff534).
pub fn replicator_read_item_target(item_type: u32) -> ReplicatorItemTarget {
    match item_type {
        1 => ReplicatorItemTarget::InstanceDelete,
        2 => ReplicatorItemTarget::InstanceNew,
        3 => ReplicatorItemTarget::ChangedProperty,
        4 => ReplicatorItemTarget::Marker,
        // IDA 0xaff5d0: cases 5 and 6 share `readDataPing`.
        5 | 6 => ReplicatorItemTarget::DataPing,
        7 => ReplicatorItemTarget::EventInvocation,
        0xB => ReplicatorItemTarget::JoinData,
        // IDA 0xaff65e..0xaff774: `ReleaseAssert(false @ Replicator.cpp:2769)`
        // then `throw std::runtime_error("")`.
        _ => panic!("Replicator::readItem: bad item type"),
    }
}

/// One `Replicator::readDataPing` packet (IDA 0xb00e44): the respond flag,
/// the ping timestamp, and the ping id. The virtual at +308 and the
/// running-average pool stay engine-side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataPing {
    pub respond: bool,
    pub sent_ms: u64,
    pub ping_id: u32,
}

/// `Replicator::readDataPing` wire reads (IDA 0xb00e68..0xb00e78):
/// `operator>><bool>`, `operator>><unsigned long long>`,
/// `operator>><unsigned int>`.
pub fn read_data_ping(stream: &mut BitStream) -> DataPing {
    let respond = stream.read_bool().expect("BitStream >> bool failed");
    let sent_ms = stream.read_u64().expect("BitStream >> unsigned long long failed");
    let ping_id = stream.read_u32().expect("BitStream >> unsigned int failed");
    DataPing { respond, sent_ms, ping_id }
}

/// `readDataPing` follow-up branch (IDA 0xb00eb6..0xb00f78).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataPingAction {
    /// Flag set: `RunningAverage::sample(now - sent)` (IDA 0xb00eca).
    SampleRtt { rtt_ms: u32 },
    /// Flag clear: pool-allocate a `PingBackItem(this, sent)` and push it
    /// to the `ItemQueue` at +1592 (IDA 0xb00f3c..0xb00f78), engine-side.
    QueuePingBack,
}

/// `readDataPing` follow-up (IDA 0xb00eb6..0xb00f78).
pub fn data_ping_action(ping: &DataPing, now_ms: u32) -> DataPingAction {
    if ping.respond {
        DataPingAction::SampleRtt { rtt_ms: now_ms.wrapping_sub(ping.sent_ms as u32) }
    } else {
        DataPingAction::QueuePingBack
    }
}

/// `readDataPing` stamp write (IDA 0xb00f7c..0xb00f8c): `this[889]` takes
/// the current time and `this[890]` is cleared. The `ReplicatorStats`
/// increment behind the +929 flag stays engine-side.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PingStamp {
    pub last_ms: u32,
    pub pending: u32,
}

/// `readDataPing` stamp write (IDA 0xb00f7c..0xb00f8c).
pub fn stamp_data_ping(stamp: &mut PingStamp, now_ms: u32) {
    stamp.last_ms = now_ms;
    stamp.pending = 0;
}

/// `Replicator::readJoinData` instance count (IDA 0xb01e04). The
/// `Compressor::readCompressed` refill and the `readInstanceNew` loop
/// stay engine-side; only the count decode is modeled here.
pub fn join_data_instance_count(stream: &mut BitStream, packed: bool) -> u32 {
    if packed {
        // IDA 0xb01e48..0xb01ea8: the +3720 flag selects the packed form —
        // 5-bit groups, low 4 bits of payload, bit 4 continues.
        let mut count = 0u32;
        let mut shift = 0u32;
        loop {
            let bits = stream.read_bits(5).expect("BitStream ReadBits(5) failed");
            count |= (bits & 0xF) << shift;
            shift += 4;
            if bits & 0x10 == 0 {
                break;
            }
        }
        count
    } else {
        // IDA 0xb01f4c: plain `operator>><unsigned int>`.
        stream.read_u32().expect("BitStream >> unsigned int failed")
    }
}

/// `Replicator::OnInternalPacket` counters (IDA 0xb04828): `this[689]`
/// counts whole packets, `this[690]` counts split packets.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SplitCounts {
    pub whole: u32,
    pub split: u32,
}

/// `Replicator::OnInternalPacket` (IDA 0xb04876..0xb0498a): nothing
/// unless the per-packet count flag is set; a nonzero split count bumps
/// the split counter (logging `"split message, id %u, size %d, split
/// count %d"` first when the log flag is set and the fragment id field
/// is 0), otherwise the whole counter bumps. The `shared_ptr` release
/// in the log arm is `Arc` bookkeeping here.
pub fn on_internal_packet(
    counts: &mut SplitCounts,
    count: bool,
    split_count: u32,
    log_split: bool,
    first_fragment: bool,
    msg_id: u8,
    size_bytes: u32,
    log: &mut dyn FnMut(u8, u32, u32),
) {
    if !count {
        return;
    }
    if split_count != 0 {
        if log_split && first_fragment {
            log(msg_id, size_bytes, split_count);
        }
        counts.split += 1;
    } else {
        counts.whole += 1;
    }
}

/// `Replicator::pushIncomingPacket` queue item (IDA 0xae1f8c): the
/// `timestamped_safe_queue_item<RakNet::Packet *>` timestamp plus the
/// packet. Packets stay engine-side as opaque tokens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimestampedPacket {
    pub time: f64,
    pub packet: u32,
}

/// `Replicator::pushIncomingPacket` (IDA 0xae1fba..0xae2118): timestamp
/// under the +3628 mutex (locking stays engine-side) and push to the
/// deque at +3588 (`_M_reallocate_map` growth is `VecDeque` growth
/// here); matching schedulers reschedule the job at +1568.
pub fn push_incoming_packet(
    queue: &mut std::collections::VecDeque<TimestampedPacket>,
    time: f64,
    packet: u32,
    scheduler: u32,
    rescheduler: u32,
    reschedule: &mut dyn FnMut(),
) {
    queue.push_back(TimestampedPacket { time, packet });
    if scheduler == rescheduler {
        reschedule();
    }
}

/// Base `Replicator::serializeSFFlags` / `deserializeSFFlags` (IDA
/// 0xb0ceb4/0xb0ceb8): both empty — the `ServerReplicator` override
/// ([`serialize_sf_flags`], IDA 0x9e2024) does the real work.
pub fn replicator_sf_flags_noop() {}

/// `Replicator::SendClusterJob` (IDA 0xb0db10/0xb0dbdc): the vtable
/// reset, the shared Replicator ref at +484, and the base `Job`
/// teardown stay engine-side; Rust drops the shared owner. D0
/// additionally frees the allocation (IDA 0xb0dc50), which a Rust drop
/// does.
#[derive(Clone, Debug)]
pub struct SendClusterJob {
    /// Shared Replicator owner at +484 (`boost::detail::shared_count`).
    pub owner: rbx_core::SharedPtr<()>,
}

/// `Replicator::SendClusterJob::sleepTime` (IDA 0xb0dcbc): forwards the
/// stats float at +488 (packed into the sleep double) to
/// `computeStandardSleepTime`.
pub fn send_cluster_job_sleep_time(
    elapsed: f64,
    rate_hz: f32,
    ctx: &crate::physics::SleepContext,
) -> f64 {
    crate::physics::standard_sleep_time(elapsed, rate_hz, ctx)
}

/// `RBX::Network::ReplicatorJob` (IDA 0xb0dfd8): same teardown shape as
/// `SendClusterJob` (vtable `off_12BCCD8`, shared ref at +484, base
/// `Job` with -1).
#[derive(Clone, Debug)]
pub struct ReplicatorJob {
    /// Shared Replicator owner at +484 (`boost::detail::shared_count`).
    pub owner: rbx_core::SharedPtr<()>,
}

/// `RBX::Network::Marker` (IDA 0xb139fc): engine-side marker node; only
/// the shared-ownership shape crosses into the deque seams below.
#[derive(Clone, Debug, Default)]
pub struct Marker;

/// `std::deque<shared_ptr<Marker>>::_M_push_back_aux` (IDA 0xb139fc):
/// refcounted push; the spinlock-pool refcounting is `Arc`
/// bookkeeping here.
pub fn marker_queue_push(
    queue: &mut std::collections::VecDeque<rbx_core::SharedPtr<Marker>>,
    marker: rbx_core::SharedPtr<Marker>,
) {
    queue.push_back(marker);
}

/// `std::deque<shared_ptr<Marker>>::_M_reallocate_map` (IDA 0xb13d44):
/// recenter-or-grow the chunk map; `reserve` keeps the growth edge
/// (recentering is a `VecDeque` no-op).
pub fn marker_queue_reserve(
    queue: &mut std::collections::VecDeque<rbx_core::SharedPtr<Marker>>,
    extra: usize,
) {
    queue.reserve(extra);
}

#[cfg(test)]
mod next107_tests {
    use super::*;

    #[test]
    fn read_item_switch() {
        assert_eq!(replicator_read_item_target(1), ReplicatorItemTarget::InstanceDelete);
        assert_eq!(replicator_read_item_target(2), ReplicatorItemTarget::InstanceNew);
        assert_eq!(replicator_read_item_target(3), ReplicatorItemTarget::ChangedProperty);
        assert_eq!(replicator_read_item_target(4), ReplicatorItemTarget::Marker);
        assert_eq!(replicator_read_item_target(5), ReplicatorItemTarget::DataPing);
        assert_eq!(replicator_read_item_target(6), ReplicatorItemTarget::DataPing);
        assert_eq!(replicator_read_item_target(7), ReplicatorItemTarget::EventInvocation);
        assert_eq!(replicator_read_item_target(0xB), ReplicatorItemTarget::JoinData);
    }

    #[test]
    #[should_panic(expected = "bad item type")]
    fn read_item_default_throws() {
        let _ = replicator_read_item_target(0);
    }

    #[test]
    fn data_ping_roundtrip() {
        let mut stream = BitStream::new();
        stream.write_bool(true);
        stream.write_u64(1_000);
        stream.write_u32(9);
        let mut read = BitStream::from_bytes(&stream.into_bytes());
        let ping = read_data_ping(&mut read);
        assert_eq!(ping, DataPing { respond: true, sent_ms: 1_000, ping_id: 9 });
        assert_eq!(data_ping_action(&ping, 1_060), DataPingAction::SampleRtt { rtt_ms: 60 });
        let queued = DataPing { respond: false, sent_ms: 1_000, ping_id: 9 };
        assert_eq!(data_ping_action(&queued, 1_060), DataPingAction::QueuePingBack);
        let mut stamp = PingStamp::default();
        stamp_data_ping(&mut stamp, 1_060);
        assert_eq!(stamp, PingStamp { last_ms: 1_060, pending: 0 });
    }

    #[test]
    fn join_data_counts() {
        // IDA 0xb01e48: 0x1A3 arrives as nibbles 3, A, 1 with continue bits.
        let mut packed = BitStream::new();
        packed.write_bits(0x13, 5);
        packed.write_bits(0x1A, 5);
        packed.write_bits(0x01, 5);
        let mut read = BitStream::from_bytes(&packed.into_bytes());
        assert_eq!(join_data_instance_count(&mut read, true), 0x1A3);
        let mut plain = BitStream::new();
        plain.write_u32(7);
        let mut read = BitStream::from_bytes(&plain.into_bytes());
        assert_eq!(join_data_instance_count(&mut read, false), 7);
    }

    #[test]
    fn internal_packet_counters() {
        let mut counts = SplitCounts::default();
        let mut logged = Vec::new();
        let mut log = |id: u8, size: u32, splits: u32| logged.push((id, size, splits));
        on_internal_packet(&mut counts, false, 3, true, true, 0x81, 100, &mut log);
        assert_eq!((counts.whole, counts.split), (0, 0));
        on_internal_packet(&mut counts, true, 0, true, true, 0x81, 100, &mut log);
        assert_eq!((counts.whole, counts.split), (1, 0));
        on_internal_packet(&mut counts, true, 3, true, true, 0x81, 100, &mut log);
        on_internal_packet(&mut counts, true, 3, true, false, 0x81, 100, &mut log);
        assert_eq!((counts.whole, counts.split), (1, 2));
        drop(log);
        assert_eq!(logged, vec![(0x81, 100, 3)]);
    }

    #[test]
    fn incoming_packet_reschedules_on_match() {
        let mut queue = std::collections::VecDeque::new();
        let mut rescheduled = 0;
        push_incoming_packet(&mut queue, 1.5, 42, 7, 7, &mut || rescheduled += 1);
        push_incoming_packet(&mut queue, 2.5, 43, 7, 8, &mut || rescheduled += 1);
        assert_eq!(queue.len(), 2);
        assert_eq!(queue[0], TimestampedPacket { time: 1.5, packet: 42 });
        assert_eq!(rescheduled, 1);
    }

    #[test]
    fn cluster_job_sleep_forwards() {
        let ctx = crate::physics::SleepContext::default();
        assert_eq!(send_cluster_job_sleep_time(0.0, 20.0, &ctx), 0.05);
        replicator_sf_flags_noop();
    }

    #[test]
    fn marker_queue_edges() {
        let mut queue = std::collections::VecDeque::new();
        marker_queue_reserve(&mut queue, 4);
        marker_queue_push(&mut queue, rbx_core::SharedPtr::from(Box::new(Marker)));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn replicator_init_defaults() {
        // IDA 0xae10b8: prime-11 buckets, 23-deep queue, 128-id windows.
        let init = replicator_init(true);
        assert_eq!(init.descriptor_buckets, 11);
        assert_eq!(init.incoming_capacity, 23);
        assert_eq!(init.rolling_id_len, 128);
        assert!(init.stats_hook);
        assert_eq!(
            replicator_created_log("127.0.0.1:53640"),
            "Replicator created for player 127.0.0.1:53640"
        );
        assert_eq!(CHAT_SEND_PRIORITY, 1);
        assert_eq!((CHAT_SEND_RELIABILITY, CHAT_SEND_ORDERING_CHANNEL), (2, 2));
    }

    #[test]
    fn chat_filter_decisions() {
        // IDA 0xaec7fc: self-address drops.
        assert_eq!(send_filtered_chat_decision(true, true, 1, false, false), None);
        // IDA 0xaec858: no filter and empty text drops.
        assert_eq!(send_filtered_chat_decision(false, false, 0, true, true), None);
        assert_eq!(send_filtered_chat_decision(false, true, 0, true, true), None);
        // IDA 0xaecb32: mapped player with filter 1 takes the replacement.
        assert_eq!(
            send_filtered_chat_decision(false, true, 1, false, false),
            Some(ChatFilterWrite::Filtered)
        );
        // IDA 0xaec904/0xaec912: filter 0 or unmapped resends the original.
        assert_eq!(
            send_filtered_chat_decision(false, true, 0, true, false),
            Some(ChatFilterWrite::Original)
        );
        assert_eq!(
            send_filtered_chat_decision(false, true, 1, true, false),
            Some(ChatFilterWrite::Original)
        );
        assert_eq!(
            send_filtered_chat_decision(false, false, 0, true, false),
            Some(ChatFilterWrite::Original)
        );
    }

    #[test]
    fn changed_property_arms() {
        // IDA 0xb00366: 2-bit index 2 of 3 descriptors.
        let mut stream = BitStream::new();
        stream.write_bits(2, 2);
        let mut read = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(changed_property_index(&mut read, 2, 3), 2);
        // IDA 0xb0040a: cleared when filtered or illegal.
        assert!(changed_property_keep(true, false, true));
        assert!(!changed_property_keep(true, true, true));
        assert!(!changed_property_keep(true, false, false));
        assert!(!changed_property_keep(false, false, true));
        assert_eq!(
            replication_log_line(Some("Baseplate"), "abc", "Position", "127.0.0.1"),
            "Replication: Baseplate-abc.Position << 127.0.0.1"
        );
        assert_eq!(
            replication_log_line(None, "abc", "Position", "127.0.0.1"),
            "Replication: ?-abc.Position << 127.0.0.1"
        );
    }

    #[test]
    fn marker_arms() {
        // IDA 0xb009f2: long id round-trips.
        let mut stream = BitStream::new();
        stream.write_i32(77);
        let mut read = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(read_marker_id(&mut read), 77);
        // IDA 0xb00a72/0xb00b5c: log lines.
        assert_eq!(marker_log_line(77, "10.0.0.2"), "Received marker 77 from 10.0.0.2");
        assert_eq!(marker_fast_log(77), "Replicator:ReadMarker id(77)");
        // IDA 0xb00bf6: front must match before the pop.
        assert!(marker_front_matches(Some(77), 77));
        assert!(!marker_front_matches(Some(78), 77));
        assert!(!marker_front_matches(None, 77));
        // IDA 0xb00caa: terrain tail gated on the chunk-defer flag.
        assert!(should_done_loading_terrain(false));
        assert!(!should_done_loading_terrain(true));
    }

    #[test]
    fn event_arms() {
        let mut stream = BitStream::new();
        stream.write_bits(1, 1);
        let mut read = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(read_event_index(&mut read, 1, 2), 1);
        // IDA 0xb0127c: +208 == 1 keeps the instance.
        assert!(event_keep(true));
        assert!(!event_keep(false));
    }

    #[test]
    fn packet_switches() {
        // IDA 0xb02984: first-byte dispatch with the physics gate.
        assert_eq!(process_packet_kind(27, true), ProcessPacketKind::Physics);
        assert_eq!(process_packet_kind(27, false), ProcessPacketKind::Unknown);
        assert_eq!(process_packet_kind(130, false), ProcessPacketKind::Schema);
        assert_eq!(process_packet_kind(131, false), ProcessPacketKind::Data);
        assert_eq!(process_packet_kind(134, true), ProcessPacketKind::Touches);
        assert_eq!(process_packet_kind(134, false), ProcessPacketKind::Unknown);
        assert_eq!(process_packet_kind(141, false), ProcessPacketKind::Instance96);
        assert_eq!(process_packet_kind(0, true), ProcessPacketKind::Unknown);
        // IDA 0xb02a4e: inner id must be ID_PHYSICS (133).
        assert!(physics_inner_valid(133));
        assert!(!physics_inner_valid(27));
        // IDA 0xb02e30: OnReceive dispatch.
        assert_eq!(on_receive_action(0x81), OnReceiveAction::PushIncoming);
        assert_eq!(on_receive_action(0x8D), OnReceiveAction::PushIncoming);
        assert_eq!(on_receive_action(0x84), OnReceiveAction::MarkerItem);
        assert_eq!(on_receive_action(0x88), OnReceiveAction::Chat);
        assert_eq!(on_receive_action(0x89), OnReceiveAction::ReportAbuse);
        assert_eq!(on_receive_action(0x10), OnReceiveAction::SchemaTeach);
        assert_eq!(on_receive_action(0x15), OnReceiveAction::Disconnect);
        assert_eq!(on_receive_action(0x16), OnReceiveAction::ConnectionLost);
        assert_eq!(on_receive_action(0x1B), OnReceiveAction::PhysicsPush);
        assert_eq!(on_receive_action(0xFF), OnReceiveAction::Ignore);
        // IDA 0xb02e30: verdicts — marker drops, chat forwards, rest handled.
        assert_eq!(on_receive_verdict(OnReceiveAction::MarkerItem, None), 0);
        assert_eq!(on_receive_verdict(OnReceiveAction::Chat, Some(true)), 2);
        assert_eq!(on_receive_verdict(OnReceiveAction::Chat, None), 0);
        assert_eq!(on_receive_verdict(OnReceiveAction::ReportAbuse, Some(false)), 0);
        assert_eq!(on_receive_verdict(OnReceiveAction::PushIncoming, None), 1);
        assert_eq!(on_receive_verdict(OnReceiveAction::Ignore, None), 1);
        assert_eq!(on_receive_log("10.0.0.2", false), "Disconnect from 10.0.0.2");
        assert_eq!(on_receive_log("10.0.0.2", true), "Lost connection to 10.0.0.2");
    }

    #[test]
    fn chat_bind_roundtrip() {
        // IDA 0xb07980/0xb14fe0: retain-then-forward bind of the filter.
        let target = rbx_core::SharedPtr::from(Box::new(Marker));
        let retained = bind_list1_replicator(&target);
        assert!(rbx_core::SharedPtr::ptr_eq(&target, &retained));
        let call = bind_chat_filter(target);
        let mut seen = Vec::new();
        let addr = crate::socket::SystemAddress::new();
        call_chat_filter(&call, &addr, "hi", "hi*", &mut |_, _, text, filtered| {
            seen.push((text.to_owned(), filtered.to_owned()));
        });
        assert_eq!(seen, vec![("hi".to_owned(), "hi*".to_owned())]);
    }

    #[test]
    fn voxel_encode_shapes() {
        // IDA 0xb15f50: widths, region keys, and budget breaks.
        assert_eq!(VOXEL_CELL_BIT_WIDTHS, (5, 4, 5));
        assert_eq!(VOXEL_REGION_BIT_WIDTHS, (4, 2, 4));
        assert_eq!(VOXEL_END_MARKER_BIT_WIDTH, 2);
        assert_eq!(voxel_region_of((63, 31, 63)), (1, 1, 1));
        assert_eq!(voxel_region_of((32, 16, 32)), (1, 1, 1));
        assert_eq!(voxel_region_of((0, 0, 0)), (0, 0, 0));
        assert!(voxel_budget_hit(64, 64));
        assert!(!voxel_budget_hit(63, 64));
        assert!(!voxel_budget_hit(usize::MAX, -1));
    }

    #[test]
    fn replication_bind_chain() {
        // IDA 0xb1c5cc/0xb1c790/0xb1c954/0xb1cb18: 4 → 4 → 3 → 2 captures.
        let bind = replication_bind4(true, true);
        assert_eq!(replication_store4(&bind), bind);
        let three = replication_store3(&bind);
        assert_eq!((three.target_alive, three.has_data), (true, true));
        let two = replication_store2(&bind);
        assert_eq!((two.target_alive, two.has_data), (true, true));
        let dead = replication_bind4(false, false);
        assert_eq!(replication_store2(&dead), ReplicationBind2::default());
    }

    #[test]
    fn voxel_encode_cells_bits() {
        // IDA 0xb15f50: two same-region cells = 26 + 15 bits, tail = 4.
        let mut stream = BitStream::new();
        voxel_encode_cells(&mut stream, &[(1, 2, 3), (4, 5, 6)], -1, 2);
        assert_eq!(stream.bits_written(), 45);
        // IDA 0xb16018: a zero budget stops before the first cell.
        let mut starved = BitStream::new();
        voxel_encode_cells(&mut starved, &[(1, 2, 3)], 0, 2);
        assert_eq!(starved.bits_written(), 4);
    }
    #[test]
    fn shared_dict_ownership() {
        // IDA 0xb2058c/0xb20850/0xb20854/0xb20860: construct then drop.
        let dict = protected_string_dict();
        assert!(dict.entries.is_empty());
        shared_dict_drop(dict);
        // IDA 0xb209b0/0xb209b4/0xb20e64/0xb20e68: null deleters.
        assert!(shared_null_deleter().is_null());
        let shared = crate::string_dictionary::SharedStringDictionary::new();
        let owned: rbx_core::SharedPtr<crate::string_dictionary::SharedStringDictionary> =
            rbx_core::SharedPtr::from(Box::new(shared));
        shared_dict_drop(owned);
    }

    #[test]
    fn physics_sender_ownership() {
        // IDA 0xb20e6c/0xb21030/0xb211f4/0xb213b8 + D1/D0/dispose triplets:
        // construct then drop each sender variant.
        shared_dict_drop(crate::physics::top_n_errors_physics_sender());
        shared_dict_drop(crate::physics::round_robin_physics_sender());
        shared_dict_drop(crate::physics::error_comp_physics_sender2());
        shared_dict_drop(crate::physics::error_comp_physics_sender());
        // IDA 0xb23cd8: PingJob construct then drop.
        shared_dict_drop(ping_job());
        assert!(shared_null_deleter().is_null());
    }

    #[test]
    fn chat_slot_lifecycle() {
        // IDA 0xb21844/0xb218f8: assign swaps the retained slot.
        let first: rbx_core::SharedPtr<ChatSlot> =
            rbx_core::SharedPtr::from(Box::new(ChatSlot::default()));
        let second: rbx_core::SharedPtr<ChatSlot> =
            rbx_core::SharedPtr::from(Box::new(ChatSlot { signal_linked: true }));
        let mut slot = rbx_core::SharedPtr::clone(&first);
        chat_slot_assign(&mut slot, &second);
        assert!(rbx_core::SharedPtr::ptr_eq(&slot, &second));
        // IDA 0xb21bec: connected reads the +12 signal link.
        assert!(!chat_slot_connected(&first));
        assert!(chat_slot_connected(&second));
        // IDA 0xb21c58: remove splices the slot out exactly once.
        let mut slots = vec![rbx_core::SharedPtr::clone(&first), rbx_core::SharedPtr::clone(&second)];
        assert!(chat_signal_remove(&mut slots, &second));
        assert!(!chat_signal_remove(&mut slots, &second));
        assert_eq!(slots.len(), 1);
        // IDA 0xb21d44/0xb21e28/0xb21fa4/0xb21fb0/0xb22064/0xb220c0: no-op init + drops.
        chat_slot_mutex_init();
        let target = rbx_core::SharedPtr::from(Box::new(Marker));
        chat_callable_drop(bind_chat_filter(target));
        chat_slot_drop(first);
    }

    #[test]
    fn chat_bind_tail_chain() {
        // IDA 0xb221c8/0xb22618/0xb22a68: list5 → storage4 → storage2 retains.
        let target = rbx_core::SharedPtr::from(Box::new(Marker));
        let call = chat_list5(&target);
        assert!(rbx_core::SharedPtr::ptr_eq(&call.target, &target));
        let narrowed = chat_store4(&call);
        let narrowest = chat_store2(&narrowed);
        assert!(rbx_core::SharedPtr::ptr_eq(&narrowest.target, &target));
        // IDA 0xb21bf8: the callable forwards the four args like the bind call.
        let mut seen = Vec::new();
        let addr = crate::socket::SystemAddress::new();
        chat_callable_call(&narrowest, &addr, "hi", "hi*", &mut |_, _, text, filtered| {
            seen.push((text.to_owned(), filtered.to_owned()));
        });
        assert_eq!(seen, vec![("hi".to_owned(), "hi*".to_owned())]);
        // IDA 0xb2332c: the declare guard runs exactly once.
        let mut declared = false;
        assert!(declare_cluster_packet_cache(&mut declared));
        assert!(!declare_cluster_packet_cache(&mut declared));
    }
}

/// `Replicator::Replicator` init constants (IDA 0xae10b8): the property
/// `unordered_map` at +1436 picks the first prime above 10 (11, IDA
/// 0xae13d4..0xae1418), the incoming `safe_queue` at +3588 is bounded at
/// 23 (IDA 0xae15e8), and the two rolling id windows (+3844..+4868 and
/// +4872..+5896, 8-byte entries) each hold 128 ids pre-filled with -1
/// (IDA 0xae16c4..0xae1706). Descriptor senders, signal slots, and stats
/// start empty; the mutex/pool wiring stays engine-side.
pub const REPLICATOR_DESCRIPTOR_BUCKETS: u32 = 11;
pub const REPLICATOR_INCOMING_CAPACITY: usize = 23;
pub const REPLICATOR_ROLLING_ID_LEN: usize = 128;

/// `Replicator::Replicator` modeled init state (IDA 0xae10b8).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReplicatorInit {
    pub descriptor_buckets: u32,
    pub incoming_capacity: usize,
    pub rolling_id_len: usize,
    pub stats_hook: bool,
}

/// `Replicator::Replicator` defaults (IDA 0xae10b8): bucket/queue/window
/// shapes above plus the +1512 stats flag (`a9`).
#[must_use]
pub fn replicator_init(stats_hook: bool) -> ReplicatorInit {
    ReplicatorInit {
        descriptor_buckets: REPLICATOR_DESCRIPTOR_BUCKETS,
        incoming_capacity: REPLICATOR_INCOMING_CAPACITY,
        rolling_id_len: REPLICATOR_ROLLING_ID_LEN,
        stats_hook,
    }
}

/// `Replicator::Replicator` creation log (IDA 0xae17da): `"Replicator
/// created for player %s"`.
#[must_use]
pub fn replicator_created_log(address: &str) -> String {
    format!("Replicator created for player {address}")
}

/// `ConcurrentRakPeer::Send` priority/reliability/ordering used by
/// `sendFilteredChatMessage` (IDA 0xaec99e: `Send(..., 1, 2, 2, ...)`).
pub const CHAT_SEND_PRIORITY: u8 = 1;
pub const CHAT_SEND_RELIABILITY: u8 = 2;
pub const CHAT_SEND_ORDERING_CHANNEL: u8 = 2;

/// `sendFilteredChatMessage` payload choice (IDA 0xaec8ae..0xaec912):
/// the original string or the filtered replacement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChatFilterWrite {
    Original,
    Filtered,
}

/// `Replicator::sendFilteredChatMessage` decision (IDA 0xaec7fc..0xaec912):
/// `None` drops the message — the destination is self, or there is no
/// player (or no chat filter) and the text is empty. A mapped player
/// with filter type 1 takes the filtered replacement; everything else
/// re-sends the original. The bitstream copy and `Send` stay
/// engine-side.
#[must_use]
pub fn send_filtered_chat_decision(
    dest_is_self: bool,
    has_player: bool,
    filter_type: u32,
    mapping_empty: bool,
    text_empty: bool,
) -> Option<ChatFilterWrite> {
    if dest_is_self {
        // IDA 0xaec7fc: `operator==` on the +1208 address.
        return None;
    }
    if (!has_player || filter_type == 0) && text_empty {
        // IDA 0xaec858: no filter and an empty string sends nothing.
        return None;
    }
    if has_player && !mapping_empty && filter_type == 1 {
        // IDA 0xaecb32: filter type 1 writes the replacement (`a5`).
        Some(ChatFilterWrite::Filtered)
    } else {
        Some(ChatFilterWrite::Original)
    }
}

/// `readChangedProperty` descriptor index (IDA 0xb00366..0xb0038e):
/// `ReadBits` with the width at +336, range-checked like
/// `read_prop_acknowledgement` (`vector::_M_range_check`, IDA 0xb0074c).
/// The `propertyDescriptor != NULL` assert (Replicator.cpp:2861) and the
/// `deserializeInstanceRef` stay engine-side.
pub fn changed_property_index(
    stream: &mut BitStream,
    descriptor_bits: u8,
    descriptor_count: usize,
) -> usize {
    let index = stream.read_bits(descriptor_bits).expect("BitStream ReadBits failed") as usize;
    if index >= descriptor_count {
        panic!("vector::_M_range_check");
    }
    index
}

/// `readChangedProperty` instance gate (IDA 0xb0040a): the instance is
/// cleared when the +244 hook fires or the +212 legality hook fails, so
/// only an unfiltered, legal instance proceeds to `deserializeProperty`.
#[must_use]
pub fn changed_property_keep(instance_present: bool, filtered: bool, legal_prop: bool) -> bool {
    instance_present && !filtered && legal_prop
}

/// `readChangedProperty` / `readEventInvocation` trace line (IDA
/// 0xb00478/0xb0116a): `"Replication: %s-%s.%s << %s"`.
#[must_use]
pub fn replication_log_line(
    instance_name: Option<&str>,
    guid: &str,
    member: &str,
    addr: &str,
) -> String {
    format!("Replication: {}-{}.{} << {}", instance_name.unwrap_or("?"), guid, member, addr)
}

/// `readMarker` id read (IDA 0xb009f2): `operator>><long>`.
pub fn read_marker_id(stream: &mut BitStream) -> i32 {
    stream.read_i32().expect("BitStream >> long failed")
}

/// `readMarker` verbose line (IDA 0xb00a72): `"Received marker %d from
/// %s"`.
#[must_use]
pub fn marker_log_line(id: i32, addr: &str) -> String {
    format!("Received marker {id} from {addr}")
}

/// `readMarker` fast log (IDA 0xb00b5c): `"Replicator:ReadMarker id(%d)"`.
#[must_use]
pub fn marker_fast_log(id: i32) -> String {
    format!("Replicator:ReadMarker id({id})")
}

/// `readMarker` front assert (IDA 0xb00bf6..0xb00c40,
/// Replicator.cpp:2830): the deque must be non-empty and its front id
/// must equal the wire id before `Marker::fireReturned` pops it.
#[must_use]
pub fn marker_front_matches(front: Option<i32>, id: i32) -> bool {
    front == Some(id)
}

/// `readMarker` terrain tail (IDA 0xb00caa..0xb00cc2): `doneLoadingTerrain`
/// runs unless `FFlag::ChunkAndDeferVoxelUpdates` is set.
#[must_use]
pub fn should_done_loading_terrain(chunk_defer_voxel_updates: bool) -> bool {
    !chunk_defer_voxel_updates
}

/// `readEventInvocation` event index (IDA 0xb010b4..0xb01100): `ReadBits`
/// with the width at +347, range-checked (`vector::_M_range_check`, IDA
/// 0xb01720).
pub fn read_event_index(
    stream: &mut BitStream,
    event_bits: u8,
    event_count: usize,
) -> usize {
    let index = stream.read_bits(event_bits).expect("BitStream ReadBits failed") as usize;
    if index >= event_count {
        panic!("vector::_M_range_check");
    }
    index
}

/// `readEventInvocation` instance gate (IDA 0xb0127c): the +208 hook
/// returning 1 keeps the instance, anything else clears it before
/// `deserializeEventInvocation`.
#[must_use]
pub fn event_keep(legal_receive: bool) -> bool {
    legal_receive
}

/// `Replicator::processPacket` dispatch (IDA 0xb02984..0xb02d6a): the
/// first byte selects the reader; the stack `BitStream` wraps the raw
/// packet (IDA 0xb029c0). Physics arms additionally require the
/// `PhysicsReceiver` at +3732.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessPacketKind {
    Physics,
    Schema,
    Data,
    Touches,
    Instance96,
    Unknown,
}

/// `processPacket` first-byte switch (IDA 0xb029d8..0xb02c58): 27 is the
/// physics packet, 130 the descriptor schema, 131 the item stream, 134
/// the touches, 141 the instance-96 ping; anything else is ignored.
#[must_use]
pub fn process_packet_kind(first_byte: u8, has_physics: bool) -> ProcessPacketKind {
    match first_byte {
        27 if has_physics => ProcessPacketKind::Physics,
        130 => ProcessPacketKind::Schema,
        131 => ProcessPacketKind::Data,
        134 if has_physics => ProcessPacketKind::Touches,
        141 => ProcessPacketKind::Instance96,
        _ => ProcessPacketKind::Unknown,
    }
}

/// `processPacket` physics inner id (IDA 0xb02a4e): the `u64`/`u8` header
/// must carry `ID_PHYSICS` (133) or `ReleaseAssert` fires
/// (Replicator.cpp:3123).
pub const PHYSICS_INNER_ID: u8 = 133;

/// `processPacket` inner-id assert (IDA 0xb02a44..0xb02a50).
#[must_use]
pub fn physics_inner_valid(marker: u8) -> bool {
    marker == PHYSICS_INNER_ID
}

/// `Replicator::OnReceive` dispatch (IDA 0xb02e30..0xb047c6): packets
/// from any other address than +1208 return 1 immediately (IDA
/// 0xb02ebc); otherwise the first byte selects the arm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnReceiveAction {
    PushIncoming,
    MarkerItem,
    Chat,
    ReportAbuse,
    SchemaTeach,
    Disconnect,
    ConnectionLost,
    PhysicsPush,
    Ignore,
}

/// `OnReceive` first-byte switch (IDA 0xb02ec8..0xb03992): 0x81/82/83/86/8D
/// queue via `pushIncomingPacket`, 0x84 appends a `MarkerItem`, 0x87/88/8B/8C
/// go to `Players::OnReceiveChat`, 0x89 to `OnReceiveReportAbuse`,
/// 0x10/0x13 teach the descriptor schema, 0x15/0x16 are disconnect/lost,
/// 0x1B pushes a physics packet; anything else returns 1.
#[must_use]
pub fn on_receive_action(first_byte: u8) -> OnReceiveAction {
    match first_byte {
        0x81 | 0x82 | 0x83 | 0x86 | 0x8D => OnReceiveAction::PushIncoming,
        0x84 => OnReceiveAction::MarkerItem,
        0x87 | 0x88 | 0x8B | 0x8C => OnReceiveAction::Chat,
        0x89 => OnReceiveAction::ReportAbuse,
        0x10 | 0x13 => OnReceiveAction::SchemaTeach,
        0x15 => OnReceiveAction::Disconnect,
        0x16 => OnReceiveAction::ConnectionLost,
        0x1B => OnReceiveAction::PhysicsPush,
        _ => OnReceiveAction::Ignore,
    }
}

/// `OnReceive` verdicts (IDA 0xb02e30): 1 is consumed-here, 2 is
/// forwarded to `Players`, 0 is dropped. Chat/report-abuse arms return 2
/// only when `Players` takes the packet; the `ReleaseAssert(false)`
/// fallthrough (Replicator.cpp:3321/3333) drops with 0. The marker arm
/// always drops with 0 after queueing (IDA 0xb03a62).
#[must_use]
pub fn on_receive_verdict(action: OnReceiveAction, forwarded: Option<bool>) -> u32 {
    match action {
        OnReceiveAction::MarkerItem => 0,
        OnReceiveAction::Chat | OnReceiveAction::ReportAbuse => match forwarded {
            Some(true) => 2,
            Some(false) | None => 0,
        },
        _ => 1,
    }
}

/// `OnReceive` disconnect line (IDA 0xb03144/0xb0353c): disconnect logs
/// `"Disconnect from %s"`, lost-connection logs `"Lost connection to
/// %s"`; the `disconnected` signal fires with `(address, lost)`.
#[must_use]
pub fn on_receive_log(addr: &str, lost: bool) -> String {
    if lost {
        format!("Lost connection to {addr}")
    } else {
        format!("Disconnect from {addr}")
    }
}

/// `boost::bind(mf4 sendFilteredChatMessage, replicator, _1.._4)` target
/// (IDA 0xb07980): the `list5` ctor retains the Replicator owner (the two
/// `shared_count` bumps are `Arc` bookkeeping here); the call forwards
/// the four trailing args to the method (AGENTS.md §4: `boost::bind` →
/// closures).
#[derive(Clone, Debug)]
pub struct ChatFilterCall {
    pub target: rbx_core::SharedPtr<Marker>,
}

/// `boost::bind` chat-filter ctor (IDA 0xb07980).
#[must_use]
pub fn bind_chat_filter(target: rbx_core::SharedPtr<Marker>) -> ChatFilterCall {
    ChatFilterCall { target }
}

/// `bind_t::operator()` for the chat filter (IDA 0xb07980): invokes the
/// bound method with the retained target plus the four call args.
pub fn call_chat_filter(
    call: &ChatFilterCall,
    addr: &crate::socket::SystemAddress,
    text: &str,
    filtered: &str,
    invoke: &mut dyn FnMut(&rbx_core::SharedPtr<Marker>, &crate::socket::SystemAddress, &str, &str),
) {
    invoke(&call.target, addr, text, filtered);
}

/// `boost::_bi::list1<value<shared_ptr<Replicator>>>::list1` (IDA
/// 0xb14fe0): copies the bound value with net +1 retain (the double-inc
/// plus single-dec on the two control words); `Arc::clone` is the
/// retain here.
#[must_use]
pub fn bind_list1_replicator(
    target: &rbx_core::SharedPtr<Marker>,
) -> rbx_core::SharedPtr<Marker> {
    rbx_core::SharedPtr::clone(target)
}

/// `RBX::Network::Replicator::PingJob` (IDA 0xb23cd8): the ping scheduler
/// job; scheduling stays engine-side, so the crate keeps a stateless
/// marker for the shared-ownership seam below.
#[derive(Clone, Debug, Default)]
pub struct PingJob;

/// `sp_pointer_construct<Replicator::PingJob, Replicator::PingJob>` (IDA
/// 0xb23cd8): publishes the fresh job control block; `Arc::new` is the
/// publish here.
#[must_use]
pub fn ping_job() -> rbx_core::SharedPtr<PingJob> {
    rbx_core::SharedPtr::from(Box::new(PingJob))
}

/// `rbx::signals::signal<...>::slot` for the chat filter (IDA
/// 0xb21844..0xb220c0): the intrusive refcounting is `Arc` bookkeeping
/// here; `signal_linked` mirrors the +12 owner link read by `connected`
/// (IDA 0xb21bf4: `*(_DWORD *)(a1 + 12) != 0`).
#[derive(Clone, Debug, Default)]
pub struct ChatSlot {
    pub signal_linked: bool,
}

/// `intrusive_ptr<slot>::operator=(slot *)` (IDA 0xb21844) and
/// `operator=(intrusive_ptr const&)` (IDA 0xb218f8): addref the incoming
/// slot (with the `c->strong < max() - 10` overflow assert,
/// intrusive_ptr_target.h:184, IDA 0xb2186c..0xb218b4), store it, then
/// release the old slot (destroying plus `free` at zero, IDA
/// 0xb218b8..0xb218ec). The atomic counts are `Arc` bookkeeping here.
pub fn chat_slot_assign(
    slot: &mut rbx_core::SharedPtr<ChatSlot>,
    next: &rbx_core::SharedPtr<ChatSlot>,
) {
    *slot = rbx_core::SharedPtr::clone(next);
}

/// `slot::connected` (IDA 0xb21bec): the +12 signal link is set.
#[must_use]
pub fn chat_slot_connected(slot: &ChatSlot) -> bool {
    slot.signal_linked
}

/// `callable<slot, bind_t<mf4 sendFilteredChatMessage, ...>>::call` (IDA
/// 0xb21bf8): resolves the member-function pointer through the vtable slot
/// (IDA 0xb21bfe..0xb21c1a) and invokes it with the retained target plus
/// the four call args — the same forward as [`call_chat_filter`].
pub fn chat_callable_call(
    call: &ChatFilterCall,
    addr: &crate::socket::SystemAddress,
    text: &str,
    filtered: &str,
    invoke: &mut dyn FnMut(&rbx_core::SharedPtr<Marker>, &crate::socket::SystemAddress, &str, &str),
) {
    call_chat_filter(call, addr, text, filtered, invoke);
}

/// `signal<...>::remove` (IDA 0xb21c58): asserts the item is not expired
/// (`!intrusive_ptr_expired`, signal.h:261/284), logs `"Removing item %p
/// from signal"` behind `FLog::SignalPrints`, then splices the slot out
/// of the intrusive `+8`-next chain. Returns whether the slot was linked.
pub fn chat_signal_remove(
    slots: &mut Vec<rbx_core::SharedPtr<ChatSlot>>,
    target: &rbx_core::SharedPtr<ChatSlot>,
) -> bool {
    if let Some(index) = slots.iter().position(|slot| rbx_core::SharedPtr::ptr_eq(slot, target)) {
        slots.remove(index);
        true
    } else {
        false
    }
}

/// `slot::safe_static_init_mutex` (IDA 0xb21d44): one-time `boost::mutex`
/// construction behind `__cxa_guard_acquire` with an `atexit` destructor;
/// Rust statics initialize inline, so there is nothing to do.
pub fn chat_slot_mutex_init() {}

/// `callable<slot, ...>::~callable` D2/D1/D0 (IDA 0xb21e28/0xb21fa4/0xb21fb0):
/// resets the slot vtable pair, drops the `shared_count` (the
/// `shared_count` dtor, IDA 0xb21e94), and releases the intrusive slot
/// (destroying plus `free` at zero, IDA 0xb21eb4..0xb21ef2). D0
/// additionally frees the allocation, which a Rust drop does.
pub fn chat_callable_drop(_call: ChatFilterCall) {}

/// `slot::~slot` D1/D0 (IDA 0xb22064/0xb220c0): resets the vtable pair and
/// releases the chained `+8` slot the same way. D0 additionally frees the
/// allocation, which a Rust drop does.
pub fn chat_slot_drop(_slot: rbx_core::SharedPtr<ChatSlot>) {}

/// `list5<value<shared_ptr<Replicator>>, arg<1..4>>::list5` (IDA 0xb221c8):
/// retains the bound Replicator owner (the spinlock-pool bumps, IDA
/// 0xb221fa..0xb222ba) and captures the four call placeholders into the
/// `storage4` tail (IDA 0xb222ca). The retains are `Arc` bookkeeping here.
#[must_use]
pub fn chat_list5(target: &rbx_core::SharedPtr<Marker>) -> ChatFilterCall {
    bind_chat_filter(rbx_core::SharedPtr::clone(target))
}

/// `storage4<value<shared_ptr<Replicator>>, arg<1..3>>::storage4` (IDA
/// 0xb22618): keeps the bound owner plus the first three placeholders.
#[must_use]
pub fn chat_store4(call: &ChatFilterCall) -> ChatFilterCall {
    ChatFilterCall { target: rbx_core::SharedPtr::clone(&call.target) }
}

/// `storage2<value<shared_ptr<Replicator>>, arg<1>>::storage2` (IDA
/// 0xb22a68): keeps only the bound owner and the first placeholder.
#[must_use]
pub fn chat_store2(call: &ChatFilterCall) -> ChatFilterCall {
    ChatFilterCall { target: rbx_core::SharedPtr::clone(&call.target) }
}

/// `RBX::Name::callDoDeclare<sClusterPacketCache>` (IDA 0xb2332c):
/// one-time `Name::declare(&sClusterPacketCache)` behind the
/// `__cxa_guard` statics guard (IDA 0xb23384..0xb233b4). Returns whether
/// this call ran the declaration.
#[must_use]
pub fn declare_cluster_packet_cache(declared: &mut bool) -> bool {
    if *declared {
        return false;
    }
    *declared = true;
    true
}

/// `Voxel::Serializer<Grid>::encodeCells` wire widths (IDA
/// 0xb15f50/0xb173b0/0xb18564, Serializer.h): each cell writes 5 + 4 + 5
/// bits, each new 32/16/32 region writes 1 + 1 + 4 + 2 + 4 header bits,
/// and every chunk (plus the tail) terminates with the 2-bit
/// `kEndSequenceMarker`.
pub const VOXEL_CELL_BIT_WIDTHS: (u8, u8, u8) = (5, 4, 5);
pub const VOXEL_REGION_BIT_WIDTHS: (u8, u8, u8) = (4, 2, 4);
pub const VOXEL_END_MARKER_BIT_WIDTH: u8 = 2;

/// `encodeCells` region key (IDA 0xb160c0..0xb160de): cells share a
/// header while `(x >> 5, y >> 4, z >> 5)` matches the cached region.
#[must_use]
pub fn voxel_region_of(cell: (i32, i32, i32)) -> (i32, i32, i32) {
    (cell.0 >> 5, cell.1 >> 4, cell.2 >> 5)
}

/// `encodeCells` budget break (IDA 0xb16018/0xb162f4): a non-negative
/// `maxBytes` stops the chunk once the written bytes reach it.
#[must_use]
pub fn voxel_budget_hit(used_bytes: usize, budget: i32) -> bool {
    budget != -1 && used_bytes >= budget as usize
}

/// `boost::_bi::list4/storage4<weak_ptr<Replicator>, ReplicationData *,
/// arg<1>, arg<2>>` capture (IDA 0xb1c5cc/0xb1c790): the weak owner, the
/// raw replication data, and the two call placeholders. The
/// spinlock-pool retains are `Arc`/`Weak` bookkeeping here.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplicationBind {
    pub target_alive: bool,
    pub has_data: bool,
}

/// `list4` ctor (IDA 0xb1c5cc).
#[must_use]
pub fn replication_bind4(target_alive: bool, has_data: bool) -> ReplicationBind {
    ReplicationBind { target_alive, has_data }
}

/// `storage4` ctor (IDA 0xb1c790): stores the full 4-tuple.
#[must_use]
pub fn replication_store4(bind: &ReplicationBind) -> ReplicationBind {
    *bind
}

/// `storage3` capture (IDA 0xb1c954): the tail placeholder (`arg<2>`)
/// is dropped, keeping weak owner, data, and `arg<1>`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplicationBind3 {
    pub target_alive: bool,
    pub has_data: bool,
}

/// `storage3` ctor (IDA 0xb1c954).
#[must_use]
pub fn replication_store3(bind: &ReplicationBind) -> ReplicationBind3 {
    ReplicationBind3 { target_alive: bind.target_alive, has_data: bind.has_data }
}

/// `storage2` capture (IDA 0xb1cb18): only the weak owner and the raw
/// data are kept.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ReplicationBind2 {
    pub target_alive: bool,
    pub has_data: bool,
}

/// `storage2` ctor (IDA 0xb1cb18).
#[must_use]
pub fn replication_store2(bind: &ReplicationBind) -> ReplicationBind2 {
    ReplicationBind2 { target_alive: bind.target_alive, has_data: bind.has_data }
}

/// `RBX::Network::SharedStringProtectedDictionary` (IDA 0xb2058c): the
/// per-property protected string table behind
/// `getSharedPropertyProtectedDictionary`; the map stays engine-side,
/// only the shared-ownership shape crosses here.
#[derive(Clone, Debug, Default)]
pub struct SharedStringProtectedDictionary {
    pub entries: std::collections::HashMap<u32, String>,
}

/// `sp_pointer_construct<SharedStringProtectedDictionary>` (IDA
/// 0xb2058c): publishes the fresh control block; `Arc::new` is the
/// publish here.
#[must_use]
pub fn protected_string_dict() -> rbx_core::SharedPtr<SharedStringProtectedDictionary> {
    rbx_core::SharedPtr::from(Box::new(SharedStringProtectedDictionary::default()))
}

/// `sp_counted_impl_p` D1/D0/dispose (IDA 0xb20850/0xb20854/0xb20860 and
/// the 0xb20d10/0xb20d14/0xb20d20 twins): D1 runs the dispose, D0
/// additionally frees (IDA `operator delete`); a Rust drop does both.
pub fn shared_dict_drop<T>(_dict: rbx_core::SharedPtr<T>) {}

/// `sp_counted_impl_p::get_deleter/get_untyped_deleter` (IDA
/// 0xb209b0/0xb209b4 and the 0xb20e64/0xb20e68 twins): no custom
/// deleter is ever installed, so both return null.
#[must_use]
pub fn shared_null_deleter() -> *const () {
    std::ptr::null()
}
/// `Voxel::Serializer<Grid>::encodeCells` chunk writer (IDA
/// 0xb15f50/0xb173b0/0xb18564, Serializer.h:145): per cell, a 1-bit
/// same-region flag — or `1, 0` plus the 4/2/4 region words on a region
/// change — then the 5/4/5 cell bits from `encodeFromPosition`; each
/// chunk ends with the 2-bit `kEndSequenceMarker` (`end_marker` here,
/// engine-side constant) and the tail writes a final 2-bit `1`s word
/// (IDA 0xb163f6). Stops when the byte budget is hit (IDA
/// 0xb16018/0xb162f4); the grid reads, cluster-chunk iteration, and the
/// `unused == nextPos` assert stay engine-side.
pub fn voxel_encode_cells(
    stream: &mut BitStream,
    cells: &[(i32, i32, i32)],
    budget: i32,
    end_marker: u8,
) {
    let mut region: Option<(i32, i32, i32)> = None;
    for &(x, y, z) in cells {
        // IDA 0xb16018: `(bitsUsed + 7) >> 3 >= maxBytes` breaks.
        if voxel_budget_hit((stream.bits_written() + 7) >> 3, budget) {
            break;
        }
        let key = voxel_region_of((x, y, z));
        if region == Some(key) {
            // IDA 0xb160e0: same region, single 0 bit.
            stream.write_bits(0, 1);
        } else {
            // IDA 0xb1610a..0xb16162: `1, 0` + 4/2/4 region words.
            region = Some(key);
            stream.write_bits(1, 1);
            stream.write_bits(0, 1);
            stream.write_bits((key.0 & 0xF) as u32, VOXEL_REGION_BIT_WIDTHS.0);
            stream.write_bits((key.1 & 0x3) as u32, VOXEL_REGION_BIT_WIDTHS.1);
            stream.write_bits((key.2 & 0xF) as u32, VOXEL_REGION_BIT_WIDTHS.2);
        }
        // IDA 0xb161f8..0xb1622c: 5/4/5 cell bits.
        stream.write_bits((x & 0x1F) as u32, VOXEL_CELL_BIT_WIDTHS.0);
        stream.write_bits((y & 0x0F) as u32, VOXEL_CELL_BIT_WIDTHS.1);
        stream.write_bits((z & 0x1F) as u32, VOXEL_CELL_BIT_WIDTHS.2);
    }
    stream.write_bits(u32::from(end_marker & 0x3), VOXEL_END_MARKER_BIT_WIDTH);
    stream.write_bits(0x3, VOXEL_END_MARKER_BIT_WIDTH);
}
