//! `RakNet::GetTime` family — milliseconds/microseconds since first call.
//!
//! Decompiled from `GetTime` (IDA 0xa58844), `GetTimeUS` (IDA 0xa588c4),
//! and `GetTimeMS` (IDA 0xa58938): each lazily snapshots `gettimeofday`
//! once (`initialized` flag) and returns the elapsed time in its unit.

#![allow(dead_code)]

use std::sync::OnceLock;
use std::time::Instant;

fn epoch() -> Instant {
    static EPOCH: OnceLock<Instant> = OnceLock::new();
    *EPOCH.get_or_init(Instant::now)
}

/// `RakNet::GetTime` (IDA 0xa58844): elapsed milliseconds.
#[must_use]
pub fn raknet_time_ms() -> u64 {
    epoch().elapsed().as_millis() as u64
}

/// `RakNet::GetTimeUS` (IDA 0xa588c4): elapsed microseconds.
#[must_use]
pub fn raknet_time_us() -> u64 {
    epoch().elapsed().as_micros() as u64
}

/// `RakNet::GetTimeMS` (IDA 0xa58938): elapsed milliseconds, same clock
/// as [`raknet_time_ms`].
#[must_use]
pub fn raknet_time_millis() -> u64 {
    epoch().elapsed().as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clocks_advance_together() {
        // IDA 0xa58844/0xa588c4/0xa58938: shared epoch, us >= ms * 1000 scale.
        let ms = raknet_time_ms();
        let us = raknet_time_us();
        let ms2 = raknet_time_millis();
        assert!(ms2 >= ms);
        assert!(us >= ms * 1000);
    }
}
