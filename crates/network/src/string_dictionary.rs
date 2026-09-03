//! `RBX::Network::SenderDictionary` / `ReceiverDictionary` /
//! `SharedStringDictionary` — rotating string interning for the wire.
//!
//! Decompiled from 0x9a2790 (`SenderDictionary<std::string>::send`), 0x9a2990
//! (`ReceiverDictionary<std::string>::receive`), 0x9a2160
//! (`SharedStringDictionary::serializeString`) and 0x9a2294
//! (`deserializeString`).
//!
//! Layout (IDA 0x9a2804..0x9a28b8, 0x9a29b8..0x9a29e4): the sender holds a
//! `map<string, u8>` plus 128 inline slots and a `next` counter at +536; the
//! receiver (at +540) holds 128 slots. Code byte `0` means empty, `< 0x80`
//! recalls a slot, `>= 0x80` carries a fresh string for slot `byte & 0x7F`.

#![allow(dead_code)]

use std::collections::HashMap;

use super::bitstream::BitStream;

/// Number of rotating intern slots on each side (IDA 0x9a28b8, 0x9a29e4).
pub const SLOT_COUNT: usize = 128;

/// `RBX::Network::SenderDictionary<std::string>`.
#[derive(Clone, Debug, Default)]
pub struct SenderDictionary {
    map: HashMap<String, u8>,
    slots: Vec<String>,
    // BUG: original at 0x9a28b8 — `next` zero-inits, so the first interned
    // string takes slot 0 and its recall code is 0, which the receiver
    // (IDA 0x9a29cc) decodes as empty. Preserved as-is.
    next: u8,
}

impl SenderDictionary {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            slots: vec![String::new(); SLOT_COUNT],
            next: 0,
        }
    }

    /// `RBX::Network::SenderDictionary<std::string>::send` (IDA 0x9a2790).
    pub fn send(&mut self, stream: &mut BitStream, s: &str) {
        if s.is_empty() {
            // IDA 0x9a28c0: empty strings encode as one zero byte.
            stream.write_u8(0);
            return;
        }
        if let Some(&code) = self.map.get(s) {
            // IDA 0x9a28d2: known string recalls its code byte.
            stream.write_u8(code);
            return;
        }
        // IDA 0x9a2848..0x9a286e: evict whatever occupies the rotating slot,
        // publish the new string there, and emit `slot | 0x80` + full string.
        let idx = self.next as usize % SLOT_COUNT;
        let old = std::mem::replace(&mut self.slots[idx], s.to_owned());
        if !old.is_empty() {
            self.map.remove(&old);
        }
        self.map.insert(s.to_owned(), idx as u8);
        stream.write_u8(idx as u8 | 0x80);
        stream.write_string(s);
        // IDA 0x9a28b8: `next = next % 127 + 1` — slot 0 is written once,
        // then slots 1..=127 rotate.
        self.next = self.next % 127 + 1;
    }
}

/// `RBX::Network::ReceiverDictionary<std::string>`.
#[derive(Clone, Debug, Default)]
pub struct ReceiverDictionary {
    slots: Vec<String>,
}

impl ReceiverDictionary {
    pub fn new() -> Self {
        Self {
            slots: vec![String::new(); SLOT_COUNT],
        }
    }

    /// `RBX::Network::ReceiverDictionary<std::string>::receive`
    /// (IDA 0x9a2990). Always returns `true` (IDA 0x9a29f0).
    pub fn receive(&mut self, stream: &mut BitStream, out: &mut String) -> bool {
        // IDA 0x9a29a4: single code byte.
        let code = stream.read_u8().unwrap_or(0);
        if code == 0 {
            // IDA 0x9a29cc: clear.
            out.clear();
        } else if code < 0x80 {
            // IDA 0x9a29b8: slot recall.
            *out = self.slots[code as usize % SLOT_COUNT].clone();
        } else {
            // IDA 0x9a29d6: fresh string follows on the wire. The decompiler
            // drops the out-param of `operator>>`; it reads into `*out`,
            // which is then published to the slot (IDA 0x9a29e8).
            *out = stream.read_string();
            self.slots[(code & 0x7F) as usize % SLOT_COUNT] = out.clone();
        }
        true
    }
}

