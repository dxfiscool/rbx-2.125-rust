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
