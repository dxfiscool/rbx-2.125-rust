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
}

impl RakPeer {
 /// `RakPeer::RakPeer` (IDA 0xa5cb00).
 pub fn new() -> Self {
 Self::default()
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

 /// `RakPeer::AllocatePacket` (IDA 0xa6406c): a packet with a
 /// zeroed `size`-byte buffer, unassigned guid and address.
 #[must_use]
 pub fn allocate_packet(size: usize) -> Packet {
 Packet::default().with_data(vec![0u8; size])
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
