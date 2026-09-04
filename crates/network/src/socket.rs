//! RakNet runtime objects: `PluginInterface2`, `RakNetSocket`, the
//! `_RakMalloc_Ex` family, and `StatisticsToString`.
//!
//! Decompiled from the plugin ctor/dtor/set (IDA 0xa5a2ac/0xa5a2c4/
//! 0xa5a2d0/0xa5a2d4/0xa5a2d8), the allocator wrappers (IDA
//! 0xa5a900/0xa5a90c/0xa5a918), the socket ctor/dtor (IDA
//! 0xa5af38/0xa5af50), and `StatisticsToString` (IDA 0xa5b5b0). File
//! descriptors, vtables, and the full statistics struct stay engine-side.

#![allow(dead_code)]

/// `RakNet::PluginInterface2` reduced to its peer handle (+4).
#[derive(Clone, Copy, Debug, Default)]
pub struct PluginInterface2 {
    pub rak_peer: Option<u32>,
}

impl PluginInterface2 {
    /// `PluginInterface2::PluginInterface2` (IDA 0xa5a2ac).
    pub fn new() -> Self {
        Self::default()
    }

    /// `PluginInterface2::SetRakPeerInterface` (IDA 0xa5a2d8):
    /// `*(this + 4) = peer`.
    pub fn set_rak_peer_interface(&mut self, peer: Option<u32>) {
        self.rak_peer = peer;
    }
}

/// `RakNet::_RakMalloc_Ex` (IDA 0xa5a900): zeroed bytes; the file/line
/// tracking stays engine-side.
#[must_use]
pub fn rak_malloc(size: usize) -> Vec<u8> {
    vec![0u8; size]
}

/// `RakNet::_RakRealloc_Ex` (IDA 0xa5a90c): grown tails zero-fill here
/// where the original leaves them uninitialized.
#[must_use]
pub fn rak_realloc(buf: Vec<u8>, size: usize) -> Vec<u8> {
    let mut buf = buf;
    buf.resize(size, 0);
    buf
}

/// `RakNet::_RakFree_Ex` (IDA 0xa5a918).
pub fn rak_free(_buf: Vec<u8>) {}
/// `RakNet::RakNetSocket::RakNetSocket` (IDA 0xa5af38): descriptor and
/// buffer init stays engine-side.
pub fn init_raknet_socket() {}
/// `RakNet::RakNetSocket::~RakNetSocket` (IDA 0xa5af50): closing the
/// descriptor stays engine-side.
pub fn free_raknet_socket() {}

/// `RakNet::StatisticsToString` null arm (IDA 0xa5b5e8).
#[must_use]
pub fn statistics_null_text() -> &'static str {
    "stats is a NULL pointer in statsToString\n"
}

/// `RakNet::StatisticsToString` brief arm (IDA 0xa5b72a): three lines
/// over bytes-sent/received per second and current packetloss.
#[must_use]
pub fn statistics_brief_text(sent_per_sec: u64, received_per_sec: u64, packetloss: f32) -> String {
    format!(
        "Bytes per second sent     {sent_per_sec}\nBytes per second received {received_per_sec}\nCurrent packetloss        {packetloss:.1}%\n"
    )
}

