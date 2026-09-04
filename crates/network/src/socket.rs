//! RakNet runtime objects: `PluginInterface2`, `RakNetSocket`, the
//! `_RakMalloc_Ex` family, and `StatisticsToString`.
//!
//! Decompiled from the plugin ctor/dtor/set (IDA 0xa5a2ac/0xa5a2c4/
//! 0xa5a2d0/0xa5a2d4/0xa5a2d8), the allocator wrappers (IDA
//! 0xa5a900/0xa5a90c/0xa5a918), the socket ctor/dtor (IDA
//! 0xa5af38/0xa5af50), and `StatisticsToString` (IDA 0xa5b5b0). File
//! descriptors, vtables, and the full statistics struct stay engine-side.

#![allow(dead_code)]

use super::bitstream::BitStream;

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
    /// `OnRakPeerStartup` (IDA 0xad5300): default hook is empty.
    pub fn on_rak_peer_startup(&self) {}
    /// `OnClosedConnection` (IDA 0xad5308): default hook is empty.
    pub fn on_closed_connection(&self) {}
    /// `OnFailedConnectionAttempt` (IDA 0xad5310): default hook is empty.
    pub fn on_failed_connection_attempt(&self) {}
    /// `UsesReliabilityLayer` (IDA 0xad5314): default returns false
    /// (`MOVS R0, #0`).
    #[must_use]
    pub fn uses_reliability_layer(&self) -> bool {
        false
    }
    /// `OnDirectSocketSend` (IDA 0xad5318): default hook is empty.
    pub fn on_direct_socket_send(&self) {}
    /// `OnReliabilityLayerPacketError` (IDA 0xad5320): default hook is empty.
    pub fn on_reliability_layer_packet_error(&self) {}
    /// `OnInternalPacket` (IDA 0xad5324): default hook is a bare `BX LR`.
    pub fn on_internal_packet(&self) {}
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
    #[test]
    fn rak_peer_lifecycle_codes() {
        // IDA 0xa5c4a4: guid equality is the negated inequality.
        let g = RakNetGuid::new();
        let h = RakNetGuid { g: 7, system_index: 0 };
        assert!(!(g.g == h.g && !g.not_equal(&h)));
        assert!(!g.not_equal(&g));
        // IDA 0xa5cb00/0xa5e3c0: ctor plus startup codes.
        let mut peer = RakPeer::new();
        assert_eq!(peer.startup(false, true, false, true, true), 0);
        assert_eq!(peer.startup(true, true, false, true, true), 1);
        assert_eq!(peer.startup(false, false, false, true, true), 2);
        assert_eq!(peer.startup(false, true, true, true, true), 5);
        assert_eq!(peer.startup(false, true, false, false, true), 6);
        assert_eq!(peer.startup(false, true, false, true, false), 9);
        // IDA 0xa5eab8/0xa5ebd4/0xa5eca0/0xa5ed50/0xa5ee80: no-ops.
        RakPeer::deref_all_sockets();
        RakPeer::clear_buffered_commands();
        RakPeer::clear_buffered_packets();
        RakPeer::update_network_loop();
        RakPeer::recv_from_loop();
    }
    #[test]
    fn security_and_password_gates() {
        // IDA 0xa5efa0/0xa5efa4: hardcoded and empty.
        let mut peer = RakPeer::new();
        assert_eq!(peer.initialize_security(), 0);
        peer.disable_security();
        // IDA 0xa5efa8/0xa5f08c/0xa5f230/0xa6f1ac: pattern list.
        assert!(!peer.is_in_security_exception_list("10.0.0.1"));
        peer.add_to_security_exception_list("10.0.0.*");
        peer.add_to_security_exception_list("192.168.1.7");
        assert!(peer.is_in_security_exception_list("10.0.0.1"));
        assert!(peer.is_in_security_exception_list("192.168.1.7"));
        assert!(!peer.is_in_security_exception_list("192.168.1.8"));
        assert!(!peer.is_in_security_exception_list(""));
        assert!(!peer.is_in_security_exception_list("12345678901234567"));
        peer.remove_from_security_exception_list(Some("10.0.0.9"));
        assert!(!peer.is_in_security_exception_list("10.0.0.1"));
        assert!(peer.is_in_security_exception_list("192.168.1.7"));
        peer.remove_from_security_exception_list(None);
        assert!(!peer.is_in_security_exception_list("192.168.1.7"));
        // IDA 0xa5f28c/0xa5f290/0xa5f294: limits and counts.
        assert_eq!(peer.maximum_incoming_connections(), 0);
        peer.set_maximum_incoming_connections(8);
        assert_eq!(peer.maximum_incoming_connections(), 8);
        assert_eq!(peer.number_of_connections(&mut || 3), 3);
        // IDA 0xa5f37c/0xa5f3a4: capped password roundtrip.
        peer.set_incoming_password(Some(&[1, 2, 3]));
        let mut len = 10;
        let mut out = Vec::new();
        assert_eq!(peer.incoming_password(Some(&mut out), &mut len), 3);
        assert_eq!((out, len), (vec![1, 2, 3], 3));
        peer.set_incoming_password(None);
        assert_eq!(peer.incoming_password(None, &mut len), 0);
        let big = vec![9u8; 300];
        peer.set_incoming_password(Some(big.as_slice()));
        assert_eq!(peer.incoming_password(None, &mut len), 255);
        // IDA 0xa5fc00: staged shutdown.
        let order = std::cell::RefCell::new(Vec::new());
        peer.shutdown(0, &mut || order.borrow_mut().push("n"), &mut || order.borrow_mut().push("d"), &mut || order.borrow_mut().push("c"));
        peer.shutdown(5, &mut || order.borrow_mut().push("n"), &mut || order.borrow_mut().push("d"), &mut || order.borrow_mut().push("c"));
        assert_eq!(order.borrow().as_slice(), ["d", "c", "n", "d", "c"]);
    }
    #[test]
    fn connection_lookup_gates() {
        // IDA 0xa606a0/0xa60878/0xa61810: table/packet releases.
        RakPeer::clear_requested_connection_list();
        RakPeer::clear_remote_system_lookup();
        RakPeer::deallocate_packet();
        // IDA 0xa60958: capped active list.
        let remotes = vec![
            SystemAddress { family: 2, port: 1, binary: 1, debug_port: 1, system_index: 0 },
            SystemAddress { family: 2, port: 2, binary: 2, debug_port: 2, system_index: 0 },
        ];
        assert!(RakPeer::connection_list(false, &remotes, 9).is_empty());
        assert_eq!(RakPeer::connection_list(true, &remotes, 1).len(), 1);
        assert_eq!(RakPeer::connection_list(true, &remotes, 9).len(), 2);
        // IDA 0xa60ab0/0xa60ad0/0xa61888: receipt counter and peer cap.
        let mut peer = RakPeer::new();
        assert_eq!(peer.next_send_receipt(), 0);
        assert_eq!(peer.increment_next_send_receipt(), 1);
        assert_eq!(peer.next_send_receipt(), 1);
        peer.max_peers = 32;
        assert_eq!(peer.maximum_number_of_peers(), 32);
        // IDA 0xa60dec/0xa613c4/0xa61e58: loopback, pop, cancel.
        let mut pushed = Vec::new();
        RakPeer::send_loopback(None, &mut |d| pushed.push(d.to_vec()));
        RakPeer::send_loopback(Some(&[7]), &mut |d| pushed.push(d.to_vec()));
        assert_eq!(pushed, vec![vec![7]]);
        assert_eq!(RakPeer::receive(Some(4)), Some(4));
        assert_eq!(RakPeer::receive(None), None);
        let mut cancelled = false;
        RakPeer::cancel_connection_attempt(&mut || cancelled = true);
        assert!(cancelled);
        // IDA 0xa62070: state precedence.
        assert_eq!(RakPeer::connection_state(true, true, 5, true, 3), 0);
        assert_eq!(RakPeer::connection_state(true, false, -1, true, 3), 6);
        assert_eq!(RakPeer::connection_state(false, false, 2, false, 3), 5);
        assert_eq!(RakPeer::connection_state(false, false, 2, true, 3), 3);
    }
    #[test]
    fn index_ban_ping_gates() {
        // IDA 0xa62178/0xa623c8/0xa622d8: index lookups with guards.
        let un = SystemAddress::new();
        let a = SystemAddress { family: 2, port: 100, binary: 10, debug_port: 100, system_index: 0 };
        let b = SystemAddress { family: 2, port: 200, binary: 20, debug_port: 200, system_index: 0 };
        let remotes = vec![(a, true), (b, false)];
        assert_eq!(RakPeer::index_from_address(&remotes, &un, &un, None), -1);
        assert_eq!(RakPeer::index_from_address(&remotes, &a, &un, Some(0)), 0);
        assert_eq!(RakPeer::index_from_address(&remotes, &b, &un, Some(0)), 1);
        assert_eq!(RakPeer::index_from_address(&remotes, &b, &un, None), 1);
        assert_eq!(RakPeer::index_from_address(&[], &a, &un, None), -1);
        let gremotes = vec![(7u64, true), (9u64, false)];
        assert_eq!(RakPeer::index_from_guid(&gremotes, UNASSIGNED_RAKNET_GUID, UNASSIGNED_RAKNET_GUID, None), -1);
        assert_eq!(RakPeer::index_from_guid(&gremotes, 9, 0, None), 1);
        assert_eq!(RakPeer::index_from_guid(&gremotes, 42, 0, None), -1);
        // IDA 0xa623e8/0xa62440: slot reads.
        let addrs = vec![None, Some(a)];
        assert_eq!(RakPeer::system_address_from_index(&addrs, 1, un), a);
        assert_eq!(RakPeer::system_address_from_index(&addrs, 0, un), un);
        assert_eq!(RakPeer::system_address_from_index(&addrs, 9, un), un);
        let guids = vec![None, Some(RakNetGuid { g: 7, system_index: 0 })];
        assert_eq!(RakPeer::guid_from_index(&guids, 1, RakNetGuid::new()).g, 7);
        assert_eq!(RakPeer::guid_from_index(&guids, 0, RakNetGuid::new()), RakNetGuid::new());
        // IDA 0xa62560/0xa62698/0xa6273c/0xa627c8/0xa627d0: ban list.
        let mut peer = RakPeer::new();
        peer.add_to_ban_list("", 100, 1000);
        peer.add_to_ban_list("12345678901234567", 100, 1000);
        assert!(peer.ban_list.is_empty());
        peer.add_to_ban_list("10.0.0.1", 100, 1000);
        peer.add_to_ban_list("10.0.0.1", 0, 2000);
        assert_eq!(peer.ban_list, vec![("10.0.0.1".to_owned(), 0)]);
        peer.add_to_ban_list("10.0.0.2", 100, 1000);
        assert!(peer.is_banned("10.0.0.2", 1050));
        assert!(!peer.is_banned("10.0.0.2", 1200));
        assert!(!peer.is_banned("10.0.0.9", 1050));
        peer.add_to_ban_list("192.168.*.*", 0, 0);
        assert!(peer.is_banned("192.168.5.5", 99999));
        peer.remove_from_ban_list(Some("192.168.*.*"));
        assert!(!peer.is_banned("192.168.5.5", 99999));
        peer.remove_from_ban_list(None);
        peer.clear_ban_list();
        assert!(peer.ban_list.is_empty());
        peer.set_limit_ip_connection_frequency(true);
        assert!(peer.limit_ip_connection_frequency);
        // IDA 0xa628c0/0xa628e4: ping gates on activity.
        let mut sent = Vec::new();
        RakPeer::send_ping(false, true, &mut |b| sent.push(b));
        RakPeer::send_ping(true, false, &mut |b| sent.push(b));
        assert_eq!(sent, [false]);
    }
    #[test]
    fn ping_remote_id_gates() {
        // IDA 0xa62af0: host presence gates the send.
        let mut n = 0;
        assert_eq!(RakPeer::ping_host(None, &mut || n += 1), 0);
        assert_eq!(RakPeer::ping_host(Some("h"), &mut || n += 1), 1);
        assert_eq!(n, 1);
        // IDA 0xa62d48/0xa62ea0/0xa62f3c: sample reductions.
        assert_eq!(RakPeer::average_ping(false, &[10, 20]), -1);
        assert_eq!(RakPeer::average_ping(true, &[]), -1);
        assert_eq!(RakPeer::average_ping(true, &[0xFFFF, 5]), -1);
        assert_eq!(RakPeer::average_ping(true, &[10, 20, 30, 40, 50, 60]), 30);
        assert_eq!(RakPeer::last_ping(false, 9), -1);
        assert_eq!(RakPeer::last_ping(true, 9), 9);
        assert_eq!(RakPeer::lowest_ping(false, 9), -1);
        assert_eq!(RakPeer::lowest_ping(true, 9), 9);
        // IDA 0xa62dec: guid/address resolution paths.
        let remotes = vec![(7u64, true)];
        assert_eq!(RakPeer::remote_system_index(true, &mut || None, &remotes, 7, false), Some(0));
        assert_eq!(RakPeer::remote_system_index(true, &mut || None, &remotes, 7, true), None);
        assert_eq!(RakPeer::remote_system_index(false, &mut || Some(3), &remotes, 0, false), Some(3));
        // IDA 0xa62fbc/0xa62fc0/0xa63000: occasional flag and response bytes.
        let mut peer = RakPeer::new();
        peer.set_occasional_ping(true);
        assert!(peer.occasional_ping);
        peer.set_offline_ping_response(Some(b"hi"));
        assert_eq!(peer.offline_ping_response(), b"hi");
        peer.set_offline_ping_response(None);
        assert!(peer.offline_ping_response().is_empty());
        // IDA 0xa63034/0xa63278/0xa63378: internal/external/guid reads.
        let un = SystemAddress::new();
        let a = SystemAddress { family: 2, port: 1, binary: 1, debug_port: 1, system_index: 0 };
        let local = vec![a];
        assert_eq!(RakPeer::internal_id(&un, &un, &local, 0, None), a);
        assert_eq!(RakPeer::internal_id(&a, &un, &local, 9, None), un);
        assert_eq!(RakPeer::internal_id(&a, &un, &local, 9, Some(a)), a);
        let remotes = vec![(a, a, false)];
        assert_eq!(RakPeer::external_id(&un, &un, a, &remotes), a);
        assert_eq!(RakPeer::external_id(&a, &un, a, &remotes), a);
        assert_eq!(RakPeer::external_id(&a, &un, a, &[]), un);
        peer.my_guid = 0x1234;
        assert_eq!(peer.my_guid(), 0x1234);
    }
    #[test]
    fn bound_timeout_mtu_gates() {
        let un = SystemAddress::new();
        let a = SystemAddress { family: 2, port: 1, binary: 1, debug_port: 1, system_index: 0 };
        let b = SystemAddress { family: 2, port: 2, binary: 2, debug_port: 2, system_index: 0 };
        // IDA 0xa6338c/0xa63490/0xa63574/0xa63620: bound and cross lookups.
        assert_eq!(RakPeer::my_bound_address(&[a, b], 1, un), b);
        assert_eq!(RakPeer::my_bound_address(&[a], 5, un), un);
        let remotes = vec![(a, 7u64)];
        assert_eq!(RakPeer::guid_from_system_address(&un, &un, 99, &remotes, None), 99);
        assert_eq!(RakPeer::guid_from_system_address(&a, &un, 99, &remotes, Some(0)), 7);
        assert_eq!(RakPeer::guid_from_system_address(&b, &un, 99, &remotes, None), UNASSIGNED_RAKNET_GUID);
        let gremotes = vec![(7u64, a)];
        assert_eq!(RakPeer::system_index_from_guid(&gremotes, UNASSIGNED_RAKNET_GUID, UNASSIGNED_RAKNET_GUID, 99, None), -1);
        assert_eq!(RakPeer::system_index_from_guid(&gremotes, 99, UNASSIGNED_RAKNET_GUID, 99, None), -1);
        assert_eq!(RakPeer::system_index_from_guid(&gremotes, 7, UNASSIGNED_RAKNET_GUID, 99, Some(0)), 0);
        assert_eq!(RakPeer::system_index_from_guid(&gremotes, 8, UNASSIGNED_RAKNET_GUID, 99, None), -1);
        assert_eq!(RakPeer::system_address_from_guid(UNASSIGNED_RAKNET_GUID, UNASSIGNED_RAKNET_GUID, 99, a, un, &gremotes, None), un);
        assert_eq!(RakPeer::system_address_from_guid(99, UNASSIGNED_RAKNET_GUID, 99, a, un, &gremotes, None), a);
        assert_eq!(RakPeer::system_address_from_guid(7, UNASSIGNED_RAKNET_GUID, 99, a, un, &gremotes, Some(0)), a);
        assert_eq!(RakPeer::system_address_from_guid(8, UNASSIGNED_RAKNET_GUID, 99, a, un, &gremotes, None), un);
        // IDA 0xa63754/0xa63844/0xa638fc: timeout and MTU paths.
        let mut peer = RakPeer::new();
        let order = std::cell::RefCell::new(Vec::new());
        peer.set_timeout_time(&un, &un, 5000, &mut || order.borrow_mut().push("all"), &mut || order.borrow_mut().push("one"));
        peer.set_timeout_time(&a, &un, 5000, &mut || order.borrow_mut().push("all"), &mut || order.borrow_mut().push("one"));
        assert_eq!(peer.default_timeout_ms, 5000);
        assert_eq!(order.borrow().as_slice(), ["all", "one"]);
        assert_eq!(RakPeer::timeout_time(&un, &un, 5000, Some(9)), 5000);
        assert_eq!(RakPeer::timeout_time(&a, &un, 5000, Some(9)), 9);
        assert_eq!(RakPeer::timeout_time(&a, &un, 5000, None), 5000);
        assert_eq!(RakPeer::mtu_size(Some(1400), 1500), 1400);
        assert_eq!(RakPeer::mtu_size(None, 1500), 1500);
        // IDA 0xa639b4/0xa639e4/0xa63a28/0xa63aa8: local address surface.
        assert_eq!(RakPeer::number_of_addresses(&[a, b]), 2);
        assert_eq!(RakPeer::local_ip(&[a], 0), a.dotted());
        assert_eq!(RakPeer::local_ip(&[a], 3), "");
        assert!(RakPeer::is_local_ip("127.0.0.1", &[]));
        assert!(RakPeer::is_local_ip("localhost", &[]));
        assert!(RakPeer::is_local_ip("10.0.0.9", &["10.0.0.9".to_owned()]));
        assert!(!RakPeer::is_local_ip("10.0.0.9", &[]));
        assert!(!RakPeer::is_local_ip("", &[]));
        peer.allow_connection_response_ip_migration(true);
        assert!(peer.allow_ip_migration);
    }
    #[test]
    fn plugin_packet_simulator_gates() {
        // IDA 0xa63bd8/0xa63c14/0xa63c1c: intervals and timeouts.
        let mut peer = RakPeer::new();
        peer.set_split_message_progress_interval(7);
        assert_eq!(peer.split_message_progress_interval(), 7);
        peer.set_unreliable_timeout(300);
        assert_eq!(peer.unreliable_timeout_ms, 300);
        // IDA 0xa63c58: TTL probe gates on host presence.
        let mut n = 0;
        RakPeer::send_ttl(None, &mut || n += 1);
        RakPeer::send_ttl(Some("h"), &mut || n += 1);
        assert_eq!(n, 1);
        // IDA 0xa63cf8/0xa63e54: attach dedupes, detach removes.
        let mut peer = RakPeer::new();
        assert_eq!(peer.attach_plugin(3, false, &mut || n += 1), 1);
        assert_eq!(peer.attach_plugin(3, false, &mut || n += 1), 1);
        assert_eq!(peer.attach_plugin(5, true, &mut || n += 1), 1);
        assert_eq!(n, 3);
        peer.detach_plugin(Some(3), false, &mut || n += 10);
        peer.detach_plugin(None, false, &mut || n += 10);
        assert_eq!(n, 13);
        assert!(peer.plugins.is_empty());
        let order = std::cell::RefCell::new(Vec::new());
        RakPeer::push_back_packet(false, true, &mut || order.borrow_mut().push("h"), &mut |b| order.borrow_mut().push(if b { "t" } else { "f" }));
        RakPeer::push_back_packet(true, false, &mut || order.borrow_mut().push("h"), &mut |b| order.borrow_mut().push(if b { "t" } else { "f" }));
        assert_eq!(order.borrow().as_slice(), ["h", "f"]);
        let packet = RakPeer::allocate_packet(9);
        assert_eq!((packet.data.len(), packet.guid), (9, 0));
        // IDA 0xa64564/0xa64568/0xa64570: compiled-out simulator.
        RakPeer::apply_network_simulator();
        peer.set_per_connection_bandwidth_limit(11);
        assert_eq!(peer.per_connection_bandwidth_limit, 11);
        assert!(!RakPeer::is_network_simulator_active());
    }
    #[test]
    fn stats_outofband_string_gates() {
        // IDA 0xa64574: id byte, guid, offline marker.
        let mut s = BitStream::new();
        RakPeer::write_out_of_band_header(&mut s, 0x0102_0304_0506_0708);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_u8(), Some(13));
        assert_eq!(r.read_u64(), Some(0x0102_0304_0506_0708));
        let mut marker = [0u8; 16];
        assert!(r.read_aligned_bytes(&mut marker));
        assert_eq!(marker, OFFLINE_MESSAGE_DATA_ID);
        // IDA 0xa647f4/0xa64b78: aggregate vs indexed fills.
        let mut n = 0;
        assert!(RakPeer::get_statistics(true, &mut || n += 1, &mut || false));
        assert!(!RakPeer::get_statistics(false, &mut || n += 1, &mut || false));
        assert!(RakPeer::get_statistics_index(&[false, true], 1, &mut || n += 10));
        assert!(!RakPeer::get_statistics_index(&[false, true], 0, &mut || n += 10));
        assert!(!RakPeer::get_statistics_index(&[false], 5, &mut || n += 10));
        assert_eq!(n, 11);
        // IDA 0xa64bb4/0xa65974/0xa6c4ec/0xa6d194: ring, slot, active.
        assert_eq!(RakPeer::receive_buffer_size(3, 7, 16), 4);
        assert_eq!(RakPeer::receive_buffer_size(14, 2, 16), 4);
        let mut slots = vec![SystemAddress::new()];
        let a = SystemAddress { family: 2, port: 1, binary: 1, debug_port: 1, system_index: 0 };
        RakPeer::reference_remote_system(&mut slots, 0, a);
        RakPeer::reference_remote_system(&mut slots, 9, a);
        assert_eq!(slots, vec![a]);
        assert!(RakPeer::new().is_active());
        RakPeer::init_remote_system();
        // IDA 0xa6eaa4/0xa6eab4/0xa6ec58: string constructors.
        assert_eq!(rak_string(), "");
        assert_eq!(rak_string_format("x"), "x");
    }
    #[test]
    fn string_pool_random_gates() {
        // IDA 0xa6ec8c/0xa6eed4/0xa6ef14/0xa6f210/0xa6f3c0: pool ops.
        let mut s = rak_string_format("abc");
        rak_string_assign(&mut s, "de");
        assert_eq!(s, "de");
        rak_string_allocate(&mut s, 64);
        assert_eq!(s, "de");
        rak_string_free(&mut s);
        assert_eq!(s, "");
        rak_string_free_pool();
        rak_string_list_drop();
        spawn_rak_thread();
        // IDA 0xa6f328/0xa6f358: length-prefixed wire form.
        let mut stream = BitStream::new();
        rak_string_serialize(&mut stream, "hi");
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(rak_string_deserialize(&mut r), Some("hi".to_owned()));
        let mut stream = BitStream::new();
        rak_string_serialize(&mut stream, "");
        let mut r = BitStream::from_bytes(&stream.into_bytes());
        assert_eq!(rak_string_deserialize(&mut r), Some(String::new()));
        assert_eq!(rak_string_deserialize(&mut BitStream::from_bytes(&[])), None);
        // IDA 0xa70260/0xa70278/0xa702a4: deterministic MT sequence.
        let mut rng = RakNetRandom::new();
        rng.seed_mt(42);
        let (a, b) = (rng.random_mt(), rng.random_mt());
        let mut rng2 = RakNetRandom::new();
        rng2.seed_mt(42);
        assert_eq!((rng2.random_mt(), rng2.random_mt()), (a, b));
        rng2.seed_mt(44);
        assert_ne!(rng2.random_mt(), a);
        // First draw twists: state[0] is the tempered twist output.
        let mut rng3 = RakNetRandom::new();
        rng3.seed_mt(1);
        let _ = rng3.random_mt();
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

/// `OFFLINE_MESSAGE_DATA_ID` (IDA 0xa62bc4/0xa645ae): the 16-byte
/// offline ping marker.
pub const OFFLINE_MESSAGE_DATA_ID: [u8; 16] = [
 0x00, 0xFF, 0xFF, 0x00, 0xFE, 0xFE, 0xFE, 0xFE,
 0xFD, 0xFD, 0xFD, 0xFD, 0x12, 0x34, 0x56, 0x78,
];

/// `RakNet::RakString::RakString` (IDA 0xa6eaa4): points at the shared
/// empty string; modeled as empty.
#[must_use]
pub fn rak_string() -> String {
 String::new()
}

/// `RakNet::RakString::RakString(format, ...)` (IDA 0xa6ec58) and
/// `Assign` (IDA 0xa6eab4): printf formatting stays engine-side (the
/// 512-byte fast path spills to heap past 0x1FE chars); this stores
/// the formatted text.
#[must_use]
pub fn rak_string_format(formatted: &str) -> String {
 formatted.to_owned()
}

/// `RakNet::RakString::Free` (IDA 0xa6ec8c): drops a reference and
/// recycles at zero, ending empty either way; the shared pool stays
/// engine-side.
pub fn rak_string_free(s: &mut String) {
 s.clear();
}

/// `RakNet::RakString::operator=` (IDA 0xa6eed4): copies the text.
pub fn rak_string_assign(dst: &mut String, src: &str) {
 dst.clear();
 dst.push_str(src);
}

/// `RakNet::RakString::Allocate` (IDA 0xa6ef14): reserves through the
/// shared-string pool engine-side; the inline 112-byte fast path and
/// heap doubling stay there too.
pub fn rak_string_allocate(s: &mut String, capacity: usize) {
 s.reserve(capacity);
}

/// `RakNet::RakString::FreeMemoryNoMutex` (IDA 0xa6f210): drains the
/// global pool; nothing observable here.
pub fn rak_string_free_pool() {}

/// `RakNet::RakString::Serialize` (IDA 0xa6f328): `u16` length plus
/// aligned bytes.
pub fn rak_string_serialize(stream: &mut BitStream, s: &str) {
 stream.write_u16(s.len() as u16);
 stream.write_aligned_bytes(s.as_bytes());
}

/// `RakNet::RakString::Deserialize` (IDA 0xa6f358): frees first, then
/// the length; empty strings only consume the alignment pad. Returns
/// `None` on short reads.
#[must_use]
pub fn rak_string_deserialize(stream: &mut BitStream) -> Option<String> {
 let len = stream.read_u16()?;
 if len == 0 {
 let mut pad: [u8; 0] = [];
 stream.read_aligned_bytes(&mut pad);
 return Some(String::new());
 }
 let mut buf = vec![0u8; len as usize];
 if !stream.read_aligned_bytes(&mut buf) {
 return None;
 }
 String::from_utf8(buf).ok()
}

/// `List<SharedString *>::~List` (IDA 0xa6f3c0): node release stays
/// engine-side.
pub fn rak_string_list_drop() {}

/// `RakNet::RakThread::Create` (IDA 0xa6fa3c): thread spawn stays
/// engine-side.
pub fn spawn_rak_thread() {}

/// `RakNet::RakNetRandom` (IDA 0xa70260): MT19937 state with the
/// remaining-use count beside it.
#[derive(Clone, Debug)]
pub struct RakNetRandom {
 state: [u32; 624],
 remaining: i32,
}

impl Default for RakNetRandom {
 /// Ctor (IDA 0xa70260): the use count starts at -1, state uninit.
 fn default() -> Self {
 Self { state: [0; 624], remaining: -1 }
 }
}

impl RakNetRandom {
 /// `RakNetRandom::RakNetRandom` (IDA 0xa70260).
 pub fn new() -> Self {
 Self::default()
 }

 /// `RakNetRandom::SeedMT` (IDA 0xa70278): `state[0] = seed | 1`,
 /// then multiply-only chaining with `0x10DCD` (the address-plus-one
 /// of the `getStringValue`method IDA names at 0xa70278). The use
 /// count resets to 0 so the next draw twists first.
 pub fn seed_mt(&mut self, seed: u32) {
 let mut s = seed | 1;
 self.state[0] = s;
 for i in 1..624 {
 s = s.wrapping_mul(0x10DCD);
 self.state[i] = s;
 }
 self.remaining = 0;
 }

 /// Standard MT19937 twist (IDA 0xa700b8 `reloadMT`, `MATRIX_A`
 /// `0x9908B0DF`).
 fn twist(&mut self) {
 for i in 0..624 {
 let y = (self.state[i] & 0x8000_0000) | (self.state[(i + 1) % 624] & 0x7FFF_FFFF);
 let mut x = self.state[(i + 397) % 624] ^ (y >> 1);
 if y & 1 != 0 {
 x ^= 0x9908_B0DF;
 }
 self.state[i] = x;
 }
 }

 /// Standard MT19937 temper (IDA 0xa702d4/0xa702e6).
 fn temper(y: u32) -> u32 {
 let y = y ^ (y >> 11);
 let y = y ^ ((y << 7) & 0x9D2C_5680);
 let y = y ^ ((y << 15) & 0xEFC6_0000);
 y ^ (y >> 18)
 }

 /// `RakNetRandom::RandomMT` (IDA 0xa702a4): counts down, twisting
 /// for a fresh cycle (the global reseed for a never-seeded instance
 /// stays engine-side).
 pub fn random_mt(&mut self) -> u32 {
 self.remaining -= 1;
 if self.remaining < 0 {
 self.twist();
 self.remaining = 623;
 return Self::temper(self.state[0]);
 }
 Self::temper(self.state[623 - self.remaining as usize])
 }
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

/// `RakNet::Packet` reduced to its payload plus routing ids (IDA
/// 0xa6406c): the data buffer, sender address, and sender guid.
#[derive(Clone, Debug, Default)]
pub struct Packet {
 pub data: Vec<u8>,
 pub address: SystemAddress,
 pub guid: u64,
}

impl Packet {
 #[must_use]
 pub fn with_data(mut self, data: Vec<u8>) -> Self {
 self.data = data;
 self
 }
}

/// `DataStructures::MemoryPool<RakNet::Packet>::Allocate` (IDA 0xa7d2ac):
/// pool blocks stay engine-side; hand out a default packet.
#[must_use]
pub fn packet_allocate() -> Packet {
    Packet::default()
}

/// `DataStructures::MemoryPool<RakNet::Packet>::Release` (IDA 0xa7d3d8):
/// return a packet to the pool (drop Rust-side).
pub fn packet_release(_packet: Packet) {}

/// `DataStructures::Queue<RakNet::Packet *>::Push` (IDA 0xa7d1d8): append
/// a packet to the back of the queue.
pub fn packet_queue_push(queue: &mut std::collections::VecDeque<Packet>, packet: Packet) {
    queue.push_back(packet);
}
/// `RakNet::RakPeer::SocketQueryOutput` (IDA 0xa6c7d0): the answering
/// socket index plus the queried address. Poll results stay engine-side.
#[derive(Clone, Copy, Debug, Default)]
pub struct SocketQueryOutput {
 pub addr: SystemAddress,
 pub socket_index: u32,
}

/// `DataStructures::MemoryPool<RakNet::RakPeer::SocketQueryOutput>::Allocate`
/// (IDA 0xa6c7d0): pool blocks stay engine-side; hand out a default output.
#[must_use]
pub fn socket_query_output_allocate() -> SocketQueryOutput {
    SocketQueryOutput::default()
}

/// `DataStructures::MemoryPool<RakNet::RakPeer::SocketQueryOutput>::Release`
/// (IDA 0xa6c8e4): return an output to the pool (drop Rust-side).
pub fn socket_query_output_release(_output: SocketQueryOutput) {}

/// `RakNet::RakPeer::RecvFromStruct` (IDA 0xa6c9ac): a received datagram
/// plus its sender. The engine-side buffer details live with the socket.
#[derive(Clone, Debug, Default)]
pub struct RecvFrom {
 pub addr: SystemAddress,
 pub bytes: Vec<u8>,
}

/// `DataStructures::MemoryPool<RakNet::RakPeer::RecvFromStruct>::Release`
/// (IDA 0xa6c9ac): return a receive struct to the pool (drop Rust-side).
pub fn recv_from_release(_recv: RecvFrom) {}

/// `DataStructures::MemoryPool<RakNet::RakPeer::BufferedCommandStruct>::Allocate`
/// (IDA 0xa6cdb0): pool blocks stay engine-side; hand out a default command.
#[must_use]
pub fn buffered_command_allocate() -> BufferedCommand {
    BufferedCommand::default()
}

/// `DataStructures::MemoryPool<RakNet::RakPeer::BufferedCommandStruct>::Release`
/// (IDA 0xa6ca84): return a command to the pool (drop Rust-side).
pub fn buffered_command_release(_cmd: BufferedCommand) {}

/// `DataStructures::Queue<RakNet::RakPeer::BufferedCommandStruct *>::Push`
/// (IDA 0xa6ccdc): append a command to the back of the queue. The original
/// is a head/tail ring that doubles at 2x; `VecDeque` keeps that edge.
pub fn buffered_command_queue_push(
    queue: &mut std::collections::VecDeque<BufferedCommand>,
    cmd: BufferedCommand,
) {
    queue.push_back(cmd);
}

/// `RakNet::RakNetSmartPtr<RakNet::RakNetSocket>` (IDA 0xa6cb5c): the
/// refcounted socket stays engine-side; only its descriptor crosses here.
pub type SocketHandle = u32;

/// `RakNet::OP_DELETE_ARRAY<RakNet::RakNetSmartPtr<RakNet::RakNetSocket>>`
/// (IDA 0xa6cb5c): delete an array of socket smart pointers (drop Rust-side).
pub fn delete_socket_array(_sockets: Vec<SocketHandle>) {}

/// `DataStructures::List<RakNet::RakNetSmartPtr<RakNet::RakNetSocket>>::Insert`
/// (IDA 0xa6d2bc): append a socket to the list. The original doubles
/// capacity (16, then 2x); `Vec` keeps that edge.
pub fn socket_list_insert(list: &mut Vec<SocketHandle>, socket: SocketHandle) {
    list.push(socket);
}

/// `DataStructures::List<RakNet::RakNetGUID>::Insert` (IDA 0xa6ced8):
/// append a guid. Guids are bare `u64` (cf. `RakPeer::my_guid`).
pub fn guid_list_insert(list: &mut Vec<u64>, guid: u64) {
    list.push(guid);
}

/// `DataStructures::List<RakNet::SystemAddress>::Insert` (IDA 0xa6d030):
/// append an address.
pub fn address_list_insert(list: &mut Vec<SystemAddress>, addr: SystemAddress) {
    list.push(addr);
}

/// `RakNet::RakPeer` (IDA 0xa5cb00): sockets, queues, and threads stay
/// engine-side; the exception list, limits, and password live here.
#[derive(Clone, Debug, Default)]
pub struct RakPeer {
 /// Security exception patterns at +1400 (IDA 0xa5efa8).
 pub security_exceptions: Vec<String>,
 /// Max incoming connections at +16 (IDA 0xa5f28c).
 pub max_incoming_connections: u16,
 /// Incoming password bytes at +296, length at +552 (IDA 0xa5f37c).
 pub incoming_password: Vec<u8>,
 /// Next send receipt at +1936 (IDA 0xa60ab0).
 pub next_send_receipt: u32,
 /// Maximum peers (IDA 0xa61888).
 pub max_peers: u16,
 /// Ban entries `(pattern, expiry_ms)` at +788/792, `0` expiry means
 /// forever (IDA 0xa62560).
 pub ban_list: Vec<(String, u32)>,
 /// IP connection-frequency limiter flag (IDA 0xa627c8).
 pub limit_ip_connection_frequency: bool,
 /// Offline ping response bytes (IDA 0xa62fc0).
 pub offline_ping_response: Vec<u8>,
 /// Occasional-ping flag (IDA 0xa62fbc).
 pub occasional_ping: bool,
 /// Own guid (IDA 0xa63378).
 pub my_guid: u64,
 /// Default timeout ms at +1348 (IDA 0xa63754).
 pub default_timeout_ms: u32,
 /// Connection-response IP migration flag (IDA 0xa63aa8).
 pub allow_ip_migration: bool,
 /// Attached plugin ids at +200/+203, selected by the vtable +44 flag
 /// (IDA 0xa63cf8).
 pub plugins: Vec<u32>,
 pub plugins_alt: Vec<u32>,
 /// Split-message progress interval (IDA 0xa63bd8).
 pub split_message_progress_interval: i32,
 /// Unreliable timeout ms (IDA 0xa63c1c).
 pub unreliable_timeout_ms: u32,
 /// Per-connection outgoing bandwidth limit at +0x554 (IDA 0xa64568).
 pub per_connection_bandwidth_limit: u32,
 /// Active flag backing `IsActive` (IDA 0xa6c4ec reads byte +4 == 0).
 pub active: bool,
 /// User update-thread callback words at +1616/+1620 (IDA 0xa645b0).
 pub user_update_thread: u32,
 pub user_update_thread_data: u32,
}

impl RakPeer {
 /// `RakPeer::RakPeer` (IDA 0xa5cb00). Fresh peers read active: the
 /// `IsActive` flag byte at +4 starts zeroed (IDA 0xa6c4ec).
 pub fn new() -> Self {
 Self { active: true, ..Self::default() }
 }

 /// `RakPeer::Startup` result codes (IDA 0xa5e3c0): 0 ok, 1 already
 /// active or no descriptors, 5 port in use, 6 bind failure, 9 thread
 /// failure. Socket binding, thread spawn, and table setup stay
 /// engine-side.
 pub fn startup(
 &mut self,
 active: bool,
 has_descriptors: bool,
 port_in_use: bool,
 bind_ok: bool,
 threads_ok: bool,
 ) -> u32 {
 if active {
 return 1;
 }
 if !has_descriptors {
 return 2;
 }
 if port_in_use {
 return 5;
 }
 if !bind_ok {
 return 6;
 }
 if !threads_ok {
 return 9;
 }
 0
 }

 /// `RakPeer::DerefAllSockets` (IDA 0xa5eab8),
 /// `ClearBufferedCommands` (IDA 0xa5ebd4), and
 /// `ClearBufferedPackets` (IDA 0xa5eca0): queue releases stay
 /// engine-side.
 pub fn deref_all_sockets() {}
 pub fn clear_buffered_commands() {}
 pub fn clear_buffered_packets() {}

 /// `UpdateNetworkLoop` (IDA 0xa5ed50) and `RecvFromLoop` (IDA
 /// 0xa5ee80): thread entry points; the loops stay engine-side.
 pub fn update_network_loop() {}
 pub fn recv_from_loop() {}
 /// `RakPeer::InitializeSecurity` (IDA 0xa5efa0): hardcoded `return 0`
 /// (security is compiled out of this build).
 pub fn initialize_security(&mut self) -> u32 {
 0
 }

 /// `RakPeer::DisableSecurity` (IDA 0xa5efa4): empty.
 pub fn disable_security(&mut self) {}

 /// `RakPeer::AddToSecurityExceptionList` (IDA 0xa5efa8): pushes the
 /// pattern; locking stays engine-side.
 pub fn add_to_security_exception_list(&mut self, addr: &str) {
 self.security_exceptions.push(addr.to_owned());
 }

 /// `RakPeer::RemoveFromSecurityExceptionList` (IDA 0xa5f08c):
 /// drops every entry matching `addr`, or clears the list for null.
 pub fn remove_from_security_exception_list(&mut self, addr: Option<&str>) {
 match addr {
 Some(a) => self.security_exceptions.retain(|e| !ip_address_match(e, a)),
 None => self.security_exceptions.clear(),
 }
 }

 /// `RakPeer::IsInSecurityExceptionList` (IDA 0xa5f230).
 #[must_use]
 pub fn is_in_security_exception_list(&self, addr: &str) -> bool {
 self.security_exceptions.iter().any(|e| ip_address_match(e, addr))
 }

 /// `RakPeer::SetMaximumIncomingConnections` (IDA 0xa5f28c).
 pub fn set_maximum_incoming_connections(&mut self, max: u16) {
 self.max_incoming_connections = max;
 }

 /// `RakPeer::GetMaximumIncomingConnections` (IDA 0xa5f290).
 #[must_use]
 pub fn maximum_incoming_connections(&self) -> u16 {
 self.max_incoming_connections
 }

 /// `RakPeer::NumberOfConnections` (IDA 0xa5f294): the active-system
 /// enumeration stays engine-side.
 #[must_use]
 pub fn number_of_connections(&self, count_actives: &mut dyn FnMut() -> u16) -> u16 {
 count_actives()
 }

 /// `RakPeer::SetIncomingPassword` (IDA 0xa5f37c): length capped at
 /// 255, zeroed for null input.
 pub fn set_incoming_password(&mut self, data: Option<&[u8]>) {
 let bytes = data.unwrap_or(&[]);
 let len = bytes.len().min(255);
 self.incoming_password = bytes[..len].to_vec();
 }

 /// `RakPeer::GetIncomingPassword` (IDA 0xa5f3a4): with an output
 /// buffer, copies up to `*len` bytes and reports the count; without
 /// one, just reports the stored length. Returns the count either way.
 pub fn incoming_password(&self, out: Option<&mut Vec<u8>>, len: &mut usize) -> usize {
 if let Some(dst) = out {
 let n = (*len).min(self.incoming_password.len());
 dst.extend_from_slice(&self.incoming_password[..n]);
 *len = n;
 } else {
 *len = self.incoming_password.len();
 }
 *len
 }

 /// `RakPeer::Shutdown` (IDA 0xa5fc00): with a nonzero block duration
 /// every active system gets a disconnect notice first; then plugins
 /// detach and every queue, table, and lookup resets engine-side.
 pub fn shutdown(
 &mut self,
 block_ms: u32,
 notify: &mut dyn FnMut(),
 detach: &mut dyn FnMut(),
 clear: &mut dyn FnMut(),
 ) {
 if block_ms > 0 {
 notify();
 }
 detach();
 clear();
 }

 /// `RakPeer::ClearRequestedConnectionList` (IDA 0xa606a0) and
 /// `ClearRemoteSystemLookup` (IDA 0xa60878): table releases stay
 /// engine-side.
 pub fn clear_requested_connection_list() {}
 pub fn clear_remote_system_lookup() {}

 /// `RakPeer::GetConnectionList` (IDA 0xa60958): up to `capacity`
 /// active addresses; empty when inactive. Returns the list (its
 /// length is the reported count).
 #[must_use]
 pub fn connection_list(active: bool, remotes: &[SystemAddress], capacity: usize) -> Vec<SystemAddress> {
 if !active {
 return Vec::new();
 }
 remotes.iter().take(capacity).copied().collect()
 }

 /// `RakPeer::GetNextSendReceipt` (IDA 0xa60ab0): the +1936 counter
 /// under lock.
 #[must_use]
 pub fn next_send_receipt(&self) -> u32 {
 self.next_send_receipt
 }

 /// `RakPeer::IncrementNextSendReceipt` (IDA 0xa60ad0).
 pub fn increment_next_send_receipt(&mut self) -> u32 {
 self.next_send_receipt += 1;
 self.next_send_receipt
 }

 /// `RakPeer::SendLoopback` (IDA 0xa60dec): builds the packet and
 /// queues it when data is present.
 pub fn send_loopback(data: Option<&[u8]>, push: &mut dyn FnMut(&[u8])) {
 if let Some(d) = data {
 push(d);
 }
 }

 /// `RakPeer::Receive` (IDA 0xa613c4): pops the next filtered packet,
 /// or null when inactive or empty. Queue and plugin filtering stay
 /// engine-side.
 #[must_use]
 pub fn receive(next: Option<u32>) -> Option<u32> {
 next
 }

 /// `RakPeer::DeallocatePacket` (IDA 0xa61810): packet release stays
 /// engine-side.
 pub fn deallocate_packet() {}

 /// `RakPeer::AllocatePacket` (IDA 0xa6406c): a packet with a
 /// zeroed `size`-byte buffer, unassigned guid and address.
 #[must_use]
 pub fn allocate_packet(size: usize) -> Packet {
 Packet::default().with_data(vec![0u8; size])
 }

 /// `RakPeer::GetMaximumNumberOfPeers` (IDA 0xa61888).
 #[must_use]
 pub fn maximum_number_of_peers(&self) -> u16 {
 self.max_peers
 }

 /// `RakPeer::CancelConnectionAttempt` (IDA 0xa61e58): drops the
 /// pending attempt engine-side.
 pub fn cancel_connection_attempt(cancel: &mut dyn FnMut()) {
 cancel();
 }

 /// `RakPeer::GetConnectionState` (IDA 0xa62070): a direct remote-list
 /// hit is connected (0); otherwise -1 maps to 6, inactive to 5, and
 /// anything else reads the mapped state.
 #[must_use]
 pub fn connection_state(
 address_known: bool,
 direct_match: bool,
 index: i32,
 active: bool,
 state: u32,
 ) -> u32 {
 if address_known && direct_match {
 return 0;
 }
 if index == -1 {
 return 6;
 }
 if !active {
 return 5;
 }
 state
 }

 /// `RakPeer::GetIndexFromSystemAddress` (IDA 0xa62178/0xa623c8):
 /// unassigned addresses map to -1; otherwise the hint wins when it
 /// names an active match, else the first active match, else the
 /// first match of any kind. The hash-vs-linear strategy split stays
 /// engine-side.
 #[must_use]
 pub fn index_from_address(
 remotes: &[(SystemAddress, bool)],
 addr: &SystemAddress,
 unassigned: &SystemAddress,
 hint: Option<usize>,
 ) -> i32 {
 if addr.equals(unassigned) {
 return -1;
 }
 if let Some(h) = hint {
 if let Some((a, active)) = remotes.get(h) {
 if *active && a.equals(addr) {
 return h as i32;
 }
 }
 }
 if let Some(i) = remotes.iter().position(|(a, active)| *active && a.equals(addr)) {
 return i as i32;
 }
 remotes.iter().position(|(a, _)| a.equals(addr)).map_or(-1, |i| i as i32)
 }

 /// `RakPeer::GetIndexFromGuid` (IDA 0xa622d8): same shape over guid
 /// equality, with the unassigned guard.
 #[must_use]
 pub fn index_from_guid(
 remotes: &[(u64, bool)],
 guid: u64,
 unassigned: u64,
 hint: Option<usize>,
 ) -> i32 {
 if guid == unassigned {
 return -1;
 }
 if let Some(h) = hint {
 if let Some((g, active)) = remotes.get(h) {
 if *active && *g == guid {
 return h as i32;
 }
 }
 }
 if let Some(i) = remotes.iter().position(|(g, active)| *active && *g == guid) {
 return i as i32;
 }
 remotes.iter().position(|(g, _)| *g == guid).map_or(-1, |i| i as i32)
 }

 /// `RakPeer::GetSystemAddressFromIndex` (IDA 0xa623e8): the slot
 /// address when the index names a connected system, else unassigned.
 #[must_use]
 pub fn system_address_from_index(
 remotes: &[Option<SystemAddress>],
 index: i32,
 unassigned: SystemAddress,
 ) -> SystemAddress {
 if index >= 0 {
 if let Some(Some(addr)) = remotes.get(index as usize) {
 return *addr;
 }
 }
 unassigned
 }

 /// `RakPeer::GetGUIDFromIndex` (IDA 0xa62440): symmetric to
 /// [`system_address_from_index`](Self::system_address_from_index).
 #[must_use]
 pub fn guid_from_index(
 remotes: &[Option<RakNetGuid>],
 index: i32,
 unassigned: RakNetGuid,
 ) -> RakNetGuid {
 if index >= 0 {
 if let Some(Some(guid)) = remotes.get(index as usize) {
 return *guid;
 }
 }
 unassigned
 }

 /// `RakPeer::AddToBanList` (IDA 0xa62560): empty or over-15-char
 /// inputs are ignored; a known pattern refreshes its expiry
 /// (`now + timeout`, or forever for zero), else it is appended.
 pub fn add_to_ban_list(&mut self, addr: &str, timeout_ms: u32, now_ms: u32) {
 if addr.is_empty() || addr.len() > 0xF {
 return;
 }
 let expiry = if timeout_ms == 0 { 0 } else { now_ms.wrapping_add(timeout_ms) };
 if let Some(entry) = self.ban_list.iter_mut().find(|(p, _)| p == addr) {
 entry.1 = expiry;
 } else {
 self.ban_list.push((addr.to_owned(), expiry));
 }
 }

 /// `RakPeer::RemoveFromBanList` (IDA 0xa62698): exact-match
 /// swap-remove; ignored for null, empty, or over-long inputs.
 pub fn remove_from_ban_list(&mut self, addr: Option<&str>) {
 if let Some(a) = addr {
 if a.is_empty() || a.len() > 0xF {
 return;
 }
 if let Some(i) = self.ban_list.iter().position(|(p, _)| p == a) {
 self.ban_list.swap_remove(i);
 }
 }
 }

 /// `RakPeer::ClearBanList` (IDA 0xa6273c).
 pub fn clear_ban_list(&mut self) {
 self.ban_list.clear();
 }

 /// `RakPeer::SetLimitIPConnectionFrequency` (IDA 0xa627c8).
 pub fn set_limit_ip_connection_frequency(&mut self, limit: bool) {
 self.limit_ip_connection_frequency = limit;
 }

 /// `RakPeer::IsBanned` (IDA 0xa627d0): expired entries are evicted
 /// swap-last first; then the wildcard walk decides.
 pub fn is_banned(&mut self, addr: &str, now_ms: u32) -> bool {
 if addr.is_empty() || addr.len() > 0xF {
 return false;
 }
 let mut i = 0;
 while i < self.ban_list.len() {
 let (expired, pattern) = {
 let (p, expiry) = &self.ban_list[i];
 (*expiry != 0 && *expiry < now_ms, p.clone())
 };
 if expired {
 self.ban_list.swap_remove(i);
 continue;
 }
 if ip_address_match(&pattern, addr) {
 return true;
 }
 i += 1;
 }
 false
 }

 /// `RakPeer::Ping` (IDA 0xa628c0) / `PingInternal` (IDA 0xa628e4):
 /// timestamped ping packet goes out when active; the broadcast arm
 /// versus direct send stays engine-side.
 pub fn send_ping(active: bool, broadcast: bool, send: &mut dyn FnMut(bool)) {
 if active {
 send(broadcast);
 }
 }

 /// `RakPeer::Ping(host, port)` (IDA 0xa62af0): resolves and pings
 /// when a host is given; returns 1 on send, 0 otherwise.
 pub fn ping_host(host: Option<&str>, send: &mut dyn FnMut()) -> u32 {
 if host.is_some() {
 send();
 1
 } else {
 0
 }
 }

 /// `RakPeer::GetAveragePing` (IDA 0xa62d48): the mean of up to five
 /// samples, stopping at the `0xFFFF` sentinel; -1 without a system
 /// or without samples.
 #[must_use]
 pub fn average_ping(found: bool, samples: &[u16]) -> i32 {
 if !found {
 return -1;
 }
 let vals: Vec<u32> = samples.iter().take(5).take_while(|&&s| s != 0xFFFF).map(|&s| s as u32).collect();
 if vals.is_empty() {
 return -1;
 }
 (vals.iter().sum::<u32>() / vals.len() as u32) as i32
 }

 /// `RakPeer::GetRemoteSystem` (IDA 0xa62dec): an unassigned guid
 /// resolves by address engine-side; otherwise the guid scans the
 /// table, optionally restricted to inactive slots.
 #[must_use]
 pub fn remote_system_index(
 guid_assigned: bool,
 by_address: &mut dyn FnMut() -> Option<u32>,
 remotes: &[(u64, bool)],
 guid: u64,
 inactive_only: bool,
 ) -> Option<u32> {
 if !guid_assigned {
 return by_address();
 }
 remotes.iter().position(|(g, active)| *g == guid && (!inactive_only || !active)).map(|i| i as u32)
 }

 /// `RakPeer::GetLastPing` (IDA 0xa62ea0): the newest sample, or -1
 /// without a system.
 #[must_use]
 pub fn last_ping(found: bool, newest: u16) -> i32 {
 if !found {
 return -1;
 }
 newest as i32
 }

 /// `RakPeer::GetLowestPing` (IDA 0xa62f3c): the minimum sample, or
 /// -1 without a system.
 #[must_use]
 pub fn lowest_ping(found: bool, minimum: u16) -> i32 {
 if !found {
 return -1;
 }
 minimum as i32
 }

 /// `RakPeer::SetOccasionalPing` (IDA 0xa62fbc).
 pub fn set_occasional_ping(&mut self, occasional: bool) {
 self.occasional_ping = occasional;
 }

 /// `RakPeer::SetOfflinePingResponse` (IDA 0xa62fc0): resets the
 /// stream, then stores the bytes when both are present.
 pub fn set_offline_ping_response(&mut self, data: Option<&[u8]>) {
 self.offline_ping_response.clear();
 if let Some(d) = data {
 self.offline_ping_response.extend_from_slice(d);
 }
 }

 /// `RakPeer::GetOfflinePingResponse` (IDA 0xa63000): the stored
 /// response bytes.
 #[must_use]
 pub fn offline_ping_response(&self) -> &[u8] {
 &self.offline_ping_response
 }

 /// `RakPeer::GetInternalID` (IDA 0xa63034): unassigned addresses read
 /// the local bound list; otherwise the first active remote match
 /// wins, else unassigned.
 #[must_use]
 pub fn internal_id(
 addr: &SystemAddress,
 unassigned: &SystemAddress,
 local: &[SystemAddress],
 local_index: usize,
 remote: Option<SystemAddress>,
 ) -> SystemAddress {
 if addr.equals(unassigned) {
 return local.get(local_index).copied().unwrap_or(*unassigned);
 }
 remote.unwrap_or(*unassigned)
 }

 /// `RakPeer::GetExternalID` (IDA 0xa63278): unassigned addresses read
 /// the local external id; otherwise the first active remote match
 /// wins, else the last inactive match, else unassigned.
 #[must_use]
 pub fn external_id(
 addr: &SystemAddress,
 unassigned: &SystemAddress,
 own_external: SystemAddress,
 remotes: &[(SystemAddress, SystemAddress, bool)],
 ) -> SystemAddress {
 if addr.equals(unassigned) {
 return own_external;
 }
 let mut fallback = *unassigned;
 for (sys, ext, active) in remotes {
 if sys.equals(addr) {
 if *active {
 return *ext;
 }
 if !sys.equals(unassigned) {
 fallback = *sys;
 }
 }
 }
 fallback
 }

 /// `RakPeer::GetMyGUID` (IDA 0xa63378).
 #[must_use]
 pub fn my_guid(&self) -> u64 {
 self.my_guid
 }

 /// `RakPeer::GetMyBoundAddress` (IDA 0xa6338c): the socket's bound
 /// address at the index, or unassigned without sockets.
 #[must_use]
 pub fn my_bound_address(sockets: &[SystemAddress], index: usize, unassigned: SystemAddress) -> SystemAddress {
 sockets.get(index).copied().unwrap_or(unassigned)
 }

 /// `RakPeer::GetGuidFromSystemAddress` (IDA 0xa63490): the peer's
 /// own guid for unassigned input, else the hint-then-scan match, else
 /// unassigned.
 #[must_use]
 pub fn guid_from_system_address(
 addr: &SystemAddress,
 unassigned: &SystemAddress,
 own_guid: u64,
 remotes: &[(SystemAddress, u64)],
 hint: Option<usize>,
 ) -> u64 {
 if addr.equals(unassigned) {
 return own_guid;
 }
 if let Some(h) = hint {
 if let Some((a, g)) = remotes.get(h) {
 if a.equals(addr) {
 return *g;
 }
 }
 }
 remotes.iter().find(|(a, _)| a.equals(addr)).map(|(_, g)| *g).unwrap_or(UNASSIGNED_RAKNET_GUID)
 }

 /// `RakPeer::GetSystemIndexFromGuid` (IDA 0xa63574): -1 for
 /// unassigned or own guids, else the hint-then-scan match.
 #[must_use]
 pub fn system_index_from_guid(
 remotes: &[(u64, SystemAddress)],
 guid: u64,
 unassigned: u64,
 own_guid: u64,
 hint: Option<usize>,
 ) -> i32 {
 if guid == unassigned || guid == own_guid {
 return -1;
 }
 if let Some(h) = hint {
 if let Some((g, _)) = remotes.get(h) {
 if *g == guid {
 return h as i32;
 }
 }
 }
 remotes.iter().position(|(g, _)| *g == guid).map_or(-1, |i| i as i32)
 }

 /// `RakPeer::GetSystemAddressFromGuid` (IDA 0xa63620): unassigned
 /// for unassigned input, the bound address for the own guid, else
 /// the hint-then-scan match.
 #[must_use]
 pub fn system_address_from_guid(
 guid: u64,
 unassigned_guid: u64,
 own_guid: u64,
 own_bound: SystemAddress,
 unassigned_addr: SystemAddress,
 remotes: &[(u64, SystemAddress)],
 hint: Option<usize>,
 ) -> SystemAddress {
 if guid == unassigned_guid {
 return unassigned_addr;
 }
 if guid == own_guid {
 return own_bound;
 }
 if let Some(h) = hint {
 if let Some((g, a)) = remotes.get(h) {
 if *g == guid {
 return *a;
 }
 }
 }
 remotes.iter().find(|(g, _)| *g == guid).map(|(_, a)| *a).unwrap_or(unassigned_addr)
 }

 /// `RakPeer::SetTimeoutTime` (IDA 0xa63754): unassigned addresses
 /// set the default and fan out to every active system; otherwise the
 /// matching system is set engine-side.
 pub fn set_timeout_time(
 &mut self,
 addr: &SystemAddress,
 unassigned: &SystemAddress,
 ms: u32,
 apply_all: &mut dyn FnMut(),
 apply_one: &mut dyn FnMut(),
 ) {
 if addr.equals(unassigned) {
 self.default_timeout_ms = ms;
 apply_all();
 } else {
 apply_one();
 }
 }

 /// `RakPeer::GetTimeoutTime` (IDA 0xa63844): the matching system's
 /// reliability timeout, else the default.
 #[must_use]
 pub fn timeout_time(
 addr: &SystemAddress,
 unassigned: &SystemAddress,
 default_ms: u32,
 slot: Option<u32>,
 ) -> u32 {
 if !addr.equals(unassigned) {
 if let Some(t) = slot {
 return t;
 }
 }
 default_ms
 }

 /// `RakPeer::GetMTUSize` (IDA 0xa638fc): the matching active
 /// system's MTU, else the peer default.
 #[must_use]
 pub fn mtu_size(matched: Option<u32>, default_mtu: u32) -> u32 {
 matched.unwrap_or(default_mtu)
 }

 /// `RakPeer::GetNumberOfAddresses` (IDA 0xa639b4): the local address
 /// count.
 #[must_use]
 pub fn number_of_addresses(locals: &[SystemAddress]) -> usize {
 locals.len()
 }

 /// `RakPeer::GetLocalIP` (IDA 0xa639e4): the indexed local address
 /// dotted (the engine refreshes via `GetMyIP` when inactive).
 #[must_use]
 pub fn local_ip(locals: &[SystemAddress], index: usize) -> String {
 locals.get(index).map(|a| a.dotted()).unwrap_or_default()
 }

 /// `RakPeer::IsLocalIP` (IDA 0xa63a28): loopback names or membership
 /// in the local list.
 #[must_use]
 pub fn is_local_ip(addr: &str, locals: &[String]) -> bool {
 if addr.is_empty() {
 return false;
 }
 if addr == "127.0.0.1" || addr == "localhost" {
 return true;
 }
 locals.iter().any(|l| l == addr)
 }

 /// `RakPeer::AllowConnectionResponseIPMigration` (IDA 0xa63aa8).
 pub fn allow_connection_response_ip_migration(&mut self, allow: bool) {
 self.allow_ip_migration = allow;
 }

 /// `RakPeer::SetSplitMessageProgressInterval` (IDA 0xa63bd8).
 pub fn set_split_message_progress_interval(&mut self, interval: i32) {
 self.split_message_progress_interval = interval;
 }

 /// `RakPeer::GetSplitMessageProgressInterval` (IDA 0xa63c14).
 #[must_use]
 pub fn split_message_progress_interval(&self) -> i32 {
 self.split_message_progress_interval
 }

 /// `RakPeer::SetUnreliableTimeout` (IDA 0xa63c1c).
 pub fn set_unreliable_timeout(&mut self, ms: u32) {
 self.unreliable_timeout_ms = ms;
 }

 /// `RakPeer::SendTTL` (IDA 0xa63c58): resolves the host and emits a
 /// two-byte TTL probe when present.
 pub fn send_ttl(host: Option<&str>, send: &mut dyn FnMut()) {
 if host.is_some() {
 send();
 }
 }

 /// `RakPeer::AttachPlugin` (IDA 0xa63cf8): already-attached plugins
 /// report their 1-based position; otherwise the attach hook runs and
 /// the id is appended, returning the new count.
 pub fn attach_plugin(&mut self, id: u32, alt: bool, on_attach: &mut dyn FnMut()) -> u32 {
 let list = if alt { &mut self.plugins_alt } else { &mut self.plugins };
 if let Some(pos) = list.iter().position(|&p| p == id) {
 return (pos + 1) as u32;
 }
 on_attach();
 list.push(id);
 list.len() as u32
 }

 /// `RakPeer::DetachPlugin` (IDA 0xa63e54): swap-removes a present id
 /// and runs the detach hook for non-null plugins.
 pub fn detach_plugin(&mut self, id: Option<u32>, alt: bool, on_detach: &mut dyn FnMut()) {
 if let Some(id) = id {
 let list = if alt { &mut self.plugins_alt } else { &mut self.plugins };
 if let Some(pos) = list.iter().position(|&p| p == id) {
 list.swap_remove(pos);
 }
 on_detach();
 }
 }

/// `RakPeer::PushBackPacket` (IDA 0xa63ed8): plugin hooks first, then
/// the packet queues at the head or tail when present.
pub fn push_back_packet(present: bool, at_head: bool, hooks: &mut dyn FnMut(), push: &mut dyn FnMut(bool)) {
if present {
hooks();
push(at_head);
}
}

 /// `RakPeer::ApplyNetworkSimulator` (IDA 0xa64564): empty (the
 /// simulator is compiled out of this build).
 pub fn apply_network_simulator() {}

 /// `RakPeer::SetPerConnectionOutgoingBandwidthLimit` (IDA 0xa64568).
 pub fn set_per_connection_bandwidth_limit(&mut self, limit: u32) {
 self.per_connection_bandwidth_limit = limit;
 }

 /// `RakPeer::IsNetworkSimulatorActive` (IDA 0xa64570): hardcoded 0.
 #[must_use]
 pub fn is_network_simulator_active() -> bool {
 false
 }

 /// `RakPeer::IsActive` (IDA 0xa6c4ec).
 #[must_use]
 pub fn is_active(&self) -> bool {
 self.active
 }

 /// `RakPeer::WriteOutOfBandHeader` (IDA 0xa64574): the id byte 13,
 /// the peer guid, and the 16 offline-message bytes.
 pub fn write_out_of_band_header(stream: &mut BitStream, guid: u64) {
 stream.write_u8(13);
 stream.write_u64(guid);
 stream.write_aligned_bytes(&OFFLINE_MESSAGE_DATA_ID);
 }

 /// `RakPeer::GetStatistics` (IDA 0xa647f4): unassigned addresses
 /// aggregate every active system; a specific address fills when found.
 /// Layer reads stay engine-side.
 pub fn get_statistics(
 unassigned: bool,
 aggregate: &mut dyn FnMut(),
 single: &mut dyn FnMut() -> bool,
 ) -> bool {
 if unassigned {
 aggregate();
 true
 } else {
 single()
 }
 }

 /// `RakPeer::GetStatistics(int)` (IDA 0xa64b78): the indexed active
 /// system's stats; `false` out of range or inactive.
 pub fn get_statistics_index(active: &[bool], index: usize, fill: &mut dyn FnMut()) -> bool {
 if active.get(index).copied().unwrap_or(false) {
 fill();
 true
 } else {
 false
 }
 }

 /// `RakPeer::GetReceiveBufferSize` (IDA 0xa64bb4): ring occupancy
 /// from head, tail, and capacity.
 #[must_use]
 pub fn receive_buffer_size(head: u32, tail: u32, capacity: u32) -> u32 {
 if head <= tail {
 tail - head
 } else {
 tail.wrapping_sub(head).wrapping_add(capacity)
 }
 }

 /// `RakPeer::ReferenceRemoteSystem` (IDA 0xa65974): rebinds the slot
 /// to the address; the lookup-hash maintenance stays engine-side.
 pub fn reference_remote_system(slots: &mut Vec<SystemAddress>, index: usize, addr: SystemAddress) {
 if let Some(slot) = slots.get_mut(index) {
 *slot = addr;
 }
 }
 /// `RakPeer::RemoteSystemStruct::RemoteSystemStruct` (IDA 0xa6d194):
 /// member init stays engine-side.
 pub fn init_remote_system() {}
 /// `RakPeer::SetUserUpdateThread` (IDA 0xa645b0): stores the two
 /// callback words.
 pub fn set_user_update_thread(&mut self, callback: u32, data: u32) {
 self.user_update_thread = callback;
 self.user_update_thread_data = data;
 }
 /// `RakNet::RakPeer::Connect` result codes (IDA 0xa5f3d8): started,
 /// bad parameter, unresolvable host, already connected, or a queued
 /// duplicate.
 pub const CONNECT_STARTED: u32 = 0;
 pub const CONNECT_INVALID_PARAMETER: u32 = 1;
 pub const CONNECT_CANNOT_RESOLVE: u32 = 2;
 pub const CONNECT_ALREADY_CONNECTED: u32 = 3;
 pub const CONNECT_ALREADY_IN_PROGRESS: u32 = 4;
 /// `RakPeer::Connect` (IDA 0xa5f3d8): a missing host, a halted peer
 /// (+4), or a socket index past the socket count reports 1; the
 /// 0xa5f404 socket-index scan falls through either way, so the valid
 /// case forwards to `send_request` and returns its code.
 pub fn connect(host: Option<&str>, halted: bool, socket_count: usize, socket_index: usize, send_request: &mut dyn FnMut() -> u32) -> u32 {
 if host.is_none() || halted || socket_index >= socket_count {
 return Self::CONNECT_INVALID_PARAMETER;
 }
 send_request()
 }
 /// Password byte cap shared by the connect paths (IDA 0xa5f7c6): at
 /// most 255 bytes, and none without data.
 pub fn capped_password_len(password: Option<&[u8]>) -> usize {
 password.map_or(0, |p| p.len().min(255))
 }
 /// `RakNet::RakPeer::SendConnectionRequest` queue gate (IDA
 /// 0xa5f460/0xa5f8cc): an unresolvable host reports 2, an active
 /// remote match reports 3, a queued duplicate reports 4, else the
 /// request is enqueued and 0 reports started.
 pub fn queue_connection_request(addr: Option<SystemAddress>, connected_active: bool, queued: &[SystemAddress], mut request: RequestedConnection, enqueue: &mut dyn FnMut(RequestedConnection)) -> u32 {
 let Some(addr) = addr else { return Self::CONNECT_CANNOT_RESOLVE; };
 request.addr = addr;
 if connected_active {
 return Self::CONNECT_ALREADY_CONNECTED;
 }
 if queued.iter().any(|q| *q == addr) {
 return Self::CONNECT_ALREADY_IN_PROGRESS;
 }
 enqueue(request);
 Self::CONNECT_STARTED
 }
 /// `RakNet::RakPeer::ConnectWithSocket` (IDA 0xa5f754): a missing
 /// host, a halted peer, or no socket reports 1; else the capped
 /// password goes out with the send and the socket ref is released.
 pub fn connect_with_socket(host: Option<&str>, halted: bool, has_socket: bool, password: Option<&[u8]>, request: RequestedConnection, queued: &[SystemAddress], connected_active: bool, resolve: &mut dyn FnMut(&str) -> Option<SystemAddress>, enqueue: &mut dyn FnMut(RequestedConnection), release_socket: &mut dyn FnMut()) -> u32 {
 if host.is_none() || halted || !has_socket {
 return Self::CONNECT_INVALID_PARAMETER;
 }
 let mut request = request;
 request.password_len = Self::capped_password_len(password);
 let rc = Self::queue_connection_request(host.and_then(|h| resolve(h)), connected_active, queued, request, enqueue);
 release_socket();
 rc
 }
 /// `RakNet::RakPeer::NotifyAndFlagForShutdown` (IDA 0xa60494): an
 /// ID_DISCONNECTION_NOTIFICATION (21) byte goes out immediate (plus
 /// the remote-slot flag) or buffered.
 pub fn notify_and_flag_for_shutdown(immediate: bool, send: &mut dyn FnMut(u8, bool), flag_remote: &mut dyn FnMut()) {
 if immediate {
 send(21, true);
 flag_remote();
 } else {
 send(21, false);
 }
 }
 /// `RakNet::RakPeer::Send` routing (IDA 0xa60af8/0xa60f00):
 /// broadcasts and foreign targets buffer, the peer's own addresses loop
 /// back locally, and an unroutable (unassigned) target drops to none.
 pub fn send_route(broadcast: bool, guid: Option<u64>, own_guid: u64, addr: &SystemAddress, unassigned: &SystemAddress, locals: &[SystemAddress], bound: &SystemAddress) -> Option<SendTarget> {
 if broadcast {
 return Some(SendTarget::Broadcast);
 }
 if let Some(g) = guid {
 if g != own_guid {
 return Some(SendTarget::Remote);
 }
 } else {
 if addr == unassigned {
 return None;
 }
 if locals.iter().any(|l| l == addr) {
 return Some(SendTarget::Loopback);
 }
 if addr != bound {
 return Some(SendTarget::Remote);
 }
 }
 Some(SendTarget::Loopback)
 }
 /// overload (IDA 0xa60f00): missing data, a halted peer, or an
 /// unroutable target reports 0; else the routed write goes out under
 /// the override receipt (or the next one) with a receipt echo on
 /// loopback sends of priority >= 5.
 pub fn send_packet(data: Option<&[u8]>, peer_ready: bool, receipt_override: Option<u32>, next_receipt: &mut dyn FnMut() -> u32, route: Option<SendTarget>, priority: u8, dispatch: &mut dyn FnMut(SendTarget, &[u8], u32, Option<u32>)) -> u32 {
 let Some(data) = data else { return 0; };
 let Some(route) = route else { return 0; };
 if !peer_ready {
 return 0;
 }
 let receipt = receipt_override.unwrap_or_else(|| next_receipt());
 let echo = (route == SendTarget::Loopback && priority >= 5).then_some(receipt);
 dispatch(route, data, receipt, echo);
 receipt
 }
 /// `RakNet::RakPeer::SendBuffered` (IDA 0xa60cac): pool-allocates a
 /// send command, copies `(bits + 7) / 8` bytes, queues it, and kicks
 /// the update event for priority 0.
 pub fn send_buffered(data: &[u8], bit_len: u32, priority: u8, reliability: u8, channel: u8, guid: u64, addr: SystemAddress, broadcast: bool, mode: u32, receipt: u64, enqueue: &mut dyn FnMut(BufferedCommand), signal: &mut dyn FnMut()) {
 let bytes = ((bit_len as usize) + 7) / 8;
 let mut buf = vec![0u8; bytes];
 let n = bytes.min(data.len());
 buf[..n].copy_from_slice(&data[..n]);
 enqueue(BufferedCommand { kind: BufferedCommandKind::Send, bit_len, priority, reliability, channel, broadcast, receipt, guid, addr, data: buf, mode });
 if priority == 0 {
 signal();
 }
 }
 /// `RakNet::RakPeer::SendList` (IDA 0xa610c4): an empty list, a
 /// halted peer, or an unroutable target reports 0; else the parts go
 /// to `SendBufferedList` under the override receipt (or the next one).
 pub fn send_list(parts: &[&[u8]], peer_ready: bool, target_valid: bool, receipt_override: Option<u32>, next_receipt: &mut dyn FnMut() -> u32, buffered_list: &mut dyn FnMut(&[&[u8]], u32)) -> u32 {
 if parts.is_empty() || !peer_ready || !target_valid {
 return 0;
 }
 let receipt = receipt_override.unwrap_or_else(|| next_receipt());
 buffered_list(parts, receipt);
 receipt
 }
 /// `RakNet::RakPeer::SendBufferedList` (IDA 0xa611b8): concatenates
 /// the parts (an empty total is dropped); loopback targets bypass the
 /// queue while the rest queue a send command, kicking the update
 /// event for priority 0.
 pub fn send_buffered_list(parts: &[&[u8]], route: SendTarget, priority: u8, reliability: u8, channel: u8, guid: u64, addr: SystemAddress, broadcast: bool, mode: u32, receipt: u64, enqueue: &mut dyn FnMut(BufferedCommand), loopback: &mut dyn FnMut(Vec<u8>), signal: &mut dyn FnMut()) {
 let total: usize = parts.iter().map(|p| p.len()).sum();
 if parts.is_empty() || total == 0 {
 return;
 }
 let mut buf = Vec::with_capacity(total);
 for p in parts {
 buf.extend_from_slice(p);
 }
 if route == SendTarget::Loopback {
 loopback(buf);
 return;
 }
 enqueue(BufferedCommand { kind: BufferedCommandKind::Send, bit_len: (total * 8) as u32, priority, reliability, channel, broadcast, receipt, guid, addr, data: buf, mode });
 if priority == 0 {
 signal();
 }
 }
 /// `RakNet::RakPeer::ShiftIncomingTimestamp` (IDA 0xa61520): reads
 /// the leading timestamp and re-bases it by the lowest live ping
 /// sample (slots read `0xFFFF` are skipped); an unknown remote leaves
 /// the stamp alone.
 pub fn shift_incoming_timestamp(stamp: u64, ping_samples: &[u16]) -> u64 {
 let floor = ping_samples.iter().take(5).filter(|s| **s != 0xFFFF).min().copied().unwrap_or(0);
 stamp.wrapping_sub(u64::from(floor))
 }
 /// `RakNet::RakPeer::CallPluginCallbacks` slot select (IDA 0xa61698):
 /// disconnect-family ids run the +40 hook, 0x10/0x13 the +36 hook,
 /// 0x15/0x16 the +32 hook; anything else skips the plugins.
 pub fn plugin_callback_slot(message_id: u8) -> Option<u32> {
 match message_id {
 0x0A | 0x0B | 0x0C | 0x11 | 0x12 | 0x14 | 0x17 | 0x18 | 0x19 | 0x1A => Some(40),
 0x10 | 0x13 => Some(36),
 0x15 | 0x16 => Some(32),
 _ => None,
 }
 }
 /// `RakNet::RakPeer::CallPluginCallbacks` (IDA 0xa61698): invokes
 /// `call` per plugin with the slot for the message id.
 pub fn call_plugin_callbacks(message_id: u8, plugin_count: usize, call: &mut dyn FnMut(usize, u32)) -> usize {
 let Some(slot) = Self::plugin_callback_slot(message_id) else { return plugin_count; };
 for i in 0..plugin_count {
 call(i, slot);
 }
 plugin_count
 }
 /// `RakNet::RakPeer::CloseConnection` (IDA 0xa6188c): closes first;
 /// the local disconnect packet (byte 22) is queued only when no
 /// remote notify was asked for and the slot still reads connected.
 pub fn close_connection(send_notification: bool, remote_connected: bool, close_internal: &mut dyn FnMut(bool), push_packet: &mut dyn FnMut(u8)) {
 close_internal(send_notification);
 if !send_notification && remote_connected {
 push_packet(22);
 }
 }
 /// `RakNet::RakPeer::CloseConnectionInternal` (IDA 0xa61a8c): a bad
 /// target or a halted peer is a no-op; notify runs the immediate
 /// path, an immediate close tears the slot down, else a close
 /// command is queued.
 pub fn close_connection_internal(target_valid: bool, peer_ready: bool, notify: bool, immediate: bool, addr: SystemAddress, channel: u8, priority: u8, notify_now: &mut dyn FnMut(), drop_now: &mut dyn FnMut(), queue_close: &mut dyn FnMut(BufferedCommand)) {
 if !target_valid || !peer_ready {
 return;
 }
 if notify {
 notify_now();
 } else if immediate {
 drop_now();
 } else {
 queue_close(BufferedCommand { kind: BufferedCommandKind::Close, addr, channel, priority, ..BufferedCommand::default() });
 }
 }
 /// `RakNet::RakPeer::GetSystemList` (IDA 0xa624a4): the addresses
 /// and guids of active slots in state 7.
 pub fn system_list(remotes: &[(SystemAddress, u64, bool, u32)]) -> (Vec<SystemAddress>, Vec<u64>) {
 let mut addrs = Vec::new();
 let mut guids = Vec::new();
 for (addr, guid, active, state) in remotes {
 if *active && *state == 7 {
 addrs.push(*addr);
 guids.push(*guid);
 }
 }
 (addrs, guids)
 }
 /// `RakNet::RakPeer::GetRemoteSystemFromSystemAddress` (IDA 0xa63140):
 /// unassigned never matches; an active hit wins, else the first
 /// inactive hit unless `active_only`. The hashed-vs-linear strategy
 /// split stays engine-side.
 pub fn remote_system_from_address(remotes: &[(SystemAddress, bool)], addr: &SystemAddress, unassigned: &SystemAddress, active_only: bool) -> Option<usize> {
 if addr == unassigned {
 return None;
 }
 let mut inactive = None;
 for (i, (a, active)) in remotes.iter().enumerate() {
 if a == addr {
 if *active {
 return Some(i);
 }
 if inactive.is_none() {
 inactive = Some(i);
 }
 }
 }
 if active_only { None } else { inactive }
 }
 /// `RakNet::RakPeer::GetClientPublicKeyFromSystemAddress` (IDA
 /// 0xa63750): hardcoded 0, security is compiled out.
 pub fn client_public_key() -> u32 {
 0
 }
 /// `RakNet::RakPeer::AdvertiseSystem` packet (IDA 0xa63ab0): the ID
 /// 29 byte plus the aligned payload; the send itself stays
 /// engine-side behind `send`.
 pub fn advertise_packet(data: &[u8]) -> Vec<u8> {
 let mut out = Vec::with_capacity(1 + data.len());
 out.push(29);
 out.extend_from_slice(data);
 out
 }
 /// `RakNet::RakPeer::GetSocket` query (IDA 0xa6410c): queues a
 /// kind-2 command, then spins up to +1000ms on the socket-query
 /// output for the first socket; a halted peer or a timeout reports
 /// none.
 pub fn query_socket(addr: SystemAddress, enqueue: &mut dyn FnMut(BufferedCommand), now_ms: &mut dyn FnMut() -> u32, sleep: &mut dyn FnMut(), poll: &mut dyn FnMut() -> Option<u32>, halted: &mut dyn FnMut() -> bool) -> Option<u32> {
 enqueue(BufferedCommand { kind: BufferedCommandKind::QuerySocket, addr, ..BufferedCommand::default() });
 let deadline = now_ms().wrapping_add(1000);
 loop {
 if now_ms() >= deadline || halted() {
 return None;
 }
 sleep();
 if let Some(sock) = poll() {
 return Some(sock);
 }
 }
 }
 /// `RakNet::RakPeer::GetSockets` (IDA 0xa643c8): clears the out
 /// list, queues a kind-2 command, and takes the first socket-query
 /// output while the peer stays up.
 pub fn get_sockets(queue_query: &mut dyn FnMut(), alive: &mut dyn FnMut() -> bool, sleep: &mut dyn FnMut(), poll: &mut dyn FnMut() -> Option<Vec<u32>>) -> Vec<u32> {
 queue_query();
 while alive() {
 sleep();
 if let Some(sockets) = poll() {
 return sockets;
 }
 }
 Vec::new()
 }
 /// `RakNet::RakPeer::ReleaseSockets` (IDA 0xa64540): deletes the
 /// array and zeroes the list.
 pub fn release_socket_list(sockets: &mut Vec<u32>) {
 *sockets = Vec::new();
 }
 /// `RakNet::RakPeer::SendOutOfBand` (IDA 0xa645bc): the offline
 /// header plus payload goes through the indexed socket after the
 /// direct-send plugin hooks; 1 on send.
 pub fn send_out_of_band(host: Option<&str>, peer_active: bool, socket_found: bool, header: &[u8], data: &[u8], notify_plugins: &mut dyn FnMut(), send_to: &mut dyn FnMut(&[u8])) -> u32 {
 let Some(host) = host else { return 0; };
 if host.is_empty() || !peer_active || !socket_found {
 return 0;
 }
 notify_plugins();
 let mut packet = Vec::with_capacity(header.len() + data.len());
 packet.extend_from_slice(header);
 packet.extend_from_slice(data);
 send_to(&packet);
 1
 }
 /// `RakNet::RakPeer::ParseConnectionRequestPacket` (IDA 0xa64be8):
 /// skips the message id, reads the guid plus receipt plus the
 /// password tail; an exact password match accepts (state 5) through
 /// `OnConnectionRequest`, else a refusal goes out (state 2). The
 /// parsed guid is unused downstream.
 pub fn parse_connection_request_packet(packet: &[u8], incoming_password: &[u8], on_accept: &mut dyn FnMut(u64), on_refuse: &mut dyn FnMut()) -> u32 {
 let mut ok = false;
 let mut receipt = 0u64;
 if packet.len() >= 18 {
 let len = packet[17] as usize;
 if packet.len() >= 18 + len {
 let mut bytes = [0u8; 8];
 bytes.copy_from_slice(&packet[9..17]);
 receipt = u64::from_be_bytes(bytes);
 ok = incoming_password == &packet[18..18 + len];
 }
 }
 if ok {
 on_accept(receipt);
 5
 } else {
 on_refuse();
 2
 }
 }
 /// `RakNet::RakPeer::SendImmediate` target select (IDA 0xa64e48): a
 /// direct send needs the resolved slot active and in state 1..=3; a
 /// broadcast collects every other active assigned slot (every slot
 /// when nothing resolves).
 pub fn send_immediate_targets(broadcast: bool, target: Option<usize>, assigned: &[bool], slot_active: &[bool], slot_state: &[u32]) -> Vec<usize> {
 if !broadcast {
 return match target {
 Some(i) if slot_active.get(i) == Some(&true) && slot_state.get(i).map_or(false, |s| (1..=3).contains(s)) => vec![i],
 _ => Vec::new(),
 };
 }
 assigned.iter().enumerate().filter(|(i, a)| **a && slot_active.get(*i) == Some(&true) && Some(*i) != target).map(|(i, _)| i).collect()
 }
 /// `RakNet::ReliabilityLayer` timeout-stamp predicate (IDA 0xa64fd8):
 /// the reliable/sequenced/ack flavors stamp the slot send time.
 pub fn send_immediate_resets_timeout(reliability: u8) -> bool {
 reliability == 6 || (2..=4).contains(&reliability) || reliability == 7
 }
 /// `RakNet::RakPeer::SendImmediate` fan-out (IDA 0xa64e48): one
 /// reliability write per target with the receipt flag on the final
 /// write and the timeout stamp on reliable flavors; true when
 /// anything went out.
 pub fn send_immediate(targets: &[usize], reliability: u8, stamp_us: u64, send: &mut dyn FnMut(usize, bool, Option<u32>)) -> bool {
 if targets.is_empty() {
 return false;
 }
 let stamp = Self::send_immediate_resets_timeout(reliability).then_some((stamp_us / 1000) as u32);
 let last = targets.len() - 1;
 for (n, t) in targets.iter().enumerate() {
 send(*t, n == last, stamp);
 }
 true
 }
 /// `RakNet::RakPeer::OnConnectionRequest` reply (IDA 0xa651fc): the
 /// ID_CONNECTION_REQUEST_ACCEPTED (16) packet — version plus the
 /// complemented address and network-order port per IPv4 entry, the
 /// slot index, our guid, and the send time — for `SendImmediate`.
 pub fn connection_request_accepted_packet(index: u16, remote: &SystemAddress, locals: &[SystemAddress], guid: u64, time_us: u64) -> Vec<u8> {
 fn write_entry(stream: &mut crate::bitstream::BitStream, addr: &SystemAddress) {
 stream.write_u8(addr.ip_version() as u8);
 if addr.ip_version() == 4 {
 stream.write_u32(!addr.binary_address());
 stream.write_u16(addr.port_network_order());
 }
 }
 let mut stream = crate::bitstream::BitStream::new();
 stream.write_u8(16);
 write_entry(&mut stream, remote);
 stream.write_u16(index);
 for i in 0..10 {
 write_entry(&mut stream, locals.get(i).copied().as_ref().unwrap_or(remote));
 }
 stream.write_u64(guid);
 stream.write_u64(time_us);
 stream.into_bytes()
 }
}
 /// `RakNet::RakPeer::BufferedCommandStruct` command word at +100
 /// (IDA 0xa61d98 close, 0xa63fc8 address-change, 0xa641cc
 /// socket-query): what the queued write asks the update loop for.
 #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
 pub enum BufferedCommandKind {
 #[default]
 Send,
 Close,
 QuerySocket,
 ChangeAddress,
 }
 /// `RakNet::RakPeer::BufferedCommandStruct` (IDA 0xa60cac): a queued
 /// peer write with its reliability framing and target.
 #[derive(Clone, Debug, Default)]
 pub struct BufferedCommand {
 pub kind: BufferedCommandKind,
 pub bit_len: u32,
 pub priority: u8,
 pub reliability: u8,
 pub channel: u8,
 pub broadcast: bool,
 pub receipt: u64,
 pub guid: u64,
 pub addr: SystemAddress,
 pub data: Vec<u8>,
 pub mode: u32,
 }
 /// `RakNet::RakPeer::RequestedConnectionStruct` (IDA 0xa5f460): a
 /// queued connection attempt; the password is capped at 255 bytes
 /// (IDA 0xa5f7c6).
 #[derive(Clone, Debug, Default)]
 pub struct RequestedConnection {
 pub addr: SystemAddress,
 pub password_len: usize,
 pub socket_index: u32,
 pub send_count: u32,
 pub timeout_ms: u32,
 pub extra_timeout_ms: u32,
 pub use_socket: bool,
 }
 /// `RakNet::RakPeer::Send` routing verdict (IDA 0xa60af8/0xa60f00).
 #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
 pub enum SendTarget {
 #[default]
 Remote,
 Broadcast,
 Loopback,
 }

/// `RakNet::RakString::IPAddressMatch` (IDA 0xa6f1ac): walks both
/// strings while equal; a `*` in the pattern at the first difference
/// matches the rest (both sides must be non-empty there). Empty or
/// over-15-char inputs never match.
#[must_use]
pub fn ip_address_match(pattern: &str, addr: &str) -> bool {
 if addr.is_empty() || addr.len() > 0xF {
 return false;
 }
 let (pb, ab) = (pattern.as_bytes(), addr.as_bytes());
 let mut i = 0;
 loop {
 let p = pb.get(i).copied().unwrap_or(0);
 let a = ab.get(i).copied().unwrap_or(0);
 if p != a {
 return p != 0 && a != 0 && p == b'*';
 }
 if p == 0 {
 return true;
 }
 i += 1;
 }
}

/// `RakNet::SocketLayer::IsPortInUse` (IDA 0xa7a700): open an `AF_INET`
/// datagram socket (`socket` at 0xa7a736) and bind it to `host:port`
/// (`htons` via `REV16` at 0xa7a71a). A failed bind means the port is taken.
#[must_use]
pub fn is_port_in_use(port: u16, host: &str) -> bool {
    std::net::UdpSocket::bind((host, port)).is_err()
}

#[cfg(test)]
mod port_tests {
    use super::*;

    #[test]
    fn port_probe_agrees_with_bind() {
        // IDA 0xa7a700: an unbound ephemeral port binds fine, a bound one fails.
        let socket = std::net::UdpSocket::bind(("127.0.0.1", 0)).expect("ephemeral");
        let port = socket.local_addr().expect("addr").port();
        assert!(is_port_in_use(port, "127.0.0.1"));
        drop(socket);
        assert!(!is_port_in_use(port, "127.0.0.1"));
    }
}

#[cfg(test)]
mod socket_layer_tests {
    use super::*;

    #[test]
    fn bind_send_recv_loopback() {
        // IDA 0xa7a898/0xa7a944/0xa7a8d0: bound pair exchanges a datagram.
        let rx = create_bound_socket(0, true, "127.0.0.1").expect("bind rx");
        let tx = create_bound_socket_old(0, true, "127.0.0.1").expect("bind tx");
        let addr = get_system_address(&rx).expect("local addr");
        assert_eq!(send_to(&tx, b"ping", addr), 4);
        let mut buf = [0u8; 8];
        let (len, from) = recv_from_blocking(&rx, &mut buf, 2000).expect("recv");
        assert_eq!((len, &buf[..len]), (4, b"ping".as_slice()));
        assert_eq!(from.port(), get_system_address(&tx).expect("tx addr").port());
        // IDA 0xa7a9ec: TTL send reaches the same socket.
        assert_eq!(send_to_ttl(&tx, b"q", addr, 64), 1);
        let (len, _) = recv_from_blocking(&rx, &mut buf, 2000).expect("recv ttl");
        assert_eq!(len, 1);
    }

    #[test]
    fn resolve_and_self_lookup() {
        // IDA 0xa7a8ac: localhost resolves to loopback.
        let ip = domain_name_to_ip("localhost").expect("resolve");
        let ip: std::net::IpAddr = ip.parse().expect("ip string");
        assert!(ip.is_loopback());
        assert!(domain_name_to_ip("nonexistent.invalid.xyz").is_none());
        // IDA 0xa7aae0/0xa7abd8: self lookup yields some interface address.
        assert!(get_my_ip_linux().is_some());
        assert!(get_my_ip().is_some());
        // IDA 0xa7a788: fragment-flag stub runs.
        set_do_not_fragment();
    }
}

/// `RakNet::SocketLayer::SetDoNotFragment` (IDA 0xa7a788): sets
/// `IP_MTU_DISCOVER`/`IP_DONTFRAG` on a raw fd via `setsockopt`, which has
/// no `std` equivalent; no-op Rust-side.
pub fn set_do_not_fragment() {}

/// `RakNet::SocketLayer::CreateBoundSocket_Old` (IDA 0xa7a78c): bind a
/// datagram socket to `host:port`, honouring the blocking flag.
pub fn create_bound_socket_old(
    port: u16,
    blocking: bool,
    host: &str,
) -> std::io::Result<std::net::UdpSocket> {
    // IDA 0xa7a78c: the old spelling shares the bind path; socket options
    // stay engine-side.
    create_bound_socket(port, blocking, host)
}

/// `RakNet::SocketLayer::CreateBoundSocket` (IDA 0xa7a898): bind a
/// datagram socket to `host:port`, honouring the blocking flag.
pub fn create_bound_socket(
    port: u16,
    blocking: bool,
    host: &str,
) -> std::io::Result<std::net::UdpSocket> {
    let socket = std::net::UdpSocket::bind((host, port))?;
    socket.set_nonblocking(!blocking)?;
    Ok(socket)
}

/// `RakNet::SocketLayer::DomainNameToIP` (IDA 0xa7a8ac): resolve a host to
/// its first IP string (the original returns a static buffer).
#[must_use]
pub fn domain_name_to_ip(host: &str) -> Option<String> {
    use std::net::ToSocketAddrs;
    (host, 0).to_socket_addrs().ok()?.next().map(|addr| addr.ip().to_string())
}

/// `RakNet::SocketLayer::RecvFromBlocking` (IDA 0xa7a8d0): datagram receive
/// with a read-timeout budget; `None` on timeout or error.
#[must_use]
pub fn recv_from_blocking(
    socket: &std::net::UdpSocket,
    buf: &mut [u8],
    timeout_ms: u32,
) -> Option<(usize, std::net::SocketAddr)> {
    use std::time::Duration;
    socket.set_read_timeout(Some(Duration::from_millis(u64::from(timeout_ms)))).ok()?;
    socket.recv_from(buf).ok()
}

/// `RakNet::SocketLayer::SendTo` (IDA 0xa7a944): datagram send; sent bytes
/// (`0` on error, where the original returns the `sendto` result).
#[must_use]
pub fn send_to(
    socket: &std::net::UdpSocket,
    data: &[u8],
    addr: std::net::SocketAddr,
) -> usize {
    socket.send_to(data, addr).unwrap_or(0)
}

/// `RakNet::SocketLayer::SendToTTL` (IDA 0xa7a9ec): set the unicast TTL,
/// then send; sent bytes (`0` on error).
#[must_use]
pub fn send_to_ttl(
    socket: &std::net::UdpSocket,
    data: &[u8],
    addr: std::net::SocketAddr,
    ttl: u32,
) -> usize {
    if socket.set_ttl(ttl).is_err() {
        return 0;
    }
    socket.send_to(data, addr).unwrap_or(0)
}

/// `GetMyIP_Linux` (IDA 0xa7aae0): outward-facing IP via the UDP-connect
/// trick (no packets sent), portable beyond Linux despite the name.
#[must_use]
pub fn get_my_ip_linux() -> Option<std::net::IpAddr> {
    let socket = std::net::UdpSocket::bind(("0.0.0.0", 0)).ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}

/// `RakNet::SocketLayer::GetMyIP` (IDA 0xa7abd8): same outward-facing
/// lookup as [`get_my_ip_linux`].
#[must_use]
pub fn get_my_ip() -> Option<std::net::IpAddr> {
    get_my_ip_linux()
}

/// `RakNet::SocketLayer::GetSystemAddress` (IDA 0xa7abe4): bound local
/// address of a socket (`getsockname`).
#[must_use]
pub fn get_system_address(socket: &std::net::UdpSocket) -> Option<std::net::SocketAddr> {
    socket.local_addr().ok()
}