/// `RBX::Network::SharedStringDictionary`: sender inline, receiver at +540
/// (IDA 0x9a22a6).
#[derive(Clone, Debug, Default)]
pub struct SharedStringDictionary {
    pub sender: SenderDictionary,
    pub receiver: ReceiverDictionary,
}

impl SharedStringDictionary {
    pub fn new() -> Self {
        Self {
            sender: SenderDictionary::new(),
            receiver: ReceiverDictionary::new(),
        }
    }

    /// `serializeString` (IDA 0x9a2160): tail-calls `SenderDictionary::send`.
    pub fn serialize_string(&mut self, s: &str, stream: &mut BitStream) {
        self.sender.send(stream, s); // IDA 0x9a216e
    }

    /// `deserializeString` (IDA 0x9a2294): tail-calls
    /// `ReceiverDictionary::receive` on the +540 sub-object.
    pub fn deserialize_string(&mut self, out: &mut String, stream: &mut BitStream) -> bool {
        self.receiver.receive(stream, out) // IDA 0x9a22a6
    }
}

/// `RBX::Network::SenderDictionary<RBX::Name const*>`: the `std::string`
/// sender keyed by `Name` identity instead of string contents.
///
/// Decompiled from 0x9a1930. Slot counter at +536, slots at +24, and the
/// `next = next % 127 + 1` rotation (IDA 0x9a19b0..0x9a19c8) match
/// [`SenderDictionary`]; only the key (a `Name` pointer) and the emitted
/// payload (that `Name`'s text via `RBX::operator<<`, IDA 0x9a19a8) differ.
/// The empty-text encodes as one zero byte (IDA 0x9a19d0..0x9a19d4),
/// exactly like the empty-string case of [`SenderDictionary::send`].
#[derive(Clone, Debug, Default)]
pub struct NameSenderDictionary {
    map: HashMap<usize, u8>,
    slots: Vec<Option<(usize, String)>>,
    // BUG: same slot-0 recall quirk as `SenderDictionary::next`.
    next: u8,
}

impl NameSenderDictionary {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            slots: vec![None; SLOT_COUNT],
            next: 0,
        }
    }

    /// `RBX::Network::SenderDictionary<RBX::Name const*>::send` (IDA 0x9a1930).
    ///
    /// `id` stands in for the `RBX::Name const*` key, `text` for its string
    /// contents.
    pub fn send(&mut self, stream: &mut BitStream, id: usize, text: &str) {
        if text.is_empty() {
            // IDA 0x9a19d0: empty names encode as one zero byte.
            stream.write_u8(0);
            return;
        }
        if let Some(&code) = self.map.get(&id) {
            // IDA 0x9a19da..0x9a19f4: known name recalls its code byte.
            stream.write_u8(code);
            return;
        }
        // IDA 0x9a196e..0x9a198e: evict the rotating slot's occupant.
        let idx = self.next as usize % SLOT_COUNT;
        if let Some((old_id, _)) = self.slots[idx].replace((id, text.to_owned())) {
            self.map.remove(&old_id);
        }
        self.map.insert(id, idx as u8);
        // IDA 0x9a199a..0x9a19a8: `slot | 0x80` + full name text.
        stream.write_u8(idx as u8 | 0x80);
        stream.write_string(text);
        // IDA 0x9a19b0..0x9a19ca: `next = next % 127 + 1`.
        self.next = self.next % 127 + 1;
    }
}

/// `RBX::Network::ReceiverStringDictionary`: owns a
/// `ReceiverDictionary<std::string>` at +0.
#[derive(Clone, Debug, Default)]
pub struct ReceiverStringDictionary {
    inner: ReceiverDictionary,
}

impl ReceiverStringDictionary {
    pub fn new() -> Self {
        Self {
            inner: ReceiverDictionary::new(),
        }
    }

    /// `RBX::Network::ReceiverStringDictionary::receive<std::string>`
    /// (IDA 0x9a29f4): code byte 0 clears (IDA 0x9a2a34), `< 0x80` recalls
    /// the slot (IDA 0x9a2a26 `get`), otherwise the fresh string follows on
    /// the wire and is published to `slot & 0x7F` (IDA 0x9a2a3e..0x9a2a4e
    /// `learn`). Always returns `true`. This is the same wire protocol as
    /// [`ReceiverDictionary::receive`] (IDA 0x9a2990), so it delegates.
    pub fn receive(&mut self, stream: &mut BitStream, out: &mut String) -> bool {
        self.inner.receive(stream, out)
    }
}

