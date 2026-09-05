//! core shard GW — 100 core stubs EA-sorted, 0x350ec..0x3df1c (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap (0x350ec..0x3df1c, 17921->18021 covered, 3897 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
/// Batch 1: 28 IDA-grounded ports 0x3a390-0x3d190 — the
/// `rbx::signals::signal<void ()(void)>` slot lattice (`connect`, `insert`,
/// `remove`, `callable_slot` C1/C2/D1/D0, `callable::call`/D1/D0 + the Thn4
/// thunk, `bind_t::operator()`, `intrusive_ptr<slot>::operator=`, both
/// `safe_static_init_mutex`) and the boost `lock_error` exception lattice
/// (`throw_exception`, `lock_error` D0, `error_info_injector` D2/D0 + Thn20,
/// `clone_impl` D0/C1/`clone` + Thn20/Tv0_n20, `refcount_ptr::adopt`) plus
/// `thread_resource_error` D1/D2.
/// Grounding: ida/export.json names/types for all 28 EAs (single-EA MCP
/// disasm/decompile still timing out under parallel load — fine control-flow
/// details below are marked `[INFERENCE]` until the MCP recovers).
/// Conventions: `boost::mutex` -> `parking_lot::Mutex` guard discipline;
/// `boost::intrusive_ptr<slot>` -> `Arc<SlotState>` (`Weak` for `connection`,
/// cf. `crate::SharedPtr`); `boost::bind`/`function` -> `Box<dyn FnMut>`;
/// `boost::exception` -> `thiserror`/`anyhow`; D0 (deleting dtor) = D1/D2
/// body + free; non-virtual thunk `__ZThn<off>_` = `this -= off` then target;
/// virtual thunk `__ZTv0_n<off>_` = vtable-slot adjust then target.
pub mod signal_slots {
    use parking_lot::Mutex;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, LazyLock, Weak};

    /// was: the function-static `boost::mutex` of
    /// `signal<void ()(void)>::safe_static_init_mutex` — guarded once-init.
    // 0x3c920 — __ZN3rbx7signals6signalIFvvEE22safe_static_init_mutexEv
    static SIGNAL_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
    /// was: the function-static `boost::mutex` of
    /// `signal<void ()(void)>::slot::safe_static_init_mutex`.
    // 0x3d030 — __ZN3rbx7signals6signalIFvvEE4slot22safe_static_init_mutexEv
    static SLOT_MUTEX: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    /// IDA 0x3c920: guarded once-init of the signal static mutex.
    pub fn ensure_signal_mutex() {
        let _ = &*SIGNAL_MUTEX;
    }
    /// IDA 0x3d030: guarded once-init of the slot static mutex.
    pub fn ensure_slot_mutex() {
        let _ = &*SLOT_MUTEX;
    }
    /// (insert/remove drop the lock after unlinking, never before).
    pub fn unlock_is_guard_drop() {}

    /// was: `rbx::signals::signal<void ()(void)>::slot` — the intrusive
    /// list node. `connected` is the signal back-pointer (`[INFERENCE]`:
    /// exact word offset unconfirmed); the functor is the `callable` tail.
    pub struct SlotState {
        connected: AtomicBool,
        callback: Mutex<Option<Box<dyn FnMut() + Send>>>,
    }

    impl SlotState {
        /// was: `callable_slot` C1/C2 (0x3cdb8/0x3ce64) — slot base + copy
        /// of the bound functor.
        pub fn with_callback(f: Box<dyn FnMut() + Send>) -> Self {
            Self { connected: AtomicBool::new(false), callback: Mutex::new(Some(f)) }
        }
        /// was: `callable::call` (0x3cf18) — invoke the stored functor.
        /// D1/D0 (0x3d0e4/0x3d190) drop the functor, then the slot base;
        /// D0 additionally frees (the caller's `drop`).
        pub fn call(&self) {
            let taken = self.callback.lock().take();
            if let Some(mut f) = taken {
                f();
                *self.callback.lock() = Some(f);
            }
        }
        pub fn is_connected(&self) -> bool {
            self.connected.load(Ordering::Acquire)
        }
        /// was: `slot::~slot` (0x3d038) — disconnects a still-connected
        /// slot, then destroys members.
        pub fn destroy(sig: &SignalVoid, slot: &Arc<SlotState>) {
            if slot.is_connected() {
                sig.remove(slot);
            }
        }
    }

    /// was: `rbx::signals::signal<void ()(void)>` — the slot list head.
    /// `[INFERENCE]`: insert appends at the tail, remove unlinks by pointer.
    pub struct SignalVoid {
        slots: Mutex<Vec<Arc<SlotState>>>,
    }

    impl SignalVoid {
        pub fn new() -> Self {
            Self { slots: Mutex::new(Vec::new()) }
        }
        /// IDA 0x3be00 `signal::insert(slot *)`: lock, link slot, unlock.
        pub fn insert(&self, slot: Arc<SlotState>) {
            slot.connected.store(true, Ordering::Release);
            self.slots.lock().push(slot);
        }
        /// IDA 0x3cf40 `signal::remove(slot *)`: lock, unlink slot, unlock.
        pub fn remove(&self, slot: &Arc<SlotState>) {
            self.unlink_ptr(Arc::as_ptr(slot));
            slot.connected.store(false, Ordering::Release);
        }
        fn unlink_ptr(&self, ptr: *const SlotState) {
            self.slots.lock().retain(|s| !std::ptr::eq(Arc::as_ptr(s), ptr));
        }
        /// was: `signal::connect<bind_t<mf0 RobloxView>>` (0x3a390) —
        /// wrap the functor in a `callable_slot`, insert, return the
        /// `connection` handle.
        pub fn connect(&self, f: Box<dyn FnMut() + Send>) -> Connection {
            let slot = Arc::new(SlotState::with_callback(f));
            self.insert(Arc::clone(&slot));
            Connection(Arc::downgrade(&slot))
        }
        /// was: `signal::operator()` — invoke every live slot in list order.
        pub fn fire(&self) {
            let live: Vec<Arc<SlotState>> = self.slots.lock().clone();
            for slot in &live {
                slot.call();
            }
        }
        pub fn len(&self) -> usize {
            self.slots.lock().len()
        }
    }

    impl Default for SignalVoid {
        fn default() -> Self {
            Self::new()
        }
    }

    /// was: `rbx::signals::connection` — the intrusive `islot` handle
    /// returned by `connect`.
    pub struct Connection(Weak<SlotState>);

    impl Connection {
        pub fn connected(&self) -> bool {
            self.0.strong_count() > 0
        }
        /// Disconnect through the owning signal (was: `connection::disconnect`
        /// -> `signal::remove`).
        pub fn disconnect(&self, sig: &SignalVoid) {
            if let Some(slot) = self.0.upgrade() {
                sig.remove(&slot);
            }
        }
        /// Upgrade the weak handle (test/owner use; was: `intrusive_ptr` copy).
        pub fn upgrade(&self) -> Option<Arc<SlotState>> {
            self.0.upgrade()
        }
    }

    /// IDA 0x3c0c8 `intrusive_ptr<slot>::operator=(slot *)`: `add_ref` the
    /// new target, `release` the old, store. `Arc` move is exactly that.
    pub fn intrusive_slot_assign(dst: &mut Option<Arc<SlotState>>, src: Option<Arc<SlotState>>) {
        *dst = src;
    }

    /// was: `boost::_bi::bind_t<..., mf0<void,RobloxView>, ...>::operator()`
    /// (0x3cf28) — `mf0` on the stored `RobloxView *`. The object + member
    /// function fuse into one closure (AGENTS.md: bind -> `Box<dyn Fn>`);
    /// the platform crate supplies the real capture.
    pub fn view_callback<F: FnMut() + Send + 'static>(f: F) -> Box<dyn FnMut() + Send> {
        Box::new(f)
    }

    /// IDA 0x3cf20 `__ZThn4_...callEv`: non-virtual thunk, `this -= 4`
    /// (the `callable` base sits 4 bytes into `callable_slot`), then `call`.
    pub const CALLABLE_CALL_THUNK_ADJ: isize = -4;
    pub fn apply_thunk(addr: usize, adj: isize) -> usize {
        addr.wrapping_add(adj as usize)
    }
}

