// Auto-generated skeletons for rbx-script — Script|Lua|LuaBridge|Yield (wdog3 1788396356)
// Filter: Script|Lua|LuaBridge|Yield (case-sensitive) — 4818 filtered, 0 remaining not yet in any crate (global), gap_filler EA-sorted asc distinct
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs EA-sorted asc | range 0xa6c7d0..0xa7d1d8 | distinct not yet in any crate (remaining 16396 -> 16276 after batch)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; boost stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use parking_lot::Mutex;
use rbx_core::SharedPtr;
use std::collections::VecDeque;
use std::sync::LazyLock;
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

/// RakNet `DataStructures::MemoryPool<T>` observable state (IDA 0xa6c7d0).
/// The pool keeps per-page free stacks plus full/empty page chains; only the
/// live count, free slots, and page count are observed here — raw
/// `rakMalloc_Ex`/`rakFree_Ex` plumbing folds into the host allocator.
#[derive(Debug, Default)]
pub struct RakMemPool {
    pub free: Vec<u32>,
    pub next: u32,
    pub live: usize,
    pub pages: usize,
}

impl RakMemPool {
    /// Fast path (IDA 0xa6c7e4..0xa6c7ee): pop the free-stack top (LIFO);
    /// slow path (0xa6c844..0xa6c89e): grow one page of block slots.
    pub fn allocate(&mut self) -> u32 {
        if let Some(slot) = self.free.pop() {
            self.live += 1;
            return slot;
        }
        let slot = self.next;
        self.next += 1;
        self.pages += 1;
        self.live += 1;
        slot
    }

    /// Push the slot back (IDA 0xa6c8f2..0xa6c8fc). Whole-page release once
    /// a page fills with `pages >= 4` (0xa6c908..0xa6c956) and the
    /// full/empty page-list rotation (0xa6c960..0xa6ca04) fold into the host.
    pub fn release(&mut self, slot: u32) {
        self.free.push(slot);
        self.live = self.live.saturating_sub(1);
    }
}

/// RakNet `DataStructures::Queue<T*>` ring buffer (IDA 0xa6ccdc): first push
/// allocates 16 slots (0xa6cd88..0xa6cda2), the tail wraps (0xa6ccf2..0xa6cd10),
/// and a full queue doubles (0xa6cd1e..0xa6cd80).
#[derive(Debug, Default)]
pub struct RakPtrQueue {
    pub items: VecDeque<u32>,
    pub capacity: usize,
}

impl RakPtrQueue {
    pub fn push(&mut self, value: u32) {
        if self.capacity == 0 {
            self.capacity = 16;
        }
        self.items.push_back(value);
        if self.items.len() == self.capacity {
            self.capacity *= 2;
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

/// RakNet `DataStructures::List<T>` dynamic array (IDA 0xa6ced8): growth is
/// 16-minimum then doubling (0xa6cf2e..0xa6cf40), old entries are copied over
/// (0xa6cf8e..0xa6cfbe), and the new element is appended (0xa6cfd2..0xa6cfe6).
#[derive(Debug, Default)]
pub struct RakList<T: Clone> {
    pub items: Vec<T>,
    pub capacity: usize,
}

impl<T: Clone> RakList<T> {
    pub fn insert(&mut self, value: T) {
        if self.items.len() == self.capacity {
            self.capacity = if self.capacity == 0 { 16 } else { self.capacity * 2 };
            self.items.reserve(self.capacity - self.items.len());
        }
        self.items.push(value);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// `~List` frees the backing array once capacity is nonzero
    /// (IDA 0xa6f3c6..0xa6f3d0); drop glue covers the elements.
    pub fn destroy(&mut self) {
        self.items.clear();
        self.capacity = 0;
    }
}

/// RakNet `DataStructures::ThreadsafeAllocatingQueue<T>` teardown state
/// (IDA 0xa6d4c0): both mutexes drop (0xa6d512/0xa6d530), the two pool chains
/// are freed block by block (0xa6d53a..0xa6d5fa), and the counts zero
/// (0xa6d600..0xa6d604). Drop glue covers the mutexes.
#[derive(Debug, Default)]
pub struct TsAllocQueue<T> {
    pub ready: Vec<T>,
    pub buffered: Vec<T>,
}

impl<T> TsAllocQueue<T> {
    pub fn destroy(&mut self) {
        self.ready.clear();
        self.buffered.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.ready.is_empty() && self.buffered.is_empty()
    }
}

/// `RakNet::RakPeer::RemoteSystemStruct` construction state (IDA 0xa6d194):
/// twelve `SystemAddress` members (0xa6d1b6..0xa6d20e), the reliability layer
/// (0xa6d21a), the GUID (0xa6d252), and the zeroed tail (0xa6d262).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RemoteSystemState {
    pub system_addresses: u8,
    pub reliability_inited: bool,
    pub guid_inited: bool,
    pub tail_zeroed: bool,
}

/// One `RakNetSmartPtr<RakNetSocket>` array element (IDA 0xa6cb5c): the slot
/// owns a single smart-pointer reference; `live` tracks the socket peer.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RakSocketSlot {
    pub refs: u32,
    pub live: bool,
}

/// `OP_DELETE_ARRAY<RakNetSmartPtr<RakNetSocket>>` target: the array owns one
/// reference per element; deleting the array walks back-to-front
/// (0xa6cbba..0xa6cc16), decrements each refcount (0xa6cbdc), destroys sockets
/// whose count reaches zero (0xa6cbf6) with their control blocks
/// (0xa6cbfc/0xa6cc0c), then frees the base (0xa6cc1c); a null array is a
/// no-op (0xa6cba8).
#[derive(Debug, Default)]
pub struct SmartPtrSocketArray {
    pub slots: Vec<RakSocketSlot>,
    pub destroyed: usize,
}

/// RakNet `RakString` value (IDA 0xa6eaa4..0xa6f358). The C++ type points at a
/// refcounted `SharedString` with a process-wide `emptyString` static; each
/// Rust value owns its buffer copy, which is observationally identical
/// because strings are never mutated through shared references. `is_static`
/// marks the shared empty (never freed, cf. 0xa6ece2).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RakString {
    pub text: String,
    pub is_static: bool,
}

/// Mutex-guarded `RakString::freeList` of recycled backings (IDA
/// 0xa6edb2..0xa6ee18, 16-minimum/double growth; pre-grown in blocks of 0x80
/// by `Allocate`, 0xa6efd6..0xa6f098). `Vec` growth covers the policy.
static RAKSTRING_FREELIST: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

/// Depth of the recycled-backing freelist (test hook).
pub fn rakstring_freelist_depth() -> usize {
    RAKSTRING_FREELIST.lock().len()
}

impl RakString {
    /// Default ctor points at the shared empty (IDA 0xa6eaae..0xa6eab0).
    pub fn empty() -> Self {
        Self { text: String::new(), is_static: true }
    }

    pub fn assigned(text: &str) -> Self {
        Self { text: text.to_owned(), is_static: text.is_empty() }
    }

    /// `Free` (IDA 0xa6ec8c): the static empty is untouched (0xa6ece2);
    /// otherwise the refcount drops (0xa6ecf0..0xa6ed08, owned-copy covers),
    /// oversized (`>= 0x71`, 0xa6ed12) buffers fold into the host allocator,
    /// and the backing is recycled into the freelist (0xa6ed3a..0xa6ee18).
    pub fn free_storage(&mut self) {
        if self.is_static {
            return;
        }
        let backing = std::mem::take(&mut self.text);
        RAKSTRING_FREELIST.lock().push(backing);
        self.is_static = true;
    }

    /// `Allocate(n)` (IDA 0xa6ef14): pops a recycled backing or makes a fresh
    /// one and reserves `n` bytes.
    pub fn allocate(&mut self, n: usize) {
        let mut backing = RAKSTRING_FREELIST.lock().pop().unwrap_or_default();
        backing.clear();
        backing.reserve(n);
        self.text = backing;
        self.is_static = false;
    }
}
/// Detached-thread spawn latch (IDA 0xa6fa3c): `pthread_attr` gets the
/// priority (0xa6fa5e..0xa6fa68), a 2 MiB stack (0xa6fa72), and detached state
/// (0xa6fa7a); `pthread_create` (0xa6fa98) folds into the host spawner.
pub const RAK_THREAD_STACK_SIZE: usize = 0x200000;
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RakThread {
    pub running: bool,
    pub priority: i32,
}

/// Marsaglia multiplier filling `RakNetRandom` (IDA 0xa70278/0xa70288:
/// `MOVW`/`MOVT` immediates `0xdcd`/`0x1`, `MULS` at 0xa7028e).
pub const RAKNET_RANDOM_MULTIPLIER: u32 = 69069;
/// MT state words (loop bound `0x270` at 0xa70296).
pub const RAKNET_RANDOM_WORDS: usize = 624;

/// `RakNet::RakNetRandom` MT state (IDA 0xa70260..0xa702a4). The ctor leaves
/// the table zeroed with the draw count unset (0xa70264); `SeedMT` stores
/// `seed|1` (0xa7027e/0xa7028c) with count zero (0xa70282) and fills
/// `mt[i] = mt[i-1] * 69069` (0xa7028e..0xa7029a).
#[derive(Debug, Clone)]
pub struct RakNetRandom {
    pub mt: [u32; RAKNET_RANDOM_WORDS],
    pub remaining: u32,
}

impl Default for RakNetRandom {
    fn default() -> Self {
        Self { mt: [0; RAKNET_RANDOM_WORDS], remaining: 0 }
    }
}

impl RakNetRandom {
    pub fn seed_mt(&mut self, seed: u32) {
        self.mt[0] = seed | 1;
        for i in 1..RAKNET_RANDOM_WORDS {
            self.mt[i] = self.mt[i - 1].wrapping_mul(RAKNET_RANDOM_MULTIPLIER);
        }
        self.remaining = 0;
    }

    /// Standard MT19937 twist (stock recurrence behind `reloadMT`, cf.
    /// 0xa702ec): the exact per-word update folds into the host.
    fn twist(&mut self) {
        for i in 0..RAKNET_RANDOM_WORDS {
            let y = (self.mt[i] & 0x8000_0000) | (self.mt[(i + 1) % RAKNET_RANDOM_WORDS] & 0x7fff_ffff);
            let mut x = self.mt[(i + 397) % RAKNET_RANDOM_WORDS] ^ (y >> 1);
            if y & 1 != 0 {
                x ^= 0x9908_b0df;
            }
            self.mt[i] = x;
        }
    }

    /// MT tempering exactly as decompiled (IDA 0xa702d4/0xa702e6).
    fn temper(x: u32) -> u32 {
        let y = x ^ (x >> 11);
        let y = y ^ ((y << 7) & 0x9d2c_5680);
        let y = y ^ ((y << 15) & 0xefc6_0000);
        y ^ (y >> 18)
    }

    /// `RandomMT` (IDA 0xa702a4): counts the remaining draws down
    /// (0xa702ac..0xa702b2), twists on exhaustion (0xa702ba..0xa702ec), and
    /// tempers the next word (0xa702bc..0xa702e6).
    pub fn random_mt(&mut self) -> u32 {
        if self.remaining == 0 {
            self.twist();
            self.remaining = RAKNET_RANDOM_WORDS as u32;
        }
        self.remaining -= 1;
        let idx = (RAKNET_RANDOM_WORDS as u32 - 1 - self.remaining) as usize;
        Self::temper(self.mt[idx])
    }
}

/// One resend-list entry (IDA 0xa74514/0xa72d5c): message number plus the
/// bit-length and refcount fields the counters derive from.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResendEntry {
    pub number: u32,
    pub bits: u32,
    pub reliability: u8,
}

/// One split-packet reassembly channel (IDA 0xa749fc): the split id plus the
/// queued packet handles. The 16-minimum/double array growth
/// (0xa74aa4..0xa74aba) folds into `Vec`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SplitChannel {
    pub id: u16,
    pub packets: Vec<u32>,
}

/// One queued outgoing packet (IDA 0xa76828): the 8-byte-aligned bit cost
/// plus the packet handle. The 16-minimum/double table growth
/// (0xa7686c..0xa768c0) folds into `Vec`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OutPacket {
    pub bits: u32,
    pub handle: u32,
}

/// One send-heap entry (IDA 0xa74dc0/0xa75100): the priority the time heap
/// orders by plus the assembled packet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SendEntry {
    pub priority: u8,
    pub packet: InternalPacket,
}

/// One datagram-history node (IDA 0xa7696c/0xa76b88): the message-number key
/// plus the two trailing words queued at 0xa76a4e..0xa76a58.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HistoryEntry {
    pub key: u32,
    pub lo: u32,
    pub hi: u32,
}

/// `RakNetStatistics` snapshot (IDA 0xa76e74): the running counters copied
/// into the out struct (0xa76e88..0xa76f12, block copies fold into the host;
/// `GetTimeUS` at 0xa76e84 folds into the host clock).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReliabilityStats {
    pub datagrams: u64,
    pub received: u64,
    pub resend_pending: usize,
    pub output_queued: usize,
    pub acked_bytes: f64,
    pub bytes_queued: u64,
}

/// `RakNet::ReliabilityLayer` observable state (IDA 0xa70938..0xa749fc),
/// mirroring `rbx_network::reliability::ReliabilityLayer`: the opaque window
/// words at +60/+96 are `0x4000` out of the ctor (0xa7097c/0xa70994), the
/// timeout sits at +2232 (0xa723f8/0xa72400), and pools, windows, and plugin
/// peers stay engine-side.
#[derive(Debug, Default)]
pub struct ReliabilityState {
    pub window_a: u32,
    pub window_b: u32,
    pub timeout_ms: u32,
    pub unreliable_timeout_ms: u32,
    pub split_progress_interval: i32,
    pub dead_connection: bool,
    pub initialized: bool,
    pub window_bytes: u32,
    pub send_throttle: u32,
    pub resend_bytes: u64,
    pub acked_bytes: f64,
    pub output: VecDeque<Vec<u8>>,
    pub resend: Vec<ResendEntry>,
    pub split_channels: Vec<SplitChannel>,
    pub plugin_notes: u64,
    pub datagrams: u64,
    pub received: u64,
    /// Last-update timestamp at word 539 (IDA 0xa71516/0xa76422).
    pub last_ack_ms: u32,
    /// Ready flag at word 541 (IDA 0xa76c6c).
    pub outgoing_ready: bool,
    /// Send-queue depth at word 607 (IDA 0xa76c7c).
    pub send_queue_len: u32,
    /// Ack ranges waiting at word 970 (IDA 0xa7648e/0xa76c8e).
    pub acks_waiting: bool,
    /// Pending ack range count drained by `SendACKs` (IDA 0xa76468).
    pub pending_ack_ranges: u32,
    /// Acks emitted (covers the `SendACKs` call at 0xa74d26).
    pub acks_sent: u64,
    /// Aligned pending bits at words 957-958 (IDA 0xa76852/0xa7685c).
    pub pending_bits: u64,
    /// Outgoing packet table (IDA 0xa76828).
    pub outgoing: Vec<OutPacket>,
    /// Datagram records (IDA 0xa766b8).
    pub datagram_table: Vec<u32>,
    /// Datagram history ring, evicted past 0x200 entries (IDA 0xa76996).
    pub datagram_history: VecDeque<HistoryEntry>,
    /// History sequence at +76, wrapped to 24 bits (IDA 0xa76a1e).
    pub history_counter: u32,
    /// Time-heap queue (IDA 0xa74dc0/0xa75100).
    pub send_heap: Vec<SendEntry>,
    /// Next reliable message number.
    pub message_number: u32,
    /// Next split-packet id.
    pub split_id_counter: u16,
    /// `Update` pump runs (IDA 0xa75548).
    pub update_count: u64,
    /// Bytes queued stat at +3924/+3932 (IDA 0xa74e6c/0xa74e82).
    pub bytes_queued: u64,
}

impl ReliabilityState {
    /// C2 ctor (IDA 0xa70938): zeroes everything, then sets the two window
    /// words (0xa7097c/0xa70994).
    pub fn new() -> Self {
        Self { window_a: 0x4000, window_b: 0x4000, ..Self::default() }
    }

    /// `InitializeVariables` (IDA 0xa7142c): zeroes the regions, stamps host
    /// time (`GetTimeUS`/`GetTimeMS` at 0xa71498/0xa71516 fold into the host
    /// clock), and sets the split interval 15 (0xa714f6) and send throttle
    /// 350000 (0xa71546).
    pub fn init_vars(&mut self) {
        let fresh = Self::new();
        *self = fresh;
        self.initialized = true;
        self.split_progress_interval = 15;
        self.send_throttle = 350_000;
    }

    /// D2 dtor (IDA 0xa71604): frees thread-safe memory (0xa71658) and the
    /// heap arrays (0xa71662..); drop glue covers the peers.
    pub fn destroy(&mut self) {
        *self = Self::default();
    }

    /// `FreeThreadSafeMemory` (IDA 0xa72408): drains every live queue; pool
    /// releases fold into the host allocator.
    pub fn free_thread_safe_memory(&mut self) {
        self.output.clear();
        self.resend.clear();
        self.split_channels.clear();
        self.resend_bytes = 0;
    }
}

/// `RakNet::InternalPacket` parsed view (IDA 0xa74750): reliability travels
/// in 3 bits (0xa747c8) with a split flag, lengths are varints, message
/// numbers ride along for reliable kinds {2,3,4} (0xa74826..0xa74830),
/// ordering indices for {1,4} (0xa7483c..0xa7485e), and ordering channels
/// plus channel byte for {1,3,4,7} (0xa7486e..0xa74888, mask `0x9a`).
/// MODEL: byte-aligned little-endian framing; the bit-level reader folds
/// into the host while field order and validation are preserved.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InternalPacket {
    pub reliability: u8,
    pub has_split: bool,
    pub length_bits: u16,
    pub message_number: u32,
    pub ordering_index: u32,
    pub ordering_channel: u8,
    pub split_id: u32,
    pub split_index: u16,
    pub split_count: u32,
    pub data: Vec<u8>,
}

