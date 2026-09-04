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

#[derive(Clone, Debug, Default)]
pub struct ReliabilityLayer {
    /// Timeout ms at +2232 (IDA 0xa723f8).
    pub timeout_ms: u32,
    /// Output packet queue at +0..+19 (IDA 0xa74d64).
    pub output: VecDeque<Vec<u8>>,
    /// `SetSplitMessageProgressInterval` value (IDA 0xa76c90, engine-side counter).
    pub split_progress_interval: i32,
    /// `SetUnreliableTimeout` value in ms (IDA 0xa76c94, engine-side timer).
    pub unreliable_timeout_ms: u32,
    /// `IsDeadConnection` flag at +0x8b4 (IDA 0xa76e6c).
    pub dead_connection: bool,
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
    /// `PushDatagram` (IDA 0xa766b8): queue the pending datagram.
    /// Engine moves layered bytes onward; the queue edge is the callback.
    pub fn push_datagram(&mut self, push: &mut dyn FnMut()) {
        push();
    }
    /// `IsOutgoingDataWaiting` (IDA 0xa76c68): passthrough gate.
    #[must_use]
    pub fn is_outgoing_data_waiting(waiting: bool) -> bool {
        waiting
    }
    /// `AreAcksWaiting` (IDA 0xa76c84): passthrough gate.
    #[must_use]
    pub fn are_acks_waiting(waiting: bool) -> bool {
        waiting
    }
    /// `SetSplitMessageProgressInterval` (IDA 0xa76c90).
    pub fn set_split_message_progress_interval(&mut self, interval: i32) {
        self.split_progress_interval = interval;
    }
    /// `SetUnreliableTimeout` (IDA 0xa76c94).
    pub fn set_unreliable_timeout(&mut self, timeout_ms: u32) {
        self.unreliable_timeout_ms = timeout_ms;
    }
    /// `IsDeadConnection` (IDA 0xa76e6c): byte flag at +0x8b4.
    #[must_use]
    pub fn is_dead_connection(&self) -> bool {
        self.dead_connection
    }
    /// `GetStatistics` (IDA 0xa76e74): stats are engine-side; the fill is the callback.
    pub fn get_statistics(&self, fill: &mut dyn FnMut()) {
        fill();
    }
    /// `ResetPacketsAndDatagrams` (IDA 0xa765e0): zeroes the five queue
    /// counters without draining pool memory (contrast `ClearPacketsAndDatagrams`,
    /// IDA 0xa72d5c, which releases too). No-op Rust-side.
    pub fn reset_packets_and_datagrams(&mut self) {}
}

/// `DatagramHeaderFormat::Deserialize` (IDA 0xa77058): sequential flag-bit
/// reads into byte offsets +0x8..+0xe, an aligned `f32` at +4 on the ack arm,
/// or an aligned `uint24` datagram number at +0 otherwise.
#[derive(Clone, Debug, Default)]
pub struct DatagramHeader {
    /// `uint24` datagram number at +0 (else arm, aligned).
    pub number: u32,
    /// Aligned `f32` at +4 (ack arm).
    pub value: f32,
    pub flag_8: bool,
    pub flag_9: bool,
    pub flag_a: bool,
    pub flag_b: bool,
    pub flag_c: bool,
    pub flag_d: bool,
    pub flag_e: bool,
}

/// Deserialize a datagram header from a bit stream (IDA 0xa77058).
#[must_use]
pub fn deserialize_datagram_header(stream: &mut crate::bitstream::BitStream) -> DatagramHeader {
    let mut header = DatagramHeader::default();
    header.flag_e = stream.read_bit().unwrap_or(false);
    header.flag_8 = stream.read_bit().unwrap_or(false);
    if header.flag_8 {
        header.flag_b = stream.read_bit().unwrap_or(false);
        // IDA 0xa77102: align-up before the float; the empty-slice call
        // only advances the cursor, its `false` is discarded.
        let mut pad: [u8; 0] = [];
        let _ = stream.read_aligned_bytes(&mut pad);
        header.value = stream.read_f32().unwrap_or(0.0);
    } else {
        header.flag_9 = stream.read_bit().unwrap_or(false);
        if header.flag_9 {
            header.flag_a = false;
        } else {
            header.flag_a = stream.read_bit().unwrap_or(false);
            header.flag_c = stream.read_bit().unwrap_or(false);
            header.flag_d = stream.read_bit().unwrap_or(false);
            // IDA 0xa771d0: align-up; `read_uint24` aligns internally.
            header.number = stream.read_uint24().unwrap_or(0);
        }
    }
    header
}