/// `RakNet::StatisticsToString` (IDA 0xa5b5b0): null stats write the
/// null text, verbosity 0 writes the brief text, and higher verbosity
/// formats the full struct engine-side.
pub fn statistics_to_string(
    present: bool,
    verbose: u32,
    sent_per_sec: u64,
    received_per_sec: u64,
    packetloss: f32,
    full: &mut dyn FnMut() -> String,
) -> String {
    if !present {
        return statistics_null_text().to_owned();
    }
    if verbose == 0 {
        return statistics_brief_text(sent_per_sec, received_per_sec, packetloss);
    }
    full()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_objects() {
        // IDA 0xa5a2ac/0xa5a2d8: plugin handle store.
        let mut plugin = PluginInterface2::new();
        assert_eq!(plugin.rak_peer, None);
        plugin.set_rak_peer_interface(Some(9));
        assert_eq!(plugin.rak_peer, Some(9));
        // IDA 0xa5a900/0xa5a90c/0xa5a918: allocator roundtrip.
        let buf = rak_malloc(4);
        assert_eq!(buf, vec![0, 0, 0, 0]);
        let buf = rak_realloc(buf, 6);
        assert_eq!(buf, vec![0, 0, 0, 0, 0, 0]);
        rak_free(buf);
        init_raknet_socket();
        free_raknet_socket();
        // IDA 0xa5b5b0: null/brief/full arms.
        assert_eq!(
            statistics_to_string(false, 0, 0, 0, 0.0, &mut || unreachable!()),
            "stats is a NULL pointer in statsToString\n"
        );
        assert_eq!(
            statistics_to_string(true, 0, 10, 20, 1.5, &mut || unreachable!()),
            "Bytes per second sent     10\nBytes per second received 20\nCurrent packetloss        1.5%\n"
        );
        assert_eq!(
            statistics_to_string(true, 2, 0, 0, 0.0, &mut || "full".to_owned()),
            "full"
        );
    }
    #[test]
    fn address_and_guid_gates() {
        // IDA 0xa5bfec/0xa5c024: default ctor plus assign.
        let mut a = SystemAddress::new();
        assert_eq!((a.family, a.port(), a.binary_address()), (2, 0, 0));
        let mut b = SystemAddress::new();
        b.binary = 0x7F00_0001;
        b.set_port_network_order(0x901F);
        assert_eq!(b.port(), 0x1F90);
        assert_eq!(b.port_network_order(), 0x901F);
        a.assign(&b);
        assert!(a.equals(&b) && !a.not_equals(&b));
        // IDA 0xa5c108 quirk: the receiver's family gates.
        let mut c = b;
        c.family = 6;
        assert!(!c.equals(&b));
        assert!(c.not_equals(&b));
        // IDA 0xa5c0d8/0xa5c154/0xa5c160.
        assert!(b.equals_excluding_port(&a));
        // IDA 0xa5c04c/0xa7cb08: hash is stable and sensitive to the
        // hashed bytes (port/binary, not family).
        assert_eq!(b.to_integer(), b.to_integer());
        let mut d = b;
        d.binary ^= 1;
        assert_ne!(b.to_integer(), d.to_integer());
        assert_eq!(super_fast_hash_incremental(&[], 99), 0);
        // IDA 0xa5c004/0xa5c038: guid default plus guid-only inequality.
        let g = RakNetGuid::new();
        assert_eq!((g.g, g.system_index), (UNASSIGNED_RAKNET_GUID, UNASSIGNED_SYSTEM_INDEX));
        let h = RakNetGuid { g: 7, system_index: UNASSIGNED_SYSTEM_INDEX };
        assert!(g.not_equal(&h) && g != h);
        assert!(!g.not_equal(&g));
    }
    #[test]
    fn address_string_cluster() {
        // IDA 0xa5c320: dotted, localhost, DNS, and port suffixes.
        let mut no_dns = |_: &str| None;
        let mut a = SystemAddress::new();
        a.set_binary_address("10.0.0.5:1234", &mut no_dns);
        assert_eq!((a.dotted(), a.port, a.debug_port), ("10.0.0.5".to_owned(), 1234, 1234));
        a.set_binary_address("localhost8080", &mut no_dns);
        assert_eq!((a.dotted(), a.port), ("127.0.0.1".to_owned(), 8080));
        a.set_binary_address("example.com", &mut |h| (h == "example.com").then_some(0x0100_007Fu32));
        assert_eq!(a.dotted(), "127.0.0.1");
        // IDA 0xa5c220/0xa5c250/0xa5c498/0xa5c164/0xa5c274.
        let mut no_dns = |_: &str| None;
        let b = SystemAddress::from_host_port("192.168.1.2", 53640, &mut no_dns);
        assert_eq!((b.dotted(), b.port, b.system_index), ("192.168.1.2".to_owned(), 53640, 0xFFFF));
        let c = SystemAddress::from_string_explicit_port("192.168.1.2", 80, &mut no_dns);
        assert_eq!((c.port, c.system_index), (80, 0));
        let mut d = SystemAddress::new();
        d.copy_port(&b);
        assert_eq!((d.port, d.debug_port), (53640, 53640));
        d.set_to_loopback(true);
        assert_eq!(d.dotted(), "127.0.0.1");
        d.set_to_loopback(false);
        assert_eq!(d.family, 6);
        d.fix_for_ip_version(&SystemAddress::new());
        assert_eq!((d.family, d.dotted()), (2, "127.0.0.1".to_owned()));
        // IDA 0xa5c18c/0xa5c068: sentinel plus dotted forms.
        let mut u = SystemAddress::new();
        u.port = 0xFFFF;
        u.binary = 0xFFFF_FFFF;
        assert_eq!(u.to_string_old(true, ':'), "UNASSIGNED_SYSTEM_ADDRESS");
        assert_eq!(b.to_string_old(true, ':'), format!("192.168.1.2:53640"));
        assert_eq!(b.to_string_old(false, ':'), "192.168.1.2");
        assert_eq!(b.address_to_string(true), "192.168.1.2:53640");
        // IDA 0xa5c0b0: socket descriptor.
        assert_eq!(socket_descriptor(123, Some("h")).host, "h");
        assert_eq!(socket_descriptor(123, None).host, "");
    }
}

