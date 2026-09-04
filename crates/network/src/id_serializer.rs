//! `RBX::Network::IdSerializer` + `DescriptorSender<T>` /
//! `DescriptorReceiver<T>` — instance-identity (Guid) codecs and the
//! descriptor-name tables used by the replicators.
//!
//! Decompiled from 0x9604a4 (ctor), 0x960624 (`setMaxGuidIndexBit`),
//! 0x960634 (`trySerializeId`), 0x96068c / 0x9607ec / 0x961094
//! (`serializeId` / `serializeId(Guid)` / `serializeInstanceRef`),
//! 0x9606d8 (`canSerializeId`), 0x96075c (`onServiceProvider`), 0x960784
//! (`extractId`), 0x9607ac (`sendId`), 0x960814
//! (`serializeIdWithoutDictionary`), 0x960a20 (`deserializeId`), 0x960c8c
//! (`deserializeIdWithoutDictionary`), 0x960f0c (`setRefValue`), 0x960f28
//! (`resolvePendingReferences`), 0x9610e0 (`deserializeInstanceRef`),
//! 0x961178 (`addPendingRef`), 0x961480 / 0x961700 / 0x961ca4 / 0x96208c
//! (`DescriptorSender<T>::teachName`), 0x961490 / 0x9618c4 / 0x961e68 /
//! 0x96209c (`DescriptorReceiver<T>::learnName`), 0x962300 / 0x962464 /
//! 0x962694 / 0x9628c4 (`DescriptorSender<T>` ctors), 0x96025c
//! (`deserialize<ContentId>`), 0x960380 (`deserializeStringProperty`),
//! 0x965f98 (`SenderDictionary<Name>::trySend`), 0x9a19f8
//! (`ReceiverDictionary<std::string>::setDefault`).
//!
//! Wire model: an id is the dictionary-coded guid name (`NameSenderDictionary`
//! at +92 / `ReceiverDictionary` at +632) followed by the low
//! `max_guid_index_bits` (24 or 32, at +1176) of the guid index, written
//! MSB-first via `WriteBits(..., 1)`; a null instance is 8 zero bits.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

use super::bitstream::BitStream;
use super::string_dictionary::{NameSenderDictionary, ReceiverDictionary};

/// Stand-in for the null `RBX::Name` (`RBX::Name::getNullName`, IDA 0x960a20):
/// the empty text declares to 0 and never carries index bits.
pub const NULL_NAME: u32 = 0;

/// `RBX::Guid::Data`: the name word at +0, the index at +4 (IDA 0x960a20).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GuidData {
    pub name: u32,
    pub index: u32,
}

/// `RBX::Network::IdSerializer::Id`: presence flag at +0, guid at +4
/// (IDA 0x960784 `extractId`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct IdValue {
    pub present: bool,
    pub data: GuidData,
}

/// `RBX::Network::IdSerializer::WaitItem`: the descriptor whose ref property
/// waits plus the instance that will fill it (`SharedPtr<RBX::Instance>`
/// stands in as an id; IDA 0x961178).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WaitItem {
    pub descriptor: u32,
    pub instance: u32,
}

/// `RBX::Name` interning reduced to `text <-> id` (`RBX::Name::declare` /
/// `RBX::Name::lookup`, IDA 0x960a20 / 0x961490). Ids start at 1; 0 is
/// [`NULL_NAME`] (the empty text declares to null, matching the
/// `setDefault` + `declare` path of `deserializeId`).
#[derive(Clone, Debug, Default)]
pub struct NameTable {
    by_text: HashMap<String, u32>,
    by_id: Vec<String>,
}