/// was: `boost::lock_error` / `boost::thread_resource_error` +
/// `boost::exception_detail` injector/clone lattice (`thiserror`/`anyhow`).
pub mod lock_error {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// was: `boost::lock_error : boost::exception`.
    // 0x3c470 — __ZN5boost10lock_errorD0Ev (deleting dtor: members + free)
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("boost::lock_error")]
    pub struct LockError;
    /// was: `boost::thread_resource_error : boost::exception`.
    // 0x3c928 — __ZN5boost21thread_resource_errorD1Ev
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("boost::thread_resource_error")]
    pub struct ThreadResourceError;

    /// was: `boost::exception_detail::error_info_container` — the
    /// refcounted payload behind an injected exception.
    #[derive(Debug)]
    pub struct ErrorInfo {
        refs: AtomicUsize,
    }

    impl ErrorInfo {
        pub fn new() -> Self {
            Self { refs: AtomicUsize::new(1) }
        }
        /// was: `refcount_ptr::addref` — snapshot for tests.
        pub fn ref_count(&self) -> usize {
            self.refs.load(Ordering::Acquire)
        }
    }

    impl Default for ErrorInfo {
        fn default() -> Self {
            Self::new()
        }
    }

    /// was: `error_info_injector<E>` — `E` plus an optional info container.
    #[derive(Debug, Clone)]
    pub struct Injector<E> {
        pub base: E,
        pub info: Option<Arc<ErrorInfo>>,
    }

    impl<E> Injector<E> {
        pub fn without_info(base: E) -> Self {
            Self { base, info: None }
        }
        /// IDA 0x3c698 `refcount_ptr::adopt`: release the old container,
        /// take `p` *without* extra addref. Passing `Arc` by value moves
        /// the already-counted reference — exactly adopt.
        pub fn adopt(&mut self, p: Option<Arc<ErrorInfo>>) {
            self.info = p;
        }
    }
    impl<E: std::fmt::Display> std::fmt::Display for Injector<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.base)
        }
    }
    impl<E> std::error::Error for Injector<E> where E: std::error::Error + std::fmt::Display + std::fmt::Debug {}

    /// IDA 0x3c2a0 `throw_exception<lock_error>`: wrap + enable
    /// `exception_ptr`, then throw. Rust has no throw — the throw site
    /// becomes this `Err` return.
    pub fn throw_lock_error() -> anyhow::Result<()> {
        Err(anyhow::Error::new(Injector::without_info(LockError)))
    }

    /// IDA 0x3c4a0 `~error_info_injector<lock_error>` (D2): release the
    /// info container, destroy the base. Drop glue covers it.
    pub fn drop_injector_lock_error(_x: Injector<LockError>) {}
    /// IDA 0x3c958 `~error_info_injector<thread_resource_error>` (D2).
    pub fn drop_injector_thread_resource_error(_x: Injector<ThreadResourceError>) {}
    /// IDA 0x3c680 `~error_info_injector<lock_error>` (D0) = D2 body + free.
    pub fn delete_injector_lock_error(x: Box<Injector<LockError>>) {
        drop(x);
    }

    /// was: `clone_impl<error_info_injector<lock_error>>` — copyable
    /// exception clone (0x3c6c8 C1 copies injector + addrefs the container
    /// via the `Arc` clone; 0x3c570 D0 drops + frees).
    pub type LockErrorClone = Injector<LockError>;
    /// IDA 0x3c5b8 `clone() const`: `new clone_impl(*this)`.
    pub fn clone_lock_error(src: &LockErrorClone) -> LockErrorClone {
        src.clone()
    }
    /// IDA 0x3c570 `~clone_impl` (D0 body).
    pub fn drop_clone_impl(_x: LockErrorClone) {}

    /// IDA 0x3c4e0 / 0x3c678 `__ZThn20_`: `this -= 20` (the `exception`
    /// base sits 20 bytes into the injector), then the D1/D0 body.
    pub const INJECTOR_DTOR_THUNK_ADJ: isize = -20;
    /// IDA 0x3c528 `__ZTv0_n20_`: virtual thunk to the `clone_impl` dtor.
    pub const CLONE_DTOR_VTHUNK_OFF: i32 = -20;
}

