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

// 0xa74c88 — __ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEtyiRNS_13SystemAddressEPNS_12RakNetRandomEtjRNS_9BitStreamE
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned int, unsigned __int64, int, RakNet::SystemAddress *, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, RakNet::BitStream *)
#[doc(alias = "RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(unsigned short,unsigned long long,int,RakNet::SystemAddress &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEtyiRNS_13SystemAddressEPNS_12RakNetRandomEtjRNS_9BitStreamE")]
pub fn stub_0xa74c88() -> ! {
    todo!("0xa74c88")
}

// 0xa74d64 — __ZN6RakNet16ReliabilityLayer7ReceiveEPPh
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned __int8 **)
#[doc(alias = "RakNet::ReliabilityLayer::Receive(unsigned char **)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer7ReceiveEPPh")]
pub fn stub_0xa74d64() -> ! {
    todo!("0xa74d64")
}

// 0xa74dc0 — __ZN6RakNet16ReliabilityLayer4SendEPcj14PacketPriority17PacketReliabilityhbiyj
// type: int __fastcall(int, const void *, int, unsigned int, unsigned int, unsigned int, int, int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::Send(char *,unsigned int,PacketPriority,PacketReliability,unsigned char,bool,int,unsigned long long,unsigned int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer4SendEPcj14PacketPriority17PacketReliabilityhbiyj")]
pub fn stub_0xa74dc0() -> ! {
    todo!("0xa74dc0")
}

// 0xa75100 — __ZN6RakNet16ReliabilityLayer11SplitPacketEPNS_14InternalPacketE
// type: int __fastcall(int, int)
#[doc(alias = "RakNet::ReliabilityLayer::SplitPacket(RakNet::InternalPacket *)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer11SplitPacketEPNS_14InternalPacketE")]
pub fn stub_0xa75100() -> ! {
    todo!("0xa75100")
}

// 0xa75548 — __ZN6RakNet16ReliabilityLayer6UpdateEiRNS_13SystemAddressEiyjRN14DataStructures4ListIPNS_16PluginInterface2EEEPNS_12RakNetRandomEtjRNS_9BitStreamE
// type: bool __fastcall(int, RakNet::SocketLayer *, sockaddr *, int, unsigned __int64, int, _DWORD *, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, void **)
#[doc(alias = "RakNet::ReliabilityLayer::Update(int,RakNet::SystemAddress &,int,unsigned long long,unsigned int,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer6UpdateEiRNS_13SystemAddressEiyjRN14DataStructures4ListIPNS_16PluginInterface2EEEPNS_12RakNetRandomEtjRNS_9BitStreamE")]
pub fn stub_0xa75548() -> ! {
    todo!("0xa75548")
}

// 0xa7641c — __ZN6RakNet16ReliabilityLayer10AckTimeoutEy
// type: int __fastcall(RakNet::ReliabilityLayer *this, unsigned __int64)
#[doc(alias = "RakNet::ReliabilityLayer::AckTimeout(unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer10AckTimeoutEy")]
pub fn stub_0xa7641c() -> ! {
    todo!("0xa7641c")
}

// 0xa76468 — __ZN6RakNet16ReliabilityLayer8SendACKsEiRNS_13SystemAddressEyPNS_12RakNetRandomEtjRNS_9BitStreamE
// type: int __fastcall(RakNet::ReliabilityLayer *this, RakNet::SocketLayer *, sockaddr *, unsigned __int64, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, void **)
#[doc(alias = "RakNet::ReliabilityLayer::SendACKs(int,RakNet::SystemAddress &,unsigned long long,RakNet::RakNetRandom *,unsigned short,unsigned int,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer8SendACKsEiRNS_13SystemAddressEyPNS_12RakNetRandomEtjRNS_9BitStreamE")]
pub fn stub_0xa76468() -> ! {
    todo!("0xa76468")
}

// 0xa765e0 — __ZN6RakNet16ReliabilityLayer24ResetPacketsAndDatagramsEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ResetPacketsAndDatagrams(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer24ResetPacketsAndDatagramsEv")]
pub fn stub_0xa765e0() -> ! {
    todo!("0xa765e0")
}

