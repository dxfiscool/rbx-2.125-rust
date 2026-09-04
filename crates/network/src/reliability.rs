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
/// `RakNet::InternalPacket` (IDA 0xa74750/0xa76a68): the layered packet the
/// reliability pump parses, queues, and serializes. Offsets below are the
/// `+N` field images from the create/write paths; `message_number` starts
/// at `0xffffff` (unassigned) and `split_count` 0 means no split header.
#[derive(Clone, Debug, Default)]
pub struct InternalPacket {
    /// +0 `uint24` message number.
    pub message_number: u32,
    /// +4 `uint24` ordering index.
    pub ordering_index: u32,
    /// +8 `uint24` ordering channel.
    pub ordering_channel: u32,
    /// +12 split byte (validated `<= 0x1f`, IDA 0xa748e4).
    pub split_byte: u8,
    /// +14 split packet id.
    pub split_id: u16,
    /// +16 split packet index (`< split_count`, IDA 0xa748e4).
    pub split_index: u32,
    /// +20 split packet count.
    pub split_count: u32,
    /// +24 bit length.
    pub bit_length: u32,
    /// +28 reliability (wire-mapped: 7->3, 6->2, 5->0, IDA 0xa76a8e).
    pub reliability: u8,
    /// +40 creation time.
    pub creation_time: u64,
    /// +60 payload bytes.
    pub data: Vec<u8>,
    /// +64 shared (refcounted) payload flag.
    pub shared: bool,
    /// +72 priority bucket.
    pub priority: u8,
    /// +76 receipt field.
    pub receipt: u32,
}

/// `DataStructures::MemoryPool<RakNet::InternalPacket>::Allocate` (IDA
/// 0xa7828c): pool blocks stay engine-side; hand out a default packet.
#[must_use]
pub fn internal_packet_allocate() -> InternalPacket {
    InternalPacket::default()
}

/// `DataStructures::MemoryPool<RakNet::InternalPacket>::Release` (IDA
/// 0xa783b4): return a packet to the pool (drop Rust-side).
pub fn internal_packet_release(_packet: InternalPacket) {}

/// `RakNet::ReliabilityLayer::MessageNumberNode` (IDA 0xa7696c): one
/// datagram-history entry's message number; links stay engine-side.
#[derive(Clone, Copy, Debug, Default)]
pub struct MessageNumberNode {
    pub message_number: u32,
}

/// `DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Allocate`
/// (IDA 0xa78670): hand out a node with its message number.
#[must_use]
pub fn message_number_node_allocate(message_number: u32) -> MessageNumberNode {
    MessageNumberNode { message_number }
}

/// `DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Release`
/// (IDA 0xa7848c): return a node to the pool (drop Rust-side).
pub fn message_number_node_release(_node: MessageNumberNode) {}

/// `RakNet::InternalPacketRefCountedData` (IDA 0xa7879c): shared payload
/// with its refcount; the bytes stay engine-side.
#[derive(Clone, Copy, Debug, Default)]
pub struct InternalPacketRefCountedData {
    pub refs: u32,
}

/// `DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Allocate`
/// (IDA 0xa7879c): hand out refcounted data starting at one reference.
#[must_use]
pub fn ref_counted_data_allocate() -> InternalPacketRefCountedData {
    InternalPacketRefCountedData { refs: 1 }
}

/// `DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Release`
/// (IDA 0xa788c8): return refcounted data to the pool (drop Rust-side).
pub fn ref_counted_data_release(_data: InternalPacketRefCountedData) {}

/// `RakNet::SplitPacketChannel` (IDA 0xa749fc): reassembly slot keyed by
/// split id, holding the received parts plus the first part's creation
/// time for the progress report.
#[derive(Clone, Debug, Default)]
pub struct SplitPacketChannel {
    pub split_id: u16,
    pub packets: Vec<InternalPacket>,
    pub creation_time: u64,
}

/// `RakNet::SplitPacketChannelComp` (IDA 0xa7090c): three-way compare of
/// the search key against the channel's split id (-1 below, 0 equal,
/// 1 above).
#[must_use]
pub fn split_packet_channel_comp(key: u16, channel_id: u16) -> i32 {
    if key < channel_id {
        -1
    } else {
        i32::from(key != channel_id)
    }
}

/// `DataStructures::OrderedList<...SplitPacketChannel...>` search (IDA
/// 0xa74a3a/0xa781da): binary-search by split id. `Ok(index)` on a hit,
/// `Err(position)` with the sorted insertion point on a miss.
pub fn split_channel_position(
    split_id: u16,
    channels: &[SplitPacketChannel],
) -> Result<usize, usize> {
    let mut lo = 0usize;
    let mut hi = channels.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match split_packet_channel_comp(split_id, channels[mid].split_id) {
            0 => return Ok(mid),
            c if c < 0 => hi = mid,
            _ => lo = mid + 1,
        }
    }
    Err(lo)
}