impl NameTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// `RBX::Name::declare(std::string const&)` (IDA 0x960a20).
    pub fn declare(&mut self, text: &str) -> u32 {
        if text.is_empty() {
            return NULL_NAME;
        }
        if let Some(&id) = self.by_text.get(text) {
            return id;
        }
        let id = self.by_id.len() as u32 + 1;
        self.by_text.insert(text.to_owned(), id);
        self.by_id.push(text.to_owned());
        id
    }

    /// `RBX::Name::lookup` (IDA 0x961490): 0 when unknown, like the null
    /// name the original compares misses against.
    pub fn lookup(&self, text: &str) -> u32 {
        self.by_text.get(text).copied().unwrap_or(NULL_NAME)
    }

    /// Text for a declared id; unknown ids (engine-side names) read as "".
    pub fn text(&self, id: u32) -> &str {
        if id == NULL_NAME {
            return "";
        }
        self.by_id.get(id as usize - 1).map(String::as_str).unwrap_or("")
    }
}

/// `RBX::Network::IdSerializer` (IDA 0x9604a4).
#[derive(Clone, Debug, Default)]
pub struct IdSerializer {
    sender: NameSenderDictionary,
    receiver: ReceiverDictionary,
    names: NameTable,
    /// `GuidItem<Instance>::Registry` membership stand-in: `Registry::reg`
    /// (the `+1144` shared_ptr, IDA 0x960634) plus the serializer-local
    /// guid map at +24 (IDA 0x9606d8) reduced to one set.
    known: HashSet<u32>,
    /// `std::map<Guid::Data, std::vector<WaitItem>>` at +1152
    /// (IDA 0x961178 / 0x960f28).
    pending: HashMap<GuidData, Vec<WaitItem>>,
    /// Guid-index bit count at +1176, set by [`IdSerializer::set_max_guid_index_bit`].
    max_guid_index_bits: u32,
    /// `SFFlag::DisableGuidStringCompression` (IDA 0x960814 / 0x960c8c).
    pub disable_guid_string_compression: bool,
    /// `shared_ptr<GuidItem<Instance>::Registry>` at +1144 (IDA 0x96075c).
    registry_handle: Option<u32>,
}

impl IdSerializer {
    /// `RBX::Network::IdSerializer::IdSerializer` (IDA 0x9604a4): base
    /// `Instance` + vtable init and the +92 / +632 dictionaries; the index
    /// width stays unset until [`IdSerializer::set_max_guid_index_bit`].
    pub fn new() -> Self {
        Self {
            sender: NameSenderDictionary::new(),
            receiver: ReceiverDictionary::new(),
            ..Self::default()
        }
    }

    /// Stand-in for `GuidItem<Instance>::Registry::reg` (IDA 0x960634).
    fn reg(&mut self, guid: GuidData) {
        self.known.insert(guid.name);
    }

    /// `RBX::Network::IdSerializer::setMaxGuidIndexBit` (IDA 0x960624):
    /// `v2 = bits - 1; width = if v2 < 0xE { 24 } else { 32 }`.
    pub fn set_max_guid_index_bit(&mut self, bits: u32) -> &mut Self {
        // IDA 0x960624: unsigned compare, so 0 wraps huge and takes 32.
        self.max_guid_index_bits = if bits.wrapping_sub(1) < 0xE { 24 } else { 32 };
        self
    }

    pub fn max_guid_index_bits(&self) -> u32 {
        self.max_guid_index_bits
    }

    /// `RBX::Network::IdSerializer::trySerializeId` (IDA 0x960634): null
    /// writes 8 zero bits and succeeds; otherwise the name must already be
    /// dictionary-known (`trySend`, IDA 0x965f98) before the index bits go
    /// out. Returns `false` without writing anything when unknown.
    pub fn try_serialize_id(&mut self, stream: &mut BitStream, instance: Option<GuidData>) -> bool {
        match instance {
            None => {
                // IDA 0x960634 null arm: zero byte, `v6 = 1`.
                stream.write_u8(0);
                true
            }
            Some(guid) => {
                self.reg(guid);
                let text = self.names.text(guid.name).to_owned();
                if self.sender.try_send(stream, guid.name as usize, &text) {
                    // IDA 0x960634: `WriteBits(guid + 4, this + 294)`.
                    stream.write_bits(guid.index, self.max_guid_index_bits as u8);
                    true
                } else {
                    false
                }
            }
        }
    }