/// `CreateInternalPacketFromBitStream` (IDA 0xa74750): needs at least 32 bits
/// (0xa7476c); rejects zero lengths, reliabilities above 7, channels above
/// `0x1f`, and split indices past their counts (0xa748c0..0xa748e4); short
/// reads fail (0xa748ac/0xa748bc); otherwise the payload is copied
/// (0xa7490c..0xa7492a).
pub fn create_internal_packet(wire: &[u8]) -> Option<InternalPacket> {
    if wire.len() < 4 {
        return None;
    }
    let reliability = wire[0] & 7;
    if reliability > 7 {
        return None;
    }
    let has_split = wire[0] & 8 != 0;
    let length_bits = u16::from_le_bytes([wire[1], wire[2]]);
    if length_bits == 0 {
        return None;
    }
    let mut off = 3usize;
    let mut pkt = InternalPacket {
        reliability,
        has_split,
        length_bits,
        message_number: 0x00ff_ffff,
        ..InternalPacket::default()
    };
    if matches!(reliability, 2 | 3 | 4) {
        let end = off.checked_add(3)?;
        let raw: [u8; 3] = wire.get(off..end)?.try_into().ok()?;
        pkt.message_number = u32::from_le_bytes([raw[0], raw[1], raw[2], 0]);
        off = end;
    }
    if matches!(reliability, 1 | 4) {
        let end = off.checked_add(3)?;
        let raw: [u8; 3] = wire.get(off..end)?.try_into().ok()?;
        pkt.ordering_index = u32::from_le_bytes([raw[0], raw[1], raw[2], 0]);
        off = end;
    }
    if matches!(reliability, 1 | 3 | 4 | 7) {
        pkt.ordering_channel = *wire.get(off)?;
        if pkt.ordering_channel > 0x1f {
            return None;
        }
        off += 1;
    }
    if has_split {
        let end = off.checked_add(10)?;
        let b = wire.get(off..end)?;
        pkt.split_id = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        pkt.split_index = u16::from_le_bytes([b[4], b[5]]);
        pkt.split_count = u32::from_le_bytes([b[6], b[7], b[8], b[9]]);
        if pkt.split_count == 0 || u32::from(pkt.split_index) >= pkt.split_count {
            return None;
        }
        off = end;
    }
    let nbytes = (length_bits as usize + 7) / 8;
    let end = off.checked_add(nbytes)?;
    pkt.data = wire.get(off..end)?.to_vec();
    Some(pkt)
}
// 0xa6c7d0 — __ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer17SocketQueryOutputEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::RakPeer::SocketQueryOutput>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer17SocketQueryOutputEE8AllocateEPKcj")]
pub fn stub_0xa6c7d0(pool: &mut RakMemPool) -> u32 {
    // IDA 0xa6c7d0: `MemoryPool<SocketQueryOutput>::Allocate` pops a free
    // 20-byte block or grows one page; file/line bookkeeping folds into
    // the host allocator.
    pool.allocate()
}
// 0xa6c8e4 — __ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer17SocketQueryOutputEE7ReleaseEPS3_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::RakPeer::SocketQueryOutput>::Release(RakNet::RakPeer::SocketQueryOutput*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer17SocketQueryOutputEE7ReleaseEPS3_PKcj")]
pub fn stub_0xa6c8e4(pool: &mut RakMemPool, slot: u32) {
    // IDA 0xa6c8e4: `MemoryPool<SocketQueryOutput>::Release` recycles the
    // block into its page free-list.
    pool.release(slot);
}

// 0xa6c9ac — __ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer14RecvFromStructEE7ReleaseEPS3_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::RakPeer::RecvFromStruct>::Release(RakNet::RakPeer::RecvFromStruct*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer14RecvFromStructEE7ReleaseEPS3_PKcj")]
pub fn stub_0xa6c9ac(pool: &mut RakMemPool, slot: u32) {
    // IDA 0xa6c9ac: `MemoryPool<RecvFromStruct>::Release` — same recycle
    // shape as 0xa6c8e4 with 0x604-byte blocks (cf. 0xa6c9de divisor).
    pool.release(slot);
}

// 0xa6ca84 — __ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer21BufferedCommandStructEE7ReleaseEPS3_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::RakPeer::BufferedCommandStruct>::Release(RakNet::RakPeer::BufferedCommandStruct*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer21BufferedCommandStructEE7ReleaseEPS3_PKcj")]
pub fn stub_0xa6ca84(pool: &mut RakMemPool, slot: u32) {
    // IDA 0xa6ca84: `MemoryPool<BufferedCommandStruct>::Release` — same
    // recycle shape as 0xa6c8e4 with 0x6c-byte blocks (cf. 0xa6cab6 divisor).
    pool.release(slot);
}

// 0xa6cb5c — __ZN6RakNet15OP_DELETE_ARRAYINS_14RakNetSmartPtrINS_12RakNetSocketEEEEEvPT_PKcj
// type: void __fastcall(int, int, int, int, int, void *, int, int, void *, RakNet::RakNetSocket *, int, int, int, int)
#[doc(alias = "void RakNet::OP_DELETE_ARRAY<RakNet::RakNetSmartPtr<RakNet::RakNetSocket>>(RakNet::RakNetSmartPtr<RakNet::RakNetSocket> *,char const*,unsigned int)")]
#[doc(alias = "__ZN6RakNet15OP_DELETE_ARRAYINS_14RakNetSmartPtrINS_12RakNetSocketEEEEEvPT_PKcj")]
pub fn stub_0xa6cb5c(arr: &mut SmartPtrSocketArray) {
    // IDA 0xa6cb5c: `OP_DELETE_ARRAY<RakNetSmartPtr<RakNetSocket>>` walks
    // the array back-to-front dropping one reference per element and
    // destroying sockets whose count reaches zero, then frees the base.
    while let Some(slot) = arr.slots.pop() {
        if slot.refs <= 1 && slot.live {
            arr.destroyed += 1;
        }
    }
}

// 0xa6ccdc — __ZN14DataStructures5QueueIPN6RakNet7RakPeer21BufferedCommandStructEE4PushERKS4_PKcj
// type: void __fastcall(int **, int *)
#[doc(alias = "DataStructures::Queue<RakNet::RakPeer::BufferedCommandStruct *>::Push(RakNet::RakPeer::BufferedCommandStruct * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIPN6RakNet7RakPeer21BufferedCommandStructEE4PushERKS4_PKcj")]
pub fn stub_0xa6ccdc(queue: &mut RakPtrQueue, value: u32) {
    // IDA 0xa6ccdc: `Queue<BufferedCommandStruct*>::Push` stores at the tail
    // with wrap and doubles the ring when full.
    queue.push(value);
}

// 0xa6cdb0 — __ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer21BufferedCommandStructEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::RakPeer::BufferedCommandStruct>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet7RakPeer21BufferedCommandStructEE8AllocateEPKcj")]
pub fn stub_0xa6cdb0(pool: &mut RakMemPool) -> u32 {
    // IDA 0xa6cdb0: `MemoryPool<BufferedCommandStruct>::Allocate` — same
    // pop-or-grow shape as 0xa6c7d0 for 0x6c-byte blocks.
    pool.allocate()
}

// 0xa6ced8 — __ZN14DataStructures4ListIN6RakNet10RakNetGUIDEE6InsertERKS2_PKcj
// type: void __fastcall(int, __int64 *, int, int, int, void *, int, int, int)
#[doc(alias = "DataStructures::List<RakNet::RakNetGUID>::Insert(RakNet::RakNetGUID const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListIN6RakNet10RakNetGUIDEE6InsertERKS2_PKcj")]
pub fn stub_0xa6ced8(list: &mut RakList<u64>, value: u64) {
    // IDA 0xa6ced8: `List<RakNetGUID>::Insert` grows 16-minimum/doubling and
    // appends the 12-byte GUID (0xa6cf4c element stride, copy at 0xa6cfa6).
    // MODEL: the GUID payload folds into a `u64` handle.
    list.insert(value);
}

// 0xa6d030 — __ZN14DataStructures4ListIN6RakNet13SystemAddressEE6InsertERKS2_PKcj
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "DataStructures::List<RakNet::SystemAddress>::Insert(RakNet::SystemAddress const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListIN6RakNet13SystemAddressEE6InsertERKS2_PKcj")]
pub fn stub_0xa6d030(list: &mut RakList<u32>, value: u32) {
    // IDA 0xa6d030: `List<SystemAddress>::Insert` — same 16/double growth and
    // append as 0xa6ced8. MODEL: the address folds into a `u32` handle.
    list.insert(value);
}

// 0xa6d194 — __ZN6RakNet7RakPeer18RemoteSystemStructC2Ev
// type: RakNet::RakPeer::RemoteSystemStruct *__fastcall(RakNet::RakPeer::RemoteSystemStruct *this)
#[doc(alias = "RakNet::RakPeer::RemoteSystemStruct::RemoteSystemStruct(void)")]
#[doc(alias = "__ZN6RakNet7RakPeer18RemoteSystemStructC2Ev")]
pub fn stub_0xa6d194() -> RemoteSystemState {
    // IDA 0xa6d194: `RemoteSystemStruct` constructs twelve `SystemAddress`
    // members, the reliability layer, and the GUID, then zeroes the tail.
    RemoteSystemState {
        system_addresses: 12,
        reliability_inited: true,
        guid_inited: true,
        tail_zeroed: true,
    }
}

// 0xa6d2bc — __ZN14DataStructures4ListIN6RakNet14RakNetSmartPtrINS1_12RakNetSocketEEEE6InsertERKS4_PKcj
// type: void __fastcall(RakNet::RakNetSocket *, _DWORD *, int, int, int, int, int, int, int, void *, int, int, int)
#[doc(alias = "DataStructures::List<RakNet::RakNetSmartPtr<RakNet::RakNetSocket>>::Insert(RakNet::RakNetSmartPtr<RakNet::RakNetSocket> const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListIN6RakNet14RakNetSmartPtrINS1_12RakNetSocketEEEE6InsertERKS4_PKcj")]
pub fn stub_0xa6d2bc(list: &mut RakList<RakSocketSlot>, refs: u32) {
    // IDA 0xa6d2bc: `List<RakNetSmartPtr<RakNetSocket>>::Insert` — same
    // 16/double growth and append as 0xa6ced8; the smart-pointer add-ref
    // folds into the owning slot. MODEL: the socket peer folds into `live`.
    list.insert(RakSocketSlot { refs, live: true });
}

// 0xa6d4c0 — __ZN14DataStructures25ThreadsafeAllocatingQueueIN6RakNet7RakPeer21BufferedCommandStructEED2Ev
// type: int *__fastcall(int *)
#[doc(alias = "DataStructures::ThreadsafeAllocatingQueue<RakNet::RakPeer::BufferedCommandStruct>::~ThreadsafeAllocatingQueue()")]
#[doc(alias = "__ZN14DataStructures25ThreadsafeAllocatingQueueIN6RakNet7RakPeer21BufferedCommandStructEED2Ev")]
pub fn stub_0xa6d4c0(queue: &mut TsAllocQueue<u32>) {
    // IDA 0xa6d4c0: `ThreadsafeAllocatingQueue<BufferedCommandStruct>`
    // D2 dtor — frees both pool chains and zeroes the counts.
    queue.destroy();
}

// 0xa6d7a0 — __ZN14DataStructures25ThreadsafeAllocatingQueueIN6RakNet7RakPeer14RecvFromStructEED2Ev
// type: int *__fastcall(int *)
#[doc(alias = "DataStructures::ThreadsafeAllocatingQueue<RakNet::RakPeer::RecvFromStruct>::~ThreadsafeAllocatingQueue()")]
#[doc(alias = "__ZN14DataStructures25ThreadsafeAllocatingQueueIN6RakNet7RakPeer14RecvFromStructEED2Ev")]
pub fn stub_0xa6d7a0(queue: &mut TsAllocQueue<u32>) {
    // IDA 0xa6d7a0: `ThreadsafeAllocatingQueue<RecvFromStruct>` D2 dtor —
    // same two-chain teardown as 0xa6d4c0.
    queue.destroy();
}

// 0xa6da80 — __ZN14DataStructures25ThreadsafeAllocatingQueueIN6RakNet7RakPeer17SocketQueryOutputEED2Ev
// type: int *__fastcall(int *)
#[doc(alias = "DataStructures::ThreadsafeAllocatingQueue<RakNet::RakPeer::SocketQueryOutput>::~ThreadsafeAllocatingQueue()")]
#[doc(alias = "__ZN14DataStructures25ThreadsafeAllocatingQueueIN6RakNet7RakPeer17SocketQueryOutputEED2Ev")]
pub fn stub_0xa6da80(queue: &mut TsAllocQueue<u32>) {
    // IDA 0xa6da80: `ThreadsafeAllocatingQueue<SocketQueryOutput>` D2 dtor —
    // same two-chain teardown as 0xa6d4c0.
    queue.destroy();
}

// 0xa6eaa4 — __ZN6RakNet9RakStringC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RakNet::RakString::RakString(void)")]
#[doc(alias = "__ZN6RakNet9RakStringC1Ev")]
pub fn stub_0xa6eaa4() -> RakString {
    // IDA 0xa6eaa4: default ctor points at the shared empty (0xa6eaae).
    RakString::empty()
}

// 0xa6eab4 — __ZN6RakNet9RakString6AssignEPKcPv
// type: int __fastcall(RakNet::RakString *this, const char *__format, va_list)
#[doc(alias = "RakNet::RakString::Assign(char const*,void *)")]
#[doc(alias = "__ZN6RakNet9RakString6AssignEPKcPv")]
pub fn stub_0xa6eab4(target: &mut RakString, formatted: &str) {
    // IDA 0xa6eab4: `Assign` formats into a 512-byte stack buffer
    // (0xa6eada..0xa6eaf6) with an 8096-doubling heap fallback
    // (0xa6eaf8..0xa6ec0e), then `Allocate`s and copies the bytes
    // (0xa6eb4e..0xa6eb5c / 0xa6ebbe..0xa6ebcc); empty output selects the
    // shared empty (0xa6ebdc/0xa6ebec). MODEL: the caller provides the
    // formatted text; only the store is observed.
    if formatted.is_empty() {
        *target = RakString::empty();
    } else {
        target.allocate(formatted.len() + 1);
        target.text = formatted.to_owned();
    }
}

// 0xa6ec58 — __ZN6RakNet9RakStringC1EPKcz
// type: RakNet::RakString *(RakNet::RakString *this, const char *, ...)
#[doc(alias = "RakNet::RakString::RakString(char const*,...)")]
#[doc(alias = "__ZN6RakNet9RakStringC1EPKcz")]
pub fn stub_0xa6ec58(formatted: &str) -> RakString {
    // IDA 0xa6ec58: format ctor forwards to `Assign` (0xa6ec6c).
    let mut out = RakString::empty();
    stub_0xa6eab4(&mut out, formatted);
    out
}

// 0xa6ec7c — __ZN6RakNet9RakStringD1Ev
// type: void __fastcall(RakNet::RakString *__hidden this)
#[doc(alias = "RakNet::RakString::~RakString()")]
#[doc(alias = "__ZN6RakNet9RakStringD1Ev")]
pub fn stub_0xa6ec7c(target: &mut RakString) {
    // IDA 0xa6ec7c: D1 dtor calls `Free` (0xa6ec82); drop glue covers it.
    target.free_storage();
}

// 0xa6ec8c — __ZN6RakNet9RakString4FreeEv
// type: void __fastcall(RakNet::SimpleMutex ***this)
#[doc(alias = "RakNet::RakString::Free(void)")]
#[doc(alias = "__ZN6RakNet9RakString4FreeEv")]
pub fn stub_0xa6ec8c(target: &mut RakString) {
    // IDA 0xa6ec8c: `Free` — delegates to the shared backing teardown.
    target.free_storage();
}

// 0xa6eed4 — __ZN6RakNet9RakStringaSERKS0_
// type: RakNet::RakString *__fastcall(RakNet::RakString *, RakNet::SimpleMutex ***)
#[doc(alias = "RakNet::RakString::operator=(RakNet::RakString const&)")]
#[doc(alias = "__ZN6RakNet9RakStringaSERKS0_")]
pub fn stub_0xa6eed4(dst: &mut RakString, src: &RakString) {
    // IDA 0xa6eed4: `operator=` frees the target (0xa6eedc), then shares the
    // source backing with a refcount bump (0xa6eeee..0xa6ef02) or selects the
    // shared empty (0xa6ef06..). MODEL: owned-copy covers the share.
    dst.free_storage();
    if src.is_static {
        *dst = RakString::empty();
    } else {
        dst.text = src.text.clone();
        dst.is_static = false;
    }
}

// 0xa6ef14 — __ZN6RakNet9RakString8AllocateEm
// type: void __fastcall(RakNet::RakString *this, unsigned int)
#[doc(alias = "RakNet::RakString::Allocate(unsigned long)")]
#[doc(alias = "__ZN6RakNet9RakString8AllocateEm")]
pub fn stub_0xa6ef14(target: &mut RakString, capacity: usize) {
    // IDA 0xa6ef14: `Allocate` recycles or creates a backing and sizes it.
    target.allocate(capacity);
}

// 0xa6f1ac — __ZN6RakNet9RakString14IPAddressMatchEPKc
// type: bool __fastcall(RakNet::RakString *this, const char *__s)
#[doc(alias = "RakNet::RakString::IPAddressMatch(char const*)")]
#[doc(alias = "__ZN6RakNet9RakString14IPAddressMatchEPKc")]
pub fn stub_0xa6f1ac(stored: &RakString, ip: Option<&str>) -> bool {
    // IDA 0xa6f1ac: null (0xa6f1b8) or empty (0xa6f1ba) input never matches,
    // nor does input longer than 15 bytes (0xa6f1c8..0xa6f1ce). Otherwise
    // bytes compare in lockstep (0xa6f1d6..0xa6f1ee): an exact run matches,
    // and a first mismatch still matches when the stored byte is `*`
    // (0xa6f20a), which requires a remaining input byte.
    let Some(ip) = ip else { return false; };
    if ip.is_empty() || ip.len() > 15 {
        return false;
    }
    if stored.text == ip {
        return true;
    }
    if let Some(prefix) = stored.text.strip_suffix('*') {
        return ip.len() > prefix.len() && ip.starts_with(prefix);
    }
    false
}

// 0xa6f210 — __ZN6RakNet9RakString17FreeMemoryNoMutexEv
// type: void __fastcall(RakNet::RakString *this)
#[doc(alias = "RakNet::RakString::FreeMemoryNoMutex(void)")]
#[doc(alias = "__ZN6RakNet9RakString17FreeMemoryNoMutexEv")]
pub fn stub_0xa6f210() -> usize {
    // IDA 0xa6f210: `FreeMemoryNoMutex` destroys every backing in the
    // freelist (0xa6f262..0xa6f2c6) and reports the drained count.
    let mut q = RAKSTRING_FREELIST.lock();
    let n = q.len();
    q.clear();
    n
}

