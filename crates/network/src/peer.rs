//! `RBX::Network::ConcurrentRakPeer` stats bookkeeping.
//!
//! Decompiled from `addStats` (IDA 0x999400), `removeStats` (IDA
//! 0x999f78), `GetBandwidthExceeded` (IDA 0x99b884), and
//! `GetCongestionControlExceeded` (IDA 0x99b990). The per-address
//! `ConnectionStats` map and its mutex stay engine-side; the lookup gates
//! live here.

#![allow(dead_code)]

/// `ConcurrentRakPeer::addStats` (IDA 0x999400): asserts the datamodel
/// wants writes engine-side, runs the stats update, emplaces the
/// connection entry, and stores the callback. Returns the update status.
pub fn concurrent_add_stats(register: &mut dyn FnMut() -> u32) -> u32 {
    register()
}

/// `ConcurrentRakPeer::removeStats` (IDA 0x999f78): erases both map
/// entries for the address under lock.
pub fn concurrent_remove_stats(erase: &mut dyn FnMut() -> u32) -> u32 {
    erase()
}

/// `ConcurrentRakPeer::GetBandwidthExceeded` (IDA 0x99b884): the flag at
/// +272 of the address entry; a missing entry reads back default (false).
#[must_use]
pub fn bandwidth_exceeded(present: bool, exceeded: bool) -> bool {
    present && exceeded
}

/// `ConcurrentRakPeer::GetCongestionControlExceeded` (IDA 0x99b990):
/// same shape at +316.
#[must_use]
pub fn congestion_control_exceeded(present: bool, exceeded: bool) -> bool {
    present && exceeded
}

/// `ConcurrentRakPeer::ConcurrentRakPeer` C1 (IDA 0x99a0fc) / C2 (IDA
/// 0x99a108): C1 delegates to C2; peer, mutex, and map init stays
/// engine-side.
pub fn init_concurrent_peer() {}

/// `ConcurrentRakPeer::Send` (IDA 0x99b398): the packet is packaged and
/// pushed to the send-job queue, then the scheduler reschedules.
pub fn concurrent_send(enqueue: &mut dyn FnMut(), reschedule: &mut dyn FnMut()) {
 enqueue();
 reschedule();
}

/// `StatsUpdateJob::updateStats` (IDA 0x99bb00): while the job handle is
/// live, the nested update runs when the reentrancy flag is set.
pub fn update_stats(job_active: bool, nested: bool, update: &mut dyn FnMut()) {
 if job_active && nested {
 update();
 }
}

/// Per-address `ConnectionStats` (IDA 0x99b884 +272, 0x99b990 +316).
#[derive(Clone, Copy, Debug, Default)]
pub struct ConnectionStats {
 pub bandwidth_exceeded: bool,
 pub congestion_exceeded: bool,
}

/// `table::operator[]` over the stats map (IDA 0x99c480): finds the
/// address entry or default-inserts it.
pub fn stats_entry(stats: &mut std::collections::HashMap<(u32, u16), ConnectionStats>, addr: (u32, u16)) -> &mut ConnectionStats {
 stats.entry(addr).or_default()
}

/// `table::reserve_for_insert` (IDA 0x99c890) and `create_buckets` (IDA
/// 0x99ca38): table sizing ahead of insertion.
pub fn stats_reserve_table(stats: &mut std::collections::HashMap<(u32, u16), ConnectionStats>, additional: usize) {
 stats.reserve(additional);
}

/// `node_constructor::construct_with_value` (IDA 0x99c6dc):
/// placement-constructs the map node; allocator-level, engine-side.
pub fn stats_node_construct() {}
/// `ConcurrentRakPeer::StatsUpdateJob` state built by its C2 ctor (IDA
/// 0x99e8f8): the `DataModelJob` base named `"Net Peer Stats"` at priority
/// 9 (IDA 0x99e9cc), the stats/callback maps sized through the
/// `prime_list` search for 10 buckets (IDA 0x99eb6e..0x99ec16), the live
/// flag at +612 (IDA 0x99ec56), and the retained `RakPeerInterface`
/// shared pointer at +620/+624 (IDA 0x99ec5c..0x99ecb8). Map storage and
/// the mutex stay engine-side; the reservation counts and flags live here.
#[derive(Clone, Debug, Default)]
pub struct StatsUpdateJob {
    pub job_name: &'static str,
    pub stats_reserve: usize,
    pub callbacks_reserve: usize,
    pub updates_enabled: bool,
    pub peer_retained: bool,
}