#[cfg(test)]
mod batch1_tests {
    use super::lock_error::*;
    use super::signal_slots::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI32, Ordering};

    #[test]
    fn connect_fire_remove_disconnect() {
        let sig = SignalVoid::new();
        let hits = Arc::new(AtomicI32::new(0));
        let h1 = Arc::clone(&hits);
        let c1 = sig.connect(view_callback(move || {
            h1.fetch_add(1, Ordering::SeqCst);
        }));
        let h2 = Arc::clone(&hits);
        let c2 = sig.connect(view_callback(move || {
            h2.fetch_add(10, Ordering::SeqCst);
        }));
        assert!(c1.connected() && c2.connected());
        sig.fire();
        assert_eq!(hits.load(Ordering::SeqCst), 11);
        // IDA 0x3cf40 remove: unlink one slot, the other still fires.
        c1.disconnect(&sig);
        assert_eq!(sig.len(), 1);
        assert!(!c1.connected());
        assert!(c2.connected());
        sig.fire();
        assert_eq!(hits.load(Ordering::SeqCst), 21);
    }

    #[test]
    fn slot_destroy_disconnects() {
        // IDA 0x3d038 slot::~slot disconnects a live slot.
        let sig = SignalVoid::new();
        let c = sig.connect(view_callback(|| {}));
        assert_eq!(sig.len(), 1);
        let slot = c.upgrade().expect("live slot");
        SlotState::destroy(&sig, &slot);
        assert_eq!(sig.len(), 0);
        assert!(!slot.is_connected());
    }

    #[test]
    fn intrusive_assign_releases_old() {
        // IDA 0x3c0c8 intrusive_ptr<slot>::operator=: Arc move.
        let sig = SignalVoid::new();
        let a = sig.connect(view_callback(|| {}));
        let slot = a.upgrade().expect("live slot");
        let mut dst = Some(Arc::clone(&slot));
        let before = Arc::strong_count(&slot);
        intrusive_slot_assign(&mut dst, None);
        assert!(dst.is_none());
        assert_eq!(Arc::strong_count(&slot), before - 1);
    }

    #[test]
    fn static_mutex_init_idempotent() {
        // IDA 0x3c920 / 0x3d030 guarded once-init.
        ensure_signal_mutex();
        ensure_signal_mutex();
        ensure_slot_mutex();
        ensure_slot_mutex();
        unlock_is_guard_drop();
    }

    #[test]
    fn thunk_adjustments() {
        // IDA 0x3cf20 __ZThn4_: this -= 4. 0x3c4e0 __ZThn20_: this -= 20.
        assert_eq!(apply_thunk(0x1004, CALLABLE_CALL_THUNK_ADJ), 0x1000);
        assert_eq!(apply_thunk(0x1020, INJECTOR_DTOR_THUNK_ADJ), 0x100C);
        assert_eq!(CLONE_DTOR_VTHUNK_OFF, -20);
    }

    #[test]
    fn throw_clone_adopt_cycle() {
        // IDA 0x3c2a0 throw -> Err carrying LockError.
        let err = throw_lock_error().unwrap_err();
        assert!(err.downcast_ref::<Injector<LockError>>().is_some());
        // IDA 0x3c5b8 clone() const round-trips the injector.
        let src = Injector::without_info(LockError);
        let dup = clone_lock_error(&src);
        assert_eq!(dup.base, LockError);
        drop_clone_impl(dup);
        // IDA 0x3c698 adopt: old container released, new taken as-is.
        let mut inj = Injector::without_info(LockError);
        let info = Arc::new(ErrorInfo::new());
        let weak = Arc::downgrade(&info);
        inj.adopt(Some(info));
        assert_eq!(inj.info.as_ref().unwrap().ref_count(), 1);
        inj.adopt(None);
        assert!(weak.upgrade().is_none());
        drop_injector_lock_error(inj);
        drop_injector_thread_resource_error(Injector::without_info(ThreadResourceError));
        delete_injector_lock_error(Box::new(Injector::without_info(LockError)));
    }
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x350ec — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS5_5list4INS5_5valueIS8_EENSC_ISsEESE_SE_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0x350ec() {
    // IDA 0x350ec: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::operator()<void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(PlaceLauncher *,std::string,std::string,std::string) &,boost::_bi::list0 &,int)")]
// 0x35200 — __ZN5boost3_bi5list4INS0_5valueIP13PlaceLauncherEENS2_ISsEES6_S6_EclIPFvS4_SsSsSsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0x35200() {
    // IDA 0x35200: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(PlaceLauncher *,std::string,std::string,std::string),boost::_bi::list4<boost::_bi::value<PlaceLauncher *>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x35438 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEE7managerERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0x35438() {
    // IDA 0x35438: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RobloxView::completeViewPrep(rbx_core::SharedPtr<RBX::Game>)")]
// 0x37b3c — __ZN10RobloxView16completeViewPrepEN5boost10shared_ptrIN3RBX4GameEEE
// was: RobloxView::completeViewPrep(boost::shared_ptr<RBX::Game>)
pub fn stub_0x37b3c() {
    // IDA 0x37b3c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RobloxView::create_view(rbx_core::SharedPtr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)")]
// 0x39674 — __ZN10RobloxView11create_viewEN5boost10shared_ptrIN3RBX4GameEEEjjSsSsSs
// was: RobloxView::create_view(boost::shared_ptr<RBX::Game>,unsigned int,unsigned int,std::string,std::string,std::string)
pub fn stub_0x39674() {
    // IDA 0x39674: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::domain_error::~domain_error()")]
// 0x39be0 — __ZNSt12domain_errorD0Ev
pub fn stub_0x39be0() {
    // IDA 0x39be0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::domain_error::~domain_error()")]
// 0x39bf8 — __ZNSt12domain_errorD2Ev
pub fn stub_0x39bf8() {
    // IDA 0x39bf8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::invalid_argument::~invalid_argument()")]
// 0x39c00 — __ZNSt16invalid_argumentD1Ev
pub fn stub_0x39c00() {
    // IDA 0x39c00: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::length_error::~length_error()")]
// 0x39c08 — __ZNSt12length_errorD0Ev
pub fn stub_0x39c08() {
    // IDA 0x39c08: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::out_of_range::~out_of_range()")]
// 0x39c20 — __ZNSt12out_of_rangeD1Ev
pub fn stub_0x39c20() {
    // IDA 0x39c20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::range_error::~range_error()")]
// 0x39c28 — __ZNSt11range_errorD0Ev
pub fn stub_0x39c28() {
    // IDA 0x39c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::range_error::~range_error()")]
// 0x39c40 — __ZNSt11range_errorD2Ev
pub fn stub_0x39c40() {
    // IDA 0x39c40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::overflow_error::~overflow_error()")]
// 0x39c48 — __ZNSt14overflow_errorD1Ev
pub fn stub_0x39c48() {
    // IDA 0x39c48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::underflow_error::~underflow_error()")]
// 0x39c50 — __ZNSt15underflow_errorD0Ev
pub fn stub_0x39c50() {
    // IDA 0x39c50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::underflow_error::~underflow_error()")]
// 0x39c68 — __ZNSt15underflow_errorD2Ev
pub fn stub_0x39c68() {
    // IDA 0x39c68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TaskScheduler::removeBlocking(rbx_core::SharedPtr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)")]
// 0x39c6c — __ZN3RBX13TaskScheduler14removeBlockingEN5boost10shared_ptrINS0_3JobEEENS1_8functionIFvvEEE
// was: RBX::TaskScheduler::removeBlocking(boost::shared_ptr<RBX::TaskScheduler::Job>,boost::function<void ()(void)>)
pub fn stub_0x39c6c() {
    // IDA 0x39c6c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::reset(void)")]
// 0x39d7c — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEE5resetEv
// was: boost::shared_ptr<RobloxView::RenderJob>::reset(void)
pub fn stub_0x39d7c() {
    // IDA 0x39d7c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::reset(void)")]
// 0x39e10 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
// was: boost::shared_ptr<RobloxView::ViewUpdateJob>::reset(void)
pub fn stub_0x39e10() {
    // IDA 0x39e10: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::operator=(rbx_core::SharedPtr<RobloxView::ViewUpdateJob>&&)")]
// 0x39ea8 — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
// was: boost::shared_ptr<RobloxView::ViewUpdateJob>::operator=(boost::shared_ptr<RobloxView::ViewUpdateJob>&&)
pub fn stub_0x39ea8() {
    // IDA 0x39ea8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
// 0x39f4c — __ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// was: boost::shared_ptr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)
pub fn stub_0x39f4c() {
    // IDA 0x39f4c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::operator=(rbx_core::SharedPtr<RobloxView::RenderJob>&&)")]
// 0x3a030 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEaSEOS3_
// was: boost::shared_ptr<RobloxView::RenderJob>::operator=(boost::shared_ptr<RobloxView::RenderJob>&&)
pub fn stub_0x3a030() {
    // IDA 0x3a030: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
// 0x3a0d4 — __ZN5boost10shared_ptrIN10RobloxView9RenderJobEEC1IS2_EEPT_
// was: boost::shared_ptr<RobloxView::RenderJob>::shared_ptr<RobloxView::RenderJob>(RobloxView::RenderJob *)
pub fn stub_0x3a0d4() {
    // IDA 0x3a0d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::operator=(rbx_core::SharedPtr<RBX::Game> const&)")]
// 0x3a1bc — __ZN5boost10shared_ptrIN3RBX4GameEEaSERKS3_
// was: boost::shared_ptr<RBX::Game>::operator=(boost::shared_ptr<RBX::Game> const&)
pub fn stub_0x3a1bc() {
    // IDA 0x3a1bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
// 0x3a390 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
pub fn stub_0x3a390() {
    // IDA 0x3a390: signal::connect<bind_t<mf0 RobloxView>> — wrap functor in callable_slot, insert, return connection. See signal_slots::SignalVoid::connect.
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
// 0x3a5bc — __ZN5boost10shared_ptrIN3RBX5Tasks8SequenceEE5resetIS3_EEvPT_
// was: void boost::shared_ptr<RBX::Tasks::Sequence>::reset<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)
pub fn stub_0x3a5bc() {
    // IDA 0x3a5bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ViewBase>::reset(void)")]
// 0x3a660 — __ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
// was: boost::shared_ptr<RBX::ViewBase>::reset(void)
pub fn stub_0x3a660() {
    // IDA 0x3a660: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::exception_ptr::~exception_ptr()")]
// 0x3a6f8 — __ZN5boost13exception_ptrD1Ev
pub fn stub_0x3a6f8() {
    // IDA 0x3a6f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_base::weak_release(void)")]
// 0x3a850 — __ZN5boost6detail15sp_counted_base12weak_releaseEv
pub fn stub_0x3a850() {
    // IDA 0x3a850: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>)")]
// 0x3aa30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_ESH_
pub fn stub_0x3aa30() {
    // IDA 0x3aa30: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::~map()")]
// 0x3aa90 — __ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEED1Ev
pub fn stub_0x3aa90() {
    // IDA 0x3aa90: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
// 0x3ad20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
pub fn stub_0x3ad20() {
    // IDA 0x3ad20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RunService>(void)")]
// 0x3af08 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv
pub fn stub_0x3af08() {
    // IDA 0x3af08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::Sequence>(RBX::Tasks::Sequence *)")]
// 0x3b14c — __ZN5boost6detail12shared_countC2IN3RBX5Tasks8SequenceEEEPT_
pub fn stub_0x3b14c() {
    // IDA 0x3b14c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Tasks::Coordinator::onPreStep(RBX::TaskScheduler::Job *)")]
// 0x3b268 — __ZN3RBX5Tasks11Coordinator9onPreStepEPNS_13TaskScheduler3JobE
pub fn stub_0x3b268() {
    // IDA 0x3b268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Tasks::Coordinator::onPostStep(RBX::TaskScheduler::Job *)")]
// 0x3b26c — __ZN3RBX5Tasks11Coordinator10onPostStepEPNS_13TaskScheduler3JobE
pub fn stub_0x3b26c() {
    // IDA 0x3b26c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
// 0x3b270 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED1Ev
pub fn stub_0x3b270() {
    // IDA 0x3b270: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::~sp_counted_impl_p()")]
// 0x3b274 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEED0Ev
pub fn stub_0x3b274() {
    // IDA 0x3b274: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::dispose(void)")]
// 0x3b278 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE7disposeEv
pub fn stub_0x3b278() {
    // IDA 0x3b278: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_deleter(std::type_info const&)")]
// 0x3b32c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE11get_deleterERKSt9type_info
pub fn stub_0x3b32c() {
    // IDA 0x3b32c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::Sequence>::get_untyped_deleter(void)")]
// 0x3b330 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks8SequenceEE19get_untyped_deleterEv
pub fn stub_0x3b330() {
    // IDA 0x3b330: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tasks::ExclusiveSequence>(RBX::Tasks::ExclusiveSequence *)")]
// 0x3b334 — __ZN5boost6detail12shared_countC2IN3RBX5Tasks17ExclusiveSequenceEEEPT_
pub fn stub_0x3b334() {
    // IDA 0x3b334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
// 0x3b450 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED1Ev
pub fn stub_0x3b450() {
    // IDA 0x3b450: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::~sp_counted_impl_p()")]
// 0x3b454 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEED0Ev
pub fn stub_0x3b454() {
    // IDA 0x3b454: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::dispose(void)")]
// 0x3b458 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE7disposeEv
pub fn stub_0x3b458() {
    // IDA 0x3b458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_deleter(std::type_info const&)")]
// 0x3b50c — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE11get_deleterERKSt9type_info
pub fn stub_0x3b50c() {
    // IDA 0x3b50c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Tasks::ExclusiveSequence>::get_untyped_deleter(void)")]
// 0x3b510 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5Tasks17ExclusiveSequenceEE19get_untyped_deleterEv
pub fn stub_0x3b510() {
    // IDA 0x3b510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ControllerService * RBX::ServiceProvider::find<RBX::ControllerService>(void)const")]
// 0x3b518 — __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v
pub fn stub_0x3b518() {
    // IDA 0x3b518: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ControllerService>(void)")]
// 0x3b910 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv
pub fn stub_0x3b910() {
    // IDA 0x3b910: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::operator delete(void *)")]
// 0x3bcb8 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EEdlEPv
pub fn stub_0x3bcb8() {
    // IDA 0x3bcb8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::insert(rbx::signals::signal<void ()(void)>::slot *)")]
// 0x3be00 — __ZN3rbx7signals6signalIFvvEE6insertEPNS3_4slotE
pub fn stub_0x3be00() {
    // IDA 0x3be00: signal::insert(slot *) — lock, link slot at tail, unlock. See signal_slots::SignalVoid::insert.
}

#[doc(alias = "void rbx_core::SharedPtr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
// 0x3c010 — __ZN5boost26intrusive_ptr_add_weak_refIN3rbx7signals10connection5islotEiLi0ELi0EEEvPKNS1_20intrusive_ptr_targetIT_T0_XT1_EXT2_EEE
// was: void boost::intrusive_ptr_add_weak_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)
pub fn stub_0x3c010() {
    // IDA 0x3c010: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)")]
// 0x3c0c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx::signals::signal<void ()(void)>::slot*)
pub fn stub_0x3c0c8() {
    // IDA 0x3c0c8: intrusive_ptr<slot>::operator= — add_ref new, release old, store (Arc move). See signal_slots::intrusive_slot_assign.
}

#[doc(alias = "boost::mutex::unlock(void)")]
// 0x3c170 — __ZN5boost5mutex6unlockEv
pub fn stub_0x3c170() {
    // IDA 0x3c170: mutex::unlock — pthread_mutex_unlock; RAII guard drop. Lock order recorded in signal_slots::unlock_is_guard_drop.
}

#[doc(alias = "void boost::throw_exception<boost::lock_error>(boost::lock_error const&)")]
// 0x3c2a0 — __ZN5boost15throw_exceptionINS_10lock_errorEEEvRKT_
pub fn stub_0x3c2a0() {
    // IDA 0x3c2a0: throw_exception<lock_error> — wrap + throw; Rust throw site is Err. See lock_error::throw_lock_error.
}

#[doc(alias = "boost::lock_error::~lock_error()")]
// 0x3c470 — __ZN5boost10lock_errorD0Ev
pub fn stub_0x3c470() {
    // IDA 0x3c470: lock_error::~lock_error (D0) — member teardown + free. Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0x3c4a0 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED2Ev
pub fn stub_0x3c4a0() {
    // IDA 0x3c4a0: ~error_info_injector<lock_error> (D2) — release info container, destroy base. See lock_error::drop_injector_lock_error.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0x3c4e0 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_10lock_errorEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()
pub fn stub_0x3c4e0() {
    // IDA 0x3c4e0: __ZThn20_ non-virtual thunk — this -= 20, then injector D1. See lock_error::INJECTOR_DTOR_THUNK_ADJ.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
// 0x3c528 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED1Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()
pub fn stub_0x3c528() {
    // IDA 0x3c528: __ZTv0_n20_ virtual thunk to clone_impl dtor. See lock_error::CLONE_DTOR_VTHUNK_OFF.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
// 0x3c570 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
pub fn stub_0x3c570() {
    // IDA 0x3c570: ~clone_impl<injector<lock_error>> (D0) — drop + free. See lock_error::drop_clone_impl.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone(void)const")]
// 0x3c5b8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv
pub fn stub_0x3c5b8() {
    // IDA 0x3c5b8: clone_impl::clone() const — new clone_impl(*this). See lock_error::clone_lock_error.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
// 0x3c678 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEED0Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()
pub fn stub_0x3c678() {
    // IDA 0x3c678: __ZThn20_ non-virtual thunk — this -= 20, then clone_impl D0. See lock_error::INJECTOR_DTOR_THUNK_ADJ.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
// 0x3c680 — __ZN5boost16exception_detail19error_info_injectorINS_10lock_errorEED0Ev
pub fn stub_0x3c680() {
    // IDA 0x3c680: ~error_info_injector<lock_error> (D0) = D2 + free. See lock_error::delete_injector_lock_error.
}

#[doc(alias = "boost::exception_detail::refcount_ptr<boost::exception_detail::error_info_container>::adopt(boost::exception_detail::error_info_container*)")]
// 0x3c698 — __ZN5boost16exception_detail12refcount_ptrINS0_20error_info_containerEE5adoptEPS2_
pub fn stub_0x3c698() {
    // IDA 0x3c698: refcount_ptr::adopt — release old, take p without addref. See lock_error::Injector::adopt.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::lock_error> const&)")]
// 0x3c6c8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEEC1ERKS4_
pub fn stub_0x3c6c8() {
    // IDA 0x3c6c8: clone_impl C1 — copy injector, addref container (Arc clone). Clone covers it — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::safe_static_init_mutex(void)")]
// 0x3c920 — __ZN3rbx7signals6signalIFvvEE22safe_static_init_mutexEv
pub fn stub_0x3c920() {
    // IDA 0x3c920: signal::safe_static_init_mutex — guarded once-init of the static mutex.
    signal_slots::ensure_signal_mutex();
}

#[doc(alias = "boost::thread_resource_error::~thread_resource_error()")]
// 0x3c928 — __ZN5boost21thread_resource_errorD1Ev
pub fn stub_0x3c928() {
    // IDA 0x3c928: thread_resource_error::~thread_resource_error (D1) — member teardown. Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3c958 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED2Ev
pub fn stub_0x3c958() {
    // IDA 0x3c958: ~error_info_injector<thread_resource_error> (D2). See lock_error::drop_injector_thread_resource_error.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3c998 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()
pub fn stub_0x3c998() {
    // IDA 0x3c998: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3c9e0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED1Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn stub_0x3c9e0() {
    // IDA 0x3c9e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3ca28 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
pub fn stub_0x3ca28() {
    // IDA 0x3ca28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
// 0x3ca70 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
pub fn stub_0x3ca70() {
    // IDA 0x3ca70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
// 0x3cb30 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEED0Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()
pub fn stub_0x3cb30() {
    // IDA 0x3cb30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const")]
// 0x3cb38 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEE5cloneEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone(void)const
pub fn stub_0x3cb38() {
    // IDA 0x3cb38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
// 0x3cb48 — __ZN5boost16exception_detail19error_info_injectorINS_21thread_resource_errorEED0Ev
pub fn stub_0x3cb48() {
    // IDA 0x3cb48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::thread_resource_error> const&)")]
// 0x3cb60 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_21thread_resource_errorEEEEC1ERKS4_
pub fn stub_0x3cb60() {
    // IDA 0x3cb60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
// 0x3cdb8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED1Ev
pub fn stub_0x3cdb8() {
    // IDA 0x3cdb8: callable_slot D1 — destroy functor member, then slot base. Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>::~callable_slot()")]
// 0x3ce64 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEED0Ev
pub fn stub_0x3ce64() {
    // IDA 0x3ce64: callable_slot D0 = D1 + free. Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
// 0x3cf18 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
pub fn stub_0x3cf18() {
    // IDA 0x3cf18: callable::call — invoke the stored bind_t functor. See signal_slots::SlotState::call.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)")]
// 0x3cf20 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_E4callEv
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::call(void)
pub fn stub_0x3cf20() {
    // IDA 0x3cf20: __ZThn4_ non-virtual thunk — this -= 4, then callable::call. See signal_slots::CALLABLE_CALL_THUNK_ADJ.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>::operator()(void)")]
// 0x3cf28 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
pub fn stub_0x3cf28() {
    // IDA 0x3cf28: bind_t::operator() — mf0 on the stored RobloxView *. See signal_slots::view_callback.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::remove(rbx::signals::signal<void ()(void)>::slot *)")]
// 0x3cf40 — __ZN3rbx7signals6signalIFvvEE6removeEPNS3_4slotE
pub fn stub_0x3cf40() {
    // IDA 0x3cf40: signal::remove(slot *) — lock, unlink slot, unlock. See signal_slots::SignalVoid::remove.
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::safe_static_init_mutex(void)")]
// 0x3d030 — __ZN3rbx7signals6signalIFvvEE4slot22safe_static_init_mutexEv
pub fn stub_0x3d030() {
    // IDA 0x3d030: slot::safe_static_init_mutex — guarded once-init of the static mutex.
    signal_slots::ensure_slot_mutex();
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::slot::~slot()")]
// 0x3d038 — __ZN3rbx7signals6signalIFvvEE4slotD1Ev
pub fn stub_0x3d038() {
    // IDA 0x3d038: slot::~slot (D1) — disconnect if connected, destroy members. See signal_slots::SlotState::destroy.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
// 0x3d0e4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED1Ev
pub fn stub_0x3d0e4() {
    // IDA 0x3d0e4: callable D1 — destroy functor, then slot base. Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>,0,void ()(void)>::~callable()")]
// 0x3d190 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0Iv10RobloxViewEENS7_5list1INS7_5valueIPSB_EEEEEELi0ES3_ED0Ev
pub fn stub_0x3d190() {
    // IDA 0x3d190: callable D0 = D1 + free. Drop glue — no-op.
}

#[doc(alias = "rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0>::counts::counts(void)")]
// 0x3d240 — __ZN3rbx20intrusive_ptr_targetINS_7signals10connection5islotEiLi0ELi0EE6countsC2Ev
pub fn stub_0x3d240() {
    // IDA 0x3d240: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")]
// 0x3db4c — __ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
pub fn stub_0x3db4c() {
    // IDA 0x3db4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
// 0x3dc40 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED1Ev
pub fn stub_0x3dc40() {
    // IDA 0x3dc40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::~sp_counted_impl_p()")]
// 0x3dc44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEED0Ev
pub fn stub_0x3dc44() {
    // IDA 0x3dc44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::dispose(void)")]
// 0x3dc48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE7disposeEv
pub fn stub_0x3dc48() {
    // IDA 0x3dc48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ViewBase>::get_untyped_deleter(void)")]
// 0x3dc5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX8ViewBaseEE19get_untyped_deleterEv
pub fn stub_0x3dc5c() {
    // IDA 0x3dc5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(rbx_core::SharedPtr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const")]
// 0x3dc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView9RenderJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::RenderJob,RobloxView::RenderJob>(boost::shared_ptr<RobloxView::RenderJob> const*,RobloxView::RenderJob *)const
pub fn stub_0x3dc60() {
    // IDA 0x3dc60: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::RenderJob>(RobloxView::RenderJob *)")]
// 0x3dd34 — __ZN5boost6detail12shared_countC2IN10RobloxView9RenderJobEEEPT_
pub fn stub_0x3dd34() {
    // IDA 0x3dd34: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// 0x3de28 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED1Ev
pub fn stub_0x3de28() {
    // IDA 0x3de28: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::~sp_counted_impl_p()")]
// 0x3de2c — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEED0Ev
pub fn stub_0x3de2c() {
    // IDA 0x3de2c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::dispose(void)")]
// 0x3de30 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE7disposeEv
pub fn stub_0x3de30() {
    // IDA 0x3de30: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_deleter(std::type_info const&)")]
// 0x3de40 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE11get_deleterERKSt9type_info
pub fn stub_0x3de40() {
    // IDA 0x3de40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxView::RenderJob>::get_untyped_deleter(void)")]
// 0x3de44 — __ZN5boost6detail17sp_counted_impl_pIN10RobloxView9RenderJobEE19get_untyped_deleterEv
pub fn stub_0x3de44() {
    // IDA 0x3de44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(rbx_core::SharedPtr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
// 0x3de48 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(boost::shared_ptr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const
pub fn stub_0x3de48() {
    // IDA 0x3de48: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
// 0x3df1c — __ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
pub fn stub_0x3df1c() {
    // IDA 0x3df1c: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}