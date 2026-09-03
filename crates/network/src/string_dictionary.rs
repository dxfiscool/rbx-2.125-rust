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
    /// Slot recall for `IdSerializer::deserializeId` (IDA 0x960a20,
    /// `*(this + 632 + 4 * byte)` on the `< 0x80` arm).
    pub fn recall(&self, slot: u8) -> String {
        self.slots[(slot & 0x7F) as usize % SLOT_COUNT].clone()
    }

    /// Slot publish for `IdSerializer::deserializeId` (IDA 0x960a20,
    /// `assign(this + 632 + 4 * (byte & 0x7F))` on the fresh-string arm).
    pub fn learn(&mut self, slot: u8, s: &str) {
        self.slots[(slot & 0x7F) as usize % SLOT_COUNT] = s.to_owned();
    }

    /// `RBX::Network::ReceiverDictionary<std::string>::setDefault` (IDA
    /// 0x9a19f8: `_M_mutate` erasing the whole string).
    pub fn set_default(out: &mut String) {
        out.clear();
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

    /// `RBX::Network::SenderDictionary<RBX::Name const*>::trySend` (IDA 0x965f98).
    ///
    /// Unlike [`NameSenderDictionary::send`], this only emits when the name
    /// is already known: empty text writes one zero byte and returns `true`
    /// (IDA 0x965f98 `__src = 0` arm), a known id writes its recall code
    /// byte (asserted `< 0x80`, Dictionary.h:133) and returns `true`, and an
    /// unknown id writes nothing and returns `false`.
    pub fn try_send(&self, stream: &mut BitStream, id: usize, text: &str) -> bool {
        if text.is_empty() {
            stream.write_u8(0);
            return true;
        }
        if let Some(&code) = self.map.get(&id) {
            debug_assert!(code & 0x80 == 0);
            stream.write_u8(code);
            return true;
        }
        false
    }
}

/// `RBX::Network::ReceiverStringDictionary`: owns a
/// `ReceiverDictionary<std::string>` at +0, a hash table at +512 and the
/// validation flag at +516 (IDA 0x9a1a0c / 0x9a1d80).
#[derive(Clone, Debug)]
pub struct ReceiverStringDictionary {
    inner: ReceiverDictionary,
    /// Hash table at +512, lazily `new[]`d on first validated `learn`
    /// (IDA 0x9a1a0c); always present here, which is observably identical.
    hashes: [u32; SLOT_COUNT],
    /// Validation flag at +516 (IDA 0x9a1a0c / 0x9a1d80).
    hash_validation: bool,
}

impl Default for ReceiverStringDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Slot-string hash shared by `learn`/`get` (IDA 0x9a1a0c / 0x9a1d80):
/// `h ^= (h << 6) + (h >> 2) + c - 1640531527` over `"a" + s + "s"`
/// (ARM `char` is unsigned, so bytes feed in directly; wrapping matches the
/// original's unsigned arithmetic).
fn slot_hash(s: &str) -> u32 {
    let mut hash = 0u32;
    for chunk in [b"a".as_slice(), s.as_bytes(), b"s".as_slice()] {
        for &byte in chunk {
            hash ^= (hash << 6)
                .wrapping_add(hash >> 2)
                .wrapping_add(u32::from(byte))
                .wrapping_sub(1_640_531_527);
        }
    }
    hash
}

impl ReceiverStringDictionary {
    pub fn new() -> Self {
        Self {
            inner: ReceiverDictionary::new(),
            hashes: [0; SLOT_COUNT],
            hash_validation: false,
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

    /// Validation flag at +516: set by the (engine-side) dictionary setup
    /// that enables hash checking in `learn`/`get`.
    pub fn set_hash_validation(&mut self, on: bool) {
        self.hash_validation = on;
    }

    /// Stored hash for `slot` (the +512 table, IDA 0x9a1a0c).
    pub fn hash(&self, slot: u8) -> u32 {
        self.hashes[slot as usize % SLOT_COUNT]
    }

    /// `RBX::Network::ReceiverStringDictionary::learn` (IDA 0x9a1a0c):
    /// publishes `s` to the slot, and when validation is on records the
    /// `"a" + s + "s"` hash alongside.
    pub fn learn(&mut self, slot: u8, s: &str) {
        // IDA 0x9a1a0c: `assign(this + 4 * slot, s)`.
        let idx = slot as usize % SLOT_COUNT;
        self.inner.learn(slot, s);
        if self.hash_validation {
            self.hashes[idx] = slot_hash(s);
        }
    }

    /// `RBX::Network::ReceiverStringDictionary::get` (IDA 0x9a1d80):
    /// recalls the slot into `out`; with validation on, a hash mismatch
    /// clears `out` (the `_M_mutate` erase) and returns `false`.
    pub fn get(&self, slot: u8, out: &mut String) -> bool {
        // IDA 0x9a1d80: `assign(out, this + 4 * slot)`, then `return 1` ...
        *out = self.inner.recall(slot);
        if !self.hash_validation {
            return true;
        }
        // ... else the `hashTable` assert (always allocated here, so it
        // holds trivially — cf. the Dictionary.cpp:83 `ReleaseAssert`) and
        // the `"a" + out + "s"` recomputation.
        if slot_hash(out) != self.hashes[slot as usize % SLOT_COUNT] {
            out.clear();
            return false;
        }
        true
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

#[cfg(test)]
mod validated_learn_tests {
    use super::*;

    #[test]
    fn learn_get_roundtrip_without_validation() {
        let dict = ReceiverStringDictionary::new();
        let mut dict = dict;
        dict.learn(3, "hello");
        let mut out = String::new();
        assert!(dict.get(3, &mut out));
        assert_eq!(out, "hello");
    }

    #[test]
    fn validated_get_detects_tamper() {
        let mut dict = ReceiverStringDictionary::new();
        dict.set_hash_validation(true);
        dict.learn(5, "brick");
        let mut out = String::new();
        assert!(dict.get(5, &mut out));
        assert_eq!(out, "brick");
        // Corrupt the slot behind the hash table's back: the mismatch
        // clears the out-param and returns false (IDA 0x9a1d80).
        dict.learn(5, "brick");
        dict.inner.learn(5, "mortar");
        assert!(!dict.get(5, &mut out));
        assert_eq!(out, "");
    }
    #[test]
    fn try_send_only_recalls() {
        let mut dict = NameSenderDictionary::new();
        let mut s = BitStream::new();
        // Unknown: nothing written, false (IDA 0x965f98 miss path).
        assert!(!dict.try_send(&mut s, 7, "Workspace"));
        assert_eq!(s.bits_written(), 0);
        // Empty text: one zero byte, true.
        assert!(dict.try_send(&mut s, 7, ""));
        // After real sends the names are known: bare recall codes, true.
        dict.send(&mut s, 7, "Workspace");
        dict.send(&mut s, 9, "Lighting");
        let mut t = BitStream::new();
        assert!(dict.try_send(&mut t, 9, "Lighting"));
        assert_eq!(t.into_bytes(), vec![1]);
    }
}