    /// `RBX::Network::IdSerializer::serializeId(RBX::Instance const*)`
    /// (IDA 0x96068c): unconditional `send` instead of `trySend`.
    /// `serializeInstanceRef` (IDA 0x961094) has the identical body and
    /// delegates here.
    pub fn serialize_id(&mut self, stream: &mut BitStream, instance: Option<GuidData>) {
        match instance {
            None => stream.write_u8(0),
            Some(guid) => {
                self.reg(guid);
                let text = self.names.text(guid.name).to_owned();
                // IDA 0x96068c: `send(this + 92, ...)` then index bits.
                self.sender.send(stream, guid.name as usize, &text);
                stream.write_bits(guid.index, self.max_guid_index_bits as u8);
            }
        }
    }

    /// `RBX::Network::IdSerializer::serializeId(RBX::Guid::Data const&)`
    /// (IDA 0x9607ec).
    pub fn serialize_guid(&mut self, stream: &mut BitStream, guid: &GuidData) {
        let text = self.names.text(guid.name).to_owned();
        self.sender.send(stream, guid.name as usize, &text);
        stream.write_bits(guid.index, self.max_guid_index_bits as u8);
    }

    /// `RBX::Network::IdSerializer::canSerializeId` (IDA 0x9606d8):
    /// null is false; otherwise membership of the guid name after `reg`.
    pub fn can_serialize_id(&mut self, instance: Option<GuidData>) -> bool {
        match instance {
            None => false,
            Some(guid) => {
                self.reg(guid);
                // IDA 0x9606d8: hash walk over the +24 map, `return found != 0`.
                self.known.contains(&guid.name)
            }
        }
    }

    /// `RBX::Network::IdSerializer::onServiceProvider` (IDA 0x96075c):
    /// `reset()` the +1144 registry handle, then adopt a fresh
    /// `GuidRegistryService` when the new provider is non-null. The
    /// serializer-local maps are untouched.
    pub fn on_service_provider(&mut self, registry: Option<u32>) {
        // IDA 0x96075c..0x960770: `reset()` first, ...
        self.registry_handle = None;
        // IDA 0x96076c..0x96077e: ... adopt on non-null (`create<...>` + `operator=`).
        if registry.is_some() {
            self.registry_handle = registry;
        }
    }

    /// `RBX::Network::IdSerializer::extractId` (IDA 0x960784).
    pub fn extract_id(&mut self, instance: Option<GuidData>) -> IdValue {
        match instance {
            Some(guid) => {
                self.reg(guid);
                // IDA 0x960784: copy `instance + 24` qword to `this + 4`, flag 1.
                IdValue { present: true, data: guid }
            }
            None => {
                // IDA 0x960784: flag 0, return 0.
                IdValue::default()
            }
        }
    }

    /// `RBX::Network::IdSerializer::sendId` (IDA 0x9607ac).
    pub fn send_id(&mut self, stream: &mut BitStream, id: &IdValue) {
        if id.present {
            let text = self.names.text(id.data.name).to_owned();
            // IDA 0x9607ac: `send(a1 + 92, ...)` + `WriteBits(id + 8, +1176)`.
            self.sender.send(stream, id.data.name as usize, &text);
            stream.write_bits(id.data.index, self.max_guid_index_bits as u8);
        } else {
            stream.write_u8(0);
        }
    }

    /// `RBX::Network::IdSerializer::serializeIdWithoutDictionary`
    /// (IDA 0x960814): full guid string (compressed `operator<<`, or
    /// `RakString::Serialize` under the flag) then the index bits; null
    /// writes the empty string only.
    ///
    /// FIDELITY: the `RakString` path keeps the same length-prefixed framing
    /// via [`BitStream::write_string`]; the RakString byte codec itself stays
    /// engine-side.
    pub fn serialize_id_without_dictionary(
        &mut self,
        stream: &mut BitStream,
        instance: Option<GuidData>,
    ) {
        match instance {
            Some(guid) => {
                self.reg(guid);
                let text = self.names.text(guid.name).to_owned();
                // IDA 0x960814: flag branch picks RakString vs `operator<<`.
                stream.write_string(&text);
                // IDA 0x960814: `WriteBits(&index, this + 294)`.
                stream.write_bits(guid.index, self.max_guid_index_bits as u8);
            }
            None => {
                stream.write_string("");
            }
        }
    }