// 0xa766b8 — __ZN6RakNet16ReliabilityLayer12PushDatagramEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::PushDatagram(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer12PushDatagramEv")]
pub fn stub_0xa766b8() -> ! {
    todo!("0xa766b8")
}

// 0xa76828 — __ZN6RakNet16ReliabilityLayer10PushPacketEyPNS_14InternalPacketEb
// type: void __fastcall(_DWORD *, unsigned __int64, int, char)
#[doc(alias = "RakNet::ReliabilityLayer::PushPacket(unsigned long long,RakNet::InternalPacket *,bool)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer10PushPacketEyPNS_14InternalPacketEb")]
pub fn stub_0xa76828() -> ! {
    todo!("0xa76828")
}

// 0xa7696c — __ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tES1_y
// type: _DWORD *__fastcall(int, int, _DWORD *, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,RakNet::uint24_t,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tES1_y")]
pub fn stub_0xa7696c() -> ! {
    todo!("0xa7696c")
}

// 0xa76a68 — __ZN6RakNet16ReliabilityLayer34WriteToBitStreamFromInternalPacketEPNS_9BitStreamEPKNS_14InternalPacketEy
// type: int __fastcall(int, RakNet::BitStream *this, int)
#[doc(alias = "RakNet::ReliabilityLayer::WriteToBitStreamFromInternalPacket(RakNet::BitStream *,RakNet::InternalPacket const*,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer34WriteToBitStreamFromInternalPacketEPNS_9BitStreamEPKNS_14InternalPacketEy")]
pub fn stub_0xa76a68() -> ! {
    todo!("0xa76a68")
}

