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
}