    /// `RBX::Network::IdSerializer::deserializeId` (IDA 0x960a20).
    /// Panics mirror the original `std::runtime_error("BitStream >>
    /// RBX::Guid::Data failed")` throws on short index reads.
    pub fn deserialize_id(&mut self, stream: &mut BitStream) -> GuidData {
        // IDA 0x960a20: `ReadBits(a2, byte, 8, 1)`, return ignored.
        // BUG: original at 0x960a20 reads uninitialized stack when that
        // short-reads; 0 models the empty/default arm.
        let code = stream.read_u8().unwrap_or(0);
        let text = if code == 0 {
            // IDA 0x960a20: `setDefault`.
            let mut out = String::new();
            ReceiverDictionary::set_default(&mut out);
            out
        } else if code < 0x80 {
            // IDA 0x960a20: recall `*(this + 632 + 4 * byte)`.
            self.receiver.recall(code)
        } else {
            // IDA 0x960a20: `operator>>` then publish to `byte & 0x7F`.
            let text = stream.read_string();
            self.receiver.learn(code, &text);
            text
        };
        // IDA 0x960a20: `Name::declare`.
        let name = self.names.declare(&text);
        // IDA 0x960a20: index zeroed, then read unless null.
        let mut index = 0u32;
        if name != NULL_NAME && {
            match stream.read_bits(self.max_guid_index_bits as u8) {
                Some(v) => {
                    index = v;
                    false
                }
                None => true,
            }
        } {
            panic!("BitStream >> RBX::Guid::Data failed");
        }
        GuidData { name, index }
    }

    /// `RBX::Network::IdSerializer::deserializeIdWithoutDictionary`
    /// (IDA 0x960c8c): like [`IdSerializer::deserialize_id`] but the string
    /// always comes off the wire (no dictionary recall/store).
    pub fn deserialize_id_without_dictionary(&mut self, stream: &mut BitStream) -> GuidData {
        // IDA 0x960c8c: `RakString::Deserialize` vs `operator>>` per flag;
        // same framing either way (see `serialize_id_without_dictionary`).
        let text = stream.read_string();
        let name = self.names.declare(&text);
        let mut index = 0u32;
        if name != NULL_NAME && {
            match stream.read_bits(self.max_guid_index_bits as u8) {
                Some(v) => {
                    index = v;
                    false
                }
                None => true,
            }
        } {
            panic!("BitStream >> RBX::Guid::Data failed");
        }
        GuidData { name, index }
    }

    /// `RBX::Network::IdSerializer::setRefValue` (IDA 0x960f0c): forwards to
    /// the descriptor virtual at vtable +64. The virtual itself stays
    /// engine-side; this returns the `(descriptor, instance)` pair it would
    /// receive (the `+36` adjustments are engine pointer math).
    pub fn set_ref_value(item: &WaitItem, instance: Option<u32>) -> (u32, Option<u32>) {
        (item.descriptor, instance)
    }

    /// `RBX::Network::IdSerializer::resolvePendingReferences`
    /// (IDA 0x960f28): fires every waiter filed under `guid` (virtual at
    /// +124 stays engine-side, modeled by `on_resolve`) and erases the map
    /// node.
    pub fn resolve_pending_references(
        &mut self,
        instance: u32,
        guid: GuidData,
        mut on_resolve: impl FnMut(&WaitItem, u32),
    ) {
        // IDA 0x960f28: rb-tree `lower_bound`-style lookup, per-waiter
        // call, then `erase` + node delete.
        if let Some(waiters) = self.pending.remove(&guid) {
            for waiter in &waiters {
                on_resolve(waiter, instance);
            }
        }
    }

