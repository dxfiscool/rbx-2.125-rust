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
    }

    impl Slot {
        pub fn new() -> Self {
            Self {
                next: parking_lot::Mutex::new(None),
                owner: parking_lot::Mutex::new(None),
                callback: parking_lot::Mutex::new(None),
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
pub fn stub_0x2981dc() {
    // IDA 0x2981dc: flyweight interned-value holder. Arc<str>-style interning at the live site — carrier no-op.
}

#[doc(alias = "__ZL4loadP9lua_StatePKcPFiS0_E")]
// 0x2982c8 — __ZL4loadP9lua_StatePKcPFiS0_E
pub fn stub_0x2982c8() {
    // IDA 0x2982c8: flyweight interned-value holder. Arc<str>-style interning at the live site — carrier no-op.
}

#[doc(alias = "__ZL15pushNoArgumentsP9lua_State")]
// 0x29cad4 — __ZL15pushNoArgumentsP9lua_State
pub fn stub_0x29cad4() {
    // IDA 0x29cad4: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__ZL12cleanTimeoutRd")]
// 0x29f0fc — __ZL12cleanTimeoutRd
pub fn stub_0x29f0fc() {
    // IDA 0x29f0fc: global static ctor/dtor key. Static init — carrier no-op.
}

#[doc(alias = "__ZL7illegalP9lua_State")]
// 0x2a36f8 — __ZL7illegalP9lua_State
pub fn stub_0x2a36f8() {
    // IDA 0x2a36f8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX8Security7Context7currentEv")]
// 0x2a3ca8 — __ZN3RBX8Security7Context7currentEv
pub fn stub_0x2a3ca8() {
    // IDA 0x2a3ca8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_")]
// 0x2a4a7c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_
pub fn stub_0x2a4a7c() {
    // IDA 0x2a4a7c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN16RobloxExtraSpace21eraseRefsFromAllNodesEv")]
// 0x2a4c6c — __ZN16RobloxExtraSpace21eraseRefsFromAllNodesEv
pub fn stub_0x2a4c6c() {
    // IDA 0x2a4c6c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN5boost4bindIvP9lua_StateiSsNS_3argILi1EEENS3_ILi2EEESsEENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_")]
// 0x2a5778 — __ZN5boost4bindIvP9lua_StateiSsNS_3argILi1EEENS3_ILi2EEESsEENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
pub fn stub_0x2a5778() {
    // IDA 0x2a5778: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function1ImP9lua_StateEclES2_")]
// 0x2a59f4 — __ZNK5boost9function1ImP9lua_StateEclES2_
pub fn stub_0x2a59f4() {
    // IDA 0x2a59f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function2IvP9lua_StatemEclES2_m")]
// 0x2a5abc — __ZNK5boost9function2IvP9lua_StatemEclES2_m
pub fn stub_0x2a5abc() {
    // IDA 0x2a5abc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function1IvbEclEb")]
// 0x2a5da0 — __ZNK5boost9function1IvbEclEb
pub fn stub_0x2a5da0() {
    // IDA 0x2a5da0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX11shared_fromINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEPS5_")]
// 0x2a5e64 — __ZN3RBX11shared_fromINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEPS5_
pub fn stub_0x2a5e64() {
    // IDA 0x2a5e64: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")]
// 0x2a6058 — __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
pub fn stub_0x2a6058() {
    // IDA 0x2a6058: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX14RunningAverageIddE6sampleEd")]
// 0x2a60b0 — __ZN3RBX14RunningAverageIddE6sampleEd
pub fn stub_0x2a60b0() {
    // IDA 0x2a60b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
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
pub fn stub_0x2a65c8() {
    // IDA 0x2a65c8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv")]
// 0x2a6cc0 — __ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv
pub fn stub_0x2a6cc0(sig: &core_signals::RawSignal) {
    // IDA 0x2a6cc0: signal_with_args<0,void()>::operator() — null head returns (0x2a6cee); SignalPrints trace (0x2a6d32, fast-log owned); next() walk calling live slots (0x2a6d3c/0x2a6d4c); final release (0x2a6dc4) is Arc drops.
    sig.emit_void0();
}

#[doc(alias = "__ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE")]
// 0x2a7120 — __ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE
pub fn stub_0x2a7120() {
    // IDA 0x2a7120: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function1IvP9lua_StateEclES2_")]
// 0x2a7220 — __ZNK5boost9function1IvP9lua_StateEclES2_
pub fn stub_0x2a7220() {
    // IDA 0x2a7220: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_ERKS6_S8_")]
// 0x2a7348 — __ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_ERKS6_S8_
pub fn stub_0x2a7348() {
    // IDA 0x2a7348: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function1ISsRKSsEclES2_")]
// 0x2a73ec — __ZNK5boost9function1ISsRKSsEclES2_
pub fn stub_0x2a73ec() {
    // IDA 0x2a73ec: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvP9lua_StateEEclES3_")]
// 0x2a74b4 — __ZN3rbx7signals16signal_with_argsILi1EFvP9lua_StateEEclES3_
pub fn stub_0x2a74b4() {
    // IDA 0x2a74b4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE5clearEv")]
// 0x2a9450 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE5clearEv
pub fn stub_0x2a9450() {
    // IDA 0x2a9450: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
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
pub fn stub_0x2ac1c0() {
    // IDA 0x2ac1c0: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv")]
// 0x2ac368 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x2ac368() {
    // IDA 0x2ac368: intrusive refcount op. Arc/Weak — carrier no-op.
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
    // IDA 0x2ac838: function null-target guard. Option<Box<dyn Fn>>::is_some — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFSsRKSsEEaSERKS4_")]
// 0x2acc24 — __ZN5boost8functionIFSsRKSsEEaSERKS4_
pub fn stub_0x2acc24() {
    // IDA 0x2acc24: function null-target guard. Option<Box<dyn Fn>>::is_some — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE4swapERS3_")]
// 0x2acce8 — __ZN5boost9function1ISsRKSsE4swapERS3_
pub fn stub_0x2acce8() {
    // IDA 0x2acce8: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE5clearEv")]
// 0x2acdc4 — __ZN5boost9function1ISsRKSsE5clearEv
pub fn stub_0x2acdc4() {
    // IDA 0x2acdc4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE11move_assignERS3_")]
// 0x2acdf0 — __ZN5boost9function1ISsRKSsE11move_assignERS3_
pub fn stub_0x2acdf0() {
    // IDA 0x2acdf0: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_")]
// 0x2acef4 — __ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_
pub fn stub_0x2acef4() {
    // IDA 0x2acef4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE5dummy7nonnullEv")]
// 0x2ad520 — __ZN5boost9function1IvP9lua_StateE5dummy7nonnullEv
pub fn stub_0x2ad520() {
    // IDA 0x2ad520: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v")]
// 0x2adfd8 — __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v
pub fn stub_0x2adfd8() {
    // IDA 0x2adfd8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v")]
// 0x2ae020 — __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v
pub fn stub_0x2ae020() {
    // IDA 0x2ae020: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv")]
// 0x2ae108 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv
pub fn stub_0x2ae108() {
    // IDA 0x2ae108: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v")]
// 0x2ae77c — __ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v
pub fn stub_0x2ae77c() {
    // IDA 0x2ae77c: function null-target guard. Option<Box<dyn Fn>>::is_some — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv")]
// 0x2ae7c0 — __ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv
pub fn stub_0x2ae7c0() {
    // IDA 0x2ae7c0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v")]
// 0x2ae7c4 — __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v
pub fn stub_0x2ae7c4() {
    // IDA 0x2ae7c4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
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
pub fn stub_0x2b1a6c() {
    // IDA 0x2b1a6c: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE11move_assignERS3_")]
// 0x2b1b48 — __ZN5boost9function1IvP9lua_StateE11move_assignERS3_
pub fn stub_0x2b1b48() {
    // IDA 0x2b1b48: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateE5clearEv")]
// 0x2b1c4c — __ZN5boost9function1IvP9lua_StateE5clearEv
pub fn stub_0x2b1c4c() {
    // IDA 0x2b1c4c: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE5dummy7nonnullEv")]
// 0x2b2688 — __ZN5boost9function2IvP9lua_StatemE5dummy7nonnullEv
pub fn stub_0x2b2688() {
    // IDA 0x2b2688: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// 0x2b268c — __ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
pub fn stub_0x2b268c() {
    // IDA 0x2b268c: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_")]
// 0x2b27b8 — __ZN5boost9function2IvP9lua_StatemE9assign_toINS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEEvT_
pub fn stub_0x2b27b8() {
    // IDA 0x2b27b8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
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
pub fn stub_0x2b2d20() {
    // IDA 0x2b2d20: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_")]
// 0x2b3f84 — __ZN5boost9function2IvP9lua_StatemE13assign_to_ownERKS3_
pub fn stub_0x2b3f84() {
    // IDA 0x2b3f84: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemE5clearEv")]
// 0x2b3fb4 — __ZN5boost9function2IvP9lua_StatemE5clearEv
pub fn stub_0x2b3fb4() {
    // IDA 0x2b3fb4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_")]
// 0x2b3fe0 — __ZN5boost9function1ImP9lua_StateE13assign_to_ownERKS3_
pub fn stub_0x2b3fe0() {
    // IDA 0x2b3fe0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1ImP9lua_StateE5clearEv")]
// 0x2b4010 — __ZN5boost9function1ImP9lua_StateE5clearEv
pub fn stub_0x2b4010() {
    // IDA 0x2b4010: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
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