// 0xa6f328 — __ZNK6RakNet9RakString9SerializeEPNS_9BitStreamE
// type: RakNet::BitStream *__fastcall(RakNet::RakString *this, RakNet::BitStream *)
#[doc(alias = "RakNet::RakString::Serialize(RakNet::BitStream *)const")]
#[doc(alias = "__ZNK6RakNet9RakString9SerializeEPNS_9BitStreamE")]
pub fn stub_0xa6f328(stored: &RakString) -> Vec<u8> {
    // IDA 0xa6f328: `Serialize` writes the `u16` length (0xa6f332..0xa6f342)
    // then the aligned bytes (0xa6f354). MODEL: the bit stream folds into
    // the returned wire bytes.
    let mut wire = Vec::with_capacity(2 + stored.text.len());
    wire.extend_from_slice(&(stored.text.len() as u16).to_le_bytes());
    wire.extend_from_slice(stored.text.as_bytes());
    wire
}

// 0xa6f358 — __ZN6RakNet9RakString11DeserializeEPNS_9BitStreamE
// type: int __fastcall(RakNet::SimpleMutex ***this, RakNet::BitStream *)
#[doc(alias = "RakNet::RakString::Deserialize(RakNet::BitStream *)")]
#[doc(alias = "__ZN6RakNet9RakString11DeserializeEPNS_9BitStreamE")]
pub fn stub_0xa6f358(target: &mut RakString, wire: &[u8]) -> bool {
    // IDA 0xa6f358: `Deserialize` frees the target (0xa6f362), reads the
    // `u16` length (0xa6f36a), and for a nonzero length allocates (0xa6f378)
    // plus reads the bytes with NUL termination (0xa6f38c..0xa6f398);
    // short reads free and fail (0xa6f3b6..0xa6f3ba), while zero length only
    // realigns the bit position (0xa6f3ae).
    target.free_storage();
    if wire.len() < 2 {
        return false;
    }
    let len = u16::from_le_bytes([wire[0], wire[1]]) as usize;
    if len == 0 {
        return true;
    }
    if wire.len() < 2 + len {
        return false;
    }
    target.allocate(len + 1);
    target.text = String::from_utf8_lossy(&wire[2..2 + len]).into_owned();
    true
}

// 0xa6f3c0 — __ZN14DataStructures4ListIPN6RakNet9RakString12SharedStringEED1Ev
// type: int __fastcall(int)
#[doc(alias = "DataStructures::List<RakNet::RakString::SharedString *>::~List()")]
#[doc(alias = "__ZN14DataStructures4ListIPN6RakNet9RakString12SharedStringEED1Ev")]
pub fn stub_0xa6f3c0(list: &mut RakList<u32>) {
    // IDA 0xa6f3c0: `List<SharedString*>::~List` frees the backing array
    // (0xa6f3c6..0xa6f3d0); drop glue covers the entries.
    list.destroy();
}

#[cfg(test)]
mod rak_pool_queue_list_batch_tests {
    use super::*;

    /// Serializes tests that observe the process-wide `RAKSTRING_FREELIST`.
    static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    #[test]
    fn pool_allocate_recycles_lifo() {
        let mut pool = RakMemPool::default();
        let a = stub_0xa6c7d0(&mut pool);
        let b = stub_0xa6cdb0(&mut pool);
        assert_ne!(a, b);
        assert_eq!(pool.live, 2);
        stub_0xa6c8e4(&mut pool, a);
        stub_0xa6c9ac(&mut pool, b);
        assert_eq!(pool.live, 0);
        assert_eq!(stub_0xa6c7d0(&mut pool), b);
        assert_eq!(stub_0xa6cdb0(&mut pool), a);
        stub_0xa6ca84(&mut pool, a);
        stub_0xa6ca84(&mut pool, b);
        assert_eq!(pool.live, 0);
    }

    #[test]
    fn queue_grows_sixteen_then_doubles() {
        let mut q = RakPtrQueue::default();
        stub_0xa6ccdc(&mut q, 7);
        assert_eq!(q.capacity, 16);
        for i in 1..16u32 {
            stub_0xa6ccdc(&mut q, i);
        }
        assert_eq!(q.len(), 16);
        assert_eq!(q.capacity, 32);
        assert_eq!(q.items[0], 7);
        assert_eq!(q.items[15], 15);
    }

    #[test]
    fn list_insert_grows_and_appends() {
        let mut guids = RakList::<u64>::default();
        for i in 0..17u64 {
            stub_0xa6ced8(&mut guids, i * 3);
        }
        assert_eq!(guids.len(), 17);
        assert_eq!(guids.capacity, 32);
        assert_eq!(guids.items[16], 48);
        let mut addrs = RakList::<u32>::default();
        stub_0xa6d030(&mut addrs, 0xdead);
        assert_eq!(addrs.items, vec![0xdead]);
        let mut sockets = RakList::<RakSocketSlot>::default();
        stub_0xa6d2bc(&mut sockets, 2);
        assert!(sockets.items[0].live);
        assert_eq!(sockets.items[0].refs, 2);
        stub_0xa6f3c0(&mut RakList::<u32> { items: vec![1], capacity: 16 });
    }

    #[test]
    fn delete_array_destroys_last_owners() {
        let mut arr = SmartPtrSocketArray::default();
        arr.slots.push(RakSocketSlot { refs: 1, live: true });
        arr.slots.push(RakSocketSlot { refs: 3, live: true });
        stub_0xa6cb5c(&mut arr);
        assert!(arr.slots.is_empty());
        assert_eq!(arr.destroyed, 1);
        stub_0xa6cb5c(&mut arr);
        assert_eq!(arr.destroyed, 1);
    }

    #[test]
    fn remote_system_ctor_shapes_state() {
        let state = stub_0xa6d194();
        assert_eq!(
            state,
            RemoteSystemState {
                system_addresses: 12,
                reliability_inited: true,
                guid_inited: true,
                tail_zeroed: true,
            }
        );
    }

    #[test]
    fn ts_alloc_queue_dtor_drains() {
        let mut q = TsAllocQueue { ready: vec![1u32], buffered: vec![2u32, 3] };
        stub_0xa6d4c0(&mut q);
        assert!(q.is_empty());
        let mut q = TsAllocQueue { ready: vec![1u32], buffered: vec![] };
        stub_0xa6d7a0(&mut q);
        assert!(q.is_empty());
        let mut q = TsAllocQueue { ready: vec![], buffered: vec![9u32] };
        stub_0xa6da80(&mut q);
        assert!(q.is_empty());
    }

    #[test]
    fn rakstring_lifecycle_round_trip() {
        let _guard = TEST_LOCK.lock();
        let empty = stub_0xa6eaa4();
        assert!(empty.is_static);
        let mut s = stub_0xa6ec58("hello");
        assert_eq!(s.text, "hello");
        assert!(!s.is_static);
        let mut t = stub_0xa6eaa4();
        stub_0xa6eed4(&mut t, &s);
        assert_eq!(t.text, "hello");
        let depth_before = rakstring_freelist_depth();
        stub_0xa6ec8c(&mut s);
        assert!(s.is_static);
        assert_eq!(rakstring_freelist_depth(), depth_before + 1);
        stub_0xa6ec7c(&mut t);
        let mut sized = stub_0xa6eaa4();
        stub_0xa6ef14(&mut sized, 64);
        assert!(sized.text.capacity() >= 64);
        assert!(!sized.is_static);
        stub_0xa6ec8c(&mut sized);
        let drained = stub_0xa6f210();
        assert!(drained >= 1);
        assert_eq!(rakstring_freelist_depth(), 0);
        let mut blank = stub_0xa6eaa4();
        stub_0xa6eab4(&mut blank, "");
        assert!(blank.is_static);
    }

    #[test]
    fn ip_match_exact_wildcard_and_rejects() {
        let stored = RakString::assigned("192.168.1.1");
        assert!(stub_0xa6f1ac(&stored, Some("192.168.1.1")));
        assert!(!stub_0xa6f1ac(&stored, Some("192.168.1.2")));
        let wild = RakString::assigned("192.168.*");
        assert!(stub_0xa6f1ac(&wild, Some("192.168.1.1")));
        assert!(!stub_0xa6f1ac(&wild, Some("10.0.0.1")));
        assert!(!stub_0xa6f1ac(&wild, Some("192.168.")));
        assert!(!stub_0xa6f1ac(&stored, None));
        assert!(!stub_0xa6f1ac(&stored, Some("")));
        assert!(!stub_0xa6f1ac(&stored, Some("123.456.789.01234")));
    }
    #[test]
    fn serialize_deserialize_round_trip() {
        let _guard = TEST_LOCK.lock();
        let s = RakString::assigned("abc");
        let wire = stub_0xa6f328(&s);
        assert_eq!(wire, vec![3, 0, b'a', b'b', b'c']);
        let mut out = stub_0xa6eaa4();
        assert!(stub_0xa6f358(&mut out, &wire));
        assert_eq!(out.text, "abc");
        let mut zero = stub_0xa6eaa4();
        assert!(stub_0xa6f358(&mut zero, &[0, 0]));
        assert!(zero.is_static);
        let mut short = stub_0xa6eaa4();
        assert!(!stub_0xa6f358(&mut short, &[5, 0, b'a']));
        let mut empty_wire = stub_0xa6eaa4();
        assert!(!stub_0xa6f358(&mut empty_wire, &[]));
    }
}

// 0xa6fa3c — __ZN6RakNet9RakThread6CreateEPFPvS1_ES1_i
// type: int __fastcall(RakNet::RakThread *this, void *(__fastcall *)(void *), int, int)
#[doc(alias = "RakNet::RakThread::Create(void * (*)(void *),void *,int)")]
#[doc(alias = "__ZN6RakNet9RakThread6CreateEPFPvS1_ES1_i")]
pub fn stub_0xa6fa3c(thread: &mut RakThread, priority: i32) -> i32 {
    // IDA 0xa6fa3c: `RakThread::Create` latches the priority, 2 MiB stack,
    // and detached state into the attr (0xa6fa5e..0xa6fa7a) and spawns
    // detached (0xa6fa98). MODEL: pthread plumbing folds into the host;
    // success (0) is observed.
    thread.running = true;
    thread.priority = priority;
    0
}

// 0xa70260 — __ZN6RakNet12RakNetRandomC1Ev
// type: int __fastcall(int this)
#[doc(alias = "RakNet::RakNetRandom::RakNetRandom(void)")]
#[doc(alias = "__ZN6RakNet12RakNetRandomC1Ev")]
pub fn stub_0xa70260() -> RakNetRandom {
    // IDA 0xa70260: C1 ctor marks the draw count unset (0xa70264); the table
    // folds into zeroed host storage.
    RakNetRandom::default()
}

// 0xa70270 — __ZN6RakNet12RakNetRandomD1Ev
// type: void __fastcall(RakNet::RakNetRandom *__hidden this)
#[doc(alias = "RakNet::RakNetRandom::~RakNetRandom()")]
#[doc(alias = "__ZN6RakNet12RakNetRandomD1Ev")]
pub fn stub_0xa70270() {
    // IDA 0xa70270: D1 dtor has an empty body; drop glue covers it.
}

// 0xa70278 — __ZN6RakNet12RakNetRandom6SeedMTEj
// type: unsigned int *__fastcall(unsigned int *this, unsigned int)
#[doc(alias = "RakNet::RakNetRandom::SeedMT(unsigned int)")]
#[doc(alias = "__ZN6RakNet12RakNetRandom6SeedMTEj")]
pub fn stub_0xa70278(rng: &mut RakNetRandom, seed: u32) {
    // IDA 0xa70278: `SeedMT` — see `RakNetRandom::seed_mt`.
    rng.seed_mt(seed);
}

// 0xa702a4 — __ZN6RakNet12RakNetRandom8RandomMTEv
// type: unsigned int __fastcall(RakNet::RakNetRandom *this)
#[doc(alias = "RakNet::RakNetRandom::RandomMT(void)")]
#[doc(alias = "__ZN6RakNet12RakNetRandom8RandomMTEv")]
pub fn stub_0xa702a4(rng: &mut RakNetRandom) -> u32 {
    // IDA 0xa702a4: `RandomMT` — see `RakNetRandom::random_mt`.
    rng.random_mt()
}

// 0xa7090c — __ZN6RakNet22SplitPacketChannelCompERKtRKPNS_18SplitPacketChannelE
// type: int __fastcall(unsigned __int16 *, int)
#[doc(alias = "RakNet::SplitPacketChannelComp(unsigned short const&,RakNet::SplitPacketChannel * const&)")]
#[doc(alias = "__ZN6RakNet22SplitPacketChannelCompERKtRKPNS_18SplitPacketChannelE")]
pub fn stub_0xa7090c(key: u16, channel_id: u16) -> i32 {
    // IDA 0xa7090c: `SplitPacketChannelComp` orders split-channel keys
    // (0xa70910..0xa70926).
    if key < channel_id {
        -1
    } else {
        (key != channel_id) as i32
    }
}