/// `DataStructures::OrderedList<...SplitPacketChannel...>::Insert` (IDA
/// 0xa781a4): binary-search by split id, then insert positionally. A
/// duplicate id inserts nothing and returns `None` (the original returns
/// `-1`, IDA 0xa781c6).
pub fn split_channel_ordered_insert(
    channels: &mut Vec<SplitPacketChannel>,
    channel: SplitPacketChannel,
) -> Option<usize> {
    match split_channel_position(channel.split_id, channels) {
        Ok(_) => None,
        Err(at) => {
            channels.insert(at, channel);
            Some(at)
        }
    }
}

/// `DataStructures::List<RakNet::SplitPacketChannel *>::Insert` indexed arm
/// (IDA 0xa7899c): grow (16, then 2x) and shift down from `index`.
/// `Vec::insert` keeps that edge; out-of-range indices clamp to the end.
pub fn split_channel_insert_at(
    channels: &mut Vec<SplitPacketChannel>,
    channel: SplitPacketChannel,
    index: usize,
) {
    let at = index.min(channels.len());
    channels.insert(at, channel);
}

/// `RakNet::ReliabilityLayer::InsertIntoSplitPacketList` (IDA 0xa749fc):
/// find-or-create the channel for the packet's split id, append the part,
/// and stamp the channel time. When the channel is still incomplete and
/// the part count hits the progress interval, the `30`-lead progress
/// payload (`count`, `split_count`, first-part bytes, first-part data;
/// IDA 0xa74c22) goes to `emit`. Returns the channel index.
pub fn insert_into_split_packet_list(
    channels: &mut Vec<SplitPacketChannel>,
    packet: InternalPacket,
    creation_time: u64,
    progress_interval: u32,
    emit: &mut dyn FnMut(Vec<u8>),
) -> usize {
    // IDA 0xa74a3a: own search first; found channels skip `Insert`
    // (LABEL_28), fresh ids go through `OrderedList::Insert`.
    let index = match split_channel_position(packet.split_id, channels) {
        Ok(found) => found,
        Err(at) => {
            channels.insert(at, SplitPacketChannel::default());
            channels[at].split_id = packet.split_id;
            at
        }
    };
    let channel = &mut channels[index];
    // IDA 0xa74b86: stamp the channel time.
    channel.creation_time = creation_time;
    // IDA 0xa74b72: append the part.
    channel.packets.push(packet);
    // IDA 0xa74b9c: progress report while incomplete, on the interval.
    if progress_interval != 0 {
        if let Some(first) = channel.packets.first() {
            let count = channel.packets.len() as u32;
            if count != first.split_count && count % progress_interval == 0 {
                let mut payload = vec![30u8];
                payload.extend_from_slice(&count.to_le_bytes());
                payload.extend_from_slice(&first.split_count.to_le_bytes());
                let first_bytes = ((first.bit_length + 7) >> 3).to_le_bytes();
                payload.extend_from_slice(&first_bytes);
                payload.extend_from_slice(&first.data);
                emit(payload);
            }
        }
    }
    index
}

/// `RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList` id arm (IDA
/// 0xa74c88): binary-search the channel; when its part count reaches the
/// first part's split count, run the ack callback, reassemble, drop the
/// channel, and return the packet. `None` while incomplete or missing.
pub fn build_packet_from_split_list(
    channels: &mut Vec<SplitPacketChannel>,
    split_id: u16,
    creation_time: u64,
    acked: &mut dyn FnMut(),
) -> Option<InternalPacket> {
    // IDA 0xa74cf2: binary search by split id.
    let mut lo = 0usize;
    let mut hi = channels.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        match split_packet_channel_comp(split_id, channels[mid].split_id) {
            0 => {
                lo = mid;
                break;
            }
            c if c < 0 => hi = mid,
            _ => lo = mid + 1,
        }
    }
    let channel = channels.get(lo)?;
    if split_packet_channel_comp(split_id, channel.split_id) != 0 {
        return None;
    }
    // IDA 0xa74d0e: complete when the part count meets the split count.
    let complete = channel
        .packets
        .first()
        .is_some_and(|first| channel.packets.len() as u32 == first.split_count);
    if !complete {
        return None;
    }
    // IDA 0xa74d26: ack, then build and remove the channel.
    acked();
    let channel = channels.remove(lo);
    Some(build_packet_from_split_channel(&channel, creation_time))
}

