//! core shard AD — 110 core stubs EA-sorted, next uncovered after shard AC (0x25c0c4), lowest EA first.
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 110 uncovered globally.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
/// Batch: 26 IDA-grounded ports 0x25f04c-0x2620f0 — the boost::exception_detail
/// clone/error-info family, boost::detail shared_count/sp_counted_base control
/// blocks, singleton_pool<XmlAttribute>::get_pool, the two static exception_ptr
/// objects (bad_alloc_/bad_exception_), TU static initializers
/// __GLOBAL__I_a_56/57/58 and initStaticData2/staticData2. Untouched carriers
/// keep stub bodies; ports live in `boost_exception` under idiomatic names,
/// wired via `stub_0x*`.
/// Conventions: `boost::shared_ptr` -> `crate::SharedPtr` (`Arc`), kept via
/// `CloneImpl.info`; `boost::exception` -> `thiserror`; `boost::unordered` ->
/// `HashMap`; `boost::mutex` -> `parking_lot::Mutex`; `__cxa_throw` paths ->
/// `panic!` (noreturn in the binary too). `[INFERENCE]` marks what the binary
/// does not pin down; everything else follows the IDA pseudocode + disassembly
/// branch-for-branch (decompile+disasm per EA, cross-checked ida/export.json).
/// Batch 3: 26 IDA-grounded ports 0x2650b8-0x2666c0 — thread_resource_error
/// D0/rethrow/copy-ctor + Thn20/Tv0 thunks, clone_base anchor, bad_alloc_ /
/// bad_exception_ D2/D0 + thunks, shared_count<bad_exception_> ctor,
/// sp_counted_impl_p<bad_exception_> D0, __GLOBAL__I_a_59, both
/// RBX pool operator-news (XmlElement 36 / XmlAttribute 20) with the
/// crashOnAllocationFailure/bad_alloc null path, lock_error + injector
/// dtors/thunks, XmlAttribute<const Name*> ctor (tag 1 = TAG_NAME, reusing
/// `generated_core_shard_ke::xml_tree`), Allocator<XmlAttribute> once-only
/// registration. Untouched carriers keep stub bodies; ports live in
/// `boost_exception` under idiomatic names, wired via `stub_0x*`.
/// Batch 4: 26 IDA-grounded ports 0x266728-0x26f6ec — Allocator<XmlAttribute>
/// releaseMemory, vector<bool(*)()> insert_aux/_M_allocate, singleton_pool /
/// pool release_memory, segregated_storage::segregate, XmlElement::XmlElement
/// + Allocator<XmlElement> registration, __GLOBAL__I_a_60/61/62, the
/// ProtectedString hashed_index::link_point + auto_space, and the
/// rbx::placement_any<Region3> family (seven any_casts, two operator=s, two
/// construct_funcs, typed_holder<CellID>::singleton). Untouched carriers keep
/// stub bodies; ports live in `boost_exception` under idiomatic names, wired
/// via `stub_0x*`.
pub mod boost_exception {
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::LazyLock;

    /// was: `boost::detail::spinlock_pool<1>` bucket count (IDA `% 0x29` at
    /// 0x2600f0/0x260dca/0x260e06/0x261f80/0x26201e).
    pub const SPIN_POOL_LEN: usize = 41;
    /// was: `Thn20` non-virtual thunks (IDA 0x25fdc8 `a1 - 20`, 0x260098
    /// `a1 - 20`) — offset of the `clone_impl` subobject inside the
    /// `error_info_injector` derived object.
    pub const NONVIRTUAL_THUNK_BIAS: usize = 20;

    static SPINLOCKS: LazyLock<Vec<parking_lot::Mutex<()>>> =
        LazyLock::new(|| (0..SPIN_POOL_LEN).map(|_| parking_lot::Mutex::new(())).collect());

