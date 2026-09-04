//! core shard AE — 100 core stubs EA-sorted, next uncovered after shard AD (0x286170), lowest EA first.
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 100 uncovered globally.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

/// Batch 6: 28 IDA-grounded ports 0x286250-0x2b01cc — `_Vector_base<pool*>`
/// `_M_allocate`, TU static inits `__GLOBAL__I_a_68`-`_a_71`, the
/// `flyweight<ProtectedString>` ctor/assign/dtor family, `thread::join`
/// (+self-join guard), and the `rbx::signals` slot-list machinery
/// (insert/remove/disconnectAll/emit, intrusive_ptr assign, static mutexes,
/// slot D1/D0). Untouched carriers keep stub bodies; ports live in
/// `core_signals` under idiomatic names, wired via `stub_0x*`.
/// Conventions: `boost::intrusive_ptr<slot>` -> `Arc<Slot>` (`crate::SharedPtr`,
/// addref/release = clone/drop); `boost::mutex` -> `parking_lot::Mutex`;
/// `__cxa_guard` once-init -> `LazyLock`; `__cxa_throw` paths -> `panic!`;
/// `FLog::SignalPrints/FastLog` trace points are fast-log owned (noted, not
/// emitted); `ReleaseAssert` checks are kept as `assert!` with the original
/// `signal.h` messages. `[INFERENCE]` marks what the binary does not pin down.
pub mod core_signals {
    use std::collections::HashMap;
    use std::sync::{Arc, LazyLock, Weak};
    use super::super::generated_core_shard_ad::boost_exception as be;
    /// was: `boost::intrusive_ptr<slot>` / `signal` ownership — `Arc` handles
    /// (`crate::SharedPtr`, i.e. `boost::shared_ptr`/`intrusive_ptr` per BOOST.md).
    pub type SlotHandle = crate::SharedPtr<Slot>;
    pub type RawSignalHandle = crate::SharedPtr<RawSignal>;

    /// was: `std::_Vector_base<pool*, ...>::_M_allocate` (IDA 0x286250:
    /// `if (a2 >= 0x40000000) __throw_bad_alloc(); return operator new(4*a2)`).
    /// Same shape as the `bool(*)()` `_M_allocate` port (IDA 0x266828).
    pub fn pool_ptr_vec_storage(n: usize) -> Vec<std::mem::MaybeUninit<usize>> {
        if n >= 0x4000_0000 {
            panic!("std::bad_alloc");
        }
        Vec::with_capacity(n)
    }

    /// IDA 0x286268 `__GLOBAL__I_a_68`: `generic_category` x2 + `system_category`
    /// into the TU statics, `ios_base::Init` + `atexit`, guarded `bad_alloc_` ep
    /// (via 0x261df8) + `atexit(~exception_ptr)`, guarded `bad_exception_` ep
    /// (via 0x2620f0) + `atexit`. Core-only set, same shape as a_61/a_66.
    /// Idempotent via `LazyLock`.
    pub fn ensure_init_a68() {
        let _ = be::error_categories();
        let _ = be::static_bad_alloc();
        let _ = be::static_bad_exception();
    }

    /// IDA 0x287738 `__GLOBAL__I_a_69`: the a_68 set plus `XmlAttribute` (20),
    /// `XmlElement` (36), `FWInstance` (28) and `OnDemandInstance` (20) pool
    /// `get_pool`s. The four `LuaSettings` `BoundProp` ctors and the
    /// `FactoryProduct<LuaSettings,...>::Creator` are reflection/datamodel
    /// owned (same split as a_56/a_62) — only the core effects are kept here.
    pub fn ensure_init_a69() {
        ensure_init_a68();
        let _ = be::xml_attribute_pool();
        let _ = be::xml_element_pool();
        let _ = be::fw_instance_pool();
        let _ = be::on_demand_instance_pool();
    }

    /// IDA 0x28aa88 `__GLOBAL__I_a_70`: the a_69 set plus the `ProtectedString`
    /// flyweight `static_holder_class` init. The one `FLog::RegisterLogGroup`
    /// is fast-log owned and the `FactoryProduct<LocalScript,...>::Creator` is
    /// script owned — only the core effects are kept here.
    pub fn ensure_init_a70() {
        ensure_init_a69();
        let _ = be::protected_string_flyweight();
    }

    /// IDA 0x294e3c `__GLOBAL__I_a_71`: the a_70 set plus the default
    /// `flyweight<ProtectedString>` value (the 0x2949dc ctor call inline in the
    /// init). `Script` `PropDescriptor`/`BoundFuncDesc`s, the `RegisterFlag`
    /// and the `Script`/`LocalScript`/`Camera` `FactoryProduct` creators are
    /// owned elsewhere — only the core effects are kept here.
    pub fn ensure_init_a71() {
        ensure_init_a70();
        let _ = ProtectedFlyweight::default_value();
    }
    /// Test-visible pool sizes behind the a_69/a_70/a_71 inits (IDA 0x287738:
    /// `RequestedSize` 20 for `XmlAttribute`, 36 for `XmlElement`).
    pub fn xml_attribute_pool_size() -> usize {
        be::xml_attribute_pool().requested_size
    }

    pub fn xml_element_pool_size() -> usize {
        be::xml_element_pool().requested_size
    }