/// `RakNet::UNASSIGNED_RAKNET_GUID` (IDA 0xa5c018).
pub const UNASSIGNED_RAKNET_GUID: u64 = 0xFFFF_FFFF_FFFF_FFFF;
/// `UNASSIGNED_SYSTEM_INDEX` backing `word_137FBB8` (IDA 0xa5c01e).
pub const UNASSIGNED_SYSTEM_INDEX: u16 = 0xFFFF;

/// `RakNet::RakNetGUID` (IDA 0xa5c004): the 64-bit guid plus the system
/// index word at +8. Equality covers the guid only (IDA 0xa5c038 loads
/// two dwords and ORs the XORs).
#[derive(Clone, Copy, Debug, Default)]
pub struct RakNetGuid {
 pub g: u64,
 pub system_index: u16,
}

impl PartialEq for RakNetGuid {
 fn eq(&self, other: &Self) -> bool {
 self.g == other.g
 }
}

impl Eq for RakNetGuid {}

impl RakNetGuid {
 /// `RakNetGUID::RakNetGUID` (IDA 0xa5c004).
 pub fn new() -> Self {
 Self { g: UNASSIGNED_RAKNET_GUID, system_index: UNASSIGNED_SYSTEM_INDEX }
 }

 /// `RakNetGUID::operator!=` (IDA 0xa5c038).
 #[must_use]
 pub fn not_equal(&self, other: &Self) -> bool {
 self.g != other.g
 }
}

/// `RakNet::SystemAddress` (IDA 0xa5bfec): sockaddr family byte at +1
/// (2 = IPv4), the port in network order at +2, and the binary address
/// at +4. Ports are kept in host order here; the +2 image is big-endian.
#[derive(Clone, Copy, Debug, Hash)]
pub struct SystemAddress {
 pub family: u8,
 pub port: u16,
 pub binary: u32,
 /// Debug port image at +16 (IDA 0xa5c414/0xa5c26e): tracks the port.
 pub debug_port: u16,
 /// Word at +18 (IDA 0xa5c24a): set to all-ones by the host/port ctor.
 pub system_index: u16,
}

impl Default for SystemAddress {
 fn default() -> Self {
 Self::new()
 }
}

impl PartialEq for SystemAddress {
 fn eq(&self, other: &Self) -> bool {
 self.equals(other)
 }
}

impl Eq for SystemAddress {}

impl SystemAddress {
 /// `SystemAddress::SystemAddress` (IDA 0xa5bfec): zeroed storage
 /// with the IPv4 family byte.
 pub fn new() -> Self {
 Self { family: 2, port: 0, binary: 0, debug_port: 0, system_index: 0 }
 }

 /// `SystemAddress::operator=` (IDA 0xa5c024).
 pub fn assign(&mut self, other: &Self) {
 *self = *other;
 }

 /// `SystemAddress::operator==` (IDA 0xa5c108): ports equal, then the
 /// receiver's family must be IPv4 and the addresses equal. Note the
 /// asymmetry: only the left side's family is checked.
 #[must_use]
 pub fn equals(&self, other: &Self) -> bool {
 if self.port != other.port {
 return false;
 }
 if self.family != 2 {
 return false;
 }
 self.binary == other.binary
 }

 /// `SystemAddress::operator!=` (IDA 0xa5c12c).
 #[must_use]
 pub fn not_equals(&self, other: &Self) -> bool {
 !self.equals(other)
 }

 /// `SystemAddress::EqualsExcludingPort` (IDA 0xa5c0d8).
 #[must_use]
 pub fn equals_excluding_port(&self, other: &Self) -> bool {
 if self.family != 2 {
 return false;
 }
 self.binary == other.binary
 }

 /// `SystemAddress::ToInteger` (IDA 0xa5c04c): SuperFastHash over the
 /// raw port bytes seeded with 2, then over the raw address bytes.
 #[must_use]
 pub fn to_integer(&self) -> u32 {
 let port = super_fast_hash_incremental(&self.port.to_be_bytes(), 2);
 super_fast_hash_incremental(&self.binary.to_le_bytes(), port)
 }