// 0xa7092c — __ZN6RakNet16ReliabilityLayerC1Ev
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ReliabilityLayer(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerC1Ev")]
pub fn stub_0xa7092c() -> ReliabilityState {
    // IDA 0xa7092c: C1 ctor forwards to the C2 ctor (0xa70934).
    ReliabilityState::new()
}

// 0xa70938 — __ZN6RakNet16ReliabilityLayerC2Ev
// type: RakNet::ReliabilityLayer *__fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ReliabilityLayer(void) [0xa70938]")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerC2Ev")]
pub fn stub_0xa70938() -> ReliabilityState {
    // IDA 0xa70938: C2 ctor — see `ReliabilityState::new`.
    ReliabilityState::new()
}

// 0xa7142c — __ZN6RakNet16ReliabilityLayer19InitializeVariablesEv
// type: void __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::InitializeVariables(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer19InitializeVariablesEv")]
pub fn stub_0xa7142c(state: &mut ReliabilityState) {
    // IDA 0xa7142c: `InitializeVariables` — see `ReliabilityState::init_vars`.
    state.init_vars();
}

// 0xa715f8 — __ZN6RakNet16ReliabilityLayerD1Ev
// type: void __fastcall(RakNet::ReliabilityLayer *__hidden this)
#[doc(alias = "RakNet::ReliabilityLayer::~ReliabilityLayer()")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerD1Ev")]
pub fn stub_0xa715f8(state: &mut ReliabilityState) {
    // IDA 0xa715f8: D1 dtor forwards to the D2 dtor (0xa715fc).
    state.destroy();
}

// 0xa71604 — __ZN6RakNet16ReliabilityLayerD2Ev
// type: void __fastcall(RakNet::ReliabilityLayer *__hidden this)
#[doc(alias = "RakNet::ReliabilityLayer::~ReliabilityLayer() [0xa71604]")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerD2Ev")]
pub fn stub_0xa71604(state: &mut ReliabilityState) {
    // IDA 0xa71604: D2 dtor — see `ReliabilityState::destroy`.
    state.destroy();
}

// 0xa723c0 — __ZN6RakNet16ReliabilityLayer5ResetEbib
// type: _QWORD *__fastcall(RakNet::ReliabilityLayer *this, int, int, bool)
#[doc(alias = "RakNet::ReliabilityLayer::Reset(bool,int,bool)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer5ResetEbib")]
pub fn stub_0xa723c0(state: &mut ReliabilityState, full: bool, mtu: u32) -> u32 {
    // IDA 0xa723c0: `Reset` frees thread-safe memory (0xa723ca); when `full`
    // it reinitializes (0xa723d6) and inits the sliding window for
    // `mtu - 28` bytes (0xa723de..0xa723f0, window plumbing folds into the
    // host).
    state.free_thread_safe_memory();
    if full {
        state.init_vars();
        state.window_bytes = mtu.saturating_sub(28);
    }
    state.window_bytes
}

// 0xa723f8 — __ZN6RakNet16ReliabilityLayer14SetTimeoutTimeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::ReliabilityLayer::SetTimeoutTime(unsigned int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer14SetTimeoutTimeEj")]
pub fn stub_0xa723f8(state: &mut ReliabilityState, ms: u32) {
    // IDA 0xa723f8: `SetTimeoutTime` stores the timeout word (0xa723f8).
    state.timeout_ms = ms;
}

// 0xa72400 — __ZN6RakNet16ReliabilityLayer14GetTimeoutTimeEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::GetTimeoutTime(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer14GetTimeoutTimeEv")]
pub fn stub_0xa72400(state: &ReliabilityState) -> u32 {
    // IDA 0xa72400: `GetTimeoutTime` answers the timeout word (0xa72404).
    state.timeout_ms
}

// 0xa72408 — __ZN6RakNet16ReliabilityLayer20FreeThreadSafeMemoryEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::FreeThreadSafeMemory(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer20FreeThreadSafeMemoryEv")]
pub fn stub_0xa72408(state: &mut ReliabilityState) {
    // IDA 0xa72408: `FreeThreadSafeMemory` — see
    // `ReliabilityState::free_thread_safe_memory`.
    state.free_thread_safe_memory();
}

// 0xa72d5c — __ZN6RakNet16ReliabilityLayer24ClearPacketsAndDatagramsEv
// type: unsigned int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ClearPacketsAndDatagrams(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer24ClearPacketsAndDatagramsEv")]
pub fn stub_0xa72d5c(state: &mut ReliabilityState) -> u32 {
    // IDA 0xa72d5c: `ClearPacketsAndDatagrams` unlinks every resend entry
    // (0xa72daa..0xa72de6), releases refcounted payloads (0xa72e00..0xa72e28),
    // frees datagram bytes (0xa72e30..0xa72e40), releases the packets
    // (0xa72e44), then trims the datagram table (0xa72e64..0xa72e88) and
    // always answers 0 (0xa72e86/0xa72e92).
    state.resend.clear();
    state.output.clear();
    state.resend_bytes = 0;
    0
}

// 0xa72e94 — __ZN6RakNet16ReliabilityLayer38HandleSocketReceiveFromConnectedPlayerEPKcjRNS_13SystemAddressERN14DataStructures4ListIPNS_16PluginInterface2EEEiiPNS_12RakNetRandomEtjyRNS_9BitStreamE
// type: int __fastcall(int, unsigned __int8 *, unsigned int, _DWORD *, _DWORD *, int, int, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, unsigned __int64, RakNet::BitStream *)
#[doc(alias = "RakNet::ReliabilityLayer::HandleSocketReceiveFromConnectedPlayer(char const*,unsigned int,RakNet::SystemAddress &,DataStructures::List<RakNet::PluginInterface2 *> &,int,int,RakNet::RakNetRandom *,unsigned short,unsigned int,unsigned long long,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer38HandleSocketReceiveFromConnectedPlayerEPKcjRNS_13SystemAddressERN14DataStructures4ListIPNS_16PluginInterface2EEEiiPNS_12RakNetRandomEtjyRNS_9BitStreamE")]
pub fn stub_0xa72e94(state: &mut ReliabilityState, datagram: Option<&[u8]>) -> bool {
    // IDA 0xa72e94: `HandleSocketReceiveFromConnectedPlayer` runs null and
    // sub-3-byte buffers through the plugin notify loop (0xa72f5c..0xa72fb4)
    // and otherwise parses the datagram header plus per-packet
    // `CreateInternalPacketFromBitStream` items, answering 1 (0xa743d4).
    // MODEL: the per-packet dispatch folds into the host; the accepted
    // datagram is queued whole and counted.
    let Some(bytes) = datagram else {
        state.plugin_notes += 1;
        return true;
    };
    if bytes.len() <= 2 {
        state.plugin_notes += 1;
        return true;
    }
    state.datagrams += 1;
    state.received += 1;
    state.output.push_back(bytes.to_vec());
    true
}

// 0xa74514 — __ZN6RakNet16ReliabilityLayer57RemovePacketFromResendListAndDeleteOlderReliableSequencedENS_8uint24_tEyRN14DataStructures4ListIPNS_16PluginInterface2EEERKNS_13SystemAddressE
// type: int __fastcall(int, _DWORD *, unsigned __int64, _DWORD *, _DWORD *)
#[doc(alias = "RakNet::ReliabilityLayer::RemovePacketFromResendListAndDeleteOlderReliableSequenced(RakNet::uint24_t,unsigned long long,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::SystemAddress const&)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer57RemovePacketFromResendListAndDeleteOlderReliableSequencedENS_8uint24_tEyRN14DataStructures4ListIPNS_16PluginInterface2EEERKNS_13SystemAddressE")]
pub fn stub_0xa74514(
    state: &mut ReliabilityState,
    number: u32,
    time_us: u64,
    plugins: usize,
) -> i32 {
    // IDA 0xa74514: notifies each plugin with the millisecond timestamp
    // (`time_us / 1000`, cf. 0xa74544), probes the resend slot
    // `number & 0x1ff` (0xa74580), and on a number match (0xa7459a) unlinks
    // the entry, folds its bytes out of the counters (0xa745a8..0xa745e4),
    // releases the payload (0xa746b8..0xa7472c), and answers 0 (0xa74746);
    // a miss answers -1 (0xa74584/0xa7474e).
    state.plugin_notes += plugins as u64;
    let _time_ms = time_us / 1000;
    if let Some(pos) = state.resend.iter().position(|e| e.number == number) {
        let entry = state.resend.remove(pos);
        state.resend_bytes = state.resend_bytes.saturating_sub(entry.bits as u64);
        state.acked_bytes += ((entry.bits + 7) >> 3) as f64;
        0
    } else {
        -1
    }
}

// 0xa74750 — __ZN6RakNet16ReliabilityLayer33CreateInternalPacketFromBitStreamEPNS_9BitStreamEy
// type: int __fastcall(RakNet::ReliabilityLayer *this, RakNet::BitStream *, unsigned __int64)
#[doc(alias = "RakNet::ReliabilityLayer::CreateInternalPacketFromBitStream(RakNet::BitStream *,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer33CreateInternalPacketFromBitStreamEPNS_9BitStreamEy")]
pub fn stub_0xa74750(wire: &[u8]) -> Option<InternalPacket> {
    // IDA 0xa74750: `CreateInternalPacketFromBitStream` — see
    // `create_internal_packet` (pool allocate at 0xa74788 folds into the
    // host).
    create_internal_packet(wire)
}

// 0xa749fc — __ZN6RakNet16ReliabilityLayer25InsertIntoSplitPacketListEPNS_14InternalPacketEy
// type: unsigned int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::InsertIntoSplitPacketList(RakNet::InternalPacket *,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer25InsertIntoSplitPacketListEPNS_14InternalPacketEy")]
pub fn stub_0xa749fc(state: &mut ReliabilityState, split_id: u16, packet: u32) -> usize {
    // IDA 0xa749fc: `InsertIntoSplitPacketList` binary-searches the channel
    // list by split id (0xa74a1a..0xa74a5e), creates the channel on a miss
    // (0xa74a62..0xa74a98), and appends the packet to its array
    // (0xa74aa4..0xa74aba). MODEL: the ordered list folds into a sorted Vec.
    match state.split_channels.binary_search_by_key(&split_id, |c| c.id) {
        Ok(pos) => {
            state.split_channels[pos].packets.push(packet);
            pos
        }
        Err(pos) => {
            state.split_channels.insert(
                pos,
                SplitChannel { id: split_id, packets: vec![packet] },
            );
            pos
        }
    }
}

#[cfg(test)]
mod reliability_layer_batch_tests {
    use super::*;

    #[test]
    fn thread_create_latches() {
        let mut t = RakThread::default();
        assert_eq!(stub_0xa6fa3c(&mut t, 5), 0);
        assert!(t.running);
        assert_eq!(t.priority, 5);
        assert_eq!(RAK_THREAD_STACK_SIZE, 0x200000);
    }

    #[test]
    fn random_seed_multiplier_and_stream() {
        let mut a = stub_0xa70260();
        let mut b = stub_0xa70260();
        stub_0xa70278(&mut a, 0);
        stub_0xa70278(&mut b, 0);
        assert_eq!(a.mt[0], 1);
        assert_eq!(a.mt[1], RAKNET_RANDOM_MULTIPLIER);
        assert_eq!(a.mt[1], 69069);
        let v1: Vec<u32> = (0..700).map(|_| stub_0xa702a4(&mut a)).collect();
        let v2: Vec<u32> = (0..700).map(|_| stub_0xa702a4(&mut b)).collect();
        assert_eq!(v1, v2);
        let mut c = stub_0xa70260();
        stub_0xa70278(&mut c, 999);
        let w: Vec<u32> = (0..700).map(|_| stub_0xa702a4(&mut c)).collect();
        assert_ne!(v1, w);
        stub_0xa70270();
    }

    #[test]
    fn split_channel_comp_orders() {
        assert_eq!(stub_0xa7090c(3, 5), -1);
        assert_eq!(stub_0xa7090c(5, 5), 0);
        assert_eq!(stub_0xa7090c(7, 5), 1);
    }

    #[test]
    fn reliability_lifecycle() {
        let mut st = stub_0xa7092c();
        assert_eq!((st.window_a, st.window_b), (0x4000, 0x4000));
        let st2 = stub_0xa70938();
        assert_eq!((st2.window_a, st2.window_b), (0x4000, 0x4000));
        stub_0xa7142c(&mut st);
        assert!(st.initialized);
        assert_eq!(st.split_progress_interval, 15);
        assert_eq!(st.send_throttle, 350_000);
        stub_0xa723f8(&mut st, 10_000);
        assert_eq!(stub_0xa72400(&st), 10_000);
        assert_eq!(stub_0xa723c0(&mut st, true, 1500), 1472);
        assert_eq!(st.window_bytes, 1472);
        assert!(st.initialized);
        stub_0xa723c0(&mut st, false, 9000);
        assert_eq!(st.window_bytes, 1472);
        st.output.push_back(vec![1]);
        st.resend.push(ResendEntry { number: 7, bits: 80, reliability: 2 });
        stub_0xa72408(&mut st);
        assert!(st.output.is_empty() && st.resend.is_empty());
        st.output.push_back(vec![1]);
        stub_0xa715f8(&mut st);
        assert!(st.output.is_empty() && !st.initialized);
        stub_0xa7142c(&mut st);
        stub_0xa71604(&mut st);
        assert!(st.output.is_empty() && !st.initialized);
        assert_eq!(st.timeout_ms, 0);
    }

    #[test]
    fn clear_packets_answers_zero() {
        let mut st = stub_0xa70938();
        st.resend.push(ResendEntry { number: 1, bits: 8, reliability: 0 });
        st.output.push_back(vec![9]);
        assert_eq!(stub_0xa72d5c(&mut st), 0);
        assert!(st.resend.is_empty() && st.output.is_empty());
        assert_eq!(stub_0xa72d5c(&mut st), 0);
    }

    #[test]
    fn receive_gate_and_accept() {
        let mut st = stub_0xa70938();
        assert!(stub_0xa72e94(&mut st, None));
        assert!(stub_0xa72e94(&mut st, Some(&[])));
        assert!(stub_0xa72e94(&mut st, Some(&[0x80, 0x00])));
        assert_eq!(st.plugin_notes, 3);
        assert_eq!(st.datagrams, 0);
        assert!(stub_0xa72e94(&mut st, Some(&[0x80, 0x00, 0x01])));
        assert_eq!(st.datagrams, 1);
        assert_eq!(st.received, 1);
        assert_eq!(st.output.len(), 1);
    }

    #[test]
    fn remove_resend_hit_and_miss() {
        let mut st = stub_0xa70938();
        st.resend.push(ResendEntry { number: 42, bits: 80, reliability: 2 });
        st.resend_bytes = 80;
        assert_eq!(stub_0xa74514(&mut st, 43, 5_000_000, 2), -1);
        assert_eq!(st.plugin_notes, 2);
        assert_eq!(stub_0xa74514(&mut st, 42, 5_000_000, 1), 0);
        assert!(st.resend.is_empty());
        assert_eq!(st.resend_bytes, 0);
        assert_eq!(st.acked_bytes, 10.0);
        assert_eq!(st.plugin_notes, 3);
    }

    #[test]
    fn create_packet_validates() {
        assert!(stub_0xa74750(&[]).is_none());
        assert!(stub_0xa74750(&[0x02, 0x00, 0x00]).is_none());
        assert!(stub_0xa74750(&[0x00, 0x00, 0x00, 0x00]).is_none());
        let wire = vec![0x02, 0x10, 0x00, 0x05, 0x00, 0x00, 0xAA, 0xBB];
        let pkt = stub_0xa74750(&wire).expect("valid reliable packet");
        assert_eq!(pkt.reliability, 2);
        assert!(!pkt.has_split);
        assert_eq!(pkt.length_bits, 16);
        assert_eq!(pkt.message_number, 5);
        assert_eq!(pkt.data, vec![0xAA, 0xBB]);
        let wire = vec![
            0x09, 0x08, 0x00,
            0x01, 0x00, 0x00,
            0x03,
            0x78, 0x56, 0x34, 0x12,
            0x00, 0x00,
            0x02, 0x00, 0x00, 0x00,
            0xFF,
        ];
        let pkt = stub_0xa74750(&wire).expect("valid split packet");
        assert_eq!((pkt.split_id, pkt.split_index, pkt.split_count), (0x12345678, 0, 2));
        assert_eq!(pkt.ordering_channel, 3);
        assert_eq!(pkt.data, vec![0xFF]);
        let mut bad = wire.clone();
        bad[6] = 0x20;
        assert!(stub_0xa74750(&bad).is_none());
        let mut bad2 = wire.clone();
        bad2[11] = 0x02;
        assert!(stub_0xa74750(&bad2).is_none());
        assert!(stub_0xa74750(&wire[..wire.len() - 1]).is_none());
    }

    #[test]
    fn split_insert_orders_channels() {
        let mut st = stub_0xa70938();
        assert_eq!(stub_0xa749fc(&mut st, 9, 100), 0);
        assert_eq!(stub_0xa749fc(&mut st, 3, 101), 0);
        assert_eq!(stub_0xa749fc(&mut st, 9, 102), 1);
        assert_eq!(st.split_channels.len(), 2);
        assert_eq!(st.split_channels[0].id, 3);
        assert_eq!(st.split_channels[1].packets, vec![100, 102]);
    }
}

/// Assemble a complete split channel into one packet (IDA 0xa74c88/0xa76ca4):
/// header fields copy from the channel head (0xa76d02..0xa76d16) and payloads
/// concatenate. MODEL: header words fold into defaults; the payload
/// concatenates the 4-byte packet handles losslessly.
pub fn assemble_split_channel(channel: &SplitChannel) -> InternalPacket {
    let mut data = Vec::with_capacity(channel.packets.len() * 4);
    for handle in &channel.packets {
        data.extend_from_slice(&handle.to_le_bytes());
    }
    InternalPacket {
        length_bits: (data.len() * 8).min(u16::MAX as usize) as u16,
        data,
        ..InternalPacket::default()
    }
}

/// Push a datagram-history node with 0x200-entry eviction (IDA 0xa76996..
/// 0xa76a1e): past the cap the oldest chain is released and the 24-bit
/// sequence bumps; the pool allocate (0xa76a38) folds into the host.
pub fn push_history(state: &mut ReliabilityState, key: u32, lo: u32, hi: u32) -> u32 {
    if state.datagram_history.len() > 0x200 {
        state.datagram_history.pop_front();
        state.history_counter = (state.history_counter + 1) & 0x00ff_ffff;
    }
    let id = state.history_counter;
    state.history_counter = (state.history_counter + 1) & 0x00ff_ffff;
    state.datagram_history.push_back(HistoryEntry { key, lo, hi });
    id
}

/// Serialize an `InternalPacket` (IDA 0xa76a68): the written kind remaps
/// 7→3, 6→2, 5→0 (0xa76a94..0xa76aa0), then the 3-bit kind (0xa76aae), split
/// bit (0xa76ab4..0xa76ac4), var16 length (0xa76ae2), number for the original
/// kind under mask `0xdc` {2,3,4,6,7} (0xa76af4 — note the parse side at
/// 0xa74826 covers only {2,3,4}), ordering index for {1,4}
/// (0xa76b02..0xa76b20), channel word plus byte for mask `0x9a`
/// (0xa76b34..0xa76b48), the split triple (0xa76b4c..0xa76b68), and the
/// payload bytes (0xa76b76). MODEL: byte-aligned LE framing matching
/// `create_internal_packet`; returns the wire bytes (the original answers
/// the bit count, 0xa76b86).
pub fn write_internal_packet(pkt: &InternalPacket) -> Vec<u8> {
    let kind = match pkt.reliability {
        7 => 3,
        6 => 2,
        5 => 0,
        r => r,
    };
    let mut wire = Vec::new();
    wire.push((kind & 7) | if pkt.has_split { 8 } else { 0 });
    wire.extend_from_slice(&pkt.length_bits.to_le_bytes());
    if matches!(pkt.reliability, 2 | 3 | 4 | 6 | 7) {
        wire.extend_from_slice(&pkt.message_number.to_le_bytes()[..3]);
    }
    if matches!(pkt.reliability, 1 | 4) {
        wire.extend_from_slice(&pkt.ordering_index.to_le_bytes()[..3]);
    }
    if matches!(pkt.reliability, 1 | 3 | 4 | 7) {
        wire.push(pkt.ordering_channel);
    }
    if pkt.has_split {
        wire.extend_from_slice(&pkt.split_id.to_le_bytes());
        wire.extend_from_slice(&pkt.split_index.to_le_bytes());
        wire.extend_from_slice(&pkt.split_count.to_le_bytes());
    }
    wire.extend_from_slice(&pkt.data);
    wire
}

/// Fragment a packet for the MTU (IDA 0xa75100): 24-byte header baseline
/// (0xa75126, the per-reliability table at 0xa75148 folds into the host),
/// payload split across fragments pushed to the time heap (0xa754a4) with
/// per-priority stats (0xa754c8..0xa754e2), and the original released
/// (0xa75500). Returns the fragment count.
pub fn split_packet_into(
    state: &mut ReliabilityState,
    base: &InternalPacket,
    priority: u8,
    mtu: usize,
) -> usize {
    let room = mtu.saturating_sub(24).max(1);
    let mut chunks: Vec<&[u8]> = base.data.chunks(room).collect();
    if chunks.is_empty() {
        chunks.push(&[]);
    }
    let count = chunks.len();
    let split_id = state.split_id_counter;
    state.split_id_counter = state.split_id_counter.wrapping_add(1);
    for (i, chunk) in chunks.into_iter().enumerate() {
        state.send_heap.push(SendEntry {
            priority,
            packet: InternalPacket {
                has_split: true,
                length_bits: (chunk.len() * 8).min(u16::MAX as usize) as u16,
                split_id: u32::from(split_id),
                split_index: i as u16,
                split_count: count as u32,
                data: chunk.to_vec(),
                ..base.clone()
            },
        });
    }
    count
}

// 0xa74c88 — __ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEtyiRNS_13SystemAddressEPNS_12RakNetRandomEtjRNS_9BitStreamE
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned int, unsigned __int64, int, RakNet::SystemAddress *, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, RakNet::BitStream *)
#[doc(alias = "RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(unsigned short,unsigned long long,int,RakNet::SystemAddress &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEtyiRNS_13SystemAddressEPNS_12RakNetRandomEtjRNS_9BitStreamE")]
pub fn stub_0xa74c88(
    state: &mut ReliabilityState,
    split_id: u16,
    expected_total: u32,
) -> Option<InternalPacket> {
    // IDA 0xa74c88: binary-searches the channel by split id
    // (0xa74c98..0xa74cf2); when the channel holds `expected_total` packets
    // (0xa74d0e) it emits acks (0xa74d26), builds the packet (0xa74d32), and
    // removes the channel by shift-down (0xa74d36..0xa74d58). MODEL: socket
    // peers fold into the host; the ack is counted.
    let pos = state.split_channels.binary_search_by_key(&split_id, |c| c.id).ok()?;
    if expected_total == 0 || state.split_channels[pos].packets.len() as u32 != expected_total {
        return None;
    }
    let channel = state.split_channels.remove(pos);
    state.acks_sent += 1;
    Some(assemble_split_channel(&channel))
}

// 0xa74d64 — __ZN6RakNet16ReliabilityLayer7ReceiveEPPh
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned __int8 **)
#[doc(alias = "RakNet::ReliabilityLayer::Receive(unsigned char **)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer7ReceiveEPPh")]
pub fn stub_0xa74d64(state: &mut ReliabilityState) -> Option<(Vec<u8>, u32)> {
    // IDA 0xa74d64: `Receive` walks the output ring (0xa74d66..0xa74d98),
    // answers the packet bytes plus bit length (0xa74da4/0xa74da8), and
    // releases the packet (0xa74db8); an empty ring answers 0. MODEL: the
    // ring folds into the queue; empty is `None`.
    state.output.pop_front().map(|bytes| {
        let bits = (bytes.len() * 8) as u32;
        (bytes, bits)
    })
}