/// `RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList` channel arm
/// (IDA 0xa76ca4): fresh packet copying the first part's header, payload
/// concatenated part-by-part at `split_index * first-part-bytes`, then the
/// parts and channel are freed (ownership here takes that edge).
#[must_use]
pub fn build_packet_from_split_channel(
    channel: &SplitPacketChannel,
    creation_time: u64,
) -> InternalPacket {
    let mut packet = InternalPacket::default();
    packet.message_number = 0xffff_ff;
    packet.creation_time = creation_time;
    if let Some(first) = channel.packets.first() {
        // IDA 0xa76d02: copy the header words off the first part.
        packet.ordering_index = first.ordering_index;
        packet.ordering_channel = first.ordering_channel;
        packet.split_byte = first.split_byte;
        packet.message_number = first.message_number;
        packet.reliability = first.reliability;
        packet.split_id = first.split_id;
        // IDA 0xa76d34: total bit length is the parts' sum.
        let total_bits: u32 = channel.packets.iter().map(|part| part.bit_length).sum();
        packet.bit_length = total_bits;
        // IDA 0xa76d7e: part bytes land at `split_index * first-part-bytes`.
        let stride = ((first.bit_length + 7) >> 3) as usize;
        let total = ((total_bits + 7) >> 3) as usize;
        let mut data = vec![0u8; total];
        for part in &channel.packets {
            let bytes = ((part.bit_length + 7) >> 3) as usize;
            let at = (part.split_index as usize).saturating_mul(stride).min(total);
            let len = bytes.min(total.saturating_sub(at)).min(part.data.len());
            data[at..at + len].copy_from_slice(&part.data[..len]);
        }
        packet.data = data;
    }
    packet
}

/// `RakNet::ReliabilityLayer::WriteToBitStreamFromInternalPacket` (IDA
/// 0xa76a68): align-up, the reliability remap (7->3, 6->2, 5->0, else the
/// low 3 bits), the split flag, the aligned var16 bit length, the gated
/// `uint24` fields (message number for reliabilities in mask `0xdc`,
/// ordering channel for 1/4, ordering index plus split byte for mask
/// `0x9a`), the split triple when set, then the payload. Returns the bits
/// written.
pub fn write_internal_packet(
    stream: &mut crate::bitstream::BitStream,
    packet: &InternalPacket,
) -> u32 {
    let before = stream.bits_written();
    // IDA 0xa76a88: align-up before the reliability bits.
    stream.write_aligned_bytes(&[]);
    // IDA 0xa76a8e: remap 7->3, 6->2, 5->0.
    let wire_rel = match packet.reliability {
        7 => 3,
        6 => 2,
        5 => 0,
        rel => rel & 7,
    };
    stream.write_bits(u32::from(wire_rel), 3);
    // IDA 0xa76ab4: split flag.
    stream.write_bit(packet.split_count != 0);
    // IDA 0xa76ad6: align-up, then the bit length.
    stream.write_aligned_bytes(&[]);
    stream.write_aligned_var16(packet.bit_length as u16);
    let rel = packet.reliability;
    // IDA 0xa76af4: message number for reliabilities in mask 0xdc.
    if rel <= 7 && (1u32 << rel) & 0xdc != 0 {
        stream.write_uint24(packet.message_number);
    }
    // IDA 0xa76b02: ordering channel for reliabilities 1/4.
    if rel == 1 || rel == 4 {
        stream.write_uint24(packet.ordering_channel);
    }
    // IDA 0xa76b34: ordering index plus split byte for mask 0x9a.
    if rel <= 7 && (1u32 << rel) & 0x9a != 0 {
        stream.write_uint24(packet.ordering_index);
        stream.write_aligned_var8(packet.split_byte);
    }
    // IDA 0xa76b4c: the split triple when set.
    if packet.split_count != 0 {
        stream.write_aligned_var32(packet.split_count);
        stream.write_aligned_var16(packet.split_id);
        stream.write_aligned_var32(packet.split_index);
    }
    // IDA 0xa76b76: payload bytes.
    let bytes = ((packet.bit_length + 7) >> 3) as usize;
    let len = bytes.min(packet.data.len());
    stream.write_aligned_bytes(&packet.data[..len]);
    (stream.bits_written() - before) as u32
}

/// `RakNet::ReliabilityLayer::CreateInternalPacketFromBitStream` (IDA
/// 0xa74750): mirror of [`write_internal_packet`]. `None` when the stream
/// is short, the reliability is unmapped, the split byte exceeds `0x1f`,
/// or a split index fails `index < count`. The payload buffer is zeroed
/// at the tail like the original's `malloc` image (IDA 0xa7491c).
#[must_use]
pub fn create_internal_packet(
    stream: &mut crate::bitstream::BitStream,
    creation_time: u64,
) -> Option<InternalPacket> {
    // IDA 0xa7476c: need 32 bits before touching the pool.
    if stream.bits_remaining() < 32 {
        return None;
    }
    let mut packet = InternalPacket::default();
    packet.message_number = 0xffff_ff;
    packet.creation_time = creation_time;
    // IDA 0xa747c4: align-up, then the 3-bit reliability.
    stream.align_read_to_byte();
    let rel = stream.read_bits(3)? as u8;
    // IDA 0xa747d2: the split flag bit.
    let split = stream.read_bit()?;
    // IDA 0xa74812: align-up, then the var16 bit length.
    stream.align_read_to_byte();
    packet.bit_length = u32::from(stream.read_aligned_var16()?);
    packet.reliability = rel;
    // IDA 0xa74826: message number unless reliability is 2..=4.
    if u32::from(rel).wrapping_sub(2) > 2 {
        packet.message_number = 0xffff_ff;
    } else {
        packet.message_number = stream.read_uint24()?;
    }
    // IDA 0xa7483c: ordering channel for reliabilities 1/4.
    if rel == 1 || rel == 4 {
        packet.ordering_channel = stream.read_uint24()?;
    }
    // IDA 0xa7486e: ordering index plus split byte for mask 0x9a.
    if rel <= 7 && (1u32 << rel) & 0x9a != 0 {
        packet.ordering_index = stream.read_uint24()?;
        packet.split_byte = stream.read_aligned_var8()?;
    }
    // IDA 0xa74892: the split triple, or the split-byte gate.
    if split {
        packet.split_count = stream.read_aligned_var32()?;
        packet.split_id = stream.read_aligned_var16()?;
        packet.split_index = stream.read_aligned_var32()?;
    } else if packet.split_byte != 1 {
        return None;
    }
    // IDA 0xa748e4: payload, split byte, and index validation.
    if packet.bit_length == 0
        || rel > 7
        || packet.split_byte > 0x1f
        || (split && packet.split_index >= packet.split_count)
    {
        return None;
    }
    // IDA 0xa7490c: sized payload with a zeroed tail byte.
    let bytes = ((packet.bit_length + 7) >> 3) as usize;
    let mut data = vec![0u8; bytes];
    if !stream.read_aligned_bytes(&mut data) {
        return None;
    }
    packet.data = data;
    Some(packet)
}