// 0xa76b88 — __ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tEy
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::AddFirstToDatagramHistory(RakNet::uint24_t,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer25AddFirstToDatagramHistoryENS_8uint24_tEy")]
pub fn stub_0xa76b88() -> ! {
    todo!("0xa76b88")
}

// 0xa76c68 — __ZN6RakNet16ReliabilityLayer21IsOutgoingDataWaitingEv
// type: bool __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::IsOutgoingDataWaiting(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer21IsOutgoingDataWaitingEv")]
pub fn stub_0xa76c68() -> ! {
    todo!("0xa76c68")
}

// 0xa76c84 — __ZN6RakNet16ReliabilityLayer14AreAcksWaitingEv
// type: bool __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::AreAcksWaiting(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer14AreAcksWaitingEv")]
pub fn stub_0xa76c84() -> ! {
    todo!("0xa76c84")
}

// 0xa76c90 — __ZN6RakNet16ReliabilityLayer31SetSplitMessageProgressIntervalEi
// type: int __fastcall(int this, int)
#[doc(alias = "RakNet::ReliabilityLayer::SetSplitMessageProgressInterval(int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer31SetSplitMessageProgressIntervalEi")]
pub fn stub_0xa76c90() -> ! {
    todo!("0xa76c90")
}

// 0xa76c94 — __ZN6RakNet16ReliabilityLayer20SetUnreliableTimeoutEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::ReliabilityLayer::SetUnreliableTimeout(unsigned int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer20SetUnreliableTimeoutEj")]
pub fn stub_0xa76c94() -> ! {
    todo!("0xa76c94")
}

// 0xa76ca4 — __ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEPNS_18SplitPacketChannelEy
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::BuildPacketFromSplitPacketList(RakNet::SplitPacketChannel *,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer30BuildPacketFromSplitPacketListEPNS_18SplitPacketChannelEy")]
pub fn stub_0xa76ca4() -> ! {
    todo!("0xa76ca4")
}

// 0xa76e6c — __ZNK6RakNet16ReliabilityLayer16IsDeadConnectionEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::IsDeadConnection(void)const")]
#[doc(alias = "__ZNK6RakNet16ReliabilityLayer16IsDeadConnectionEv")]
pub fn stub_0xa76e6c() -> ! {
    todo!("0xa76e6c")
}

// 0xa76e74 — __ZN6RakNet16ReliabilityLayer13GetStatisticsEPNS_16RakNetStatisticsE
// type: int __fastcall(int, int)
#[doc(alias = "RakNet::ReliabilityLayer::GetStatistics(RakNet::RakNetStatistics *)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer13GetStatisticsEPNS_16RakNetStatisticsE")]
pub fn stub_0xa76e74() -> ! {
    todo!("0xa76e74")
}

// 0xa77058 — __ZN20DatagramHeaderFormat11DeserializeEPN6RakNet9BitStreamE
// type: _DWORD __fastcall(DatagramHeaderFormat *__hidden this, RakNet::BitStream *)
#[doc(alias = "DatagramHeaderFormat::Deserialize(RakNet::BitStream *)")]
#[doc(alias = "__ZN20DatagramHeaderFormat11DeserializeEPN6RakNet9BitStreamE")]
pub fn stub_0xa77058() -> ! {
    todo!("0xa77058")
}

// 0xa771e8 — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE11DeserializeEPNS1_9BitStreamE
// type: int __fastcall(_DWORD *, RakNet::BitStream *, int, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Deserialize(RakNet::BitStream *)")]
#[doc(alias = "__ZN14DataStructures9RangeListIN6RakNet8uint24_tEE11DeserializeEPNS1_9BitStreamE")]
pub fn stub_0xa771e8() -> ! {
    todo!("0xa771e8")
}

// 0xa772b8 — __ZN14DataStructures5QueueIPN6RakNet14InternalPacketEE4PushERKS3_PKcj
// type: void __fastcall(int **, int *)
#[doc(alias = "DataStructures::Queue<RakNet::InternalPacket *>::Push(RakNet::InternalPacket * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIPN6RakNet14InternalPacketEE4PushERKS3_PKcj")]
pub fn stub_0xa772b8() -> ! {
    todo!("0xa772b8")
}

// 0xa7738c — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE6InsertES2_
// type: void __fastcall(int *, unsigned int *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Insert(RakNet::uint24_t)")]
#[doc(alias = "__ZN14DataStructures9RangeListIN6RakNet8uint24_tEE6InsertES2_")]
pub fn stub_0xa7738c() -> ! {
    todo!("0xa7738c")
}

// 0xa77784 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE3PopEj
// type: int __fastcall(int *, unsigned int)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Pop(unsigned int)")]
#[doc(alias = "__ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE3PopEj")]
pub fn stub_0xa77784() -> ! {
    todo!("0xa77784")
}

// 0xa77950 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE4PushERKyRKS3_PKcj
// type: unsigned int __fastcall(char **, int *, int *)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::Push(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE4PushERKyRKS3_PKcj")]
pub fn stub_0xa77950() -> ! {
    todo!("0xa77950")
}

// 0xa77a84 — __ZN20DatagramHeaderFormat9SerializeEPN6RakNet9BitStreamE
// type: int __fastcall(DatagramHeaderFormat *this, RakNet::BitStream *)
#[doc(alias = "DatagramHeaderFormat::Serialize(RakNet::BitStream *)")]
#[doc(alias = "__ZN20DatagramHeaderFormat9SerializeEPN6RakNet9BitStreamE")]
pub fn stub_0xa77a84() -> ! {
    todo!("0xa77a84")
}

// 0xa77b3c — __ZN14DataStructures9RangeListIN6RakNet8uint24_tEE9SerializeEPNS1_9BitStreamEjb
// type: int __fastcall(int *, RakNet::BitStream *, unsigned int, int)
#[doc(alias = "DataStructures::RangeList<RakNet::uint24_t>::Serialize(RakNet::BitStream *,unsigned int,bool)")]
#[doc(alias = "__ZN14DataStructures9RangeListIN6RakNet8uint24_tEE9SerializeEPNS1_9BitStreamEjb")]
pub fn stub_0xa77b3c() -> ! {
    todo!("0xa77b3c")
}

// 0xa77d60 — __ZN6RakNet9BitStream5WriteINS_8uint24_tEEEvRKT_
// type: void __fastcall(RakNet::BitStream *this, _BYTE *, int, int, int)
#[doc(alias = "void RakNet::BitStream::Write<RakNet::uint24_t>(RakNet::uint24_t const&)")]
#[doc(alias = "__ZN6RakNet9BitStream5WriteINS_8uint24_tEEEvRKT_")]
pub fn stub_0xa77d60() -> ! {
    todo!("0xa77d60")
}

// 0xa77ea4 — __ZN6RakNet9BitStream4ReadINS_8uint24_tEEEbRT_
// type: int __fastcall(_DWORD *, _BYTE *)
#[doc(alias = "bool RakNet::BitStream::Read<RakNet::uint24_t>(RakNet::uint24_t &)")]
#[doc(alias = "__ZN6RakNet9BitStream4ReadINS_8uint24_tEEEbRT_")]
pub fn stub_0xa77ea4() -> ! {
    todo!("0xa77ea4")
}

// 0xa77ff4 — __ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE10PushSeriesERKyRKS3_PKcj
// type: unsigned int __fastcall(int, int *, int *)
#[doc(alias = "DataStructures::Heap<unsigned long long,RakNet::InternalPacket *,false>::PushSeries(unsigned long long const&,RakNet::InternalPacket * const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4HeapIyPN6RakNet14InternalPacketELb0EE10PushSeriesERKyRKS3_PKcj")]
pub fn stub_0xa77ff4() -> ! {
    todo!("0xa77ff4")
}

// 0xa781a4 — __ZN14DataStructures11OrderedListItPN6RakNet18SplitPacketChannelEXadL_ZNS1_22SplitPacketChannelCompERKtRKS3_EEE6InsertES5_S7_bPKcjPFiS5_S7_E
// type: unsigned int __fastcall(int **, int, int *, int, int, int, int (__fastcall *)(int, int))
#[doc(alias = "DataStructures::OrderedList<unsigned short,RakNet::SplitPacketChannel *,&RakNet::SplitPacketChannelComp>::Insert(unsigned short const&,RakNet::SplitPacketChannel * const&,bool,char const*,unsigned int,int (*)(unsigned short const&,RakNet::SplitPacketChannel * const&))")]
#[doc(alias = "__ZN14DataStructures11OrderedListItPN6RakNet18SplitPacketChannelEXadL_ZNS1_22SplitPacketChannelCompERKtRKS3_EEE6InsertES5_S7_bPKcjPFiS5_S7_E")]
pub fn stub_0xa781a4() -> ! {
    todo!("0xa781a4")
}

// 0xa7828c — __ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacket>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE8AllocateEPKcj")]
pub fn stub_0xa7828c() -> ! {
    todo!("0xa7828c")
}

// 0xa783b4 — __ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE7ReleaseEPS2_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacket>::Release(RakNet::InternalPacket*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet14InternalPacketEE7ReleaseEPS2_PKcj")]
pub fn stub_0xa783b4() -> ! {
    todo!("0xa783b4")
}

// 0xa7848c — __ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE7ReleaseEPS3_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Release(RakNet::ReliabilityLayer::MessageNumberNode*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE7ReleaseEPS3_PKcj")]
pub fn stub_0xa7848c() -> ! {
    todo!("0xa7848c")
}

// 0xa78560 — __ZN14DataStructures5QueueIN6RakNet16ReliabilityLayer19DatagramHistoryNodeEE4PushERKS3_PKcj
// type: void __fastcall(_DWORD *, __int64 *)
#[doc(alias = "DataStructures::Queue<RakNet::ReliabilityLayer::DatagramHistoryNode>::Push(RakNet::ReliabilityLayer::DatagramHistoryNode const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIN6RakNet16ReliabilityLayer19DatagramHistoryNodeEE4PushERKS3_PKcj")]
pub fn stub_0xa78560() -> ! {
    todo!("0xa78560")
}

// 0xa78670 — __ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::ReliabilityLayer::MessageNumberNode>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet16ReliabilityLayer17MessageNumberNodeEE8AllocateEPKcj")]
pub fn stub_0xa78670() -> ! {
    todo!("0xa78670")
}

// 0xa7879c — __ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE8AllocateEPKcj
// type: int __fastcall(_DWORD *, unsigned int, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Allocate(char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE8AllocateEPKcj")]
pub fn stub_0xa7879c() -> ! {
    todo!("0xa7879c")
}

// 0xa788c8 — __ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE7ReleaseEPS2_PKcj
// type: _DWORD *__fastcall(_DWORD *result, int, void *, char *)
#[doc(alias = "DataStructures::MemoryPool<RakNet::InternalPacketRefCountedData>::Release(RakNet::InternalPacketRefCountedData*,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures10MemoryPoolIN6RakNet28InternalPacketRefCountedDataEE7ReleaseEPS2_PKcj")]
pub fn stub_0xa788c8() -> ! {
    todo!("0xa788c8")
}

// 0xa7899c — __ZN14DataStructures4ListIPN6RakNet18SplitPacketChannelEE6InsertERKS3_jPKcj
// type: unsigned int __fastcall(int, _DWORD *, int)
#[doc(alias = "DataStructures::List<RakNet::SplitPacketChannel *>::Insert(RakNet::SplitPacketChannel * const&,unsigned int,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListIPN6RakNet18SplitPacketChannelEE6InsertERKS3_jPKcj")]
pub fn stub_0xa7899c() -> ! {
    todo!("0xa7899c")
}

// 0xa78a2c — __ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_jPKcj
// type: int __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,unsigned int,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_jPKcj")]
pub fn stub_0xa78a2c() -> ! {
    todo!("0xa78a2c")
}

// 0xa78b08 — __ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_PKcj
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "DataStructures::List<DataStructures::RangeNode<RakNet::uint24_t>>::Insert(DataStructures::RangeNode<RakNet::uint24_t> const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures4ListINS_9RangeNodeIN6RakNet8uint24_tEEEE6InsertERKS4_PKcj")]
pub fn stub_0xa78b08() -> ! {
    todo!("0xa78b08")
}