// 0xa74dc0 — __ZN6RakNet16ReliabilityLayer4SendEPcj14PacketPriority17PacketReliabilityhbiyj
// type: int __fastcall(int, const void *, int, unsigned int, unsigned int, unsigned int, int, int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::Send(char *,unsigned int,PacketPriority,PacketReliability,unsigned char,bool,int,unsigned long long,unsigned int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer4SendEPcj14PacketPriority17PacketReliabilityhbiyj")]
pub fn stub_0xa74dc0(
    state: &mut ReliabilityState,
    data: &[u8],
    mut priority: u8,
    mut reliability: u8,
    mut channel: u8,
    mtu: usize,
) -> bool {
    // IDA 0xa74dc0: `Send` clamps priority above 4 to 1 (0xa74dde..0xa74de0),
    // channels above 0x1f to 0 (0xa74de8..0xa74dea), and reliabilities above
    // 7 to 2 (0xa74df0..0xa74df2); empty payloads answer 0 (0xa74dfc/0xa750fe).
    // Otherwise the packet is allocated (0xa74e12..0xa74e2e), queued with
    // byte stats (0xa74e3e..0xa74e82), and either pushed to the time heap
    // (0xa75052..0xa750ec) or fragmented via `SplitPacket` (0xa74fb6/0xa750f0),
    // answering 1. MODEL: pool/heap peers fold into the host queue.
    if priority > 4 {
        priority = 1;
    }
    if channel > 0x1f {
        channel = 0;
    }
    if reliability > 7 {
        reliability = 2;
    }
    if data.is_empty() {
        return false;
    }
    state.bytes_queued += ((data.len() + 7) / 8) as u64;
    let number = if matches!(reliability, 2 | 3 | 4) {
        let n = state.message_number;
        state.message_number += 1;
        n
    } else {
        0x00ff_ffff
    };
    let base = InternalPacket {
        reliability,
        ordering_channel: channel,
        length_bits: (data.len() * 8).min(u16::MAX as usize) as u16,
        message_number: number,
        data: data.to_vec(),
        ..InternalPacket::default()
    };
    if data.len() + 24 <= mtu {
        state.send_heap.push(SendEntry { priority, packet: base });
    } else {
        split_packet_into(state, &base, priority, mtu);
    }
    true
}

// 0xa75100 — __ZN6RakNet16ReliabilityLayer11SplitPacketEPNS_14InternalPacketE
// type: int __fastcall(int, int)
#[doc(alias = "RakNet::ReliabilityLayer::SplitPacket(RakNet::InternalPacket *)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer11SplitPacketEPNS_14InternalPacketE")]
pub fn stub_0xa75100(
    state: &mut ReliabilityState,
    data: &[u8],
    reliability: u8,
    channel: u8,
    priority: u8,
    mtu: usize,
) -> usize {
    // IDA 0xa75100: `SplitPacket` — see `split_packet_into`.
    let base = InternalPacket {
        reliability,
        ordering_channel: channel,
        length_bits: (data.len() * 8).min(u16::MAX as usize) as u16,
        data: data.to_vec(),
        ..InternalPacket::default()
    };
    split_packet_into(state, &base, priority, mtu)
}

// 0xa75548 — __ZN6RakNet16ReliabilityLayer6UpdateEiRNS_13SystemAddressEiyjRN14DataStructures4ListIPNS_16PluginInterface2EEEPNS_12RakNetRandomEtjRNS_9BitStreamE
// type: bool __fastcall(int, RakNet::SocketLayer *, sockaddr *, int, unsigned __int64, int, _DWORD *, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, void **)
#[doc(alias = "RakNet::ReliabilityLayer::Update(int,RakNet::SystemAddress &,int,unsigned long long,unsigned int,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer6UpdateEiRNS_13SystemAddressEiyjRN14DataStructures4ListIPNS_16PluginInterface2EEEPNS_12RakNetRandomEtjRNS_9BitStreamE")]
pub fn stub_0xa75548(state: &mut ReliabilityState, ack_timed_out: bool) -> bool {
    // IDA 0xa75548: `Update` pumps retransmits, acks, and split builds; on an
    // ack timeout it latches the dead flag (0xa757b8) and answers 1
    // (0xa757b6), else it clears finished packets (0xa763f8) and answers
    // whether the pump is idle (0xa76412). MODEL: the socket/queue pump folds
    // into the host; the latch, run count, and idle answer are observed.
    if ack_timed_out {
        state.dead_connection = true;
        return true;
    }
    state.update_count += 1;
    true
}

// 0xa7641c — __ZN6RakNet16ReliabilityLayer10AckTimeoutEy
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned __int64)
#[doc(alias = "RakNet::ReliabilityLayer::AckTimeout(unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer10AckTimeoutEy")]
pub fn stub_0xa7641c(state: &ReliabilityState, now: u64) -> bool {
    // IDA 0xa7641c: `AckTimeout` treats a last-update within the last 0x2711
    // ms as live (0xa76422..0xa76448); otherwise the link is dead once
    // `now - last` passes the timeout — or overflows 32 bits
    // (0xa76450..0xa76464).
    let base = u64::from(state.last_ack_ms);
    if now <= base {
        base - now >= 0x2711
    } else {
        let diff = now - base;
        diff > u64::from(state.timeout_ms) || diff > u64::from(u32::MAX)
    }
}

// 0xa76468 — __ZN6RakNet16ReliabilityLayer8SendACKsEiRNS_13SystemAddressEyPNS_12RakNetRandomEtjRNS_9BitStreamE
// type: int __fastcall(RakNet::ReliabilityLayer *this, RakNet::SocketLayer *, sockaddr *, unsigned __int64, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, void **)
#[doc(alias = "RakNet::ReliabilityLayer::SendACKs(int,RakNet::SystemAddress &,unsigned long long,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer8SendACKsEiRNS_13SystemAddressEyPNS_12RakNetRandomEtjRNS_9BitStreamE")]
pub fn stub_0xa76468(state: &mut ReliabilityState) -> u32 {
    // IDA 0xa76468: `SendACKs` serializes the pending ack ranges into
    // MTU-sized datagrams (0xa7648a..0xa764ce) and clears them. MODEL: socket
    // writes fold into the host; the drained range count is observed.
    if !state.acks_waiting {
        return 0;
    }
    state.acks_waiting = false;
    let n = state.pending_ack_ranges;
    state.pending_ack_ranges = 0;
    state.acks_sent += 1;
    n
}

// 0xa765e0 — __ZN6RakNet16ReliabilityLayer24ResetPacketsAndDatagramsEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ResetPacketsAndDatagrams(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer24ResetPacketsAndDatagramsEv")]
pub fn stub_0xa765e0(state: &mut ReliabilityState) {
    // IDA 0xa765e0: `ResetPacketsAndDatagrams` drops the four packet tables
    // past 0x200 entries (pairs 944/942, 947/945, 950/948, 953/951 at
    // 0xa765ea..0xa76682) and zeroes their counts. MODEL: table capacities
    // fold into the host; all four queues drain but configuration is kept.
    state.output.clear();
    state.resend.clear();
    state.split_channels.clear();
    state.send_heap.clear();
    state.outgoing.clear();
    state.datagram_table.clear();
    state.datagram_history.clear();
    state.resend_bytes = 0;
    state.pending_bits = 0;
}

// 0xa766b8 — __ZN6RakNet16ReliabilityLayer12PushDatagramEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::PushDatagram(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer12PushDatagramEv")]
pub fn stub_0xa766b8(state: &mut ReliabilityState, record: u32) -> u64 {
    // IDA 0xa766b8: `PushDatagram` answers the pending bit count (0xa766c2);
    // when nonzero it appends the datagram record with 16-minimum/double
    // growth (0xa766da..0xa76720). MODEL: growth folds into `Vec`.
    if state.pending_bits == 0 {
        return 0;
    }
    state.datagram_table.push(record);
    state.pending_bits
}

// 0xa76828 — __ZN6RakNet16ReliabilityLayer10PushPacketEyPNS_14InternalPacketEb
// type: void __fastcall(_DWORD *, unsigned __int64, int, char)
#[doc(alias = "RakNet::ReliabilityLayer::PushPacket(unsigned long long,RakNet::InternalPacket *,bool)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer10PushPacketEyPNS_14InternalPacketEb")]
pub fn stub_0xa76828(state: &mut ReliabilityState, bits: u32, handle: u32) {
    // IDA 0xa76828: `PushPacket` adds the 8-byte-aligned bit cost (0xa7684c)
    // to the pending totals (0xa76852/0xa7685c) and appends the packet with
    // 16-minimum/double growth (0xa7686a..0xa768c0). MODEL: growth folds into
    // `Vec`.
    state.pending_bits += (u64::from(bits) + 7) & !7;
    state.outgoing.push(OutPacket { bits, handle });
}

// 0xa7696c — __ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tES1_y
// type: _DWORD *__fastcall(int, int, _DWORD *, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,RakNet::uint24_t,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tES1_y")]
pub fn stub_0xa7696c(state: &mut ReliabilityState, key: u32, lo: u32, hi: u32) -> u32 {
    // IDA 0xa7696c: `AddFirstToDatagramHistory` — see `push_history` (the
    // queued triple lands at 0xa76a4e..0xa76a58).
    push_history(state, key, lo, hi)
}

// 0xa76a68 — __ZN6RakNet16ReliabilityLayer34WriteToBitStreamFromInternalPacketEPNS_9BitStreamEPKNS_14InternalPacketEy
// type: int __fastcall(int, RakNet::BitStream *this, int)
#[doc(alias = "RakNet::ReliabilityLayer::WriteToBitStreamFromInternalPacket(RakNet::BitStream *,RakNet::InternalPacket const*,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer34WriteToBitStreamFromInternalPacketEPNS_9BitStreamEPKNS_14InternalPacketEy")]
pub fn stub_0xa76a68(packet: &InternalPacket) -> Vec<u8> {
    // IDA 0xa76a68: `WriteToBitStreamFromInternalPacket` — see
    // `write_internal_packet`.
    write_internal_packet(packet)
}

// 0xa76b88 — __ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tEy
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tEy")]
pub fn stub_0xa76b88(state: &mut ReliabilityState, key: u32, lo: u32) -> u32 {
    // IDA 0xa76b88: `AddFirstToDatagramHistory` two-word form — same
    // 0x200-capped ring as 0xa7696c (0xa76bb2..0xa76c3a) with a zeroed third
    // word.
    push_history(state, key, lo, 0)
}

// 0xa76c68 — __ZN6RakNet16ReliabilityLayer21IsOutgoingDataWaitingEv
// type: bool __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::IsOutgoingDataWaiting(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer21IsOutgoingDataWaitingEv")]
pub fn stub_0xa76c68(state: &ReliabilityState) -> bool {
    // IDA 0xa76c68: ready when the word-541 flag is set, else when the
    // word-607 queue is nonempty (0xa76c6a..0xa76c7c).
    state.outgoing_ready || state.send_queue_len != 0
}

// 0xa76c84 — __ZN6RakNet16ReliabilityLayer14AreAcksWaitingEv
// type: bool __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::AreAcksWaiting(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer14AreAcksWaitingEv")]
pub fn stub_0xa76c84(state: &ReliabilityState) -> bool {
    // IDA 0xa76c84: acks wait while the word-970 range list is nonempty
    // (0xa76c8e).
    state.acks_waiting
}

// 0xa76c90 — __ZN6RakNet16ReliabilityLayer31SetSplitMessageProgressIntervalEi
// type: int __fastcall(int this, int)
#[doc(alias = "RakNet::ReliabilityLayer::SetSplitMessageProgressInterval(int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer31SetSplitMessageProgressIntervalEi")]
pub fn stub_0xa76c90(state: &mut ReliabilityState, interval: i32) {
    // IDA 0xa76c90: `SetSplitMessageProgressInterval` stores the interval
    // word (0xa76c90).
    state.split_progress_interval = interval;
}

// 0xa76c94 — __ZN6RakNet16ReliabilityLayer20SetUnreliableTimeoutEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::ReliabilityLayer::SetUnreliableTimeout(unsigned int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer20SetUnreliableTimeoutEj")]
pub fn stub_0xa76c94(state: &mut ReliabilityState, seconds: u32) {
    // IDA 0xa76c94: `SetUnreliableTimeout` stores `1000 * seconds` ms
    // (0xa76c9c).
    state.unreliable_timeout_ms = seconds.saturating_mul(1000);
}

// 0xa76ca4 — __ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEPNS_18SplitPacketChannelEy
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(RakNet::SplitPacketChannel *,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEPNS_18SplitPacketChannelEy")]
pub fn stub_0xa76ca4(
    state: &mut ReliabilityState,
    index: usize,
    _time_lo: u32,
    _time_hi: u32,
) -> Option<InternalPacket> {
    // IDA 0xa76ca4: `BuildPacketFromSplitPacketList` assembles the channel's
    // payloads, releases each fragment and its refcounted bytes
    // (0xa76e02..0xa76e2c), frees the channel (0xa76e4a..0xa76e5c), and
    // answers the assembled packet (0xa76e68). MODEL: timestamps fold into
    // the host.
    let channel = state.split_channels.get(index)?.clone();
    state.split_channels.remove(index);
    Some(assemble_split_channel(&channel))
}

// 0xa76e6c — __ZNK6RakNet16ReliabilityLayer16IsDeadConnectionEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::IsDeadConnection(void)const")]
#[doc(alias = "__ZNK6RakNet16ReliabilityLayer16IsDeadConnectionEv")]
pub fn stub_0xa76e6c(state: &ReliabilityState) -> bool {
    // IDA 0xa76e6c: `IsDeadConnection` answers the flag byte at 2228
    // (0xa76e70).
    state.dead_connection
}

// 0xa76e74 — __ZN6RakNet16ReliabilityLayer13GetStatisticsEPNS_16RakNetStatisticsE
// type: int __fastcall(int, int)
#[doc(alias = "RakNet::ReliabilityLayer::GetStatistics(RakNet::RakNetStatistics *)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer13GetStatisticsEPNS_16RakNetStatisticsE")]
pub fn stub_0xa76e74(state: &ReliabilityState) -> ReliabilityStats {
    // IDA 0xa76e74: `GetStatistics` snapshots the running counters into the
    // out struct — see `ReliabilityStats`.
    ReliabilityStats {
        datagrams: state.datagrams,
        received: state.received,
        resend_pending: state.resend.len(),
        output_queued: state.output.len(),
        acked_bytes: state.acked_bytes,
        bytes_queued: state.bytes_queued,
    }
}

#[cfg(test)]
mod reliability_pump_batch_tests {
    use super::*;

    #[test]
    fn build_complete_channel_assembles() {
        let mut st = stub_0xa70938();
        assert_eq!(stub_0xa749fc(&mut st, 9, 100), 0);
        assert_eq!(stub_0xa749fc(&mut st, 9, 101), 0);
        assert!(stub_0xa74c88(&mut st, 9, 3).is_none());
        let pkt = stub_0xa74c88(&mut st, 9, 2).expect("complete channel builds");
        assert_eq!(pkt.data.len(), 8);
        assert_eq!(&pkt.data[..4], &100u32.to_le_bytes());
        assert_eq!(&pkt.data[4..], &101u32.to_le_bytes());
        assert_eq!(pkt.length_bits, 64);
        assert!(st.split_channels.is_empty());
        assert_eq!(st.acks_sent, 1);
        assert!(stub_0xa74c88(&mut st, 9, 2).is_none());
    }

    #[test]
    fn build_from_channel_index() {
        let mut st = stub_0xa70938();
        stub_0xa749fc(&mut st, 4, 7);
        let pkt = stub_0xa76ca4(&mut st, 0, 0, 0).expect("index builds");
        assert_eq!(pkt.data, 7u32.to_le_bytes());
        assert!(st.split_channels.is_empty());
        assert!(stub_0xa76ca4(&mut st, 0, 0, 0).is_none());
    }

    #[test]
    fn receive_pops_output() {
        let mut st = stub_0xa70938();
        assert!(stub_0xa74d64(&mut st).is_none());
        assert!(stub_0xa72e94(&mut st, Some(&[1, 2, 3])));
        let (bytes, bits) = stub_0xa74d64(&mut st).expect("queued datagram");
        assert_eq!(bytes, vec![1, 2, 3]);
        assert_eq!(bits, 24);
        assert!(stub_0xa74d64(&mut st).is_none());
    }

    #[test]
    fn send_clamps_queues_and_splits() {
        let mut st = stub_0xa70938();
        assert!(!stub_0xa74dc0(&mut st, &[], 0, 2, 0, 1500));
        assert!(stub_0xa74dc0(&mut st, &[1, 2], 9, 9, 0xff, 1500));
        let entry = &st.send_heap[0];
        assert_eq!(entry.priority, 1);
        assert_eq!(entry.packet.reliability, 2);
        assert_eq!(entry.packet.ordering_channel, 0);
        assert_eq!(entry.packet.message_number, 0);
        assert_eq!(st.message_number, 1);
        assert_eq!(st.bytes_queued, 1);
        let big = vec![0xabu8; 100];
        assert!(stub_0xa74dc0(&mut st, &big, 0, 2, 0, 50));
        assert_eq!(st.send_heap.len(), 1 + 4);
        let frag = &st.send_heap[1].packet;
        assert!(frag.has_split);
        assert_eq!((frag.split_index, frag.split_count), (0, 4));
        assert_eq!(st.send_heap[4].packet.split_index, 3);
        assert_eq!(stub_0xa75100(&mut st, &[9u8; 10], 0, 0, 3, 1000), 1);
        assert_eq!(st.send_heap.len(), 1 + 4 + 1);
    }

    #[test]
    fn update_and_ack_timeout() {
        let mut st = stub_0xa70938();
        stub_0xa723f8(&mut st, 5000);
        st.last_ack_ms = 1000;
        assert!(stub_0xa75548(&mut st, false));
        assert_eq!(st.update_count, 1);
        assert!(!st.dead_connection);
        assert!(!stub_0xa7641c(&st, 2000));
        assert!(stub_0xa7641c(&st, 7000));
        assert!(!stub_0xa7641c(&st, 500));
        st.last_ack_ms = 0x10000;
        assert!(!stub_0xa7641c(&st, 0x10000 - 100));
        assert!(stub_0xa7641c(&st, 0x10000 - 0x2711));
        assert!(stub_0xa7641c(&st, u64::from(u32::MAX) + 100_000));
        assert!(stub_0xa75548(&mut st, true));
        assert!(st.dead_connection);
        assert!(stub_0xa76e6c(&st));
    }

    #[test]
    fn ack_send_flow() {
        let mut st = stub_0xa70938();
        assert!(!stub_0xa76c84(&st));
        assert_eq!(stub_0xa76468(&mut st), 0);
        st.acks_waiting = true;
        st.pending_ack_ranges = 3;
        assert!(stub_0xa76c84(&st));
        assert_eq!(stub_0xa76468(&mut st), 3);
        assert!(!stub_0xa76c84(&st));
        assert_eq!(st.acks_sent, 1);
    }

    #[test]
    fn reset_packets_keeps_config() {
        let mut st = stub_0xa70938();
        stub_0xa723f8(&mut st, 4242);
        stub_0xa76c90(&mut st, 7);
        st.output.push_back(vec![1]);
        st.resend.push(ResendEntry { number: 1, bits: 8, reliability: 0 });
        st.split_channels.push(SplitChannel { id: 1, packets: vec![2] });
        st.send_heap.push(SendEntry::default());
        st.outgoing.push(OutPacket { bits: 8, handle: 1 });
        stub_0xa765e0(&mut st);
        assert!(st.output.is_empty());
        assert!(st.resend.is_empty());
        assert!(st.split_channels.is_empty());
        assert!(st.send_heap.is_empty());
        assert!(st.outgoing.is_empty());
        assert_eq!(st.timeout_ms, 4242);
        assert_eq!(st.split_progress_interval, 7);
        stub_0xa76c94(&mut st, 30);
        assert_eq!(st.unreliable_timeout_ms, 30_000);
    }

    #[test]
    fn datagram_push_history_and_flags() {
        let mut st = stub_0xa70938();
        assert_eq!(stub_0xa766b8(&mut st, 11), 0);
        assert!(st.datagram_table.is_empty());
        stub_0xa76828(&mut st, 80, 5);
        assert_eq!(st.pending_bits, 80);
        assert_eq!(st.outgoing, vec![OutPacket { bits: 80, handle: 5 }]);
        assert_eq!(stub_0xa766b8(&mut st, 11), 80);
        assert_eq!(st.datagram_table, vec![11]);
        assert!(!stub_0xa76c68(&st));
        st.outgoing_ready = true;
        assert!(stub_0xa76c68(&st));
        st.outgoing_ready = false;
        st.send_queue_len = 5;
        assert!(stub_0xa76c68(&st));
        assert!(!stub_0xa76e6c(&st));
        let first = stub_0xa7696c(&mut st, 0xabcd, 1, 2);
        assert_eq!(stub_0xa76b88(&mut st, 0x1234, 3), first + 1);
        for i in 0..520u32 {
            stub_0xa7696c(&mut st, i, i, i);
        }
        assert_eq!(st.datagram_history.len(), 513);
        assert_eq!(
            st.datagram_history.back(),
            Some(&HistoryEntry { key: 519, lo: 519, hi: 519 })
        );
    }

    #[test]
    fn write_read_round_trip_with_remap() {
        let pkt = InternalPacket {
            reliability: 2,
            length_bits: 16,
            message_number: 9,
            data: vec![0xDE, 0xAD],
            ..InternalPacket::default()
        };
        let wire = stub_0xa76a68(&pkt);
        let back = create_internal_packet(&wire).expect("round trip");
        assert_eq!(back.reliability, 2);
        assert_eq!(back.message_number, 9);
        assert_eq!(back.data, vec![0xDE, 0xAD]);
        let seven = InternalPacket { reliability: 7, ..pkt.clone() };
        let wire7 = stub_0xa76a68(&seven);
        assert_eq!(wire7[0] & 7, 3);
        let back2 = stub_0xa74750(&wire).expect("stub parse");
        assert_eq!(back2.message_number, 9);
    }

    #[test]
    fn stats_snapshot() {
        let mut st = stub_0xa70938();
        stub_0xa72e94(&mut st, Some(&[1, 2, 3]));
        st.acked_bytes = 12.0;
        st.bytes_queued = 4;
        let stats = stub_0xa76e74(&st);
        assert_eq!(stats.datagrams, 1);
        assert_eq!(stats.received, 1);
        assert_eq!(stats.output_queued, 1);
        assert_eq!(stats.acked_bytes, 12.0);
        assert_eq!(stats.bytes_queued, 4);
    }
}

/// One `RangeNode<uint24>` [min, max] pair (IDA 0xa7738c, 8-byte entries).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RangeNode {
    pub min: u32,
    pub max: u32,
}