    /// `RBX::Network::IdSerializer::deserializeInstanceRef` (IDA 0x9610e0):
    /// `deserializeId` + `Registry::lookupByGuid`. `lookup` stands in for
    /// the registry; the `FLog::Asserts` same-`ServiceProvider` check stays
    /// engine-side.
    pub fn deserialize_instance_ref(
        &mut self,
        stream: &mut BitStream,
        lookup: &HashMap<GuidData, u32>,
    ) -> (Option<u32>, GuidData) {
        let guid = self.deserialize_id(stream);
        (lookup.get(&guid).copied(), guid)
    }

    /// `RBX::Network::IdSerializer::addPendingRef` (IDA 0x961178):
    /// `map[guid].push_back(WaitItem{descriptor, instance})`.
    pub fn add_pending_ref(&mut self, descriptor: u32, instance: u32, guid: GuidData) {
        self.pending.entry(guid).or_default().push(WaitItem { descriptor, instance });
    }

    /// Test hook: declare engine-side guid text so serialize paths can find it.
    pub fn declare_name(&mut self, text: &str) -> u32 {
        self.names.declare(text)
    }
}

/// Bit-length stored at sender +24 by the `DescriptorSender<T>` ctors
/// (IDA 0x962300 / 0x962464 / 0x962694 / 0x9628c4):
/// `for (i = size; i; i >>= 1) ++bits`.
pub fn index_bits(len: usize) -> u32 {
    (usize::BITS - len.leading_zeros()) as u32
}

/// `RBX::Network::DescriptorSender<T>` index (IDA 0x962300): each known
/// descriptor maps to its dense position.
#[derive(Clone, Debug, Default)]
pub struct DescriptorSender {
    index_of: HashMap<u32, u32>,
    bits: u32,
}

impl DescriptorSender {
    /// The four `DescriptorSender<T>` ctors build the same map over
    /// `allClasses` / `allDescriptors` / `getAllTypes`; `keys` are the
    /// engine-side descriptor ids in enumeration order.
    pub fn new(keys: &[u32]) -> Self {
        Self {
            index_of: keys.iter().enumerate().map(|(i, &k)| (k, i as u32)).collect(),
            bits: index_bits(keys.len()),
        }
    }

    pub fn index_of(&self, key: u32) -> Option<u32> {
        self.index_of.get(&key).copied()
    }

    /// Encoded index width (sender +24).
    pub fn bits(&self) -> u32 {
        self.bits
    }

    /// `DescriptorSender<PropertyDescriptor>::send` (IDA 0x9e013c, via the
    /// `this + 1304` sender): writes the dense index in `bits` bits.
    /// Unknown descriptors panic, mirroring the original's inability to
    /// encode them (engine-side always sends known ones).
    pub fn send_index(&self, stream: &mut crate::bitstream::BitStream, descriptor: u32) {
        let index = self
            .index_of(descriptor)
            .expect("DescriptorSender::send: unknown descriptor");
        stream.write_bits(index, self.bits as u8);
    }
}

/// `DescriptorSender<ClassDescriptor/EventDescriptor...>::teachName` for
/// plain names (IDA 0x961480 / 0x96208c): copies the descriptor name.
pub fn teach_name(name: &str) -> String {
    name.to_owned()
}

/// `DescriptorSender<EventDescriptor/PropertyDescriptor>::teachName`
/// (IDA 0x961700 / 0x961ca4): `Class + ":" + member`.
pub fn teach_qualified_name(class: &str, member: &str) -> String {
    format!("{class}:{member}")
}

/// `RBX::Network::DescriptorReceiver<T>` slot array: each `learnName`
/// stores the resolved descriptor id (or null) at `slots[index]`.
#[derive(Clone, Debug, Default)]
pub struct DescriptorReceiver {
    slots: Vec<Option<u32>>,
}

impl DescriptorReceiver {
    pub fn new(count: usize) -> Self {
        Self { slots: vec![None; count] }
    }

    pub fn get(&self, slot: usize) -> Option<u32> {
        self.slots.get(slot).copied().flatten()
    }