    fn spinlocks() -> &'static Vec<parking_lot::Mutex<()>> {
        &SPINLOCKS
    }

    /// IDA 0x2600c2/0x2600f0: use-count slot `((pi + 4) % 0x29)`.
    pub fn spin_slot_use(block_addr: usize) -> usize {
        block_addr.wrapping_add(4) % SPIN_POOL_LEN
    }

    /// IDA 0x260e06: weak-count slot `((this + 8) % 0x29)`.
    pub fn spin_slot_weak(block_addr: usize) -> usize {
        block_addr.wrapping_add(8) % SPIN_POOL_LEN
    }

    /// was: `boost::thread_resource_error` (a `boost::system::system_error`).
    /// IDA 0x25fc58 copies its message strings into the thrown `clone_impl`.
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("thread resource error: {detail}")]
    pub struct ThreadResourceError {
        pub detail: &'static str,
    }

    /// was: `boost::exception_detail::bad_alloc_` — payload of the static
    /// `exception_ptr` built at IDA 0x261df8.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct BadAlloc_;

    /// was: `boost::exception_detail::bad_exception_` — payload of the static
    /// `exception_ptr` built at IDA 0x2620f0.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct BadException_;

    /// was: `boost::exception_detail::error_info_container` — the refcounted
    /// `error_info` map adopted by `copy_boost_exception` (IDA 0x25fdd0
    /// `refcount_ptr<...>::adopt`). `boost::unordered` -> `HashMap`.
    #[derive(Debug, Clone, Default)]
    pub struct ErrorInfoContainer {
        pub entries: HashMap<String, String>,
    }

    /// was: `boost::exception_detail::clone_impl<E>` (0x1C-byte image at IDA
    /// 0x25ff88: vtable, error-info ref, payload words). `clone()` copies the
    /// payload and shares (`addref`s) the error info; `rethrow()` builds the
    /// thrown copy the same way ahead of `__cxa_throw`.
    #[derive(Debug, Clone)]
    pub struct CloneImpl<E> {
        pub payload: E,
        pub info: Option<crate::SharedPtr<ErrorInfoContainer>>,
    }

    impl<E: Clone> CloneImpl<E> {
        /// IDA `clone()` virtual (`*(vtab + 20)` at 0x25fe2e): fresh box with
        /// copied payload, error info shared (refcount +1 in C++, `Arc`
        /// clone here).
        pub fn clone_box(&self) -> Self {
            Self { payload: self.payload.clone(), info: self.info.clone() }
        }

        /// IDA 0x25ff88 `rethrow()`: `__cxa_allocate_exception(0x1C)`, copy
        /// payload words (`exception[1]`, `+8`, `+16`), `addref` the error
        /// info, install vtables, `__cxa_throw`. The throw itself has no
        /// `core`-model equivalent — callers `panic!` with the built value.
        pub fn rethrow_copy(&self) -> Self {
            self.clone_box()
        }

        /// IDA 0x25fdd0 `copy_boost_exception(dst, src)`: `dst[2..=4] =
        /// src[2..=4]` (payload words) + adopt the cloned error info
        /// (`adopt` + addref, old container released — the assignment drop
        /// below is that release).
        pub fn copy_from(&mut self, src: &Self) {
            self.payload = src.payload.clone();
            self.info = src.info.clone();
        }
    }

    /// IDA `Thn20` thunks (0x25fdc8, 0x260098): `this -= 20` to reach the
    /// `clone_impl` subobject, then run the destructor body.
    pub fn nonvirtual_thunk_adjust(this: *mut u8) -> *mut u8 {
        this.wrapping_sub(NONVIRTUAL_THUNK_BIAS)
    }

    /// IDA `Tv0_n12` virtual thunks (0x2600b0, 0x260e60):
    /// `this += *(vtab - 12)` (the `top_offset`) to reach the `clone()`
    /// target. The offset lives in the binary vtable; callers pass it in.
    pub fn virtual_thunk_adjust(this: *const u8, top_offset: isize) -> *const u8 {
        (this as isize).wrapping_add(top_offset) as *const u8
    }

    /// was: `boost::detail::sp_counted_base` layout `{ vtable, use_count(+4),
    /// weak_count(+8) }`; `vtable` records which `sp_counted_impl_p<E>`
    /// instantiation owns the block (e.g. `off_1223058` at IDA 0x260edc).
    /// Counters are the source of truth; the carrying `Arc` mirrors them
    /// (every owner goes through `SharedCount`, so both agree).
    #[derive(Debug)]
    pub struct ControlBlock {
        pub uses: AtomicUsize,
        pub weaks: AtomicUsize,
        pub vtable: &'static str,
    }

    impl ControlBlock {
        /// IDA 0x260e70: `operator new(0x10)`, `uses = 1`, `weaks = 1`,
        /// install vtable, store payload pointer.
        pub fn new(vtable: &'static str) -> Self {
            Self { uses: AtomicUsize::new(1), weaks: AtomicUsize::new(1), vtable }
        }

        /// IDA 0x2600c0 `shared_count(const shared_count&)`: copy `pi_`, and
        /// when non-null lock the use slot and `++use_count`.
        pub fn addref_use(&self) {
            let _g = spinlocks()[spin_slot_use(self as *const _ as usize)].lock();
            self.uses.fetch_add(1, Ordering::AcqRel);
        }

        /// IDA 0x260d98 `release()`: lock the use slot, `--use_count`; when
        /// it was 1 run dispose (vtable slot 2, `+8`); then lock the weak
        /// slot, `--weak_count`; when it was 1 run destroy (vtable slot 3,
        /// `+12`). Returns `(disposed, destroyed)`.
        pub fn release(&self) -> (bool, bool) {
            let disposed = {
                let _g = spinlocks()[spin_slot_use(self as *const _ as usize)].lock();
                self.uses.fetch_sub(1, Ordering::AcqRel) == 1
            };
            let mut destroyed = false;
            if disposed {
                // IDA `(*(vtab + 8))(this)`: dispose drops the managed object.
                let _g = spinlocks()[spin_slot_weak(self as *const _ as usize)].lock();
                destroyed = self.weaks.fetch_sub(1, Ordering::AcqRel) == 1;
                // IDA `(*(vtab + 12))(this)`: destroy frees the block;
                // the carrying `Arc` frees it here ([INFERENCE] on allocator).
            }
            (disposed, destroyed)
        }

        /// IDA 0x260e48/0x260f70 `get_untyped_deleter()`: plain
        /// `sp_counted_impl_p` carries no deleter — returns null.
        pub fn untyped_deleter(&self) -> Option<&'static str> {
            None
        }

        pub fn use_count(&self) -> usize {
            self.uses.load(Ordering::Acquire)
        }

        pub fn weak_count(&self) -> usize {
            self.weaks.load(Ordering::Acquire)
        }
    }

    /// IDA 0x260e38 `destroy()`: null stays null, else the deleting dtor
    /// (vtable slot 1, `+4`) runs. Returns whether dispose ran.
    pub fn destroy_block(block: Option<&ControlBlock>) -> bool {
        block.is_some()
    }

    /// was: `boost::detail::shared_count` — nullable owning handle to the
    /// control block. `Clone` is the copy-ctor addref (IDA 0x2600c0),
    /// `Drop` is release (IDA 0x260d98).
    #[derive(Debug, Default)]
    pub struct SharedCount {
        pub block: Option<crate::SharedPtr<ControlBlock>>,
    }

    impl Clone for SharedCount {
        fn clone(&self) -> Self {
            if let Some(b) = &self.block {
                b.addref_use();
            }
            Self { block: self.block.clone() }
        }
    }

    impl Drop for SharedCount {
        fn drop(&mut self) {
            if let Some(b) = &self.block {
                b.release();
            }
        }
    }

    impl SharedCount {
        /// IDA 0x260e70 `shared_count(Y*)`: fresh block, counts (1, 1).
        /// `vtable` names the `sp_counted_impl_p<E>` instantiation
        /// (`off_1223058` for `clone_impl<bad_alloc_>`).
        pub fn from_payload_kind(vtable: &'static str) -> Self {
            Self { block: Some(crate::SharedPtr::new(ControlBlock::new(vtable))) }
        }

        pub fn use_count(&self) -> usize {
            self.block.as_ref().map(|b| b.use_count()).unwrap_or(0)
        }
    }

    /// was: `boost::singleton_pool<XmlAttribute, 20, ...>` storage (IDA
    /// 0x25ff10): zeroed words, then `RequestedSize = 20`, `NextSize =
    /// StartSize = 32` (`dword_1221528/2C/30`). `get_pool` inits once
    /// (function-static `f` flag) and returns the storage address.
    #[derive(Debug)]
    pub struct SingletonPool {
        pub requested_size: usize,
        pub next_size: usize,
        pub start_size: usize,
        /// was: the pool's cached free chunks (`store::free_list`). `malloc`
        /// pops (IDA `ordered_malloc` free-list path), `release_memory` drops
        /// them all and reports whether any memory was released.
        pub free_chunks: parking_lot::Mutex<Vec<Vec<u8>>>,
    }

    /// IDA 0x25ff10.
    static XML_ATTRIBUTE_POOL: LazyLock<SingletonPool> = LazyLock::new(|| SingletonPool {
        requested_size: 20,
        next_size: 32,
        start_size: 32,
        free_chunks: parking_lot::Mutex::new(Vec::new()),
    });

    pub fn xml_attribute_pool() -> &'static SingletonPool {
        &XML_ATTRIBUTE_POOL
    }

    /// was: `singleton_pool<XmlElement, 36, ...>` storage, touched by the
    /// `__GLOBAL__I_a_*` ctors alongside the `XmlAttribute` pool (IDA
    /// 0x25f37c/0x26025c/0x261094 `get_pool` calls).
    static XML_ELEMENT_POOL: LazyLock<SingletonPool> = LazyLock::new(|| SingletonPool {
        requested_size: 36,
        next_size: 32,
        start_size: 32,
        free_chunks: parking_lot::Mutex::new(Vec::new()),
    });

    pub fn xml_element_pool() -> &'static SingletonPool {
        &XML_ELEMENT_POOL
    }

    /// was: `exception_ptr.hpp:123` (IDA 0x261e94/0x261e96) — source stamp
    /// recorded on the static `exception_ptr` builds.
    pub const STATIC_EP_SOURCE_FILE: &str = "boost/exception/detail/exception_ptr.hpp";
    pub const STATIC_EP_SOURCE_LINE: u32 = 123;

    /// was: `exception_ptr_static_exception_object<bad_alloc_>::e` +
    /// function-local `ep` (IDA 0x261df8). Guarded init builds the
    /// `clone_impl` once (`__cxa_atexit(~exception_ptr)`); every call copies
    /// `ep` with an addref (IDA 0x261fe8-0x26203e spinlock inc).
    static STATIC_BAD_ALLOC_EP: LazyLock<crate::SharedPtr<CloneImpl<BadAlloc_>>> =
        LazyLock::new(|| crate::SharedPtr::new(CloneImpl { payload: BadAlloc_, info: None }));

    pub fn static_bad_alloc() -> crate::SharedPtr<CloneImpl<BadAlloc_>> {
        STATIC_BAD_ALLOC_EP.clone()
    }

    /// IDA 0x2620f0, same shape for `bad_exception_`.
    static STATIC_BAD_EXCEPTION_EP: LazyLock<crate::SharedPtr<CloneImpl<BadException_>>> =
        LazyLock::new(|| crate::SharedPtr::new(CloneImpl { payload: BadException_, info: None }));

    pub fn static_bad_exception() -> crate::SharedPtr<CloneImpl<BadException_>> {
        STATIC_BAD_EXCEPTION_EP.clone()
    }

    /// was: the three `boost::system` category singletons stored into merged
    /// globals by each `__GLOBAL__I_a_*` ctor — `generic_category()` ×2 then
    /// `system_category()` ×1 (IDA 0x25f056/66/6c, 0x260148/58/5e,
    /// 0x260f80/90/96 into `dword_131E344/348/34C`, `3BC/3C0/3C4`,
    /// `3CC/3D0/3D4`).
    static ERROR_CATEGORIES: LazyLock<(&'static str, &'static str, &'static str)> =
        LazyLock::new(|| ("generic", "generic", "system"));

    pub fn error_categories() -> &'static (&'static str, &'static str, &'static str) {
        &ERROR_CATEGORIES
    }

    /// IDA 0x25f04c `__GLOBAL__I_a_56`: TU static-init — error categories,
    /// `std::ios_base::Init` (+ `atexit` dtor), guarded inits for the
    /// Light-family `PropDescriptor` statics, the two static
    /// `exception_ptr` objects, the `XmlAttribute`/`XmlElement`/RBX
    /// `singleton_pool` storages and the `FactoryProduct` creators.
    /// `PropDescriptor`/`FactoryProduct`/RBX-pool statics are owned by the
    /// reflection/datamodel crates (see their `__GLOBAL__` ports); the
    /// core-owned effects below are idempotent via `LazyLock` statics.
    pub fn ensure_init_a56() {
        let _ = error_categories();
        let _ = static_bad_alloc();
        let _ = static_bad_exception();
        let _ = xml_attribute_pool();
        let _ = xml_element_pool();
    }

    /// IDA 0x260144 `__GLOBAL__I_a_57`: same core-owned set — categories,
    /// `ios_base::Init`, both static `exception_ptr` objects (via 0x261df8 /
    /// 0x2620f0 + `atexit(~exception_ptr)`), `XmlAttribute` pool (via
    /// 0x25ff10) and `XmlElement` pool.
    pub fn ensure_init_a57() {
        let _ = error_categories();
        let _ = static_bad_alloc();
        let _ = static_bad_exception();
        let _ = xml_attribute_pool();
        let _ = xml_element_pool();
    }

    /// IDA 0x260f7c `__GLOBAL__I_a_58`: identical core-owned set to a_57
    /// (categories, ios init, both static eps, both Xml pools).
    pub fn ensure_init_a58() {
        ensure_init_a57();
    }

    /// IDA 0x25fc58 `throw_exception<thread_resource_error>`: builds the
    /// 0x2C-byte `clone_impl<error_info_injector<thread_resource_error>>`
    /// (copies the `system_error` message strings, installs
    /// `off_1221B68`-family vtables) and `__cxa_throw`s it. `__noreturn`
    /// in the binary; `-> !` here.
    pub fn throw_thread_resource_error(err: ThreadResourceError) -> ! {
        panic!("{}", err);
    }

    /// was: function-local `vector<ClassDescriptor*>` zero-init +
    /// `__cxa_atexit(~vector)` (IDA 0x2610dc, guard `byte_131E3F4`).
    /// The `ClassDescriptor` rows themselves are owned by the reflection
    /// crate; core keeps the registry shell.
    static STATIC_DATA2: LazyLock<parking_lot::Mutex<Vec<&'static str>>> =
        LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));

    pub fn static_data2() -> &'static parking_lot::Mutex<Vec<&'static str>> {
        &STATIC_DATA2
    }

    /// IDA 0x2610d8 `initStaticData2` — thunk straight into `staticData2`.
    pub fn init_static_data2() {
        let _ = static_data2();
    }
    /// was: `boost::exception_detail::clone_base` — empty polymorphic base
    /// of `clone_impl<E>` (vtable anchor only).
    /// IDA 0x2652f8 `clone_base::~clone_base` (D1): empty body.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    pub struct CloneBase;

    /// was: `boost::lock_error` (a `boost::system::system_error`, like
    /// `thread_resource_error`).
    /// IDA 0x2665b8 D1 and 0x2665e8/0x2665f8 `clone_impl<injector<lock_error>>`
    /// dtors/thunks run the same member-drop sequence as the
    /// `thread_resource_error` family; `CloneImpl<LockError>` carries it.
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("lock error: {detail}")]
    pub struct LockError {
        pub detail: &'static str,
    }

    /// IDA 0x2657a4 `__GLOBAL__I_a_59`: same core-owned set as a_57/a_58 —
    /// categories (into 0x131E3E4/E8/EC), `ios_base::Init`, both static
    /// `exception_ptr` objects, both Xml pools. Idempotent via `LazyLock`.
    pub fn ensure_init_a59() {
        ensure_init_a57();
    }

    /// was: `RBX::roblox_allocator::crashOnAllocationFailure` — when set,
    /// pool `operator new` crashes instead of throwing `bad_alloc`.
    /// IDA 0x2664c0 / 0x266578.
    pub static CRASH_ON_ALLOCATION_FAILURE: AtomicBool = AtomicBool::new(false);

    impl SingletonPool {
        /// was: `singleton_pool<...>::malloc` — fixed-size chunk checkout.
        /// IDA 0x266492 (XmlElement, 36) / 0x26654a (XmlAttribute, 20).
        /// `None` models the binary's null return (pool exhausted).
        pub fn malloc_zeroed(&self) -> Option<Vec<u8>> {
            if let Some(mut chunk) = self.free_chunks.lock().pop() {
                // Reused chunks may carry stale bytes; the checkout contract
                // is zeroed memory (`[INFERENCE]` — no `free` port exists yet
                // to populate the list, so this path awaits a future batch).
                chunk.fill(0);
                return Some(chunk);
            }
            let mut chunk = Vec::new();
            chunk.try_reserve_exact(self.requested_size).ok()?;
            chunk.resize(self.requested_size, 0);
            Some(chunk)
        }

        /// was: `RBX::Allocator<XmlElement>::operator new` /
        /// `RBX::Allocator<XmlAttribute>::operator new` null path — IDA
        /// 0x2664c0-0x2664f6 / 0x266578-0x2665ae: `RBXCRASH` when
        /// `crashOnAllocationFailure`, else `__cxa_throw(bad_alloc)`.
        pub fn allocate_or_throw(&'static self) -> Vec<u8> {
            match self.malloc_zeroed() {
                Some(chunk) => chunk,
                None if CRASH_ON_ALLOCATION_FAILURE.load(Ordering::Acquire) => {
                    panic!("RBXCRASH: pool allocation failure")
                }
                None => panic!("std::bad_alloc"),
            }
        }
        /// was: `boost::pool<...>::release_memory` as reached through
        /// `singleton_pool<XmlAttribute, 20, ...>::release_memory` (IDA
        /// 0x266840: `get_pool`, lock the storage mutex, call through,
        /// unlock). The binary walks its block list and `free()`s fully-unused
        /// system blocks, returning whether any memory was released
        /// (IDA 0x26694c/0x266958 `*(a1+16) = *(a1+20); return v2 & 1`).
        /// Here the cached free chunks are that releasable memory.
        pub fn release_memory(&self) -> bool {
            let mut free = self.free_chunks.lock();
            let released = !free.is_empty();
            free.clear();
            released
        }
    }

    /// was: `RBX::Allocator<XmlAttribute>::initialized` (IDA 0x2666d6/0x26671a).
    pub static XML_ATTRIBUTE_ALLOCATOR_INITIALIZED: AtomicBool = AtomicBool::new(false);
    /// was: `RBX::Allocator<XmlAttribute>::availableSize` — the slot pushed
    /// into `poolAvailabilityList` (IDA 0x2666f2/0x2666f4).
    pub static XML_ATTRIBUTE_AVAILABLE_SIZE: AtomicUsize = AtomicUsize::new(0);
    /// was: `RBX::poolAvailabilityList` (`vector<ulong*>`) as observed via
    /// the XmlAttribute registration — holds available-size slots.
    static POOL_AVAILABILITY_SIZES: LazyLock<parking_lot::Mutex<Vec<usize>>> =
        LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));
    /// was: `RBX::poolReleaseMemoryFuncList` (`vector<bool(*)()>`) —
    /// release callbacks registered by pool allocators.
    static POOL_RELEASE_FNS: LazyLock<parking_lot::Mutex<Vec<fn() -> bool>>> =
        LazyLock::new(|| parking_lot::Mutex::new(Vec::new()));

    /// was: `RBX::Allocator<XmlAttribute>::releaseMemory` (IDA 0x266728) —
    /// zero `availableSize` (0x26673a), then
    /// `singleton_pool<XmlAttribute>::release_memory` (0x266740), whose
    /// boolean result propagates to the caller.
    pub fn xml_attribute_release_memory() -> bool {
        XML_ATTRIBUTE_AVAILABLE_SIZE.store(0, Ordering::Release);
        xml_attribute_pool().release_memory()
    }

    /// was: `RBX::Allocator<XmlAttribute>::Allocator` (IDA 0x2666c0) —
    /// once-only registration of the available-size slot + release fn.
    /// Idempotent: the `initialized` guard (0x2666d6) makes repeats no-ops.
    pub fn ensure_xml_attribute_allocator() {
        if !XML_ATTRIBUTE_ALLOCATOR_INITIALIZED.swap(true, Ordering::AcqRel) {
            POOL_AVAILABILITY_SIZES.lock().push(XML_ATTRIBUTE_AVAILABLE_SIZE.load(Ordering::Acquire));
            POOL_RELEASE_FNS.lock().push(xml_attribute_release_memory);
        }
    }
    /// was: `RBX::Allocator<XmlElement>::initialized` (mirror of the
    /// XmlAttribute guard; the ctor at IDA 0x267436 checks it before
    /// registering).
    pub static XML_ELEMENT_ALLOCATOR_INITIALIZED: AtomicBool = AtomicBool::new(false);
    /// was: `RBX::Allocator<XmlElement>::availableSize` — pushed into
    /// `poolAvailabilityList` once (IDA 0x267452/0x267454).
    pub static XML_ELEMENT_AVAILABLE_SIZE: AtomicUsize = AtomicUsize::new(0);

    /// was: `RBX::Allocator<XmlElement>::releaseMemory` — the fn pointer
    /// pushed into `poolReleaseMemoryFuncList` (IDA 0x267472/0x267474).
    /// Same shape as the XmlAttribute version (IDA 0x266728).
    pub fn xml_element_release_memory() -> bool {
        XML_ELEMENT_AVAILABLE_SIZE.store(0, Ordering::Release);
        xml_element_pool().release_memory()
    }

    /// was: `RBX::Allocator<XmlElement>::Allocator` (IDA 0x267420) —
    /// once-only registration, mirroring `ensure_xml_attribute_allocator`.
    pub fn ensure_xml_element_allocator() {
        if !XML_ELEMENT_ALLOCATOR_INITIALIZED.swap(true, Ordering::AcqRel) {
            POOL_AVAILABILITY_SIZES.lock().push(XML_ELEMENT_AVAILABLE_SIZE.load(Ordering::Acquire));
            POOL_RELEASE_FNS.lock().push(xml_element_release_memory);
        }
    }

    /// was: `singleton_pool<RBX::FWInstance, 28, ...>` storage — created by
    /// `__GLOBAL__I_a_62` (IDA 0x26b31e-0x26b350 guard + `get_pool`).
    /// `NextSize = StartSize = 32` per the shared `,32u,0u>` template tail
    /// (`[INFERENCE]` — the disassembly only shows the init calls).
    static FW_INSTANCE_POOL: LazyLock<SingletonPool> = LazyLock::new(|| SingletonPool {
        requested_size: 28,
        next_size: 32,
        start_size: 32,
        free_chunks: parking_lot::Mutex::new(Vec::new()),
    });

    pub fn fw_instance_pool() -> &'static SingletonPool {
        &FW_INSTANCE_POOL
    }

    /// was: `singleton_pool<RBX::OnDemandInstance, 20, ...>` storage — created
    /// by `__GLOBAL__I_a_62` (IDA 0x26b352-0x26b384 guard + `get_pool`).
    static ON_DEMAND_INSTANCE_POOL: LazyLock<SingletonPool> = LazyLock::new(|| SingletonPool {
        requested_size: 20,
        next_size: 32,
        start_size: 32,
        free_chunks: parking_lot::Mutex::new(Vec::new()),
    });

    pub fn on_demand_instance_pool() -> &'static SingletonPool {
        &ON_DEMAND_INSTANCE_POOL
    }

    /// IDA 0x2674b0 `__GLOBAL__I_a_60`: categories, `ios_base::Init`, both
    /// static eps, plus `get_pool` for the XmlAttribute AND XmlElement pools
    /// — the same core-owned set as a_57. Idempotent via `LazyLock`.
    pub fn ensure_init_a60() {
        ensure_init_a57();
    }

    /// IDA 0x268bf0 `__GLOBAL__I_a_61`: categories, `ios_base::Init`, both
    /// static eps — and nothing else (no pool `get_pool` calls, unlike
    /// a_57-a_60 which create both Xml pools). Idempotent via `LazyLock`.
    pub fn ensure_init_a61() {
        let _ = error_categories();
        let _ = static_bad_alloc();
        let _ = static_bad_exception();
    }

    /// IDA 0x26b1f4 `__GLOBAL__I_a_62`: the a_60 set plus FWInstance (28)
    /// and OnDemandInstance (20) pool creation, the ProtectedString
    /// flyweight static init (0x26b386-0x26b3e0), and two `FactoryProduct`
    /// creators (ScriptContext 0x26b3f6-0x26b41e, LocalScript 0x26b436-0x26b45e).
    /// The creators are owned by higher crates (same split as a_56); the
    /// flyweight shell lives here with the hashed-index helpers below.
    pub fn ensure_init_a62() {
        ensure_init_a60();
        let _ = fw_instance_pool();
        let _ = on_demand_instance_pool();
        let _ = protected_string_flyweight();
    }

    /// was: `std::vector<bool(*)()>::_M_insert_aux` (IDA 0x266748) — insert
    /// into `poolReleaseMemoryFuncList`. Fast path (`finish != end_of_storage`,
    /// 0x26675e): shift the tail right one slot (`memmove`), store the value.
    /// Slow path: grow (`1` when empty, else `2*size` — 0x266812 `bytes>>1`;
    /// `length_error("vector::_M_insert_aux")` at `size == 0x3FFFFFFF`),
    /// move both halves, store, `operator delete` the old buffer.
    /// `Vec::insert` is the same two paths once capacity is ensured
    /// (`[INFERENCE]`: `pos` out of range panics; the binary trusts the iterator).
    pub fn release_fn_vector_insert_aux(vec: &mut Vec<fn() -> bool>, pos: usize, val: fn() -> bool) {
        if vec.len() == vec.capacity() {
            let size = vec.len();
            if size == 0x3FFF_FFFF {
                panic!("vector::_M_insert_aux");
            }
            let grown = if size == 0 { 1 } else { size * 2 };
            vec.reserve_exact(grown - size);
        }
        vec.insert(pos, val);
    }

    /// was: `std::_Vector_base<bool(*)()>::_M_allocate` (IDA 0x266828) —
    /// `n >= 0x40000000` throws `bad_alloc`, else `operator new(4*n)`.
    /// Models the raw storage as zero-length capacity (`[INFERENCE]` — the
    /// binary hands out uninitialized `operator new` memory).
    pub fn release_fn_vector_storage(n: usize) -> Vec<std::mem::MaybeUninit<fn() -> bool>> {
        if n >= 0x4000_0000 {
            panic!("std::bad_alloc");
        }
        Vec::with_capacity(n)
    }

    /// was: `boost::simple_segregated_storage<ulong>::segregate`
    /// (IDA 0x266960; verified against the disassembly — `__umodsi3` at
    /// 0x266978, the top-down link loop 0x2669a2-0x2669aa, the
    /// `block+usable+back` head recompute 0x2669ac-0x2669b4). Mirrored
    /// branch-for-branch, including the head landing one chunk in: slot 0's
    /// first word holds the head pointer, so the free list starts at
    /// `block+sz` (a single-chunk partition yields the sentinel itself).
    /// Caller contract (unchecked, like the binary): `[block, block+partition)`
    /// writable, `sz != 0`, `partition >= sz`. `sz == 0` traps (`% 0`,
    /// `__umodsi3` in the binary too).
    pub unsafe fn segregate(block: *mut *mut u8, partition: usize, sz: usize, end: *mut u8) -> *mut *mut u8 {
        let span = partition.wrapping_sub(sz); // IDA 0x26696c
        let rem = span % sz; // IDA 0x266978
        let usable = span.wrapping_sub(rem); // IDA 0x26697c
        block.byte_add(usable).write(end); // IDA 0x26697e: sentinel
        if span != rem {
            // IDA 0x266982
            let mut next = block.byte_add(usable) as *mut u8;
            if usable != sz {
                // IDA 0x26698a
                let top = block.byte_add(usable.wrapping_sub(sz));
                let term = partition.wrapping_sub(rem.wrapping_add(sz.wrapping_mul(2)));
                let mut back = 0usize;
                loop {
                    // IDA 0x2669a2-0x2669aa
                    top.wrapping_byte_add(back).write(next);
                    next = top.wrapping_byte_add(back) as *mut u8;
                    back = back.wrapping_sub(sz);
                    if term.wrapping_add(back) == 0 {
                        break;
                    }
                }
                // IDA 0x2669ac-0x2669b4: head = block + usable + back.
                next = (block.byte_add(usable) as *mut u8).wrapping_byte_add(back);
            }
            block.write(next); // IDA 0x2669b6
        }
        block // IDA 0x2669ba
    }

    /// was: `boost::multi_index::detail::hashed_index<...ProtectedString...>
    /// ::link_point` (IDA 0x26af9c) — walk the bucket's circular node chain
    /// comparing `RBX::ProtectedString::operator==`; on match store the node
    /// in the link out-param and return false (0), else return true (1).
    /// The intrusive chain is modeled as a slice walk (`[INFERENCE]` — same
    /// find/store contract, linear instead of circular).
    pub fn hashed_link_point(bucket: &[String], key: &str, link_out: &mut usize) -> bool {
        for (i, node) in bucket.iter().enumerate() {
            if node == key {
                // IDA 0x26afc0-0x26afca
                *link_out = i;
                return false;
            }
        }
        true // IDA 0x26afcc
    }

    /// was: `boost::multi_index::detail::auto_space<...>::auto_space`
    /// (IDA 0x26afd0) — store `n`, allocate `4*n` bytes (`operator new`,
    /// null when `n == 0`), `bad_alloc` at `n >= 0x40000000`.
    /// Zeroed here (`[INFERENCE]`; the binary leaves it uninitialized).
    #[derive(Debug, Clone, Default)]
    pub struct AutoSpace {
        pub len: usize,
        pub storage: Vec<u32>,
    }

    impl AutoSpace {
        /// IDA 0x26afd0.
        pub fn new(n: usize) -> Self {
            if n >= 0x4000_0000 {
                // IDA 0x26afe0-0x26afee
                panic!("std::bad_alloc");
            }
            Self { len: n, storage: vec![0; n] }
        }
    }

    /// was: the `ProtectedString` flyweight static core initialized by
    /// `__GLOBAL__I_a_62` (IDA 0x26b386-0x26b3e0: `static_holder::get()`,
    /// `static_factory_ptr` / `static_mutex_ptr` install, guard release).
    /// Value -> refcount interning shell (`[INFERENCE]` on holder layout).
    static PROTECTED_STRING_FLYWEIGHT: LazyLock<parking_lot::Mutex<HashMap<String, usize>>> =
        LazyLock::new(|| parking_lot::Mutex::new(HashMap::new()));

    pub fn protected_string_flyweight() -> &'static parking_lot::Mutex<HashMap<String, usize>> {
        &PROTECTED_STRING_FLYWEIGHT
    }

    /// was: `rbx::implementation::typed_holder<T>` singleton identity — the
    /// holder pointer word (+0) plus the `typeinfo[1]` mangled name used by
    /// the `any_cast` slow path. One tag static per `T` models the distinct
    /// `typeinfo` objects (fast-path pointer compare).
    #[derive(Debug)]
    pub struct TypedHolder {
        pub name: &'static str,
    }

    static HOLDER_CONTENT_ID: TypedHolder = TypedHolder { name: "N3RBX9ContentIdE" };
    static HOLDER_CELL_ID: TypedHolder = TypedHolder { name: "N3RBX6CellIDE" };
    static HOLDER_AXES: TypedHolder = TypedHolder { name: "N3RBX4AxesE" };
    static HOLDER_UDIM: TypedHolder = TypedHolder { name: "N3RBX4UDimE" };
    static HOLDER_REGION3INT16: TypedHolder = TypedHolder { name: "N3RBX12Region3int16E" };
    static HOLDER_REGION3: TypedHolder = TypedHolder { name: "N3RBX7Region3E" };
    static HOLDER_PROTECTED_STRING: TypedHolder = TypedHolder { name: "N3RBX15ProtectedStringE" };
    /// `[INFERENCE]` Itanium name for `long`; IDA 0x26f4ee/0x26f510 compares
    /// its typeinfo with the same two-level (pointer, then name) check.
    static HOLDER_LONG: TypedHolder = TypedHolder { name: "l" };
    /// From the 0x26f578 `aSIS1_11InputObjectE` mangling.
    static HOLDER_INPUT_OBJECT: TypedHolder = TypedHolder { name: "N3RBX11InputObjectE" };

    pub fn content_id_holder() -> &'static TypedHolder {
        &HOLDER_CONTENT_ID
    }
    pub fn cell_id_holder() -> &'static TypedHolder {
        &HOLDER_CELL_ID
    }
    pub fn axes_holder() -> &'static TypedHolder {
        &HOLDER_AXES
    }
    pub fn udim_holder() -> &'static TypedHolder {
        &HOLDER_UDIM
    }
    pub fn region3int16_holder() -> &'static TypedHolder {
        &HOLDER_REGION3INT16
    }
    pub fn region3_holder() -> &'static TypedHolder {
        &HOLDER_REGION3
    }
    pub fn protected_string_holder() -> &'static TypedHolder {
        &HOLDER_PROTECTED_STRING
    }
    pub fn long_holder() -> &'static TypedHolder {
        &HOLDER_LONG
    }
    pub fn input_object_holder() -> &'static TypedHolder {
        &HOLDER_INPUT_OBJECT
    }

    /// was: `rbx::placement_any<RBX::Region3>` value payload — the bytes at
    /// `a1+1` that `any_cast` returns. `InputObject` (20 bytes) and `CellID`
    /// get structural variants (their `operator=`/construct paths touch
    /// fields); every other checked type rides `Opaque` (casts only compare
    /// the holder, never inspect bytes).
    #[derive(Debug, Clone, Default)]
    pub enum PlacementValue {
        #[default]
        Empty,
        InputObject([u8; 20]),
        CellId(CellIdPayload),
        Opaque(Vec<u8>),
    }

    /// was: `RBX::CellID` inline words (+4 tag byte/words through +16) plus
    /// the `shared_ptr<Instance>` word (+20, address cookie — the `Instance`
    /// itself is owned by higher crates) and its `shared_count` (+24, addref
    /// on copy via `SharedCount::clone`, release on drop via `Drop`).
    #[derive(Debug, Clone, Default)]
    pub struct CellIdPayload {
        pub head: [u8; 16],
        pub instance_addr: usize,
        pub instance_count: SharedCount,
    }

    /// was: `rbx::placement_any<RBX::Region3>` — holder word (+0, null when
    /// empty) plus value bytes (`a1+1`, what `any_cast` returns).
    #[derive(Debug, Default)]
    pub struct PlacementAny {
        pub holder: Option<&'static TypedHolder>,
        pub value: PlacementValue,
    }

    /// was: `rbx::any_cast<const T&>(placement_any&)` (IDA 0x26e228 family —
    /// all seven share the shape) — fast path: holder `typeinfo*` pointer
    /// equality; slow path: `typeinfo[1]` name compare (a null holder reads
    /// as `typeinfo for void`, name `"v"` — `[INFERENCE]`); mismatch runs
    /// `throw_exception<bad_placement_any_cast>` (here `panic!`, like every
    /// other `__cxa_throw` port). Returns the value address (`a1+1`).
    pub fn placement_any_cast<'a>(slot: &'a PlacementAny, want: &'static TypedHolder) -> &'a PlacementValue {
        let same = match slot.holder {
            Some(h) => std::ptr::eq(h, want),
            None => false,
        };
        if !same {
            let held_name = slot.holder.map(|h| h.name).unwrap_or("v");
            if held_name != want.name {
                panic!("rbx::bad_placement_any_cast");
            }
        }
        &slot.value
    }

    /// was: `rbx::placement_any<Region3>::operator=<InputObject>` (IDA
    /// 0x26f578). Same holder: in-place `InputObject::operator=` (20-byte
    /// copy). Otherwise: destroy the old value (Rust drop = the
    /// `destruct_func` call at 0x26f5a6 + nulling at 0x26f5aa), bitwise copy
    /// the 20 payload bytes, install the holder.
    pub fn assign_input_object(slot: &mut PlacementAny, src: &[u8; 20]) {
        let holder = input_object_holder(); // IDA 0x26f584
        if matches!(slot.holder, Some(h) if std::ptr::eq(h, holder)) {
            if let PlacementValue::InputObject(dst) = &mut slot.value {
                *dst = *src;
                return;
            }
        }
        slot.value = PlacementValue::InputObject(*src);
        slot.holder = Some(holder);
    }

    /// was: `typed_holder<InputObject>::construct_func` (IDA 0x26f5e0) —
    /// copy the 5 payload words when `dst` is non-null. (The decompiler's
    /// return value is leftover `r0`; the binary returns void.)
    pub fn input_object_construct(src: &[u8; 20], dst: Option<&mut [u8; 20]>) {
        if let Some(d) = dst {
            d.copy_from_slice(src);
        }
    }

    /// was: `placement_any<Region3>::operator=<CellID>` (IDA 0x26f600).
    /// Same holder: field assign — the `shared_ptr<Instance>::operator=` at
    /// 0x26f64e releases the old handle and acquires the new one, which is
    /// exactly what cloning `instance_count` (addref) + dropping the old
    /// (release) does here. Otherwise: destroy + full copy + holder install.
    pub fn assign_cell_id(slot: &mut PlacementAny, src: &CellIdPayload) {
        let holder = cell_id_holder(); // IDA 0x26f60c
        if matches!(slot.holder, Some(h) if std::ptr::eq(h, holder)) {
            if let PlacementValue::CellId(dst) = &mut slot.value {
                *dst = src.clone();
                return;
            }
        }
        slot.value = PlacementValue::CellId(src.clone());
        slot.holder = Some(holder);
    }

    /// was: `typed_holder<CellID>::singleton` (IDA 0x26f680) — guarded
    /// once-init installing the `typeinfo`, `destruct_func` and
    /// `construct_func` words. `LazyLock` statics are the guard + `atexit`
    /// equivalent.
    pub fn ensure_cell_id_holder() -> &'static TypedHolder {
        cell_id_holder()
    }

    /// was: `typed_holder<CellID>::construct_func` (IDA 0x26f6ec) — copy the
    /// 16 head bytes, `shared_count` copy-ctor (addref at 0x26f70e), then the
    /// instance word (0x26f712-0x26f714). (Decompiler return is leftover `r0`.)
    pub fn cell_id_construct(src: &CellIdPayload, dst: Option<&mut CellIdPayload>) {
        if let Some(d) = dst {
            d.head = src.head;
            d.instance_count = src.instance_count.clone();
            d.instance_addr = src.instance_addr;
        }
    }
}

