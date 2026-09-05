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
pub fn stub_0xa6fa3c() -> ! {
    todo!("0xa6fa3c")
}

// 0xa70260 — __ZN6RakNet12RakNetRandomC1Ev
// type: int __fastcall(int this)
#[doc(alias = "RakNet::RakNetRandom::RakNetRandom(void)")]
#[doc(alias = "__ZN6RakNet12RakNetRandomC1Ev")]
pub fn stub_0xa70260() -> ! {
    todo!("0xa70260")
}

// 0xa70270 — __ZN6RakNet12RakNetRandomD1Ev
// type: void __fastcall(RakNet::RakNetRandom *__hidden this)
#[doc(alias = "RakNet::RakNetRandom::~RakNetRandom()")]
#[doc(alias = "__ZN6RakNet12RakNetRandomD1Ev")]
pub fn stub_0xa70270() -> ! {
    todo!("0xa70270")
}

// 0xa70278 — __ZN6RakNet12RakNetRandom6SeedMTEj
// type: unsigned int *__fastcall(unsigned int *this, unsigned int)
#[doc(alias = "RakNet::RakNetRandom::SeedMT(unsigned int)")]
#[doc(alias = "__ZN6RakNet12RakNetRandom6SeedMTEj")]
pub fn stub_0xa70278() -> ! {
    todo!("0xa70278")
}

// 0xa702a4 — __ZN6RakNet12RakNetRandom8RandomMTEv
// type: unsigned int __fastcall(RakNet::RakNetRandom *this)
#[doc(alias = "RakNet::RakNetRandom::RandomMT(void)")]
#[doc(alias = "__ZN6RakNet12RakNetRandom8RandomMTEv")]
pub fn stub_0xa702a4() -> ! {
    todo!("0xa702a4")
}

// 0xa7090c — __ZN6RakNet22SplitPacketChannelCompERKtRKPNS_18SplitPacketChannelE
// type: int __fastcall(unsigned __int16 *, int)
#[doc(alias = "RakNet::SplitPacketChannelComp(unsigned short const&,RakNet::SplitPacketChannel * const&)")]
#[doc(alias = "__ZN6RakNet22SplitPacketChannelCompERKtRKPNS_18SplitPacketChannelE")]
pub fn stub_0xa7090c() -> ! {
    todo!("0xa7090c")
}

