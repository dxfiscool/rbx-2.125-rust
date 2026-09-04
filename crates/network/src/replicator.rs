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
}