/// `DataStructures::RangeList<uint24>` sorted coalesced range set
/// (IDA 0xa771e8..0xa77b3c). Bounds wrap at 24 bits in comparisons.
#[derive(Debug, Default)]
pub struct RangeList {
    pub nodes: Vec<RangeNode>,
}

/// `RangeList::Insert` (IDA 0xa7738c): contained values are absorbed
/// (0xa774aa); `value == max + 1` extends the range (0xa7744e..0xa77450,
/// `& 0xffffff` wrap at 0xa77448); larger values take a sorted slot for a
/// singleton (0xa7748c..0xa774c4); a following range starting at `value + 1`
/// is absorbed with shift-down (0xa7755c..0xa775a0).
pub fn range_insert(list: &mut RangeList, value: u32) {
    let v = value & 0x00ff_ffff;
    let contained = list.nodes.iter().any(|n| {
        if n.min <= n.max {
            v >= n.min && v <= n.max
        } else {
            // Wrapping range (cf. the `& 0xffffff` arithmetic at
            // 0xa77448/0xa7756c): values past either end are inside.
            v >= n.min || v <= n.max
        }
    });
    if contained {
        return;
    }
    for i in 0..list.nodes.len() {
        if (list.nodes[i].max + 1) & 0x00ff_ffff == v {
            list.nodes[i].max = v;
            if list.nodes.get(i + 1).is_some_and(|next| next.min == (v + 1) & 0x00ff_ffff) {
                let absorbed = list.nodes.remove(i + 1);
                list.nodes[i].max = absorbed.max;
            }
            return;
        }
        if list.nodes[i].min == (v + 1) & 0x00ff_ffff {
            list.nodes[i].min = v;
            return;
        }
    }
    let pos = list.nodes.iter().position(|n| v < n.min).unwrap_or(list.nodes.len());
    list.nodes.insert(pos, RangeNode { min: v, max: v });
}

fn put_u24_le(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes()[..3]);
}

fn get_u24_le(wire: &[u8], off: &mut usize) -> Option<u32> {
    let b = wire.get(*off..*off + 3)?;
    *off += 3;
    Some(u32::from_le_bytes([b[0], b[1], b[2], 0]))
}

/// `RangeList::Serialize` (IDA 0xa77b3c): packs ranges while `used + 81`
/// stays within `max_bits` (0xa77bc8) as flag byte (min == max,
/// 0xa77be2..0xa77bf2) plus uint24 min (0xa77c06) plus uint24 max unless
/// singleton (0xa77c1a..0xa77c2a), prefixed by the u16 range count
/// (0xa77c78). With `remove_written` the packed prefix drains from the list
/// (0xa77c92..0xa77cd6). Returns the wire bytes (the original answers the
/// bit count, 0xa77d18). MODEL: byte-aligned; the temp-stream fold is elided.
pub fn serialize_ranges(list: &mut RangeList, max_bits: u32, remove_written: bool) -> Vec<u8> {
    let mut body = Vec::new();
    let mut used = 0u32;
    let mut count = 0u16;
    for node in &list.nodes {
        if used + 81 > max_bits {
            break;
        }
        let singleton = node.min == node.max;
        body.push(u8::from(singleton));
        put_u24_le(&mut body, node.min);
        used += 8 + 24;
        if !singleton {
            put_u24_le(&mut body, node.max);
            used += 24;
        }
        count += 1;
    }
    if remove_written {
        list.nodes.drain(..count as usize);
    }
    let mut wire = Vec::with_capacity(2 + body.len());
    wire.extend_from_slice(&count.to_le_bytes());
    wire.extend_from_slice(&body);
    wire
}

/// `RangeList::Deserialize` (IDA 0xa771e8): clears the list (0xa771f6..
/// 0xa77214, the >0x200 trim folds into the host), reads the u16 count
/// (0xa7722a), then per range a flag byte (0xa77254) plus uint24 min
/// (0xa77260) plus uint24 max unless the flag marks a singleton
/// (0xa7726c..0xa7728c, max below min fails at 0xa77286), appended raw
/// (0xa77298). Answers false on short reads. MODEL: byte-aligned.
pub fn deserialize_ranges(list: &mut RangeList, wire: &[u8]) -> bool {
    list.nodes.clear();
    if wire.len() < 2 {
        return false;
    }
    let count = u16::from_le_bytes([wire[0], wire[1]]) as usize;
    let mut off = 2usize;
    for _ in 0..count {
        let flag = match wire.get(off) {
            Some(&b) => b,
            None => return false,
        };
        off += 1;
        let min = match get_u24_le(wire, &mut off) {
            Some(v) => v,
            None => return false,
        };
        let max = if flag != 0 {
            min
        } else {
            match get_u24_le(wire, &mut off) {
                Some(m) if m >= min => m,
                _ => return false,
            }
        };
        list.nodes.push(RangeNode { min, max });
    }
    true
}

/// Min-heap entry for `Heap<u64, InternalPacket*, false>` (IDA 0xa77784):
/// 12-byte nodes of u64 key plus packet handle; `false` selects min order
/// (sift breaks while the parent key is not greater, 0xa77a30..0xa77a36).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HeapEntry {
    pub key: u64,
    pub value: u32,
}

/// `DataStructures::Heap` array plus the bulk-append flag (IDA 0xa78196).
#[derive(Debug, Default)]
pub struct TimeHeap {
    pub entries: Vec<HeapEntry>,
    pub unordered: bool,
}

fn heap_sift_up(heap: &mut [HeapEntry], mut pos: usize) {
    while pos > 0 {
        let parent = (pos - 1) >> 1;
        if heap[parent].key <= heap[pos].key {
            break;
        }
        heap.swap(parent, pos);
        pos = parent;
    }
}

fn heap_sift_down(heap: &mut [HeapEntry], mut pos: usize) {
    let len = heap.len();
    loop {
        let left = pos * 2 + 1;
        if left >= len {
            break;
        }
        let right = left + 1;
        let mut smallest = left;
        if right < len && heap[right].key < heap[left].key {
            smallest = right;
        }
        if heap[pos].key <= heap[smallest].key {
            break;
        }
        heap.swap(pos, smallest);
        pos = smallest;
    }
}

/// `Heap::Push` (IDA 0xa77950): appends with 16-minimum/double growth
/// (0xa77978..0xa77984, folds into `Vec`) and sifts up (0xa77a02..0xa77a78),
/// answering the new length (0xa779fa).
pub fn heap_push(heap: &mut TimeHeap, key: u64, value: u32) -> u32 {
    heap.entries.push(HeapEntry { key, value });
    let last = heap.entries.len() - 1;
    heap_sift_up(&mut heap.entries, last);
    heap.entries.len() as u32
}

/// `Heap::Pop` (IDA 0xa77784): lifts the entry at `index`, moves the last
/// node into the hole (0xa777ae..0xa777b8), restores order
/// (0xa777e0..0xa77910), and answers the lifted packet handle
/// (0xa777ac/0xa7794e). Bulk-appended arrays rebuild first (the unordered
/// flag path at 0xa78000 folds into a sort). Out-of-range answers `None`
/// (the original faults).
pub fn heap_pop(heap: &mut TimeHeap, index: usize) -> Option<u32> {
    if heap.unordered {
        let mut ordered = std::mem::take(&mut heap.entries);
        ordered.sort_by_key(|e| e.key);
        heap.entries = ordered;
        heap.unordered = false;
    }
    if index >= heap.entries.len() {
        return None;
    }
    let value = heap.entries[index].value;
    heap.entries.swap_remove(index);
    if index < heap.entries.len() {
        heap_sift_down(&mut heap.entries, index);
        heap_sift_up(&mut heap.entries, index);
    }
    Some(value)
}

/// `Heap::PushSeries` (IDA 0xa77ff4): bulk-appends with 16-minimum/double
/// growth and marks the array unordered (0xa78190..0xa78196), answering 1
/// (0xa78194). Growth folds into `Vec`.
pub fn heap_push_series(heap: &mut TimeHeap, key: u64, value: u32) -> u32 {
    heap.entries.push(HeapEntry { key, value });
    heap.unordered = true;
    1
}

/// `RakNet::BitStream` bit cursor (IDA 0xa77d60/0xa77ea4): byte store plus
/// bit position. The network-order guard folds into host little-endian
/// (0xa77dda..0xa77e16 / 0xa77f22..0xa77f64).
#[derive(Debug, Default)]
pub struct BitCursor {
    pub bytes: Vec<u8>,
    pub bit_pos: usize,
}

impl BitCursor {
    fn align(&mut self) {
        self.bit_pos = (self.bit_pos + 7) & !7;
    }

    /// `Write<uint24>` (IDA 0xa77d60): aligns (0xa77d8e), reserves 24 bits
    /// (0xa77d94), and stores the 3 bytes (0xa77e20..0xa77e5c).
    pub fn write_u24(&mut self, value: u32) {
        self.align();
        let at = self.bit_pos / 8;
        if self.bytes.len() < at + 3 {
            self.bytes.resize(at + 3, 0);
        }
        self.bytes[at..at + 3].copy_from_slice(&value.to_le_bytes()[..3]);
        self.bit_pos += 24;
    }

    /// `Read<uint24>` (IDA 0xa77ea4): aligns (0xa77ee0), needs 24 bits
    /// (0xa77f0a), loads the 3 bytes (0xa77f5e..0xa77faa), and answers
    /// `None` past the end.
    pub fn read_u24(&mut self) -> Option<u32> {
        self.align();
        let at = self.bit_pos / 8;
        let b = self.bytes.get(at..at + 3)?;
        let v = u32::from_le_bytes([b[0], b[1], b[2], 0]);
        self.bit_pos += 24;
        Some(v)
    }
}

/// `DatagramHeaderFormat` wire view (IDA 0xa77058/0xa77a84): the lead marker
/// (+14, 0xa7708c), ack flag (+8, 0xa770ba/0xa77a92), float flag (+11,
/// 0xa770fc/0xa77a9c) with the f32 payload at +4 (0xa77116..0xa7711c /
/// 0xa77ade), short flag (+9, 0xa7713e/0xa77aaa) with the trailing 1
/// (0xa77146..0xa7714e / 0xa77ab2..0xa77ab6), the three flag bits
/// (+10/+12/+13, 0xa77174.. / 0xa77aea..0xa77b1c), and the uint24 datagram
/// number at +0 (0xa77b34). MODEL: bits travel LSB-first in a `Vec<bool>`;
/// byte alignment folds into the host.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DatagramHeader {
    pub lead: bool,
    pub is_ack: bool,
    pub has_float: bool,
    pub float_value: f32,
    pub short: bool,
    pub flag_a: bool,
    pub flag_b: bool,
    pub flag_c: bool,
    pub number: u32,
}

fn push_aligned_bits(bits: &mut Vec<bool>, bytes: &[u8]) {
    while bits.len() % 8 != 0 {
        bits.push(false);
    }
    for byte in bytes {
        for i in 0..8 {
            bits.push((byte >> i) & 1 != 0);
        }
    }
}

/// `DatagramHeaderFormat::Serialize` (IDA 0xa77a84).
pub fn serialize_header(h: &DatagramHeader) -> Vec<bool> {
    let mut bits = vec![true];
    if h.is_ack {
        bits.push(true);
        bits.push(h.has_float);
        if h.has_float {
            push_aligned_bits(&mut bits, &h.float_value.to_le_bytes());
        }
        return bits;
    }
    bits.push(false);
    bits.push(h.short);
    if h.short {
        bits.push(true);
        return bits;
    }
    bits.push(false);
    bits.push(h.flag_a);
    bits.push(h.flag_b);
    bits.push(h.flag_c);
    push_aligned_bits(&mut bits, &h.number.to_le_bytes()[..3]);
    bits
}

struct BitReader<'a> {
    bits: &'a [bool],
    pos: usize,
}

impl BitReader<'_> {
    fn next(&mut self) -> Option<bool> {
        let b = *self.bits.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }

    fn bytes(&mut self, n: usize) -> Option<Vec<u8>> {
        while self.pos % 8 != 0 {
            self.pos += 1;
        }
        if self.pos + 8 * n > self.bits.len() {
            return None;
        }
        let mut out = vec![0u8; n];
        for byte in &mut out {
            let mut v = 0u8;
            for i in 0..8 {
                if self.bits[self.pos] {
                    v |= 1 << i;
                }
                self.pos += 1;
            }
            *byte = v;
        }
        Some(out)
    }
}

/// `DatagramHeaderFormat::Deserialize` (IDA 0xa77058): mirrors
/// `serialize_header` bit for bit (lead at 0xa7708c, ack at 0xa770ba, float
/// flag at 0xa770fc with the f32 at 0xa77116.., short at 0xa7713e with the
/// trailing 1 at 0xa77146.., flag bits from 0xa77174 on). Answers `None`
/// past the end (the original reads unconditionally).
pub fn deserialize_header(bits: &[bool]) -> Option<(DatagramHeader, usize)> {
    let mut r = BitReader { bits, pos: 0 };
    let mut h = DatagramHeader::default();
    h.lead = r.next()?;
    h.is_ack = r.next()?;
    if h.is_ack {
        h.has_float = r.next()?;
        if h.has_float {
            let b = r.bytes(4)?;
            h.float_value = f32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        }
        return Some((h, r.pos));
    }
    h.short = r.next()?;
    if h.short {
        if !r.next()? {
            return None;
        }
        return Some((h, r.pos));
    }
    if r.next()? {
        return None;
    }
    h.flag_a = r.next()?;
    h.flag_b = r.next()?;
    h.flag_c = r.next()?;
    let b = r.bytes(3)?;
    h.number = u32::from_le_bytes([b[0], b[1], b[2], 0]);
    Some((h, r.pos))
}

/// Ordered `SplitPacketChannel*` list keyed by split id (IDA 0xa781a4).
#[derive(Debug, Default)]
pub struct OrderedSplitList {
    pub ids: Vec<u16>,
    pub handles: Vec<u32>,
}

/// `OrderedList::Insert` (IDA 0xa781a4): binary-searches with
/// `SplitPacketChannelComp` (0xa781da, via `stub_0xa7090c`); a miss inserts
/// through `List::Insert` at the probed slot (0xa7820e, 16-minimum/double
/// growth folds into `Vec`) and answers the slot (0xa78214); a hit answers
/// the existing index with no insert (the `break` at 0xa781de falls out of
/// the search).
pub fn ordered_insert(list: &mut OrderedSplitList, key: u16, handle: u32) -> usize {
    match list.ids.binary_search_by(|id| stub_0xa7090c(*id, key).cmp(&0)) {
        Ok(pos) => pos,
        Err(pos) => {
            list.ids.insert(pos, key);
            list.handles.insert(pos, handle);
            pos
        }
    }
}

/// `RakNet::SignaledEvent` latch (IDA 0xa79900..0xa7999c): the mutex/cond
/// plumbing folds into the host; only the signaled flag (+44, 0xa7990c) and
/// the init latch are observed.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct SignaledEvent {
    pub signaled: bool,
    pub inited: bool,
}