 /// `SystemAddress::GetBinaryAddress` (IDA 0xa5c0f0).
 #[must_use]
 pub fn binary_address(&self) -> u32 {
 self.binary
 }

 /// `SystemAddress::GetPort` (IDA 0xa5c0f4): network-to-host swap.
 #[must_use]
 pub fn port(&self) -> u16 {
 self.port
 }
 /// `SystemAddress::GetPortNetworkOrder` (IDA 0xa5c0fc): the raw +2
 /// image.
 #[must_use]
 pub fn port_network_order(&self) -> u16 {
 self.port.swap_bytes()
 }

 /// `SystemAddress::SetPortNetworkOrder` (IDA 0xa5c100).
 pub fn set_port_network_order(&mut self, port: u16) {
 self.port = port.swap_bytes();
 }

 /// `SystemAddress::GetIPVersion` (IDA 0xa5c154): 4 for IPv4, else 6.
 #[must_use]
 pub fn ip_version(&self) -> u32 {
 if self.family == 2 {
 4
 } else {
 6
 }
 }

 /// `SystemAddress::GetIPPROTO` (IDA 0xa5c160): always 0.
 #[must_use]
 pub fn ip_proto(&self) -> u32 {
 0
 }

 /// Dotted quad of the raw address (IDA 0xa5c1ea `inet_ntoa`).
 #[must_use]
 pub fn dotted(&self) -> String {
 let o = self.binary.to_le_bytes();
 format!("{}.{}.{}.{}", o[0], o[1], o[2], o[3])
 }

 /// Whether this is the unassigned sentinel (IDA 0xa5c1a8..0xa5c1b8:
 /// IPv4 plus the `word_137FC02` port and `dword_137FC04` address,
 /// all-ones in practice).
 #[must_use]
 pub fn is_unassigned(&self) -> bool {
 self.family == 2 && self.port == 0xFFFF && self.binary == 0xFFFF_FFFF
 }

 /// `SystemAddress::ToString_Old` (IDA 0xa5c18c): the sentinel name,
 /// else dotted plus the delimiter and decimal port when asked. The
 /// original returns a pointer 10 past the buffer on the sentinel arm;
 /// only the buffer text is modeled.
 #[must_use]
 pub fn to_string_old(&self, print_port: bool, delimiter: char) -> String {
 if self.is_unassigned() {
 return "UNASSIGNED_SYSTEM_ADDRESS".to_owned();
 }
 if print_port {
 format!("{}{}{}", self.dotted(), delimiter, self.port)
 } else {
 self.dotted()
 }
 }

 /// `SystemAddress::ToString` (IDA 0xa5c068/0xa5c0a4): same text via a
 /// rotating static buffer engine-side; this returns an owned copy.
 #[must_use]
 pub fn address_to_string(&self, print_port: bool) -> String {
 self.to_string_old(print_port, ':')
 }

 /// Parse a dotted quad like `inet_addr` (IDA 0xa5c3fc).
 fn parse_dotted(text: &str) -> Option<u32> {
 let mut octets = [0u8; 4];
 let mut parts = text.split('.');
 for o in &mut octets {
 *o = parts.next()?.parse::<u8>().ok()?;
 }
 if parts.next().is_some() {
 return None;
 }
 Some(u32::from_le_bytes(octets))
 }

 /// `SystemAddress::SetBinaryAddress` (IDA 0xa5c320): dotted
 /// `[host][:port]`, `localhost`, or DNS via `resolve`. The port sticks
 /// only when the address parses, like the original.
 pub fn set_binary_address(&mut self, host: &str, resolve: &mut dyn FnMut(&str) -> Option<u32>) {
 let starts_numeric = host.as_bytes().first().is_some_and(|&c| c.is_ascii_digit() || c == b'-');
 if starts_numeric || host.contains(':') {
 let (ip, port) = match host.split_once(':') {
 Some((ip, port)) => (ip, port.parse::<u16>().ok()),
 None => (host, None),
 };
 if let Some(binary) = Self::parse_dotted(ip) {
 self.binary = binary;
 if let Some(port) = port {
 self.port = port;
 self.debug_port = port;
 }
 }
 } else if let Some(rest) = host.strip_prefix("localhost") {
 self.binary = u32::from_le_bytes([127, 0, 0, 1]);
 if let Ok(port) = rest.parse::<u16>() {
 self.port = port;
 self.debug_port = port;
 }
 } else if let Some(binary) = resolve(host) {
 self.binary = binary;
 }
 }