/// `RakNet::ReliabilityLayer::Send` header half (IDA 0xa74dc0): clamp the
/// priority (>4 becomes 1), the channel (>0x1f becomes 0), and the
/// reliability (>7 becomes 2). An empty payload sends nothing. Oversize
/// payloads (past `mtu - 32` bytes) upgrade unreliable 5->6 / 1->4 / 0->2
/// and take the split path. Returns the clamped triple plus the split
/// verdict; ordering assignment, BPS stats, and the resend heap stay
/// engine-side.
#[must_use]
pub fn send_plan(
    priority: u8,
    channel: u8,
    reliability: u8,
    bit_length: u32,
    mtu: u32,
) -> Option<(u8, u8, u8, bool)> {
    // IDA 0xa74dde: priority clamp.
    let priority = if priority > 4 { 1 } else { priority };
    // IDA 0xa74de8: channel clamp.
    let channel = if channel > 0x1f { 0 } else { channel };
    // IDA 0xa74df0: reliability clamp.
    let mut reliability = if reliability > 7 { 2 } else { reliability };
    // IDA 0xa74dfc: empty payloads return 0.
    if bit_length == 0 {
        return None;
    }
    let bytes = (bit_length + 7) >> 3;
    // IDA 0xa74f3c: oversize upgrades plus the split path.
    if bytes > mtu.saturating_sub(32) {
        match reliability {
            5 => reliability = 6,
            1 | 4 => reliability = 4,
            0 => reliability = 2,
            _ => {}
        }
        return Some((priority, channel, reliability, true));
    }
    Some((priority, channel, reliability, false))
}

/// `RakNet::ReliabilityLayer::PushPacket` (IDA 0xa76828): account the
/// aligned payload bytes into both counters, then append the packet and
/// its flag. The original grows both arrays (16, then 2x); `Vec` keeps
/// that edge.
#[derive(Clone, Debug, Default)]
pub struct PacketQueue {
    pub packets: Vec<InternalPacket>,
    pub flags: Vec<bool>,
    pub bytes_a: u32,
    pub bytes_b: u32,
}

/// Append a packet and its flag to the queue (IDA 0xa76828).
pub fn push_packet(queue: &mut PacketQueue, packet: InternalPacket, flag: bool) {
    // IDA 0xa7684c: header allowance plus the aligned bit length, in bytes.
    let bytes = ((56 + ((packet.bit_length + 7) & 0xffff_fff8) + 7) & 0xffff_fff8) >> 3;
    queue.bytes_a = queue.bytes_a.wrapping_add(bytes);
    queue.bytes_b = queue.bytes_b.wrapping_add(bytes);
    queue.packets.push(packet);
    queue.flags.push(flag);
}

/// `RakNet::ReliabilityLayer::DatagramHistoryNode` (IDA 0xa7696c): one
/// history slot — the message number plus two aux words.
#[derive(Clone, Copy, Debug, Default)]
pub struct DatagramHistoryNode {
    pub message: u32,
    pub aux_a: u32,
    pub aux_b: u32,
}

/// Datagram history ring (IDA 0xa7696c/0xa76b88): past `0x200` entries the
/// oldest slot's message nodes are released (drop Rust-side) and the slot
/// is recycled.
#[derive(Clone, Debug, Default)]
pub struct DatagramHistory {
    pub slots: std::collections::VecDeque<DatagramHistoryNode>,
}

/// Evict the oldest slot past the `0x200` cap (IDA 0xa76996).
fn evict_datagram_history(history: &mut DatagramHistory) {
    let used = history.slots.len() as u32;
    if used > 0x200 {
        history.slots.pop_front();
    }
}