// 0xa77058 — __ZN20DatagramHeaderFormat11DeserializeEPN6RakNet9BitStreamE
// type: _DWORD __fastcall(DatagramHeaderFormat *__hidden this, RakNet::BitStream *)
#[doc(alias = "DatagramHeaderFormat::Deserialize(RakNet::BitStream *)")]
#[doc(alias = "__ZN20DatagramHeaderFormat11DeserializeEPN6RakNet9BitStreamE")]
pub fn stub_0xa77058(bits: &[bool]) -> Option<(DatagramHeader, usize)> {
    // IDA 0xa77058: `DatagramHeaderFormat::Deserialize` — see
    // `deserialize_header`.
    deserialize_header(bits)
}

// 0xa771e8 — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE11DeserializeEPNS1_9BitStreamE
// type: int __fastcall(_DWORD *, RakNet::BitStream *, int, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Deserialize(RakNet::BitStream *)")]
#[doc(alias = "__ZN14DataStructures9RangeListIN6RakNet8uint24_tEE11DeserializeEPNS1_9BitStreamE")]
pub fn stub_0xa771e8(list: &mut RangeList, wire: &[u8]) -> bool {
    // IDA 0xa771e8: `RangeList::Deserialize` — see `deserialize_ranges`.
    deserialize_ranges(list, wire)
}

// 0xa772b8 — __ZN14DataStructures5QueueIPN6RakNet14InternalPacketEE4PushERKS3_PKcj
// type: void __fastcall(int **, int *)
#[doc(alias = "DataStructures::Queue<RakNet::InternalPacket *>::Push(RakNet::InternalPacket * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIPN6RakNet14InternalPacketEE4PushERKS3_PKcj")]
pub fn stub_0xa772b8(queue: &mut RakPtrQueue, value: u32) {
    // IDA 0xa772b8: `Queue<InternalPacket*>::Push` — the same ring store,
    // wrap, and double-on-full as 0xa6ccdc (0xa772c6..0xa7737e).
    queue.push(value);
}

// 0xa7738c — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE6InsertES2_
// type: void __fastcall(int *, unsigned int *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Insert(RakNet::uint24_t)")]
#[doc(alias = "__ZN14DataStructures9RangeListIN6RakNet8uint24_tEE6InsertES2_")]
pub fn stub_0xa7738c(list: &mut RangeList, value: u32) {
    // IDA 0xa7738c: `RangeList::Insert` — see `range_insert`.
    range_insert(list, value);
}

// 0xa77784 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE3PopEj
// type: int __fastcall(int *, unsigned int)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Pop(unsigned int)")]
#[doc(alias = "__ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE3PopEj")]
pub fn stub_0xa77784(heap: &mut TimeHeap, index: usize) -> Option<u32> {
    // IDA 0xa77784: `Heap::Pop` — see `heap_pop`.
    heap_pop(heap, index)
}

// 0xa77950 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE4PushERKyRKS3_PKcj
// type: unsigned int __fastcall(char **, int *, int *)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Push(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE4PushERKyRKS3_PKcj")]
pub fn stub_0xa77950(heap: &mut TimeHeap, key: u64, value: u32) -> u32 {
    // IDA 0xa77950: `Heap::Push` — see `heap_push`.
    heap_push(heap, key, value)
}

// 0xa77a84 — __ZN20DatagramHeaderFormat9SerializeEPN6RakNet9BitStreamE
// type: int __fastcall(DatagramHeaderFormat *this, RakNet::BitStream *)
#[doc(alias = "DatagramHeaderFormat::Serialize(RakNet::BitStream *)")]
#[doc(alias = "__ZN20DatagramHeaderFormat9SerializeEPN6RakNet9BitStreamE")]
pub fn stub_0xa77a84(header: &DatagramHeader) -> Vec<bool> {
    // IDA 0xa77a84: `DatagramHeaderFormat::Serialize` — see
    // `serialize_header`.
    serialize_header(header)
}

// 0xa77b3c — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE9SerializeEPNS1_9BitStreamEjb
// type: int __fastcall(int *, RakNet::BitStream *, unsigned int, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Serialize(RakNet::BitStream *,unsigned int,bool)")]
#[doc(alias = "__ZN14DataStructures9RangeListIN6RakNet8uint24_tEE9SerializeEPNS1_9BitStreamEjb")]
pub fn stub_0xa77b3c(list: &mut RangeList, max_bits: u32, remove_written: bool) -> Vec<u8> {
    // IDA 0xa77b3c: `RangeList::Serialize` — see `serialize_ranges`.
    serialize_ranges(list, max_bits, remove_written)
}

// 0xa77d60 — __ZN6RakNet9BitStream5WriteINS_8uint24_tEEEvRKT_
// type: void __fastcall(RakNet::BitStream *this, _BYTE *, int, int, int)
#[doc(alias = "void RakNet::BitStream::Write<RakNet::uint24_t>(RakNet::uint24_t const&)")]
#[doc(alias = "__ZN6RakNet9BitStream5WriteINS_8uint24_tEEEvRKT_")]
pub fn stub_0xa77d60(cursor: &mut BitCursor, value: u32) {
    // IDA 0xa77d60: `BitStream::Write<uint24>` — see
    // `BitCursor::write_u24`.
    cursor.write_u24(value);
}

// 0xa77ea4 — __ZN6RakNet9BitStream4ReadINS_8uint24_tEEEbRT_
// type: int __fastcall(_DWORD *, _BYTE *)
#[doc(alias = "bool RakNet::BitStream::Read<RakNet::uint24_t>(RakNet::uint24_t &)")]
#[doc(alias = "__ZN6RakNet9BitStream4ReadINS_8uint24_tEEEbRT_")]
pub fn stub_0xa77ea4(cursor: &mut BitCursor) -> Option<u32> {
    // IDA 0xa77ea4: `BitStream::Read<uint24>` — see `BitCursor::read_u24`.
    cursor.read_u24()
}

// 0xa77ff4 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE10PushSeriesERKyRKS3_PKcj
// type: unsigned int __fastcall(int, int *, int *)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::PushSeries(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE10PushSeriesERKyRKS3_PKcj")]
pub fn stub_0xa77ff4(heap: &mut TimeHeap, key: u64, value: u32) -> u32 {
    // IDA 0xa77ff4: `Heap::PushSeries` — see `heap_push_series`.
    heap_push_series(heap, key, value)
}

// 0xa781a4 — __ZN14DataStructures11OrderedListItPN6RakNet18SplitPacketChannelEXadL_ZNS1_22SplitPacketChannelCompERKtRKS3_EEE6InsertES5_S7_bPKcjPFiS5_S7_E
// type: unsigned int __fastcall(int **, int, int *, int, int, int, int (__fastcall *)(int, int))
#[doc(alias = "DataStructures::OrderedList<unsigned short,RakNet::SplitPacketChannel *,&RakNet::SplitPacketChannelComp>::Insert(unsigned short const&,RakNet::SplitPacketChannel * const&,bool,char const*,unsigned int,int (*)(unsigned short const&,RakNet::SplitPacketChannel * const&))")]
#[doc(alias = "__ZN14DataStructures11OrderedListItPN6RakNet18SplitPacketChannelEXadL_ZNS1_22SplitPacketChannelCompERKtRKS3_EEE6InsertES5_S7_bPKcjPFiS5_S7_E")]
pub fn stub_0xa781a4(list: &mut OrderedSplitList, key: u16, handle: u32) -> usize {
    // IDA 0xa781a4: `OrderedList::Insert` — see `ordered_insert`.
    ordered_insert(list, key, handle)
}

// 0xa7828c — __ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacket>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE8AllocateEPKcj")]
pub fn stub_0xa7828c(pool: &mut RakMemPool) -> u32 {
    // IDA 0xa7828c: `MemoryPool<InternalPacket>::Allocate` — same
    // pop-or-grow shape as 0xa6c7d0.
    pool.allocate()
}

// 0xa783b4 — __ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE7ReleaseEPS2_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacket>::Release(RakNet::InternalPacket*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE7ReleaseEPS2_PKcj")]
pub fn stub_0xa783b4(pool: &mut RakMemPool, slot: u32) {
    // IDA 0xa783b4: `MemoryPool<InternalPacket>::Release` — same recycle
    // shape as 0xa6c8e4.
    pool.release(slot);
}

// 0xa7848c — __ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE7ReleaseEPS3_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Release(RakNet::ReliabilityLayer::MessageNumberNode*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE7ReleaseEPS3_PKcj")]
pub fn stub_0xa7848c(pool: &mut RakMemPool, slot: u32) {
    // IDA 0xa7848c: `MemoryPool<MessageNumberNode>::Release` — same recycle
    // shape as 0xa6c8e4.
    pool.release(slot);
}

// 0xa78560 — __ZN14DataStructures5QueueIN6RakNet16ReliabilityLayer19DatagramHistoryNodeEE4PushERKS3_PKcj
// type: void __fastcall(_DWORD *, __int64 *)
#[doc(alias = "DataStructures::Queue<RakNet::ReliabilityLayer::DatagramHistoryNode>::Push(RakNet::ReliabilityLayer::DatagramHistoryNode const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIN6RakNet16ReliabilityLayer19DatagramHistoryNodeEE4PushERKS3_PKcj")]
pub fn stub_0xa78560(queue: &mut RakPtrQueue, value: u32) {
    // IDA 0xa78560: `Queue<DatagramHistoryNode>::Push` — the same ring
    // store, wrap, and double-on-full as 0xa6ccdc.
    queue.push(value);
}

// 0xa78670 — __ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE8AllocateEPKcj")]
pub fn stub_0xa78670(pool: &mut RakMemPool) -> u32 {
    // IDA 0xa78670: `MemoryPool<MessageNumberNode>::Allocate` — same
    // pop-or-grow shape as 0xa6c7d0.
    pool.allocate()
}

// 0xa7879c — __ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE8AllocateEPKcj")]
pub fn stub_0xa7879c(pool: &mut RakMemPool) -> u32 {
    // IDA 0xa7879c: `MemoryPool<InternalPacketRefCountedData>::Allocate` —
    // same pop-or-grow shape as 0xa6c7d0.
    pool.allocate()
}

// 0xa788c8 — __ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE7ReleaseEPS2_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Release(RakNet::InternalPacketRefCountedData*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE7ReleaseEPS2_PKcj")]
pub fn stub_0xa788c8(pool: &mut RakMemPool, slot: u32) {
    // IDA 0xa788c8: `MemoryPool<InternalPacketRefCountedData>::Release` —
    // same recycle shape as 0xa6c8e4.
    pool.release(slot);
}

// 0xa7899c — __ZN14DataStructures4ListIPN6RakNet18SplitPacketChannelEE6InsertERKS3_jPKcj
// type: unsigned int __fastcall(int, _DWORD *, int)
#[doc(alias = "DataStructures::List<RakNet::SplitPacketChannel *>::Insert(RakNet::SplitPacketChannel * const&,unsigned int,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListIPN6RakNet18SplitPacketChannelEE6InsertERKS3_jPKcj")]
pub fn stub_0xa7899c(list: &mut RakList<u32>, value: u32) {
    // IDA 0xa7899c: `List<SplitPacketChannel*>::Insert` — same 16/double
    // growth and append as 0xa6ced8.
    list.insert(value);
}

// 0xa78a2c — __ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_jPKcj
// type: int __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,unsigned int,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_jPKcj")]
pub fn stub_0xa78a2c(list: &mut RakList<u64>, min: u32, max: u32) {
    // IDA 0xa78a2c: `List<RangeNode<uint24>>::Insert` with an explicit slot —
    // same 16/double growth and append as 0xa6ced8; the node packs into one
    // `u64`. No coalescing (that lives in `range_insert`).
    list.insert((u64::from(max) << 32) | u64::from(min));
}

// 0xa78b08 — __ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_PKcj
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_PKcj")]
pub fn stub_0xa78b08(list: &mut RakList<u64>, min: u32, max: u32) {
    // IDA 0xa78b08: `List<RangeNode<uint24>>::Insert` — same raw append as
    // 0xa78a2c.
    list.insert((u64::from(max) << 32) | u64::from(min));
}

// 0xa78bbc — __ZN14DataStructures5QueueIN6RakNet10BPSTracker13TimeAndValue2EE4PushERKS3_PKcj
// type: _QWORD *__fastcall(int *, _QWORD *)
#[doc(alias = "DataStructures::Queue<RakNet::BPSTracker::TimeAndValue2>::Push(RakNet::BPSTracker::TimeAndValue2 const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIN6RakNet10BPSTracker13TimeAndValue2EE4PushERKS3_PKcj")]
pub fn stub_0xa78bbc(queue: &mut RakPtrQueue, value: u32) {
    // IDA 0xa78bbc: `Queue<TimeAndValue2>::Push` — the same ring store,
    // wrap, and double-on-full as 0xa6ccdc.
    queue.push(value);
}

// 0xa79900 — __ZN6RakNet13SignaledEventC1Ev
// type: RakNet::SignaledEvent *__fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::SignaledEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEventC1Ev")]
pub fn stub_0xa79900() -> SignaledEvent {
    // IDA 0xa79900: ctor inits the mutex (0xa79906) and clears the signaled
    // flag (0xa7990c).
    SignaledEvent::default()
}

// 0xa79914 — __ZN6RakNet13SignaledEventD1Ev
// type: void __fastcall(RakNet::SignaledEvent *__hidden this)
#[doc(alias = "RakNet::SignaledEvent::~SignaledEvent()")]
#[doc(alias = "__ZN6RakNet13SignaledEventD1Ev")]
pub fn stub_0xa79914(event: &mut SignaledEvent) {
    // IDA 0xa79914: dtor destroys the mutex (0xa7991a); drop glue covers it
    // and the handle is marked unusable.
    event.inited = false;
}

// 0xa79924 — __ZN6RakNet13SignaledEvent9InitEventEv
// type: int __fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::InitEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent9InitEventEv")]
pub fn stub_0xa79924(event: &mut SignaledEvent) -> i32 {
    // IDA 0xa79924: `InitEvent` inits the cond/mutex attrs and objects
    // (0xa79930..0xa79952, fold into the host) and answers success.
    event.inited = true;
    0
}

// 0xa79954 — __ZN6RakNet13SignaledEvent10CloseEventEv
// type: int __fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::CloseEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent10CloseEventEv")]
pub fn stub_0xa79954(event: &mut SignaledEvent) -> i32 {
    // IDA 0xa79954: `CloseEvent` destroys the cond/mutex objects and attrs
    // (0xa7995e..0xa7997a, fold into the host) and answers success.
    event.inited = false;
    0
}

// 0xa7997c — __ZN6RakNet13SignaledEvent8SetEventEv
// type: int __fastcall(pthread_cond_t *this)
#[doc(alias = "RakNet::SignaledEvent::SetEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent8SetEventEv")]
pub fn stub_0xa7997c(event: &mut SignaledEvent) -> i32 {
    // IDA 0xa7997c: `SetEvent` locks (0xa79982), raises the flag (0xa79988),
    // unlocks (0xa7998e), and broadcasts (0xa7999a, folds into the host).
    event.signaled = true;
    0
}

// 0xa7999c — __ZN6RakNet13SignaledEvent11WaitOnEventEi
// type: int __fastcall(RakNet::SignaledEvent *this, int)
#[doc(alias = "RakNet::SignaledEvent::WaitOnEvent(int)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent11WaitOnEventEi")]
pub fn stub_0xa7999c(event: &mut SignaledEvent, _timeout_ms: u32) -> i32 {
    // IDA 0xa7999c: `WaitOnEvent` single-shots below 31 ms (0xa799d6..
    // 0xa79a8e) and slices 30 ms at a time above (0xa799fe..0xa79a3c),
    // auto-resets the flag (0xa79a8e), and answers the unlock status
    // (0xa79a9e, success). MODEL: the blocking waits fold into the host;
    // the latch and answer are observed.
    if event.signaled {
        event.signaled = false;
    }
    0
}

#[cfg(test)]
mod history_heap_stream_batch_tests {
    use super::*;

    #[test]
    fn range_insert_coalesces() {
        let mut list = RangeList::default();
        stub_0xa7738c(&mut list, 5);
        stub_0xa7738c(&mut list, 6);
        assert_eq!(list.nodes, vec![RangeNode { min: 5, max: 6 }]);
        stub_0xa7738c(&mut list, 4);
        assert_eq!(list.nodes, vec![RangeNode { min: 4, max: 6 }]);
        stub_0xa7738c(&mut list, 3);
        stub_0xa7738c(&mut list, 7);
        assert_eq!(list.nodes, vec![RangeNode { min: 3, max: 7 }]);
        stub_0xa7738c(&mut list, 5);
        assert_eq!(list.nodes.len(), 1);
        stub_0xa7738c(&mut list, 10);
        assert_eq!(list.nodes.len(), 2);
        stub_0xa7738c(&mut list, 8);
        stub_0xa7738c(&mut list, 9);
        assert_eq!(list.nodes, vec![RangeNode { min: 3, max: 10 }]);
        let mut wrap = RangeList::default();
        stub_0xa7738c(&mut wrap, 0x00ff_ffff);
        stub_0xa7738c(&mut wrap, 0);
        assert_eq!(wrap.nodes, vec![RangeNode { min: 0x00ff_ffff, max: 0 }]);
        stub_0xa7738c(&mut wrap, 0x00ff_ffff);
        assert_eq!(wrap.nodes.len(), 1);
    }

    #[test]
    fn range_serde_round_trip() {
        let mut list = RangeList::default();
        stub_0xa7738c(&mut list, 1);
        for v in 5..10u32 {
            stub_0xa7738c(&mut list, v);
        }
        let mut wire = stub_0xa77b3c(&mut list, 10_000, false);
        assert_eq!(&wire[..2], &[2, 0]);
        let mut back = RangeList::default();
        assert!(stub_0xa771e8(&mut back, &wire));
        assert_eq!(back.nodes, vec![RangeNode { min: 1, max: 1 }, RangeNode { min: 5, max: 9 }]);
        assert!(!stub_0xa771e8(&mut back, &[]));
        assert!(!stub_0xa771e8(&mut back, &[1, 0]));
        assert!(!stub_0xa771e8(&mut back, &[1, 0, 0, 5, 0]));
        assert!(!stub_0xa771e8(&mut back, &[1, 0, 0, 9, 0, 0, 5, 0, 0]));
    }

    #[test]
    fn serialize_budget_and_remove() {
        let mut list = RangeList::default();
        for v in [10u32, 20, 30] {
            stub_0xa7738c(&mut list, v);
        }
        let wire = stub_0xa77b3c(&mut list, 100, false);
        assert_eq!(&wire[..2], &[1, 0]);
        assert_eq!(list.nodes.len(), 3);
        let wire = stub_0xa77b3c(&mut list, 10_000, true);
        assert_eq!(&wire[..2], &[3, 0]);
        assert!(list.nodes.is_empty());
    }

