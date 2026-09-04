//! `RakNet::ReliabilityLayer` packet pump bookkeeping.
//!
//! Decompiled from the ctors (IDA 0xa7092c/0xa70938), `InitializeVariables`
//! (IDA 0xa7142c), the dtors (IDA 0xa715f8/0xa71604), `Reset` (IDA
//! 0xa723c0), `SetTimeoutTime`/`GetTimeoutTime` (IDA 0xa723f8/0xa72400),
//! `FreeThreadSafeMemory` (IDA 0xa72408), `ClearPacketsAndDatagrams`
//! (IDA 0xa72d5c), and `Receive` (IDA 0xa74d64). Pools, windows, and the
//! resend tables stay engine-side; the timeout, the output queue, and the
//! lifecycle gates live here.

#![allow(dead_code)]

use std::collections::VecDeque;

/// `RakNet::ReliabilityLayer` reduced to its timeout and output queue.
#[derive(Clone, Debug, Default)]
pub struct ReliabilityLayer {
    /// Timeout ms at +2232 (IDA 0xa723f8).
    pub timeout_ms: u32,
    /// Output packet queue at +0..+19 (IDA 0xa74d64).
    pub output: VecDeque<Vec<u8>>,
}

impl ReliabilityLayer {
    /// Ctors (IDA 0xa7092c/0xa70938); full member init runs in
    /// [`init`](Self::init).
    pub fn new() -> Self {
        Self::default()
    }

    /// `InitializeVariables` (IDA 0xa7142c): zeroes the regions,
    /// including the timeout.
    pub fn init(&mut self) {
        *self = Self::default();
    }

    /// `Reset` (IDA 0xa723c0): frees thread-safe memory, then
    /// reinitializes (plus the sliding-window init engine-side) when
    /// `full` is set.
    pub fn reset(&mut self, full: bool) {
        self.output.clear();
        if full {
            self.init();
        }
    }

    /// `SetTimeoutTime` (IDA 0xa723f8).
    pub fn set_timeout_time(&mut self, ms: u32) {
        self.timeout_ms = ms;
    }

    /// `GetTimeoutTime` (IDA 0xa72400).
    #[must_use]
    pub fn timeout_time(&self) -> u32 {
        self.timeout_ms
    }

    /// `FreeThreadSafeMemory` (IDA 0xa72408): drains the output queue;
    /// pool releases stay engine-side.
    pub fn free_thread_safe_memory(&mut self) {
        self.output.clear();
    }

    /// `ClearPacketsAndDatagrams` (IDA 0xa72d5c): packet and datagram
    /// releases stay engine-side.
    pub fn clear_packets_and_datagrams(&mut self) {}
    /// `Receive` (IDA 0xa74d64): pops the output ring, returning the
    /// packet bytes (`None` when empty, where the original returns 0).
    #[must_use]
    pub fn receive_packet(&mut self) -> Option<Vec<u8>> {
        self.output.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_timeout_queue() {
        // IDA 0xa7092c/0xa7142c/0xa723c0: ctor, init, gated reset.
        let mut layer = ReliabilityLayer::new();
        layer.set_timeout_time(1000);
        layer.output.push_back(vec![1, 2]);
        layer.reset(false);
        assert_eq!((layer.timeout_time(), layer.output.len()), (1000, 0));
        layer.set_timeout_time(1000);
        layer.output.push_back(vec![1, 2]);
        layer.reset(true);
        assert_eq!((layer.timeout_time(), layer.output.len()), (0, 0));
        // IDA 0xa723f8/0xa72400/0xa72408/0xa72d5c: timeout and releases.
        layer.set_timeout_time(250);
        assert_eq!(layer.timeout_time(), 250);
        layer.output.push_back(vec![3]);
        layer.free_thread_safe_memory();
        assert!(layer.output.is_empty());
        layer.output.push_back(vec![3]);
        layer.clear_packets_and_datagrams();
        // IDA 0xa74d64: pop or nothing.
        assert_eq!(layer.receive_packet(), Some(vec![3]));
        assert_eq!(layer.receive_packet(), None);
    }
}