 /// `SystemAddress::SystemAddress(host, port)` (IDA 0xa5c220): family
 /// plus address, the port images, and the all-ones word.
 pub fn from_host_port(host: &str, port: u16, resolve: &mut dyn FnMut(&str) -> Option<u32>) -> Self {
 let mut addr = Self::new();
 addr.set_binary_address(host, resolve);
 addr.port = port;
 addr.debug_port = port;
 addr.system_index = 0xFFFF;
 addr
 }

 /// `SystemAddress::FromStringExplicitPort` (IDA 0xa5c250): same
 /// without the trailing word.
 pub fn from_string_explicit_port(host: &str, port: u16, resolve: &mut dyn FnMut(&str) -> Option<u32>) -> Self {
 let mut addr = Self::new();
 addr.set_binary_address(host, resolve);
 addr.port = port;
 addr.debug_port = port;
 addr
 }

 /// `SystemAddress::SetToLoopback` (IDA 0xa5c164): `127.0.0.1` for
 /// IPv4; IPv6 loopback only flips the family (the 16-byte address
 /// stays engine-side).
 pub fn set_to_loopback(&mut self, ipv4: bool) {
 if ipv4 {
 self.family = 2;
 self.binary = u32::from_le_bytes([127, 0, 0, 1]);
 } else {
 self.family = 6;
 }
 }

 /// `SystemAddress::CopyPort` (IDA 0xa5c498): the port and debug
 /// images.
 pub fn copy_port(&mut self, other: &Self) {
 self.port = other.port;
 self.debug_port = other.debug_port;
 }

 /// `SystemAddress::FixForIPVersion` (IDA 0xa5c274): an IPv6 loopback
 /// talking to IPv4 becomes `127.0.0.1`.
 pub fn fix_for_ip_version(&mut self, other: &Self) {
 if self.family != 2 && other.family == 2 {
 self.set_to_loopback(true);
 }
 }
}

/// `RakNet::SocketDescriptor` (IDA 0xa5c0b0): the port plus an optional
/// host string; the remaining words stay engine-side.
#[derive(Clone, Debug, Default)]
pub struct SocketDescriptor {
 pub port: u16,
 pub host: String,
}

/// `SocketDescriptor::SocketDescriptor` (IDA 0xa5c0b0).
#[must_use]
pub fn socket_descriptor(port: u16, host: Option<&str>) -> SocketDescriptor {
 SocketDescriptor { port, host: host.unwrap_or("").to_owned() }
}

/// `SuperFastHashIncremental` (IDA 0xa7cb08, Paul Hsieh's hash as
/// embedded in RakNet): little-endian halfword loop plus avalanche.
/// Empty input returns 0 without touching the seed.
#[must_use]
pub fn super_fast_hash_incremental(data: &[u8], hash: u32) -> u32 {
 if data.is_empty() {
 return 0;
 }
 let mut hash = hash;
 let mut chunks = data.chunks_exact(4);
 for c in &mut chunks {
 let w1 = u16::from_le_bytes([c[0], c[1]]) as u32;
 let w2 = u16::from_le_bytes([c[2], c[3]]) as u32;
 let t = hash.wrapping_add(w1);
 let t = t ^ t.wrapping_shl(16) ^ w2.wrapping_shl(11);
 hash = t.wrapping_add(t >> 11);
 }
 match chunks.remainder() {
 [b0] => {
 let t = (*b0 as u32).wrapping_add(hash);
 let t = t ^ t.wrapping_shl(10);
 hash = t.wrapping_add(t >> 1);
 }
 [b0, b1] => {
 let t = u16::from_le_bytes([*b0, *b1]) as u32;
 let t = t.wrapping_add(hash);
 let t = t ^ t.wrapping_shl(11);
 hash = t.wrapping_add(t >> 17);
 }
 [b0, b1, b2] => {
 let t = u16::from_le_bytes([*b0, *b1]) as u32;
 let t = t.wrapping_add(hash);
 let t = t ^ ((*b2 as u32) << 18) ^ t.wrapping_shl(16);
 hash = t.wrapping_add(t >> 11);
 }
 _ => {}
 }
 let t = hash ^ hash.wrapping_shl(3);
 let hash = t.wrapping_add(t >> 5);
 let t = hash ^ hash.wrapping_shl(4);
 let hash = t.wrapping_add(t >> 17);
 let t = hash ^ hash.wrapping_shl(25);
 t.wrapping_add(t >> 6)
}