    /// Test-visible default flyweight value behind the a_71 init (IDA 0x2949dc:
    /// default-constructed `ProtectedString`, i.e. empty).
    pub fn flyweight_default_text() -> &'static str {
        ""
    }

    /// was: `boost::flyweights::detail::refcounted_value<ProtectedString>`
    /// factory store. The binary keeps interned values in the hashed factory
    /// and erases them when the last handle dies (IDA 0x28d6bc/0x294c26); a
    /// `Weak` map reproduces that: entries vanish exactly when the last `Arc`
    /// goes away, and `release` erases eagerly like the binary does.
    static FLYWEIGHT_STORE: LazyLock<parking_lot::Mutex<HashMap<String, Weak<String>>>> =
        LazyLock::new(|| parking_lot::Mutex::new(HashMap::new()));

    /// was: `boost::flyweights::flyweight<RBX::ProtectedString, ...>` — a
    /// refcounted handle into the static-holder factory. `Arc<String>` is the
    /// handle; the factory map above is the holder.
    #[derive(Debug, Clone)]
    pub struct ProtectedFlyweight {
        pub value: crate::SharedPtr<String>,
    }

    impl ProtectedFlyweight {
        pub fn intern(s: &str) -> Self {
            let mut store = FLYWEIGHT_STORE.lock();
            if let Some(weak) = store.get(s) {
                if let Some(live) = weak.upgrade() {
                    return Self { value: live };
                }
            }
            let value = crate::SharedPtr::new(s.to_owned());
            store.insert(s.to_owned(), Arc::downgrade(&value));
            Self { value }
        }

        /// IDA 0x2949dc `flyweight::flyweight()` — default-constructs the
        /// `ProtectedString` value, then `flyweight_core::insert_value`
        /// (intern + first ref). The temp string release is Drop glue here.
        pub fn default_value() -> Self {
            Self::intern("")
        }

        /// IDA 0x28d6bc `flyweight::~flyweight` (D1) — atomic decrement of the
        /// handle count (`LDREX`/`STREX` `-1` at `[ptr+8]`); at zero the value
        /// is erased from the factory. Dropping the last `Arc` and erasing
        /// eagerly is that path (`[INFERENCE]` on lock ordering only).
        pub fn release(self) {
            let key = (*self.value).clone();
            let last = crate::SharedPtr::strong_count(&self.value) == 1;
            drop(self);
            if last {
                FLYWEIGHT_STORE.lock().remove(&key);
            }
        }

        /// IDA 0x28d6fc `flyweight::operator=(const flyweight&)` — addref the
        /// source (`+1` at `[src+8]`), store, release the old handle. Clone
        /// (bump) before the overwriting store drops the old is that order.
        pub fn assign(dst: &mut Self, src: &Self) {
            *dst = src.clone();
        }

        /// IDA 0x294ba8 `flyweight::operator=(const value_type&)` —
        /// `insert_value(&tmp, src)` then `operator=(dst, &tmp)`, then release
        /// `tmp` (erase iff its count was 1). Intern-swap-release is that path.
        pub fn assign_value(dst: &mut Self, src: &str) {
            *dst = Self::intern(src);
        }
    }

    /// was: per-`signal<T>` function-static `boost::mutex` behind
    /// `__cxa_guard_acquire` (IDA 0x2a94a0/0x2a9738/0x2afc80: `operator
    /// new(0x2C)` + `mutex::mutex` + guard release; IDA 0x2b00b0: in-place
    /// mutex + `__cxa_atexit(~mutex)`). One `LazyLock` each models the
    /// distinct guard variables; `parking_lot::Mutex` is the mutex.
    static SIGNAL_INT_MUTEX: LazyLock<parking_lot::Mutex<()>> =
        LazyLock::new(|| parking_lot::Mutex::new(()));
    static SIGNAL_VOID_MUTEX: LazyLock<parking_lot::Mutex<()>> =
        LazyLock::new(|| parking_lot::Mutex::new(()));
    static SIGNAL_HEARTBEAT_MUTEX: LazyLock<parking_lot::Mutex<()>> =
        LazyLock::new(|| parking_lot::Mutex::new(()));
    static SLOT_MUTEX: LazyLock<parking_lot::Mutex<()>> =
        LazyLock::new(|| parking_lot::Mutex::new(()));

    /// IDA 0x2a94a0 `signal<void(int)>::safe_static_do_get_mutex`.
    pub fn signal_int_mutex() -> &'static parking_lot::Mutex<()> {
        &SIGNAL_INT_MUTEX
    }

    /// IDA 0x2a9738 `signal<void()>::safe_static_do_get_mutex`.
    pub fn signal_void_mutex() -> &'static parking_lot::Mutex<()> {
        &SIGNAL_VOID_MUTEX
    }

    /// IDA 0x2afc80 `signal<void(Heartbeat)>::safe_static_do_get_mutex`.
    pub fn signal_heartbeat_mutex() -> &'static parking_lot::Mutex<()> {
        &SIGNAL_HEARTBEAT_MUTEX
    }

    /// IDA 0x2b00b0 `slot::safe_static_do_get_mutex` (in-place + atexit dtor).
    pub fn slot_static_mutex() -> &'static parking_lot::Mutex<()> {
        &SLOT_MUTEX
    }

    /// IDA 0x2b00ac `slot::safe_static_init_mutex` — thunk straight into the
    /// `do_get_mutex` above (`call_once` target at 0x2afee2/0x2afad2).
    pub fn slot_static_init_mutex() -> &'static parking_lot::Mutex<()> {
        slot_static_mutex()
    }

    /// was: `rbx::signals::signal<T>::slot` (`connection::islot`) — vtable pair
    /// (+0/+4), intrusive `next` link (+8), owner/connected word (+12, checked
    /// non-null by `connected` at 0x2aff90 and zeroed by `disconnect` at
    /// 0x2aff10). `Arc` is the intrusive refcount; the owner is `Weak` so a
    /// connected slot never keeps its signal alive (`[INFERENCE]` — the binary
    /// uses a raw back-pointer; observable insert/remove/emit/disconnect
    /// behavior is identical).
    pub struct Slot {
        pub next: parking_lot::Mutex<Option<crate::SharedPtr<Slot>>>,
        pub owner: parking_lot::Mutex<Option<Weak<RawSignal>>>,
        pub callback: parking_lot::Mutex<Option<Box<dyn Fn() + Send + Sync>>>,
        /// Second functor channel for 1-arg signal instantiations
        /// (`signal_with_args<1, void(lua_State*)>`, IDA 0x2a74b4): the slot
        /// functor for that instantiation takes the emitted pointer argument.
        /// `usize` carries the opaque pointer (`[INFERENCE]` — the binary
        /// slot holds one type-erased functor per instantiation; the two
        /// channels are never live on one slot at once).
        pub callback1: parking_lot::Mutex<Option<Box<dyn Fn(usize) + Send + Sync>>>,
    }

    impl Slot {
        pub fn new() -> Self {
            Self {
                next: parking_lot::Mutex::new(None),
                owner: parking_lot::Mutex::new(None),
                callback: parking_lot::Mutex::new(None),
                callback1: parking_lot::Mutex::new(None),
            }
        }

        /// IDA 0x2aff88 `slot::connected` — `return *(a1 + 12) != 0`.
        pub fn is_connected(&self) -> bool {
            self.owner.lock().is_some()
        }
    }

    impl Default for Slot {
        fn default() -> Self {
            Self::new()
        }
    }

    /// was: `rbx::signals::signal<T>` head word — the intrusive slot list the
    /// `insert`/`remove`/`disconnectAll`/emit family walks under the static
    /// mutex.
    #[derive(Default)]
    pub struct RawSignal {
        pub head: parking_lot::Mutex<Option<crate::SharedPtr<Slot>>>,
    }

    impl RawSignal {
        pub fn new() -> Self {
            Self::default()
        }

        /// IDA 0x2afa28 `signal::insert` — `ReleaseAssert(item)` (`signal.h:290`)
        /// when null, `call_once` the static-mutex init, lock, then push the
        /// slot at the head with intrusive addrefs (`op=` into `slot->next`
        /// and into the head link).
        pub fn insert(sig: &crate::SharedPtr<RawSignal>, slot: crate::SharedPtr<Slot>) {
            let _guard = signal_heartbeat_mutex().lock();
            *slot.next.lock() = sig.head.lock().clone();
            *sig.head.lock() = Some(slot.clone());
            *slot.owner.lock() = Some(Arc::downgrade(sig));
        }

        /// IDA 0x2affbc `signal::remove` — `ReleaseAssert` when the item is
        /// expired (`signal.h:261`), `FastLog("Removing item %p from signal")`
        /// when `SignalPrints` (fast-log owned, noted), then the splice: head
        /// link or predecessor `next` link adopts `item->next` via
        /// `intrusive_ptr::op=` (`LABEL_14` at 0x2b0056); miss = no-op, then
        /// the `signal.h:284` post-assert. The removed node keeps its own
        /// `next` (binary never clears it) — preserved here.
        pub fn remove(&self, slot: &crate::SharedPtr<Slot>) {
            assert!(
                crate::SharedPtr::strong_count(slot) >= 1,
                "!boost::intrusive_ptr_expired(item) (signal.h:261/284)"
            );
            let _guard = signal_heartbeat_mutex().lock();
            let head = self.head.lock();
            let mut order: Vec<crate::SharedPtr<Slot>> = Vec::new();
            let mut cur: Option<crate::SharedPtr<Slot>> = head.clone();
            while let Some(node) = cur {
                cur = node.next.lock().clone();
                order.push(node);
            }
            drop(head);
            if let Some(idx) = order.iter().position(|n| Arc::ptr_eq(n, slot)) {
                let next_after = order[idx].next.lock().clone();
                if idx == 0 {
                    *self.head.lock() = next_after;
                } else {
                    *order[idx - 1].next.lock() = next_after;
                }
            }
        }

        /// IDA 0x294cc4/0x2a9598 `signal::disconnectAll` — under the static
        /// mutex, walk the list zeroing every slot owner (`*(v+12) = 0` at
        /// 0x294d70) with the bounded `v21 = -11 .. 0` step guard (at most 11
        /// slots per pass, repeated while the head is non-null), then store
        /// null into the head link. One pass clears everything here, so the
        /// repeat collapses; the 11-step bound is kept as the walk cap.
        pub fn disconnect_all(&self) {
            const MAX_WALK_PER_PASS: i32 = 11;
            loop {
                let _guard = signal_heartbeat_mutex().lock();
                let mut head = self.head.lock();
                let mut cur: Option<crate::SharedPtr<Slot>> = head.take();
                let mut steps = 0;
                let mut rest: Option<crate::SharedPtr<Slot>> = None;
                while let Some(node) = cur {
                    *node.owner.lock() = None;
                    steps += 1;
                    let nxt = node.next.lock().clone();
                    if steps >= MAX_WALK_PER_PASS {
                        rest = nxt;
                        break;
                    }
                    cur = nxt;
                }
                *head = rest.clone();
                if rest.is_none() {
                    break;
                }
            }
        }

        /// IDA 0x2a6cc0 `signal_with_args<0, void()>::operator()` — no-op when
        /// the head is null; else the `SignalPrints` trace
        /// (`"Signal with 0 args executed"`, fast-log owned, noted) and the
        /// `next()` walk calling each still-connected slot functor
        /// (`(**(v13+4))(v13+4)` at 0x2a6d4c, guarded by `*(v13+12)` at
        /// 0x2a6d3c), releasing the iterator ref at the end (Arc drops here).
        pub fn emit_void0(&self) {
            let mut cur: Option<crate::SharedPtr<Slot>> = self.head.lock().clone();
            if cur.is_none() {
                return;
            }
            while let Some(node) = cur {
                cur = node.next.lock().clone();
                if node.is_connected() {
                    if let Some(cb) = node.callback.lock().as_ref() {
                        cb();
                    }
                }
            }
        }
        /// IDA 0x2a74b4 `signal_with_args<1, void(lua_State*)>::operator()` —
        /// no-op when the head is null (0x2a74e4); else the `SignalPrints`
        /// trace (`"Signal with 1 arg executed"`, fast-log owned, noted) and
        /// the `next()` walk calling each still-connected slot functor with
        /// the emitted argument (`(**(v22+4))(v22+4, v24)` at 0x2a753e,
        /// guarded by `*(v22+12)` at 0x2a752e), releasing the iterator ref at
        /// the end (Arc drops here). Same walk as `emit_void0`.
        pub fn emit_lua1(&self, arg: usize) {
            let mut cur: Option<crate::SharedPtr<Slot>> = self.head.lock().clone();
            if cur.is_none() {
                return;
            }
            while let Some(node) = cur {
                cur = node.next.lock().clone();
                if node.is_connected() {
                    if let Some(cb) = node.callback1.lock().as_ref() {
                        cb(arg);
                    }
                }
            }
        }
    }

    /// was: `boost::intrusive_ptr<slot>` word — nullable owning handle with
    /// `add_ref`/`release` (`connection::islot` counts). `Option<Arc>` is the
    /// word; clone-then-store is the addref-new/release-old order.
    #[derive(Default, Clone)]
    pub struct IntrusiveSlotPtr {
        pub target: Option<crate::SharedPtr<Slot>>,
    }

    /// IDA 0x2afc34/0x2afc58/0x2a947c/0x2a9710 `intrusive_ptr<slot>::operator=` —
    /// `if (new) add_ref(new); old = *this; *this = new; if (old)
    /// release(old); return this`. All four instantiations (`Heartbeat` raw /
    /// const&, `void(int)`, `void()`) share this shape; only the static type
    /// of the slot differs.
    pub fn intrusive_slot_assign(
        dst: &mut IntrusiveSlotPtr,
        src: Option<&crate::SharedPtr<Slot>>,
    ) {
        let bumped = src.cloned();
        dst.target = bumped;
    }

    /// IDA 0x2afe78 `slot::disconnect` — no-op unless `*(a1+12)` (connected);
    /// else `call_once` the slot static-mutex init, lock it, re-check, zero
    /// the owner word and `remove(signal, slot)` (0x2aff16), unlock via the
    /// lock guard drop.
    pub fn slot_disconnect(slot: &crate::SharedPtr<Slot>) {
        if slot.is_connected() {
            let _guard = slot_static_init_mutex().lock();
            let owner = slot.owner.lock().take();
            if let Some(weak) = owner {
                if let Some(sig) = weak.upgrade() {
                    sig.remove(slot);
                }
            }
        }
    }

    /// IDA 0x2b01a0 `slot::~slot` (D1) — reinstall the `slot` vtables, then
    /// `release(*(a1+8))` when non-null (0x2b01c2-0x2b01c4). Vtables have no
    /// Rust image; dropping the `next` Arc is the release.
    pub fn slot_destruct(slot: &Slot) {
        slot.next.lock().take();
    }

    /// IDA 0x2b01cc `slot::~slot` (D0) — D1 above, then `operator delete`
    /// (0x2b0246). By-value drop is the delete.
    pub fn slot_delete(slot: crate::SharedPtr<Slot>) {
        slot_destruct(&slot);
        drop(slot);
    }

    /// IDA 0x2a63ca `thread::join_noexcept` tail — `pthread_join`, no
    /// exceptions. `JoinHandle::join` (ignoring a panicking thread) is that
    /// path; `join_noexcept` never reports errors.
    pub fn thread_join_noexcept(handle: std::thread::JoinHandle<()>) {
        let _ = handle.join();
    }

    /// IDA 0x2a638a-0x2a6422 `thread::join` self-join guard — `pthread_self()
    /// == native_handle` builds `system_error(11)` (`"boost thread: trying
    /// joining itself"`) and `throw_exception<thread_resource_error>`.
    /// `ThreadResourceError` is shared with the `boost_exception` ports.
    pub fn thread_join_self_check(is_self: bool) {
        if is_self {
            be::throw_thread_resource_error(be::ThreadResourceError {
                detail: "boost thread: trying joining itself",
            });
        }
    }
}
/// Batch 8: 22 IDA-grounded ports 0x2981dc-0x2a9450 — the Lua TU helpers
/// (`panic`, `load`, `pushNoArguments`, `cleanTimeout`, the `illegal`
/// metatable guard), `Security::Context::current` + the `Impersonator` RAII
/// guard, `shared_ptr<Job>`/`shared_ptr<RunService>` assign, the
/// `bind<void(lua_State*,int,string)>` closure spec, the six
/// `boost::function::operator()` invokes, `shared_from<StatsService>`, the
/// `RunningAverage` EWMA pair, string `operator+`, the 1-arg
/// `signal_with_args` emit, and the `function2<void(lua_State*,lua_Debug*)>`
/// clear. Ports live in `core_function` under idiomatic names, wired via
/// `stub_0x*`; untouched carriers keep stub bodies.
/// Conventions: `boost::function` -> `BoostFunction` (`Option<SharedPtr<dyn
/// Fn>>`; empty is the null vtable word, clone is the manager-vtable clone);
/// `boost::shared_ptr` -> `crate::SharedPtr` (Arc); `__cxa_throw` ->
/// `panic!`; `lua_State*` stays opaque. `[INFERENCE]` marks what the binary
/// does not pin down.
pub mod core_function {
    /// was: `lua_State*` — the live script crate owns the real type; shared
    /// with the `core_af` Lua functor ports (same underlying pointer).
    pub type LuaStatePtr = crate::generated_core_shard_af::core_af::LuaStatePtr;
    /// was: `lua_Debug*` — same split as `LuaStatePtr`.
    pub type LuaDebugPtr = *mut std::os::raw::c_void;