/// `StatsUpdateJob::StatsUpdateJob` (IDA 0x99e8f8).
pub fn stats_update_job_init(job: &mut StatsUpdateJob, retain_peer: &mut dyn FnMut()) {
    job.job_name = "Net Peer Stats"; // IDA 0x99e9cc
    job.stats_reserve = 10; // IDA 0x99eb6e..0x99ebb4
    job.callbacks_reserve = 10; // IDA 0x99ebd0..0x99ec16
    job.updates_enabled = true; // +612 = 1, IDA 0x99ec56
    retain_peer(); // shared peer copy, IDA 0x99ec5c..0x99ecb8
    job.peer_retained = true;
}

/// `ConcurrentRakPeer::PacketJob` state built by its C2 ctor (IDA
/// 0x99f9b8): the `DataModelJob` base named `"Net Peer Send"` at priority
/// 9 (IDA 0x99fa8c), the send deque plus its mutex at +520 (IDA
/// 0x99fc28..0x99fc54), the enable byte at +564 copied from the
/// `NetworkSettings` singleton word at +112 (IDA 0x99fc5c..0x99fc66),
/// and the retained `RakPeerInterface` shared pointer at +568/+572 (IDA
/// 0x99fc70..0x99fc78). Queue storage stays engine-side.
#[derive(Clone, Debug, Default)]
pub struct PacketJob {
    pub job_name: &'static str,
    pub send_enabled: bool,
    pub peer_retained: bool,
}

/// `PacketJob::PacketJob` (IDA 0x99f9b8).
pub fn packet_job_init(job: &mut PacketJob, settings_flag: bool, retain_peer: &mut dyn FnMut()) {
    job.job_name = "Net Peer Send"; // IDA 0x99fa8c
    job.send_enabled = settings_flag; // +564 = singleton +112, IDA 0x99fc66
    retain_peer(); // shared peer copy, IDA 0x99fc70..0x99fc78
    job.peer_retained = true;
}


/// `RBX::Network::Peer::encryptDataPart` (IDA 0xad36dc): AES-CBC encrypts
/// the stream bytes at `[1..bytes]` in place with the peer key
/// (`DataBlockEncryptor::SetKey` from `aesKey` at 0xad3752, `Encrypt` at
/// 0xad37ac) after padding the allocation to
/// `((8 * bytes + 160) & ~0x7f) + 8` bits (IDA 0xad3782), then sets the
/// write offset past the ciphertext (IDA 0xad37be). The cipher and key
/// schedule stay engine-side (`this + 112`); no-op Rust-side.
pub fn encrypt_data_part() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_gates() {
        // IDA 0x999400/0x999f78: callback status passes through.
        assert_eq!(concurrent_add_stats(&mut || 1), 1);
        assert_eq!(concurrent_remove_stats(&mut || 1), 1);
        // IDA 0x99b884/0x99b990: missing entries read false.
        assert!(!bandwidth_exceeded(false, true));
        assert!(bandwidth_exceeded(true, true));
        assert!(!bandwidth_exceeded(true, false));
        assert!(!congestion_control_exceeded(false, true));
        assert!(congestion_control_exceeded(true, true));
        assert!(!congestion_control_exceeded(true, false));
    }
    #[test]
    fn lifecycle_and_table_gates() {
        // IDA 0x99a0fc/0x99a108/0x99c6dc: init no-ops.
        init_concurrent_peer();
        stats_node_construct();
        // IDA 0x99b398: enqueue then reschedule.
        let order = std::cell::RefCell::new(Vec::new());
        concurrent_send(&mut || order.borrow_mut().push("q"), &mut || order.borrow_mut().push("sched"));
        assert_eq!(order.borrow().as_slice(), ["q", "sched"]);
        // IDA 0x99bb00: nested update only when live and flagged.
        let mut n = 0;
        update_stats(false, true, &mut || n += 1);
        update_stats(true, false, &mut || n += 1);
        update_stats(true, true, &mut || n += 1);
        assert_eq!(n, 1);
        // IDA 0x99c480/0x99c890/0x99ca38: entry and reserve.
        let mut map = std::collections::HashMap::new();
        stats_reserve_table(&mut map, 4);
        stats_entry(&mut map, (1, 2)).bandwidth_exceeded = true;
        assert!(map[&(1, 2)].bandwidth_exceeded);
        assert!(!stats_entry(&mut map, (3, 4)).congestion_exceeded);
    }
}
