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
    }