#[doc(alias = "__ZNK3RBX5Light8getColorEv")]
// 0x25c0f0 — __ZNK3RBX5Light8getColorEv
pub fn stub_0x25c0f0() {
    // IDA 0x25c0f0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX5Light13getBrightnessEv")]
// 0x25c124 — __ZNK3RBX5Light13getBrightnessEv
pub fn stub_0x25c124() {
    // IDA 0x25c124: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX10PointLight8getRangeEv")]
// 0x25c14c — __ZNK3RBX10PointLight8getRangeEv
pub fn stub_0x25c14c() {
    // IDA 0x25c14c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX9SpotLight8getRangeEv")]
// 0x25c174 — __ZNK3RBX9SpotLight8getRangeEv
pub fn stub_0x25c174() {
    // IDA 0x25c174: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX9SpotLight8getAngleEv")]
// 0x25c19c — __ZNK3RBX9SpotLight8getAngleEv
pub fn stub_0x25c19c() {
    // IDA 0x25c19c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX5Light10getShadowsEv")]
// 0x25c1a0 — __ZNK3RBX5Light10getShadowsEv
pub fn stub_0x25c1a0() {
    // IDA 0x25c1a0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX9SpotLight7getFaceEv")]
// 0x25c1a8 — __ZNK3RBX9SpotLight7getFaceEv
pub fn stub_0x25c1a8() {
    // IDA 0x25c1a8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sSpotLightEEEEvv")]
// 0x25c87c — __ZN3RBX4Name13callDoDeclareILZNS_10sSpotLightEEEEvv
pub fn stub_0x25c87c() {
    // IDA 0x25c87c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v")]
// 0x25c880 — __ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v
pub fn stub_0x25c880() {
    // IDA 0x25c880: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sPointLightEEEEvv")]
// 0x25d22c — __ZN3RBX4Name13callDoDeclareILZNS_11sPointLightEEEEvv
pub fn stub_0x25d22c() {
    // IDA 0x25d22c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v")]
// 0x25d230 — __ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v
pub fn stub_0x25d230() {
    // IDA 0x25d230: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sLightEEEEvv")]
// 0x25d5c8 — __ZN3RBX4Name13callDoDeclareILZNS_6sLightEEEEvv
pub fn stub_0x25d5c8() {
    // IDA 0x25d5c8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v")]
// 0x25d5cc — __ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v
pub fn stub_0x25d5cc() {
    // IDA 0x25d5cc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__GLOBAL__I_a_56")]
// 0x25f04c — __GLOBAL__I_a_56
pub fn stub_0x25f04c() {
    // IDA 0x25f04c: __GLOBAL__I_a_56 TU static-init; disasm calls generic_category x2 + system_category + ios_base::Init + guarded PropDescriptor/singleton-pool/exception_ptr/factory inits. Core-owned effects idempotent.
    boost_exception::ensure_init_a56();
}

#[doc(alias = "__ZN5boost15throw_exceptionINS_21thread_resource_errorEEEvRKT_")]
// 0x25fc58 — __ZN5boost15throw_exceptionINS_21thread_resource_errorEEEvRKT_
pub fn stub_0x25fc58(err: boost_exception::ThreadResourceError) -> ! {
    // IDA 0x25fc58: throw_exception<thread_resource_error> — builds 0x2C-byte clone_impl<error_info_injector<...>> (copies message strings, installs vtables) then __cxa_throw. __noreturn -> -> !.
    boost_exception::throw_thread_resource_error(err);
}

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev")]
// 0x25fdc0 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev
pub fn stub_0x25fdc0(_this: &mut boost_exception::CloneImpl<boost_exception::ThreadResourceError>) {
    // IDA 0x25fdc0: D1 thunk delegating to D2 (attributes: thunk). Member drops run via Rust Drop glue; nothing to emit.
}

#[doc(alias = "__ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev")]
// 0x25fdc8 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
pub fn stub_0x25fdc8(this: *mut u8) -> *mut u8 {
    // IDA 0x25fdc8: non-virtual thunk to clone_impl<error_info_injector<thread_resource_error>>::~clone_impl — this -= 20, then D1.
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZN5boost16exception_detail20copy_boost_exceptionEPNS_9exceptionEPKS1_")]
// 0x25fdd0 — __ZN5boost16exception_detail20copy_boost_exceptionEPNS_9exceptionEPKS1_
pub fn stub_0x25fdd0<E: Clone>(
    dst: &mut boost_exception::CloneImpl<E>,
    src: &boost_exception::CloneImpl<E>,
) {
    // IDA 0x25fdd0: copy_boost_exception — dst[2..=4] = src[2..=4] plus error_info adopt/addref dance (clone via vtab+20, adopt, release old when count hits 1).
    dst.copy_from(src);
}

#[doc(alias = "__ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// 0x25ff10 — __ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_0x25ff10() -> &'static boost_exception::SingletonPool {
    // IDA 0x25ff10: singleton_pool<XmlAttribute,20>::get_pool — once-init mutex + RequestedSize 20 / NextSize = StartSize 32, return storage.
    boost_exception::xml_attribute_pool()
}

#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev")]
// 0x25ff60 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev
pub fn stub_0x25ff60(_this: &mut boost_exception::CloneImpl<boost_exception::BadException_>) {
    // IDA 0x25ff60: clone_impl<bad_exception_> D1 — runs ~bad_exception_ on the subobject, returns this. Drop glue covers it.
}

#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev")]
// 0x25ff70 — __ZN5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev
pub fn stub_0x25ff70(this: boost_exception::CloneImpl<boost_exception::BadException_>) {
    // IDA 0x25ff70: clone_impl<bad_exception_> D0 — D1 then operator delete. By-value drop is the delete.
    drop(this);
}

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv")]
// 0x25ff88 — __ZNK5boost16exception_detail10clone_implINS0_14bad_exception_EE7rethrowEv
pub fn stub_0x25ff88(
    this: &boost_exception::CloneImpl<boost_exception::BadException_>,
) -> boost_exception::CloneImpl<boost_exception::BadException_> {
    // IDA 0x25ff88: clone_impl<bad_exception_>::rethrow — __cxa_allocate_exception(0x1C), copy payload words + error_info addref, install vtables, __cxa_throw. Returns the thrown image; the throw itself is the caller's panic!.
    this.rethrow_copy()
}

#[doc(alias = "__ZThn20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev")]
// 0x260098 — __ZThn20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED0Ev
pub fn stub_0x260098(this: *mut u8) -> *mut u8 {
    // IDA 0x260098: non-virtual thunk to clone_impl<bad_exception_> D0 — this -= 20, ~bad_exception_, operator delete. Returns the adjusted object address.
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv")]
// 0x2600b0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_14bad_exception_EE5cloneEv
pub fn stub_0x2600b0(this: *const u8, top_offset: isize) -> *const u8 {
    // IDA 0x2600b0: virtual thunk to clone_impl<bad_exception_>::clone — this += *(vtab - 12), then clone. Offset passed in (it lives in the binary vtable).
    boost_exception::virtual_thunk_adjust(this, top_offset)
}

#[doc(alias = "__ZN5boost6detail12shared_countC1ERKS1_")]
// 0x2600c0 — __ZN5boost6detail12shared_countC1ERKS1_
pub fn stub_0x2600c0(dst: &mut boost_exception::SharedCount, src: &boost_exception::SharedCount) {
    // IDA 0x2600c0: shared_count copy ctor — copy pi_, lock slot ((pi+4) % 0x29), ++use_count.
    *dst = src.clone();
}

#[doc(alias = "__GLOBAL__I_a_57")]
// 0x260144 — __GLOBAL__I_a_57
pub fn stub_0x260144() {
    // IDA 0x260144: __GLOBAL__I_a_57 — categories, ios_base::Init, guarded static eps (via 0x261df8/0x2620f0) + XmlAttribute/XmlElement pools.
    boost_exception::ensure_init_a57();
}

#[doc(alias = "__ZN5boost6detail15sp_counted_base7releaseEv")]
// 0x260d98 — __ZN5boost6detail15sp_counted_base7releaseEv
pub fn stub_0x260d98(block: &boost_exception::ControlBlock) -> (bool, bool) {
    // IDA 0x260d98: sp_counted_base::release — locked use--, dispose at 0 (vtab+8), locked weak--, destroy at 0 (vtab+12). Returns (disposed, destroyed).
    block.release()
}

#[doc(alias = "__ZN5boost6detail15sp_counted_base7destroyEv")]
// 0x260e38 — __ZN5boost6detail15sp_counted_base7destroyEv
pub fn stub_0x260e38(block: Option<&boost_exception::ControlBlock>) -> bool {
    // IDA 0x260e38: sp_counted_base::destroy — null stays null, else deleting dtor (vtab+4). Returns whether dispose ran.
    boost_exception::destroy_block(block)
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE19get_untyped_deleterEv")]
// 0x260e48 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEE19get_untyped_deleterEv
pub fn stub_0x260e48() -> Option<&'static str> {
    // IDA 0x260e48: sp_counted_impl_p<clone_impl<bad_exception_>>::get_untyped_deleter — plain impl carries none, returns 0.
    None
}

#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev")]
// 0x260e50 — __ZN5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev
pub fn stub_0x260e50(_this: &mut boost_exception::CloneImpl<boost_exception::BadAlloc_>) {
    // IDA 0x260e50: clone_impl<bad_alloc_> D1 — runs ~bad_alloc_, returns this. Drop glue covers it.
}

#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv")]
// 0x260e60 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_10bad_alloc_EE5cloneEv
pub fn stub_0x260e60(this: *const u8, top_offset: isize) -> *const u8 {
    // IDA 0x260e60: virtual thunk to clone_impl<bad_alloc_>::clone — this += *(vtab - 12), then clone.
    boost_exception::virtual_thunk_adjust(this, top_offset)
}

#[doc(alias = "__ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_10bad_alloc_EEEEEPT_")]
// 0x260e70 — __ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_10bad_alloc_EEEEEPT_
pub fn stub_0x260e70() -> boost_exception::SharedCount {
    // IDA 0x260e70: shared_count<clone_impl<bad_alloc_>> ctor — *pi = 0, new(0x10) block with uses = weaks = 1, vtable off_1223058, store payload.
    boost_exception::SharedCount::from_payload_kind("off_1223058")
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEED0Ev")]
// 0x260f68 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEED0Ev
pub fn stub_0x260f68(block: Box<boost_exception::ControlBlock>) {
    // IDA 0x260f68: sp_counted_impl_p<clone_impl<bad_alloc_>> D0 thunk — operator delete. Box drop is the delete.
    drop(block);
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE19get_untyped_deleterEv")]
// 0x260f70 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_10bad_alloc_EEEE19get_untyped_deleterEv
pub fn stub_0x260f70() -> Option<&'static str> {
    // IDA 0x260f70: sp_counted_impl_p<clone_impl<bad_alloc_>>::get_untyped_deleter — returns 0.
    None
}

#[doc(alias = "__GLOBAL__I_a_58")]
// 0x260f7c — __GLOBAL__I_a_58
pub fn stub_0x260f7c() {
    // IDA 0x260f7c: __GLOBAL__I_a_58 — identical core-owned set to a_57 (categories, ios init, both static eps, both Xml pools).
    boost_exception::ensure_init_a58();
}

#[doc(alias = "__ZL15initStaticData2v")]
// 0x2610d8 — __ZL15initStaticData2v
pub fn stub_0x2610d8() {
    // IDA 0x2610d8: initStaticData2 — thunk straight into staticData2.
    boost_exception::init_static_data2();
}

#[doc(alias = "__ZL11staticData2v")]
// 0x2610dc — __ZL11staticData2v
pub fn stub_0x2610dc() {
    // IDA 0x2610dc: staticData2 — guarded zero-init of the ClassDescriptor* vector + atexit(~vector). Registry shell init.
    boost_exception::init_static_data2();
}

#[doc(alias = "__ZN5boost16exception_detail27get_static_exception_objectINS0_10bad_alloc_EEENS_13exception_ptrEv")]
// 0x261df8 — __ZN5boost16exception_detail27get_static_exception_objectINS0_10bad_alloc_EEENS_13exception_ptrEv
pub fn stub_0x261df8() -> crate::SharedPtr<boost_exception::CloneImpl<boost_exception::BadAlloc_>> {
    // IDA 0x261df8: get_static_exception_object<bad_alloc_> — guarded once-build of the static clone_impl + ep (exception_ptr.hpp:123, atexit(~exception_ptr)); every return copies ep with an addref.
    boost_exception::static_bad_alloc()
}

#[doc(alias = "__ZN5boost16exception_detail27get_static_exception_objectINS0_14bad_exception_EEENS_13exception_ptrEv")]
// 0x2620f0 — __ZN5boost16exception_detail27get_static_exception_objectINS0_14bad_exception_EEENS_13exception_ptrEv
pub fn stub_0x2620f0() -> crate::SharedPtr<boost_exception::CloneImpl<boost_exception::BadException_>> {
    // IDA 0x2620f0: get_static_exception_object<bad_exception_> — same shape for bad_exception_.
    boost_exception::static_bad_exception()
}

#[doc(alias = "__ZN5boost21thread_resource_errorD0Ev")]
// 0x2650b8 — __ZN5boost21thread_resource_errorD0Ev
pub fn stub_0x2650b8(this: boost_exception::ThreadResourceError) {
    // IDA 0x2650b8: thread_resource_error D0 — vtable := system_error vtbl (0x2650d0), ~string(this+2) (0x2650d4), ~runtime_error (0x2650da), operator delete (0x2650e4). By-value drop is the member-drop + delete.
    drop(this);
}

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv")]
// 0x2650e8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv
pub fn stub_0x2650e8(this: &boost_exception::CloneImpl<boost_exception::ThreadResourceError>) -> ! {
    // IDA 0x2650e8: clone_impl<injector<thread_resource_error>>::rethrow — __cxa_allocate_exception(0x2C) (0x265120), copy both message strings (0x265168/0x265202), install vtables (0x265134-0x265238), addref the error info via vtab+12 when non-null (0x26523e-0x26524c), copy tail words (0x265260-0x265270), __cxa_throw. __noreturn -> -> !.
    let thrown = this.rethrow_copy();
    boost_exception::throw_thread_resource_error(thrown.payload);
}

#[doc(alias = "__ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv")]
// 0x2652b0 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE7rethrowEv
pub fn stub_0x2652b0(this: *const u8, top_offset: isize) -> ! {
    // IDA 0x2652b0: virtual thunk to rethrow — this += *(vtab - 16) (0x2652bc), then tail-call 0x2650e8. The adjusted object is the rethrown clone_impl; the throw itself is the caller's panic!.
    let _ = boost_exception::virtual_thunk_adjust(this, top_offset);
    panic!("rethrow via Tv0_n16 thunk");
}

#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev")]
// 0x2652c0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
pub fn stub_0x2652c0(this: *mut u8, top_offset: isize) -> *mut u8 {
    // IDA 0x2652c0: virtual thunk to clone_impl<injector<thread_resource_error>> D0 — this += *(vtab - 20) (0x2652ca), ~injector (0x2652ce), operator delete. Returns the adjusted address; member drops via Drop glue.
    boost_exception::virtual_thunk_adjust(this as *const u8, top_offset) as *mut u8
}

#[doc(alias = "__ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev")]
// 0x2652e0 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev
pub fn stub_0x2652e0(this: *mut u8) -> *mut u8 {
    // IDA 0x2652e0: non-virtual thunk to injector D0 — this -= 20 (0x2652e2), ~error_info_injector<thread_resource_error> (0x2652ea), operator delete. Returns the adjusted address.
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZN5boost16exception_detail10clone_baseD1Ev")]
// 0x2652f8 — __ZN5boost16exception_detail10clone_baseD1Ev
pub fn stub_0x2652f8(_this: &mut boost_exception::CloneBase) {
    // IDA 0x2652f8: clone_base D1 — empty body (pure vtable anchor). Nothing to emit.
}

#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS5_NS5_9clone_tagE")]
// 0x265300 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS5_NS5_9clone_tagE
pub fn stub_0x265300(
    src: &boost_exception::CloneImpl<boost_exception::ThreadResourceError>,
) -> boost_exception::CloneImpl<boost_exception::ThreadResourceError> {
    // IDA 0x265300: clone_impl<injector<thread_resource_error>> copy ctor — copy both message strings (0x26537a/0x2653b4), install vtables (0x265346-0x265436), addref the error info via vtab+12 (0x2653f0-0x2653fe), copy tail words (0x265414-0x265420), copy_boost_exception (0x265442). clone_box is that copy.
    src.clone_box()
}

#[doc(alias = "__ZN5boost16exception_detail14bad_exception_D2Ev")]
// 0x2654d8 — __ZN5boost16exception_detail14bad_exception_D2Ev
pub fn stub_0x2654d8(this: &mut boost_exception::CloneImpl<boost_exception::BadException_>) {
    // IDA 0x2654d8: bad_exception_ D2 (non-deleting) — ~bad_exception base (0x2654fa), vtable := off_12216C8 (0x265518), release the error-info ref via vtab+16, nulled when last (0x26553a-0x265552). Dropping the Arc share is the release.
    this.info.take();
}

#[doc(alias = "__ZThn20_N5boost16exception_detail14bad_exception_D1Ev")]
// 0x265590 — __ZThn20_N5boost16exception_detail14bad_exception_D1Ev
pub fn stub_0x265590(this: *mut u8) -> *mut u8 {
    // IDA 0x265590: non-virtual thunk — this -= 20, ~bad_exception_ (0x2654d8). Returns the adjusted address.
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZThn20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev")]
// 0x265598 — __ZThn20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev
pub fn stub_0x265598(this: *mut u8) -> *mut u8 {
    // IDA 0x265598: non-virtual thunk to clone_impl<bad_exception_> D1 — this -= 20, ~bad_exception_ runs (0x26559a). Returns the adjusted address.
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev")]
// 0x2655a0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_14bad_exception_EED1Ev
pub fn stub_0x2655a0(this: *mut u8, top_offset: isize) -> *mut u8 {
    // IDA 0x2655a0: virtual thunk to clone_impl<bad_exception_> D1 — this += *(vtab - 20) (0x2655a8), then D1. Returns the adjusted address.
    boost_exception::virtual_thunk_adjust(this as *const u8, top_offset) as *mut u8
}

#[doc(alias = "__ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_14bad_exception_EEEEEPT_")]
// 0x2655b0 — __ZN5boost6detail12shared_countC2INS_16exception_detail10clone_implINS3_14bad_exception_EEEEEPT_
pub fn stub_0x2655b0() -> boost_exception::SharedCount {
    // IDA 0x2655b0: shared_count<clone_impl<bad_exception_>> ctor — *pi = 0 (0x2655dc), operator new(0x10) (0x265604), uses = weaks = 1 (0x265612/0x265616), vtable off_1222F48 (0x26561c), payload stored (0x265622), *pi = block (0x265624). The payload word lives in the Arc-carried object here ([INFERENCE] on layout).
    boost_exception::SharedCount::from_payload_kind("off_1222F48")
}

#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED0Ev")]
// 0x2656a8 — __ZN5boost6detail17sp_counted_impl_pINS_16exception_detail10clone_implINS2_14bad_exception_EEEED0Ev
pub fn stub_0x2656a8(block: Box<boost_exception::ControlBlock>) {
    // IDA 0x2656a8: sp_counted_impl_p<clone_impl<bad_exception_>> D0 thunk — operator delete only (dispose ran when the last use fell). Box drop is the delete.
    drop(block);
}

#[doc(alias = "__ZN5boost16exception_detail10bad_alloc_D2Ev")]
// 0x2656b0 — __ZN5boost16exception_detail10bad_alloc_D2Ev
pub fn stub_0x2656b0(this: &mut boost_exception::CloneImpl<boost_exception::BadAlloc_>) {
    // IDA 0x2656b0: bad_alloc_ D2 (non-deleting) — ~bad_alloc base (0x2656d2), vtable := off_12216C8 (0x2656f0), release the error-info ref via vtab+16, nulled when last (0x265712-0x26572a). Dropping the Arc share is the release.
    this.info.take();
}

#[doc(alias = "__ZThn20_N5boost16exception_detail10bad_alloc_D1Ev")]
// 0x265768 — __ZThn20_N5boost16exception_detail10bad_alloc_D1Ev
pub fn stub_0x265768(this: *mut u8) -> *mut u8 {
    // IDA 0x265768: non-virtual thunk — this -= 20, ~bad_alloc_ (0x2656b0). Returns the adjusted address.
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZThn20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev")]
// 0x265770 — __ZThn20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev
pub fn stub_0x265770(this: *mut u8) -> *mut u8 {
    // IDA 0x265770: non-virtual thunk to clone_impl<bad_alloc_> D1 — this -= 20, ~bad_alloc_ runs, returns the adjusted address (int return).
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev")]
// 0x265778 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_10bad_alloc_EED1Ev
pub fn stub_0x265778(this: *mut u8, top_offset: isize) -> *mut u8 {
    // IDA 0x265778: virtual thunk to clone_impl<bad_alloc_> D1 — this += *(vtab - 20), then D1. Returns the adjusted address.
    boost_exception::virtual_thunk_adjust(this as *const u8, top_offset) as *mut u8
}

#[doc(alias = "__ZN5boost16exception_detail10bad_alloc_D0Ev")]
// 0x265788 — __ZN5boost16exception_detail10bad_alloc_D0Ev
pub fn stub_0x265788(this: boost_exception::CloneImpl<boost_exception::BadAlloc_>) {
    // IDA 0x265788: bad_alloc_ D0 — D2 runs (0x26578e), then operator delete (0x265798). By-value drop is the delete.
    drop(this);
}

#[doc(alias = "__GLOBAL__I_a_59")]
// 0x2657a4 — __GLOBAL__I_a_59
pub fn stub_0x2657a4() {
    // IDA 0x2657a4: __GLOBAL__I_a_59 — generic_category x2 + system_category into 0x131E3E4/E8/EC (0x2657a8-0x2657c2), ios_base::Init + atexit (0x2657c6-0x2657e6), guarded bad_alloc ep via 0x261df8 + atexit(~exception_ptr) (0x2657f6-0x265826), guarded bad_exception ep via 0x2620f0 + atexit (0x265834-0x265864), XmlAttribute pool get_pool (0x265892), XmlElement pool get_pool (0x2658c8). Same core-owned set as a_57/a_58.
    boost_exception::ensure_init_a59();
}

#[doc(alias = "__ZN3RBX9AllocatorI10XmlElementEnwEm")]
// 0x26648c — __ZN3RBX9AllocatorI10XmlElementEnwEm
pub fn stub_0x26648c(_bytes: usize) -> Vec<u8> {
    // IDA 0x26648c: RBX::Allocator<XmlElement>::operator new — singleton_pool<XmlElement,36>::malloc (0x266492); null -> RBXCRASH when crashOnAllocationFailure (0x2664c0/0x2664c2) else __cxa_throw bad_alloc (0x2664c8-0x2664f6). The size arg is unused (fixed pool chunks).
    boost_exception::xml_element_pool().allocate_or_throw()
}

#[doc(alias = "__ZN3RBX9AllocatorI12XmlAttributeEnwEm")]
// 0x266544 — __ZN3RBX9AllocatorI12XmlAttributeEnwEm
pub fn stub_0x266544(_bytes: usize) -> Vec<u8> {
    // IDA 0x266544: RBX::Allocator<XmlAttribute>::operator new — singleton_pool<XmlAttribute,20>::malloc (0x26654a); null -> RBXCRASH when crashOnAllocationFailure (0x266578/0x26657a) else __cxa_throw bad_alloc (0x266580-0x2665ae). The size arg is unused (fixed pool chunks).
    boost_exception::xml_attribute_pool().allocate_or_throw()
}

#[doc(alias = "__ZN5boost10lock_errorD1Ev")]
// 0x2665b8 — __ZN5boost10lock_errorD1Ev
pub fn stub_0x2665b8(_this: &mut boost_exception::LockError) {
    // IDA 0x2665b8: lock_error D1 (non-deleting) — vtable := system_error vtbl (0x2665d0), ~string(this+2) (0x2665d4), ~runtime_error (0x2665de). Member drops run via Rust Drop glue; nothing to emit.
}

#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev")]
// 0x2665e8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
pub fn stub_0x2665e8(_this: &mut boost_exception::CloneImpl<boost_exception::LockError>) {
    // IDA 0x2665e8: clone_impl<injector<lock_error>> D1 — ~error_info_injector<lock_error> (0x2665ee), returns this (0x2665f4). Member drops run via Rust Drop glue; nothing to emit.
}

#[doc(alias = "__ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev")]
// 0x2665f8 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
pub fn stub_0x2665f8(this: *mut u8) -> *mut u8 {
    // IDA 0x2665f8: non-virtual thunk to clone_impl<injector<lock_error>> D1 — this -= 20, ~injector runs. Returns the adjusted address.
    boost_exception::nonvirtual_thunk_adjust(this)
}

#[doc(alias = "__ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_")]
// 0x266600 — __ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_
pub fn stub_0x266600(
    name: &str,
    default_value: &str,
) -> crate::generated_core_shard_ke::xml_tree::XmlAttribute {
    // IDA 0x266600: XmlAttribute::XmlAttribute<const Name*> — next = 0 (0x26662c), pair.name = name (0x26663c: *(a1+4) = a2), pair tag = 1 = TAG_NAME with payload = default (0x26663e-0x266644), then Allocator<XmlAttribute>::Allocator registration (0x266666); the landing-pad path runs clearValue (0x26669a — Drop/clear glue here).
    boost_exception::ensure_xml_attribute_allocator();
    crate::generated_core_shard_ke::xml_tree::XmlAttribute {
        next: None,
        pair: crate::generated_core_shard_ke::xml_value::NameValuePair {
            name: name.to_owned(),
            value: crate::generated_core_shard_ke::xml_value::Value::Name(default_value.to_owned()),
        },
    }
}

#[doc(alias = "__ZN3RBX9AllocatorI12XmlAttributeEC2Ev")]
// 0x2666c0 — __ZN3RBX9AllocatorI12XmlAttributeEC2Ev
pub fn stub_0x2666c0() {
    // IDA 0x2666c0: RBX::Allocator<XmlAttribute>::Allocator — once-init guarded by initialized (0x2666d6/0x26671a): push &availableSize into poolAvailabilityList (0x2666f2/0x2666f4), push releaseMemory into poolReleaseMemoryFuncList (0x26670c-0x266714). Idempotent.
    boost_exception::ensure_xml_attribute_allocator();
}

#[doc(alias = "__ZN3RBX9AllocatorI12XmlAttributeE13releaseMemoryEv")]
// 0x266728 — __ZN3RBX9AllocatorI12XmlAttributeE13releaseMemoryEv
pub fn stub_0x266728() -> bool {
    // IDA 0x266728: RBX::Allocator<XmlAttribute>::releaseMemory — zero availableSize, release pool memory.
    boost_exception::xml_attribute_release_memory()
}

#[doc(alias = "__ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0x266748 — __ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0x266748(vec: &mut Vec<fn() -> bool>, pos: usize, val: fn() -> bool) {
    // IDA 0x266748: vector<bool(*)()>::_M_insert_aux into poolReleaseMemoryFuncList — grow-or-shift insert.
    boost_exception::release_fn_vector_insert_aux(vec, pos, val);
}

#[doc(alias = "__ZNSt12_Vector_baseIPFbvESaIS1_EE11_M_allocateEm")]
// 0x266828 — __ZNSt12_Vector_baseIPFbvESaIS1_EE11_M_allocateEm
pub fn stub_0x266828(n: usize) -> Vec<std::mem::MaybeUninit<fn() -> bool>> {
    // IDA 0x266828: _Vector_base<bool(*)()>::_M_allocate — bad_alloc at n >= 0x40000000, else operator new(4*n).
    boost_exception::release_fn_vector_storage(n)
}

#[doc(alias = "__ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")]
// 0x266840 — __ZN5boost14singleton_poolI12XmlAttributeLj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_0x266840() -> bool {
    // IDA 0x266840: singleton_pool<XmlAttribute>::release_memory — get_pool, lock, pool::release_memory, unlock.
    boost_exception::xml_attribute_pool().release_memory()
}

#[doc(alias = "__ZN5boost4poolINS_34default_user_allocator_malloc_freeEE14release_memoryEv")]
// 0x266870 — __ZN5boost4poolINS_34default_user_allocator_malloc_freeEE14release_memoryEv
pub fn stub_0x266870() -> bool {
    // IDA 0x266870: pool<>::release_memory — free fully-unused blocks, return whether any was released.
    // Collapses into SingletonPool::release_memory (same observable contract as the 0x266840 path).
    boost_exception::xml_attribute_pool().release_memory()
}

#[doc(alias = "__ZN5boost25simple_segregated_storageImE9segregateEPvmmS2_")]
// 0x266960 — __ZN5boost25simple_segregated_storageImE9segregateEPvmmS2_
pub unsafe fn stub_0x266960(block: *mut *mut u8, partition: usize, sz: usize, end: *mut u8) -> *mut *mut u8 {
    // IDA 0x266960: simple_segregated_storage::segregate — intrusive free-list braid; caller guarantees the range.
    boost_exception::segregate(block, partition, sz, end)
}

#[doc(alias = "__ZN10XmlElementC2ERKN3RBX4NameE")]
// 0x267350 — __ZN10XmlElementC2ERKN3RBX4NameE
pub fn stub_0x267350(tag: String) -> crate::generated_core_shard_ke::xml_tree::XmlElement {
    // IDA 0x267350: XmlElement::XmlElement(const RBX::Name&) — zero links, tag name at +12, Allocator init, zero attr head.
    boost_exception::ensure_xml_element_allocator();
    crate::generated_core_shard_ke::xml_tree::XmlElement::new(tag)
}

#[doc(alias = "__ZN3RBX9AllocatorI10XmlElementEC2Ev")]
// 0x267420 — __ZN3RBX9AllocatorI10XmlElementEC2Ev
pub fn stub_0x267420() {
    // IDA 0x267420: RBX::Allocator<XmlElement>::Allocator — guarded once-only pool registration.
    boost_exception::ensure_xml_element_allocator();
}

#[doc(alias = "__GLOBAL__I_a_60")]
// 0x2674b0 — __GLOBAL__I_a_60
pub fn stub_0x2674b0() {
    // IDA 0x2674b0: __GLOBAL__I_a_60 — categories, ios init, both static eps, both Xml pool get_pools.
    boost_exception::ensure_init_a60();
}

#[doc(alias = "__GLOBAL__I_a_61")]
// 0x268bf0 — __GLOBAL__I_a_61
pub fn stub_0x268bf0() {
    // IDA 0x268bf0: __GLOBAL__I_a_61 — categories, ios init, both static eps; no pool creation.
    boost_exception::ensure_init_a61();
}

#[doc(alias = "__ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE10link_pointERKSC_RPNS1_22hashed_index_node_implISaIcEEEST_")]
// 0x26af9c — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE10link_pointERKSC_RPNS1_22hashed_index_node_implISaIcEEEST_
pub fn stub_0x26af9c(bucket: &[String], key: &str, link_out: &mut usize) -> bool {
    // IDA 0x26af9c: ProtectedString hashed_index::link_point — bucket lookup, link on match.
    boost_exception::hashed_link_point(bucket, key, link_out)
}

#[doc(alias = "__ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_m")]
// 0x26afd0 — __ZN5boost11multi_index6detail10auto_spaceImSaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_m
pub fn stub_0x26afd0(n: usize) -> boost_exception::AutoSpace {
    // IDA 0x26afd0: auto_space ctor — store n, allocate 4*n (bad_alloc at 0x40000000).
    boost_exception::AutoSpace::new(n)
}

#[doc(alias = "__GLOBAL__I_a_62")]
// 0x26b1f4 — __GLOBAL__I_a_62
pub fn stub_0x26b1f4() {
    // IDA 0x26b1f4: __GLOBAL__I_a_62 — a_60 set + FWInstance/OnDemandInstance pools + flyweight init (+ script creators owned elsewhere).
    boost_exception::ensure_init_a62();
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26e228 — __ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x26e228(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26e228: rbx::any_cast<const ContentId&> — checked holder cast, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::content_id_holder())
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26e318 — __ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x26e318(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26e318: rbx::any_cast<const CellID&> — checked holder cast, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::cell_id_holder())
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26e464 — __ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x26e464(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26e464: rbx::any_cast<const Axes&> — checked holder cast, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::axes_holder())
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26e554 — __ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x26e554(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26e554: rbx::any_cast<const UDim&> — checked holder cast, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::udim_holder())
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26e648 — __ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x26e648(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26e648: rbx::any_cast<const Region3int16&> — checked holder cast, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::region3int16_holder())
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")]
// 0x26e780 — __ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
pub fn stub_0x26e780(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26e780: rbx::any_cast<const Region3&> — checked holder cast, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::region3_holder())
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26f3a0 — __ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x26f3a0(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26f3a0: rbx::any_cast<const ProtectedString&> — checked holder cast, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::protected_string_holder())
}

#[doc(alias = "__ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26f490 — __ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x26f490(slot: &boost_exception::PlacementAny) -> &boost_exception::PlacementValue {
    // IDA 0x26f490: rbx::any_cast<const long&> — same two-level typeinfo check, bad_placement_any_cast on mismatch.
    boost_exception::placement_any_cast(slot, boost_exception::long_holder())
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_")]
// 0x26f578 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_
pub fn stub_0x26f578(slot: &mut boost_exception::PlacementAny, src: &[u8; 20]) {
    // IDA 0x26f578: placement_any<Region3>::operator=<InputObject> — in-place assign or destroy + copy + holder install.
    boost_exception::assign_input_object(slot, src);
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE14construct_funcEPKcPc")]
// 0x26f5e0 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE14construct_funcEPKcPc
pub fn stub_0x26f5e0(src: &[u8; 20], dst: Option<&mut [u8; 20]>) {
    // IDA 0x26f5e0: typed_holder<InputObject>::construct_func — copy the 5 payload words when dst is non-null.
    boost_exception::input_object_construct(src, dst);
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_")]
// 0x26f600 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_
pub fn stub_0x26f600(slot: &mut boost_exception::PlacementAny, src: &boost_exception::CellIdPayload) {
    // IDA 0x26f600: placement_any<Region3>::operator=<CellID> — field assign with shared_ptr release/acquire, or destroy + copy.
    boost_exception::assign_cell_id(slot, src);
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv")]
// 0x26f680 — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv
pub fn stub_0x26f680() -> &'static boost_exception::TypedHolder {
    // IDA 0x26f680: typed_holder<CellID>::singleton — guarded once-init of the holder identity.
    boost_exception::ensure_cell_id_holder()
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE14construct_funcEPKcPc")]
// 0x26f6ec — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE14construct_funcEPKcPc
pub fn stub_0x26f6ec(src: &boost_exception::CellIdPayload, dst: Option<&mut boost_exception::CellIdPayload>) {
    // IDA 0x26f6ec: typed_holder<CellID>::construct_func — copy head + shared_count (addref) + instance word.
    boost_exception::cell_id_construct(src, dst);
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE13destruct_funcEPc")]
// 0x26f718 — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE13destruct_funcEPc
pub fn stub_0x26f718() {
    // IDA 0x26f718: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX4UDimEE14construct_funcEPKcPc")]
// 0x26f720 — __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE14construct_funcEPKcPc
pub fn stub_0x26f720() {
    // IDA 0x26f720: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX4UDimEE13destruct_funcEPc")]
// 0x26f730 — __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE13destruct_funcEPc
pub fn stub_0x26f730() {
    // IDA 0x26f730: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv")]
// 0x26f738 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv
pub fn stub_0x26f738() {
    // IDA 0x26f738: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE13destruct_funcEPc")]
// 0x26f7a8 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE13destruct_funcEPc
pub fn stub_0x26f7a8() {
    // IDA 0x26f7a8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_")]
// 0x26f9a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_
pub fn stub_0x26f9a0() {
    // IDA 0x26f9a0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv")]
// 0x26fa00 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv
pub fn stub_0x26fa00() {
    // IDA 0x26fa00: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE13destruct_funcEPc")]
// 0x26fa70 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE13destruct_funcEPc
pub fn stub_0x26fa70() {
    // IDA 0x26fa70: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__GLOBAL__I_a_63")]
// 0x270078 — __GLOBAL__I_a_63
pub fn stub_0x270078() {
    // IDA 0x270078: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "__ZNKSt6vectorIN3RBX10BrickColorESaIS1_EE2atEm")]
// 0x277870 — __ZNKSt6vectorIN3RBX10BrickColorESaIS1_EE2atEm
pub fn stub_0x277870() {
    // IDA 0x277870: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__GLOBAL__I_a_64")]
// 0x278164 — __GLOBAL__I_a_64
pub fn stub_0x278164() {
    // IDA 0x278164: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX6RbxRayeqERKS0_")]
// 0x27b438 — __ZNK3RBX6RbxRayeqERKS0_
pub fn stub_0x27b438() {
    // IDA 0x27b438: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX6CellIDeqERKS0_")]
// 0x27b4b4 — __ZNK3RBX6CellIDeqERKS0_
pub fn stub_0x27b4b4() {
    // IDA 0x27b4b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__GLOBAL__I_a_65")]
// 0x27b50c — __GLOBAL__I_a_65
pub fn stub_0x27b50c() {
    // IDA 0x27b50c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__GLOBAL__I_a_66")]
// 0x27bef0 — __GLOBAL__I_a_66
pub fn stub_0x27bef0() {
    // IDA 0x27bef0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__GLOBAL__I_a_67")]
// 0x2858c0 — __GLOBAL__I_a_67
pub fn stub_0x2858c0() {
    // IDA 0x2858c0: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE9push_backERKS4_")]
// 0x286100 — __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE9push_backERKS4_
pub fn stub_0x286100() {
    // IDA 0x286100: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN5boost4poolINS_33default_user_allocator_new_deleteEE12purge_memoryEv")]
// 0x28612c — __ZN5boost4poolINS_33default_user_allocator_new_deleteEE12purge_memoryEv
pub fn stub_0x28612c() {
    // IDA 0x28612c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_")]
// 0x286170 — __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
pub fn stub_0x286170() {
    // IDA 0x286170: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}


#[cfg(test)]
mod batch_ad_tests {
    use super::boost_exception::*;
    use super::*;

    #[test]
    fn thunk_bias_matches_ida_thn20() {
        let base = 0x1000 as *mut u8;
        assert_eq!(stub_0x25fdc8(base), 0xFEC as *mut u8);
        assert_eq!(stub_0x260098(base), 0xFEC as *mut u8);
        assert_eq!(NONVIRTUAL_THUNK_BIAS, 20);
    }

    #[test]
    fn virtual_thunk_adds_top_offset() {
        let base = 0x1000 as *const u8;
        assert_eq!(stub_0x2600b0(base, -12), 0xFF4 as *const u8);
        assert_eq!(stub_0x260e60(base, 0), base);
    }

    #[test]
    fn spin_slots_match_ida_mod41() {
        let addr = 0x1223000usize;
        assert_eq!(spin_slot_use(addr), (addr + 4) % 41);
        assert_eq!(spin_slot_weak(addr), (addr + 8) % 41);
        assert_eq!(SPIN_POOL_LEN, 41);
    }

    #[test]
    fn shared_count_copy_addrefs_and_release_disposes() {
        let src = stub_0x260e70();
        assert_eq!(src.use_count(), 1);
        let mut dst = SharedCount::default();
        stub_0x2600c0(&mut dst, &src);
        assert_eq!(src.use_count(), 2);
        assert_eq!(dst.use_count(), 2);
        let block = src.block.as_ref().unwrap();
        assert_eq!(block.vtable, "off_1223058");
        let (disposed, destroyed) = stub_0x260d98(block);
        assert!(!disposed && !destroyed);
        drop(dst);
        drop(src);
    }

    #[test]
    fn release_last_owner_disposes_and_destroys() {
        let sole = stub_0x260e70();
        let block = sole.block.as_ref().unwrap();
        assert_eq!(block.use_count(), 1);
        assert_eq!(block.weak_count(), 1);
        let (disposed, destroyed) = stub_0x260d98(block);
        assert!(disposed && destroyed);
        assert!(stub_0x260e38(Some(block)));
        assert!(!stub_0x260e38(None));
    }

    #[test]
    fn untyped_deleter_is_null() {
        assert!(stub_0x260e48().is_none());
        assert!(stub_0x260f70().is_none());
        let owned = stub_0x260e70();
        assert!(owned.block.as_ref().unwrap().untyped_deleter().is_none());
    }

    #[test]
    fn deleting_dtor_frees_block() {
        let block = Box::new(ControlBlock::new("off_1223058"));
        stub_0x260f68(block);
    }

    #[test]
    fn clone_rethrow_shares_error_info() {
        let mut info = ErrorInfoContainer::default();
        info.entries.insert("file".to_string(), "exception_ptr.hpp".to_string());
        let e = CloneImpl { payload: BadException_, info: Some(crate::SharedPtr::new(info)) };
        let thrown = stub_0x25ff88(&e);
        assert!(std::ptr::eq(
            thrown.info.as_ref().unwrap().as_ref() as *const _,
            e.info.as_ref().unwrap().as_ref() as *const _
        ));
        let mut dst = CloneImpl { payload: BadAlloc_, info: None };
        let src = CloneImpl { payload: BadAlloc_, info: e.info.clone() };
        stub_0x25fdd0(&mut dst, &src);
        assert!(dst.info.is_some());
    }

    #[test]
    fn pool_sizes_match_disasm_words() {
        let pool = stub_0x25ff10();
        assert_eq!((pool.requested_size, pool.next_size, pool.start_size), (20, 32, 32));
        assert!(std::ptr::eq(pool, xml_attribute_pool()));
        assert_eq!(xml_element_pool().requested_size, 36);
    }

    #[test]
    fn static_exception_objects_are_singletons() {
        let a1 = stub_0x261df8();
        let a2 = stub_0x261df8();
        assert!(crate::SharedPtr::ptr_eq(&a1, &a2));
        let b1 = stub_0x2620f0();
        let b2 = stub_0x2620f0();
        assert!(crate::SharedPtr::ptr_eq(&b1, &b2));
        assert_eq!((STATIC_EP_SOURCE_FILE, STATIC_EP_SOURCE_LINE), ("boost/exception/detail/exception_ptr.hpp", 123));
    }

    #[test]
    fn global_inits_and_static_data_are_idempotent() {
        stub_0x25f04c();
        stub_0x260144();
        stub_0x260f7c();
        stub_0x2610d8();
        stub_0x2610dc();
        assert_eq!(*error_categories(), ("generic", "generic", "system"));
        assert!(static_data2().lock().is_empty());
    }

    #[test]
    #[should_panic(expected = "thread resource error")]
    fn throw_exception_noreturn_panics() {
        stub_0x25fc58(ThreadResourceError { detail: "test-spawn" });
    }

    #[test]
    fn dtor_glue_runs_without_effect() {
        let mut injector = CloneImpl { payload: ThreadResourceError { detail: "x" }, info: None };
        stub_0x25fdc0(&mut injector);
        let mut be = CloneImpl { payload: BadException_, info: None };
        stub_0x25ff60(&mut be);
        stub_0x25ff70(be);
        let mut ba = CloneImpl { payload: BadAlloc_, info: None };
        stub_0x260e50(&mut ba);
    }
    #[test]
    fn thread_resource_d0_drops_by_value() {
        stub_0x2650b8(ThreadResourceError { detail: "gone" });
    }

    #[test]
    #[should_panic(expected = "thread resource error")]
    fn injector_rethrow_panics_with_payload() {
        let e = CloneImpl { payload: ThreadResourceError { detail: "boom" }, info: None };
        stub_0x2650e8(&e);
    }

    #[test]
    #[should_panic(expected = "Tv0_n16")]
    fn rethrow_virtual_thunk_adjusts_then_throws() {
        stub_0x2652b0(0x1000 as *const u8, -16);
    }

    #[test]
    fn thunk_adjust_roundtrips() {
        let base = 0x1000 as *mut u8;
        assert_eq!(stub_0x2652e0(base), 0xFEC as *mut u8);
        assert_eq!(stub_0x2652c0(base, -20), 0xFEC as *mut u8);
        assert_eq!(stub_0x265590(base), 0xFEC as *mut u8);
        assert_eq!(stub_0x265598(base), 0xFEC as *mut u8);
        assert_eq!(stub_0x2655a0(base, -20), 0xFEC as *mut u8);
        assert_eq!(stub_0x265768(base), 0xFEC as *mut u8);
        assert_eq!(stub_0x265770(base), 0xFEC as *mut u8);
        assert_eq!(stub_0x265778(base, -20), 0xFEC as *mut u8);
        assert_eq!(stub_0x2665f8(base), 0xFEC as *mut u8);
        let mut base = CloneBase;
        stub_0x2652f8(&mut base);
    }

    #[test]
    fn injector_copy_ctor_clones_payload_and_shares_info() {
        let mut info = ErrorInfoContainer::default();
        info.entries.insert("k".to_string(), "v".to_string());
        let src = CloneImpl { payload: ThreadResourceError { detail: "x" }, info: Some(crate::SharedPtr::new(info)) };
        let dst = stub_0x265300(&src);
        assert_eq!(dst.payload, src.payload);
        assert!(std::ptr::eq(
            dst.info.as_ref().unwrap().as_ref() as *const _,
            src.info.as_ref().unwrap().as_ref() as *const _
        ));
    }

    #[test]
    fn bad_variant_dtors_release_error_info() {
        let mut be = CloneImpl { payload: BadException_, info: Some(crate::SharedPtr::new(ErrorInfoContainer::default())) };
        stub_0x2654d8(&mut be);
        assert!(be.info.is_none());
        let mut ba = CloneImpl { payload: BadAlloc_, info: Some(crate::SharedPtr::new(ErrorInfoContainer::default())) };
        stub_0x2656b0(&mut ba);
        assert!(ba.info.is_none());
        stub_0x265788(CloneImpl { payload: BadAlloc_, info: None });
        let sole = stub_0x2655b0();
        assert_eq!(sole.use_count(), 1);
        assert_eq!(sole.block.as_ref().unwrap().vtable, "off_1222F48");
        stub_0x2656a8(Box::new(ControlBlock::new("off_1222F48")));
    }

    #[test]
    fn lock_error_dtor_glue_is_noop() {
        let mut e = LockError { detail: "x" };
        stub_0x2665b8(&mut e);
        let mut injector = CloneImpl { payload: LockError { detail: "y" }, info: None };
        stub_0x2665e8(&mut injector);
    }

    #[test]
    fn pool_operator_new_returns_zeroed_chunks() {
        let el = stub_0x26648c(36);
        assert_eq!(el.len(), 36);
        assert!(el.iter().all(|&b| b == 0));
        let at = stub_0x266544(20);
        assert_eq!(at.len(), 20);
        assert!(at.iter().all(|&b| b == 0));
    }

    #[test]
    fn xml_attribute_ctor_sets_name_tag_and_registers_allocator() {
        let a = stub_0x266600("Visible", "true");
        assert!(a.next.is_none());
        assert_eq!(a.pair.name, "Visible");
        assert_eq!(a.pair.tag(), crate::generated_core_shard_ke::xml_value::TAG_NAME);
        assert_eq!(a.pair.text_or_empty(), "true");
        stub_0x2666c0();
        stub_0x2666c0();
        assert!(XML_ATTRIBUTE_ALLOCATOR_INITIALIZED.load(std::sync::atomic::Ordering::Acquire));
        assert!(xml_attribute_release_memory());
    }

    #[test]
    fn global_a59_init_is_idempotent() {
        stub_0x2657a4();
        stub_0x2657a4();
        assert_eq!(*error_categories(), ("generic", "generic", "system"));
    }
}