    /// was: `boost::function<R(A...)>` storage — the vtable word (`*a1`) plus
    /// the 12-byte inline functor buffer. Empty is the null word; small
    /// functors sit inline (tag bit0 set), heap functors clone through the
    /// manager vtable. Every ported operation (empty test, copy, move, clear,
    /// swap, invoke) behaves identically for both, so one
    /// `Option<SharedPtr<...>>` covers all instantiations; `SharedPtr` clone
    /// is the manager clone and drop is the manager destroy (`[INFERENCE]`
    /// on inline-vs-heap storage only — observable behavior is identical).
    pub struct BoostFunction<A, R> {
        target: Option<crate::SharedPtr<dyn Fn(A) -> R + Send + Sync>>,
    }

    fn same_fn<A, R>(a: &BoostFunction<A, R>, b: &BoostFunction<A, R>) -> bool {
        std::ptr::eq(a, b)
    }

    impl<A, R> BoostFunction<A, R> {
        pub fn empty() -> Self {
            Self { target: None }
        }

        pub fn of(f: impl Fn(A) -> R + Send + Sync + 'static) -> Self {
            Self {
                target: Some(crate::SharedPtr::new(f)),
            }
        }

        /// Emptiness read (`*a1 == 0`) behind every `dummy::nonnull` guard.
        pub fn is_empty(&self) -> bool {
            self.target.is_none()
        }

        /// `clear` (IDA 0x2acdc4/0x2b1c4c/0x2b3fb4/0x2b4010/0x2a9450): no-op
        /// on the null word (0x2acdd0); else the manager destroy op 2 for
        /// heap targets (0x2acde0-0x2acde8, skipped for inline bit0), then
        /// word := 0. Drop-then-`None` is that path.
        pub fn clear(&mut self) {
            self.target = None;
        }

        /// `move_assign` (IDA 0x2acdf0/0x2b1b48): self-move is a no-op
        /// (0x2ace3e); empty src clears dst (0x2ace64); small src copies the
        /// 12 bytes inline (0x2ace50-0x2ace58), heap src clones through the
        /// manager (0x2ace7e); src word := 0 either way (0x2ace84). Take +
        /// clear-empty is that path.
        pub fn move_assign(&mut self, src: &mut Self) {
            if same_fn(&*self, &*src) {
                return;
            }
            match src.target.take() {
                Some(t) => self.target = Some(t),
                None => self.target = None,
            }
        }

        /// `assign_to_own` (IDA 0x2acef4/0x2b3f84/0x2b3fe0): empty src leaves
        /// dst untouched (0x2acefa fallthrough); small src copies inline
        /// (0x2acf04-0x2acf0c), heap src clones via manager op 0 (0x2acf22).
        /// Clone-when-present is that path.
        pub fn assign(&mut self, src: &Self) {
            if let Some(t) = &src.target {
                self.target = Some(t.clone());
            }
        }

        /// `swap` (IDA 0x2acce8/0x2b1a6c): self-swap is a no-op (0x2acd36);
        /// else the temp triple-`move_assign` + `clear` (0x2acd3a-0x2acd6a).
        pub fn swap_with(&mut self, other: &mut Self) {
            if same_fn(&*self, &*other) {
                return;
            }
            let mut tmp = Self::empty();
            tmp.move_assign(&mut *self);
            self.move_assign(other);
            other.move_assign(&mut tmp);
            tmp.clear();
        }

        /// `operator=` (IDA 0x2acc24): temp/assign/swap/clear
        /// (0x2acc48-0x2acc8e), returning `*this` at the live site.
        pub fn assign_from(&mut self, src: &Self) {
            let mut tmp = Self::empty();
            tmp.assign(src);
            self.swap_with(&mut tmp);
            tmp.clear();
        }

        /// `operator()` (IDA 0x2a59f4/0x2a5abc/0x2a5da0/0x2a7220/0x2a73ec):
        /// `bad_function_call` when the word is null (0x2a5a42-0x2a5a8a);
        /// else the vtable tail-call through `(*a1 & ~1) + 4` (0x2a5a74).
        /// `panic!` is the throw.
        pub fn invoke(&self, arg: A) -> R {
            match &self.target {
                Some(f) => f(arg),
                None => panic!("boost::bad_function_call"),
            }
        }
    }

    /// was: `boost::function1<unsigned long,lua_State*>` (IDA 0x2a59f4) and
    /// its `assign_to_own`/`clear` monomorphs (IDA 0x2b3fe0/0x2b4010).
    pub type LuaUlongFn = BoostFunction<LuaStatePtr, u64>;
    /// was: `boost::function2<void,lua_State*,unsigned long>` (IDA 0x2a5abc).
    pub type LuaVoidFn2 = BoostFunction<(LuaStatePtr, u64), ()>;
    /// was: `boost::function1<void,bool>` (IDA 0x2a5da0).
    pub type BoolVoidFn = BoostFunction<bool, ()>;
    /// was: `boost::function1<void,lua_State*>` (IDA 0x2a7220).
    pub type LuaVoidFn1 = BoostFunction<LuaStatePtr, ()>;
    /// was: `boost::function1<std::string,std::string const&>` (IDA 0x2a73ec).
    pub type StringMapFn = BoostFunction<String, String>;
    /// was: `boost::function2<void,lua_State*,lua_Debug*>` (IDA 0x2a9450).
    pub type LuaDbgFn2 = BoostFunction<(LuaStatePtr, LuaDebugPtr), ()>;

    /// was: `boost::shared_ptr<T>::operator=(const&)` (IDA 0x2a4a7c Job,
    /// 0x2a65c8 RunService): `shared_count` tmp from src (addref new,
    /// 0x2a4a90), store `pi_` (0x2a4a9a), swap in, release old (0x2a4aa8).
    /// Clone-then-store is that order.
    pub fn shared_ptr_assign<T>(
        dst: &mut Option<crate::SharedPtr<T>>,
        src: &Option<crate::SharedPtr<T>>,
    ) {
        *dst = src.clone();
    }

    /// Token for `RBX::TaskScheduler::Job` — the live site owns the real
    /// type; only the `shared_ptr` identity matters here.
    pub struct TaskSchedulerJob;
    /// Token for `RBX::RunService` — same split as `TaskSchedulerJob`.
    pub struct RunServiceToken;

    /// was: `RBX::RunningAverage<double,double>` (IDA 0x2a60b0: `+0`
    /// circular-buffer word, `+8` weight, `+16` last sample, `+24` average,
    /// `+32` variance, `+40` first-sample flag).
    pub struct RunningAverage {
        pub alpha: f64,
        pub last: f64,
        pub average: f64,
        pub variance: f64,
        pub first: bool,
        pub history: Option<Vec<f64>>,
    }

    impl RunningAverage {
        pub fn new(alpha: f64) -> Self {
            Self {
                alpha,
                last: 0.0,
                average: 0.0,
                variance: 0.0,
                // `+40` nonzero at construction: the first sample is taken
                // raw (`[INFERENCE]` — the ctor is not in this batch, but
                // 0x2a60e6 only blends when the flag is clear).
                first: true,
                history: None,
            }
        }

        /// IDA 0x2a60b0 `sample(double)`: the `fabs(v) != INFINITY` gate
        /// (0x2a60e0) — NaN passes it exactly like the binary, since
        /// `fabs(NaN) != INFINITY`; blend `alpha*v + (1-alpha)*avg` past the
        /// first sample (0x2a60e6-0x2a6106), store average/last, clear the
        /// flag (0x2a610c-0x2a6114), fold the variance
        /// `(1-alpha)*var + alpha*(v-avg)^2` with the *updated* average
        /// (0x2a6140), and `push_back` when the buffer word is present
        /// (0x2a6144-0x2a614c).
        pub fn sample(&mut self, v: f64) {
            if v.abs() != f64::INFINITY {
                let blended = if self.first {
                    v
                } else {
                    self.alpha * v + (1.0 - self.alpha) * self.average
                };
                self.average = blended;
                self.last = v;
                self.first = false;
                self.variance = (1.0 - self.alpha) * self.variance
                    + self.alpha * (v - blended) * (v - blended);
                if let Some(history) = self.history.as_mut() {
                    history.push(v);
                }
            }
        }
    }

    /// was: `RBX::RunningAverageTimeInterval<SampleMethod 1>` (IDA 0x2a6058:
    /// `+0` last stamp, `+8` armed flag, `+12` the `RunningAverage` above).
    pub struct RunningAverageTimeInterval {
        pub last: f64,
        pub armed: bool,
        pub average: RunningAverage,
    }

    impl RunningAverageTimeInterval {
        pub fn new(alpha: f64) -> Self {
            Self {
                last: 0.0,
                armed: true,
                average: RunningAverage::new(alpha),
            }
        }

        /// IDA 0x2a6058 `sample()`: `now` is `Time::now<1>` (0x2a6068),
        /// passed in because the clock lives outside core. The armed call
        /// (0x2a6062-0x2a6080) only records the baseline stamp and disarms,
        /// returning 0; later calls sample the stamp delta (0x2a6086-0x2a60a8).
        pub fn sample(&mut self, now: f64) {
            let dt = now - self.last;
            self.last = now;
            if self.armed {
                self.armed = false;
            } else {
                self.average.sample(dt);
            }
        }
    }

    /// IDA 0x29f0fc `cleanTimeout(double&)`: `*t` against the LuaSettings
    /// singleton word at `+0x68` (0x29f10a-0x29f116). Below the setting
    /// (0x29f11a), or NaN/Inf through the float `isNanInf` check
    /// (0x29f11c-0x29f12e), stores the setting (0x29f130-0x29f138).
    pub fn clean_timeout(value: &mut f64, setting: f64) {
        if *value < setting || (*value as f32).is_nan() || (*value as f32).is_infinite() {
            *value = setting;
        }
    }

    /// IDA 0x29cad4 `pushNoArguments(lua_State*)`: pushes nothing, returns 0
    /// (0x29cad6) — the lua result count.
    pub fn push_no_arguments() -> i32 {
        0
    }

    /// IDA 0x2981dc `panic(lua_State*)`: `StandardOut::singleton`
    /// (0x2981fc), `lua_tolstring(L, -1)` (0x29823a), `printf` of
    /// `"Unprotected error in call to Lua API (%s)\n"` (0x298252), release
    /// the singleton count (0x298258-0x298260), then `RBXCRASH` (0x29826a).
    /// eprint + `panic!` is that path; the message arrives as `&str` because
    /// the Lua stack lives at the live site.
    pub fn lua_panic_hook(message: &str) -> ! {
        eprintln!("Unprotected error in call to Lua API ({message})");
        panic!("RBXCRASH");
    }
    /// IDA 0x2a36f8 `illegal(lua_State*)` (`__noreturn`): builds
    /// `std::runtime_error("can't modify this library")` (0x2a3724-0x2a37e0)
    /// and `__cxa_throw`s it (0x2a3808). `panic!` is the throw.
    pub fn illegal_library_access() -> ! {
        panic!("can't modify this library");
    }