/// `ReliabilityLayer::SplitPacket` split-count half (IDA 0xa75100): header
/// overhead in bits is 24 plus a per-reliability table (`dword_101E350`,
/// indexed by reliability minus 2), plus 24 for reliabilities 1/4 and 32 for
/// reliabilities in mask 0x9a; the payload is chunked into `mtu - 32` bytes.
#[must_use]
pub fn split_packet_count(bit_length: u32, mtu: u32, reliability: u8) -> u32 {
    // IDA 0x101e350: per-reliability header table for values 2..=7.
    const HEADER_TABLE: [u32; 6] = [48, 48, 48, 24, 48, 48];
    let mut header = 24u32;
    if (2..=7).contains(&reliability) {
        header += HEADER_TABLE[(reliability - 2) as usize];
    }
    if reliability == 1 || reliability == 4 {
        header += 24;
    }
    if reliability <= 7 && ((1 << reliability) & 0x9a) != 0 {
        header += 32;
    }
    let _ = header;
    let bytes = bit_length.div_ceil(8).saturating_sub(1);
    bytes / mtu.saturating_sub(32).max(1) + 1
}

/// `ReliabilityLayer::AckTimeout` (IDA 0xa7641c): no timeout while the ack
/// age is under 10001 ms; past that the wrapped `sent - now` high word is
/// nonzero (timed out) and only its low word is compared to the limit.
#[must_use]
pub fn ack_timeout(now_ms: u64, sent_ms: u64, timeout_ms: u32) -> bool {
    if now_ms.wrapping_sub(sent_ms) < 10001 {
        return false;
    }
    let neg = sent_ms.wrapping_sub(now_ms);
    if (neg >> 32) as u32 != 0 {
        return true;
    }
    (neg as u32) > timeout_ms
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

    #[test]
    fn split_ack_datagram_flags() {
        // IDA 0xa75100: 100 payload bits in a 500-byte MTU fit one chunk.
        assert_eq!(split_packet_count(100, 500, 2), 1);
        // 500 bytes need two 468-byte chunks.
        assert_eq!(split_packet_count(4000, 500, 2), 2);
        // IDA 0xa7641c: fresh acks never time out.
        assert!(!ack_timeout(20000, 15000, 5000));
        assert!(ack_timeout(20000, 5000, 5000));
        // IDA 0xa76e6c/0xa76c68/0xa76c84: flag gates.
        let mut layer = ReliabilityLayer::new();
        assert!(!layer.is_dead_connection());
        layer.dead_connection = true;
        assert!(layer.is_dead_connection());
        assert!(ReliabilityLayer::is_outgoing_data_waiting(true));
        assert!(!ReliabilityLayer::are_acks_waiting(false));
        layer.set_split_message_progress_interval(7);
        layer.set_unreliable_timeout(250);
        assert_eq!((layer.split_progress_interval, layer.unreliable_timeout_ms), (7, 250));
        let mut pushed = 0;
        layer.push_datagram(&mut || pushed += 1);
        assert_eq!(pushed, 1);
        let mut filled = 0;
        layer.get_statistics(&mut || filled += 1);
        assert_eq!(filled, 1);
        layer.reset_packets_and_datagrams();
        // IDA 0xa77058: else arm with flag_9 set clears flag_a.
        let mut stream = crate::bitstream::BitStream::new();
        stream.write_bit(false);
        stream.write_bit(false);
        stream.write_bit(true);
        let header = deserialize_datagram_header(&mut stream);
        assert!(!header.flag_e && !header.flag_8 && header.flag_9 && !header.flag_a);
    }
}