/// `RakNet::ReliabilityLayer::AddFirstToDatagramHistory` 3-word arm (IDA
/// 0xa7696c): evict past the cap, allocate a message-number node, and push
/// `{message, aux_a, aux_b}`.
pub fn add_first_to_datagram_history(
    history: &mut DatagramHistory,
    message: u32,
    aux_a: u32,
    aux_b: u32,
) {
    evict_datagram_history(history);
    let node = message_number_node_allocate(message);
    history.slots.push_back(DatagramHistoryNode { message: node.message_number, aux_a, aux_b });
}

/// `RakNet::ReliabilityLayer::AddFirstToDatagramHistory` 2-word arm (IDA
/// 0xa76b88): evict past the cap, then push a zero-message node.
pub fn push_datagram_history(history: &mut DatagramHistory, aux_a: u32, aux_b: u32) {
    evict_datagram_history(history);
    history.slots.push_back(DatagramHistoryNode { message: 0, aux_a, aux_b });
}

/// `RakNet::ReliabilityLayer::RemovePacketFromResendListAndDeleteOlderReliableSequenced`
/// (IDA 0xa74514): notify the plugins (`message_number`, `time_ms / 1000`),
/// then remove the acked packet from its `message_number & 0x1ff` resend
/// bucket. Ack-receipt reliabilities (>= 6) additionally emit the `14`-lead
/// control packet carrying the packet's trailing word (IDA 0xa7464c).
/// Bandwidth doubles and the resend links stay engine-side. Returns whether
/// a packet was removed.
pub fn remove_from_resend_list(
    resend: &mut [Vec<InternalPacket>],
    message_number: u32,
    time_ms: u32,
    plugin_count: usize,
    notify: &mut dyn FnMut(u32, u32),
    emit: &mut dyn FnMut(Vec<u8>),
) -> bool {
    // IDA 0xa74526: plugin ack notifications carry seconds.
    if plugin_count > 0 {
        notify(message_number, time_ms / 0x3e8);
    }
    // IDA 0xa74580: bucket by the low 9 bits of the message number.
    let bucket = resend.get_mut((message_number & 0x1ff) as usize);
    let Some(slot) = bucket else {
        return false;
    };
    let Some(at) = slot.iter().position(|packet| packet.message_number == message_number) else {
        return false;
    };
    let packet = slot.remove(at);
    // IDA 0xa745ec: ack-receipt reliabilities emit the control packet.
    if packet.reliability >= 6 {
        let mut control = vec![14u8];
        control.extend_from_slice(&packet.receipt.to_le_bytes());
        emit(control);
    }
    true
}

/// `RakNet::ReliabilityLayer::SendACKs` (IDA 0xa76468): while acks wait,
/// build one datagram per pass inside `8 * mtu - 72` bits (`build`) and
/// hand it to the socket (`emit`). BPS accounting, the sliding window,
/// and the actual send stay engine-side.
pub fn send_acks(
    acks_waiting: &mut bool,
    mtu: u32,
    build: &mut dyn FnMut(u32) -> Vec<u8>,
    emit: &mut dyn FnMut(Vec<u8>),
) {
    // IDA 0xa7648e: nothing to do without waiting acks.
    while *acks_waiting {
        // IDA 0xa764a0: range budget inside the datagram.
        let budget = 8u32.saturating_mul(mtu).saturating_sub(72);
        let datagram = build(budget);
        emit(datagram);
    }
}

/// `DataStructures::RangeList<RakNet::uint24_t>::Serialize` (IDA 0xa77b3c):
/// drain up to `max_bits` of ranges (each costs 40 bits single, 72 paired;
/// the lookahead stops past `max_bits - 81`) into a temp stream, align the
/// output, write the count plus the temp bits, and drop the written ranges
/// when `remove_written` is set. Returns the bits written.
pub fn serialize_range_list(
    list: &mut RangeList,
    stream: &mut crate::bitstream::BitStream,
    max_bits: u32,
    remove_written: bool,
) -> u32 {
    let mut tmp = crate::bitstream::BitStream::new();
    let mut used = 0u32;
    let mut count = 0u16;
    for &(min, max) in &list.ranges {
        // IDA 0xa77bc8: conservative 81-bit lookahead per range.
        if used + 81 > max_bits {
            break;
        }
        // IDA 0xa77be2: flag byte, nonzero for a single value.
        tmp.write_u8(u8::from(min == max));
        tmp.write_uint24(min);
        used += 40;
        if min != max {
            // IDA 0xa77c2a: paired max.
            tmp.write_uint24(max);
            used += 32;
        }
        count += 1;
    }
    // IDA 0xa77c6e: align the output, then the count and the temp bits.
    stream.write_aligned_bytes(&[]);
    let aligned = stream.bits_written();
    stream.write_u16(count);
    let bits = tmp.bits_written();
    stream.write_stream_bits(&mut tmp, bits);
    // IDA 0xa77c92: drop the written ranges when asked.
    if remove_written && count > 0 {
        let drop = (count as usize).min(list.ranges.len());
        list.ranges.drain(..drop);
    }
    (bits + (stream.bits_written() - aligned)) as u32
}