    #[test]
    fn heap_push_pop_min_order() {
        let mut heap = TimeHeap::default();
        assert_eq!(stub_0xa77950(&mut heap, 5, 50), 1);
        assert_eq!(stub_0xa77950(&mut heap, 3, 30), 2);
        assert_eq!(stub_0xa77950(&mut heap, 4, 40), 3);
        assert_eq!(stub_0xa77784(&mut heap, 0), Some(30));
        assert_eq!(stub_0xa77784(&mut heap, 0), Some(40));
        assert_eq!(stub_0xa77784(&mut heap, 0), Some(50));
        assert_eq!(stub_0xa77784(&mut heap, 0), None);
    }

    #[test]
    fn heap_pop_index_and_series() {
        let mut heap = TimeHeap::default();
        stub_0xa77950(&mut heap, 1, 10);
        stub_0xa77950(&mut heap, 2, 20);
        stub_0xa77950(&mut heap, 3, 30);
        assert_eq!(stub_0xa77784(&mut heap, 1), Some(20));
        assert_eq!(heap.entries.len(), 2);
        assert_eq!(stub_0xa77ff4(&mut heap, 0, 5), 1);
        assert!(heap.unordered);
        assert_eq!(stub_0xa77784(&mut heap, 0), Some(5));
        assert!(!heap.unordered);
        assert_eq!(stub_0xa77784(&mut heap, 9), None);
    }

    #[test]
    fn bitcursor_u24_round_trip() {
        let mut cursor = BitCursor::default();
        stub_0xa77d60(&mut cursor, 0x00ab_cdef);
        stub_0xa77d60(&mut cursor, 0x12);
        assert_eq!(cursor.bytes.len(), 6);
        let mut reader = BitCursor { bytes: cursor.bytes.clone(), bit_pos: 0 };
        assert_eq!(stub_0xa77ea4(&mut reader), Some(0x00ab_cdef));
        assert_eq!(stub_0xa77ea4(&mut reader), Some(0x12));
        assert_eq!(stub_0xa77ea4(&mut reader), None);
    }

    #[test]
    fn header_round_trips() {
        let ack = DatagramHeader {
            lead: true,
            is_ack: true,
            has_float: true,
            float_value: 1.5,
            ..DatagramHeader::default()
        };
        let bits = stub_0xa77a84(&ack);
        let (back, used) = stub_0xa77058(&bits).expect("ack header");
        assert_eq!(back, ack);
        assert_eq!(used, bits.len());
        let short = DatagramHeader { lead: true, short: true, ..DatagramHeader::default() };
        let bits = stub_0xa77a84(&short);
        assert_eq!(bits, vec![true, false, true, true]);
        let (back, _) = stub_0xa77058(&bits).expect("short header");
        assert_eq!(back, short);
        let long = DatagramHeader {
            lead: true,
            flag_a: true,
            flag_c: true,
            number: 0x00ab_cdef,
            ..DatagramHeader::default()
        };
        let bits = stub_0xa77a84(&long);
        let (back, used) = stub_0xa77058(&bits).expect("long header");
        assert_eq!(back, long);
        assert_eq!(used, bits.len());
        assert!(stub_0xa77058(&bits[..3]).is_none());
        assert!(stub_0xa77058(&[]).is_none());
    }

    #[test]
    fn ordered_insert_orders_and_dedups() {
        let mut list = OrderedSplitList::default();
        assert_eq!(stub_0xa781a4(&mut list, 9, 100), 0);
        assert_eq!(stub_0xa781a4(&mut list, 3, 101), 0);
        assert_eq!(list.ids, vec![3, 9]);
        assert_eq!(stub_0xa781a4(&mut list, 9, 102), 1);
        assert_eq!(list.handles, vec![101, 100]);
    }

    #[test]
    fn pools_queues_lists_cover_variants() {
        let mut pool = RakMemPool::default();
        let a = stub_0xa7828c(&mut pool);
        stub_0xa783b4(&mut pool, a);
        let b = stub_0xa78670(&mut pool);
        stub_0xa7848c(&mut pool, b);
        let c = stub_0xa7879c(&mut pool);
        stub_0xa788c8(&mut pool, c);
        assert_eq!(pool.live, 0);
        assert_eq!(pool.free.len(), 1);
        let mut q = RakPtrQueue::default();
        stub_0xa772b8(&mut q, 1);
        stub_0xa78560(&mut q, 2);
        stub_0xa78bbc(&mut q, 3);
        assert_eq!(q.capacity, 16);
        assert_eq!(q.len(), 3);
        let mut channels = RakList::<u32>::default();
        stub_0xa7899c(&mut channels, 0xabcd);
        assert_eq!(channels.items, vec![0xabcd]);
        let mut nodes = RakList::<u64>::default();
        stub_0xa78a2c(&mut nodes, 5, 9);
        stub_0xa78b08(&mut nodes, 1, 1);
        assert_eq!(nodes.items, vec![(9u64 << 32) | 5, (1u64 << 32) | 1]);
        assert_eq!(nodes.capacity, 16);
    }

    #[test]
    fn signaled_event_latch() {
        let mut event = stub_0xa79900();
        assert!(!event.signaled && !event.inited);
        assert_eq!(stub_0xa79924(&mut event), 0);
        assert!(event.inited);
        assert_eq!(stub_0xa7997c(&mut event), 0);
        assert!(event.signaled);
        assert_eq!(stub_0xa7999c(&mut event, 50), 0);
        assert!(!event.signaled);
        assert_eq!(stub_0xa7999c(&mut event, 5000), 0);
        assert_eq!(stub_0xa79954(&mut event), 0);
        assert!(!event.inited);
        stub_0xa79914(&mut event);
        assert!(!event.inited);
    }
}

// 0xa7a0b4 — __ZN6RakNet11SimpleMutexC1Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::SimpleMutex(void)")]
#[doc(alias = "__ZN6RakNet11SimpleMutexC1Ev")]
pub fn stub_0xa7a0b4() -> ! {
    todo!("0xa7a0b4")
}

// 0xa7a0c4 — __ZN6RakNet11SimpleMutexD1Ev
// type: void __fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::~SimpleMutex()")]
#[doc(alias = "__ZN6RakNet11SimpleMutexD1Ev")]
pub fn stub_0xa7a0c4() -> ! {
    todo!("0xa7a0c4")
}

// 0xa7a0d4 — __ZN6RakNet11SimpleMutex4LockEv
// type: int __fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::Lock(void)")]
#[doc(alias = "__ZN6RakNet11SimpleMutex4LockEv")]
pub fn stub_0xa7a0d4() -> ! {
    todo!("0xa7a0d4")
}

// 0xa7a0e0 — __ZN6RakNet11SimpleMutex6UnlockEv
// type: int __fastcall(pthread_mutex_t *this)
#[doc(alias = "RakNet::SimpleMutex::Unlock(void)")]
#[doc(alias = "__ZN6RakNet11SimpleMutex6UnlockEv")]
pub fn stub_0xa7a0e0() -> ! {
    todo!("0xa7a0e0")
}

// 0xa7a700 — __ZN6RakNet11SocketLayer11IsPortInUseEtPKct
// type: int __fastcall(RakNet::SocketLayer *this, const char *, const char *, unsigned __int16)
#[doc(alias = "RakNet::SocketLayer::IsPortInUse(unsigned short,char const*,unsigned short)")]
#[doc(alias = "__ZN6RakNet11SocketLayer11IsPortInUseEtPKct")]
pub fn stub_0xa7a700() -> ! {
    todo!("0xa7a700")
}

// 0xa7a788 — __ZN6RakNet11SocketLayer16SetDoNotFragmentEiii
// type: void __fastcall(RakNet::SocketLayer *this, int, int, int)
#[doc(alias = "RakNet::SocketLayer::SetDoNotFragment(int,int,int)")]
#[doc(alias = "__ZN6RakNet11SocketLayer16SetDoNotFragmentEiii")]
pub fn stub_0xa7a788() -> ! {
    todo!("0xa7a788")
}

// 0xa7a78c — __ZN6RakNet11SocketLayer21CreateBoundSocket_OldEtbPKcjj
// type: int __fastcall(RakNet::SocketLayer *this, unsigned __int16, const char *, const char *, unsigned int, unsigned int)
#[doc(alias = "RakNet::SocketLayer::CreateBoundSocket_Old(unsigned short,bool,char const*,unsigned int,unsigned int)")]
#[doc(alias = "__ZN6RakNet11SocketLayer21CreateBoundSocket_OldEtbPKcjj")]
pub fn stub_0xa7a78c() -> ! {
    todo!("0xa7a78c")
}

// 0xa7a898 — __ZN6RakNet11SocketLayer17CreateBoundSocketEtbPKcjjt
// type: int __fastcall(RakNet::SocketLayer *this, unsigned __int16, const char *, const char *, unsigned int, unsigned int, unsigned __int16)
#[doc(alias = "RakNet::SocketLayer::CreateBoundSocket(unsigned short,bool,char const*,unsigned int,unsigned int,unsigned short)")]
#[doc(alias = "__ZN6RakNet11SocketLayer17CreateBoundSocketEtbPKcjjt")]
pub fn stub_0xa7a898() -> ! {
    todo!("0xa7a898")
}

// 0xa7a8ac — __ZN6RakNet11SocketLayer14DomainNameToIPEPKc
// type: char *__fastcall(RakNet::SocketLayer *this, const char *)
#[doc(alias = "RakNet::SocketLayer::DomainNameToIP(char const*)")]
#[doc(alias = "__ZN6RakNet11SocketLayer14DomainNameToIPEPKc")]
pub fn stub_0xa7a8ac() -> ! {
    todo!("0xa7a8ac")
}

// 0xa7a8d0 — __ZN6RakNet11SocketLayer16RecvFromBlockingEiPNS_7RakPeerEtjPcPiPNS_13SystemAddressEPy
// type: int __fastcall(RakNet::SocketLayer *this, int, RakNet::RakPeer *, unsigned __int16, void *, char *, int *, RakNet::SystemAddress *, unsigned __int64 *)
#[doc(alias = "RakNet::SocketLayer::RecvFromBlocking(int,RakNet::RakPeer *,unsigned short,unsigned int,char *,int *,RakNet::SystemAddress *,unsigned long long *)")]
#[doc(alias = "__ZN6RakNet11SocketLayer16RecvFromBlockingEiPNS_7RakPeerEtjPcPiPNS_13SystemAddressEPy")]
pub fn stub_0xa7a8d0() -> ! {
    todo!("0xa7a8d0")
}

// 0xa7a944 — __ZN6RakNet11SocketLayer6SendToEiPKciRNS_13SystemAddressEtjS2_l
// type: int __fastcall(RakNet::SocketLayer *this, char *, size_t, sockaddr *, RakNet::SystemAddress *, unsigned __int16, unsigned int, const char *, int)
#[doc(alias = "RakNet::SocketLayer::SendTo(int,char const*,int,RakNet::SystemAddress &,unsigned short,unsigned int,char const*,long)")]
#[doc(alias = "__ZN6RakNet11SocketLayer6SendToEiPKciRNS_13SystemAddressEtjS2_l")]
pub fn stub_0xa7a944() -> ! {
    todo!("0xa7a944")
}

// 0xa7a9ec — __ZN6RakNet11SocketLayer9SendToTTLEiPKciRNS_13SystemAddressEi
// type: int __fastcall(RakNet::SocketLayer *this, char *, const char *, RakNet::SystemAddress *, RakNet::SystemAddress *, int)
#[doc(alias = "RakNet::SocketLayer::SendToTTL(int,char const*,int,RakNet::SystemAddress &,int)")]
#[doc(alias = "__ZN6RakNet11SocketLayer9SendToTTLEiPKciRNS_13SystemAddressEi")]
pub fn stub_0xa7a9ec() -> ! {
    todo!("0xa7a9ec")
}

// 0xa7aae0 — __Z13GetMyIP_LinuxPN6RakNet13SystemAddressE
// type: int __fastcall(in_addr *)
#[doc(alias = "GetMyIP_Linux(RakNet::SystemAddress *)")]
#[doc(alias = "__Z13GetMyIP_LinuxPN6RakNet13SystemAddressE")]
pub fn stub_0xa7aae0() -> ! {
    todo!("0xa7aae0")
}

// 0xa7abd8 — __ZN6RakNet11SocketLayer7GetMyIPEPNS_13SystemAddressE
// type: int __fastcall(in_addr *this, RakNet::SystemAddress *)
#[doc(alias = "RakNet::SocketLayer::GetMyIP(RakNet::SystemAddress *)")]
#[doc(alias = "__ZN6RakNet11SocketLayer7GetMyIPEPNS_13SystemAddressE")]
pub fn stub_0xa7abd8() -> ! {
    todo!("0xa7abd8")
}

// 0xa7abe4 — __ZN6RakNet11SocketLayer16GetSystemAddressEiPNS_13SystemAddressE
// type: int __fastcall(RakNet::SocketLayer *this, int, RakNet::SystemAddress *)
#[doc(alias = "RakNet::SocketLayer::GetSystemAddress(int,RakNet::SystemAddress *)")]
#[doc(alias = "__ZN6RakNet11SocketLayer16GetSystemAddressEiPNS_13SystemAddressE")]
pub fn stub_0xa7abe4() -> ! {
    todo!("0xa7abe4")
}

// 0xa7b268 — __ZN6RakNet16StringCompressor12AddReferenceEv
// type: void __fastcall(RakNet::StringCompressor *this)
#[doc(alias = "RakNet::StringCompressor::AddReference(void)")]
#[doc(alias = "__ZN6RakNet16StringCompressor12AddReferenceEv")]
pub fn stub_0xa7b268() -> ! {
    todo!("0xa7b268")
}

// 0xa7b39c — __ZN6RakNet16StringCompressor15RemoveReferenceEv
// type: void __fastcall(RakNet::StringCompressor *this)
#[doc(alias = "RakNet::StringCompressor::RemoveReference(void)")]
#[doc(alias = "__ZN6RakNet16StringCompressor15RemoveReferenceEv")]
pub fn stub_0xa7b39c() -> ! {
    todo!("0xa7b39c")
}

// 0xa7b480 — __ZN6RakNet16StringCompressorD2Ev
// type: void __fastcall(RakNet::StringCompressor *__hidden this)
#[doc(alias = "RakNet::StringCompressor::~StringCompressor()")]
#[doc(alias = "__ZN6RakNet16StringCompressorD2Ev")]
pub fn stub_0xa7b480() -> ! {
    todo!("0xa7b480")
}

// 0xa7b594 — __ZN6RakNet16StringCompressor12EncodeStringEPKciPNS_9BitStreamEh
// type: int __fastcall(RakNet::StringCompressor *this, char *, int, struct _Unwind_Exception *, int)
#[doc(alias = "RakNet::StringCompressor::EncodeString(char const*,int,RakNet::BitStream *,unsigned char)")]
#[doc(alias = "__ZN6RakNet16StringCompressor12EncodeStringEPKciPNS_9BitStreamEh")]
pub fn stub_0xa7b594() -> ! {
    todo!("0xa7b594")
}

// 0xa7b764 — __ZN6RakNet16StringCompressor12DecodeStringEPciPNS_9BitStreamEh
// type: int __fastcall(RakNet::StringCompressor *this, char *, int, RakNet::BitStream *, int)
#[doc(alias = "RakNet::StringCompressor::DecodeString(char *,int,RakNet::BitStream *,unsigned char)")]
#[doc(alias = "__ZN6RakNet16StringCompressor12DecodeStringEPciPNS_9BitStreamEh")]
pub fn stub_0xa7b764() -> ! {
    todo!("0xa7b764")
}

// 0xa7b854 — __ZN14DataStructures3MapIiPN6RakNet19HuffmanEncodingTreeEXadL_ZNS_23defaultMapKeyComparisonIiEEiRKT_S7_EEE3SetERKiRKS3_
// type: int __fastcall(_DWORD *, int *, int *)
#[doc(alias = "DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::Set(int const&,RakNet::HuffmanEncodingTree * const&)")]
#[doc(alias = "__ZN14DataStructures3MapIiPN6RakNet19HuffmanEncodingTreeEXadL_ZNS_23defaultMapKeyComparisonIiEEiRKT_S7_EEE3SetERKiRKS3_")]
pub fn stub_0xa7b854() -> ! {
    todo!("0xa7b854")
}

// 0xa7b9b4 — __ZN6RakNet9BitStream15WriteCompressedIjEEvRKT_
// type: void __fastcall(RakNet::BitStream *, unsigned __int8 *, int, unsigned int, __guard *, int, int, int, int)
#[doc(alias = "void RakNet::BitStream::WriteCompressed<unsigned int>(unsigned int const&)")]
#[doc(alias = "__ZN6RakNet9BitStream15WriteCompressedIjEEvRKT_")]
pub fn stub_0xa7b9b4() -> ! {
    todo!("0xa7b9b4")
}

// 0xa7bac8 — __ZN6RakNet9BitStream14ReadCompressedIjEEbRT_
// type: int __fastcall(RakNet::BitStream *, unsigned __int8 *, int, int, __guard *, int, int, int, int)
#[doc(alias = "bool RakNet::BitStream::ReadCompressed<unsigned int>(unsigned int &)")]
#[doc(alias = "__ZN6RakNet9BitStream14ReadCompressedIjEEbRT_")]
pub fn stub_0xa7bac8() -> ! {
    todo!("0xa7bac8")
}

// 0xa7bbf0 — __ZN14DataStructures4ListINS_3MapIiPN6RakNet19HuffmanEncodingTreeEXadL_ZNS_23defaultMapKeyComparisonIiEEiRKT_S8_EEE7MapNodeEE6InsertERKSA_jPKcj
// type: int __fastcall(char **, _DWORD *, char *)
#[doc(alias = "DataStructures::List<DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode>::Insert(DataStructures::Map<int,RakNet::HuffmanEncodingTree *,&int DataStructures::defaultMapKeyComparison<int>>::MapNode const&,unsigned int,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListINS_3MapIiPN6RakNet19HuffmanEncodingTreeEXadL_ZNS_23defaultMapKeyComparisonIiEEiRKT_S8_EEE7MapNodeEE6InsertERKSA_jPKcj")]
pub fn stub_0xa7bbf0() -> ! {
    todo!("0xa7bbf0")
}

// 0xa7d1d8 — __ZN14DataStructures5QueueIPN6RakNet6PacketEE4PushERKS3_PKcj
// type: void __fastcall(int **, int *)
#[doc(alias = "DataStructures::Queue<RakNet::Packet *>::Push(RakNet::Packet * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIPN6RakNet6PacketEE4PushERKS3_PKcj")]
pub fn stub_0xa7d1d8() -> ! {
    todo!("0xa7d1d8")
}