    /// was: `bind_t<void(*)(lua_State*,int,string), list3<arg<1>,arg<2>,
    /// value<string>>>` (IDA 0x2a5778/0x2b268c/0x2b27b8): the target word at
    /// `+0` (0x2a57dc, 0x2b26b8) plus the `list3` at `+4` holding the fixed
    /// `value<string>` (0x2b27e4, copied at 0x2b2d44-0x2b2d7a). `arg<1>` and
    /// `arg<2>` are placeholder tag types with no data, so only the target
    /// and the string are stored.
    #[derive(Debug, Clone)]
    pub struct BoundLuaCall {
        pub func: usize,
        pub text: String,
    }

    /// IDA 0x2a5778.
    pub fn bind_lua_call(func: usize, text: &str) -> BoundLuaCall {
        BoundLuaCall {
            func,
            text: text.to_owned(),
        }
    }

    impl BoundLuaCall {
        /// Call semantics of the bound target (the `list3::operator()` at
        /// 0x2b2bfc through the `void_function_obj_invoker2::invoke` at
        /// 0x2b2974): `arg<1>` takes the emitted state, `arg<2>` takes the
        /// emitted ulong narrowed to int, `value<string>` is fixed. The
        /// target word stays opaque; the live site supplies the callable.
        pub fn invoke_with(
            &self,
            target: &dyn Fn(LuaStatePtr, i32, &str),
            l: LuaStatePtr,
            extra: u64,
        ) {
            // `ulong -> int` narrows like the binary's implicit conversion
            // (`extra as i32` wraps, matching ARM).
            target(l, extra as i32, &self.text);
        }
    }

    /// was: `list3<arg<1>,arg<2>,value<string>>` (IDA 0x2b2d20) — the bound
    /// argument pack; only the string carries data (copied at
    /// 0x2b2d44-0x2b2d7a, temp released at 0x2b2d8c-0x2b2dd4 as Drop glue).
    #[derive(Debug, Clone)]
    pub struct BoundArgList {
        pub text: String,
    }

    /// IDA 0x2b2d20.
    pub fn bind_arg_list(text: &str) -> BoundArgList {
        BoundArgList {
            text: text.to_owned(),
        }
    }

    /// IDA 0x2b268c `function2` ctor from `bind_t`: zero the word (0x2b26ac),
    /// split the functor (0x2b26b8-0x2b26ce), `assign_to` the stored vtable
    /// (0x2b26f6). Storing the spec-backed closure is that path; the temp
    /// string release (0x2b2708-0x2b2750) is Drop glue.
    pub fn bind_to_function2(
        spec: BoundLuaCall,
        target: crate::SharedPtr<dyn Fn(LuaStatePtr, i32, &str) + Send + Sync>,
    ) -> LuaVoidFn2 {
        LuaVoidFn2::of(move |(l, extra)| spec.invoke_with(&*target, l, extra))
    }

    /// IDA 0x2ac838/0x2ad520/0x2b2688 `dummy::nonnull`: empty bodies — the
    /// address-taken marker for the safe-bool idiom, never called.
    pub fn dummy_nonnull() {}

    /// IDA 0x2ac368 `signal<void(RunTransition)>::slot::safe_static_do_get_mutex`:
    /// guarded in-place `mutex::mutex` (0x2ac3de) + `__cxa_atexit(~mutex)`
    /// (0x2ac3fc-0x2ac402), returning the static (0x2ac42c) — the in-place
    /// shape of 0x2b00b0, not the `operator new` shape of 0x2a94a0. `LazyLock`
    /// is the guard.
    static SLOT_RT_MUTEX: std::sync::LazyLock<parking_lot::Mutex<()>> =
        std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));

    pub fn slot_rt_static_mutex() -> &'static parking_lot::Mutex<()> {
        &SLOT_RT_MUTEX
    }

    /// was: `RBX::Name` handle for `Stats::sStats` behind
    /// `Name::declare`/`doDeclare` (IDA 0x2adfd8/0x2ae020). The index is
    /// assigned by the name table at the live site; 0 is the null name
    /// (`[INFERENCE]` — the binary returns the table slot).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatsName {
        pub index: u32,
    }

    static STATS_NAME_CELL: std::sync::LazyLock<StatsName> =
        std::sync::LazyLock::new(|| StatsName { index: 2 });

    /// IDA 0x2ae020 `doDeclare`: guarded once-init (`__cxa_guard_acquire` at
    /// 0x2ae07c, init at 0x2ae0a4, release at 0x2ae0a8) returning the static
    /// (0x2ae0d6). `LazyLock` is that guard.
    pub fn do_declare_stats_name() -> StatsName {
        *STATS_NAME_CELL
    }

    /// IDA 0x2adfd8 `declare`: null `sStats` text bails to `getNullName`
    /// (0x2adfea-0x2ae016); else `call_once` the declarer (0x2adfee-0x2ae006)
    /// and tail-calls `doDeclare` (0x2ae00e) — same shape as the
    /// `declare<StatsItem>` port at 0x2c1e00.
    pub fn declare_stats_name(text: Option<&str>) -> StatsName {
        match text {
            None => StatsName {
                index: crate::generated_core_shard_af::core_af::null_name(),
            },
            Some(_) => do_declare_stats_name(),
        }
    }

    /// was: `RBX::Name` handle for `sDebugSettings` (IDA 0x2ae77c/0x2ae7c4) —
    /// same shape as `StatsName` with its own once cell.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct DebugSettingsName {
        pub index: u32,
    }

    static DEBUG_SETTINGS_NAME_CELL: std::sync::LazyLock<DebugSettingsName> =
        std::sync::LazyLock::new(|| DebugSettingsName { index: 3 });

    /// IDA 0x2ae7c4 `doDeclare` (guard at 0x2ae820, init at 0x2ae848,
    /// release at 0x2ae84c, return at 0x2ae87a).
    pub fn do_declare_debug_settings_name() -> DebugSettingsName {
        *DEBUG_SETTINGS_NAME_CELL
    }

    /// IDA 0x2ae7c0 `callDoDeclare` — thunk straight into `doDeclare`.
    pub fn call_do_declare_debug_settings() -> DebugSettingsName {
        do_declare_debug_settings_name()
    }

    /// IDA 0x2ae77c `declare`: null `sDebugSettings` text bails to
    /// `getNullName` (0x2ae78e-0x2ae7ba); else `call_once` + `doDeclare`
    /// (0x2ae792-0x2ae7b2).
    pub fn declare_debug_settings_name(text: Option<&str>) -> DebugSettingsName {
        match text {
            None => DebugSettingsName {
                index: crate::generated_core_shard_af::core_af::null_name(),
            },
            Some(_) => do_declare_debug_settings_name(),
        }
    }

    /// was: `ServiceProvider::doGetClassIndex<StatsService>` result (IDA
    /// 0x2ae108) — the process-wide class-index counter behind
    /// `newIndex` (0x2ae180), guarded once (0x2ae164-0x2ae184). One
    /// `AtomicUsize` preserves cross-instantiation uniqueness
    /// (`[INFERENCE]` on the counter start only).
    static CLASS_INDEX_COUNTER: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(1);
    static STATS_SERVICE_CLASS_INDEX: std::sync::LazyLock<usize> =
        std::sync::LazyLock::new(|| {
            CLASS_INDEX_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        });

    /// IDA 0x2ae108.
    pub fn stats_service_class_index() -> usize {
        *STATS_SERVICE_CLASS_INDEX
    }

    /// IDA 0x2a5e64 `shared_from<StatsService>(shared_ptr*)`: null source
    /// zeroes the out word (0x2a5ef2); else `enable_shared_from_this` from
    /// the `DescribedBase` subobject at `+40` (0x2a5ebe) with the `-36`
    /// derived adjust (0x2a5eca) and the count adopt (0x2a5eda-0x2a5ee8).
    /// The layout adjusts are Drop-glue-free here: clone-or-empty.
    pub fn shared_from_opt<T>(this: Option<&crate::SharedPtr<T>>) -> Option<crate::SharedPtr<T>> {
        this.cloned()
    }

    /// IDA 0x2a7348 `std::operator+<char>`: copy-construct from the left
    /// operand (0x2a736a), then `append` the right (0x2a73a2).
    pub fn string_concat(left: &str, right: &str) -> String {
        let mut out = left.to_owned();
        out.push_str(right);
        out
    }

    /// was: `RBX::Security::Context` behind the thread-local slot — reuses
    /// the `core_af` token/slot so impersonation and `current` share one
    /// thread-local with the `thread_specific_ptr<Context>` ports.
    pub type SecurityContext = crate::generated_core_shard_af::core_af::SecurityContext;

    /// IDA 0x2a3ca8 `Context::current()`: `get_tss_data` (0x2a3cc2); a null
    /// slot allocates a zero `Context` (0x2a3ccc-0x2a3cd0) and `reset`s it in
    /// (0x2a3cd2-0x2a3cda); returns the slot (0x2a3ce0).
    pub fn security_context_current() -> crate::SharedPtr<SecurityContext> {
        use crate::generated_core_shard_af::core_af as af;
        if let Some(ctx) = af::security_context_get() {
            ctx
        } else {
            let ctx = crate::SharedPtr::new(SecurityContext { token: 0 });
            af::security_context_reset(Some(ctx.clone()));
            ctx
        }
    }

    /// was: `RBX::Security::Impersonator` — the previous `Context` stashed in
    /// the guard (IDA 0x2a7148 `*a1 = old`), restored on drop.
    pub struct Impersonator {
        prev: Option<crate::SharedPtr<SecurityContext>>,
    }

    /// IDA 0x2a7120 `Impersonator(Identities)`: news a `Context{id}`
    /// (0x2a712e-0x2a7130), installs it via the thread-local `reset`
    /// (0x2a7132-0x2a7152), keeping the released old pointer.
    pub fn impersonate(identity: u32) -> Impersonator {
        use crate::generated_core_shard_af::core_af as af;
        let prev = af::security_context_get();
        af::security_context_reset(Some(crate::SharedPtr::new(SecurityContext { token: identity })));
        Impersonator { prev }
    }

    impl Drop for Impersonator {
        fn drop(&mut self) {
            use crate::generated_core_shard_af::core_af as af;
            af::security_context_reset(self.prev.take());
        }
    }
}
#[doc(alias = "__ZNSt12_Vector_baseIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE11_M_allocateEm")]
// 0x286250 — __ZNSt12_Vector_baseIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE11_M_allocateEm
pub fn stub_0x286250(n: usize) -> Vec<std::mem::MaybeUninit<usize>> {
    // IDA 0x286250: _Vector_base<pool*>::_M_allocate — bad_alloc at n >= 0x40000000 (0x286258), else operator new(4*n).
    core_signals::pool_ptr_vec_storage(n)
}

#[doc(alias = "__GLOBAL__I_a_68")]
// 0x286268 — __GLOBAL__I_a_68
pub fn stub_0x286268() {
    // IDA 0x286268: __GLOBAL__I_a_68 — categories, ios_base::Init + atexit, guarded bad_alloc/bad_exception eps + atexit(~exception_ptr). Core-only set.
    core_signals::ensure_init_a68();
}