    /// `DescriptorReceiver<ClassDescriptor>::learnName` (IDA 0x961490):
    /// linear search of `allClasses` by name; miss logs
    /// `"ClassDescriptor failed to learn %s"` and stores null.
    pub fn learn_class(&mut self, slot: usize, name: &str, classes: &[(String, u32)]) {
        let found = classes.iter().find(|(n, _)| n == name).map(|(_, id)| *id);
        if found.is_none() {
            // IDA 0x961490: `StandardOut::printf(2, "ClassDescriptor failed to learn %s", ...)`.
            log_learn_miss("ClassDescriptor", name);
        }
        if let Some(slot) = self.slots.get_mut(slot) {
            *slot = found;
        }
    }

    /// `DescriptorReceiver<EventDescriptor>::learnName` (IDA 0x9618c4):
    /// splits `"Class:Event"`, finds the class, then the event in its map;
    /// miss logs `"EventDescriptor failed to learn %s"` and stores null.
    pub fn learn_event(
        &mut self,
        slot: usize,
        qualified: &str,
        classes: &[(String, u32)],
        events: &[(u32, String, u32)],
    ) {
        let found = learn_member(qualified, classes, events);
        if found.is_none() {
            // IDA 0x9618c4: `StandardOut::printf(2, "EventDescriptor failed to learn %s", ...)`.
            log_learn_miss("EventDescriptor", qualified);
        }
        if let Some(slot) = self.slots.get_mut(slot) {
            *slot = found;
        }
    }

    /// `DescriptorReceiver<PropertyDescriptor>::learnName` (IDA 0x961e68):
    /// same shape as `learn_event`, but a miss stores null silently — the
    /// original has no `printf` on this path.
    pub fn learn_property(
        &mut self,
        slot: usize,
        qualified: &str,
        classes: &[(String, u32)],
        properties: &[(u32, String, u32)],
    ) {
        // IDA 0x961e68: no log call; stores 0 on miss.
        let found = learn_member(qualified, classes, properties);
        if let Some(slot) = self.slots.get_mut(slot) {
            *slot = found;
        }
    }

    /// `DescriptorReceiver<Type>::learnName` (IDA 0x96209c): linear search
    /// of `getAllTypes` by name; miss logs `"Type failed to learn %s"` and
    /// stores null.
    pub fn learn_type(&mut self, slot: usize, name: &str, types: &[(String, u32)]) {
        let found = types.iter().find(|(n, _)| n == name).map(|(_, id)| *id);
        if found.is_none() {
            // IDA 0x96209c: `StandardOut::printf(2, "Type failed to learn %s", ...)`.
            log_learn_miss("Type", name);
        }
        if let Some(slot) = self.slots.get_mut(slot) {
            *slot = found;
        }
    }
}

/// Shared `"Class:Member"` resolution for the Event/Property receivers
/// (IDA 0x9618c4 / 0x961e68: `boost::split` on `':'`, class lookup over
/// `allClasses`, member lookup in the class map).
fn learn_member(
    qualified: &str,
    classes: &[(String, u32)],
    members: &[(u32, String, u32)],
) -> Option<u32> {
    let mut parts = qualified.split(':');
    let (class, member) = match (parts.next(), parts.next(), parts.next()) {
        (Some(c), Some(m), None) => (c, m),
        _ => return None,
    };
    let class_id = classes.iter().find(|(n, _)| n == class).map(|(_, id)| *id)?;
    members
        .iter()
        .find(|(c, m, _)| *c == class_id && m == member)
        .map(|(_, _, id)| *id)
}

/// `RBX::StandardOut::printf(2, "<Kind> failed to learn %s")` (IDA 0x961490
/// / 0x9618c4 / 0x96209c) reduced to `eprintln!`; the log level `2` is a
/// warning channel.
fn log_learn_miss(kind: &str, name: &str) {
    eprintln!("{kind} failed to learn {name}");
}