// 0xa78bbc — __ZN14DataStructures5QueueIN6RakNet10BPSTracker13TimeAndValue2EE4PushERKS3_PKcj
// type: _QWORD *__fastcall(int *, _QWORD *)
#[doc(alias = "DataStructures::Queue<RakNet::BPSTracker::TimeAndValue2>::Push(RakNet::BPSTracker::TimeAndValue2 const&,char const*,unsigned int)")]
#[doc(alias = "__ZN14DataStructures5QueueIN6RakNet10BPSTracker13TimeAndValue2EE4PushERKS3_PKcj")]
pub fn stub_0xa78bbc() -> ! {
    todo!("0xa78bbc")
}

// 0xa79900 — __ZN6RakNet13SignaledEventC1Ev
// type: RakNet::SignaledEvent *__fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::SignaledEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEventC1Ev")]
pub fn stub_0xa79900() -> ! {
    todo!("0xa79900")
}

// 0xa79914 — __ZN6RakNet13SignaledEventD1Ev
// type: void __fastcall(RakNet::SignaledEvent *__hidden this)
#[doc(alias = "RakNet::SignaledEvent::~SignaledEvent()")]
#[doc(alias = "__ZN6RakNet13SignaledEventD1Ev")]
pub fn stub_0xa79914() -> ! {
    todo!("0xa79914")
}

