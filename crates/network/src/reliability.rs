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
        // IDA 0xa77102: align-up before the float.
        stream.align_read_to_byte();
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
/// `DataStructures::RangeList<RakNet::uint24_t>` (IDA 0xa771e8/0xa7738c):
/// sorted `(min, max)` uint24 ranges with 24-bit wrapping arithmetic.
/// `List::Insert(node, index, file, line)` (IDA 0xa77486) inserts
/// positionally; the merge arms shift nodes in place like `Vec::remove`.
#[derive(Clone, Debug, Default)]
pub struct RangeList {
    /// Sorted non-overlapping `(min, max)` pairs, each masked to 24 bits.
    pub ranges: Vec<(u32, u32)>,
}

/// 24-bit mask shared by the range arithmetic (IDA 0xa77460).
pub const UINT24_MASK: u32 = 0xffffff;

impl RangeList {
    /// `RangeList::Insert` (IDA 0xa7738c): binary-search the minima, then
    /// insert `(value, value)`, extend an adjacent bound, or merge with a
    /// neighbour when `value` touches it (all wrapping at 24 bits).
    /// Values already covered are a no-op.
    pub fn insert_value(&mut self, value: u32) {
        let value = value & UINT24_MASK;
        // IDA 0xa773de: an empty list takes the value as its first range.
        if self.ranges.is_empty() {
            self.ranges.push((value, value));
            return;
        }
        let len = self.ranges.len();
        // Lower bound over the minima (IDA 0xa773f8): first index with
        // `min > value`, or the exact-match index.
        let mut lo = 0usize;
        let mut hi = len;
        let mut exact = false;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.ranges[mid].0 == value {
                lo = mid;
                exact = true;
                break;
            } else if self.ranges[mid].0 < value {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        if lo == len {
            // Past the end (IDA 0xa7743e): extend or append the last range.
            let last = self.ranges[len - 1];
            let next = (last.1 + 1) & UINT24_MASK;
            if value == next {
                self.ranges[len - 1].1 = value;
            } else if value > next {
                // Above `max + 1` but past every minimum: append. The
                // re-search in the original provably lands here too, so its
                // middle-insert arm is dead code.
                self.ranges.push((value, value));
            }
            // Else inside the last range: no-op.
            return;
        }
        let (mn, mx) = self.ranges[lo];
        let below = (mn + UINT24_MASK) & UINT24_MASK;
        if !exact && value < below {
            // Strictly below `min - 1` (IDA 0xa77466): insert before.
            self.ranges.insert(lo, (value, value));
        } else if !exact && value == below {
            // Touching below (IDA 0xa774e6): extend down, merging the
            // previous range when it now touches.
            self.ranges[lo].0 = value;
            if lo > 0 && (self.ranges[lo - 1].1 + 1) & UINT24_MASK == value {
                let cur_max = self.ranges[lo].1;
                self.ranges[lo - 1].1 = cur_max;
                self.ranges.remove(lo);
            }
        } else if value >= mn && value <= mx {
            // Covered: no-op (IDA 0xa77542 first arm).
        } else if value == (mx + 1) & UINT24_MASK {
            // Touching above (IDA 0xa7754c): extend up, merging the next
            // range when it now touches.
            self.ranges[lo].1 = value;
            if lo + 1 < self.ranges.len() && self.ranges[lo + 1].0 == (value + 1) & UINT24_MASK {
                let nxt = self.ranges.remove(lo + 1);
                self.ranges[lo].1 = nxt.1;
            }
        }
        // Else above `max + 1` at an exact-match index: no-op.
    }
}

/// `RangeList::Deserialize` (IDA 0xa771e8): align-up, a `u16` range count,
/// then per range a flag byte (nonzero = single value) plus `uint24`
/// `min` (and `max`, rejected when below `min`). `None` on short reads.
#[must_use]
pub fn deserialize_range_list(
    stream: &mut crate::bitstream::BitStream,
) -> Option<RangeList> {
    let mut out = RangeList::default();
    // IDA 0xa77226: align-up before the count; `read_u16` itself is raw.
    let mut pad: [u8; 0] = [];
    let _ = stream.read_aligned_bytes(&mut pad);
    let count = stream.read_u16()?;
    for _ in 0..count {
        let single = stream.read_u8()? != 0;
        let min = stream.read_uint24()?;
        if single {
            out.ranges.push((min, min));
        } else {
            let max = stream.read_uint24()?;
            if max < min {
                return None;
            }
            out.ranges.push((min, max));
        }
    }
    Some(out)
}

/// `DatagramHeaderFormat::Serialize` (IDA 0xa77a84): mirror of
/// [`deserialize_datagram_header`]. Note the leading bit is a constant 1,
/// not `flag_e` (IDA 0xa77a8e).
pub fn serialize_datagram_header(
    header: &DatagramHeader,
    stream: &mut crate::bitstream::BitStream,
) {
    stream.write_bit(true);
    if header.flag_8 {
        stream.write_bit(true);
        stream.write_bit(header.flag_b);
        // IDA 0xa77ad0: align-up; the empty call only moves the cursor.
        stream.write_aligned_bytes(&[]);
        stream.write_f32(header.value);
    } else {
        stream.write_bit(false);
        stream.write_bit(header.flag_9);
        if !header.flag_9 {
            stream.write_bit(header.flag_a);
            stream.write_bit(header.flag_c);
            stream.write_bit(header.flag_d);
            // IDA 0xa77b2e: align-up; `write_uint24` aligns internally.
            stream.write_uint24(header.number);
        }
    }
}

/// `DataStructures::Queue<RakNet::BPSTracker::TimeAndValue2>::Push`
/// (IDA 0xa78bbc): append a throughput sample.
#[derive(Clone, Copy, Debug, Default)]
pub struct BpsSample {
    /// Sample time in ms.
    pub time_ms: u32,
    /// Sampled byte count.
    pub value: u32,
}

/// Append a throughput sample to the back of the queue (IDA 0xa78bbc).
pub fn push_bps_sample(queue: &mut std::collections::VecDeque<BpsSample>, sample: BpsSample) {
    queue.push_back(sample);
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

    #[test]
    fn range_list_insert_merge() {
        // IDA 0xa7738c: append, extend, merge, cover.
        let mut list = RangeList::default();
        list.insert_value(10);
        list.insert_value(12);
        assert_eq!(list.ranges, vec![(10, 10), (12, 12)]);
        list.insert_value(11);
        assert_eq!(list.ranges, vec![(10, 12)]);
        list.insert_value(20);
        list.insert_value(13);
        // IDA 0xa77466: 13 lands before (20, 20) and does not reach back
        // to (10, 12); the original only merges at the insertion index.
        assert_eq!(list.ranges, vec![(10, 12), (13, 13), (20, 20)]);
        // IDA 0xa774e6: 19 touches (20, 20) from below.
        list.insert_value(19);
        assert_eq!(list.ranges, vec![(10, 12), (13, 13), (19, 20)]);
        // Bridging merge: 8..=12 extend past the end, 14..=20 append, then
        // 13 touches (14, 20) from below and absorbs (8, 12) (IDA 0xa774e6).
        let mut bridge = RangeList::default();
        for v in 8..=12 {
            bridge.insert_value(v);
        }
        for v in 14..=20 {
            bridge.insert_value(v);
        }
        assert_eq!(bridge.ranges, vec![(8, 12), (14, 20)]);
        bridge.insert_value(13);
        assert_eq!(bridge.ranges, vec![(8, 20)]);
        // Wrap edge: 0 sorts before min 0xffffff, so the original inserts a
        // second range instead of extending across the edge (IDA 0xa77466).
        let mut edge = RangeList::default();
        edge.insert_value(0xff_ffff);
        edge.insert_value(0);
        assert_eq!(edge.ranges, vec![(0, 0), (0xff_ffff, 0xff_ffff)]);
    }

    #[test]
    fn range_list_and_header_roundtrip() {
        // IDA 0xa771e8: flag byte selects single vs pair; max < min fails.
        let mut stream = crate::bitstream::BitStream::new();
        stream.write_u16(2);
        stream.write_u8(1);
        stream.write_uint24(7);
        stream.write_u8(0);
        stream.write_uint24(9);
        stream.write_uint24(12);
        let list = deserialize_range_list(&mut stream).expect("ranges");
        assert_eq!(list.ranges, vec![(7, 7), (9, 12)]);
        let mut bad = crate::bitstream::BitStream::new();
        bad.write_u16(1);
        bad.write_u8(0);
        bad.write_uint24(9);
        bad.write_uint24(5);
        assert!(deserialize_range_list(&mut bad).is_none());
        // IDA 0xa77a84: serialize mirrors the deserialize arms.
        let mut header = DatagramHeader::default();
        header.flag_8 = true;
        header.flag_b = true;
        header.value = 1.5;
        let mut out = crate::bitstream::BitStream::new();
        serialize_datagram_header(&header, &mut out);
        let back = deserialize_datagram_header(&mut out);
        assert!(back.flag_e && back.flag_8 && back.flag_b);
        assert_eq!(back.value, 1.5);
        let mut queue = std::collections::VecDeque::new();
        push_bps_sample(&mut queue, BpsSample { time_ms: 3, value: 9 });
        assert_eq!(queue.len(), 1);
    }
}