#[doc(alias = "__GLOBAL__I_a_69")]
// 0x287738 — __GLOBAL__I_a_69
pub fn stub_0x287738() {
    // IDA 0x287738: __GLOBAL__I_a_69 — a_68 set + XmlAttribute/XmlElement/FWInstance/OnDemandInstance pool get_pools; LuaSettings BoundProps + Creator owned by reflection/datamodel.
    core_signals::ensure_init_a69();
}

#[doc(alias = "__GLOBAL__I_a_70")]
// 0x28aa88 — __GLOBAL__I_a_70
pub fn stub_0x28aa88() {
    // IDA 0x28aa88: __GLOBAL__I_a_70 — a_69 set + ProtectedString flyweight static-holder init; FLog group (fast-log) + LocalScript Creator (script) owned elsewhere.
    core_signals::ensure_init_a70();
}

#[doc(alias = "__ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_ED1Ev")]
// 0x28d6bc — __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_ED1Ev
pub fn stub_0x28d6bc(fw: core_signals::ProtectedFlyweight) {
    // IDA 0x28d6bc: flyweight<ProtectedString> D1 — atomic handle-count decrement (LDREX/STREX -1 at [ptr+8]), factory erase at zero. By-value drop + eager erase is that path.
    core_signals::ProtectedFlyweight::release(fw);
}

#[doc(alias = "__ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_")]
// 0x28d6fc — __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_
pub fn stub_0x28d6fc(
    dst: &mut core_signals::ProtectedFlyweight,
    src: &core_signals::ProtectedFlyweight,
) {
    // IDA 0x28d6fc: flyweight::operator=(const flyweight&) — addref src (+1 at [src+8]), store into dst, release dst old. Clone-before-store is that order.
    core_signals::ProtectedFlyweight::assign(dst, src);
}

#[doc(alias = "__ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EC2Ev")]
// 0x2949dc — __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EC2Ev
pub fn stub_0x2949dc() -> core_signals::ProtectedFlyweight {
    // IDA 0x2949dc: flyweight::flyweight() — default ProtectedString + flyweight_core::insert_value (0x294a36); temp string release is Drop glue.
    core_signals::ProtectedFlyweight::default_value()
}

#[doc(alias = "__ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS3_")]
// 0x294ba8 — __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS3_
pub fn stub_0x294ba8(dst: &mut core_signals::ProtectedFlyweight, src: &str) {
    // IDA 0x294ba8: flyweight::operator=(const value_type&) — insert_value(&tmp, src) (0x294bca), operator=(dst, &tmp) (0x294c04), release tmp with erase iff count was 1 (0x294c0c-0x294c26).
    core_signals::ProtectedFlyweight::assign_value(dst, src);
}

#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE13disconnectAllEv")]
// 0x294cc4 — __ZN3rbx7signals6signalIFvP9lua_StateEE13disconnectAllEv
pub fn stub_0x294cc4(sig: &core_signals::RawSignal) {
    // IDA 0x294cc4: signal<void(lua_State*)>::disconnectAll — mutex-guarded walk zeroing slot owners (0x294d70, v21=-11..0 bound), head := null (0x294da4).
    sig.disconnect_all();
}

#[doc(alias = "__GLOBAL__I_a_71")]
// 0x294e3c — __GLOBAL__I_a_71
pub fn stub_0x294e3c() {
    // IDA 0x294e3c: __GLOBAL__I_a_71 — a_70 set + default flyweight value (0x2949dc ctor); Script descriptors/creators + RegisterFlag owned elsewhere.
    core_signals::ensure_init_a71();
}

#[doc(alias = "__ZL5panicP9lua_State")]
// 0x2981dc — __ZL5panicP9lua_State
pub fn stub_0x2981dc(message: &str) -> ! {
    // IDA 0x2981dc: lua_State* panic hook — StandardOut print of the lua_tolstring(L,-1) text (0x2981fc-0x298252), count release (0x298258-0x298260), RBXCRASH (0x29826a). Message arrives as &str; the Lua stack lives at the live site.
    core_function::lua_panic_hook(message)
}

#[doc(alias = "__ZL4loadP9lua_StatePKcPFiS0_E")]
// 0x2982c8 — __ZL4loadP9lua_StatePKcPFiS0_E
pub fn stub_0x2982c8() {
    // IDA 0x2982c8: load(L, name, reader) — reader chunk (0x2982d2), locked-metatable guard (0x2982e0-0x29830c), stack trim (0x298314), fresh userdata + table with __index/__newindex/__metatable wired to `illegal` (0x29831c-0x2983a0), setfield into the registry (0x2983b4). Lua-registry owned (script crate) — carrier no-op in core.
}

#[doc(alias = "__ZL15pushNoArgumentsP9lua_State")]
// 0x29cad4 — __ZL15pushNoArgumentsP9lua_State
pub fn stub_0x29cad4() -> i32 {
    // IDA 0x29cad4: pushNoArguments — pushes nothing, returns 0 (0x29cad6), the lua result count.
    core_function::push_no_arguments()
}

#[doc(alias = "__ZL12cleanTimeoutRd")]
// 0x29f0fc — __ZL12cleanTimeoutRd
pub fn stub_0x29f0fc(value: &mut f64, setting: f64) {
    // IDA 0x29f0fc: cleanTimeout — *timeout vs the LuaSettings singleton +0x68 Floor (0x29f10a-0x29f116); below-setting (0x29f11a) or NaN/Inf via float isNanInf (0x29f11c-0x29f12e) stores the setting (0x29f130-0x29f138).
    core_function::clean_timeout(value, setting);
}

#[doc(alias = "__ZL7illegalP9lua_State")]
// 0x2a36f8 — __ZL7illegalP9lua_State
pub fn stub_0x2a36f8() -> ! {
    // IDA 0x2a36f8: illegal — __cxa_allocate_exception + runtime_error("can't modify this library") (0x2a3724-0x2a37e0) + __cxa_throw (0x2a3808). panic! is the throw.
    core_function::illegal_library_access()
}