// 0xa79924 — __ZN6RakNet13SignaledEvent9InitEventEv
// type: int __fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::InitEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent9InitEventEv")]
pub fn stub_0xa79924() -> ! {
    todo!("0xa79924")
}

// 0xa79954 — __ZN6RakNet13SignaledEvent10CloseEventEv
// type: int __fastcall(RakNet::SignaledEvent *this)
#[doc(alias = "RakNet::SignaledEvent::CloseEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent10CloseEventEv")]
pub fn stub_0xa79954() -> ! {
    todo!("0xa79954")
}

// 0xa7997c — __ZN6RakNet13SignaledEvent8SetEventEv
// type: int __fastcall(pthread_cond_t *this)
#[doc(alias = "RakNet::SignaledEvent::SetEvent(void)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent8SetEventEv")]
pub fn stub_0xa7997c() -> ! {
    todo!("0xa7997c")
}

// 0xa7999c — __ZN6RakNet13SignaledEvent11WaitOnEventEi
// type: int __fastcall(RakNet::SignaledEvent *this, int)
#[doc(alias = "RakNet::SignaledEvent::WaitOnEvent(int)")]
#[doc(alias = "__ZN6RakNet13SignaledEvent11WaitOnEventEi")]
pub fn stub_0xa7999c() -> ! {
    todo!("0xa7999c")
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