/// `RBX::Network::SharedStringProtectedDictionary`: sender inline, receiver
/// at +540 (IDA 0x9a265a).
#[derive(Clone, Debug, Default)]
pub struct SharedStringProtectedDictionary {
    pub sender: SenderDictionary,
    pub receiver: ReceiverDictionary,
}

impl SharedStringProtectedDictionary {
    pub fn new() -> Self {
        Self {
            sender: SenderDictionary::new(),
            receiver: ReceiverDictionary::new(),
        }
    }

    /// `serializeString` (IDA 0x9a2514): tail-calls
    /// `SenderDictionary<std::string>::send`.
    pub fn serialize_string(&mut self, s: &str, stream: &mut BitStream) {
        self.sender.send(stream, s); // IDA 0x9a2522
    }

    /// `deserializeString` (IDA 0x9a2648): tail-calls
    /// `ReceiverStringDictionary::receive` on the +540 sub-object.
    pub fn deserialize_string(&mut self, out: &mut String, stream: &mut BitStream) -> bool {
        self.receiver.receive(stream, out) // IDA 0x9a265a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_codes_zero() {
        let mut dict = SharedStringDictionary::new();
        let mut s = BitStream::new();
        dict.serialize_string("", &mut s);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = String::from("x");
        assert!(dict.deserialize_string(&mut out, &mut r));
        assert_eq!(out, "");
    }

    #[test]
    fn fresh_then_recalled_string() {
        let mut tx = SharedStringDictionary::new();
        let mut rx = SharedStringDictionary::new();
        let mut s = BitStream::new();
        // Slot 0 is taken first; recall is exercised on the second string
        // (slot 1), since a slot-0 recall decodes as empty per the noted
        // original quirk.
        tx.serialize_string("brick", &mut s);
        tx.serialize_string("mortar", &mut s);
        tx.serialize_string("mortar", &mut s);
        let bytes = s.into_bytes();
        // The recall emit must be a bare code byte (< 0x80), not a resend.
        assert!(bytes.len() < "brick".len() + "mortar".len() * 2 + 12);
        let mut r = BitStream::from_bytes(&bytes);
        let mut out = String::new();
        assert!(rx.deserialize_string(&mut out, &mut r));
        assert_eq!(out, "brick");
        assert!(rx.deserialize_string(&mut out, &mut r));
        assert_eq!(out, "mortar");
        assert!(rx.deserialize_string(&mut out, &mut r));
        assert_eq!(out, "mortar");
    }
}

#[cfg(test)]
mod name_dict_tests {
    use super::*;

    #[test]
    fn empty_name_codes_zero() {
        let mut dict = NameSenderDictionary::new();
        let mut s = BitStream::new();
        dict.send(&mut s, 1, "");
        assert_eq!(s.into_bytes(), vec![0]);
    }

    #[test]
    fn fresh_then_recalled_name() {
        let mut dict = NameSenderDictionary::new();
        let mut s = BitStream::new();
        dict.send(&mut s, 7, "Workspace");
        dict.send(&mut s, 9, "Lighting");
        dict.send(&mut s, 9, "Lighting");
        let bytes = s.into_bytes();
        // Fresh emits are `slot | 0x80` + text; the recall is one bare byte.
        assert_eq!(bytes[0], 0x80);
        assert_eq!(*bytes.last().unwrap(), 1);
    }

    #[test]
    fn protected_dict_roundtrip() {
        let mut tx = SharedStringProtectedDictionary::new();
        let mut rx = SharedStringProtectedDictionary::new();
        let mut s = BitStream::new();
        tx.serialize_string("secret", &mut s);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = String::new();
        assert!(rx.deserialize_string(&mut out, &mut r));
        assert_eq!(out, "secret");
    }

    #[test]
    fn receiver_string_dict_clears_on_zero() {
        let mut dict = ReceiverStringDictionary::new();
        let mut s = BitStream::new();
        s.write_u8(0);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        let mut out = String::from("leftover");
        assert!(dict.receive(&mut r, &mut out));
        assert_eq!(out, "");
    }
}