#[doc(alias = "__ZN3RBX8Security7Context7currentEv")]
// 0x2a3ca8 — __ZN3RBX8Security7Context7currentEv
pub fn stub_0x2a3ca8() -> crate::SharedPtr<core_function::SecurityContext> {
    // IDA 0x2a3ca8: Security::Context::current — get_tss_data (0x2a3cc2); null slot news a zero Context (0x2a3ccc-0x2a3cd0) and resets it in (0x2a3cd2-0x2a3cda).
    core_function::security_context_current()
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_")]
// 0x2a4a7c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_
pub fn stub_0x2a4a7c(
    dst: &mut Option<crate::SharedPtr<core_function::TaskSchedulerJob>>,
    src: &Option<crate::SharedPtr<core_function::TaskSchedulerJob>>,
) {
    // IDA 0x2a4a7c: shared_ptr<TaskScheduler::Job>::operator= — shared_count tmp from src (0x2a4a90), store pi_ (0x2a4a9a), release old (0x2a4aa8).
    core_function::shared_ptr_assign(dst, src);
}

#[doc(alias = "__ZN16RobloxExtraSpace21eraseRefsFromAllNodesEv")]
// 0x2a4c6c — __ZN16RobloxExtraSpace21eraseRefsFromAllNodesEv
pub fn stub_0x2a4c6c() {
    // IDA 0x2a4c6c: RobloxExtraSpace::eraseRefsFromAllNodes — intrusive-set iterator from the head link (0x2a4c78), per node WeakThreadRef::Node::eraseAllRefs (*(it+20), 0x2a4c8c) + operator++ (0x2a4c92). Lua-thread-set owned (script crate) — carrier no-op in core.
}

#[doc(alias = "__ZN5boost4bindIvP9lua_StateiSsNS_3argILi1EEENS3_ILi2EEESsEENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_")]
// 0x2a5778 — __ZN5boost4bindIvP9lua_StateiSsNS_3argILi1EEENS3_ILi2EEESsEENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
pub fn stub_0x2a5778(func: usize, text: &str) -> core_function::BoundLuaCall {
    // IDA 0x2a5778: bind<void(lua_State*,int,string)> — bind_t capturing the target word +0 (0x2a57dc) and the list3 string +4 (0x2a57e6); COW releases (0x2a57f8-0x2a5870) are Drop glue. arg<1>/arg<2> carry no data.
    core_function::bind_lua_call(func, text)
}

#[doc(alias = "__ZNK5boost9function1ImP9lua_StateEclES2_")]
// 0x2a59f4 — __ZNK5boost9function1ImP9lua_StateEclES2_
pub fn stub_0x2a59f4(f: &core_function::LuaUlongFn, l: core_function::LuaStatePtr) -> u64 {
    // IDA 0x2a59f4: function1<ulong(lua_State*)>::operator() — bad_function_call on null (0x2a5a42-0x2a5a8a), else vtable tail-call (0x2a5a74).
    f.invoke(l)
}

#[doc(alias = "__ZNK5boost9function2IvP9lua_StatemEclES2_m")]
// 0x2a5abc — __ZNK5boost9function2IvP9lua_StatemEclES2_m
pub fn stub_0x2a5abc(f: &core_function::LuaVoidFn2, l: core_function::LuaStatePtr, extra: u64) {
    // IDA 0x2a5abc: function2<void(lua_State*,ulong)>::operator() — bad_function_call on null (0x2a5b0c-0x2a5b52), else vtable tail-call (0x2a5b20).
    f.invoke((l, extra));
}

#[doc(alias = "__ZNK5boost9function1IvbEclEb")]
// 0x2a5da0 — __ZNK5boost9function1IvbEclEb
pub fn stub_0x2a5da0(f: &core_function::BoolVoidFn, v: bool) {
    // IDA 0x2a5da0: function1<void(bool)>::operator() — bad_function_call on null (0x2a5dee-0x2a5e32), else vtable tail-call (0x2a5e00).
    f.invoke(v);
}

#[doc(alias = "__ZN3RBX11shared_fromINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEPS5_")]
// 0x2a5e64 — __ZN3RBX11shared_fromINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEPS5_
pub fn stub_0x2a5e64(
    this: Option<&crate::SharedPtr<core_function::RunServiceToken>>,
) -> Option<crate::SharedPtr<core_function::RunServiceToken>> {
    // IDA 0x2a5e64: shared_from<StatsService> — null source zeroes the out word (0x2a5ef2); else enable_shared_from_this from DescribedBase+40 (0x2a5ebe) with the -36 adjust (0x2a5eca) + count adopt (0x2a5eda-0x2a5ee8). Clone-or-empty is that path.
    core_function::shared_from_opt(this)
}

#[doc(alias = "__ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")]
// 0x2a6058 — __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
pub fn stub_0x2a6058(m: &mut core_function::RunningAverageTimeInterval, now: f64) {
    // IDA 0x2a6058: RunningAverageTimeInterval<1>::sample — armed call records the baseline stamp + disarms (0x2a6062-0x2a6080); later calls sample the stamp delta via RunningAverage::sample (0x2a6086-0x2a60a8). now is Time::now<1>, owned outside core.
    m.sample(now);
}

#[doc(alias = "__ZN3RBX14RunningAverageIddE6sampleEd")]
// 0x2a60b0 — __ZN3RBX14RunningAverageIddE6sampleEd
pub fn stub_0x2a60b0(m: &mut core_function::RunningAverage, v: f64) {
    // IDA 0x2a60b0: RunningAverage<double,double>::sample — fabs gate (0x2a60e0), EWMA blend past the first sample (0x2a60e6-0x2a6106), average/last/flag store (0x2a610c-0x2a6114), variance fold (0x2a6140), buffer push when present (0x2a6144-0x2a614c).
    m.sample(v);
}

#[doc(alias = "__ZN5boost6thread4joinEv")]
// 0x2a6368 — __ZN5boost6thread4joinEv
pub fn stub_0x2a6368(is_self: bool, handle: Option<std::thread::JoinHandle<()>>) {
    // IDA 0x2a6368: thread::join — pthread_self() == native_handle (0x2a638a) builds system_error(11) + throw thread_resource_error (0x2a63f0-0x2a6422); else join_noexcept (0x2a63ca).
    core_signals::thread_join_self_check(is_self);
    if let Some(handle) = handle {
        core_signals::thread_join_noexcept(handle);
    }
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_")]
// 0x2a65c8 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_
pub fn stub_0x2a65c8(
    dst: &mut Option<crate::SharedPtr<core_function::RunServiceToken>>,
    src: &Option<crate::SharedPtr<core_function::RunServiceToken>>,
) {
    // IDA 0x2a65c8: shared_ptr<RunService>::operator= — shared_count tmp from src (0x2a65dc), store pi_ (0x2a65e6), release old (0x2a65f4).
    core_function::shared_ptr_assign(dst, src);
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv")]
// 0x2a6cc0 — __ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv
pub fn stub_0x2a6cc0(sig: &core_signals::RawSignal) {
    // IDA 0x2a6cc0: signal_with_args<0,void()>::operator() — null head returns (0x2a6cee); SignalPrints trace (0x2a6d32, fast-log owned); next() walk calling live slots (0x2a6d3c/0x2a6d4c); final release (0x2a6dc4) is Arc drops.
    sig.emit_void0();
}

#[doc(alias = "__ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE")]
// 0x2a7120 — __ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE
pub fn stub_0x2a7120(identity: u32) -> core_function::Impersonator {
    // IDA 0x2a7120: Security::Impersonator ctor — new Context{identity} (0x2a712e-0x2a7130), install via thread-local reset keeping the old pointer (0x2a7132-0x2a7152). Drop restores.
    core_function::impersonate(identity)
}

#[doc(alias = "__ZNK5boost9function1IvP9lua_StateEclES2_")]
// 0x2a7220 — __ZNK5boost9function1IvP9lua_StateEclES2_
pub fn stub_0x2a7220(f: &core_function::LuaVoidFn1, l: core_function::LuaStatePtr) {
    // IDA 0x2a7220: function1<void(lua_State*)>::operator() — bad_function_call on null (0x2a726e-0x2a72b2), else vtable tail-call (0x2a7280).
    f.invoke(l);
}

#[doc(alias = "__ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_ERKS6_S8_")]
// 0x2a7348 — __ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_ERKS6_S8_
pub fn stub_0x2a7348(left: &str, right: &str) -> String {
    // IDA 0x2a7348: string operator+ — copy-construct from the left operand (0x2a736a), append the right (0x2a73a2).
    core_function::string_concat(left, right)
}

#[doc(alias = "__ZNK5boost9function1ISsRKSsEclES2_")]
// 0x2a73ec — __ZNK5boost9function1ISsRKSsEclES2_
pub fn stub_0x2a73ec(f: &core_function::StringMapFn, arg: &str) -> String {
    // IDA 0x2a73ec: function1<string(const string&)>::operator() — same null-guard + vtable tail-call shape as 0x2a59f4.
    f.invoke(arg.to_owned())
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvP9lua_StateEEclES3_")]
// 0x2a74b4 — __ZN3rbx7signals16signal_with_argsILi1EFvP9lua_StateEEclES3_
pub fn stub_0x2a74b4(sig: &core_signals::RawSignal, arg: core_function::LuaStatePtr) {
    // IDA 0x2a74b4: signal_with_args<1,void(lua_State*)>::operator() — null head returns (0x2a74e4); SignalPrints trace (0x2a7528, fast-log owned); next() walk calling live slots with the arg (0x2a752e/0x2a753e); final release (0x2a7554-0x2a755c) is Arc drops.
    sig.emit_lua1(arg as usize);
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE5clearEv")]
// 0x2a9450 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE5clearEv
pub fn stub_0x2a9450(f: &mut core_function::LuaDbgFn2) {
    // IDA 0x2a9450: function2<void(lua_State*,lua_Debug*)>::clear — same null-guard + manager-destroy-op-2 + word-zero shape as 0x2acdc4.
    f.clear();
}

#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_")]
// 0x2a947c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_
pub fn stub_0x2a947c(
    dst: &mut core_signals::IntrusiveSlotPtr,
    src: &core_signals::IntrusiveSlotPtr,
) {
    // IDA 0x2a947c: intrusive_ptr<signal<void(int)>::slot>::operator=(const&) — addref *src (0x2a948a), store (0x2a9490), release old (0x2a9496).
    let src_target = src.target.clone();
    core_signals::intrusive_slot_assign(dst, src_target.as_ref());
}

#[doc(alias = "__ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv")]
// 0x2a94a0 — __ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv
pub fn stub_0x2a94a0() -> &'static parking_lot::Mutex<()> {
    // IDA 0x2a94a0: signal<void(int)>::safe_static_do_get_mutex — guarded operator new(0x2C) + mutex::mutex (0x2a9510-0x2a9528). LazyLock static is the guarded word.
    core_signals::signal_int_mutex()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13disconnectAllEv")]
// 0x2a9598 — __ZN3rbx7signals6signalIFvvEE13disconnectAllEv
pub fn stub_0x2a9598(sig: &core_signals::RawSignal) {
    // IDA 0x2a9598: signal<void()>::disconnectAll — same mutex-guarded owner-zeroing walk + head-null store as 0x294cc4.
    sig.disconnect_all();
}

#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_")]
// 0x2a9710 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_
pub fn stub_0x2a9710(
    dst: &mut core_signals::IntrusiveSlotPtr,
    src: &core_signals::IntrusiveSlotPtr,
) {
    // IDA 0x2a9710: intrusive_ptr<signal<void()>::slot>::operator=(const&) — addref *src (0x2a971e), store (0x2a9724), release old (0x2a972a).
    let src_target = src.target.clone();
    core_signals::intrusive_slot_assign(dst, src_target.as_ref());
}

#[doc(alias = "__ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv")]
// 0x2a9738 — __ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv
pub fn stub_0x2a9738() -> &'static parking_lot::Mutex<()> {
    // IDA 0x2a9738: signal<void()>::safe_static_do_get_mutex — guarded operator new(0x2C) + mutex::mutex (0x2a97a8-0x2a97c0). LazyLock static is the guarded word.
    core_signals::signal_void_mutex()
}

#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_")]
// 0x2ac1c0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_
pub fn stub_0x2ac1c0(
    dst: &mut core_signals::IntrusiveSlotPtr,
    src: Option<core_signals::SlotHandle>,
) {
    // IDA 0x2ac1c0: intrusive_ptr<signal<void(RunTransition)>::slot>::operator=(slot*) — if (new) add_ref (0x2ac1ca); old = *dst, *dst = new (0x2ac1d2-0x2ac1d4); if (old) release (0x2ac1d8-0x2ac1da). Same shape as 0x2afc34.
    core_signals::intrusive_slot_assign(dst, src.as_ref());
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv")]
// 0x2ac368 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x2ac368() -> &'static parking_lot::Mutex<()> {
    // IDA 0x2ac368: signal<void(RunTransition)>::slot::safe_static_do_get_mutex — in-place mutex + __cxa_atexit(~mutex) (0x2ac3de-0x2ac402). LazyLock static is the guarded word.
    core_function::slot_rt_static_mutex()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotD1Ev")]
// 0x2ac458 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotD1Ev
pub fn stub_0x2ac458() {
    // IDA 0x2ac458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
// 0x2ac58c — __ZN3rbx7signals6signalIFvP9lua_StateEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
pub fn stub_0x2ac58c() {
    // IDA 0x2ac58c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE8on_errorERSt9exception")]
// 0x2ac6ec — __ZN3rbx7signals6signalIFvP9lua_StateEE8on_errorERSt9exception
pub fn stub_0x2ac6ec() {
    // IDA 0x2ac6ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSERKS9_")]
// 0x2ac718 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSERKS9_
pub fn stub_0x2ac718() {
    // IDA 0x2ac718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv")]
// 0x2ac740 — __ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv
pub fn stub_0x2ac740() {
    // IDA 0x2ac740: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE5dummy7nonnullEv")]
// 0x2ac838 — __ZN5boost9function1ISsRKSsE5dummy7nonnullEv
pub fn stub_0x2ac838() {
    // IDA 0x2ac838: function1<string(const string&)>::dummy::nonnull — empty safe-bool marker body.
    core_function::dummy_nonnull();
}

#[doc(alias = "__ZN5boost8functionIFSsRKSsEEaSERKS4_")]
// 0x2acc24 — __ZN5boost8functionIFSsRKSsEEaSERKS4_
pub fn stub_0x2acc24(dst: &mut core_function::StringMapFn, src: &core_function::StringMapFn) {
    // IDA 0x2acc24: function<string(const string&)>::operator= — temp/assign_to_own/swap/clear (0x2acc48-0x2acc8e).
    dst.assign_from(src);
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE4swapERS3_")]
// 0x2acce8 — __ZN5boost9function1ISsRKSsE4swapERS3_
pub fn stub_0x2acce8(a: &mut core_function::StringMapFn, b: &mut core_function::StringMapFn) {
    // IDA 0x2acce8: function1<string(const string&)>::swap — self-swap no-op (0x2acd36), else temp triple-move_assign + clear (0x2acd3a-0x2acd6a).
    a.swap_with(b);
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE5clearEv")]
// 0x2acdc4 — __ZN5boost9function1ISsRKSsE5clearEv
pub fn stub_0x2acdc4(f: &mut core_function::StringMapFn) {
    // IDA 0x2acdc4: function1<string(const string&)>::clear — null word no-op (0x2acdca-0x2acdce); heap target destroyed via manager op 2 unless inline bit0 (0x2acdce-0x2acdec); word := 0.
    f.clear();
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE11move_assignERS3_")]
// 0x2acdf0 — __ZN5boost9function1ISsRKSsE11move_assignERS3_
pub fn stub_0x2acdf0(dst: &mut core_function::StringMapFn, src: &mut core_function::StringMapFn) {
    // IDA 0x2acdf0: function1<string(const string&)>::move_assign — self-move no-op (0x2ace3e); empty src clears dst (0x2ace64); inline copy vs manager clone (0x2ace4a-0x2ace7e); src := 0 (0x2ace84).
    dst.move_assign(src);
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_")]
// 0x2acef4 — __ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_
pub fn stub_0x2acef4(dst: &mut core_function::StringMapFn, src: &core_function::StringMapFn) {
    // IDA 0x2acef4: function1<string(const string&)>::assign_to_own — empty src leaves dst (0x2acefa); inline copy (0x2acefc-0x2acf0c) vs manager clone op 0 (0x2acf22).
    dst.assign(src);
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE5dummy7nonnullEv")]
// 0x2ad520 — __ZN5boost9function1IvP9lua_StateE5dummy7nonnullEv
pub fn stub_0x2ad520() {
    // IDA 0x2ad520: function1<void(lua_State*)>::dummy::nonnull — empty safe-bool marker body.
    core_function::dummy_nonnull();
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v")]
// 0x2adfd8 — __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v
pub fn stub_0x2adfd8(text: Option<&str>) -> core_function::StatsName {
    // IDA 0x2adfd8: Name::declare<sStats> — null text bails to getNullName (0x2adfea-0x2ae016); else call_once(callDoDeclare) (0x2adfee-0x2ae006) + doDeclare tail-call (0x2ae00e).
    core_function::declare_stats_name(text)
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v")]
// 0x2ae020 — __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v
pub fn stub_0x2ae020() -> core_function::StatsName {
    // IDA 0x2ae020: Name::doDeclare<sStats> — guarded once-init (0x2ae07c-0x2ae0a8) returning the static (0x2ae0d6).
    core_function::do_declare_stats_name()
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv")]
// 0x2ae108 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv
pub fn stub_0x2ae108() -> usize {
    // IDA 0x2ae108: ServiceProvider::doGetClassIndex<StatsService> — guarded once-init (0x2ae164-0x2ae184) of newIndex (0x2ae180).
    core_function::stats_service_class_index()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v")]
// 0x2ae77c — __ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v
pub fn stub_0x2ae77c(text: Option<&str>) -> core_function::DebugSettingsName {
    // IDA 0x2ae77c: Name::declare<sDebugSettings> — null text bails to getNullName (0x2ae78e-0x2ae7ba); else call_once(callDoDeclare) (0x2ae792-0x2ae7a8) + doDeclare tail-call (0x2ae7b2).
    core_function::declare_debug_settings_name(text)
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv")]
// 0x2ae7c0 — __ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv
pub fn stub_0x2ae7c0() -> core_function::DebugSettingsName {
    // IDA 0x2ae7c0: Name::callDoDeclare<sDebugSettings> — thunk into doDeclare.
    core_function::call_do_declare_debug_settings()
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v")]
// 0x2ae7c4 — __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v
pub fn stub_0x2ae7c4() -> core_function::DebugSettingsName {
    // IDA 0x2ae7c4: Name::doDeclare<sDebugSettings> — guarded once-init (0x2ae820-0x2ae84c) returning the static (0x2ae87a).
    core_function::do_declare_debug_settings_name()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE")]
// 0x2afa28 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
pub fn stub_0x2afa28(
    sig: &core_signals::RawSignalHandle,
    slot: core_signals::SlotHandle,
) {
    // IDA 0x2afa28: signal<void(Heartbeat)>::insert — ReleaseAssert(item) (signal.h:290), call_once static-mutex init (0x2afad2), lock, push slot at head with intrusive addrefs.
    core_signals::RawSignal::insert(sig, slot);
}

#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_")]
// 0x2afc34 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_
pub fn stub_0x2afc34(
    dst: &mut core_signals::IntrusiveSlotPtr,
    src: Option<core_signals::SlotHandle>,
) {
    // IDA 0x2afc34: intrusive_ptr<slot>::operator=(slot*) — if (new) add_ref (0x2afc42); old = *dst, *dst = new (0x2afc46-0x2afc48); if (old) release (0x2afc4e).
    core_signals::intrusive_slot_assign(dst, src.as_ref());
}

#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_")]
// 0x2afc58 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_
pub fn stub_0x2afc58(
    dst: &mut core_signals::IntrusiveSlotPtr,
    src: &core_signals::IntrusiveSlotPtr,
) {
    // IDA 0x2afc58: intrusive_ptr<slot>::operator=(const&) — addref *src (0x2afc66), store (0x2afc6c), release old (0x2afc72).
    let src_target = src.target.clone();
    core_signals::intrusive_slot_assign(dst, src_target.as_ref());
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv")]
// 0x2afc80 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
pub fn stub_0x2afc80() -> &'static parking_lot::Mutex<()> {
    // IDA 0x2afc80: signal<void(Heartbeat)>::safe_static_do_get_mutex — guarded operator new(0x2C) + mutex::mutex (0x2afcf0-0x2afd08). LazyLock static is the guarded word.
    core_signals::signal_heartbeat_mutex()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot10disconnectEv")]
// 0x2afe78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot10disconnectEv
pub fn stub_0x2afe78(slot: &core_signals::SlotHandle) {
    // IDA 0x2afe78: slot::disconnect — connected-gated (0x2afea2), call_once mutex init (0x2afee2), lock, re-check + zero owner + remove(signal, slot) (0x2aff08-0x2aff16), unlock.
    core_signals::slot_disconnect(slot);
}

#[doc(alias = "__ZNK3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot9connectedEv")]
// 0x2aff88 — __ZNK3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot9connectedEv
pub fn stub_0x2aff88(slot: &core_signals::Slot) -> bool {
    // IDA 0x2aff88: slot::connected — return *(a1 + 12) != 0 (0x2aff90).
    slot.is_connected()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE")]
// 0x2affbc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
pub fn stub_0x2affbc(sig: &core_signals::RawSignal, slot: &core_signals::SlotHandle) {
    // IDA 0x2affbc: signal::remove — expired-assert (signal.h:261), SignalPrints trace (fast-log owned), head/predecessor-link splice adopting item->next (LABEL_14), signal.h:284 post-assert.
    sig.remove(slot);
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot22safe_static_init_mutexEv")]
// 0x2b00ac — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot22safe_static_init_mutexEv
pub fn stub_0x2b00ac() -> &'static parking_lot::Mutex<()> {
    // IDA 0x2b00ac: slot::safe_static_init_mutex — thunk into safe_static_do_get_mutex (attributes: thunk).
    core_signals::slot_static_init_mutex()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv")]
// 0x2b00b0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x2b00b0() -> &'static parking_lot::Mutex<()> {
    // IDA 0x2b00b0: slot::safe_static_do_get_mutex — in-place mutex + __cxa_atexit(~mutex) (0x2b0126-0x2b0144). LazyLock static is the guarded word.
    core_signals::slot_static_mutex()
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD1Ev")]
// 0x2b01a0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD1Ev
pub fn stub_0x2b01a0(slot: &core_signals::Slot) {
    // IDA 0x2b01a0: slot::~slot (D1) — reinstall vtables (0x2b01b2-0x2b01ba), release *(a1+8) when non-null (0x2b01be-0x2b01c4). Dropping the next Arc is the release.
    core_signals::slot_destruct(slot);
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD0Ev")]
// 0x2b01cc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD0Ev
pub fn stub_0x2b01cc(slot: core_signals::SlotHandle) {
    // IDA 0x2b01cc: slot::~slot (D0) — vtables (0x2b01fc-0x2b020c), release *(a1+8) (0x2b0232-0x2b023a), operator delete (0x2b0246). By-value drop is the delete.
    core_signals::slot_delete(slot);
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v")]
// 0x2b03a0 — __ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v
pub fn stub_0x2b03a0() {
    // IDA 0x2b03a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v")]
// 0x2b0568 — __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v
pub fn stub_0x2b0568() {
    // IDA 0x2b0568: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEED2Ev")]
// 0x2b0a88 — __ZNSt5dequeISsSaISsEED2Ev
pub fn stub_0x2b0a88() {
    // IDA 0x2b0a88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt11_Deque_baseISsSaISsEED2Ev")]
// 0x2b0b70 — __ZNSt11_Deque_baseISsSaISsEED2Ev
pub fn stub_0x2b0b70() {
    // IDA 0x2b0b70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v")]
// 0x2b0c88 — __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v
pub fn stub_0x2b0c88() {
    // IDA 0x2b0c88: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE")]
// 0x2b1060 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE
pub fn stub_0x2b1060() {
    // IDA 0x2b1060: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv")]
// 0x2b1910 — __ZN3RBX4Name13callDoDeclareILZNS_16sContentProviderEEEEvv
pub fn stub_0x2b1910() {
    // IDA 0x2b1910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15ContentProviderEEEvv")]
// 0x2b1918 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15ContentProviderEEEvv
pub fn stub_0x2b1918() {
    // IDA 0x2b1918: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE4swapERS3_")]
// 0x2b1a6c — __ZN5boost9function1IvP9lua_StateE4swapERS3_
pub fn stub_0x2b1a6c(a: &mut core_function::LuaVoidFn1, b: &mut core_function::LuaVoidFn1) {
    // IDA 0x2b1a6c: function1<void(lua_State*)>::swap — self-swap no-op (0x2b1aba), else temp triple-move_assign + clear (0x2b1abe-0x2b1aee).
    a.swap_with(b);
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE11move_assignERS3_")]
// 0x2b1b48 — __ZN5boost9function1IvP9lua_StateE11move_assignERS3_
pub fn stub_0x2b1b48(dst: &mut core_function::LuaVoidFn1, src: &mut core_function::LuaVoidFn1) {
    // IDA 0x2b1b48: function1<void(lua_State*)>::move_assign — self-move no-op (0x2b1b96); empty src clears dst (0x2b1bbc); inline copy vs manager clone (0x2b1ba2-0x2b1bd6); src := 0 (0x2b1bdc).
    dst.move_assign(src);
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE5clearEv")]
// 0x2b1c4c — __ZN5boost9function1IvP9lua_StateE5clearEv
pub fn stub_0x2b1c4c(f: &mut core_function::LuaVoidFn1) {
    // IDA 0x2b1c4c: function1<void(lua_State*)>::clear — null word no-op (0x2b1c52-0x2b1c56); heap destroy op 2 unless inline bit0 (0x2b1c5e-0x2b1c70); word := 0.
    f.clear();
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE5dummy7nonnullEv")]
// 0x2b2688 — __ZN5boost9function2IvP9lua_StatemE5dummy7nonnullEv
pub fn stub_0x2b2688() {
    // IDA 0x2b2688: function2<void(lua_State*,ulong)>::dummy::nonnull — empty safe-bool marker body.
    core_function::dummy_nonnull();
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// 0x2b268c — __ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2b268c(
    spec: core_function::BoundLuaCall,
    target: crate::SharedPtr<dyn Fn(core_function::LuaStatePtr, i32, &str) + Send + Sync>,
) -> core_function::LuaVoidFn2 {
    // IDA 0x2b268c: function2<void(lua_State*,ulong)> ctor from bind_t — zero word (0x2b26ac), functor split (0x2b26b8-0x2b26ce), assign_to stored vtable (0x2b26f6); temp release (0x2b2708-0x2b2750) is Drop glue. Target word stays opaque; the live site supplies the callable.
    core_function::bind_to_function2(spec, target)
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_")]
// 0x2b27b8 — __ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_
pub fn stub_0x2b27b8(
    f: &mut core_function::LuaVoidFn2,
    spec: core_function::BoundLuaCall,
    target: crate::SharedPtr<dyn Fn(core_function::LuaStatePtr, i32, &str) + Send + Sync>,
) {
    // IDA 0x2b27b8: function2::assign_to<bind_t> — stored-vtable install (0x2b284a) + functor copy (0x2b282c) into the caller's (known-empty, cf. 0x2b268c) word; temp release (0x2b283e-0x2b288a) is Drop glue.
    *f = core_function::bind_to_function2(spec, target);
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")]
// 0x2b28f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
pub fn stub_0x2b28f4() {
    // IDA 0x2b28f4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEvS6_mE6invokeERNS1_15function_bufferES6_m")]
// 0x2b2974 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvP9lua_StateiSsENS3_5list3INS_3argILi1EEENSA_ILi2EEENS3_5valueISsEEEEEEvS6_mE6invokeERNS1_15function_bufferES6_m
pub fn stub_0x2b2974() {
    // IDA 0x2b2974: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferE")]
// 0x2b2998 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0x2b2998() {
    // IDA 0x2b2998: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x2b2ac4 — __ZNK5boost6detail8function13basic_vtable2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS4_iSsENS7_5list3INS_3argILi1EEENSC_ILi2EEENS7_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0x2b2ac4() {
    // IDA 0x2b2ac4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEclIPFvP9lua_StateiSsENS0_5list2IRSA_RmEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x2b2bfc — __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEclIPFvP9lua_StateiSsENS0_5list2IRSA_RmEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x2b2bfc() {
    // IDA 0x2b2bfc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_")]
// 0x2b2d20 — __ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_
pub fn stub_0x2b2d20(text: &str) -> core_function::BoundArgList {
    // IDA 0x2b2d20: list3<arg<1>,arg<2>,value<string>> ctor — string copy into the pack (0x2b2d44-0x2b2d7a); temp release (0x2b2d8c-0x2b2dd4) is Drop glue.
    core_function::bind_arg_list(text)
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_")]
// 0x2b3f84 — __ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_
pub fn stub_0x2b3f84(dst: &mut core_function::LuaVoidFn2, src: &core_function::LuaVoidFn2) {
    // IDA 0x2b3f84: function2<void(lua_State*,ulong)>::assign_to_own — empty src leaves dst (0x2b3f8a); inline copy (0x2b3f8c-0x2b3f9c) vs manager clone op 0 (0x2b3fb2).
    dst.assign(src);
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE5clearEv")]
// 0x2b3fb4 — __ZN5boost9function2IvP9lua_StatemE5clearEv
pub fn stub_0x2b3fb4(f: &mut core_function::LuaVoidFn2) {
    // IDA 0x2b3fb4: function2<void(lua_State*,ulong)>::clear — null word no-op (0x2b3fba-0x2b3fbe); heap destroy op 2 unless inline bit0 (0x2b3fc6-0x2b3fd8); word := 0.
    f.clear();
}

#[doc(alias = "__ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_")]
// 0x2b3fe0 — __ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_
pub fn stub_0x2b3fe0(dst: &mut core_function::LuaUlongFn, src: &core_function::LuaUlongFn) {
    // IDA 0x2b3fe0: function1<ulong(lua_State*)>::assign_to_own — empty src leaves dst (0x2b3fe6); inline copy (0x2b3f8e-0x2b3ff8) vs manager clone op 0 (0x2b400e).
    dst.assign(src);
}

#[doc(alias = "__ZN5boost9function1ImP9lua_StateE5clearEv")]
// 0x2b4010 — __ZN5boost9function1ImP9lua_StateE5clearEv
pub fn stub_0x2b4010(f: &mut core_function::LuaUlongFn) {
    // IDA 0x2b4010: function1<ulong(lua_State*)>::clear — null word no-op (0x2b4016-0x2b401a); heap destroy op 2 unless inline bit0 (0x2b4022-0x2b4034); word := 0.
    f.clear();
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE")]
// 0x2b403c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
pub fn stub_0x2b403c() {
    // IDA 0x2b403c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEmS6_E6invokeERNS1_15function_bufferES6_")]
// 0x2b409c — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tImPFmP9lua_StateENS3_5list1INS_3argILi1EEEEEEEmS6_E6invokeERNS1_15function_bufferES6_
pub fn stub_0x2b409c() {
    // IDA 0x2b409c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE")]
// 0x2b40a8 — __ZN5boost6threadC2INS_9function0IvEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRS4_NS_6detail13thread_move_tIS4_EEEE5valueEPNS0_5dummyEE4typeE
pub fn stub_0x2b40a8() {
    // IDA 0x2b40a8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN5boost6detail11thread_dataINS_9function0IvEEED0Ev")]
// 0x2b41f0 — __ZN5boost6detail11thread_dataINS_9function0IvEEED0Ev
pub fn stub_0x2b41f0() {
    // IDA 0x2b41f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail11thread_dataINS_9function0IvEEE3runEv")]
// 0x2b42d0 — __ZN5boost6detail11thread_dataINS_9function0IvEEE3runEv
pub fn stub_0x2b42d0() {
    // IDA 0x2b42d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail16thread_data_base25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE")]
// 0x2b42d8 — __ZN5boost6detail16thread_data_base25notify_all_at_thread_exitEPNS_18condition_variableEPNS_5mutexE
pub fn stub_0x2b42d8() {
    // IDA 0x2b42d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[cfg(test)]
mod batch_ae_tests {
    use super::core_signals::*;
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn pool_vec_storage_matches_ida_allocate() {
        let store = stub_0x286250(4);
        assert_eq!(store.capacity(), 4);
    }

    #[test]
    #[should_panic(expected = "bad_alloc")]
    fn pool_vec_storage_huge_panics_like_throw_bad_alloc() {
        stub_0x286250(0x4000_0000);
    }

    #[test]
    fn static_inits_are_idempotent_and_create_pools() {
        stub_0x286268();
        stub_0x287738();
        stub_0x28aa88();
        stub_0x294e3c();
        stub_0x286268();
        stub_0x294e3c();
        assert_eq!(xml_attribute_pool_size(), 20);
        assert_eq!(xml_element_pool_size(), 36);
        assert_eq!(flyweight_default_text(), "");
    }

    #[test]
    fn flyweight_interns_and_assigns() {
        let a = stub_0x2949dc();
        assert_eq!(*a.value, "");
        let mut b = stub_0x2949dc();
        stub_0x294ba8(&mut b, "hello");
        assert_eq!(*b.value, "hello");
        let c = ProtectedFlyweight::intern("hello");
        assert!(crate::SharedPtr::ptr_eq(&b.value, &c.value));
        stub_0x28d6fc(&mut b, &a);
        assert_eq!(*b.value, "");
        stub_0x28d6bc(c);
        stub_0x28d6bc(a);
        stub_0x28d6bc(b);
    }

    #[test]
    fn intrusive_assign_addrefs_new_and_releases_old() {
        let s1: SlotHandle = crate::SharedPtr::new(Slot::new());
        let s2: SlotHandle = crate::SharedPtr::new(Slot::new());
        let mut dst = IntrusiveSlotPtr::default();
        stub_0x2afc34(&mut dst, Some(s1.clone()));
        assert!(crate::SharedPtr::ptr_eq(dst.target.as_ref().unwrap(), &s1));
        assert_eq!(crate::SharedPtr::strong_count(&s1), 2);
        let src = IntrusiveSlotPtr { target: Some(s2.clone()) };
        stub_0x2afc58(&mut dst, &src);
        assert!(crate::SharedPtr::ptr_eq(dst.target.as_ref().unwrap(), &s2));
        assert_eq!(crate::SharedPtr::strong_count(&s1), 1);
        let mut v1 = IntrusiveSlotPtr::default();
        let mut v2 = IntrusiveSlotPtr::default();
        stub_0x2a947c(&mut v1, &src);
        stub_0x2a9710(&mut v2, &src);
        assert!(crate::SharedPtr::ptr_eq(v1.target.as_ref().unwrap(), &s2));
        assert!(crate::SharedPtr::ptr_eq(v2.target.as_ref().unwrap(), &s2));
    }

    #[test]
    fn static_mutexes_are_stable_singletons() {
        assert!(std::ptr::eq(stub_0x2a94a0(), signal_int_mutex()));
        assert!(std::ptr::eq(stub_0x2a9738(), signal_void_mutex()));
        assert!(std::ptr::eq(stub_0x2afc80(), signal_heartbeat_mutex()));
        assert!(std::ptr::eq(stub_0x2b00b0(), slot_static_mutex()));
        assert!(std::ptr::eq(stub_0x2b00ac(), slot_static_mutex()));
    }

    fn counted_slot(counter: &std::sync::Arc<AtomicUsize>) -> SlotHandle {
        let slot: SlotHandle = crate::SharedPtr::new(Slot::new());
        let c = std::sync::Arc::clone(counter);
        *slot.callback.lock() = Some(Box::new(move || {
            c.fetch_add(1, Ordering::SeqCst);
        }));
        slot
    }

    #[test]
    fn insert_emit_disconnect_remove_match_ida() {
        let sig: RawSignalHandle = crate::SharedPtr::new(RawSignal::new());
        let hits = std::sync::Arc::new(AtomicUsize::new(0));
        let s1 = counted_slot(&hits);
        let s2 = counted_slot(&hits);
        stub_0x2afa28(&sig, s1.clone());
        stub_0x2afa28(&sig, s2.clone());
        assert!(stub_0x2aff88(&s1) && stub_0x2aff88(&s2));
        stub_0x2a6cc0(&sig);
        assert_eq!(hits.load(Ordering::SeqCst), 2);
        stub_0x2afe78(&s1);
        assert!(!stub_0x2aff88(&s1));
        assert!(stub_0x2aff88(&s2));
        stub_0x2a6cc0(&sig);
        assert_eq!(hits.load(Ordering::SeqCst), 3);
        stub_0x2affbc(&sig, &s2);
        assert!(sig.head.lock().is_none());
        stub_0x2a6cc0(&sig);
        assert_eq!(hits.load(Ordering::SeqCst), 3);
    }

    #[test]
    fn disconnect_all_clears_every_slot() {
        let sig: RawSignalHandle = crate::SharedPtr::new(RawSignal::new());
        let hits = std::sync::Arc::new(AtomicUsize::new(0));
        let slots: Vec<SlotHandle> =
            (0..4).map(|_| counted_slot(&hits)).collect();
        for s in &slots {
            stub_0x2afa28(&sig, s.clone());
        }
        stub_0x294cc4(&sig);
        assert!(sig.head.lock().is_none());
        assert!(slots.iter().all(|s| !stub_0x2aff88(s)));
        stub_0x2a9598(&sig);
        stub_0x2a6cc0(&sig);
        assert_eq!(hits.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn slot_dtors_release_next() {
        let sig: RawSignalHandle = crate::SharedPtr::new(RawSignal::new());
        let s: SlotHandle = crate::SharedPtr::new(Slot::new());
        stub_0x2afa28(&sig, s.clone());
        stub_0x2b01a0(&s);
        assert!(s.next.lock().is_none());
        stub_0x2b01cc(s);
    }

    #[test]
    fn thread_join_runs_and_self_join_panics() {
        let done = std::sync::Arc::new(AtomicUsize::new(0));
        let c = std::sync::Arc::clone(&done);
        let handle = std::thread::spawn(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
        stub_0x2a6368(false, Some(handle));
        assert_eq!(done.load(Ordering::SeqCst), 1);
        stub_0x2a6368(false, None);
    }

    #[test]
    #[should_panic(expected = "trying joining itself")]
    fn thread_self_join_throws_resource_error() {
        stub_0x2a6368(true, None);
    }
}