// 0xa7092c — __ZN6RakNet16ReliabilityLayerC1Ev
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ReliabilityLayer(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerC1Ev")]
pub fn stub_0xa7092c() -> ! {
    todo!("0xa7092c")
}

// 0xa70938 — __ZN6RakNet16ReliabilityLayerC2Ev
// type: RakNet::ReliabilityLayer *__fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ReliabilityLayer(void) [0xa70938]")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerC2Ev")]
pub fn stub_0xa70938() -> ! {
    todo!("0xa70938")
}

// 0xa7142c — __ZN6RakNet16ReliabilityLayer19InitializeVariablesEv
// type: void __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::InitializeVariables(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer19InitializeVariablesEv")]
pub fn stub_0xa7142c() -> ! {
    todo!("0xa7142c")
}

// 0xa715f8 — __ZN6RakNet16ReliabilityLayerD1Ev
// type: void __fastcall(RakNet::ReliabilityLayer *__hidden this)
#[doc(alias = "RakNet::ReliabilityLayer::~ReliabilityLayer()")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerD1Ev")]
pub fn stub_0xa715f8() -> ! {
    todo!("0xa715f8")
}

// 0xa71604 — __ZN6RakNet16ReliabilityLayerD2Ev
// type: void __fastcall(RakNet::ReliabilityLayer *__hidden this)
#[doc(alias = "RakNet::ReliabilityLayer::~ReliabilityLayer() [0xa71604]")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayerD2Ev")]
pub fn stub_0xa71604() -> ! {
    todo!("0xa71604")
}

// 0xa723c0 — __ZN6RakNet16ReliabilityLayer5ResetEbib
// type: _QWORD *__fastcall(RakNet::ReliabilityLayer *this, int, int, bool)
#[doc(alias = "RakNet::ReliabilityLayer::Reset(bool,int,bool)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer5ResetEbib")]
pub fn stub_0xa723c0() -> ! {
    todo!("0xa723c0")
}

// 0xa723f8 — __ZN6RakNet16ReliabilityLayer14SetTimeoutTimeEj
// type: int __fastcall(int this, unsigned int)
#[doc(alias = "RakNet::ReliabilityLayer::SetTimeoutTime(unsigned int)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer14SetTimeoutTimeEj")]
pub fn stub_0xa723f8() -> ! {
    todo!("0xa723f8")
}

// 0xa72400 — __ZN6RakNet16ReliabilityLayer14GetTimeoutTimeEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::GetTimeoutTime(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer14GetTimeoutTimeEv")]
pub fn stub_0xa72400() -> ! {
    todo!("0xa72400")
}

// 0xa72408 — __ZN6RakNet16ReliabilityLayer20FreeThreadSafeMemoryEv
// type: int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::FreeThreadSafeMemory(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer20FreeThreadSafeMemoryEv")]
pub fn stub_0xa72408() -> ! {
    todo!("0xa72408")
}

// 0xa72d5c — __ZN6RakNet16ReliabilityLayer24ClearPacketsAndDatagramsEv
// type: unsigned int __fastcall(RakNet::ReliabilityLayer *this)
#[doc(alias = "RakNet::ReliabilityLayer::ClearPacketsAndDatagrams(void)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer24ClearPacketsAndDatagramsEv")]
pub fn stub_0xa72d5c() -> ! {
    todo!("0xa72d5c")
}

// 0xa72e94 — __ZN6RakNet16ReliabilityLayer38HandleSocketReceiveFromConnectedPlayerEPKcjRNS_13SystemAddressERN14DataStructures4ListIPNS_16PluginInterface2EEEiiPNS_12RakNetRandomEtjyRNS_9BitStreamE
// type: int __fastcall(int, unsigned __int8 *, unsigned int, _DWORD *, _DWORD *, int, int, RakNet::RakNetRandom *, RakNet::SystemAddress *, unsigned __int16, unsigned __int64, RakNet::BitStream *)
#[doc(alias = "RakNet::ReliabilityLayer::HandleSocketReceiveFromConnectedPlayer(char const*,unsigned int,RakNet::SystemAddress &,DataStructures::List<RakNet::PluginInterface2 *> &,int,int,RakNet::RakNetRandom *,unsigned short,unsigned int,unsigned long long,RakNet::BitStream &)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer38HandleSocketReceiveFromConnectedPlayerEPKcjRNS_13SystemAddressERN14DataStructures4ListIPNS_16PluginInterface2EEEiiPNS_12RakNetRandomEtjyRNS_9BitStreamE")]
pub fn stub_0xa72e94() -> ! {
    todo!("0xa72e94")
}

// 0xa74514 — __ZN6RakNet16ReliabilityLayer57RemovePacketFromResendListAndDeleteOlderReliableSequencedENS_8uint24_tEyRN14DataStructures4ListIPNS_16PluginInterface2EEERKNS_13SystemAddressE
// type: int __fastcall(int, _DWORD *, unsigned __int64, _DWORD *, _DWORD *)
#[doc(alias = "RakNet::ReliabilityLayer::RemovePacketFromResendListAndDeleteOlderReliableSequenced(RakNet::uint24_t,unsigned long long,DataStructures::List<RakNet::PluginInterface2 *> &,RakNet::SystemAddress const&)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer57RemovePacketFromResendListAndDeleteOlderReliableSequencedENS_8uint24_tEyRN14DataStructures4ListIPNS_16PluginInterface2EEERKNS_13SystemAddressE")]
pub fn stub_0xa74514() -> ! {
    todo!("0xa74514")
}

// 0xa74750 — __ZN6RakNet16ReliabilityLayer33CreateInternalPacketFromBitStreamEPNS_9BitStreamEy
// type: int __fastcall(RakNet::ReliabilityLayer *this, RakNet::BitStream *, unsigned __int64)
#[doc(alias = "RakNet::ReliabilityLayer::CreateInternalPacketFromBitStream(RakNet::BitStream *,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer33CreateInternalPacketFromBitStreamEPNS_9BitStreamEy")]
pub fn stub_0xa74750() -> ! {
    todo!("0xa74750")
}

// 0xa749fc — __ZN6RakNet16ReliabilityLayer25InsertIntoSplitPacketListEPNS_14InternalPacketEy
// type: unsigned int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "RakNet::ReliabilityLayer::InsertIntoSplitPacketList(RakNet::InternalPacket *,unsigned long long)")]
#[doc(alias = "__ZN6RakNet16ReliabilityLayer25InsertIntoSplitPacketListEPNS_14InternalPacketEy")]
pub fn stub_0xa749fc() -> ! {
    todo!("0xa749fc")
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