/// `DataStructures::Queue<RakNet::InternalPacket *>::Push` (IDA 0xa772b8):
/// append a packet to the back of the queue.
pub fn internal_packet_queue_push(
    queue: &mut std::collections::VecDeque<InternalPacket>,
    packet: InternalPacket,
) {
    queue.push_back(packet);
}
/// `DataStructures::Heap<unsigned long long, RakNet::InternalPacket *, false>`
/// (IDA 0xa77784/0xa77950/0xa77ff4): min-heap of `(key, packet)` pairs backed
/// by a flat 12-byte-node array (key lo/hi plus data). `Vec` keeps the
/// 16-then-2x growth edge; `series` is the sticky bulk-append flag the
/// original keeps at +12.
#[derive(Clone, Debug, Default)]
pub struct PacketHeap {
    pub entries: Vec<(u64, InternalPacket)>,
    pub series: bool,
}

/// `DataStructures::Heap<...>::Push` (IDA 0xa77950): append, then sift up
/// while the parent key is greater (`u64` compare). Returns the final slot
/// (the original returns the new count on a first push and a parent slot
/// otherwise; no in-repo caller observes it).
pub fn heap_push(heap: &mut PacketHeap, key: u64, packet: InternalPacket) -> usize {
    heap.entries.push((key, packet));
    let mut at = heap.entries.len() - 1;
    // IDA 0xa77a02: sift up past greater parents.
    while at > 0 {
        let parent = (at - 1) >> 1;
        if heap.entries[parent].0 <= key {
            break;
        }
        heap.entries.swap(at, parent);
        at = parent;
    }
    at
}

/// `DataStructures::Heap<...>::Pop` (IDA 0xa77784): remove the entry at
/// `index`, move the last entry into the hole, and sift it down past
/// smaller children (`u64` keys). `None` out of range. Returns the removed
/// packet.
pub fn heap_pop(heap: &mut PacketHeap, index: usize) -> Option<InternalPacket> {
    if index >= heap.entries.len() {
        return None;
    }
    // IDA 0xa777a2: `swap_remove` is the move-last-into-hole plus pop.
    let removed = heap.entries.swap_remove(index);
    // IDA 0xa777e0: sift down past the smaller child while greater.
    let mut at = index;
    loop {
        let left = 2 * at + 1;
        if left >= heap.entries.len() {
            break;
        }
        let right = left + 1;
        let mut child = left;
        if right < heap.entries.len() && heap.entries[right].0 < heap.entries[left].0 {
            child = right;
        }
        if heap.entries[at].0 <= heap.entries[child].0 {
            break;
        }
        heap.entries.swap(at, child);
        at = child;
    }
    Some(removed.1)
}

/// `DataStructures::Heap<...>::PushSeries` (IDA 0xa77ff4): in series mode
/// append blindly; otherwise scan the back half — a key at/after every
/// entry there appends directly and latches series mode, anything smaller
/// falls back to [`heap_push`]. Returns the final slot.
pub fn heap_push_series(heap: &mut PacketHeap, key: u64, packet: InternalPacket) -> usize {
    // IDA 0xa78000: series mode appends without scanning.
    if !heap.series {
        // IDA 0xa7807c: scan from `(count - 1) >> 1` to the end.
        let count = heap.entries.len();
        let mut ordered = true;
        let mut at = count.saturating_sub(1) >> 1;
        while at < count {
            if key < heap.entries[at].0 {
                ordered = false;
                break;
            }
            at += 1;
        }
        if !ordered {
            // IDA 0xa780de: smaller than the tail falls back to Push.
            return heap_push(heap, key, packet);
        }
        // IDA 0xa78196: direct appends latch series mode, even the first.
        heap.series = true;
    }
    heap.entries.push((key, packet));
    heap.entries.len() - 1
}