/// `RBX::Network::deserialize<ContentId>` (IDA 0x96025c): `operator>>` the
/// string, then the property virtual at vtable +44 (engine-side).
/// Returns the wire string; the caller applies it.
pub fn deserialize_content_id(stream: &mut BitStream) -> String {
    stream.read_string()
}

/// `RBX::Network::deserializeStringProperty` (IDA 0x960380): byte-identical
/// body to [`deserialize_content_id`] — same `operator>>` + vtable +44 call.
pub fn deserialize_string_property(stream: &mut BitStream) -> String {
    stream.read_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    fn serializer() -> IdSerializer {
        let mut s = IdSerializer::new();
        s.set_max_guid_index_bit(32);
        s
    }

    fn guid_with(s: &mut IdSerializer, text: &str, index: u32) -> GuidData {
        let name = s.declare_name(text);
        GuidData { name, index }
    }

    #[test]
    fn max_guid_index_bit_threshold() {
        let mut s = IdSerializer::new();
        // IDA 0x960624: `bits - 1 < 0xE -> 24 else 32`.
        s.set_max_guid_index_bit(14);
        assert_eq!(s.max_guid_index_bits(), 24);
        s.set_max_guid_index_bit(15);
        assert_eq!(s.max_guid_index_bits(), 32);
        s.set_max_guid_index_bit(0);
        assert_eq!(s.max_guid_index_bits(), 32);
    }

    #[test]
    fn serialize_deserialize_id_roundtrip() {
        let mut s = serializer();
        let guid = guid_with(&mut s, "Workspace", 0x1234);
        let mut stream = BitStream::new();
        s.serialize_id(&mut stream, Some(guid));
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(s.deserialize_id(&mut r), guid);
    }

    #[test]
    fn null_id_is_one_zero_byte() {
        let mut s = serializer();
        let mut stream = BitStream::new();
        s.serialize_id(&mut stream, None);
        assert_eq!(stream.into_bytes(), vec![0]);
        let mut r = BitStream::from_bytes(&[0]);
        let back = s.deserialize_id(&mut r);
        assert_eq!(back, GuidData { name: NULL_NAME, index: 0 });
    }
    #[test]
    fn try_serialize_unknown_is_false_without_writing() {
        let mut s = serializer();
        // Fresh name the sender has never seen: trySend fails (IDA 0x965f98).
        let guid = guid_with(&mut s, "NeverSent", 7);
        let mut stream = BitStream::new();
        assert!(!s.try_serialize_id(&mut stream, Some(guid)));
        assert_eq!(stream.bits_written(), 0);
        // Filler occupies slot 0 (recall code 0 reads as empty per the
        // sender quirk); both fresh strings go on the wire so the receiver
        // side learns its slots, exactly like a real peer.
        let filler = guid_with(&mut s, "Filler", 0);
        let mut wire = BitStream::new();
        s.serialize_id(&mut wire, Some(filler));
        s.serialize_id(&mut wire, Some(guid));
        let mut r = BitStream::from_bytes(&wire.into_bytes());
        assert_eq!(s.deserialize_id(&mut r), filler);
        assert_eq!(s.deserialize_id(&mut r), guid);
        // Now the recall code resolves on the receiving side too.
        let mut stream2 = BitStream::new();
        assert!(s.try_serialize_id(&mut stream2, Some(guid)));
        let mut r2 = BitStream::from_bytes(&stream2.into_bytes());
        assert_eq!(s.deserialize_id(&mut r2), guid);
    }

    #[test]
    fn can_serialize_registers() {
        let mut s = serializer();
        assert!(!s.can_serialize_id(None));
        let guid = guid_with(&mut s, "Players", 1);
        assert!(s.can_serialize_id(Some(guid)));
    }
    #[test]
    fn extract_and_send_id_roundtrip() {
        let mut s = serializer();
        let guid = guid_with(&mut s, "Lighting", 9);
        let id = s.extract_id(Some(guid));
        assert!(id.present);
        assert_eq!(id.data, guid);
        assert!(!s.extract_id(None).present);
        let mut stream = BitStream::new();
        s.send_id(&mut stream, &id);
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(s.deserialize_id(&mut r), guid);
    }

    #[test]
    fn without_dictionary_roundtrip() {
        let mut s = serializer();
        let guid = guid_with(&mut s, "StarterPack", 0xAB);
        let mut stream = BitStream::new();
        s.serialize_id_without_dictionary(&mut stream, Some(guid));
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(s.deserialize_id_without_dictionary(&mut r), guid);
        // Null writes the empty string; reads back as null with index 0.
        let mut stream = BitStream::new();
        s.serialize_id_without_dictionary(&mut stream, None);
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(s.deserialize_id_without_dictionary(&mut r), GuidData::default());
    }

    #[test]
    #[should_panic(expected = "BitStream >> RBX::Guid::Data failed")]
    fn short_index_read_panics() {
        let mut s = serializer();
        // Code byte recalls nothing useful; name declares non-null, then
        // the 32 index bits are missing (IDA 0x960a20 throw).
        s.declare_name("Workspace");
        let mut stream = BitStream::new();
        stream.write_u8(0x80);
        stream.write_string("Workspace");
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        let _ = s.deserialize_id(&mut r);
    }

    #[test]
    fn pending_refs_resolve_and_erase() {
        let mut s = serializer();
        let guid = guid_with(&mut s, "Part", 3);
        s.add_pending_ref(11, 100, guid);
        s.add_pending_ref(12, 101, guid);
        let mut fired = Vec::new();
        s.resolve_pending_references(999, guid, |w, inst| fired.push((w.descriptor, w.instance, inst)));
        assert_eq!(fired, vec![(11, 100, 999), (12, 101, 999)]);
        // Second resolve fires nothing: the node was erased (IDA 0x960f28).
        s.resolve_pending_references(999, guid, |_, _| panic!("erased node refired"));
    }

    #[test]
    fn instance_ref_lookup_miss_gives_none() {
        let mut s = serializer();
        let guid = guid_with(&mut s, "Ghost", 5);
        let mut stream = BitStream::new();
        s.serialize_id(&mut stream, Some(guid));
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        let (instance, back) = s.deserialize_instance_ref(&mut r, &HashMap::new());
        assert_eq!(instance, None);
        assert_eq!(back, guid);
    }

    #[test]
    fn descriptor_tables_teach_and_learn() {
        assert_eq!(teach_name("Player"), "Player");
        assert_eq!(teach_qualified_name("Player", "Name"), "Player:Name");
        // Bit widths match the IDA tree-height loop.
        assert_eq!(index_bits(0), 0);
        assert_eq!(index_bits(1), 1);
        assert_eq!(index_bits(5), 3);
        let sender = DescriptorSender::new(&[10, 20, 30]);
        assert_eq!(sender.index_of(20), Some(1));
        assert_eq!(sender.index_of(99), None);
        assert_eq!(sender.bits(), 2);

        let classes = vec![("Player".to_owned(), 1u32)];
        let events = vec![(1u32, "Chatted".to_owned(), 7u32)];
        let mut rx = DescriptorReceiver::new(4);
        rx.learn_class(0, "Player", &classes);
        assert_eq!(rx.get(0), Some(1));
        rx.learn_class(1, "Nope", &classes);
        assert_eq!(rx.get(1), None);
        rx.learn_event(2, "Player:Chatted", &classes, &events);
        assert_eq!(rx.get(2), Some(7));
        // Property miss is silent but still stores null (IDA 0x961e68).
        rx.learn_property(3, "Player:Missing", &classes, &[]);
        assert_eq!(rx.get(3), None);
        rx.learn_type(3, "int", &[("int".to_owned(), 42)]);
        assert_eq!(rx.get(3), Some(42));
    }

    #[test]
    fn content_and_string_property_read_wire_string() {
        let mut stream = BitStream::new();
        stream.write_string("rbxasset://x");
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(deserialize_content_id(&mut r), "rbxasset://x");
        let mut stream = BitStream::new();
        stream.write_string("hi");
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(deserialize_string_property(&mut r), "hi");
    }
}