/// `DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert`
/// (IDA 0xa78a2c indexed, 0xa78b08 append): positional insert or back-append
/// of a `(min, max)` node. Out-of-range indices clamp to the end.
pub fn range_node_insert(list: &mut RangeList, min: u32, max: u32, index: Option<usize>) {
    let node = (min & UINT24_MASK, max & UINT24_MASK);
    match index {
        Some(at) => list.ranges.insert(at.min(list.ranges.len()), node),
        None => list.ranges.push(node),
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
    #[test]
    fn split_channel_comp_and_insert() {
        // IDA 0xa7090c: below, equal, above.
        assert_eq!(split_packet_channel_comp(3, 5), -1);
        assert_eq!(split_packet_channel_comp(5, 5), 0);
        assert_eq!(split_packet_channel_comp(7, 5), 1);
        // IDA 0xa781a4: ordered insert keeps split-id order; duplicates
        // insert nothing and report None.
        let mut channels = Vec::new();
        let at = split_channel_ordered_insert(
            &mut channels,
            SplitPacketChannel { split_id: 9, ..SplitPacketChannel::default() },
        );
        assert_eq!(at, Some(0));
        let at = split_channel_ordered_insert(
            &mut channels,
            SplitPacketChannel { split_id: 3, ..SplitPacketChannel::default() },
        );
        assert_eq!(at, Some(0));
        let dup = split_channel_ordered_insert(
            &mut channels,
            SplitPacketChannel { split_id: 9, ..SplitPacketChannel::default() },
        );
        assert_eq!((dup, channels.len()), (None, 2));
        // IDA 0xa7899c: indexed insert shifts down from the index.
        split_channel_insert_at(
            &mut channels,
            SplitPacketChannel { split_id: 6, ..SplitPacketChannel::default() },
            1,
        );
        let ids: Vec<u16> = channels.iter().map(|channel| channel.split_id).collect();
        assert_eq!(ids, vec![3, 6, 9]);
        // Position search: hit vs insertion point.
        assert_eq!(split_channel_position(6, &channels), Ok(1));
        assert_eq!(split_channel_position(7, &channels), Err(2));
    }

    #[test]
    fn packet_heap_push_pop_series() {
        // IDA 0xa77950: pushes sift up into a min-heap.
        let mut heap = PacketHeap::default();
        heap_push(&mut heap, 30, InternalPacket::default());
        heap_push(&mut heap, 10, InternalPacket::default());
        heap_push(&mut heap, 20, InternalPacket::default());
        let keys: Vec<u64> = heap.entries.iter().map(|entry| entry.0).collect();
        assert_eq!(keys[0], 10);
        // IDA 0xa77784: pop removes the slot and restores the heap.
        let top = heap_pop(&mut heap, 0).expect("top");
        assert_eq!((top.bit_length, heap.entries.len()), (0, 2));
        let keys: Vec<u64> = heap.entries.iter().map(|entry| entry.0).collect();
        assert_eq!(keys, vec![20, 30]);
        assert!(heap_pop(&mut heap, 9).is_none());
        // IDA 0xa77ff4: ordered series appends latch the sticky flag, so
        // later series pushes append blindly.
        let mut series = PacketHeap::default();
        heap_push_series(&mut series, 5, InternalPacket::default());
        assert!(series.series);
        heap_push_series(&mut series, 9, InternalPacket::default());
        heap_push_series(&mut series, 1, InternalPacket::default());
        let keys: Vec<u64> = series.entries.iter().map(|entry| entry.0).collect();
        assert_eq!(keys, vec![5, 9, 1]);
        // The Push fallback runs while the flag is clear: seed with plain
        // pushes, then a smaller series key sifts to the top.
        let mut mixed = PacketHeap::default();
        heap_push(&mut mixed, 9, InternalPacket::default());
        assert!(!mixed.series);
        heap_push_series(&mut mixed, 1, InternalPacket::default());
        assert_eq!(mixed.entries[0].0, 1);
    }

    #[test]
    fn range_node_insert_positions() {
        // IDA 0xa78a2c/0xa78b08: indexed insert vs back-append.
        let mut list = RangeList::default();
        range_node_insert(&mut list, 9, 12, None);
        range_node_insert(&mut list, 3, 3, Some(0));
        range_node_insert(&mut list, 5, 5, Some(99));
        assert_eq!(list.ranges, vec![(3, 3), (9, 12), (5, 5)]);
    }

    #[test]
    fn send_plan_clamps_and_split() {
        // IDA 0xa74dc0: empty payloads send nothing.
        assert_eq!(send_plan(1, 0, 2, 0, 512), None);
        // Priority/channel/reliability clamps.
        assert_eq!(send_plan(9, 0x40, 9, 64, 512), Some((1, 0, 2, false)));
        // Oversize upgrades: 5->6, 1->4, 0->2, then the split path.
        assert_eq!(send_plan(1, 2, 5, 8 * 600, 512).map(|plan| (plan.2, plan.3)), Some((6, true)));
        assert_eq!(send_plan(1, 2, 1, 8 * 600, 512).map(|plan| (plan.2, plan.3)), Some((4, true)));
        assert_eq!(send_plan(1, 2, 0, 8 * 600, 512).map(|plan| (plan.2, plan.3)), Some((2, true)));
        // Small reliable payloads go direct.
        assert_eq!(send_plan(2, 3, 2, 80, 512), Some((2, 3, 2, false)));
    }

    #[test]
    fn internal_packet_write_create_roundtrip() {
        // IDA 0xa76a68/0xa74750: header, gated fields, and payload survive.
        let mut packet = InternalPacket::default();
        packet.message_number = 0x1234;
        packet.ordering_index = 0x56;
        packet.ordering_channel = 0x78;
        packet.split_byte = 1;
        packet.bit_length = 24;
        packet.reliability = 3;
        packet.data = vec![0xaa, 0xbb, 0xcc];
        let mut stream = crate::bitstream::BitStream::new();
        assert!(write_internal_packet(&mut stream, &packet) > 0);
        let back = create_internal_packet(&mut stream, 7).expect("packet");
        assert_eq!(
            (back.message_number, back.ordering_index, back.bit_length, back.reliability),
            (0x1234, 0x56, 24, 3),
        );
        assert_eq!((back.data, back.creation_time), (vec![0xaa, 0xbb, 0xcc], 7));
        // Short streams and bad split indices fail.
        let mut short = crate::bitstream::BitStream::new();
        short.write_u8(0);
        assert!(create_internal_packet(&mut short, 0).is_none());
    }

    #[test]
    fn split_reassembly_flow() {
        // IDA 0xa749fc/0xa74c88/0xa76ca4: two parts in, one packet out.
        let mut part_a = InternalPacket::default();
        part_a.split_id = 11;
        part_a.split_index = 0;
        part_a.split_count = 2;
        part_a.bit_length = 16;
        part_a.reliability = 2;
        part_a.message_number = 0x42;
        part_a.data = vec![1, 2];
        let mut part_b = part_a.clone();
        part_b.split_index = 1;
        part_b.data = vec![3, 4];
        let mut channels = Vec::new();
        let mut emitted = 0;
        insert_into_split_packet_list(&mut channels, part_a, 100, 0, &mut |_| emitted += 1);
        insert_into_split_packet_list(&mut channels, part_b, 100, 0, &mut |_| emitted += 1);
        assert_eq!((channels.len(), emitted), (1, 0));
        let mut acked = 0;
        let packet =
            build_packet_from_split_list(&mut channels, 11, 101, &mut || acked += 1)
                .expect("complete");
        assert_eq!((acked, packet.data, packet.message_number), (1, vec![1, 2, 3, 4], 0x42));
        assert!(channels.is_empty());
        // Missing channels build nothing.
        assert!(build_packet_from_split_list(&mut channels, 11, 101, &mut || {}).is_none());
    }

    #[test]
    fn datagram_history_evicts_and_resend_removes() {
        // IDA 0xa7696c/0xa76b88: the ring caps at 0x200 plus the fresh push.
        let mut history = DatagramHistory::default();
        for i in 0..0x205 {
            add_first_to_datagram_history(&mut history, i, i + 1, i + 2);
        }
        push_datagram_history(&mut history, 1, 2);
        assert_eq!(history.slots.len(), 0x201);
        assert_eq!(history.slots.back().map(|node| node.message), Some(0));
        // IDA 0xa74514: bucket removal plus the receipt emit for rel >= 6.
        let mut resend: Vec<Vec<InternalPacket>> = vec![Vec::new(); 512];
        let mut gone = InternalPacket::default();
        gone.message_number = 0x300;
        gone.reliability = 6;
        gone.receipt = 0xde_ad;
        resend[(0x300 & 0x1ff) as usize].push(gone);
        let mut notified = Vec::new();
        let mut emitted = Vec::new();
        let removed = remove_from_resend_list(
            &mut resend,
            0x300,
            2000,
            1,
            &mut |msg, secs| notified.push((msg, secs)),
            &mut |bytes| emitted.push(bytes),
        );
        assert!(removed);
        assert_eq!(notified, vec![(0x300, 2)]);
        assert_eq!(emitted.len(), 1);
        assert_eq!((emitted[0][0], emitted[0].len()), (14, 5));
        assert!(!remove_from_resend_list(&mut resend, 0x300, 2000, 0, &mut |_, _| {}, &mut |_| {}));
    }

    #[test]
    fn range_list_serialize_roundtrip() {
        // IDA 0xa77b3c: count plus temp bits out; written ranges dropped.
        let mut list = RangeList::default();
        list.insert_value(7);
        for value in 9..=12 {
            list.insert_value(value);
        }
        let mut stream = crate::bitstream::BitStream::new();
        let bits = serialize_range_list(&mut list, &mut stream, 1 << 20, true);
        assert!(bits > 0);
        assert!(list.ranges.is_empty());
        let back = deserialize_range_list(&mut stream).expect("ranges");
        assert_eq!(back.ranges, vec![(7, 7), (9, 12)]);
        // Tight budgets write nothing and keep the ranges.
        let mut kept = RangeList::default();
        kept.insert_value(3);
        let mut thin = crate::bitstream::BitStream::new();
        serialize_range_list(&mut kept, &mut thin, 10, true);
        assert_eq!(kept.ranges, vec![(3, 3)]);
    }

    #[test]
    fn packet_queue_push_counts() {
        // IDA 0xa76828: counters plus the queued packet and flag.
        let mut queue = PacketQueue::default();
        let mut packet = InternalPacket::default();
        packet.bit_length = 8;
        push_packet(&mut queue, packet, true);
        assert_eq!((queue.packets.len(), queue.flags, queue.bytes_a), (1, vec![true], queue.bytes_b));
        assert!(queue.bytes_a > 0);
        // IDA 0xa772b8: plain packet-queue push.
        let mut plain = std::collections::VecDeque::new();
        internal_packet_queue_push(&mut plain, InternalPacket::default());
        assert_eq!(plain.len(), 1);
    }
}
